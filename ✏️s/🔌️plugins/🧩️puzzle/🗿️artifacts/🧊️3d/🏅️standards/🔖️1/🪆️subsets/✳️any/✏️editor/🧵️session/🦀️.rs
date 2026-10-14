//! 🧵️ Puzzle 3d editing session — one document instance's resident state (precompute engine, instance residency, mesh
//! lane), the process-global session registry that keeps it across calls, the action context every command arm reads,
//! the engine bridge from the transient scene to the precompute session, and the context menu.

use {crate::editor::puzzle3d::snapshot::PUZZLE3D_FALLBACK_MESH_KIND, crate::editor::puzzle3d::snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, crate::editor::puzzle3d::snapshot::PUZZLE3D_GRANULARITY_OBJECT, crate::editor::puzzle3d::snapshot::PUZZLE3D_GRANULARITY_REFERENCE, crate::editor::puzzle3d::snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, crate::editor::puzzle3d::snapshot::PUZZLE3D_GRANULARITY_VORTEX, crate::editor::puzzle3d::snapshot::PUZZLE3D_INTERACTION_DOMAIN, crate::editor::puzzle3d::snapshot::PUZZLE3D_LOCALIZATION_UNSUPPORTED, crate::editor::puzzle3d::snapshot::Puzzle3dAttraction, crate::editor::puzzle3d::snapshot::Puzzle3dInteractionSnapshot, crate::editor::puzzle3d::snapshot::Puzzle3dObject, crate::editor::puzzle3d::snapshot::Puzzle3dScene, crate::editor::puzzle3d::snapshot::Puzzle3dSceneSnapshot, crate::editor::puzzle3d::snapshot::Puzzle3dTargetVolume, crate::editor::puzzle3d::snapshot::Puzzle3dVortex, crate::editor::puzzle3d::snapshot::collect_mesh_urls, crate::editor::puzzle3d::snapshot::mesh_selection_ids};
use {crate::editor::puzzle3d::config::Puzzle3dConfig, crate::editor::puzzle3d::config::Puzzle3dRuntime};
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use {crate::editor::puzzle3d::precompute::brush::BrushSuggestionsLink, crate::editor::puzzle3d::precompute::brush::BrushSuggestionsOwner};
use {crate::editor::puzzle3d::precompute::Puzzle3dCollisionSession, crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession};
use {crate::editor::puzzle3d::terminology::puzzle3d_labels, crate::editor::puzzle3d::terminology::Puzzle3dLabels};
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::standards::v1::subsets::any::schema::Puzzle3dEngineCommand;
use crate::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use {semio_framework_job::InteractiveJobCloseStep, semio_framework_job::StepContext};
use semio_framework_plugin::mesh_from_kind;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::MergeMode;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use std::collections::HashMap;
use {std::sync::Mutex, std::sync::OnceLock};
use crate::editor::puzzle3d::snapshot;

/// 🎯️ Bridges this app's own document model into `⚙️engine`'s `SceneConfig` wire shape — schema
/// translation between two independently-evolved Rust types, not a wasm-bindgen boundary. Built
/// TYPED, field by field: routing the whole document through a `dsl::DslValue` tree and back cost a
/// measured 3.5 ms per sync on the 180-object Nakagin document (0.5 ms to build the tree, 3.0 ms to
/// re-parse it), four times this artifact's own per-step budget, for a translation whose every field
/// is a move or a copy. Only the two genuinely untyped members of `Puzzle3dSceneMetadata`
/// (`kindCatalogs`/`kindCompatibility`, which this app's document model keeps as raw `DslValue`)
/// still decode through `FromValue`, and a malformed one refuses the whole scene exactly as the
/// all-`FromValue` shape did. `#[cfg(test)] scene_config_value` below is the differential oracle this
/// is checked against.
pub(crate) fn scene_config(envelope: &Puzzle3dScene) -> Option<crate::standards::v1::subsets::any::schema::SceneConfig> {
    let meta = &envelope.scene_snapshot.meta;
    let kind_catalogs = match meta.kind_catalogs.as_ref() {
        None | Some(semio_framework_value::DslValue::Null) => None,
        Some(catalogs) => Some(FromValue::from_value(catalogs.clone()).ok()?),
    };
    let kind_compatibility = match meta.kind_compatibility.as_ref() {
        None => Vec::new(),
        Some(entries) => FromValue::from_value(entries.clone()).ok()?,
    };
    Some(crate::standards::v1::subsets::any::schema::SceneConfig {
        scene_snapshot: crate::standards::v1::subsets::any::schema::EngineSceneSnapshot {
            objects: envelope.scene_snapshot.objects.iter().map(engine_scene_object).collect(),
            attractions: envelope.scene_snapshot.attractions.iter().map(engine_attraction_props).collect(),
            target_volumes: envelope.scene_snapshot.target_volumes.iter().map(engine_world_volume_props).collect(),
        },
        kind_catalogs,
        kind_compatibility,
        contact_tolerance: envelope.runtime.contact_tolerance,
        seed: 1,
        host_rules: crate::standards::v1::subsets::any::schema::BrushHostRules::default(),
        weights: crate::standards::v1::subsets::any::schema::BrushKindWeights {
            object_weights: envelope.runtime.object_kind_weights.iter().map(|(kind, weight)| (kind.clone(), *weight)).collect(),
            vortex_weights: envelope.runtime.vortex_kind_weights.iter().map(|(kind, weight)| (kind.clone(), *weight)).collect(),
        },
    })
}

/// 🧱️ One document object as the engine sees it. `anchor` has no counterpart in this app's own
/// document model, so it takes the same default the JSON bridge gave it.
fn engine_scene_object(object: &Puzzle3dObject) -> crate::standards::v1::subsets::any::schema::EngineSceneObject {
    crate::standards::v1::subsets::any::schema::EngineSceneObject {
        id: object.id.clone(),
        object_kind: object.object_kind.clone(),
        anchor: crate::standards::v1::subsets::any::schema::Puzzle3dObjectAnchor::default(),
        mesh_url: object.mesh_url.clone(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.clone(),
        vortices: object.vortices.iter().map(engine_vortex_props).collect(),
    }
}

fn engine_vortex_props(vortex: &Puzzle3dVortex) -> crate::standards::v1::subsets::any::schema::VortexProps {
    crate::standards::v1::subsets::any::schema::VortexProps { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), position: vortex.position, direction: vortex.direction }
}

/// 🔗️ One attraction as the engine sees it. `x`/`y` are diagram-only coordinates this app's own
/// document model does not carry, so they take the same default the JSON bridge gave them.
fn engine_attraction_props(attraction: &Puzzle3dAttraction) -> crate::standards::v1::subsets::any::schema::AttractionProps {
    crate::standards::v1::subsets::any::schema::AttractionProps {
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
        x: 0.0,
        y: 0.0,
    }
}

fn engine_world_volume_props(volume: &Puzzle3dTargetVolume) -> crate::standards::v1::subsets::any::schema::WorldVolumeProps {
    crate::standards::v1::subsets::any::schema::WorldVolumeProps { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.clone() }
}

/// 🌉️ The all-`DslValue` bridge `scene_config` replaced, retained as its differential oracle: the
/// derived `ToValue`/`FromValue` machinery is an independent implementation of the same translation,
/// so `puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example` can prove the typed
/// construction agrees with it on every shipped document.
#[cfg(test)]
pub(crate) fn scene_config_value(envelope: &Puzzle3dScene) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([
        (
            "sceneSnapshot".to_string(),
            semio_framework_value::DslValue::object([
                ("objects".to_string(), semio_framework_value::ToValue::to_value(&envelope.scene_snapshot.objects)),
                ("attractions".to_string(), semio_framework_value::ToValue::to_value(&envelope.scene_snapshot.attractions)),
                ("targetVolumes".to_string(), semio_framework_value::ToValue::to_value(&envelope.scene_snapshot.target_volumes)),
            ]),
        ),
        ("kindCatalogs".to_string(), envelope.scene_snapshot.meta.kind_catalogs.clone().unwrap_or(semio_framework_value::DslValue::Null)),
        ("kindCompatibility".to_string(), envelope.scene_snapshot.meta.kind_compatibility.clone().unwrap_or_else(|| semio_framework_value::DslValue::Array(Vec::new()))),
        ("contactTolerance".to_string(), semio_framework_value::DslValue::float(envelope.runtime.contact_tolerance)),
        ("seed".to_string(), semio_framework_value::DslValue::uint(1)),
        ("weights".to_string(), semio_framework_value::DslValue::object([("objectWeights".to_string(), semio_framework_value::ToValue::to_value(&envelope.runtime.object_kind_weights)), ("vortexWeights".to_string(), semio_framework_value::ToValue::to_value(&envelope.runtime.vortex_kind_weights))])),
    ])
}

