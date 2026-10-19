//! 💡️ Accept suggestion work: the staged acceptance of one suggestion, its target, candidate, vortices and existing attractions, then the published object, attraction and result.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAcceptSuggestionStage {
    Target,
    Candidate,
    Vortices,
    ExistingAttractions,
    PublishObject,
    PublishAttraction,
    PublishResult,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dAcceptSuggestionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dAcceptSuggestionWorkOwners {
    pub(super) vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    pub(super) mutations: Vec<Puzzle3dMutation>,
    target_id: Option<String>,
    pub(super) object_id: Option<String>,
    candidate: Option<semio_s_artifact_puzzle_3d_model::BrushPreviewState>,
    pub(super) instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
}

pub struct Puzzle3dAcceptSuggestionWork {
    pub(super) stage: Puzzle3dAcceptSuggestionStage,
    pub(super) object_cursor: usize,
    pub(super) vortex_cursor: usize,
    pub(super) attraction_cursor: usize,
    target_id: Option<String>,
    pub(super) kind_index: Option<usize>,
    pub(super) object_id: Option<String>,
    candidate: Option<semio_s_artifact_puzzle_3d_model::BrushPreviewState>,
    pub(super) vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    pub(super) mutations: Vec<Puzzle3dMutation>,
    pub(super) instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dAcceptSuggestionWorkOwners>,
}

impl Default for Puzzle3dAcceptSuggestionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dAcceptSuggestionStage::Target,
            object_cursor: 0,
            vortex_cursor: 0,
            attraction_cursor: 0,
            target_id: None,
            kind_index: None,
            object_id: None,
            candidate: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            mutations: Vec::with_capacity(2),
            instance_owner: None,
        }
    }
}

