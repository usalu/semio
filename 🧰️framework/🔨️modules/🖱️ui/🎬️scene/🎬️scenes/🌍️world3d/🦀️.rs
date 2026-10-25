//! 🌍️ The world-3d scene payload with its presentation and typed lanes, the modelling builders and the world snapshot lease.
//!
//! Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_scene_doc::{scene_lane_hash, scene_pack_wire, value_decode, value_decode_default, value_decode_option, value_push, value_push_if_nonempty, value_push_option, SceneDoc, SceneLanePayload, SceneLaneRef};
use semio_framework_ui_scene_modelling::{world3d_modelling_from_lane_text, world3d_modelling_lane_text, World3dAnnotationLayer, World3dHighlight, World3dModellingOptions, World3dPickGranularity, World3dScalarField, World3dSection};
use semio_framework_ui_scene_pack::{from_bytes, to_bytes, PackError};
use semio_framework_ui_scene_pass::SceneViewportMask3d;
use serde::{Deserialize, Serialize};
#[path = "../../🌍️world3d-snapshot/🦀️.rs"]
mod world3d_snapshot;
pub use world3d_snapshot::{
    world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_claim_draw_permit, world3d_snapshot_close_step, world3d_snapshot_recover_lease,
    world3d_snapshot_recover_page, world3d_snapshot_recover_write, world3d_snapshot_recovery_close_step, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty, world3d_snapshot_with_page, world3d_snapshot_write_terminal_is_empty,
    World3dRejectedSnapshotPage, World3dSnapshotDescriptor, World3dSnapshotDrawPermit, World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotSpan, World3dSnapshotWriteToken,
    WORLD3D_SNAPSHOT_CAPACITY, WORLD3D_SNAPSHOT_DRAW_BYTE_CAPACITY, WORLD3D_SNAPSHOT_DRAW_CAPACITY, WORLD3D_SNAPSHOT_DRAW_INSTANCE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY,
};

//#region 🔖️World3dScene
/// 🎭️ Presentation owned by one world surface independently of its content LOD and camera.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dPresentation {
    #[serde(default = "world3d_presentation_true")]
    pub show_grid: bool,
    #[serde(default = "world3d_presentation_true")]
    pub show_gizmo: bool,
    #[serde(default = "world3d_presentation_true")]
    pub interactive: bool,
    #[serde(default)]
    pub viewport_mask: SceneViewportMask3d,
    #[serde(default)]
    pub clear: World3dPresentationClear,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
}

impl Default for World3dPresentation {
    fn default() -> Self {
        Self { show_grid: true, show_gizmo: true, interactive: true, viewport_mask: Default::default(), clear: Default::default(), source_aspect: None }
    }
}

/// 🫧️ Fallback background owned by a world presentation when the environment has no explicit color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum World3dPresentationClear {
    #[default]
    Theme,
    Transparent,
}