/// 🧊️ Scales the unit box fallback (`mesh_from_kind` extent 1.0) past the collision engine's minimum
/// brush mesh extent (2.0), otherwise its registration is a silent no-operation and brush candidates
/// never populate before a real GLB arrives.
const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

/// 🧊️ The scaled box every mesh identity collides as until its real geometry is installed.
pub(crate) fn puzzle3d_fallback_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    (fallback.positions.iter().map(|value| value * PUZZLE3D_FALLBACK_MESH_SCALE).collect(), fallback.indices)
}

/// 🧊️ Pushes the current scene into the precompute session. The engine compares the decoded scene
/// structurally and no-operates on a resync, so this is the one call a caller makes per scene — and it
/// is deliberately the LAST half of a sync: installing a mesh identity rebuilds the fill preparation
/// and the brush queue against whatever scene is installed, so seeding first and pushing once costs one
/// rebuild instead of one per mesh (a measured 0.6–4.4 ms each, twelve of them on Nakagin).
pub fn sync_precompute_scene(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    if let Some(scene) = scene_config(envelope) {
        push_precompute_scene(session, scene);
    }
}

/// 🧊️ Installs an already-built engine scene. Split from `scene_config` so a staged caller pays the
/// document-shaped translation (a measured 1.3 ms on Nakagin) and the engine's own scene install (0.8 ms,
/// it rebuilds the fill preparation and re-arms the brush broad phase) in two separate bounded turns.
pub fn push_precompute_scene(session: &mut Puzzle3dPrecomputeSession, scene: crate::standards::v1::subsets::any::schema::SceneConfig) {
    let _ = session.dispatch(Puzzle3dEngineCommand::SetScene { scene });
}

/// 🥽️ Seeds the scaled box fallback for ONE mesh identity the session holds no geometry for yet, so
/// a real GLB registered earlier via `registerBrushMesh` survives every resync. Answers whether it
/// registered one, i.e. whether the caller should come back for the next; one registration costs a
/// measured 0.6 ms and the 180-object Nakagin document carries twelve of them, so a staged caller
/// spends one bounded turn on each instead of 8 ms on all of them at once.
pub fn seed_one_precompute_mesh_fallback(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) -> bool {
    let (fallback_positions, fallback_indices) = puzzle3d_fallback_mesh_buffers();
    if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
        session.register_mesh_fallback(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback_indices);
        return true;
    }
    let Some(url) = collect_mesh_urls(&envelope.scene_snapshot).into_iter().find(|url| !session.has_mesh(url)) else {
        return false;
    };
    session.register_mesh_fallback(&url, &fallback_positions, &fallback_indices);
    true
}

/// 🧊️ The whole sync, for the callers that are not themselves step-bounded (render, the restored
/// session, the brush lane driver): every owed mesh fallback, then the scene.
pub fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    while seed_one_precompute_mesh_fallback(session, envelope) {}
    sync_precompute_scene(session, envelope);
}

pub fn sync_precompute_weights(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    let object_weights = envelope.runtime.object_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let vortex_weights = envelope.runtime.vortex_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let _ = session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights });
}

/// ⏱️ Bounded to one small chunk per call — `handle` runs synchronously on the UI thread, so a large
/// per-call budget here is exactly what froze the UI: hundreds of Monte-Carlo collision task units, blocking.
pub fn drive_precompute(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    sync_precompute_session(session, envelope);
    session.precompute_step(8);
}

/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (window-option materialization, precompute sync, chrome effects, delta computation, config
/// snapshotting) stays in [`crate::editor::puzzle3d::dispatch::puzzle3d_handle_action`]; an arm only mutates this bundle.
pub struct Puzzle3dActionCtx<'a> {
    /// 🧠️ The app's long-lived precompute session and gumball scratch — every arm reaching them goes
    /// through `borrow_mut()`.
    pub session: &'a Puzzle3dSession,
    /// 🧠️ The document instance's retained operation owner — the brush suggestions link lives there. `None` on
    /// the unstaged adapter paths that run outside an admitted instance.
    pub instance_owner: Option<&'a ArtifactInstanceOperationOwnerHandle>,
    /// ⏯️ The instance's tool run as of command admission (`ArtifactOwnedToolJobContext::tool_run`) — the
    /// identity a run action this arm dispatches targets. `None` on paths admitted without it.
    pub tool_run: Option<&'a semio_framework_plugin::ToolRunView>,
    pub scene: &'a mut Puzzle3dScene,
    /// 🪟️ The window instance this action targets (already defaulted to the main window).
    pub window_id: &'a str,
    /// 🎛️ The pre-action config snapshot, for the few arms that must read state the scene runtime's
    /// materialized copy does not carry.
    pub config: &'a Puzzle3dRuntime,
    pub view_state: Option<&'a semio_framework_plugin::ViewModel>,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (current selection plus
    /// `"pointer"` hover — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Retained
    /// selection-acting verbs (delete/duplicate/focus/rotate/scale/translate-selection,
    /// select-same-kind, set-selection-flag, engagement-control-select) read it through the
    /// `selected_*_ids` helpers below instead of the deleted `Puzzle3dConfig` selection fields.
    pub interaction: &'a Puzzle3dInteractionSnapshot,
    pub ui_scope: &'a mut UiDirtyScope,
    pub effects: &'a mut Vec<Effect>,
    /// 🕹️ App-initiated selection writes this arm wants applied once its own mutations land — the
    /// single sanctioned way a reducer selects what it just created or widened to
    /// (`semio_framework_plugin::InteractionWrite`, applied by `VcsArtifactApp` through the same
    /// state machine the reserved `interactionSelect` verb uses). Empty for every arm that does not
    /// move the selection.
    pub interaction_writes: &'a mut Vec<InteractionWrite>,
    /// 🛑️ Set by an arm that must skip the whole epilogue (window save, delta, config snapshot).
    pub abort: bool,
    /// 🧬️ The typed document this action reads as its base — what the transform tool yields against.
    pub base: &'a std::sync::Arc<Puzzle3dSnapshot>,
    /// 🌱️ The admission's authoring seed every tool transaction this action commits is minted from; empty for
    /// a view without command authority, which then publishes the yielded mutations plainly.
    pub authoring_seed: &'a str,
    /// 🛠️ The mutations a committed tool transaction yielded — published after the scene delta.
    pub artifact_mutations: &'a mut Vec<Puzzle3dMutation>,
    /// 🛠️ The tool transaction this action committed — stamped on every op it publishes.
    pub transaction: &'a mut Option<protocol::TransactionRef>,
}

