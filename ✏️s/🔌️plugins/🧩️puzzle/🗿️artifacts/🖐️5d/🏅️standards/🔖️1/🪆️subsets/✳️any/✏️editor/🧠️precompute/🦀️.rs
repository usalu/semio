//! 🧠️ Puzzle 5d play app — the bridge onto the puzzle 3d planners. The 5d document is the unification of a 2d
//! board and a 3d world, so its placement and suggestion solvers ARE the puzzle 3d fill and brush run jobs: a 5d
//! document is handed to them as the equivalent 3d document (parts as objects, grips as vortices, fasteners as
//! attractions, and the kind catalogs they place from), and every tick they report is translated back —
//! `create_object`/`connect_vortices` become `create-part`/`connect-grips` with a board position synthesized next
//! to the host grip, and every `instance3d` trace record gains a `placement2d` twin for the board window.
//! Contract: `📋️tool-run-contract.md` §2.7, §3.2, §3.7.

use crate::editor::puzzle5d::config::Puzzle5dConfig;
use crate::editor::puzzle5d::precompute::geometry::Puzzle5dPointGrid;
use crate::editor::puzzle5d::{
    collect_mesh_urls, engine_grip_kind, grips_from_templates, puzzle5d_grip_full_id, resolve_part_mesh_url, world_grip_position, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dPart, Puzzle5dPart2d,
    Puzzle5dPart3d, PUZZLE5D_BOARD_PLACEMENT_GAP, PUZZLE5D_DEFAULT_PART_RADIUS, PUZZLE5D_FALLBACK_MESH_KIND,
};
use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::{connect_grips, create_part, Puzzle5dMutation};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepBudget, StepContext, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_plugin::{Fault, ToolRunJob};
use semio_framework_tool_run::{ToolRunIdentity, ToolRunTick, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_TRACE_PAGE_OPS_MAX};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::config::Puzzle3dConfig;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main as world3d;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::puzzle3d_fixture_from_snapshot;
use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::mutations::{ConnectVortices, Puzzle3dMutation};
use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::BrushPreviewState;
use semio_s_artifact_puzzle_3d::{Puzzle3dAttraction, Puzzle3dCatalogObjectKind, Puzzle3dCatalogVortexKind, Puzzle3dCatalogVortexTemplate, Puzzle3dKindCatalogs, Puzzle3dMeta, Puzzle3dObject, Puzzle3dPlaySnapshot, Puzzle3dRepresentation, Puzzle3dScale, Puzzle3dSnapshot, Puzzle3dVortex, PUZZLE_3D_SCHEMA};
use std::collections::{HashMap, VecDeque};

