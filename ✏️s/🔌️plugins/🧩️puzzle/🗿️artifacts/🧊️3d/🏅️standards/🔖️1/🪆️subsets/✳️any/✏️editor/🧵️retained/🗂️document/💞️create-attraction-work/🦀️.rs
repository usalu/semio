//! 💞️ Create attraction work: the staged creation of one attraction, its existing attractions, the two endpoints and their compatibility, with the relocation page size.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

pub const PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dCreateAttractionStage {
    Existing,
    Attracting,
    Attracted,
    Compatibility,
    Publish,
    Complete,
    Closing,
}

#[derive(semio_framework_value::RetireOwned)]

pub(super) struct Puzzle3dAttractionEndpoint {
    vortex_id: String,
    pub(super) vortex_kind: Option<String>,
    local_position: [f64; 3],
    local_direction: [f64; 3],
    object_position: [f64; 3],
    object_orientation: [f64; 4],
}

/// ♻️ The owners one `Puzzle3dCreateAttractionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dCreateAttractionWorkOwners {
    pub(super) attracting: Option<Puzzle3dAttractionEndpoint>,
    pub(super) attracted: Option<Puzzle3dAttractionEndpoint>,
}

pub struct Puzzle3dCreateAttractionWork {
    pub(super) stage: Puzzle3dCreateAttractionStage,
    pub(super) item_cursor: usize,
    pub(super) child_cursor: usize,
    compatibility_cursor: usize,
    pub(super) attracting: Option<Puzzle3dAttractionEndpoint>,
    pub(super) attracted: Option<Puzzle3dAttractionEndpoint>,
    compatible: bool,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dCreateAttractionWorkOwners>,
}

impl Default for Puzzle3dCreateAttractionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle3dCreateAttractionStage::Existing, item_cursor: 0, child_cursor: 0, compatibility_cursor: 0, attracting: None, attracted: None, compatible: false }
    }
}

impl Puzzle3dCreateAttractionWork {
    pub(super) fn ids(command: &Puzzle3dCommand) -> (&str, &str) {
        let args = command.args();
        (args.and_then(|args| args.get("attracting")).and_then(Value::as_str).unwrap_or(""), args.and_then(|args| args.get("attracted")).and_then(Value::as_str).unwrap_or(""))
    }

    fn endpoint(object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject, vortex: &semio_s_artifact_puzzle_3d_model::Puzzle3dVortex) -> Puzzle3dAttractionEndpoint {
        Puzzle3dAttractionEndpoint {
            vortex_id: puzzle3d_vortex_full_id(&object.id, &vortex.id),
            vortex_kind: vortex.vortex_kind.clone(),
            local_position: vortex.position,
            local_direction: vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
            object_position: object.origin,
            object_orientation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
        }
    }