impl<'a> Puzzle3dActionCtx<'a> {
    /// 🔗️ Runs `apply` on this instance's brush suggestions link; `None` without an instance owner or while
    /// the owner is busy.
    pub fn brush_suggestions<R>(&self, apply: impl FnOnce(&mut BrushSuggestionsLink) -> R) -> Option<R> {
        self.instance_owner?.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(apply(&mut owner.brush_suggestions))).ok()
    }

    pub fn selected_object_ids(&self) -> Vec<String> {
        self.interaction.selected_object_ids().to_vec()
    }
    pub fn selected_vortex_ids(&self) -> Vec<String> {
        self.interaction.selected_vortex_ids().to_vec()
    }
    pub fn selected_attraction_ids(&self) -> Vec<String> {
        self.interaction.selected_attraction_ids().to_vec()
    }
    pub fn selected_target_volume_ids(&self) -> Vec<String> {
        self.interaction.selected_target_volume_ids().to_vec()
    }
    pub fn selected_reference_ids(&self) -> Vec<String> {
        self.interaction.selected_reference_ids().to_vec()
    }

    /// 🕹️ Replaces this app's whole `vortex` selection with `ids` at `granularity` once the action's
    /// own mutations have landed — "select the thing I just created/widened to".
    pub fn replace_selection(&mut self, granularity: &str, ids: impl IntoIterator<Item = String>) {
        let write = InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, granularity, ids);
        if !write.targets.is_empty() {
            self.interaction_writes.push(write);
        }
    }

    /// 🧹️ Empties this app's whole `vortex` selection through the same sanctioned reducer channel
    /// [`Self::replace_selection`] uses. It is expressed as a `Subtractive` write naming exactly what
    /// is selected right now, NOT an empty `Replace`: the framework's state machine returns the
    /// current selection unchanged when a write names no target at all (`protocol::next_selection`),
    /// so an empty `Replace` is a silent no-op. Nothing selected means nothing to clear.
    pub fn clear_selection(&mut self) {
        let targets: Vec<InteractionTarget> = [
            (PUZZLE3D_GRANULARITY_OBJECT, self.selected_object_ids()),
            (PUZZLE3D_GRANULARITY_VORTEX, self.selected_vortex_ids()),
            (PUZZLE3D_GRANULARITY_ATTRACTION, self.selected_attraction_ids()),
            (PUZZLE3D_GRANULARITY_TARGET_VOLUME, self.selected_target_volume_ids()),
            (PUZZLE3D_GRANULARITY_REFERENCE, self.selected_reference_ids()),
        ]
        .into_iter()
        .flat_map(|(granularity, ids)| ids.into_iter().map(move |id| InteractionTarget { granularity: granularity.to_string(), id }))
        .collect();
        if targets.is_empty() {
            return;
        }
        self.interaction_writes.push(InteractionWrite { domain: PUZZLE3D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
    }

    /// 🧯️ Surfaces ONE bounded, localized notice through the shell's transient-notice channel
    /// (`Effect::Notify` → `ShellHost`'s `showTransientNotice`) — how an arm whose engine work was
    /// REJECTED tells the user, instead of the silence
    /// `addBrushObject`/`acceptSuggestion` used to fall through (`📓️2026-09-09-user-feature-
    /// checklist.md` §9/§13). At most one notice per action: a second call replaces nothing and adds
    /// nothing, so the effect list stays fixed-width. An unauthored locale×terminology axis carries
    /// the app's own `ui.localization.unsupported` code rather than an English sentence — this UI has
    /// no default language, and a silent drop would restore exactly the defect being fixed.
    /// 🎯️ Refuses a selection-scoped command that has nothing to act on: ONE localized notice, then the
    /// same `abort` `select_same_kind` already uses, so no empty document edit and no empty interaction
    /// write is emitted. Answers whether it refused, so an arm reads
    /// `if ctx.refuse_without_selection(&ids) { return }`.
    ///
    /// 🕹️ Why this exists: every one of these arms used to run to completion over an EMPTY id list —
    /// `duplicate_selection` cloned nothing and then called `replace_selection` with no targets,
    /// `translate/rotate/scale_selection` moved nothing, `delete_selection` retained everything. The
    /// emitted delta was empty, so the framework logged no command row and the outliner never changed:
    /// in the browser "Duplicate Selection" was indistinguishable from a dead menu row (measured
    /// 2026-09-09 21:05, no notice of any kind). A refusal must be visible.
    pub fn refuse_without_selection(&mut self, ids: &[String]) -> bool {
        if !ids.is_empty() {
            return false;
        }
        self.notice(|labels| labels.nothing_selected.as_str());
        self.abort = true;
        true
    }

    pub fn refuse_when_locked(&mut self) -> bool {
        self.notice(|labels| labels.selection_locked.as_str());
        self.abort = true;
        true
    }

    /// 🧲️ One gumball pose delta (`translateSelection`/`rotateSelection`/`scaleSelection`) as ONE tool
    /// transaction over the gesture's own object ids (else the selected objects) and the selected target volumes.
    pub fn commit_gumball(&mut self, verb: &str, args: Option<&Value>) {
        let targets = [mesh_selection_ids(args, &self.selected_object_ids()), self.selected_target_volume_ids()].concat();
        if let Some(record) = utilities::transform::Puzzle3dSelectionRecord::from_gumball(verb, args, targets) {
            self.commit_selection(verb, vec![record]);
        }
    }

    /// 🛠️ Commits `records` through the transform tool machine as ONE tool transaction of this action — the
    /// parametric selection leaves plus the attractions their drops land, yielded as `verb`. Nothing named is
    /// the `nothing_selected` refusal; a request whose every target is locked is the `selection_locked` one;
    /// a request with a movable target is yielded whole, and its leaf reports the locked rest as
    /// `mutation.partial`. A motionless request leaves zero trace.
    pub fn commit_selection(&mut self, verb: &str, records: Vec<utilities::transform::Puzzle3dSelectionRecord>) {
        if records.iter().all(|record| record.targets.is_empty()) {
            self.refuse_without_selection(&[]);
            return;
        }
        if records.iter().any(|record| record.refused_as_locked(self.base)) && !records.iter().any(|record| record.applies_to(self.base)) {
            self.refuse_when_locked();
            return;
        }
        let request = utilities::transform::TransformToolRequest { base: std::sync::Arc::clone(self.base), records };
        if let Some((transaction, mutations)) = utilities::transform::puzzle3d_transform_tool_commit(verb, self.authoring_seed, request) {
            *self.transaction = (!self.authoring_seed.is_empty()).then_some(transaction);
            self.artifact_mutations.extend(mutations);
        }
    }

    pub fn notice(&mut self, message: impl Fn(&Puzzle3dLabels) -> &'static str) {
        if self.effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })) {
            return;
        }
        let text = self.view_state.and_then(puzzle3d_labels).map_or_else(|| PUZZLE3D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
        self.effects.push(Effect::Notify { message: text });
    }
}

/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d label admission failed"))
}

/// 🖱️ Bespoke row builder — every row here carries a localized (`Puzzle3dLabels`) label/icon that the
/// declared `ActionDefinition` (English-only) cannot resolve, so each row is emitted via `Menu::item`
/// rather than `Menu::action`. Grouping/ordering/the pre-destructive separator are still handled by
/// `Menu::group` + the `organize_context_menu` funnel in `context_menu`.
fn puzzle3d_context_menu_row(id: &str, label: impl Into<String>, icon: &str, action: &str, args: Option<Value>, destructive: bool) -> semio_framework_plugin::ContextMenuItemSpec {
    semio_framework_plugin::ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)),
        destructive: destructive.then_some(true),
        ..Default::default()
    }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-granularity ids
/// `ContextMenuRequest.surface.selection` carries — the CLIENT-supplied, always-available substitute
/// for `runtime.selection` at context-menu time (unlike `render`, `context_menu` never had a live
/// config selection to read even before this ticket; `ContextMenuSurfaceTarget.selection` is the
/// framework's own sanctioned channel for it).
#[derive(Default)]
struct Puzzle3dContextSelection {
    object_ids: Vec<String>,
    vortex_ids: Vec<String>,
    attraction_ids: Vec<String>,
    target_volume_ids: Vec<String>,
    reference_ids: Vec<String>,
    /// 🎯️ The pointer's hit alone is the subject, so no selection read may widen it again.
    hit_subject: bool,
}

impl Puzzle3dContextSelection {
    fn bucket(&mut self, domain: &str) -> Option<&mut Vec<String>> {
        match domain {
            "node" | PUZZLE3D_GRANULARITY_OBJECT => Some(&mut self.object_ids),
            PUZZLE3D_GRANULARITY_VORTEX => Some(&mut self.vortex_ids),
            PUZZLE3D_GRANULARITY_ATTRACTION => Some(&mut self.attraction_ids),
            PUZZLE3D_GRANULARITY_TARGET_VOLUME => Some(&mut self.target_volume_ids),
            PUZZLE3D_GRANULARITY_REFERENCE => Some(&mut self.reference_ids),
            _ => None,
        }
    }