//#region 🔖️Constants
/// 🪞️ The bit that turns a planner trace key into the key of its board twin. Planner keys never carry it: fill
/// candidates count up from zero, revalidation keys set bit 63 and brush candidates are candidate indices.
pub const PUZZLE5D_PLANNER_TRACE_TWIN_BIT: u64 = 1 << 62;
/// ⭕️ The board shape index every trace twin draws: planner parts are circles.
pub const PUZZLE5D_PLANNER_TRACE_SHAPE_CIRCLE: u32 = 0;
/// 🧲️ Cell edge and reach (world units) of the grip grid a trace twin finds its host grip in.
pub const PUZZLE5D_PLANNER_HOST_GRIP_CELL: f64 = 2.0;
pub const PUZZLE5D_PLANNER_HOST_GRIP_REACH: f64 = 16.0;
/// 🧬️ Provisional ops one placement appends: `create-part` then `connect-grips`.
pub const PUZZLE5D_PLACEMENT_OPS: usize = 2;
/// 🧵️ Kind meshes (box, vortex marker) the world mesh lane publishes ahead of every mesh url.
pub const PUZZLE5D_WORLD_MESH_KINDS: usize = 2;
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🆔️ The provisional entity of a placed part: the first eight little-endian bytes of its id digest, exactly
/// the entity the planner reports, so the world window can stamp provisional instances.
pub fn puzzle5d_placement_entity(part_id: &str) -> u64 {
    u64::from_le_bytes(semio_framework_hash::hash(part_id.as_bytes()).as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 🧵️ The world mesh lane a planner run's `instance3d` subjects index: the planner's own lane over the 3d document
/// of `document`, or the kind meshes and sorted urls when that document cannot be built (no run can start then).
pub fn puzzle5d_mesh_lane(snapshot: &Puzzle5dPlaySnapshot, document: &Puzzle5dDocument) -> Vec<String> {
    match puzzle5d_authored_kind_catalogs(snapshot).and_then(|catalogs| puzzle3d_snapshot(document, catalogs)) {
        Ok(snapshot) => world3d::mesh_lane(&puzzle3d_fixture_from_snapshot(snapshot.typed())),
        Err(_) => {
            let mut urls = collect_mesh_urls(document);
            urls.sort();
            [PUZZLE5D_FALLBACK_MESH_KIND.to_string(), world3d::VORTEX_MARKER_MESH_KIND.to_string()].into_iter().chain(urls).collect()
        }
    }
}

/// 🎚️ The planner configuration a 5d configuration asks for.
pub(crate) fn puzzle3d_config(config: &Puzzle5dConfig) -> Puzzle3dConfig {
    Puzzle3dConfig { fill_count: config.fill_count, contact_tolerance: config.contact_tolerance, object_kind_weights: config.object_kind_weights.clone(), vortex_kind_weights: config.vortex_kind_weights.clone(), ..Puzzle3dConfig::default() }
}

/// 🌉️ The puzzle 3d document the planner sees for a 5d document: parts as objects with their grips as vortices,
/// fasteners as attractions, and the kind catalogs the planner places from.
pub fn puzzle3d_snapshot(document: &Puzzle5dDocument, catalogs: Option<crate::Puzzle5dKindCatalogs>) -> Result<Puzzle3dPlaySnapshot, Fault> {
    let kind_compatibility = document.kind_compatibility.as_ref().map(|entries| dsl::FromValue::from_value(dsl::DslValue::from(entries))).transpose().map_err(|error: dsl::ValueError| Fault::from(format!("puzzle5d-planner-kind-compatibility: {error}")))?.unwrap_or_default();
    let typed = Puzzle3dSnapshot {
        schema: PUZZLE_3D_SCHEMA.into(),
        domain: document.domain.clone(),
        meta: Puzzle3dMeta { kind_catalogs: Some(puzzle3d_kind_catalogs(document, catalogs)?), kind_compatibility },
        objects: document.parts.iter().map(|part| puzzle3d_object(part, document.kind_catalogs.as_ref())).collect(),
        attractions: document.fasteners.iter().map(puzzle3d_attraction).collect(),
        target_volumes: document.target_volumes.iter().map(puzzle3d_target_volume).collect(),
        references: Vec::new(),
    };
    dsl::FromValue::from_value(dsl::ToValue::to_value(&typed)).map_err(|error: dsl::ValueError| Fault::from(format!("puzzle5d-planner-snapshot: {error}")))
}

/// 🗂️ The authored kind catalogs of a 5d play snapshot (`kindCatalogs` child plus its `kindCatalogsExtra` rows),
/// or `None` when it authors none.
pub fn puzzle5d_authored_kind_catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Result<Option<crate::Puzzle5dKindCatalogs>, Fault> {
    let typed: crate::Puzzle5dSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(snapshot.value())).map_err(|error: dsl::ValueError| Fault::from(format!("puzzle5d-planner-snapshot: {error}")))?;
    Ok(crate::kind_catalogs_of(&typed.kind_catalogs, &typed.kind_catalogs_extra).filter(|catalogs| !catalogs.parts.is_empty()))
}

/// 🧰️ The planner's kind catalogs for a 5d document: its authored catalogs under the engine's naming, or — for a
/// document that authors none — the kinds it already contains, each part kind with the grips of its first part
/// as vortex templates and every grip kind as a vortex kind, so such a document fills with its own kinds.
pub fn puzzle3d_kind_catalogs(document: &Puzzle5dDocument, authored: Option<crate::Puzzle5dKindCatalogs>) -> Result<Puzzle3dKindCatalogs, Fault> {
    if let Some(authored) = authored {
        let renamed = puzzle3d_catalog_names(dsl::ToValue::to_value(&authored), 0);
        return dsl::FromValue::from_value(renamed).map_err(|error: dsl::ValueError| Fault::from(format!("puzzle5d-planner-kind-catalogs: {error}")));
    }
    let mut catalogs = Puzzle3dKindCatalogs::default();
    let mut kinds = std::collections::HashSet::new();
    let mut grip_kinds = std::collections::BTreeSet::new();
    for part in &document.parts {
        grip_kinds.extend(part.grips.iter().map(engine_grip_kind).filter(|kind| !kind.is_empty()));
        if !kinds.insert(part.part_kind.clone()) {
            continue;
        }
        catalogs.objects.push(Puzzle3dCatalogObjectKind {
            id: part.part_kind.clone(),
            name: part.part_kind.clone(),
            label: part.part_kind.clone(),
            description: String::new(),
            icon: String::new(),
            image: String::new(),
            unit: String::new(),
            is_abstract: false,
            base_kinds: Vec::new(),
            representations: resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).map(|url| Puzzle3dRepresentation { id: format!("{}:rep0", part.part_kind), name: "default".into(), url, mime: String::new(), tags: Vec::new(), lod: None, description: String::new() }).into_iter().collect(),
            vortices: part
                .grips
                .iter()
                .map(|grip| Puzzle3dCatalogVortexTemplate { id: grip.id.clone(), name: grip.id.clone(), label: grip.id.clone(), description: String::new(), icon: String::new(), vortex_kind: Some(engine_grip_kind(grip)).filter(|kind| !kind.is_empty()), point: grip.grip_3d.position, direction: grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]), t: None, mandatory: None, radius: (grip.grip_3d.radius > 0.0).then_some(grip.grip_3d.radius) })
                .collect(),
            attributes: Vec::new(),
            authors: Vec::new(),
        });
    }
    catalogs.vortices = grip_kinds.into_iter().map(|kind| Puzzle3dCatalogVortexKind { id: kind.clone(), code: Some(kind.clone()), label: Some(kind), order: None, compatible_with: Vec::new(), description: String::new(), icon: String::new(), color: String::new(), default_cable_kind: String::new() }).collect();
    Ok(catalogs)
}

