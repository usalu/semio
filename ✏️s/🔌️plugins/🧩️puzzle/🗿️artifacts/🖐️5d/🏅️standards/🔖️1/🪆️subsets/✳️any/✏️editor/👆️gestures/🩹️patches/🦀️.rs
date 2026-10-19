//! 🩹️ Patch gestures of the puzzle 5d editor: the retained part, fastener and grip patch work. See [`super`].

use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value::FromValue;
use semio_framework_pack_json::Value;
use std::collections::HashSet;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::entities::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, puzzle5d_axis_index, puzzle5d_editor_projection, puzzle5d_grip_full_id, puzzle5d_record_from_projection, puzzle5d_resolve_number_edit};
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand, puzzle5d_notice_emit};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPatchPartStage {
    Selection,
    Parts,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dPatchPartWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchPartWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
}

pub struct Puzzle5dPatchPartWork {
    pub stage: Puzzle5dPatchPartStage,
    pub selection_cursor: usize,
    pub part_cursor: usize,
    pub mutations: Vec<Puzzle5dMutation>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchPartWorkOwners>,
}

impl Default for Puzzle5dPatchPartWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            mutations: Vec::new(),
        }
    }
}

impl Puzzle5dPatchPartWork {
    /// 🪙️ The original construction quote owns the exact typed mutation buffer.
    pub(super) fn original_birth_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let capacity = (crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS as usize).checked_mul(size_of::<Puzzle5dMutation>())
            .ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Puzzle5D Puzzle5dPatchPartWork backing extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })
    }

    /// 🏭️ Original scope custody precedes allocation and receives the actual native capacity.
    pub(super) fn from_original_construction(scope: &mut semio_framework_action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        scope.admit_owner(Self::original_birth_demands()?)?;
        let work = Self { mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), ..Self::default() };
        let capacity = work.mutations.capacity() * size_of::<Puzzle5dMutation>();
        Ok((work, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() }))
    }

    fn targets(&self, command: &Puzzle5dCommand, id: &str) -> bool {
        (0..self.selection_cursor).any(|index| Self::source_id(command, index) == Some(id))
    }

    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("partIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("partId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("partIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("partId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    pub fn mutation(command: &Puzzle5dCommand, part: &Puzzle5dPart) -> Option<Puzzle5dMutation> {
        let args = command.args()?;
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        let text = value.and_then(Value::as_str);
        match field {
            "partKind" => text.map(|text| crate::standards::v1::subsets::any::schema::mutations::change_part_kind(part.id.clone(), Some(text.to_string()))),
            "anchor" => text.map(|text| {
                let anchor = match text.to_ascii_lowercase().as_str() {
                    "derived" | "connected" => crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPartAnchor::Derived,
                    _ => crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPartAnchor::Fixed,
                };
                crate::standards::v1::subsets::any::schema::mutations::change_part_anchor(part.id.clone(), anchor)
            }),
            "text" => text.map(|text| crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text(part.id.clone(), Some(text.to_string()))),
            "label" => Some(crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "meshUrl" => Some(crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "x" => puzzle5d_resolve_number_edit(part.part_2d.x, value, delta).map(|updated| crate::standards::v1::subsets::any::schema::mutations::move_part_2d(part.id.clone(), updated, part.part_2d.y)),
            "y" => puzzle5d_resolve_number_edit(part.part_2d.y, value, delta).map(|updated| crate::standards::v1::subsets::any::schema::mutations::move_part_2d(part.id.clone(), part.part_2d.x, updated)),
            _ => {
                let axis = puzzle5d_axis_index(field, "origin")?;
                let updated = puzzle5d_resolve_number_edit(part.part_3d.origin[axis], value, delta)?;
                let mut origin = part.part_3d.origin;
                origin[axis] = updated;
                Some(crate::standards::v1::subsets::any::schema::mutations::move_part_3d(part.id.clone(), origin))
            }
        }
    }

}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dPatchPartWork {
    fn tool_id(&self) -> &'static str {
        "patchPart"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, semio_framework_job::OriginalTurnFailure> = (|| {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if Self::source_id(command, self.selection_cursor).is_some() {
                    if self.selection_cursor >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-part-selection-capacity"));
                    }
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-part-selection", "Reading part target", "Teilziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    if self.mutations.is_empty() {
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.edit_not_applicable.as_str())));
                    }
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.part_cursor += 1;
                let part: Puzzle5dPart = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-patch-part-malformed"))?;
                if self.targets(command, &part.id) {
                    if let Some(mutation) = Self::mutation(command, &part) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-part-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-part-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchPartWorkOwners { mutations: std::mem::take(&mut self.mutations) });
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
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dPatchFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchFastenerWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
}

pub struct Puzzle5dPatchFastenerWork {
    pub stage: Puzzle5dPatchPartStage,
    pub selection_cursor: usize,
    pub fastener_cursor: usize,
    pub mutations: Vec<Puzzle5dMutation>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchFastenerWorkOwners>,
}

impl Default for Puzzle5dPatchFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            fastener_cursor: 0,
            mutations: Vec::new(),
        }
    }
}