    /// 🎯️ `surface.hits` is the entity the pointer is actually over, `surface.selection` the entities the
    /// document already holds selected. A hit OUTSIDE the selection is the menu's whole subject, so a
    /// right-click on an unselected object opens that object's menu instead of the empty menu that made the
    /// shell fallback take over the viewport (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 8), and a
    /// right-click on a vortex of the selected object opens the vortex's menu, not the object's. A hit
    /// inside the current selection changes nothing — the whole selection stays the menu's subject, so
    /// "Delete (3 objects)" never silently narrows to the one row under the cursor.
    fn from_surface(surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Self {
        let mut out = Self::default();
        let Some(surface) = surface else {
            return out;
        };
        for group in &surface.selection {
            let ids = group.ids.clone();
            if let Some(bucket) = out.bucket(group.domain.as_str()) {
                bucket.extend(ids);
            }
        }
        // 🎯️ Hosts list every target under the pointer, most specific first, and only that one is the subject.
        let mut hits = Self::default();
        if let Some(hit) = surface.hits.iter().find(|hit| hits.bucket(hit.domain.as_str()).is_some()) {
            hits.bucket(hit.domain.as_str()).into_iter().for_each(|bucket| bucket.push(hit.id.clone()));
        }
        let hit_selected = |selected: &[String], hit: &[String]| hit.iter().all(|id| selected.contains(id));
        let inside = hit_selected(&out.object_ids, &hits.object_ids)
            && hit_selected(&out.vortex_ids, &hits.vortex_ids)
            && hit_selected(&out.attraction_ids, &hits.attraction_ids)
            && hit_selected(&out.target_volume_ids, &hits.target_volume_ids)
            && hit_selected(&out.reference_ids, &hits.reference_ids);
        if surface.hits.is_empty() || inside {
            return out;
        }
        hits.hit_subject = true;
        hits
    }

    /// 🕹️ Fills in the granularities the CLIENT surface never sends. `World3dHost` only puts its
    /// painted object ids (domain `"object"`) and component ids (`"feature"`) into
    /// `ContextMenuSurfaceTarget.selection`, so a selected vortex / attraction / target volume /
    /// reference reaches a menu only through the framework-owned domain read. Per-granularity
    /// additive: whatever the surface DID supply keeps priority (a right-click on an unselected
    /// entity still targets what was clicked).
    fn fill_from_interaction(&mut self, interaction: &Puzzle3dInteractionSnapshot) {
        if self.hit_subject {
            return;
        }
        for (bucket, ids) in [
            (&mut self.object_ids, interaction.selected_object_ids()),
            (&mut self.vortex_ids, interaction.selected_vortex_ids()),
            (&mut self.attraction_ids, interaction.selected_attraction_ids()),
            (&mut self.target_volume_ids, interaction.selected_target_volume_ids()),
            (&mut self.reference_ids, interaction.selected_reference_ids()),
        ] {
            if bucket.is_empty() {
                bucket.extend(ids.iter().cloned());
            }
        }
    }
}

fn puzzle3d_context_menu_items(envelope: &Puzzle3dScene, selection: &Puzzle3dContextSelection, labels: &Puzzle3dLabels, view_state: &semio_framework_plugin::ViewModel, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::Menu;
    if !selection.object_ids.is_empty() {
        let all_hidden = envelope.scene_snapshot.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.hidden);
        let all_locked = envelope.scene_snapshot.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.locked);
        let count = selection.object_ids.len();
        let phrase = if count == 1 { format!("1 {}", labels.object.as_str()) } else { format!("{count} {}", labels.objects.as_str()) };
        return {
            Menu::of(registry, view_state)
                .item(puzzle3d_context_menu_row("duplicate", labels.duplicate, "copy", "duplicateSelection", None, false))
                .item(puzzle3d_context_menu_row("select-same-kind", labels.select_same_kind, "layers", "selectSameKindSelection", None, false))
                .item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false))
                .group("hand", |m| {
                    m.item(puzzle3d_context_menu_row("hide-show", if all_hidden { labels.show } else { labels.hide }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": !all_hidden })), false))
                        .item(puzzle3d_context_menu_row(
                            "lock-unlock",
                            if all_locked { labels.unlock } else { labels.lock },
                            if all_locked { "lock-open" } else { "lock" },
                            "setSelectionFlag",
                            Some(json!({ "flag": "locked", "value": !all_locked })),
                            false,
                        ))
                })
                .item(puzzle3d_context_menu_row("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
                .build()
        };
    }
    if !selection.vortex_ids.is_empty() {
        let mut menu = Menu::of(registry, view_state);
        if let [only] = selection.vortex_ids.as_slice() {
            menu = menu.item(puzzle3d_context_menu_row("suggest", labels.suggest_objects, "sparkles", "openVortexSuggestions", Some(json!({ "fullId": only.as_str() })), false));
        }
        return menu.item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false)).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).build();
    }
    if let Some(id) = selection.attraction_ids.first() {
        return Menu::of(registry, view_state).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteAttraction", Some(json!({ "id": id.as_str() })), true)).build();
    }
    if let Some(id) = selection.target_volume_ids.first() {
        let target_volume = envelope.scene_snapshot.target_volumes.iter().find(|volume| &volume.id == id);
        let hidden = target_volume.is_some_and(|volume| volume.hidden);
        let locked = target_volume.is_some_and(|volume| volume.locked);
        return {
            Menu::of(registry, view_state)
                .group("targets", |m| {
                    m.item(puzzle3d_context_menu_row(
                        "hide-show",
                        if hidden { labels.show } else { labels.hide },
                        if hidden { "eye" } else { "eye-off" },
                        "setTargetVolumeFlag",
                        Some(json!({ "id": id.as_str(), "flag": "hidden", "value": !hidden })),
                        false,
                    ))
                    .item(puzzle3d_context_menu_row(
                        "lock-unlock",
                        if locked { labels.unlock } else { labels.lock },
                        if locked { "lock-open" } else { "lock" },
                        "setTargetVolumeFlag",
                        Some(json!({ "id": id.as_str(), "flag": "locked", "value": !locked })),
                        false,
                    ))
                })
                .item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteTargetVolume", Some(json!({ "id": id.as_str() })), true))
                .build()
        };
    }
    if !selection.reference_ids.is_empty() {
        return {
            Menu::of(registry, view_state).item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false)).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).build()
        };
    }
    Vec::new()
}

// 🧩️ Puzzle-3d play app. Owns the precompute engine; the persisted document (bare `Puzzle3dSceneSnapshot`
// json) lives in the wrapping `VcsArtifactApp`'s operation store and the view state in
// `Puzzle3dConfig`. Each action rehydrates the engine from the projection, mutates a transient
// [`Puzzle3dScene`], then emits the granular operation delta.
//
// 🧲️ Gumball drags carry NO app-side session: both hosts paint the drag locally and dispatch exactly
// ONE start→end pose delta (`translateSelection`/`rotateSelection`/`scaleSelection`) on release, which the
// transform tool turns into ONE `ToolTransaction` of relative, parametric selection leaves.

/// 🎟️ Document instances that may hold a live cache slot at once. Sized for real desktop use — a dozen
/// open documents across split panes, with headroom — and deliberately unrelated to
/// any fixed owner page.
const PUZZLE3D_SESSION_SLOTS: usize = 64;

/// 🎯️ Slots probed for one instance id before the registry gives up or evicts, mirroring
/// `FillEnvelopeRegistry::begin_measurement`'s bounded candidate walk.
const PUZZLE3D_SESSION_PROBES: usize = 4;

/// ⚖️ Process-wide ceiling on cached session bytes. A check-in whose census would cross it is dropped
/// rather than admitted, so the next call for that instance pays exactly one cold rebuild — the
/// pre-session baseline, never a correctness change.
const PUZZLE3D_SESSION_PROCESS_BYTES: usize = 96 * 1024 * 1024;

/// 🧠 Everything one document instance keeps between two dispatches. It is boxed (like the whole
/// state), so the fixed slot row itself never carries the multi-kilobyte payload inline.
#[derive(Default)]
struct Puzzle3dSessionState {
    /// 🚚️ Wave B44: the per-object instance residency, so the record text an unchanged object already
    /// has survives a dispatch and a worker hop exactly as the whole-set blob used to.
    instances: Option<Box<Puzzle3dInstanceResidency>>,
    meshes: Option<(u64, String)>,
    collision: Option<Puzzle3dCollisionSession>,
}

impl Puzzle3dSessionState {
    fn bytes(&self) -> usize {
        self.instances
            .as_ref()
            .map_or(0, |residency| residency.bytes())
            .saturating_add(self.meshes.as_ref().map_or(0, |(_, meshes)| meshes.len()))
            .saturating_add(self.collision.as_ref().map_or(0, Puzzle3dCollisionSession::bytes))
    }
}