/// 🔤️ Renames a 5d catalog value into the engine's naming: `parts`/`grips`/`ropes`/`fasteners` at the top,
/// `grips` inside a part kind, and the grip, rope and fastener kind references inside their rows.
fn puzzle3d_catalog_names(value: dsl::DslValue, depth: usize) -> dsl::DslValue {
    match value {
        dsl::DslValue::Object(fields) => dsl::DslValue::Object(
            fields
                .into_iter()
                .map(|(key, value)| {
                    let key = match (depth, key.as_str()) {
                        (0, "parts") => "objects".to_string(),
                        (0, "grips") | (2, "grips") => "vortices".to_string(),
                        (0, "ropes") => "cables".to_string(),
                        (0, "fasteners") => "attractions".to_string(),
                        (_, "gripKind") => "vortexKind".to_string(),
                        (_, "defaultRopeKind") => "defaultCableKind".to_string(),
                        (_, "defaultFastenerKind") => "defaultAttractionKind".to_string(),
                        _ => key,
                    };
                    (key, puzzle3d_catalog_names(value, depth + 1))
                })
                .collect(),
        ),
        dsl::DslValue::Array(items) => dsl::DslValue::Array(items.into_iter().map(|item| puzzle3d_catalog_names(item, depth)).collect()),
        other => other,
    }
}

pub(crate) fn puzzle3d_object(part: &Puzzle5dPart, kind_catalogs: Option<&serde_json::Value>) -> Puzzle3dObject {
    Puzzle3dObject {
        id: part.id.clone(),
        label: None,
        object_kind: Some(part.part_kind.clone()),
        anchor: Default::default(),
        origin: part.part_3d.origin,
        orientation: part.part_3d.orientation,
        scale: part.part_3d.scale.as_ref().and_then(puzzle3d_scale),
        mesh_url: resolve_part_mesh_url(part, kind_catalogs),
        vortices: part
            .grips
            .iter()
            .map(|grip| Puzzle3dVortex { id: grip.id.clone(), vortex_kind: Some(engine_grip_kind(grip)), label: None, position: grip.grip_3d.position, direction: grip.grip_3d.direction, radius: (grip.grip_3d.radius > 0.0).then_some(grip.grip_3d.radius), hidden: false, locked: false })
            .collect(),
        hidden: false,
        locked: false,
    }
}

fn puzzle3d_scale(scale: &serde_json::Value) -> Option<Puzzle3dScale> {
    match scale {
        serde_json::Value::Number(factor) => factor.as_f64().map(Puzzle3dScale::Uniform),
        serde_json::Value::Array(axes) if axes.len() >= 3 => Some(Puzzle3dScale::Vec3([axes[0].as_f64()?, axes[1].as_f64()?, axes[2].as_f64()?])),
        _ => None,
    }
}

