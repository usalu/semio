//! 🧲️ Proximity connect gesture of the puzzle 5d editor: the retained proximity-connect work. See [`super`].

use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_pack_json::Value;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::entities::{PUZZLE5D_PROXIMITY_RADIUS, puzzle5d_editor_projection, puzzle5d_grip_full_id, puzzle5d_value_as_f64_3, puzzle5d_value_as_f64_4, quat_rotate_vector};
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dProximityConnectStage {
    Moved,
    Candidate,
    Existing,
    Compatibility,
    Emit,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dProximityConnectWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dProximityConnectWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
    moved_id: Option<String>,
    moved_kind: Option<String>,
    moved_position: Option<[f64; 3]>,
    candidate_id: Option<String>,
    candidate_kind: Option<String>,
}

pub struct Puzzle5dProximityConnectWork {
    pub stage: Puzzle5dProximityConnectStage,
    pub part_cursor: usize,
    pub grip_cursor: usize,
    pub fastener_cursor: usize,
    pub compatibility_cursor: usize,
    pub processed_units: usize,
    moved_id: Option<String>,
    moved_kind: Option<String>,
    moved_position: Option<[f64; 3]>,
    candidate_id: Option<String>,
    candidate_kind: Option<String>,
    pub mutations: Vec<Puzzle5dMutation>,
    pub operation_nonce: u64,
    pub fresh_cursor: u64,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dProximityConnectWorkOwners>,
}

impl Default for Puzzle5dProximityConnectWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dProximityConnectStage::Moved,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            compatibility_cursor: 0,
            processed_units: 0,
            moved_id: None,
            moved_kind: None,
            moved_position: None,
            candidate_id: None,
            candidate_kind: None,
            mutations: Vec::new(),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }
}

impl Puzzle5dProximityConnectWork {
    pub fn argument<'a>(command: &'a Puzzle5dCommand, key: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    pub fn grip_kind(grip: &Value) -> Option<String> {
        grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string)
    }

    pub fn world_position(part: &Value, grip: &Value) -> [f64; 3] {
        let origin = part.get("3d").and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let position = grip.get("3d").and_then(|grip| grip.get("position")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let rotated = quat_rotate_vector(orientation, position);
        [origin[0] + rotated[0], origin[1] + rotated[1], origin[2] + rotated[2]]
    }

    fn clear_candidate(&mut self) {
        self.candidate_id = None;
        self.candidate_kind = None;
        self.fastener_cursor = 0;
        self.compatibility_cursor = 0;
        self.stage = Puzzle5dProximityConnectStage::Candidate;
    }

    pub fn complete<A: Puzzle5dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle5dProximityConnectStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }
}

impl Puzzle5dProximityConnectWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dProximityConnectWork {
    fn tool_id(&self) -> &'static str {
        "proximityConnect"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_add(fasteners.checked_mul(parts)?)?.checked_add(compatibility.checked_mul(parts)?)?.checked_add(parts)?;
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
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-proximity-connect-work-capacity"));
        }
        self.processed_units += 1;
        let part_id = Self::argument(command, "partId").or_else(|| Self::argument(command, "objectId")).unwrap_or("");
        if part_id.is_empty() {
            return Ok(self.complete());
        }
        match self.stage {
            Puzzle5dProximityConnectStage::Moved => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                self.part_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.first()) else { return Ok(self.complete()) };
                    let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(self.complete()) };
                    self.moved_id = Some(puzzle5d_grip_full_id(part_id, grip_id));
                    self.moved_kind = Self::grip_kind(grip);
                    self.moved_position = Some(Self::world_position(part, grip));
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dProximityConnectStage::Candidate;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-moved", "Finding moved grip", "Verschobener Griff wird gesucht"))
            }
            Puzzle5dProximityConnectStage::Candidate => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.grip_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let Some(peer_part_id) = part.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Skipping malformed part", "Fehlerhaftes Teil wird übersprungen")) };
                let Some(peer_grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Skipping malformed grip", "Fehlerhafter Griff wird übersprungen")) };
                let peer_id = puzzle5d_grip_full_id(peer_part_id, peer_grip_id);
                if self.moved_id.as_deref() == Some(peer_id.as_str()) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let moved = self.moved_position.ok_or_else(|| Fault::from("puzzle5d-proximity-position-owner"))?;
                let peer = Self::world_position(part, grip);
                let radius = command.args().and_then(|args| args.get("radius")).and_then(Value::as_f64).unwrap_or(PUZZLE5D_PROXIMITY_RADIUS).max(0.0);
                let dx = moved[0] - peer[0];
                let dy = moved[1] - peer[1];
                let dz = moved[2] - peer[2];
                if (dx * dx + dy * dy + dz * dz).sqrt() > radius {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                self.candidate_id = Some(peer_id);
                self.candidate_kind = Self::grip_kind(grip);
                self.fastener_cursor = 0;
                self.stage = Puzzle5dProximityConnectStage::Existing;
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Existing => {
                if let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                    let peer = self.candidate_id.as_deref().unwrap_or("");
                    let moved = self.moved_id.as_deref().unwrap_or("");
                    if (source == peer && target == moved) || (source == moved && target == peer) {
                        self.clear_candidate();
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dProximityConnectStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.candidate_kind.is_none() || self.moved_kind.is_none() {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-emit", "Connecting nearby grip", "Naher Griff wird verbunden"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else {
                    self.clear_candidate();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.compatibility_cursor += 1;
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.candidate_kind.as_deref().unwrap_or("");
                let target_kind = self.moved_kind.as_deref().unwrap_or("");
                if (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind) {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Emit => {
                if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-proximity-connect-output-capacity"));
                }
                let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                let source = self.candidate_id.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-proximity-source-owner"))?;
                let target = self.moved_id.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-proximity-target-owner"))?;
                let arg = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0);
                let kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, arg("gap"), arg("shift"), arg("rise"), arg("rotation"), arg("turn"), arg("tilt"), arg("x"), arg("y"), None));
                self.clear_candidate();
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Complete => Err(Fault::from("puzzle5d-proximity-connect-complete-repolled")),
            Puzzle5dProximityConnectStage::Closing => Err(Fault::from("puzzle5d-proximity-connect-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dProximityConnectStage::Closing;
        self.close_owners.stage(Puzzle5dProximityConnectWorkOwners { mutations: std::mem::take(&mut self.mutations), moved_id: std::mem::take(&mut self.moved_id), moved_kind: std::mem::take(&mut self.moved_kind), moved_position: std::mem::take(&mut self.moved_position), candidate_id: std::mem::take(&mut self.candidate_id), candidate_kind: std::mem::take(&mut self.candidate_kind) });
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
        self.stage == Puzzle5dProximityConnectStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp

impl Puzzle5dProximityConnectWork {
    pub fn original_birth_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let capacity = (crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS as usize).checked_mul(std::mem::size_of::<Puzzle5dMutation>())
            .ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Puzzle5D Puzzle5dProximityConnectWork backing extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })
    }

    pub fn from_original_construction(scope: &mut semio_framework_action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        scope.admit_owner(Self::original_birth_demands()?)?;
        let work = Self { mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), ..Self::default() };
        let capacity = work.mutations.capacity() * std::mem::size_of::<Puzzle5dMutation>();
        Ok((work, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() }))
    }
}