/// 🎟️ One slot of the fixed row. The cached state lives behind a pointer on purpose: the row is a
/// `[Option<Puzzle3dSessionSlot>; PUZZLE3D_SESSION_SLOTS]` built by value, so an inline payload would be
/// multiplied by every slot on the stack of whichever call first touches the registry.
struct Puzzle3dSessionSlot {
    app_instance_id: u32,
    artifact_id: Option<String>,
    bytes: usize,
    touched: u64,
    state: Option<Box<Puzzle3dSessionState>>,
}

/// 🎫 What a caller holds while a slot's state is checked out. The slot's own generation plus the
/// instance id make it ABA-proof: a slot that was retired, evicted or re-keyed rejects the lease
/// instead of adopting a cache built against a different document.
#[derive(Clone, Copy)]
struct Puzzle3dSessionLease {
    slot: usize,
    app_instance_id: u32,
    generation: u64,
}

struct Puzzle3dSessionRegistry {
    slots: [Option<Puzzle3dSessionSlot>; PUZZLE3D_SESSION_SLOTS],
    generations: [u64; PUZZLE3D_SESSION_SLOTS],
    aggregate_bytes: usize,
    sequence: u64,
}

impl Default for Puzzle3dSessionRegistry {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), generations: [0; PUZZLE3D_SESSION_SLOTS], aggregate_bytes: 0, sequence: 0 }
    }
}

fn puzzle3d_session_registry() -> &'static Mutex<Puzzle3dSessionRegistry> {
    static REGISTRY: OnceLock<Mutex<Puzzle3dSessionRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Puzzle3dSessionRegistry::default()))
}

impl Puzzle3dSessionRegistry {
    fn candidates(app_instance_id: u32) -> impl Iterator<Item = usize> {
        let base = usize::try_from(app_instance_id).unwrap_or(0) % PUZZLE3D_SESSION_SLOTS;
        (0..PUZZLE3D_SESSION_PROBES).map(move |offset| (base + offset) % PUZZLE3D_SESSION_SLOTS)
    }

    fn owns(&self, slot: usize, app_instance_id: u32) -> bool {
        self.slots[slot].as_ref().is_some_and(|entry| entry.app_instance_id == app_instance_id)
    }

    /// 🚪️ Drops the state a slot held and bumps its generation, so every lease taken against it fails to
    /// check in. Driven in production the moment one instance id is observed carrying a different
    /// parent document — the framework reuses instance ids, and a reused id is a new document.
    fn retire(&mut self, slot: usize) {
        self.vacate(slot);
        crate::editor::puzzle3d::precompute::retire_abandoned_brush_mesh_uploads();
    }

    /// 🕳️ Empties one slot and bumps its generation without sweeping the process-wide page-run uploads.
    fn vacate(&mut self, slot: usize) {
        if let Some(entry) = self.slots[slot].take() {
            self.aggregate_bytes = self.aggregate_bytes.saturating_sub(entry.bytes);
        }
        self.generations[slot] = self.generations[slot].saturating_add(1);
    }

    fn resolve_slot(&mut self, app_instance_id: u32, artifact_id: Option<&str>) -> Option<usize> {
        if let Some(slot) = Self::candidates(app_instance_id).find(|slot| self.owns(*slot, app_instance_id)) {
            let rekeyed = self.slots[slot].as_ref().is_some_and(|entry| entry.artifact_id.as_deref().zip(artifact_id).is_some_and(|(held, current)| held != current));
            if rekeyed {
                self.retire(slot);
            }
            return Some(slot);
        }
        if let Some(slot) = Self::candidates(app_instance_id).find(|slot| self.slots[*slot].is_none()) {
            return Some(slot);
        }
        let evicted = Self::candidates(app_instance_id).filter(|slot| self.slots[*slot].as_ref().is_some_and(|entry| entry.state.is_some())).min_by_key(|slot| self.slots[*slot].as_ref().map_or(u64::MAX, |entry| entry.touched))?;
        self.retire(evicted);
        Some(evicted)
    }

    fn check_out(&mut self, app_instance_id: u32, artifact_id: Option<&str>) -> Option<(Puzzle3dSessionLease, Puzzle3dSessionState)> {
        let slot = self.resolve_slot(app_instance_id, artifact_id)?;
        self.sequence = self.sequence.saturating_add(1);
        let previous = self.slots[slot].take();
        if let Some(entry) = &previous {
            self.aggregate_bytes = self.aggregate_bytes.saturating_sub(entry.bytes);
        }
        let held = previous.and_then(|entry| entry.state);
        let artifact_id = artifact_id.map(str::to_string);
        self.slots[slot] = Some(Puzzle3dSessionSlot { app_instance_id, artifact_id, bytes: 0, touched: self.sequence, state: None });
        Some((Puzzle3dSessionLease { slot, app_instance_id, generation: self.generations[slot] }, held.map_or_else(Puzzle3dSessionState::default, |state| *state)))
    }

    fn check_in(&mut self, lease: Puzzle3dSessionLease, state: Puzzle3dSessionState) {
        if self.generations[lease.slot] != lease.generation {
            return;
        }
        let bytes = state.bytes();
        if self.aggregate_bytes.checked_add(bytes).is_none_or(|total| total > PUZZLE3D_SESSION_PROCESS_BYTES) {
            return;
        }
        let Some(entry) = self.slots[lease.slot].as_mut().filter(|entry| entry.app_instance_id == lease.app_instance_id) else {
            return;
        };
        entry.bytes = bytes;
        entry.state = Some(Box::new(state));
        self.aggregate_bytes = self.aggregate_bytes.saturating_add(bytes);
    }
}

/// 🎟️ Adopts one instance's session onto a freshly built app. An exhausted slot row or a first-ever call
/// resolves to "no cached state", which is exactly the pre-session behaviour. The registry lock is WAITED
/// for, never skipped: its critical section is a slot move, and the session carries behaviour, not only
/// speed — the standing mesh re-upload set lives here, so a skipped adoption would answer an already
/// requested identity as new and re-arm the re-announce storm (wave B32).
fn puzzle3d_session_check_out(app_instance_id: u32, artifact_id: Option<&str>, app: &Puzzle3dSession) -> Option<Puzzle3dSessionLease> {
    let (lease, state) = puzzle3d_session_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).check_out(app_instance_id, artifact_id)?;
    *app.instance_residency.lock().expect("instance residency") = state.instances;
    *app.mesh_cache.lock().expect("mesh cache") = state.meshes;
    {
        let mut session = app.precompute.borrow_mut();
        if let Some(collision) = state.collision {
            session.install_collision_session(collision);
        }
    }
    Some(lease)
}

/// 🎟️ Returns the app's caches to their slot, waiting for the registry lock like
/// [`puzzle3d_session_check_out`]. A stale lease (the slot was retired or re-keyed mid-call) is rejected and
/// the state simply dropped.
fn puzzle3d_session_check_in(lease: Puzzle3dSessionLease, app: &Puzzle3dSession) {
    let collision = Some(app.precompute.borrow_mut().take_collision_session());
    let state = Puzzle3dSessionState {
        instances: app.instance_residency.lock().expect("instance residency").take(),
        meshes: app.mesh_cache.lock().expect("mesh cache").take(),
        collision,
    };
    puzzle3d_session_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).check_in(lease, state);
}

/// 🧹️ Vacates every slot `app_instance_id` owns — what a scene_snapshot app does when it is dropped, so one law's
/// session never outlives it into the process-global row the next law probes. The page-run upload sweep is
/// left to the production retirements, so a concurrent law's open upload is never swept by another's drop.
#[cfg(test)]
pub(crate) fn puzzle3d_session_release(app_instance_id: u32) {
    let mut registry = puzzle3d_session_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    for slot in Puzzle3dSessionRegistry::candidates(app_instance_id).collect::<Vec<_>>() {
        if registry.owns(slot, app_instance_id) {
            registry.vacate(slot);
        }
    }
}