/// 🧊️ One 5d target volume as the planner's own: the box is already stated in the 3d pose space both
/// artifacts share, so the bridge renames the type and nothing else. A HIDDEN volume still constrains
/// — `hidden` is a paint flag, and dropping it here would silently widen the fill region.
pub(crate) fn puzzle3d_target_volume(volume: &crate::editor::puzzle5d::Puzzle5dTargetVolume) -> semio_s_artifact_puzzle_3d::Puzzle3dTargetVolume {
    semio_s_artifact_puzzle_3d::Puzzle3dTargetVolume {
        id: volume.id.clone(),
        origin: volume.origin,
        orientation: volume.orientation,
        scale: volume.scale.as_ref().and_then(puzzle3d_scale),
        hidden: volume.hidden,
        locked: volume.locked,
    }
}

fn puzzle3d_attraction(fastener: &Puzzle5dFastener) -> Puzzle3dAttraction {
    Puzzle3dAttraction { id: fastener.id.clone(), attracting: fastener.source.clone(), attracted: fastener.target.clone(), gap: fastener.gap, shift: fastener.shift, rise: fastener.rise, rotation: fastener.rotation, turn: fastener.turn, tilt: fastener.tilt, x: fastener.x, y: fastener.y }
}

pub(crate) fn editor_part(part: &crate::Puzzle5dPart) -> Result<Puzzle5dPart, Fault> {
    serde_json::from_value(serde_json::Value::from(&dsl::ToValue::to_value(part))).map_err(|error| Fault::from(format!("puzzle5d-planner-part: {error}")))
}

fn schema_part(part: &Puzzle5dPart) -> Result<crate::Puzzle5dPart, Fault> {
    let value = serde_json::to_value(part).map_err(|error| Fault::from(format!("puzzle5d-planner-part: {error}")))?;
    dsl::FromValue::from_value(dsl::DslValue::from(&value)).map_err(|error: dsl::ValueError| Fault::from(format!("puzzle5d-planner-part: {error}")))
}
//#endregion 🔖️Document

//#region 🔖️Board
/// 🧩️ One part as the board synthesis sees it: flat center and radius, its kind and grips.
#[derive(Clone, Debug)]
struct Puzzle5dPlannerBoardPart {
    part_kind: String,
    center: [f64; 2],
    radius: f64,
    grips: Vec<(String, f64, [f64; 3])>,
    templates: Vec<Puzzle5dGrip2d>,
}

/// 🗺️ A run's board: the committed parts plus every provisional placement in op order, with a grip grid for host
/// lookups. Retracting truncates the provisional tail.
#[derive(Clone, Debug)]
pub struct Puzzle5dPlannerBoard {
    catalogs: Puzzle5dDocument,
    parts: Vec<Puzzle5dPlannerBoardPart>,
    committed: usize,
    grips: HashMap<String, (usize, f64)>,
    grid: Puzzle5dPointGrid<(u32, u32)>,
    kinds: HashMap<String, usize>,
}

impl Puzzle5dPlannerBoard {
    /// 🏗️ The board of `document` with the run's `provisional` placements already on it.
    pub fn new(document: &Puzzle5dDocument, provisional: &[Puzzle5dMutation]) -> Result<Self, Fault> {
        let catalogs = Puzzle5dDocument { parts: Vec::new(), fasteners: Vec::new(), ..document.clone() };
        let mut board = Self { catalogs, parts: Vec::with_capacity(document.parts.len()), committed: document.parts.len(), grips: HashMap::new(), grid: Puzzle5dPointGrid::new(PUZZLE5D_PLANNER_HOST_GRIP_CELL), kinds: HashMap::new() };
        for part in &document.parts {
            board.push(part);
        }
        for mutation in provisional {
            if let Puzzle5dMutation::CreatePart(create) = mutation {
                board.push(&editor_part(&create.part)?);
            }
        }
        Ok(board)
    }