fn world3d_presentation_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World3dScene {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<World3dSnapshotLease>,
    pub camera_json: String,
    #[serde(default = "world3d_default_meshes_json")]
    pub meshes_json: String,
    pub instances_json: String,
    /// 🚚️ Wave B44 (ticket 26/09/02/PUZZLE-3D-END-TO-END): the per-object DELTA that produced the
    /// current `instances_json` — `{base, revision, count, changed:[record], removed:[id]}`. A
    /// consumer whose retained instance set is at `base` may apply `changed`/`removed` by id instead
    /// of re-parsing the whole set; one that is not, or that ignores the lane entirely, reads
    /// `instances_json`, which stays AUTHORITATIVE on every publication. Producers that have no
    /// incremental residency leave it unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances_delta_json: Option<String>,
    #[serde(default = "world3d_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vortices_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attractions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_volumes_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engagement_preview_json: Option<String>,
    /// 🧲️ The SUB-OBJECT pick targets this surface hit-tests against, as a JSON array of
    /// `{kind, id, point, points, typology, selectable, style}` — the wire half of the CAD picking
    /// engine's `SpatialPickTarget`/`target_style`
    /// (`✏️editor/⚙️engine/🧲️picking/`), and the one lane that lets a renderer resolve a
    /// click to a `vertex`/`edge`/`face`/`object` instead of only to the whole instance.
    ///
    /// 🩸️ Before ticket 26/09/17 packet W14g the overlay rode `engagement_preview_json`, which is a
    /// PAINT lane: sub-object picking could be shown and never hit-tested, so every pick still
    /// resolved to the object through the domain's `object` granularity.
    ///
    /// 🧮️ Bounded by construction on both ends: a producer caps the array it publishes and a
    /// consumer caps what it retains, so the lane can never grow an unbounded `Vec` on the wire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pick_targets_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunking_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terrain_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_json: Option<String>,
    /// ⏯️ The base64url `ToolRunTraceDelta` (`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs`) the tool run
    /// ledger pages to this window after the renderer's echoed `toolRunTraceCursor`. Opaque to this
    /// crate by layering: only renderers decode it, and an idle run publishes no lane at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_run_trace: Option<String>,
    /// 📏️ The annotation layer — linear dimensions, angles, point markers and leader labels with
    /// caller-supplied EN/DE text. A typed lane: [`crate::World3dAnnotationLayer`] is carried as JSON text
    /// only on the wire, never as an opaque string field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<World3dAnnotationLayer>,
    /// 🌡️ The scalar analysis field painted as a heatmap with an on-screen legend — a typed lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scalar_field: Option<World3dScalarField>,
    /// ⚙️ Pick granularity filter, section plane and sub-element highlight tokens — a typed lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modelling_options: Option<World3dModellingOptions>,
    /// 🪟️ The framework [`InteractionRef`]/`InteractionDefinition` id this world window is bound to,
    /// serialized as `domainId`. `None` leaves the window on the OS's own shared `world` board
    /// domain and plain plugin-private actions (`setHover`/`worldPick`/`worldSelect`). When set, a
    /// renderer routes its own instance pick/hover through the framework verbs
    /// `interactionSelect`/`interactionHover` on this domain. [`World3dScene::base`] leaves it unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// 🎯️ `domain_id`'s bound domain granularity id for a plain (non-component) instance pick/hover
    /// hit — `None` when `domain_id` is `None` or the domain has no plain-hit granularity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_granularity_id: Option<String>,
    /// 🛤️ On a scene SPINE (what [`SceneDoc::split_lanes`] leaves inside the fixed-capacity surface
    /// doc) this names every payload lane that rides outside it, with the byte length and content
    /// hash of each. Empty on an assembled scene — a renderer that reassembles the lanes back into
    /// their `*_json` fields leaves the list in place, and a producer that never publishes through a
    /// surface (the wgpu target builds `World3dScene` directly) never populates it.
    ///
    /// `bytes` is what lets a consumer tell a fully arrived lane from one still spread across
    /// reconcile pages; `hash` is what makes the spine itself change — and therefore the surface
    /// node's own `Component` differ — exactly when a lane's content changed, so an unchanged lane
    /// costs nothing on a partial refresh.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

scene_pack_wire!(World3dScenePack, World3dScene {
    snapshot: Option<World3dSnapshotLease>,
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    instances_delta_json: Option<String>,
    selection_json: String,
    vortices_json: Option<String>,
    attractions_json: Option<String>,
    target_volumes_json: Option<String>,
    references_json: Option<String>,
    brush_preview_json: Option<String>,
    interaction_json: Option<String>,
    engagement_preview_json: Option<String>,
    pick_targets_json: Option<String>,
    lod_json: Option<String>,
    presentation_json: Option<String>,
    chunking_json: Option<String>,
    environment_json: Option<String>,
    frame_json: Option<String>,
    fit_json: Option<String>,
    terrain_json: Option<String>,
    points_json: Option<String>,
    status_json: Option<String>,
    tool_run_trace: Option<String>,
    // 📏️ The typed modelling lanes never ride the pack: `SceneDoc::encode_pack` refuses a scene that still
    // carries one, and `split_lanes` moves them into their own carriers first.
    #[serde(skip)]
    annotations: Option<World3dAnnotationLayer>,
    #[serde(skip)]
    scalar_field: Option<World3dScalarField>,
    #[serde(skip)]
    modelling_options: Option<World3dModellingOptions>,
    domain_id: Option<String>,
    domain_granularity_id: Option<String>,
    // 🚚️ An assembled scene carries no manifest, and neither does a scene built by a producer that
    // never publishes through a surface — so a missing key decodes to "no lanes", exactly as serde
    // already treats every `Option` field above.
    #[serde(default)]
    lanes: Vec<SceneLaneRef>,
});

impl SceneDoc for World3dScene {
    const SCHEMA: &'static str = "world-3d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        if self.annotations.is_some() || self.scalar_field.is_some() || self.modelling_options.is_some() {
            return Err(PackError::Unsupported("typed modelling lanes ride scene lanes; split_lanes first"));
        }
        to_bytes(&World3dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes::<World3dScenePack>(bytes).map(Into::into)
    }

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in World3dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = World3dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload)
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn world3d_default_meshes_json() -> String {
    "[]".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn world3d_default_selection_json() -> String {
    r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into()
}

impl World3dScene {
    /** 🌐️ Builds a world-3d scene with optional extensions unset. */
    pub fn base(camera_json: String, meshes_json: String, instances_json: String, selection_json: String) -> Self {
        Self {
            snapshot: None,
            camera_json,
            meshes_json,
            instances_json,
            instances_delta_json: None,
            selection_json,
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            pick_targets_json: None,
            lod_json: None,
            presentation_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            tool_run_trace: None,
            annotations: None,
            scalar_field: None,
            modelling_options: None,
            domain_id: None,
            domain_granularity_id: None,
            lanes: Vec::new(),
        }
    }
}

impl ToValue for World3dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push_option(&mut entries, "snapshot", &self.snapshot);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "meshesJson", &self.meshes_json);
        value_push(&mut entries, "instancesJson", &self.instances_json);
        value_push_option(&mut entries, "instancesDeltaJson", &self.instances_delta_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "vorticesJson", &self.vortices_json);
        value_push_option(&mut entries, "attractionsJson", &self.attractions_json);
        value_push_option(&mut entries, "targetVolumesJson", &self.target_volumes_json);
        value_push_option(&mut entries, "referencesJson", &self.references_json);
        value_push_option(&mut entries, "brushPreviewJson", &self.brush_preview_json);
        value_push_option(&mut entries, "interactionJson", &self.interaction_json);
        value_push_option(&mut entries, "engagementPreviewJson", &self.engagement_preview_json);
        value_push_option(&mut entries, "pickTargetsJson", &self.pick_targets_json);
        value_push_option(&mut entries, "lodJson", &self.lod_json);
        value_push_option(&mut entries, "presentationJson", &self.presentation_json);
        value_push_option(&mut entries, "chunkingJson", &self.chunking_json);
        value_push_option(&mut entries, "environmentJson", &self.environment_json);
        value_push_option(&mut entries, "frameJson", &self.frame_json);
        value_push_option(&mut entries, "fitJson", &self.fit_json);
        value_push_option(&mut entries, "terrainJson", &self.terrain_json);
        value_push_option(&mut entries, "pointsJson", &self.points_json);
        value_push_option(&mut entries, "statusJson", &self.status_json);
        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);
        value_push_option(&mut entries, "annotations", &self.annotations);
        value_push_option(&mut entries, "scalarField", &self.scalar_field);
        value_push_option(&mut entries, "modellingOptions", &self.modelling_options);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        value_push_option(&mut entries, "domainGranularityId", &self.domain_granularity_id);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            snapshot: value_decode_option(&entries, "snapshot")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            meshes_json: value_decode_default(&entries, "meshesJson", world3d_default_meshes_json)?,
            instances_json: value_decode(&entries, "instancesJson")?,
            instances_delta_json: value_decode_option(&entries, "instancesDeltaJson")?,
            selection_json: value_decode_default(&entries, "selectionJson", world3d_default_selection_json)?,
            vortices_json: value_decode_option(&entries, "vorticesJson")?,
            attractions_json: value_decode_option(&entries, "attractionsJson")?,
            target_volumes_json: value_decode_option(&entries, "targetVolumesJson")?,
            references_json: value_decode_option(&entries, "referencesJson")?,
            brush_preview_json: value_decode_option(&entries, "brushPreviewJson")?,
            interaction_json: value_decode_option(&entries, "interactionJson")?,
            engagement_preview_json: value_decode_option(&entries, "engagementPreviewJson")?,
            pick_targets_json: value_decode_option(&entries, "pickTargetsJson")?,
            lod_json: value_decode_option(&entries, "lodJson")?,
            presentation_json: value_decode_option(&entries, "presentationJson")?,
            chunking_json: value_decode_option(&entries, "chunkingJson")?,
            environment_json: value_decode_option(&entries, "environmentJson")?,
            frame_json: value_decode_option(&entries, "frameJson")?,
            fit_json: value_decode_option(&entries, "fitJson")?,
            terrain_json: value_decode_option(&entries, "terrainJson")?,
            points_json: value_decode_option(&entries, "pointsJson")?,
            status_json: value_decode_option(&entries, "statusJson")?,
            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,
            annotations: value_decode_option(&entries, "annotations")?,
            scalar_field: value_decode_option(&entries, "scalarField")?,
            modelling_options: value_decode_option(&entries, "modellingOptions")?,
            domain_id: value_decode_option(&entries, "domainId")?,
            domain_granularity_id: value_decode_option(&entries, "domainGranularityId")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️World3dScene

