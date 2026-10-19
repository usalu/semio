//! 🔍️ Patch inspector work: the staged inspector patch that pages through the selected objects, vortices, attractions, references and target volumes.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;
use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dEditor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dPatchInspectorStage {
    Selection,
    Objects,
    Vortices,
    Attractions,
    AttractionReconnect,
    References,
    Volumes,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dPatchInspectorWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dPatchInspectorWorkOwners {
    pub(super) mutations: Vec<Puzzle3dMutation>,
    pending_attraction: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction>,
}

pub struct Puzzle3dPatchInspectorWork {
    pub(super) stage: Puzzle3dPatchInspectorStage,
    selection_cursor: usize,
    pub(super) item_cursor: usize,
    pub(super) child_cursor: usize,
    pending_attraction: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction>,
    pub(super) mutations: Vec<Puzzle3dMutation>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dPatchInspectorWorkOwners>,
}

impl Default for Puzzle3dPatchInspectorWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dPatchInspectorStage::Selection,
            selection_cursor: 0,
            item_cursor: 0,
            child_cursor: 0,
            pending_attraction: None,
            mutations: Vec::new(),
        }
    }
}

impl Puzzle3dPatchInspectorWork {
    /// 🪙️ Exact typed backing storage belongs to the initiating original construction scope.
    pub fn original_birth_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let mut capacity = 0_usize;
        capacity = (semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS as usize).checked_mul(size_of::<Puzzle3dMutation>()).and_then(|bytes| capacity.checked_add(bytes))
            .ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Puzzle3D Puzzle3dPatchInspectorWork backing extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })
    }

    /// 🏭️ The scope authorizes every typed buffer before its owner is born and receives actual capacities.
    pub fn from_original_construction(scope: &mut semio_framework_action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        scope.admit_owner(Self::original_birth_demands()?)?;
        let work = Self {
            mutations: Vec::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS),
            ..Self::default()
        };
        let capacity = work.mutations.capacity() * size_of::<Puzzle3dMutation>();
        Ok((work, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() }))
    }

    fn entity(command: &Puzzle3dCommand) -> &str {
        command.args().and_then(|args| args.get("entity")).and_then(Value::as_str).unwrap_or("")
    }

    fn field(command: &Puzzle3dCommand) -> &str {
        command.args().and_then(|args| args.get("field")).and_then(Value::as_str).unwrap_or("")
    }

    fn granularity(entity: &str) -> &str {
        match entity {
            "object" => PUZZLE3D_GRANULARITY_OBJECT,
            "vortex" => PUZZLE3D_GRANULARITY_VORTEX,
            "attraction" => PUZZLE3D_GRANULARITY_ATTRACTION,
            "reference" => PUZZLE3D_GRANULARITY_REFERENCE,
            "targetVolume" => PUZZLE3D_GRANULARITY_TARGET_VOLUME,
            _ => "",
        }
    }

    fn source_id<'a>(command: &'a Puzzle3dCommand, interaction: &'a protocol::InteractionState, index: usize) -> Option<&'a str> {
        if let Some(ids) = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()) {
            return ids.get(index).and_then(Value::as_str);
        }
        let granularity = Self::granularity(Self::entity(command));
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).and_then(|selection| selection.ids.get(index)).map(String::as_str)
    }

    fn source_len(command: &Puzzle3dCommand, interaction: &protocol::InteractionState) -> usize {
        command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()).map_or_else(
            || {
                let granularity = Self::granularity(Self::entity(command));
                interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or(0, |selection| selection.ids.len())
            },
            Vec::len,
        )
    }

    fn targets(&self, command: &Puzzle3dCommand, interaction: &protocol::InteractionState, id: &str) -> bool {
        (0..self.selection_cursor).any(|index| Self::source_id(command, interaction, index) == Some(id))
    }

    fn targets_vortex(&self, command: &Puzzle3dCommand, interaction: &protocol::InteractionState, object: &str, vortex: &str) -> bool {
        if vortex.contains(':') { return self.targets(command, interaction, vortex); }
        (0..self.selection_cursor).any(|index| Self::source_id(command, interaction, index)
            .and_then(|id| id.strip_prefix(object)).and_then(|id| id.strip_prefix(':')) == Some(vortex))
    }

    fn first_stage(entity: &str) -> Puzzle3dPatchInspectorStage {
        match entity {
            "object" => Puzzle3dPatchInspectorStage::Objects,
            "vortex" => Puzzle3dPatchInspectorStage::Vortices,
            "attraction" => Puzzle3dPatchInspectorStage::Attractions,
            "reference" => Puzzle3dPatchInspectorStage::References,
            "targetVolume" => Puzzle3dPatchInspectorStage::Volumes,
            _ => Puzzle3dPatchInspectorStage::Complete,
        }
    }

    pub(super) fn push(&mut self, mutation: Puzzle3dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-patch-inspector-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    pub(super) fn complete<A: Puzzle3dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle3dPatchInspectorStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("patchInspector")), ..Default::default() })
    }

    pub(super) fn scale(value: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale>) -> [f64; 3] {
        match value {
            Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(value)) => [value; 3],
            Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3(value)) => value,
            None => [1.0; 3],
        }
    }

    fn object_mutation(command: &Puzzle3dCommand, object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::change_object_hidden(object.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::change_object_locked(object.id.clone(), value)),
            "label" => Some(semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::edit_object_label(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "objectKind" => Some(semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::change_object_kind(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "meshUrl" => Some(semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::change_object_mesh(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "origin" => value.and_then(value_as_vec3).map(|origin| semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object(object.id.clone(), origin)),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                    let mut origin = object.origin;
                    origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object(object.id.clone(), origin));
                }
                if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                    let mut scale = Self::scale(object.scale);
                    scale[axis] = puzzle3d_resolve_number_edit(scale[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::scale_object(object.id.clone(), Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3(scale))));
                }
                let axis = puzzle3d_axis_index(field, "orientation")?;
                let mut orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                orientation[axis] = puzzle3d_resolve_number_edit(orientation[axis], value, delta)?;
                Some(semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::rotate_object(object.id.clone(), Some(quat_normalize(orientation))))
            }
        }
    }

    fn vortex_mutation(command: &Puzzle3dCommand, object_id: &str, vortex: &semio_s_artifact_puzzle_3d_model::Puzzle3dVortex) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        let mut next = vortex.clone();
        match field {
            "hidden" => next.hidden = value.and_then(Value::as_bool)?,
            "locked" => next.locked = value.and_then(Value::as_bool)?,
            "vortexKind" => next.vortex_kind = value.and_then(Value::as_str).map(str::to_string),
            "position" => next.position = value.and_then(value_as_vec3)?,
            "direction" => next.direction = Some(value.and_then(value_as_vec3)?),
            "radius" => next.radius = Some(puzzle3d_resolve_number_edit(next.radius.unwrap_or(0.35), value, delta)?),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "position") {
                    next.position[axis] = puzzle3d_resolve_number_edit(next.position[axis], value, delta)?;
                } else {
                    let axis = puzzle3d_axis_index(field, "direction")?;
                    let mut direction = next.direction.unwrap_or([0.0, 0.0, 1.0]);
                    direction[axis] = puzzle3d_resolve_number_edit(direction[axis], value, delta)?;
                    next.direction = Some(direction);
                }
            }
        }
        Some(semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::replace_object_vortex(object_id.to_string(), vortex.id.clone(), next))
    }

    fn attraction_geometry(command: &Puzzle3dCommand, attraction: &semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        let mut geometry = [attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y];
        let index = match field {
            "gap" => 0,
            "shift" => 1,
            "rise" => 2,
            "rotation" => 3,
            "turn" => 4,
            "tilt" => 5,
            _ => return None,
        };
        geometry[index] = puzzle3d_resolve_number_edit(geometry[index], value, delta)?;
        Some(semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::replace_attraction_geometry(semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry {
            id: attraction.id.clone(),
            new_gap: geometry[0],
            new_shift: geometry[1],
            new_rise: geometry[2],
            new_rotation: geometry[3],
            new_turn: geometry[4],
            new_tilt: geometry[5],
            new_x: geometry[6],
            new_y: geometry[7],
        }))
    }

    fn reference_mutation(command: &Puzzle3dCommand, reference: &semio_s_artifact_puzzle_3d_model::Puzzle3dReference) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::change_reference_hidden(reference.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::change_reference_locked(reference.id.clone(), value)),
            "sourceUrl" | "mediaKind" => {
                let mut source = reference.source.clone();
                if field == "sourceUrl" {
                    source.url = value.and_then(Value::as_str)?.to_string();
                } else {
                    source.media_kind = value.and_then(Value::as_str).map(str::to_string);
                }
                Some(semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::replace_reference_source(reference.id.clone(), source))
            }
            "origin" => value.and_then(value_as_vec3).map(|origin| semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::move_reference(reference.id.clone(), origin)),
            "widthWorld" => puzzle3d_resolve_number_edit(reference.width_world, value, delta).map(|width| semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::resize_reference(reference.id.clone(), width)),
            _ => {
                let axis = puzzle3d_axis_index(field, "origin")?;
                let mut origin = reference.origin;
                origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                Some(semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::move_reference(reference.id.clone(), origin))
            }
        }
    }

    fn volume_mutation(command: &Puzzle3dCommand, volume: &semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::change_target_volume_hidden(volume.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::change_target_volume_locked(volume.id.clone(), value)),
            "origin" => value.and_then(value_as_vec3).map(|origin| semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume(volume.id.clone(), origin)),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                    let mut origin = volume.origin;
                    origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume(volume.id.clone(), origin));
                }
                if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                    let mut scale = Self::scale(volume.scale);
                    scale[axis] = puzzle3d_resolve_number_edit(scale[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::scale_target_volume(volume.id.clone(), Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3(scale))));
                }
                let axis = puzzle3d_axis_index(field, "orientation")?;
                let mut orientation = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                orientation[axis] = puzzle3d_resolve_number_edit(orientation[axis], value, delta)?;
                Some(semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::rotate_target_volume(volume.id.clone(), Some(quat_normalize(orientation))))
            }
        }
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dPatchInspectorWork {
    fn tool_id(&self) -> &'static str {
        "patchInspector"
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let source = Self::source_len(command, interaction);
        let document = snapshot.typed();
        let scan = match Self::entity(command) {
            "object" => document.objects.len().checked_add(1)?,
            "vortex" => {
                let mut vortices = 0usize;
                for object in &document.objects {
                    vortices = vortices.checked_add(object.vortices.len())?;
                }
                document.objects.len().checked_add(1)?.checked_add(vortices)?
            }
            "attraction" => document.attractions.len().checked_mul(2)?.checked_add(1)?,
            "reference" => document.references.len().checked_add(1)?,
            "targetVolume" => document.target_volumes.len().checked_add(1)?,
            _ => 0,
        };
        let items = source.checked_add(scan)?.checked_add(1)?;
        (source <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context: _context, .. } = *input;
        match self.stage {
            Puzzle3dPatchInspectorStage::Selection => {
                if Self::source_id(command, interaction, self.selection_cursor).is_some() {
                    if self.selection_cursor >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-patch-inspector-selection-capacity"));
                    }
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-selection", "Reading inspector target", "Inspektionsziel wird gelesen"));
                }
                self.stage = Self::first_stage(Self::entity(command));
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-scan", "Finding inspector target", "Inspektionsziel wird gesucht"))
            }
            Puzzle3dPatchInspectorStage::Objects => {
                let Some(object) = snapshot.typed().objects.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.targets(command, interaction, &object.id) {
                    if let Some(mutation) = Self::object_mutation(command, object) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-object", "Patching object", "Objekt wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Vortices => {
                let Some(object) = snapshot.typed().objects.get(self.item_cursor) else { return Ok(self.complete()) };
                let Some(vortex) = object.vortices.get(self.child_cursor) else {
                    self.item_cursor += 1;
                    self.child_cursor = 0;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-vortex-owner", "Advancing vortex owner", "Vortex-Eigentümer wird gewechselt"));
                };
                self.child_cursor += 1;
                if self.targets_vortex(command, interaction, &object.id, &vortex.id) {
                    if let Some(mutation) = Self::vortex_mutation(command, &object.id, vortex) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-vortex", "Patching vortex", "Vortex wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Attractions => {
                let Some(attraction) = snapshot.typed().attractions.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if !self.targets(command, interaction, &attraction.id) {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction", "Scanning attraction", "Anziehung wird geprüft"));
                }
                let field = Self::field(command);
                if matches!(field, "attracting" | "attracted") {
                    let Some(value) = command.args().and_then(|args| args.get("value")).and_then(Value::as_str) else {
                        return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction", "Skipping malformed attraction", "Fehlerhafte Anziehung wird übersprungen"));
                    };
                    let mut next = attraction.clone();
                    if field == "attracting" {
                        next.attracting = value.to_string();
                    } else {
                        next.attracted = value.to_string();
                    }
                    self.push(semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices(attraction.id.clone()))?;
                    self.pending_attraction = Some(next);
                    self.stage = Puzzle3dPatchInspectorStage::AttractionReconnect;
                } else if let Some(mutation) = Self::attraction_geometry(command, attraction) {
                    self.push(mutation)?;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction", "Patching attraction", "Anziehung wird geändert"))
            }
            Puzzle3dPatchInspectorStage::AttractionReconnect => {
                let attraction = self.pending_attraction.take().ok_or_else(|| Fault::from("puzzle3d-patch-inspector-attraction-owner"))?;
                self.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(
                    attraction.id,
                    attraction.attracting,
                    attraction.attracted,
                    attraction.gap,
                    attraction.shift,
                    attraction.rise,
                    attraction.rotation,
                    attraction.turn,
                    attraction.tilt,
                    attraction.x,
                    attraction.y, None,
                ))?;
                self.stage = Puzzle3dPatchInspectorStage::Attractions;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction-reconnect", "Reconnecting attraction", "Anziehung wird neu verbunden"))
            }
            Puzzle3dPatchInspectorStage::References => {
                let Some(reference) = snapshot.typed().references.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.targets(command, interaction, &reference.id) {
                    if let Some(mutation) = Self::reference_mutation(command, reference) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-reference", "Patching reference", "Referenz wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Volumes => {
                let Some(volume) = snapshot.typed().target_volumes.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.targets(command, interaction, &volume.id) {
                    if let Some(mutation) = Self::volume_mutation(command, volume) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-volume", "Patching target volume", "Zielvolumen wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Complete => Err(Fault::from("puzzle3d-patch-inspector-complete-repolled")),
            Puzzle3dPatchInspectorStage::Closing => Err(Fault::from("puzzle3d-patch-inspector-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dPatchInspectorStage::Closing;
        self.close_owners.stage(Puzzle3dPatchInspectorWorkOwners { mutations: std::mem::take(&mut self.mutations), pending_attraction: std::mem::take(&mut self.pending_attraction) });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.close_owners.close_step(cx)
    }) }

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
        self.stage == Puzzle3dPatchInspectorStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