    fn push(&mut self, part: &Puzzle5dPart) {
        let index = self.parts.len();
        let radius = if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
        let grips: Vec<(String, f64, [f64; 3])> = part.grips.iter().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), grip.grip_2d.angle, world_grip_position(part, grip))).collect();
        for (grip_index, (full_id, angle, world)) in grips.iter().enumerate() {
            self.grips.insert(full_id.clone(), (index, *angle));
            self.grid.insert(*world, (index as u32, grip_index as u32));
        }
        self.kinds.entry(part.part_kind.clone()).or_insert(index);
        self.parts.push(Puzzle5dPlannerBoardPart { part_kind: part.part_kind.clone(), center: [part.part_2d.x, part.part_2d.y], radius, grips, templates: part.grips.iter().map(|grip| grip.grip_2d.clone()).collect() });
    }

    /// 🪚️ Keeps the committed parts and the first `placements` provisional ones.
    fn truncate(&mut self, placements: usize) {
        while self.parts.len() > self.committed + placements {
            let Some(part) = self.parts.pop() else { break };
            let index = self.parts.len();
            for (grip_index, (full_id, _, world)) in part.grips.iter().enumerate() {
                self.grips.remove(full_id);
                self.grid.remove(*world, (index as u32, grip_index as u32));
            }
            if self.kinds.get(&part.part_kind) == Some(&index) {
                self.kinds.remove(&part.part_kind);
            }
        }
    }

    pub fn placements(&self) -> usize {
        self.parts.len() - self.committed
    }

    /// 🧷️ The flat center a new part of `radius` takes next to its host grip: out along the host grip's flat
    /// angle, one gap past both rims — the board synthesis every adopted planner placement uses.
    fn beside(&self, host: (usize, f64), radius: f64) -> [f64; 2] {
        let part = &self.parts[host.0];
        let distance = radius + part.radius + PUZZLE5D_BOARD_PLACEMENT_GAP;
        [part.center[0] + host.1.cos() * distance, part.center[1] + host.1.sin() * distance]
    }

    /// 🏷️ The display label the next placement of `part_kind` takes: this board already carries every
    /// committed part and every provisional placement of the run, so a filled part is numbered in the same
    /// series a hand-placed one would join (`puzzle5d_next_part_label`'s contract, over the board).
    fn next_label(&self, part_kind: &str) -> String {
        let base = crate::editor::puzzle5d::puzzle5d_kind_catalog_label(&self.catalogs, part_kind);
        let peers = self.parts.iter().filter(|part| part.part_kind == part_kind).count();
        if peers == 0 { base } else { format!("{base} {}", peers.saturating_add(1)) }
    }

    /// 🧲️ The flat center of a candidate posed at `origin`: beside the grip nearest to it.
    fn candidate_center(&self, origin: [f64; 3]) -> [f64; 2] {
        match self.grid.nearest(origin, PUZZLE5D_PLANNER_HOST_GRIP_REACH) {
            Some((_, (part, grip))) => self.beside((part as usize, self.parts[part as usize].grips[grip as usize].1), PUZZLE5D_DEFAULT_PART_RADIUS),
            None => self.parts.first().map_or([0.0, 0.0], |part| part.center),
        }
    }

    /// 🌱️ The 5d part and fastener one planner placement becomes, placed on the board.
    fn adopt(&mut self, object: &Puzzle3dObject, connect: &ConnectVortices) -> Result<(Puzzle5dPart, Puzzle5dFastener), Fault> {
        let part_kind = object.object_kind.clone().unwrap_or_else(|| "Part".into());
        let templates: Vec<Puzzle5dGrip2d> = match self.kinds.get(&part_kind) {
            Some(index) => self.parts[*index].templates.clone(),
            None => grips_from_templates(&self.catalogs, &part_kind).into_iter().map(|grip| grip.grip_2d).collect(),
        };
        let grips: Vec<Puzzle5dGrip> = object
            .vortices
            .iter()
            .enumerate()
            .map(|(index, vortex)| Puzzle5dGrip {
                id: vortex.id.clone(),
                grip_kind: vortex.vortex_kind.clone().unwrap_or_else(|| "grip".into()),
                grip_2d: templates.get(index).cloned().unwrap_or_default(),
                grip_3d: Puzzle5dGrip3d { position: vortex.position, direction: vortex.direction, radius: vortex.radius.unwrap_or(0.36), label: vortex.label.clone() },
            })
            .collect();
        let own = |grip: &str| grip.split_once(':').is_some_and(|(part, _)| part == object.id);
        let host = [&connect.attracting, &connect.attracted].into_iter().find(|grip| !own(grip)).and_then(|grip| self.grips.get(grip.as_str()).copied()).ok_or_else(|| Fault::from("puzzle5d-planner-host-grip"))?;
        let center = self.beside(host, PUZZLE5D_DEFAULT_PART_RADIUS);
        let label = object.label.clone().filter(|label| !label.is_empty()).unwrap_or_else(|| self.next_label(&part_kind));
        let part = Puzzle5dPart {
            id: object.id.clone(),
            part_kind: part_kind.clone(),
            anchor: Default::default(),
            part_2d: Puzzle5dPart2d { x: center[0], y: center[1], shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d {
                origin: object.origin,
                mesh_url: object.mesh_url.clone(),
                orientation: object.orientation.or(Some([0.0, 0.0, 0.0, 1.0])),
                scale: object.scale.as_ref().map(|scale| match scale {
                    Puzzle3dScale::Uniform(factor) => serde_json::json!(factor),
                    Puzzle3dScale::Vec3(axes) => serde_json::json!(axes),
                }),
                label: Some(label),
            },
            grips,
        };
        let fastener = Puzzle5dFastener { id: connect.id.clone(), source: connect.attracting.clone(), target: connect.attracted.clone(), fastener_kind: None, gap: connect.gap, shift: connect.shift, rise: connect.rise, rotation: connect.rotation, turn: connect.turn, tilt: connect.tilt, x: connect.x, y: connect.y };
        self.push(&part);
        Ok((part, fastener))
    }

    /// 🖌️ The 5d part and fastener a brush suggestion becomes, placed on the board: the candidate kind posed as the
    /// search posed it, its grips from the planner's kind catalog, docked from its source grip onto the target.
    pub fn adopt_suggestion(&mut self, catalogs: &Puzzle3dKindCatalogs, preview: &BrushPreviewState, part_id: String, fastener_id: String) -> Result<(Puzzle5dPart, Puzzle5dFastener), Fault> {
        let kind = catalogs.objects.iter().find(|kind| kind.id == preview.object_kind_id).ok_or_else(|| Fault::from("puzzle5d-planner-suggestion-kind"))?;
        let source = kind.vortices.get(preview.source_vortex_index).ok_or_else(|| Fault::from("puzzle5d-planner-suggestion-grip"))?;
        let object = Puzzle3dObject {
            id: part_id.clone(),
            label: None,
            object_kind: Some(preview.object_kind_id.clone()),
            anchor: Default::default(),
            origin: preview.origin,
            orientation: Some(preview.orientation),
            scale: preview.scale.clone().and_then(|scale| dsl::FromValue::from_value(scale).ok()),
            mesh_url: Some(preview.mesh_url.clone()),
            vortices: kind.vortices.iter().map(|template| Puzzle3dVortex { id: template.id.clone(), vortex_kind: template.vortex_kind.clone(), label: None, position: template.point, direction: Some(template.direction), radius: template.radius, hidden: false, locked: false }).collect(),
            hidden: false,
            locked: false,
        };
        let connect = ConnectVortices { id: fastener_id, attracting: preview.target_vortex_full_id.clone(), attracted: puzzle5d_grip_full_id(&part_id, &source.id), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 };
        self.adopt(&object, &connect)
    }

    /// 🔁️ One planner tick as the 5d tick: the provisional tail retracted, every placement pair translated and
    /// placed, and every `instance3d` record paired with its `placement2d` twin (a placed part's twin sits exactly
    /// on the part).
    pub fn translate(&mut self, tick: ToolRunTick) -> Result<ToolRunTick, Fault> {
        if let Some(length) = tick.retract_to {
            self.truncate(length as usize / PUZZLE5D_PLACEMENT_OPS);
        }
        if tick.append_ops.len() % PUZZLE5D_PLACEMENT_OPS != 0 {
            return Err(Fault::from("puzzle5d-planner-tick-ops"));
        }
        let mut append_ops = Vec::with_capacity(tick.append_ops.len());
        let mut placed: Vec<([f32; 3], [f64; 2])> = Vec::new();
        for pair in tick.append_ops.chunks(PUZZLE5D_PLACEMENT_OPS) {
            let decode = |bytes: &[u8]| <Puzzle3dMutation as protocol::OpBinary>::decode_op(bytes).map_err(|error| Fault::from(format!("puzzle5d-planner-tick-op: {error}")));
            let (Puzzle3dMutation::CreateObject(create), Puzzle3dMutation::ConnectVortices(connect)) = (decode(&pair[0])?, decode(&pair[1])?) else {
                return Err(Fault::from("puzzle5d-planner-tick-placement"));
            };
            let (part, fastener) = self.adopt(&create.object, &connect)?;
            placed.push((create.object.origin.map(|axis| axis as f32), [part.part_2d.x, part.part_2d.y]));
            let encode = |mutation: Puzzle5dMutation| <Puzzle5dMutation as protocol::OpBinary>::encode_op(&mutation).map_err(|error| Fault::from(format!("puzzle5d-planner-tick-op: {error}")));
            append_ops.push(encode(create_part(schema_part(&part)?, None))?);
            append_ops.push(encode(connect_grips(fastener.id, fastener.source, fastener.target, fastener.fastener_kind, fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y))?);
        }
        let mut trace = Vec::with_capacity(tick.trace.len());
        for page in tick.trace {
            let mut ops = Vec::with_capacity(page.ops.len() * 2);
            for op in page.ops {
                ops.push(op);
                match op {
                    ToolRunTraceOp::Upsert { key, verdict, reason, subject: ToolRunTraceSubject::Instance3d { position, .. } } => {
                        let twin = Self::twin(key)?;
                        let accepted = (verdict == ToolRunVerdict::Success).then(|| placed.iter().position(|(origin, _)| *origin == position)).flatten().map(|index| placed.remove(index).1);
                        let center = accepted.unwrap_or_else(|| self.candidate_center(position.map(f64::from)));
                        ops.push(ToolRunTraceOp::Upsert { key: twin, verdict, reason, subject: ToolRunTraceSubject::Placement2d { shape: PUZZLE5D_PLANNER_TRACE_SHAPE_CIRCLE, position: center.map(|axis| axis as f32), rotation: 0.0 } });
                    }
                    ToolRunTraceOp::Retire { key } => ops.push(ToolRunTraceOp::Retire { key: Self::twin(key)? }),
                    _ => {}
                }
            }
            for chunk in ops.chunks(TOOL_RUN_TRACE_PAGE_OPS_MAX) {
                trace.push(ToolRunTracePage { identity: page.identity, page: page.page, ops: chunk.to_vec() });
            }
        }
        Ok(ToolRunTick { append_ops, trace, ..tick })
    }

    fn twin(key: u64) -> Result<u64, Fault> {
        if key & PUZZLE5D_PLANNER_TRACE_TWIN_BIT != 0 {
            return Err(Fault::from("puzzle5d-planner-trace-key-space"));
        }
        Ok(key | PUZZLE5D_PLANNER_TRACE_TWIN_BIT)
    }
}
//#endregion 🔖️Board

