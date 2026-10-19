//! ➕️ Add gestures of the puzzle 5d editor kit: the retained add-node and add-brush-part work. See [`super`].
use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;

use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionWrite;
use semio_framework_value::FromValue;
use semio_framework_pack_json::Value;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::catalog::{puzzle5d_default_part_kind, puzzle5d_part_kind_options};
use crate::editor::puzzle5d::entities::{PUZZLE5D_DEFAULT_PART_RADIUS, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, puzzle5d_editor_projection, puzzle5d_grip_full_id, puzzle5d_value_as_f64_3, puzzle5d_value_as_f64_4, quat_rotate_vector};
use crate::editor::puzzle5d::gestures::proximity_connect::Puzzle5dProximityConnectWork;
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::catalog::{puzzle5d_next_part_label_from_projection};
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dAddNodeStage {
    Catalog,
    Grips,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dAddNodeWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dAddNodeWorkOwners {
    pub mutation: Option<Puzzle5dMutation>,
    pub grips: Vec<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>,
    pub mesh_url: Option<String>,
}

pub struct Puzzle5dAddNodeWork {
    pub stage: Puzzle5dAddNodeStage,
    catalog_cursor: usize,
    pub grip_cursor: usize,
    catalog_index: Option<usize>,
    pub mesh_url: Option<String>,
    pub grips: Vec<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>,
    pub mutation: Option<Puzzle5dMutation>,
    pub operation_nonce: u64,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dAddNodeWorkOwners>,
}

impl Default for Puzzle5dAddNodeWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dAddNodeStage::Catalog, catalog_cursor: 0, grip_cursor: 0, catalog_index: None, mesh_url: None, grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS), mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dAddNodeWork {
    pub fn part_kind(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("kind")).and_then(Value::as_str).unwrap_or("Part")
    }

    pub fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).cloned().unwrap_or_default()
    }
}

