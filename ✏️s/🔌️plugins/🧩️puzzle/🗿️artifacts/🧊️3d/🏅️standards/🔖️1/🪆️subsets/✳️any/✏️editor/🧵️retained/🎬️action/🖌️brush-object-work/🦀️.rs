//! 🖌️ Brush object work: the staged placement of one brush-previewed object, its kind, representation and vortices, the attractions it joins and the published object and attraction.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAddBrushObjectStage {
    Decode,
    Kind,
    Representation,
    Vortices,
    ExistingAttractions,
    PublishObject,
    PublishAttraction,
    Complete,
    Closing,
}

#[derive(semio_framework_value::RetireOwned)]

pub(super) struct Puzzle3dBrushPayloadOwner {
    target_vortex_id: String,
    pub(super) object_kind_id: String,
    pub(super) source_vortex_index: usize,
    pub(super) origin: [f64; 3],
    pub(super) orientation: [f64; 4],
    pub(super) scale: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale>,
}

/// ♻️ The owners one `Puzzle3dAddBrushObjectWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dAddBrushObjectWorkOwners {
    pub(super) payload: Option<Puzzle3dBrushPayloadOwner>,
    pub(super) object_id: Option<String>,
    pub(super) mesh_url: Option<String>,
    pub(super) vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    pub(super) mutations: Vec<Puzzle3dMutation>,
}

pub struct Puzzle3dAddBrushObjectWork {
    pub(super) stage: Puzzle3dAddBrushObjectStage,
    kind_cursor: usize,
    representation_cursor: usize,
    pub(super) vortex_cursor: usize,
    pub(super) attraction_cursor: usize,
    pub(super) kind_index: Option<usize>,
    pub(super) payload: Option<Puzzle3dBrushPayloadOwner>,
    pub(super) object_id: Option<String>,
    pub(super) mesh_url: Option<String>,
    pub(super) vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    pub(super) mutations: Vec<Puzzle3dMutation>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dAddBrushObjectWorkOwners>,
}

impl Default for Puzzle3dAddBrushObjectWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dAddBrushObjectStage::Decode,
            kind_cursor: 0,
            representation_cursor: 0,
            vortex_cursor: 0,
            attraction_cursor: 0,
            kind_index: None,
            payload: None,
            object_id: None,
            mesh_url: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            mutations: Vec::with_capacity(2),
        }
    }
}

