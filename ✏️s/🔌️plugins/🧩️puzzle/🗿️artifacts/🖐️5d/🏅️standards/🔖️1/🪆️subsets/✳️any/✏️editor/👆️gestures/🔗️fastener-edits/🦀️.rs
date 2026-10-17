//! 🔗️ Fastener edit gestures of the puzzle 5d editor: the retained edit, retarget and delete fastener work. See [`super`].

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_pack_json::Value;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::entities::{Puzzle5dFastener, puzzle5d_editor_projection, puzzle5d_grip_full_id, puzzle5d_record_from_projection, puzzle5d_resolve_number_edit};
use crate::editor::puzzle5d::kit::board_events_and_fasteners::Puzzle5dGripScan;
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dEditFastenerStage {
    Scan,
    Kind,
    Geometry,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dEditFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dEditFastenerWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
    pub fastener: Option<Puzzle5dFastener>,
}

pub struct Puzzle5dEditFastenerWork {
    pub stage: Puzzle5dEditFastenerStage,
    pub cursor: usize,
    pub fastener: Option<Puzzle5dFastener>,
    pub mutations: Vec<Puzzle5dMutation>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dEditFastenerWorkOwners>,
}

impl Default for Puzzle5dEditFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dEditFastenerStage::Scan, cursor: 0, fastener: None, mutations: Vec::with_capacity(2) }
    }
}

impl Puzzle5dEditFastenerWork {
    pub fn id(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    fn updated_kind(command: &Puzzle5dCommand, current: &Puzzle5dFastener) -> Option<Option<String>> {
        let args = command.args()?;
        let mut update = args.get("fastenerKind").or_else(|| args.get("edgeKind")).and_then(Value::as_str).filter(|text| !text.is_empty()).map(|text| Some(text.to_string()));
        if matches!(args.get("field").and_then(Value::as_str), Some("fastenerKind" | "edgeKind")) {
            update = Some(args.get("value").and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string));
        }
        update.filter(|updated| updated != &current.fastener_kind)
    }

    fn updated_geometry(command: &Puzzle5dCommand, current: &Puzzle5dFastener) -> Option<[f64; 8]> {
        let args = command.args()?;
        let mut geometry = [current.gap, current.shift, current.rise, current.rotation, current.turn, current.tilt, current.x, current.y];
        let keys = ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"];
        let mut changed = false;
        for (index, key) in keys.iter().enumerate() {
            if let Some(value) = args.get(key) {
                if let Some(updated) = puzzle5d_resolve_number_edit(geometry[index], Some(value), None) {
                    changed |= updated != geometry[index];
                    geometry[index] = updated;
                }
            }
        }
        if let Some(index) = keys.iter().position(|key| args.get("field").and_then(Value::as_str) == Some(*key)) {
            if let Some(updated) = puzzle5d_resolve_number_edit(geometry[index], args.get("value"), args.get("delta")) {
                changed |= updated != geometry[index];
                geometry[index] = updated;
            }
        }
        changed.then_some(geometry)
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dEditFastenerWork {
    fn tool_id(&self) -> &'static str {
        "editFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len).checked_add(2).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dEditFastenerStage::Scan => {
                let target = Self::id(command);
                if target.is_empty() {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)).cloned() else {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                };
                self.cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(target) {
                    self.fastener = Some(puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-edit-fastener-malformed"))?);
                    self.stage = Puzzle5dEditFastenerStage::Kind;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-edit-fastener-scan", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dEditFastenerStage::Kind => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(kind) = Self::updated_kind(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), kind));
                }
                self.stage = Puzzle5dEditFastenerStage::Geometry;
                Ok(crate::puzzle_progress_step!("puzzle5d-edit-fastener-kind", "Updating fastener kind", "Verbindungsart wird aktualisiert"))
            }
            Puzzle5dEditFastenerStage::Geometry => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(geometry) = Self::updated_geometry(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }));
                }
                self.stage = Puzzle5dEditFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dEditFastenerStage::Complete => Err(Fault::from("puzzle5d-edit-fastener-complete-repolled")),
            Puzzle5dEditFastenerStage::Closing => Err(Fault::from("puzzle5d-edit-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dEditFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dEditFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), fastener: std::mem::take(&mut self.fastener) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle5dEditFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dRetargetFastenerStage {
    Fastener,
    SourceGrip,
    TargetGrip,
    Duplicate,
    Compatibility,
    Disconnect,
    Connect,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dRetargetFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dRetargetFastenerWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
    pub fastener: Option<Puzzle5dFastener>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub source_kind: Option<String>,
    pub target_kind: Option<String>,
}

pub struct Puzzle5dRetargetFastenerWork {
    pub stage: Puzzle5dRetargetFastenerStage,
    pub part_cursor: usize,
    pub grip_cursor: usize,
    pub fastener_cursor: usize,
    pub compatibility_cursor: usize,
    pub processed_units: usize,
    pub fastener: Option<Puzzle5dFastener>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub source_kind: Option<String>,
    pub target_kind: Option<String>,
    pub mutations: Vec<Puzzle5dMutation>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dRetargetFastenerWorkOwners>,
}

impl Default for Puzzle5dRetargetFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dRetargetFastenerStage::Fastener,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            compatibility_cursor: 0,
            processed_units: 0,
            fastener: None,
            source: None,
            target: None,
            source_kind: None,
            target_kind: None,
            mutations: Vec::with_capacity(2),
        }
    }
}