/// 🪪️ The artifact-instance identity the framework already threads into every view — the session key.
/// A command operation carries the parent artifact id too, which is what lets a reused instance id
/// retire the previous artifact's cache instead of adopting it.
fn puzzle3d_view_session_key(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Option<(u32, Option<String>)> {
    if let Some(operation) = doc.operation_optional() {
        return Some((operation.app_instance_id, Some(operation.parent_document_id.clone())));
    }
    doc.render_operation().map(|operation| (operation.app_instance_id, None))
}

/// 🧠 One document instance's play session. `ArtifactApp` methods are associated fns (no `&self`), so
/// every dispatch still builds a fresh app object — but its caches are now checked out of, and back
/// into, a process-global slot keyed by `app_instance_id`, so `geometry_cache`/`instance_residency`/
/// registered brush meshes/the brush broad-phase index survive a call and a worker hop. No tool run state
/// lives here: a fill run is owned by the framework tool run ledger.
fn with_puzzle3d_session_for<R>(session: Option<(u32, Option<String>)>, config: &Puzzle3dRuntime, f: impl FnOnce(&Puzzle3dSession) -> R) -> R {
    let app = Puzzle3dSession::default();
    let lease = session.and_then(|(app_instance_id, artifact_id)| puzzle3d_session_check_out(app_instance_id, artifact_id.as_deref(), &app));
    let _ = config;
    let result = f(&app);
    if let Some(lease) = lease {
        puzzle3d_session_check_in(lease, &app);
    }
    result
}

#[cfg(test)]
pub(crate) fn with_puzzle3d_session<R>(f: impl FnOnce(&Puzzle3dSession) -> R) -> R {
    let app = Puzzle3dSession::default();
    f(&app)
}

#[cfg(test)]
thread_local! {
    /// 🔬️ How many times `geometry_jsons` genuinely re-serialized a scene_snapshot on THIS thread. The session
    /// registry exists so that a second call on the same document does not, and the tests read this
    /// counter instead of inferring cache behaviour from timings. Thread-local so one test's measurement
    /// cannot be perturbed by another test running concurrently in the same process.
    pub(crate) static PUZZLE3D_GEOMETRY_SERIALIZATIONS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// 🔬️ How many times `ArtifactApp::interaction_topology` decoded the document and rebuilt the whole
    /// object/vortex/attraction forest on THIS thread. One pick used to pay it TWICE — once in the
    /// framework's own `interactionSelect` arm and once in the revalidation behind it — which on the
    /// 180-object flagship is the O(n) term wave B44 §2.2 measured as `interactionSelect`'s 20×.
    pub(crate) static PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// 🧠️ What one document instance retains across turns for its commands, renders and tool run jobs: the brush
/// suggestions link. Ephemeral local-only; it dies with the instance.
#[derive(Default)]
pub struct Puzzle3dInstanceOperationOwner {
    pub brush_suggestions: BrushSuggestionsLink,
    retirement: crate::puzzle_job::WorkClosing<BrushSuggestionsLink>,
}

impl BrushSuggestionsOwner for Puzzle3dInstanceOperationOwner {
    fn brush_suggestions(&mut self) -> &mut BrushSuggestionsLink {
        &mut self.brush_suggestions
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Puzzle3dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.retirement.is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.brush_suggestions.is_empty()), ..Default::default() });
        }
        self.retirement.demands(body)
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,_grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        Ok(semio_framework_plugin::PluginLifecycleStep::Complete(Default::default()))
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        if self.retirement.is_empty() && !self.brush_suggestions.is_empty() {
            self.retirement.stage(std::mem::take(&mut self.brush_suggestions));
        }
        Ok(match self.retirement.close_step(grant) {
            InteractiveJobCloseStep::Pending { progress } => semio_framework_plugin::PluginLifecycleStep::Progress(progress),
            InteractiveJobCloseStep::Complete { progress } => semio_framework_plugin::PluginLifecycleStep::Complete(progress),
            InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginLifecycleStep::Blocked { reason: "puzzle3d instance owner close is blocked" },
            InteractiveJobCloseStep::Refused { kind, .. } => return Err(Fault::from(format!("puzzle3d instance owner close was refused: {kind:?}"))),
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.brush_suggestions.is_empty() && self.retirement.is_empty()
    }
}

pub struct Puzzle3dSession {
    pub(crate) precompute: std::cell::RefCell<Box<Puzzle3dPrecomputeSession>>,
    /// 🚚️ Wave B44: per-object instance residency — the ONE owner of the published instance text, and
    /// the source of the changed/removed id delta the world lane rides with.
    pub(crate) instance_residency: Mutex<Option<Box<Puzzle3dInstanceResidency>>>,
    mesh_cache: Mutex<Option<(u64, String)>>,
}

impl Default for Puzzle3dSession {
    fn default() -> Self {
        Self {
            precompute: std::cell::RefCell::new(Box::new(Puzzle3dPrecomputeSession::new())),
            instance_residency: Mutex::new(None),
            mesh_cache: Mutex::new(None),
        }
    }
}

impl Puzzle3dSession {
    /// 🚚️ The world lane's three geometry payloads for one scene_snapshot: the authoritative instance set, the
    /// mesh declarations, and the per-object delta that produced the current instance revision.
    ///
    /// ⏱️ Wave B44: invalidation is PER OBJECT. Only the records whose own
    /// `instance_record_fingerprint` moved are re-serialized; the mesh declarations and the outliner
    /// memo stay keyed on the (now allocation-free) whole-scene_snapshot `scene_geometry_fingerprint`, which
    /// is what actually changes when the catalogs or the mesh set do.
    fn geometry_jsons(&self, scene_snapshot: &Puzzle3dSceneSnapshot, provisional: &std::collections::BTreeSet<u64>, referenced: &[String]) -> (String, String, Option<String>) {
        let fingerprint = scene_geometry_fingerprint(scene_snapshot);
        let mut residency = self.instance_residency.lock().expect("instance residency");
        let residency = residency.get_or_insert_with(Box::<Puzzle3dInstanceResidency>::default);
        #[cfg_attr(not(test), allow(unused_variables))]
        let republished = residency.refresh(scene_snapshot, provisional, referenced);
        let mut meshes = self.mesh_cache.lock().expect("mesh cache");
        let mesh_miss = meshes.as_ref().is_none_or(|(cached, _)| *cached != fingerprint);
        if mesh_miss {
            *meshes = Some((fingerprint, world_meshes_json(scene_snapshot)));
        }
        #[cfg(test)]
        if republished || mesh_miss {
            PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(|counter| counter.set(counter.get().saturating_add(1)));
        }
        let (_, meshes) = meshes.as_ref().expect("mesh cache populated");
        (residency.instances_json().to_string(), meshes.clone(), residency.delta_json().map(str::to_string))
    }

}

use semio_framework_plugin::world3d_mesh_id_from_url;
use std::hash::Hasher;

/// 🙈️ Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
/// Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks.
pub fn world_instances_geometry_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let mut residency = Puzzle3dInstanceResidency::default();
    residency.refresh(scene_snapshot, &std::collections::BTreeSet::new(), &[]);
    residency.instances_json().to_string()
}

/// 🧱️ ONE instance record, exactly the shape [`world_instances_geometry_json`] publishes; `provisional` marks an
/// object a running tool placed and has not finalized (the `provisional` mesh style, never pickable), `highlighted` an
/// object the open time-travel draft references (both hosts paint the `highlighted` instance row).
fn instance_record_json(object: &Puzzle3dObject, mesh_id: &str, provisional: bool, highlighted: bool) -> String {
    let scale = if object.hidden { serde_json::json!([0.0, 0.0, 0.0]) } else { serde_json::json!(object_scale_json(object)) };
    let mut instance = serde_json::json!({
        "id": object.id,
        "meshId": mesh_id,
        "position": [
            object.origin.first().copied().unwrap_or(0.0),
            object.origin.get(1).copied().unwrap_or(0.0),
            object.origin.get(2).copied().unwrap_or(0.0),
        ],
        "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
        "scale": scale,
        "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
        "disabled": object.locked,
    });
    if let Some(kind) = &object.object_kind {
        instance["objectKind"] = serde_json::json!(kind);
    }
    if provisional {
        instance["provisional"] = serde_json::json!(true);
    }
    if highlighted {
        instance["highlighted"] = serde_json::json!(true);
    }
    serde_json::to_string(&instance).unwrap_or_else(|_| "{}".into())
}