//#region 🔖️World3dSceneLanes
/// 🎒️ The twenty-five world-3d payload fields that ride OUTSIDE the fixed-capacity surface doc, each as
/// its own retained, individually paged text carrier rooted at [`World3dSceneLane::body_key`].
///
/// Everything NOT in this list stays in the spine: `camera_json` (a ~120-byte per-frame descriptor
/// every consumer needs synchronously), the wgpu `snapshot` lease, the interaction `domain_id`/
/// `domain_granularity_id`, and the `lanes` manifest itself. Splitting is what removes the 32 KiB
/// `UiFixedBytes` ceiling from the scene as a whole (see [`SceneDoc::split_lanes`]); keeping the
/// lanes SEPARATE — rather than one big out-of-doc blob — is what lets the reconciler re-publish only
/// the lanes that actually changed, which matters because `instances`, `interaction` and `lod` have
/// very different change rates.
///
/// Mirrored in TypeScript by `WORLD3D_SCENE_LANES` in `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`. Both
/// sides are pinned against the one language-neutral declaration in
/// `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`, so neither can drift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSceneLane {
    Meshes,
    Instances,
    InstancesDelta,
    Selection,
    Vortices,
    Attractions,
    TargetVolumes,
    References,
    BrushPreview,
    Interaction,
    EngagementPreview,
    PickTargets,
    Lod,
    Presentation,
    Chunking,
    Environment,
    Frame,
    Fit,
    Terrain,
    Points,
    Status,
    ToolRunTrace,
    Annotations,
    ScalarField,
    ModellingOptions,
}