    fn scan_endpoint(&mut self, snapshot: &Puzzle3dPlaySnapshot, requested: &str) -> Result<Option<Puzzle3dAttractionEndpoint>, Fault> {
        let Some(object) = snapshot.typed().objects.get(self.item_cursor) else {
            return Ok(None);
        };
        if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
            return Err(Fault::from("puzzle3d-create-attraction-vortex-capacity"));
        }
        let Some(vortex) = object.vortices.get(self.child_cursor) else {
            self.item_cursor += 1;
            self.child_cursor = 0;
            return Ok(None);
        };
        self.child_cursor += 1;
        Ok((puzzle3d_vortex_full_id(&object.id, &vortex.id) == requested).then(|| Self::endpoint(object, vortex)))
    }

    fn begin_endpoint_scan(&mut self, stage: Puzzle3dCreateAttractionStage) {
        self.item_cursor = 0;
        self.child_cursor = 0;
        self.stage = stage;
    }

    pub(super) fn complete<A: Puzzle3dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle3dCreateAttractionStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dCreateAttractionWork {
    fn tool_id(&self) -> &'static str {
        "createAttraction"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let document = snapshot.typed();
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let endpoint_scan_stage = object_vortices.checked_add(document.objects.len())?.checked_add(1)?;
        let items = document.attractions.len().checked_add(1)?.checked_add(endpoint_scan_stage)?.checked_add(endpoint_scan_stage)?.checked_add(document.meta.kind_compatibility.len())?.checked_add(2)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let (attracting_id, attracted_id) = Self::ids(command);
        if attracting_id.is_empty() || attracted_id.is_empty() || attracting_id == attracted_id {
            return Ok(self.complete());
        }
        match self.stage {
            Puzzle3dCreateAttractionStage::Existing => {
                let Some(attraction) = snapshot.typed().attractions.get(self.item_cursor) else {
                    self.begin_endpoint_scan(Puzzle3dCreateAttractionStage::Attracting);
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-attracting", "Finding attracting vortex", "Anziehender Vortex wird gesucht"));
                };
                self.item_cursor += 1;
                if (attraction.attracting == attracting_id && attraction.attracted == attracted_id) || (attraction.attracting == attracted_id && attraction.attracted == attracting_id) {
                    return Ok(self.complete());
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-existing", "Checking existing attraction", "Bestehende Anziehung wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Attracting => {
                if self.item_cursor >= snapshot.typed().objects.len() {
                    return Ok(self.complete());
                }
                if let Some(endpoint) = self.scan_endpoint(snapshot, attracting_id)? {
                    self.attracting = Some(endpoint);
                    self.begin_endpoint_scan(Puzzle3dCreateAttractionStage::Attracted);
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-attracting", "Scanning attracting vortex", "Anziehender Vortex wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Attracted => {
                if self.item_cursor >= snapshot.typed().objects.len() {
                    return Ok(self.complete());
                }
                if let Some(endpoint) = self.scan_endpoint(snapshot, attracted_id)? {
                    self.attracted = Some(endpoint);
                    self.compatibility_cursor = 0;
                    self.compatible = snapshot.typed().meta.kind_compatibility.is_empty();
                    self.stage = Puzzle3dCreateAttractionStage::Compatibility;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-attracted", "Scanning attracted vortex", "Angezogener Vortex wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Compatibility => {
                let attracting_kind = self.attracting.as_ref().and_then(|endpoint| endpoint.vortex_kind.as_deref());
                let attracted_kind = self.attracted.as_ref().and_then(|endpoint| endpoint.vortex_kind.as_deref());
                let (Some(attracting_kind), Some(attracted_kind)) = (attracting_kind, attracted_kind) else {
                    return Ok(self.complete());
                };
                let Some(row) = snapshot.typed().meta.kind_compatibility.get(self.compatibility_cursor) else {
                    if self.compatible {
                        self.stage = Puzzle3dCreateAttractionStage::Publish;
                        return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-publish", "Preparing attraction", "Anziehung wird vorbereitet"));
                    }
                    return Ok(self.complete());
                };
                self.compatibility_cursor += 1;
                self.compatible |= (row.source == attracting_kind && row.target == attracted_kind) || (row.bidirectional && row.source == attracted_kind && row.target == attracting_kind);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-compatibility", "Checking vortex compatibility", "Vortex-Kompatibilität wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Publish => {
                let attracting = self.attracting.take().ok_or_else(|| Fault::from("puzzle3d-create-attraction-attracting-owner"))?;
                let attracted = self.attracted.take().ok_or_else(|| Fault::from("puzzle3d-create-attraction-attracted-owner"))?;
                let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(
                    attracting.object_position,
                    attracting.object_orientation,
                    attracting.local_position,
                    attracting.local_direction,
                    attracted.local_position,
                    attracted.local_direction,
                    attracted.object_position,
                    attracted.object_orientation,
                );
                let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                self.stage = Puzzle3dCreateAttractionStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: vec![semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(id, attracting.vortex_id, attracted.vortex_id, gap, shift, rise, rotation, turn, tilt, 0.0, 0.0, None)],
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("createAttraction")),
                    ..Default::default()
                }))
            }
            Puzzle3dCreateAttractionStage::Complete => Err(Fault::from("puzzle3d-create-attraction-complete-repolled")),
            Puzzle3dCreateAttractionStage::Closing => Err(Fault::from("puzzle3d-create-attraction-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dCreateAttractionStage::Closing;
        self.close_owners.stage(Puzzle3dCreateAttractionWorkOwners { attracting: std::mem::take(&mut self.attracting), attracted: std::mem::take(&mut self.attracted) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
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
        self.stage == Puzzle3dCreateAttractionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