/// 🔑️ The per-object change key for ONE instance record — every field [`instance_record_json`] reads
/// and nothing else, hashed structurally. No JSON is materialized: this is the difference between a
/// per-object key that costs a handful of `Hash::hash` calls and the whole-document
/// `format!`-then-hash [`scene_geometry_fingerprint`] used to be (measured 20 716 µs on the
/// 180-object Nakagin document, ticket 26/09/02 wave B44 §1).
fn instance_record_fingerprint(object: &Puzzle3dObject, mesh_id: &str, provisional: bool, highlighted: bool) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    object.id.hash(&mut hasher);
    provisional.hash(&mut hasher);
    highlighted.hash(&mut hasher);
    mesh_id.hash(&mut hasher);
    for axis in object.origin {
        axis.to_bits().hash(&mut hasher);
    }
    object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(&mut hasher);
    match &object.scale {
        Some(scale) => hash_dsl_value(scale, &mut hasher),
        None => 0_u8.hash(&mut hasher),
    }
    object.label.hash(&mut hasher);
    object.object_kind.hash(&mut hasher);
    object.hidden.hash(&mut hasher);
    object.locked.hash(&mut hasher);
    hasher.finish()
}

/// 🧮️ Structural hash of one DSL value — the allocation-free replacement for hashing its JSON text.
/// Floats go in by `to_bits`, so the key is exact rather than format-dependent, and an object's keys
/// are hashed in their authored order (the order `ToValue` emits, which is what the previous JSON
/// hash was sensitive to as well).
pub fn hash_dsl_value<H: Hasher>(value: &semio_framework_value::DslValue, hasher: &mut H) {
    match value {
        semio_framework_value::DslValue::Null => 0_u8.hash(hasher),
        semio_framework_value::DslValue::Bool(flag) => {
            1_u8.hash(hasher);
            flag.hash(hasher);
        }
        semio_framework_value::DslValue::Number(number) => {
            2_u8.hash(hasher);
            match number {
                semio_framework_value::Number::UInt(value) => value.hash(hasher),
                semio_framework_value::Number::Int(value) => value.hash(hasher),
                semio_framework_value::Number::Float(value) => value.to_bits().hash(hasher),
            }
        }
        semio_framework_value::DslValue::String(text) => {
            3_u8.hash(hasher);
            text.hash(hasher);
        }
        semio_framework_value::DslValue::Array(items) => {
            4_u8.hash(hasher);
            items.len().hash(hasher);
            for item in items {
                hash_dsl_value(item, hasher);
            }
        }
        semio_framework_value::DslValue::Object(entries) => {
            5_u8.hash(hasher);
            entries.len().hash(hasher);
            for (key, item) in entries {
                key.hash(hasher);
                hash_dsl_value(item, hasher);
            }
        }
        semio_framework_value::DslValue::Bytes(bytes) => {
            6_u8.hash(hasher);
            bytes.hash(hasher);
        }
    }
}

/// 🚚️ Per-object residency for the world `instances` lane: the one place the published instance text
/// is built, kept and invalidated PER OBJECT instead of per document.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44. Before it, one viewport render of the 180-object
/// Nakagin document paid 20 716 µs hashing the whole scene_snapshot's JSON plus 2 449 µs re-serializing all
/// 180 records — for a gumball drag that moved ONE of them, and once per window and once more for the
/// outliner. This walks the objects, re-serializes only the records whose own
/// [`instance_record_fingerprint`] moved, and reports exactly which ids those were, so an intake that
/// wants to update its instanced meshes in place has the id list to do it with ([`Self::delta_json`]).
///
/// 🧾️ `instances_json` stays the AUTHORITATIVE full set on every publication: a pose-only lane would
/// be O(changed) bytes but would leave every consumer that does not merge deltas (the wgpu world
/// target builds `World3dScene` directly) rendering a stale pose. The delta rides ALONGSIDE it.
#[derive(Default)]
pub struct Puzzle3dInstanceResidency {
    entries: std::collections::HashMap<String, (u64, String)>,
    order: Vec<String>,
    assembled: String,
    revision: u64,
    changed: Vec<String>,
    removed: Vec<String>,
    delta: String,
    rebuilt: u32,
}