/// 🔑️ Reserved carrier-key namespace. Dotted and `framework.`-prefixed so a lane root can never
/// collide with an app-authored node id, exactly like `UI_REFRESH_SECTION_BODY_KEYS`.
pub const WORLD3D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.world3d.";

/// 🔤️ Wire name of each [`World3dSceneLane`], in `World3dSceneLane::ALL` order.
pub const WORLD3D_SCENE_LANE_NAMES: [&str; 25] = [
    "meshes",
    "instances",
    "instancesDelta",
    "selection",
    "vortices",
    "attractions",
    "targetVolumes",
    "references",
    "brushPreview",
    "interaction",
    "engagementPreview",
    "pickTargets",
    "lod",
    "presentation",
    "chunking",
    "environment",
    "frame",
    "fit",
    "terrain",
    "points",
    "status",
    "toolRunTrace",
    "annotations",
    "scalarField",
    "modellingOptions",
];

/// 📋️ [`World3dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const WORLD3D_SCENE_LANE_FIELDS: [&str; 25] = [
    "meshesJson",
    "instancesJson",
    "instancesDeltaJson",
    "selectionJson",
    "vorticesJson",
    "attractionsJson",
    "targetVolumesJson",
    "referencesJson",
    "brushPreviewJson",
    "interactionJson",
    "engagementPreviewJson",
    "pickTargetsJson",
    "lodJson",
    "presentationJson",
    "chunkingJson",
    "environmentJson",
    "frameJson",
    "fitJson",
    "terrainJson",
    "pointsJson",
    "statusJson",
    "toolRunTrace",
    "annotations",
    "scalarField",
    "modellingOptions",
];