impl Puzzle5dAddNodeWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dAddNodeWork {
    fn tool_id(&self) -> &'static str {
        "addNode"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddNodeStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddNodeStage::Grips;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(Self::part_kind(command)) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddNodeStage::Grips;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-add-node-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddNodeStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-node-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip2d = match template.get("2d") {
                        Some(value) => <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip2d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip2d-malformed"))?,
                        None => Default::default(),
                    };
                    let grip_3d: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip3d = match template.get("3d") {
                        Some(value) => <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip3d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip3d-malformed"))?,
                        None => Default::default(),
                    };
                    self.grips.push(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip { id: format!("v{}", self.grip_cursor), grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                let x = command.args().and_then(|args| args.get("x")).and_then(Value::as_f64).unwrap_or(120.0);
                let y = command.args().and_then(|args| args.get("y")).and_then(Value::as_f64).unwrap_or(120.0);
                let flat_to_world = PUZZLE5D_FLAT_TO_WORLD;
                let origin = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| {
                    let peer_2d = peer.get("2d");
                    let peer_3d = peer.get("3d");
                    let peer_x = peer_2d.and_then(|part| part.get("x")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_y = peer_2d.and_then(|part| part.get("y")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_origin = peer_3d.and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
                    [peer_origin[0] + (x - peer_x) * flat_to_world, peer_origin[1] - (y - peer_y) * flat_to_world, peer_origin[2]]
                });
                let part_kind = Self::part_kind(command).to_string();
                let created_id = format!("part-{:016x}-0", self.operation_nonce);
                let part = crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart {
                    id: created_id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind.clone()), ..Default::default() },
                    part_3d: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), label: Some(puzzle5d_next_part_label_from_projection(&projection, &part_kind)), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None));
                self.stage = Puzzle5dAddNodeStage::Complete;
                // 🕹️ Re-select what this gesture just created — a palette drop that leaves the old selection
                // standing makes every follow-up verb (patch, transform, delete) act on the wrong part.
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: self.mutation.take().into_iter().collect(),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [created_id])],
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dAddNodeStage::Complete => Err(Fault::from("puzzle5d-add-node-complete-repolled")),
            Puzzle5dAddNodeStage::Closing => Err(Fault::from("puzzle5d-add-node-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddNodeStage::Closing;
        self.close_owners.stage(Puzzle5dAddNodeWorkOwners { mutation: std::mem::take(&mut self.mutation), grips: std::mem::take(&mut self.grips), mesh_url: std::mem::take(&mut self.mesh_url) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,_grant|{
            self.close_owners.close_step(cx)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddNodeStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Puzzle5dAddBrushPartStage {
    Catalog,
    Grips,
    Target,
    Create,
    Connect,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dAddBrushPartWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dAddBrushPartWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
    pub payload: Option<Value>,
    pub grips: Vec<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>,
    pub mesh_url: Option<String>,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    target_direction: Option<[f64; 3]>,
    created_id: Option<String>,
    created_grip_id: Option<String>,
}

pub struct Puzzle5dAddBrushPartWork {
    pub tool_id: &'static str,
    pub payload: Option<Value>,
    pub stage: Puzzle5dAddBrushPartStage,
    catalog_cursor: usize,
    pub grip_cursor: usize,
    pub part_cursor: usize,
    target_grip_cursor: usize,
    pub processed_units: usize,
    catalog_index: Option<usize>,
    pub mesh_url: Option<String>,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    target_direction: Option<[f64; 3]>,
    created_id: Option<String>,
    created_grip_id: Option<String>,
    pub grips: Vec<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>,
    pub mutations: Vec<Puzzle5dMutation>,
    pub operation_nonce: u64,
    pub fresh_cursor: u64,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dAddBrushPartWorkOwners>,
}

impl Puzzle5dAddBrushPartWork {
    pub fn is_closing(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing
    }

    pub fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            payload: None,
            stage: Puzzle5dAddBrushPartStage::Catalog,
            catalog_cursor: 0,
            grip_cursor: 0,
            part_cursor: 0,
            target_grip_cursor: 0,
            processed_units: 0,
            catalog_index: None,
            mesh_url: None,
            target_id: None,
            target_position: None,
            target_direction: None,
            created_id: None,
            created_grip_id: None,
            grips: Vec::new(),
            mutations: Vec::new(),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }



    /// 🌉️ `self.payload` (materialized once via `take_payload`/`parse`) and `command.args()` are
    /// both this file's own first-party `Value` now, so no bridge is needed — an in-progress board
    /// payload wins, falling back to the dispatch args only when there is none.
    pub fn args(&self, command: &Puzzle5dCommand) -> Option<Value> {
        self.payload.clone().or_else(|| command.args().cloned())
    }

    /// 🗂️ The kind this run adds: the caller's own `partKind`, else THIS tool's own declared select
    /// default — never a literal.
    ///
    /// 🐛️ It used to fall back to the string `"Part"`, the very literal
    /// `PUZZLE5D_SHIPPED_PART_KINDS` replaced in the select, so a bare `addPartKind` (the arg form
    /// dispatched with no args, which is what an agent sends) added a part of a kind no catalog
    /// declares. Measured 2026-09-22 (slice PZ2): `add_part_kind_materializes_the_declared_kind_default`
    /// read back `"Part"` where the declared default is `"Hexagonal Cut Concrete Forest Left"`.
    /// `addBrushPart` declares a one-row `"Part"` select of its own, so reading each tool's OWN
    /// declared default keeps that verb byte-identical while fixing this one.
    fn owned_part_kind(&self, command: &Puzzle5dCommand) -> String {
        self.args(command)
            .and_then(|args| args.get("partKind").or_else(|| args.get("objectKindId")).or_else(|| args.get("nodeKind")).and_then(Value::as_str).map(str::to_string))
            .filter(|kind| !kind.is_empty())
            .unwrap_or_else(|| if self.tool_id == "addPartKind" { puzzle5d_default_part_kind(&puzzle5d_part_kind_options()) } else { "Part".to_string() })
    }

    pub fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        Puzzle5dAddNodeWork::catalogs(snapshot)
    }

    pub fn target(&self, command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Option<String> {
        self.args(command)
            .and_then(|args| args.get("targetVortexFullId").or_else(|| args.get("targetGripFullId")).and_then(Value::as_str).map(str::to_string))
            .filter(|id| !id.is_empty())
            .or_else(|| interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_GRIP).and_then(|selection| selection.ids.first().cloned()))
    }

    fn world_direction(part: &Value, grip: &Value) -> [f64; 3] {
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let direction = grip.get("3d").and_then(|grip| grip.get("direction")).and_then(puzzle5d_value_as_f64_3).unwrap_or([0.0, 0.0, -1.0]);
        quat_rotate_vector(orientation, direction)
    }
}

impl Puzzle5dAddBrushPartWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dAddBrushPartWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?.checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-add-brush-part-work-capacity"));
        }
        self.processed_units += 1;
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddBrushPartStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(self.owned_part_kind(command).as_str()) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddBrushPartStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-brush-part-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d = template.get("2d").map(|value| <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip2d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip2d-malformed"))?.unwrap_or_default();
                    let grip_3d = template.get("3d").map(|value| <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip3d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip3d-malformed"))?.unwrap_or_default();
                    let id = format!("v{}", self.grip_cursor);
                    if self.created_grip_id.is_none() {
                        self.created_grip_id = Some(id.clone());
                    }
                    self.grips.push(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip { id, grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                self.target_id = self.target(command, interaction);
                self.stage = if self.target_id.is_some() { Puzzle5dAddBrushPartStage::Target } else { Puzzle5dAddBrushPartStage::Create };
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Target => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.target_id = None;
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.target_grip_cursor)) else {
                    self.part_cursor += 1;
                    self.target_grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"));
                };
                self.target_grip_cursor += 1;
                let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                if self.target_id.as_deref() == Some(puzzle5d_grip_full_id(part_id, grip_id).as_str()) {
                    self.target_position = Some(Puzzle5dProximityConnectWork::world_position(part, grip));
                    self.target_direction = Some(Self::world_direction(part, grip));
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Create => {
                let x = self.args(command).and_then(|args| args.get("x").and_then(Value::as_f64)).unwrap_or(120.0);
                let y = self.args(command).and_then(|args| args.get("y").and_then(Value::as_f64)).unwrap_or(120.0);
                let origin = match (self.target_position, self.target_direction) {
                    (Some(position), Some(direction)) => [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]],
                    _ => [x * PUZZLE5D_FLAT_TO_WORLD, -y * PUZZLE5D_FLAT_TO_WORLD, 0.0],
                };
                let part_kind = self.owned_part_kind(command);
                let id = self.args(command).and_then(|args| args.get("nodeId").or_else(|| args.get("partId")).or_else(|| args.get("objectId")).and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                    let id = format!("part-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                    self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                    id
                });
                let part = crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart {
                    id: id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind.clone()), ..Default::default() },
                    part_3d: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), label: Some(puzzle5d_next_part_label_from_projection(&projection, &part_kind)), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.created_id = Some(id);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None));
                self.stage = Puzzle5dAddBrushPartStage::Connect;
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"))
            }
            Puzzle5dAddBrushPartStage::Connect => {
                if let (Some(source), Some(part), Some(grip)) = (self.target_id.as_ref(), self.created_id.as_ref(), self.created_grip_id.as_ref()) {
                    let id = self.args(command).and_then(|args| args.get("edgeId").and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                        let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                        self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        id
                    });
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source.clone(), puzzle5d_grip_full_id(part, grip), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                }
                self.stage = Puzzle5dAddBrushPartStage::Complete;
                // 🕹️ Re-select the placed part so the brush's next gesture chains off it, exactly as puzzle
                // 3d's `addBrushObject` does.
                let interaction_writes = self.created_id.take().map(|id| InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])).into_iter().collect();
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes, ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dAddBrushPartStage::Complete => Err(Fault::from("puzzle5d-add-brush-part-complete-repolled")),
            Puzzle5dAddBrushPartStage::Closing => Err(Fault::from("puzzle5d-add-brush-part-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddBrushPartStage::Closing;
        self.close_owners.stage(Puzzle5dAddBrushPartWorkOwners { mutations: std::mem::take(&mut self.mutations), payload: std::mem::take(&mut self.payload), grips: std::mem::take(&mut self.grips), mesh_url: std::mem::take(&mut self.mesh_url), target_id: std::mem::take(&mut self.target_id), target_position: std::mem::take(&mut self.target_position), target_direction: std::mem::take(&mut self.target_direction), created_id: std::mem::take(&mut self.created_id), created_grip_id: std::mem::take(&mut self.created_grip_id) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,_grant|{
            self.close_owners.close_step(cx)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp

impl Puzzle5dAddNodeWork {
    pub fn original_birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  let capacity=0_usize.checked_add((crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS as usize).checked_mul(std::mem::size_of::<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Puzzle5D vector extent overflow"))?).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Puzzle5D vector extent overflow"))?;
  Ok(semio_framework_value::RetirementDemand{capacity_bytes:capacity,depth:1,..Default::default()})
 }

    pub fn from_original_construction(scope:&mut semio_framework_action_bus::OriginalToolConstructionScope<'_>)->Result<(Self,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{
  scope.admit_owner(Self::original_birth_demands()?)?;
  let work=Self{grips:Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),..Self::default()};let capacity=work.grips.capacity()*std::mem::size_of::<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>();
  Ok((work,semio_framework_value::RetainedCloneProgress{retained_capacity_bytes:capacity,..Default::default()}))
 }
}

impl Puzzle5dAddBrushPartWork {
    pub fn original_birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  let capacity=0_usize.checked_add((crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS as usize).checked_mul(std::mem::size_of::<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Puzzle5D vector extent overflow"))?).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Puzzle5D vector extent overflow"))?.checked_add((2 as usize).checked_mul(std::mem::size_of::<Puzzle5dMutation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Puzzle5D vector extent overflow"))?).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Puzzle5D vector extent overflow"))?;
  Ok(semio_framework_value::RetirementDemand{capacity_bytes:capacity,depth:1,..Default::default()})
 }

    pub fn from_original_construction(tool_id: &'static str, scope:&mut semio_framework_action_bus::OriginalToolConstructionScope<'_>)->Result<(Self,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{
  scope.admit_owner(Self::original_birth_demands()?)?;
  let work=Self{grips:Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),mutations:Vec::with_capacity(2),..Self::new(tool_id)};let capacity=work.grips.capacity()*std::mem::size_of::<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip>() + work.mutations.capacity()*std::mem::size_of::<Puzzle5dMutation>();
  Ok((work,semio_framework_value::RetainedCloneProgress{retained_capacity_bytes:capacity,..Default::default()}))
 }
}