impl Puzzle3dAddBrushObjectWork {
    fn vector3(value: Option<&Value>) -> Option<[f64; 3]> {
        let values = value.and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])
    }

    fn quaternion(value: Option<&Value>) -> Option<[f64; 4]> {
        let values = value.and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?, values.get(3).and_then(Value::as_f64)?])
    }

    pub(super) fn scale(value: Option<&Value>) -> Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale> {
        match value {
            Some(Value::Number(value)) => Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(value.as_f64())),
            Some(Value::Array(values)) if values.len() >= 3 => Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])),
            _ => None,
        }
    }

    fn decode(command: &Puzzle3dCommand) -> Option<Puzzle3dBrushPayloadOwner> {
        let args = command.args()?;
        let target_vortex_id = args.get("targetVortexFullId").and_then(Value::as_str)?.to_string();
        let object_kind_id = args.get("objectKindId").and_then(Value::as_str)?.to_string();
        let source_vortex_index = args.get("sourceVortexIndex").and_then(Value::as_u64)? as usize;
        Some(Puzzle3dBrushPayloadOwner { target_vortex_id, object_kind_id, source_vortex_index, origin: Self::vector3(args.get("origin"))?, orientation: Self::quaternion(args.get("orientation"))?, scale: Self::scale(args.get("scale")) })
    }

    pub(super) fn object_id(snapshot: &Puzzle3dPlaySnapshot, payload: &Puzzle3dBrushPayloadOwner) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        snapshot.typed().objects.len().hash(&mut hasher);
        payload.target_vortex_id.hash(&mut hasher);
        payload.object_kind_id.hash(&mut hasher);
        payload.source_vortex_index.hash(&mut hasher);
        for value in payload.origin.into_iter().chain(payload.orientation) {
            value.to_bits().hash(&mut hasher);
        }
        format!("puzzle3d.brush.{:016x}", hasher.finish())
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dAddBrushObjectWork {
    fn tool_id(&self) -> &'static str {
        "addBrushObject"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref()?;
        let items = catalogs.objects.len().checked_add(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?)?.checked_add(snapshot.typed().attractions.len())?.checked_add(4)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        // 🧯️ Every early terminal below is a click that placed nothing. Each one now says so — an
        // absent/mismatched kind, a kind with no mesh or no source vortex, and a target vortex that is
        // already attracted are all invisible refusals otherwise.
        let Some(catalogs) = snapshot.typed().meta.kind_catalogs.as_ref() else {
            self.stage = Puzzle3dAddBrushObjectStage::Complete;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
        };
        match self.stage {
            Puzzle3dAddBrushObjectStage::Decode => {
                let Some(payload) = Self::decode(command) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_rejected.as_str())));
                };
                self.object_id = Some(Self::object_id(snapshot, &payload));
                self.payload = Some(payload);
                self.stage = Puzzle3dAddBrushObjectStage::Kind;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-kind", "Finding brush object kind", "Pinselobjektart wird gesucht"))
            }
            Puzzle3dAddBrushObjectStage::Kind => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let Some(kind) = catalogs.objects.get(self.kind_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                if kind.id == payload.object_kind_id {
                    if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                        return Err(Fault::from("puzzle3d-brush-catalog-capacity"));
                    }
                    self.kind_index = Some(self.kind_cursor);
                    self.stage = Puzzle3dAddBrushObjectStage::Representation;
                } else {
                    self.kind_cursor += 1;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-kind", "Scanning brush object kind", "Pinselobjektart wird geprüft"))
            }
            Puzzle3dAddBrushObjectStage::Representation => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-brush-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-brush-kind-cursor"))?;
                let Some(representation) = kind.representations.get(self.representation_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                self.representation_cursor += 1;
                if !representation.url.is_empty() {
                    self.mesh_url = Some(representation.url.clone());
                    self.stage = Puzzle3dAddBrushObjectStage::Vortices;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-representation", "Finding brush mesh", "Pinsel-Mesh wird gesucht"))
            }
            Puzzle3dAddBrushObjectStage::Vortices => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-brush-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-brush-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortex_cursor) else {
                    let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                    if payload.source_vortex_index >= self.vortices.len() {
                        self.stage = Puzzle3dAddBrushObjectStage::Complete;
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_rejected.as_str())));
                    }
                    self.stage = Puzzle3dAddBrushObjectStage::ExistingAttractions;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-existing-attraction", "Checking brush target", "Pinselziel wird geprüft"));
                };
                let object_id = self.object_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?;
                let index = self.vortex_cursor;
                self.vortex_cursor += 1;
                self.vortices.push(semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
                    id: format!("{object_id}:v{index}"),
                    label: None,
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-vortex", "Building brush vortex", "Pinsel-Vortex wird aufgebaut"))
            }
            Puzzle3dAddBrushObjectStage::ExistingAttractions => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let source = self.vortices.get(payload.source_vortex_index).ok_or_else(|| Fault::from("puzzle3d-brush-source-vortex-owner"))?;
                let Some(attraction) = snapshot.typed().attractions.get(self.attraction_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::PublishObject;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-publish-object", "Preparing brush object", "Pinselobjekt wird vorbereitet"));
                };
                self.attraction_cursor += 1;
                if attraction.attracting == payload.target_vortex_id || attraction.attracted == source.id {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_occupied.as_str())));
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-existing-attraction", "Scanning brush target", "Pinselziel wird geprüft"))
            }
            Puzzle3dAddBrushObjectStage::PublishObject => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let scene_snapshot = puzzle3d_scene_snapshot_from_document(snapshot.typed());
                let object = semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
                    id: self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?,
                    label: Some(puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, &payload.object_kind_id)),
                    object_kind: Some(payload.object_kind_id.clone()),
                    anchor: Default::default(),
                    origin: payload.origin,
                    orientation: Some(payload.orientation),
                    scale: payload.scale,
                    mesh_url: self.mesh_url.clone(),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None));
                self.stage = Puzzle3dAddBrushObjectStage::PublishAttraction;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-publish-object", "Publishing brush object", "Pinselobjekt wird veröffentlicht"))
            }
            Puzzle3dAddBrushObjectStage::PublishAttraction => {
                let payload = self.payload.take().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let object_id = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?;
                let attracted = format!("{object_id}:v{}", payload.source_vortex_index);
                let attraction_id = format!("attraction-{}-{attracted}", payload.target_vortex_id);
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(attraction_id, payload.target_vortex_id, attracted, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                self.stage = Puzzle3dAddBrushObjectStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addBrushObject")), ..Default::default() }))
            }
            Puzzle3dAddBrushObjectStage::Complete => Err(Fault::from("puzzle3d-brush-complete-repolled")),
            Puzzle3dAddBrushObjectStage::Closing => Err(Fault::from("puzzle3d-brush-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAddBrushObjectStage::Closing;
        self.close_owners.stage(Puzzle3dAddBrushObjectWorkOwners { payload: std::mem::take(&mut self.payload), object_id: std::mem::take(&mut self.object_id), mesh_url: std::mem::take(&mut self.mesh_url), vortices: std::mem::take(&mut self.vortices), mutations: std::mem::take(&mut self.mutations) });
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
        self.stage == Puzzle3dAddBrushObjectStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