/// 🗝️ Reserved carrier key of each lane — `WORLD3D_SCENE_LANE_KEY_PREFIX` + its name, spelled out
/// so the constant is greppable and pinnable rather than assembled at runtime.
pub const WORLD3D_SCENE_LANE_BODY_KEYS: [&str; 25] = [
    "framework.scene.world3d.meshes",
    "framework.scene.world3d.instances",
    "framework.scene.world3d.instancesDelta",
    "framework.scene.world3d.selection",
    "framework.scene.world3d.vortices",
    "framework.scene.world3d.attractions",
    "framework.scene.world3d.targetVolumes",
    "framework.scene.world3d.references",
    "framework.scene.world3d.brushPreview",
    "framework.scene.world3d.interaction",
    "framework.scene.world3d.engagementPreview",
    "framework.scene.world3d.pickTargets",
    "framework.scene.world3d.lod",
    "framework.scene.world3d.presentation",
    "framework.scene.world3d.chunking",
    "framework.scene.world3d.environment",
    "framework.scene.world3d.frame",
    "framework.scene.world3d.fit",
    "framework.scene.world3d.terrain",
    "framework.scene.world3d.points",
    "framework.scene.world3d.status",
    "framework.scene.world3d.toolRunTrace",
    "framework.scene.world3d.annotations",
    "framework.scene.world3d.scalarField",
    "framework.scene.world3d.modellingOptions",
];

/// ❓️ Whether each lane's [`World3dScene`] field is an `Option<String>` (`true`) rather than a plain
/// required `String` (`false`). A required lane always publishes — its empty payload is still a lane
/// — while an absent optional lane publishes no carrier at all.
pub const WORLD3D_SCENE_LANE_OPTIONAL: [bool; 25] = [false, false, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true];

impl World3dSceneLane {
    pub const ALL: [Self; 25] = [
        Self::Meshes,
        Self::Instances,
        Self::InstancesDelta,
        Self::Selection,
        Self::Vortices,
        Self::Attractions,
        Self::TargetVolumes,
        Self::References,
        Self::BrushPreview,
        Self::Interaction,
        Self::EngagementPreview,
        Self::PickTargets,
        Self::Lod,
        Self::Presentation,
        Self::Chunking,
        Self::Environment,
        Self::Frame,
        Self::Fit,
        Self::Terrain,
        Self::Points,
        Self::Status,
        Self::ToolRunTrace,
        Self::Annotations,
        Self::ScalarField,
        Self::ModellingOptions,
    ];