impl Puzzle5dRetargetFastenerWork {
    pub fn argument<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    pub fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_editor_projection(snapshot);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    pub fn complete_empty<A: Puzzle5dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle5dRetargetFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dRetargetFastenerWork {
    fn tool_id(&self) -> &'static str {
        "retargetFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners.checked_mul(2)?)?.checked_add(compatibility)?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-retarget-fastener-work-capacity"));
        }
        self.processed_units += 1;
        match self.stage {
            Puzzle5dRetargetFastenerStage::Fastener => {
                let Some(id) = Self::argument(command, "id", "fastenerId") else { return Ok(self.complete_empty()) };
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    return Ok(self.complete_empty());
                };
                self.fastener_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(id) {
                    let fastener: Puzzle5dFastener = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-retarget-fastener-malformed"))?;
                    self.source = Some(Self::argument(command, "source", "attracting").map_or_else(|| fastener.source.clone(), str::to_string));
                    self.target = Some(Self::argument(command, "target", "attracted").map_or_else(|| fastener.target.clone(), str::to_string));
                    if self.source.as_deref().is_none_or(str::is_empty) || self.target.as_deref().is_none_or(str::is_empty) || self.source == self.target {
                        return Ok(self.complete_empty());
                    }
                    self.fastener = Some(fastener);
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dRetargetFastenerStage::SourceGrip;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-fastener", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dRetargetFastenerStage::SourceGrip => {
                let source = self.source.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &source) {
                    Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-retarget-source", "Finding source grip", "Quellgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.source_kind = kind;
                        self.part_cursor = 0;
                        self.grip_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::TargetGrip;
                        Ok(crate::puzzle_progress_step!("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::TargetGrip => {
                let target = self.target.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &target) {
                    Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.target_kind = kind;
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::Duplicate;
                        Ok(crate::puzzle_progress_step!("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::Duplicate => {
                if let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let id = row.get("id").and_then(Value::as_str).unwrap_or("");
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                    let own_id = self.fastener.as_ref().map_or("", |fastener| fastener.id.as_str());
                    let next_source = self.source.as_deref().unwrap_or("");
                    let next_target = self.target.as_deref().unwrap_or("");
                    if id != own_id && ((source == next_source && target == next_target) || (source == next_target && target == next_source)) {
                        return Ok(self.complete_empty());
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dRetargetFastenerStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-retarget-disconnect", "Disconnecting old fastener", "Alte Verbindung wird getrennt"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else { return Ok(self.complete_empty()) };
                self.compatibility_cursor += 1;
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.source_kind.as_deref().unwrap_or("");
                let target_kind = self.target_kind.as_deref().unwrap_or("");
                if (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind) {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Disconnect => {
                let id = self.fastener.as_ref().map(|fastener| fastener.id.clone()).ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id));
                self.stage = Puzzle5dRetargetFastenerStage::Connect;
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-connect", "Connecting retargeted fastener", "Neu ausgerichtete Verbindung wird erstellt"))
            }
            Puzzle5dRetargetFastenerStage::Connect => {
                let fastener = self.fastener.as_ref().ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                    fastener.id.clone(),
                    self.source.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-retarget-source-owner"))?,
                    self.target.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-retarget-target-owner"))?,
                    fastener.fastener_kind.clone(),
                    fastener.gap,
                    fastener.shift,
                    fastener.rise,
                    fastener.rotation,
                    fastener.turn,
                    fastener.tilt,
                    fastener.x,
                    fastener.y, None,
                ));
                self.stage = Puzzle5dRetargetFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dRetargetFastenerStage::Complete => Err(Fault::from("puzzle5d-retarget-fastener-complete-repolled")),
            Puzzle5dRetargetFastenerStage::Closing => Err(Fault::from("puzzle5d-retarget-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRetargetFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dRetargetFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), fastener: std::mem::take(&mut self.fastener), source: std::mem::take(&mut self.source), target: std::mem::take(&mut self.target), source_kind: std::mem::take(&mut self.source_kind), target_kind: std::mem::take(&mut self.target_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle5dRetargetFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dDeleteFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dDeleteFastenerWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
}

pub struct Puzzle5dDeleteFastenerWork {
    pub cursor: usize,
    pub closing: bool,
    pub mutations: Vec<Puzzle5dMutation>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dDeleteFastenerWorkOwners>,
}

impl Default for Puzzle5dDeleteFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), cursor: 0, closing: false, mutations: Vec::with_capacity(1) }
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dDeleteFastenerWork {
    fn tool_id(&self) -> &'static str {
        "deleteFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("fasteners").and_then(Value::as_array).map_or(Some(1), |fasteners| fasteners.len().checked_add(1)).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        let target = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty());
        let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)) else {
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
        };
        self.cursor += 1;
        if target == row.get("id").and_then(Value::as_str) {
            if let Some(id) = target {
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()));
            }
        }
        Ok(crate::puzzle_progress_step!("puzzle5d-delete-fastener", "Scanning fastener", "Verbindung wird geprüft"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.close_owners.stage(Puzzle5dDeleteFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