impl Puzzle5dPatchFastenerWork {
    /// 🪙️ The original construction quote owns the exact typed mutation buffer.
    pub(super) fn original_birth_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let capacity = (crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS as usize).checked_mul(size_of::<Puzzle5dMutation>())
            .ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Puzzle5D Puzzle5dPatchFastenerWork backing extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })
    }

    /// 🏭️ Original scope custody precedes allocation and receives the actual native capacity.
    pub(super) fn from_original_construction(scope: &mut semio_framework_action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        scope.admit_owner(Self::original_birth_demands()?)?;
        let work = Self { mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), ..Self::default() };
        let capacity = work.mutations.capacity() * size_of::<Puzzle5dMutation>();
        Ok((work, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() }))
    }

    fn targets(&self, command: &Puzzle5dCommand, id: &str) -> bool {
        (0..self.selection_cursor).any(|index| Self::source_id(command, index) == Some(id))
    }

    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("fastenerIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("fastenerId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("fastenerIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("fastenerId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    pub fn mutation(command: &Puzzle5dCommand, fastener: &Puzzle5dFastener) -> Option<Puzzle5dMutation> {
        let args = command.args()?;
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        if field == "fastenerKind" {
            return Some(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), value.and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string)));
        }
        let mut geometry = [fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y];
        let index = match field {
            "gap" => 0,
            "shift" => 1,
            "rise" => 2,
            "rotation" => 3,
            "turn" => 4,
            "tilt" => 5,
            "x" => 6,
            "y" => 7,
            _ => return None,
        };
        geometry[index] = puzzle5d_resolve_number_edit(geometry[index], value, delta)?;
        Some(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }))
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dPatchFastenerWork {
    fn tool_id(&self) -> &'static str {
        "patchFastener"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, semio_framework_job::OriginalTurnFailure> = (|| {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if Self::source_id(command, self.selection_cursor).is_some() {
                    if self.selection_cursor >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-fastener-selection-capacity"));
                    }
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-fastener-selection", "Reading fastener target", "Verbindungsziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.fastener_cursor += 1;
                let fastener: Puzzle5dFastener = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-patch-fastener-malformed"))?;
                if self.targets(command, &fastener.id) {
                    if let Some(mutation) = Self::mutation(command, &fastener) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-fastener-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-fastener-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations) });
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
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dPatchGripWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchGripWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
}

pub struct Puzzle5dPatchGripWork {
    pub stage: Puzzle5dPatchPartStage,
    pub selection_cursor: usize,
    pub part_cursor: usize,
    pub grip_cursor: usize,
    processed_grips: usize,
    pub mutations: Vec<Puzzle5dMutation>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchGripWorkOwners>,
}