//#region 🔖️Job
/// ⏯️ A 5d planner run job as the framework ledger steps it (fill run, fill revalidation, brush suggestions): the
/// planner job steps inside its own
/// step context under this step's fuel, deadline and cancellation, and every tick it reports is translated
/// onto the 5d document before it leaves the job — one job payload page per step, so a translated tick larger
/// than a page leaves as consecutive ticks over the following steps, before the planner steps again.
pub struct Puzzle5dPlannerToolRunJob {
    inner: ToolRunJob,
    board: Puzzle5dPlannerBoard,
    pending: VecDeque<Vec<u8>>,
    preview_sequence: u64,
    pub(crate) clock: fn() -> Option<u64>,
}

impl Puzzle5dPlannerToolRunJob {
    pub fn new(inner: ToolRunJob, board: Puzzle5dPlannerBoard) -> Self {
        Self { inner, board, pending: VecDeque::new(), preview_sequence: 0, clock: semio_framework_job::default_now_us }
    }

    fn fault(context: &mut StepContext<'_>, fault: Fault) -> StepOutcome {
        let detail = context.payload_from_bytes(JobPayloadStream::Fault, fault.message.as_bytes()).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(JobPayloadStream::Fault)
        });
        StepOutcome::Fault(JobFault { detail })
    }

    fn admit(context: &mut StepContext<'_>, bytes: &[u8]) -> StepOutcome {
        match context.payload_from_bytes(JobPayloadStream::Preview, bytes) {
            Ok(payload) => StepOutcome::PreviewReady(payload),
            Err(rejected) => {
                drop(rejected.into_source());
                Self::fault(context, Fault::from("puzzle5d-planner-tick-admission"))
            }
        }
    }

    fn translate(&mut self, mut payload: RetainedJobPayload) -> Result<(), Fault> {
        let mut bytes = Vec::with_capacity(payload.len());
        for index in 0..payload.page_count() {
            bytes.extend_from_slice(payload.page(index).ok_or_else(|| Fault::from("puzzle5d-planner-tick-page"))?);
        }
        while !payload.terminal_is_empty() {
            payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        let tick = ToolRunTick::decode(&bytes).map_err(|error| Fault::from(format!("puzzle5d-planner-tick: {error}")))?;
        self.pending.extend(puzzle5d_planner_tick_pages(self.board.translate(tick)?)?);
        Ok(())
    }
}