impl Puzzle3dInstanceResidency {
    /// 🔄️ Reconciles the residency against one scene_snapshot, the running tool's provisional entities
    /// (`ArtifactView::tool_run().provisional_entities`, keyed by `fill_run_entity(object.id)`) and the object ids the open
    /// time-travel draft references (`InteractionView::draft_references`), so opening or closing a history edit republishes
    /// exactly the referenced records through the delta. Answers whether the published text changed — `false` means every
    /// record and the order are bit-identical and no consumer owes any work.
    pub fn refresh(&mut self, scene_snapshot: &Puzzle3dSceneSnapshot, provisional: &std::collections::BTreeSet<u64>, referenced: &[String]) -> bool {
        let kind_meshes = Puzzle3dKindMeshIndex::of(&scene_snapshot.meta);
        let mut order = Vec::with_capacity(scene_snapshot.objects.len());
        let mut changed = Vec::new();
        let mut rebuilt = 0_u32;
        for object in &scene_snapshot.objects {
            let mesh_id = kind_meshes.resolve(object).map_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into(), world3d_mesh_id_from_url);
            let placed = !provisional.is_empty() && provisional.contains(&crate::editor::puzzle3d::precompute::fill::fill_run_entity(&object.id));
            let highlighted = referenced.iter().any(|id| id == &object.id);
            let fingerprint = instance_record_fingerprint(object, &mesh_id, placed, highlighted);
            if self.entries.get(&object.id).is_none_or(|(cached, _)| *cached != fingerprint) {
                self.entries.insert(object.id.clone(), (fingerprint, instance_record_json(object, &mesh_id, placed, highlighted)));
                changed.push(object.id.clone());
                rebuilt += 1;
            }
            order.push(object.id.clone());
        }
        let removed: Vec<String> = self.order.iter().filter(|id| order.iter().all(|live| live != *id)).cloned().collect();
        for id in &removed {
            self.entries.remove(id);
        }
        let reordered = self.order != order;
        self.order = order;
        // 🪟️ A no-op refresh leaves the LAST delta standing: two window instances of one document render
        // off one session, so the second render must still be able to hand its own consumer the delta the
        // first render produced (both carry the same `base`/`revision`, and a consumer whose retained set
        // is not at `base` falls back to `instancesJson`).
        if !reordered && changed.is_empty() && removed.is_empty() && !self.assembled.is_empty() {
            self.rebuilt = 0;
            return false;
        }
        self.changed = changed;
        self.removed = removed;
        self.rebuilt = rebuilt;
        self.assembled = self.assemble();
        self.revision = self.revision.saturating_add(1);
        self.delta = if self.delta_is_worth_publishing() { self.assemble_delta() } else { String::new() };
        true
    }

    /// ⚖️ Retained bytes, for the session registry's process-wide census.
    pub fn bytes(&self) -> usize {
        self.assembled.len().saturating_add(self.delta.len()).saturating_add(self.entries.values().map(|(_, record)| record.len() + 48).sum::<usize>())
    }

    fn assemble(&self) -> String {
        let mut text = String::with_capacity(self.order.iter().filter_map(|id| self.entries.get(id)).map(|(_, record)| record.len() + 1).sum::<usize>() + 2);
        text.push('[');
        for id in &self.order {
            if let Some((_, record)) = self.entries.get(id) {
                if text.len() > 1 {
                    text.push(',');
                }
                text.push_str(record);
            }
        }
        text.push(']');
        text
    }

    /// 🧾️ The authoritative full instance set, exactly what [`world_instances_geometry_json`] returns.
    pub fn instances_json(&self) -> &str {
        if self.assembled.is_empty() {
            "[]"
        } else {
            &self.assembled
        }
    }

    /// 🚚️ The last refresh's delta, keyed by instance id: the records that changed (whole records, so
    /// an applying consumer never has to diff fields) and the ids that went away. `base` is the
    /// revision the delta applies TO, `revision` the one it produces — a consumer whose retained set
    /// is not at `base` must fall back to `instancesJson`.
    pub fn delta_json(&self) -> Option<&str> {
        if self.delta.is_empty() {
            None
        } else {
            Some(&self.delta)
        }
    }

    /// ⚖️ Whether a delta is worth publishing at all. A cold publication (nothing retained anywhere
    /// yet) and one that names at least half the set buy a consumer nothing — it has to read
    /// `instancesJson` either way — while costing a second copy of the same bytes on the wire and one
    /// more lane carrier to page. Measured: publishing the cold 180-record delta alongside the
    /// 55 154-byte full set put `openVortexSuggestions`'s worst turn at 4.56 ms against this
    /// artifact's own 2 ms unoptimized ceiling.
    fn delta_is_worth_publishing(&self) -> bool {
        self.revision > 1 && self.changed.len() * 2 < self.order.len() && self.removed.len() * 2 < self.order.len().max(1)
    }

    /// 🚚️ Assembles the delta text ONCE per republication, by concatenating the record strings the
    /// residency already holds. Never by `serde_json` round-tripping them: a cold Nakagin publication
    /// names 180 changed records, and re-parsing them on every `delta_json` call (which every render
    /// makes, republishing or not) cost more than the whole-set re-serialization this wave removed —
    /// it broke `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`
    /// before being caught.
    fn assemble_delta(&self) -> String {
        let mut text = String::with_capacity(self.changed.iter().filter_map(|id| self.entries.get(id)).map(|(_, record)| record.len() + 1).sum::<usize>() + 128);
        text.push_str(r#"{"base":"#);
        text.push_str(&self.revision.saturating_sub(1).to_string());
        text.push_str(r#","revision":"#);
        text.push_str(&self.revision.to_string());
        text.push_str(r#","count":"#);
        text.push_str(&self.order.len().to_string());
        text.push_str(r#","changed":["#);
        let mut first = true;
        for id in &self.changed {
            if let Some((_, record)) = self.entries.get(id) {
                if !first {
                    text.push(',');
                }
                text.push_str(record);
                first = false;
            }
        }
        text.push_str(r#"],"removed":"#);
        text.push_str(&serde_json::to_string(&self.removed).unwrap_or_else(|_| "[]".into()));
        text.push('}');
        text
    }

    /// 🔬️ The ids whose record was re-serialized by the last refresh — the observable the O(changed)
    /// laws read instead of inferring incrementality from timings.
    pub fn changed_ids(&self) -> &[String] {
        &self.changed
    }

    pub fn removed_ids(&self) -> &[String] {
        &self.removed
    }

    /// 🔬️ How many records the last refresh genuinely re-serialized.
    pub fn rebuilt_records(&self) -> u32 {
        self.rebuilt
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }
}

use crate::editor::puzzle3d::snapshot::VORTEX_MARKER_MESH_KIND;
use semio_framework_plugin::world3d_meshes_json_from_kinds_and_urls;

/// 🗄️ Cheap change key for everything the instance/mesh payloads (and the document tree) derive from.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44: hashed STRUCTURALLY, never through JSON text.
/// The `format!` of four `to_json_string` calls this used to be materialized ~60 KiB per call on the
/// 180-object Nakagin document and measured **20 716 µs** — 54 % of a cache-HIT viewport render, paid
/// again per window and again for the outliner memo's key. The structural walk is the same O(n) key
/// over the same four members with none of the allocation.
pub fn scene_geometry_fingerprint(scene_snapshot: &Puzzle3dSceneSnapshot) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    scene_snapshot.objects.len().hash(&mut hasher);
    for object in &scene_snapshot.objects {
        hash_object(object, &mut hasher);
    }
    scene_snapshot.references.len().hash(&mut hasher);
    for reference in &scene_snapshot.references {
        reference.id.hash(&mut hasher);
        reference.source.url.hash(&mut hasher);
        reference.source.media_kind.hash(&mut hasher);
        hash_axes(&reference.origin, &mut hasher);
        reference.width_world.to_bits().hash(&mut hasher);
        reference.locked.hash(&mut hasher);
        reference.hidden.hash(&mut hasher);
    }
    scene_snapshot.target_volumes.len().hash(&mut hasher);
    for volume in &scene_snapshot.target_volumes {
        volume.id.hash(&mut hasher);
        hash_axes(&volume.origin, &mut hasher);
        volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(&mut hasher);
        hash_optional_dsl_value(volume.scale.as_ref(), &mut hasher);
        volume.hidden.hash(&mut hasher);
        volume.locked.hash(&mut hasher);
    }
    hash_optional_dsl_value(scene_snapshot.meta.kind_catalogs.as_ref(), &mut hasher);
    hash_optional_dsl_value(scene_snapshot.meta.kind_compatibility.as_ref(), &mut hasher);
    hasher.finish()
}

fn hash_axes<H: Hasher>(axes: &[f64], hasher: &mut H) {
    axes.len().hash(hasher);
    for axis in axes {
        axis.to_bits().hash(hasher);
    }
}

fn hash_optional_dsl_value<H: Hasher>(value: Option<&semio_framework_value::DslValue>, hasher: &mut H) {
    match value {
        Some(value) => {
            1_u8.hash(hasher);
            hash_dsl_value(value, hasher);
        }
        None => 0_u8.hash(hasher),
    }
}

/// 🧱️ Every persisted field of ONE object, vortices included — the geometry key's per-object half.
fn hash_object<H: Hasher>(object: &Puzzle3dObject, hasher: &mut H) {
    object.id.hash(hasher);
    object.label.hash(hasher);
    object.object_kind.hash(hasher);
    hash_axes(&object.origin, hasher);
    object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]).map(f64::to_bits).hash(hasher);
    hash_optional_dsl_value(object.scale.as_ref(), hasher);
    object.mesh_url.hash(hasher);
    object.hidden.hash(hasher);
    object.locked.hash(hasher);
    object.vortices.len().hash(hasher);
    for vortex in &object.vortices {
        vortex.id.hash(hasher);
        vortex.vortex_kind.hash(hasher);
        hash_axes(&vortex.position, hasher);
        vortex.direction.unwrap_or([0.0, 0.0, 0.0]).map(f64::to_bits).hash(hasher);
        vortex.radius.unwrap_or(0.0).to_bits().hash(hasher);
        vortex.hidden.hash(hasher);
        vortex.locked.hash(hasher);
    }
}

/// 🧵️ Kind meshes published ahead of every mesh url, in this order.
const WORLD_MESH_KINDS: [&str; 2] = [PUZZLE3D_FALLBACK_MESH_KIND, VORTEX_MARKER_MESH_KIND];

/// 🧵️ The world mesh lane in `meshesJson` order: the kind meshes, then every sorted mesh url whose mesh id
/// is new. A tool run trace subject's `mesh` is an index into exactly this list, so a fill run job is
/// built with it and `meshesJson` is published from it.
pub fn mesh_lane(scene_snapshot: &Puzzle3dSceneSnapshot) -> Vec<String> {
    let mut lane: Vec<String> = WORLD_MESH_KINDS.iter().map(|kind| (*kind).to_string()).collect();
    let mut ids: std::collections::HashSet<String> = lane.iter().cloned().collect();
    lane.extend(collect_mesh_urls(scene_snapshot).into_iter().filter(|url| ids.insert(world3d_mesh_id_from_url(url))));
    lane
}

pub fn world_meshes_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let lane = mesh_lane(scene_snapshot);
    world3d_meshes_json_from_kinds_and_urls(&lane[..WORLD_MESH_KINDS.len()], &lane[WORLD_MESH_KINDS.len()..])
}

//#region 🧩️EditorContract
/// 🧩️ The puzzle3d editor contract: every editor whose lanes are puzzle3d's play snapshot, mutation union, config,
/// presence and command vocabulary. The action dispatch, the retained works and the tool run builders are written
/// against it, so they sit below the one concrete editor that implements it.
pub trait Puzzle3dEditor: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle3dPlaySnapshot, Mutation = Puzzle3dMutation, Config = Puzzle3dConfig, ConfigMutation = crate::editor::puzzle3d::config::Puzzle3dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = crate::editor::puzzle3d::presence::Puzzle3dPresence, PresenceMutation = crate::editor::puzzle3d::presence::Puzzle3dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = crate::editor::puzzle3d::snapshot::Puzzle3dCommand> {}

impl<A> Puzzle3dEditor for A where A: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle3dPlaySnapshot, Mutation = Puzzle3dMutation, Config = Puzzle3dConfig, ConfigMutation = crate::editor::puzzle3d::config::Puzzle3dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = crate::editor::puzzle3d::presence::Puzzle3dPresence, PresenceMutation = crate::editor::puzzle3d::presence::Puzzle3dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = crate::editor::puzzle3d::snapshot::Puzzle3dCommand> {}
//#endregion 🧩️EditorContract