    /// 🏷️ See [`WORLD3D_SCENE_LANE_NAMES`].
    pub fn name(self) -> &'static str {
        WORLD3D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🔖️ See [`WORLD3D_SCENE_LANE_FIELDS`].
    pub fn field(self) -> &'static str {
        WORLD3D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`WORLD3D_SCENE_LANE_BODY_KEYS`].
    pub fn body_key(self) -> &'static str {
        WORLD3D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// ❔️ See [`WORLD3D_SCENE_LANE_OPTIONAL`].
    pub fn optional(self) -> bool {
        WORLD3D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to the lane it carries, `None` for every other node key.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// ↩️ Resolves a lane wire name back to its lane.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`, leaving the field empty/absent. `None` when an
    /// optional lane is unset — that lane publishes no carrier at all.
    pub fn take(self, scene: &mut World3dScene) -> Option<String> {
        match self {
            Self::Meshes => Some(std::mem::take(&mut scene.meshes_json)),
            Self::Instances => Some(std::mem::take(&mut scene.instances_json)),
            Self::InstancesDelta => scene.instances_delta_json.take(),
            Self::Selection => Some(std::mem::take(&mut scene.selection_json)),
            Self::Vortices => scene.vortices_json.take(),
            Self::Attractions => scene.attractions_json.take(),
            Self::TargetVolumes => scene.target_volumes_json.take(),
            Self::References => scene.references_json.take(),
            Self::BrushPreview => scene.brush_preview_json.clone(),
            Self::Interaction => scene.interaction_json.take(),
            Self::EngagementPreview => scene.engagement_preview_json.take(),
            Self::PickTargets => scene.pick_targets_json.take(),
            Self::Lod => scene.lod_json.take(),
            Self::Presentation => scene.presentation_json.take(),
            Self::Chunking => scene.chunking_json.take(),
            Self::Environment => scene.environment_json.take(),
            Self::Frame => scene.frame_json.take(),
            Self::Fit => scene.fit_json.take(),
            Self::Terrain => scene.terrain_json.take(),
            Self::Points => scene.points_json.take(),
            Self::Status => scene.status_json.take(),
            Self::ToolRunTrace => scene.tool_run_trace.take(),
            Self::Annotations => scene.annotations.take().map(|layer| world3d_modelling_lane_text(&layer)),
            Self::ScalarField => scene.scalar_field.take().map(|field| world3d_modelling_lane_text(&field)),
            Self::ModellingOptions => scene.modelling_options.take().map(|options| world3d_modelling_lane_text(&options)),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`World3dSceneLane::take`].
    /// A typed lane whose payload the schema or its semantic rules refuse leaves the scene unchanged
    /// and returns `false`; every `*_json` lane always returns `true`.
    pub fn put(self, scene: &mut World3dScene, payload: String) -> bool {
        match self {
            Self::Meshes => scene.meshes_json = payload,
            Self::Instances => scene.instances_json = payload,
            Self::InstancesDelta => scene.instances_delta_json = Some(payload),
            Self::Selection => scene.selection_json = payload,
            Self::Vortices => scene.vortices_json = Some(payload),
            Self::Attractions => scene.attractions_json = Some(payload),
            Self::TargetVolumes => scene.target_volumes_json = Some(payload),
            Self::References => scene.references_json = Some(payload),
            Self::BrushPreview => scene.brush_preview_json = Some(payload),
            Self::Interaction => scene.interaction_json = Some(payload),
            Self::EngagementPreview => scene.engagement_preview_json = Some(payload),
            Self::PickTargets => scene.pick_targets_json = Some(payload),
            Self::Lod => scene.lod_json = Some(payload),
            Self::Presentation => scene.presentation_json = Some(payload),
            Self::Chunking => scene.chunking_json = Some(payload),
            Self::Environment => scene.environment_json = Some(payload),
            Self::Frame => scene.frame_json = Some(payload),
            Self::Fit => scene.fit_json = Some(payload),
            Self::Terrain => scene.terrain_json = Some(payload),
            Self::Points => scene.points_json = Some(payload),
            Self::Status => scene.status_json = Some(payload),
            Self::ToolRunTrace => scene.tool_run_trace = Some(payload),
            Self::Annotations => match world3d_modelling_from_lane_text(&payload) {
                Ok(layer) => scene.annotations = Some(layer),
                Err(_) => return false,
            },
            Self::ScalarField => match world3d_modelling_from_lane_text(&payload) {
                Ok(field) => scene.scalar_field = Some(field),
                Err(_) => return false,
            },
            Self::ModellingOptions => match world3d_modelling_from_lane_text(&payload) {
                Ok(options) => scene.modelling_options = Some(options),
                Err(_) => return false,
            },
        }
        true
    }
}

//#endregion 🔖️World3dSceneLanes

//#region 🔖️ModellingBuilders
impl World3dScene {
    /// 🥪️ Sets the annotation layer lane; refuses an invalid layer.
    pub fn with_annotations(mut self, layer: World3dAnnotationLayer) -> Result<Self, ValueError> {
        layer.validate()?;
        self.annotations = Some(layer);
        Ok(self)
    }

    /// 🎚️ Sets the scalar field lane; refuses an invalid field.
    pub fn with_scalar_field(mut self, field: World3dScalarField) -> Result<Self, ValueError> {
        field.validate()?;
        self.scalar_field = Some(field);
        Ok(self)
    }

    /// 🔧️ Replaces the modelling options lane; refuses invalid options.
    pub fn with_modelling_options(mut self, options: World3dModellingOptions) -> Result<Self, ValueError> {
        options.validate()?;
        self.modelling_options = Some(options);
        Ok(self)
    }

    /// 🔦️ Restricts hover and selection to one granularity.
    pub fn with_pick_filter(mut self, filter: World3dPickGranularity) -> Self {
        self.modelling_options.get_or_insert_with(Default::default).pick_filter = Some(filter);
        self
    }

    /// ✂️ Sets the section plane; refuses a degenerate plane.
    pub fn with_section(mut self, section: World3dSection) -> Result<Self, ValueError> {
        section.validate()?;
        self.modelling_options.get_or_insert_with(Default::default).section = Some(section);
        Ok(self)
    }

    /// 🖍️ Sets the sub-element highlight tokens; refuses an out-of-range width.
    pub fn with_highlight(mut self, highlight: World3dHighlight) -> Result<Self, ValueError> {
        for style in [highlight.face, highlight.edge, highlight.vertex].into_iter().flatten() {
            style.validate()?;
        }
        self.modelling_options.get_or_insert_with(Default::default).highlight = Some(highlight);
        Ok(self)
    }
}
//#endregion 🔖️ModellingBuilders

#[cfg(test)]
#[path = "../../🧪️tests/🔬️world3d-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️world3d-value-round-trip/🦀️.rs"]
mod value_round_trip_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️world3d-modelling-scene-unit/🦀️.rs"]
mod modelling_scene_tests;