/// 📃️ One translated tick as consecutive encoded ticks of at most one job payload page each: the first carries
/// the retraction, the placements and their entities, the last the steps and progress, and the trace records
/// are spread over all of them in order.
pub fn puzzle5d_planner_tick_pages(tick: ToolRunTick) -> Result<Vec<Vec<u8>>, Fault> {
    let encode = |tick: &ToolRunTick| tick.encode().map_err(|error| Fault::from(format!("puzzle5d-planner-tick: {error}")));
    let whole = encode(&tick)?;
    if whole.len() <= JOB_PAYLOAD_PAGE_BYTES {
        return Ok(vec![whole]);
    }
    let records: Vec<(ToolRunIdentity, u32, ToolRunTraceOp)> = tick.trace.iter().flat_map(|page| page.ops.iter().map(move |op| (page.identity, page.page, *op))).collect();
    for parts in (whole.len() / JOB_PAYLOAD_PAGE_BYTES + 1)..=records.len().max(1) {
        let chunk = records.len().div_ceil(parts).max(1);
        let ticks: Vec<ToolRunTick> = (0..parts)
            .map(|index| {
                let (first, last) = (index == 0, index + 1 == parts);
                let mut trace: Vec<ToolRunTracePage> = Vec::new();
                for (identity, page, op) in records.iter().skip(index * chunk).take(chunk) {
                    match trace.last_mut() {
                        Some(current) if current.identity == *identity && current.page == *page && current.ops.len() < TOOL_RUN_TRACE_PAGE_OPS_MAX => current.ops.push(*op),
                        _ => trace.push(ToolRunTracePage { identity: *identity, page: *page, ops: vec![*op] }),
                    }
                }
                ToolRunTick {
                    identity: tick.identity,
                    sequence: tick.sequence,
                    progress: if last { tick.progress.clone() } else { None },
                    steps: if last { tick.steps.clone() } else { Vec::new() },
                    trace,
                    append_ops: if first { tick.append_ops.clone() } else { Vec::new() },
                    append_entities: if first { tick.append_entities.clone() } else { Vec::new() },
                    retract_to: if first { tick.retract_to } else { None },
                    payload: if last { tick.payload.clone() } else { None },
                }
            })
            .collect();
        let encoded = ticks.iter().map(encode).collect::<Result<Vec<_>, _>>()?;
        if encoded.iter().all(|bytes| bytes.len() <= JOB_PAYLOAD_PAGE_BYTES) {
            return Ok(encoded);
        }
    }
    Err(Fault::from("puzzle5d-planner-tick-page-bytes"))
}

impl InteractiveJob for Puzzle5dPlannerToolRunJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if let Some(bytes) = self.pending.pop_front() {
            return Self::admit(context, &bytes);
        }
        let fuel = context.fuel_remaining();
        let mut planner = StepContext::new(context.operation(), context.generation(), StepBudget::new(fuel, context.deadline_us()), context.cancel_token(), self.clock, &mut self.preview_sequence);
        let outcome = self.inner.step(&mut planner);
        context.consume_fuel(fuel.saturating_sub(planner.fuel_remaining()));
        match outcome {
            StepOutcome::PreviewReady(payload) => match self.translate(payload) {
                Ok(()) => match self.pending.pop_front() {
                    Some(bytes) => Self::admit(context, &bytes),
                    None => StepOutcome::Yield,
                },
                Err(fault) => Self::fault(context, fault),
            },
            outcome => outcome,
        }
    }

    fn begin_close(&mut self) {
        self.pending.clear();
        self.inner.begin_close();
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.pending.clear();
        self.inner.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.pending.is_empty() && self.inner.terminal_is_empty()
    }
}
//#endregion 🔖️Job

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