impl Puzzle3dAcceptSuggestionWork {
    fn requested_target(command: &Puzzle3dCommand, transient: &window_ownership::Puzzle3dWindowTransient, interaction: &protocol::InteractionState) -> Option<String> {
        command
            .args()
            .and_then(|args| args.get("fullId"))
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| transient.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
            .or_else(|| interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE3D_GRANULARITY_VORTEX).and_then(|selection| selection.ids.first().cloned()))
    }

    /// 🪪️ Binds the admission's retained operation owner.
    pub fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {
        self.instance_owner = Some(instance.owner.clone());
        self
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dAcceptSuggestionWork {
    fn tool_id(&self) -> &'static str {
        "acceptSuggestion"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let document = snapshot.typed();
        document.meta.kind_catalogs.as_ref()?;
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let target_scan_stage = object_vortices.checked_add(document.objects.len())?.checked_add(1)?;
        let kind_scan_stage = PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?.checked_add(2)?;
        let items = target_scan_stage.checked_add(1)?.checked_add(kind_scan_stage)?.checked_add(document.attractions.len())?.checked_add(1)?.checked_add(3)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        let document = snapshot.typed();
        let transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
        // 🧯️ The popup always retires, but a dismissed popup that placed nothing is
        // indistinguishable from a successful one unless the refusal itself speaks — `accept_suggestion_
        // closes_menu_even_when_placement_fails` is literally named after that silence.
        let Some(catalogs) = document.meta.kind_catalogs.as_ref() else {
            self.stage = Puzzle3dAcceptSuggestionStage::Complete;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
        };
        match self.stage {
            Puzzle3dAcceptSuggestionStage::Target => {
                let Some(requested) = Self::requested_target(command, &transient, interaction) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                let Some(object) = document.objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-accept-target-vortex-capacity"));
                }
                let Some(vortex) = object.vortices.get(self.vortex_cursor) else {
                    self.object_cursor += 1;
                    self.vortex_cursor = 0;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-target-object", "Scanning target object", "Zielobjekt wird geprüft"));
                };
                self.vortex_cursor += 1;
                if puzzle3d_vortex_full_id(&object.id, &vortex.id) == requested {
                    self.target_id = Some(requested);
                    self.stage = Puzzle3dAcceptSuggestionStage::Candidate;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-target-vortex", "Scanning target vortex", "Ziel-Vortex wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::Candidate => {
                let target = self.target_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let requested = command.args().and_then(|args| args.get("index")).and_then(Value::as_u64).unwrap_or(transient.brush_candidate_index as u64) as usize;
                let candidate = self.instance_owner.as_ref().and_then(|owner| {
                    owner
                        .with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| {
                            let link = &mut owner.brush_suggestions;
                            let free: Vec<_> = link.found(&target).into_iter().flat_map(|found| found.free()).cloned().collect();
                            link.close_menu();
                            Ok((!free.is_empty()).then(|| free[requested % free.len()].clone()))
                        })
                        .ok()
                        .flatten()
                });
                let Some((index, candidate)) = candidate.and_then(|candidate| catalogs.objects.iter().position(|kind| kind.id == candidate.object_kind_id).map(|index| (index, candidate))) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                let kind = &catalogs.objects[index];
                if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-accept-candidate-capacity"));
                }
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                target.hash(&mut hasher);
                kind.id.hash(&mut hasher);
                candidate.source_vortex_index.hash(&mut hasher);
                document.objects.len().hash(&mut hasher);
                self.object_id = Some(format!("puzzle3d.suggestion.{:016x}", hasher.finish()));
                self.kind_index = Some(index);
                self.candidate = Some(candidate);
                self.stage = Puzzle3dAcceptSuggestionStage::Vortices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-candidate", "Selecting suggestion candidate", "Vorschlagskandidat wird gewählt"))
            }
            Puzzle3dAcceptSuggestionStage::Vortices => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortices.len()) else {
                    if self.vortices.is_empty() {
                        self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                    }
                    self.stage = Puzzle3dAcceptSuggestionStage::ExistingAttractions;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-existing", "Checking target ownership", "Zielinhaberschaft wird geprüft"));
                };
                let object_id = self.object_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                let index = self.vortices.len();
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
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-vortex", "Building one suggested vortex", "Ein Vorschlags-Vortex wird aufgebaut"))
            }
            Puzzle3dAcceptSuggestionStage::ExistingAttractions => {
                let target = self.target_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let Some(attraction) = document.attractions.get(self.attraction_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::PublishObject;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-publish-object", "Preparing suggested object", "Vorschlagsobjekt wird vorbereitet"));
                };
                self.attraction_cursor += 1;
                if attraction.attracting == *target || attraction.attracted == *target {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_occupied.as_str())));
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-existing", "Scanning existing attraction", "Bestehende Anziehung wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::PublishObject => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let candidate = self.candidate.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-candidate-owner"))?;
                let scene_snapshot = puzzle3d_scene_snapshot_from_document(document);
                let object = semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
                    id: self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?,
                    label: Some(puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, &kind.id)),
                    object_kind: Some(kind.id.clone()),
                    anchor: Default::default(),
                    origin: candidate.origin,
                    orientation: Some(candidate.orientation),
                    scale: candidate.scale.clone().and_then(|value| FromValue::from_value(value).ok()),
                    mesh_url: Some(candidate.mesh_url.clone()),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None));
                self.stage = Puzzle3dAcceptSuggestionStage::PublishAttraction;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-publish-object", "Transferring suggested object", "Vorschlagsobjekt wird übertragen"))
            }
            Puzzle3dAcceptSuggestionStage::PublishAttraction => {
                let target = self.target_id.take().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let object_id = self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                let source = format!("{object_id}:v{}", self.candidate.take().ok_or_else(|| Fault::from("puzzle3d-accept-candidate-owner"))?.source_vortex_index);
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(format!("attraction-{target}-{source}"), target, source, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                self.stage = Puzzle3dAcceptSuggestionStage::PublishResult;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-publish-attraction", "Transferring suggested attraction", "Vorschlagsanziehung wird übertragen"))
            }
            Puzzle3dAcceptSuggestionStage::PublishResult => {
                self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                let placed = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations: Vec::new(),
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("acceptSuggestion")),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_GRANULARITY_OBJECT, [placed])],
                    ..Default::default()
                }))
            }
            Puzzle3dAcceptSuggestionStage::Complete => Err(Fault::from("puzzle3d-accept-complete-repolled")),
            Puzzle3dAcceptSuggestionStage::Closing => Err(Fault::from("puzzle3d-accept-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAcceptSuggestionStage::Closing;
        self.close_owners.stage(Puzzle3dAcceptSuggestionWorkOwners { vortices: std::mem::take(&mut self.vortices), mutations: std::mem::take(&mut self.mutations), target_id: std::mem::take(&mut self.target_id), object_id: std::mem::take(&mut self.object_id), candidate: std::mem::take(&mut self.candidate), instance_owner: std::mem::take(&mut self.instance_owner) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.close_owners.close_step(grant)
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
        self.stage == Puzzle3dAcceptSuggestionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