impl Default for Puzzle5dPatchGripWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            grip_cursor: 0,
            processed_grips: 0,
            mutations: Vec::new(),
        }
    }
}

impl Puzzle5dPatchGripWork {
    /// 🪙️ The original construction quote owns the exact typed mutation buffer.
    pub(super) fn original_birth_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let capacity = (crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS as usize).checked_mul(size_of::<Puzzle5dMutation>())
            .ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Puzzle5D Puzzle5dPatchGripWork backing extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })
    }

    /// 🏭️ Original scope custody precedes allocation and receives the actual native capacity.
    pub(super) fn from_original_construction(scope: &mut semio_framework_action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        scope.admit_owner(Self::original_birth_demands()?)?;
        let work = Self { mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), ..Self::default() };
        let capacity = work.mutations.capacity() * size_of::<Puzzle5dMutation>();
        Ok((work, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() }))
    }

    fn targets(&self, command: &Puzzle5dCommand, part: &str, grip: &str) -> bool {
        if grip.contains(':') { return (0..self.selection_cursor).any(|index| Self::source_id(command, index) == Some(grip)); }
        (0..self.selection_cursor).any(|index| Self::source_id(command, index)
            .and_then(|id| id.strip_prefix(part)).and_then(|id| id.strip_prefix(':')) == Some(grip))
    }

    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("gripFullIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("gripFullId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("gripFullIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("gripFullId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    pub fn patch(command: &Puzzle5dCommand, grip: &mut crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip) -> bool {
        let Some(args) = command.args() else { return false };
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        let text = value.and_then(Value::as_str);
        match field {
            "gripKind" => {
                let Some(text) = text else { return false };
                grip.grip_kind = Some(text.to_string());
                grip.grip_2d.grip_kind = Some(text.to_string());
            }
            "angle" => {
                let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_2d.angle, value, delta) else { return false };
                grip.grip_2d.angle = updated;
            }
            "radius" => {
                let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.radius.unwrap_or(0.0), value, delta) else { return false };
                grip.grip_2d.radius = Some(updated);
                grip.grip_3d.radius = Some(updated);
            }
            "label" => grip.grip_3d.label = text.filter(|text| !text.is_empty()).map(str::to_string),
            _ => {
                if let Some(axis) = puzzle5d_axis_index(field, "position") {
                    let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.position[axis], value, delta) else { return false };
                    grip.grip_3d.position[axis] = updated;
                } else if let Some(axis) = puzzle5d_axis_index(field, "direction") {
                    let mut direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
                    let Some(updated) = puzzle5d_resolve_number_edit(direction[axis], value, delta) else { return false };
                    direction[axis] = updated;
                    grip.grip_3d.direction = Some(direction);
                } else {
                    return false;
                }
            }
        }
        true
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dPatchGripWork {
    fn tool_id(&self) -> &'static str {
        "patchGrip"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, semio_framework_job::OriginalTurnFailure> = (|| {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if Self::source_id(command, self.selection_cursor).is_some() {
                    if self.selection_cursor >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-grip-selection-capacity"));
                    }
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-grip-selection", "Reading grip target", "Griffziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                let Some(grip_value) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)).cloned() else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-grip-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                if self.processed_grips >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-patch-grip-work-capacity"));
                }
                self.processed_grips += 1;
                self.grip_cursor += 1;
                let part_id = part.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-patch-grip-part-id-malformed"))?;
                let mut grip: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip = <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&grip_value)).map_err(|_| Fault::from("puzzle5d-patch-grip-malformed"))?;
                if self.targets(command, part_id, &grip.id) && Self::patch(command, &mut grip) {
                    let grip_id = grip.id.clone();
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_part_grip(part_id.to_string(), grip_id, grip));
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-grip-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-grip-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchGripWorkOwners { mutations: std::mem::take(&mut self.mutations) });
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
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
