//! ⚖️ Kind weight work: the staged edit of the brush kind weights, the catalog, its validation, the sum of the other weights and the rebuilt publication.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dKindWeightStage {
    Catalog,
    Validate,
    SumOthers,
    Changed,
    Build,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dKindWeightWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dKindWeightWorkOwners {
    pub(super) ids: Vec<String>,
    changed_id: Option<String>,
}

pub struct Puzzle3dKindWeightWork {
    pub(super) tool_id: &'static str,
    pub(super) stage: Puzzle3dKindWeightStage,
    pub(super) cursor: usize,
    pub(super) ids: Vec<String>,
    missing: bool,
    base_sum: f64,
    other_sum: f64,
    other_count: usize,
    changed_id: Option<String>,
    pub(super) requested: f64,
    ignored: bool,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dKindWeightWorkOwners>,
}

impl Puzzle3dKindWeightWork {
    /// 🪙️ Exact typed backing storage belongs to the initiating original construction scope.
    pub fn original_birth_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let mut capacity = 0_usize;
        capacity = (semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS as usize).checked_mul(size_of::<String>()).and_then(|bytes| capacity.checked_add(bytes))
            .ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Puzzle3D Puzzle3dKindWeightWork backing extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })
    }

    /// 🏭️ The scope authorizes every typed buffer before its owner is born and receives actual capacities.
    pub fn from_original_construction(tool_id: &'static str, scope: &mut semio_framework::action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        scope.admit_owner(Self::original_birth_demands()?)?;
        let work = Self {
            ids: Vec::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS),
            ..Self::new(tool_id)
        };
        let capacity = work.ids.capacity() * size_of::<String>();
        Ok((work, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() }))
    }

    pub fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            stage: Puzzle3dKindWeightStage::Catalog,
            cursor: 0,
            ids: Vec::new(),
            missing: false,
            base_sum: 0.0,
            other_sum: 0.0,
            other_count: 0,
            changed_id: None,
            requested: 1.0,
            ignored: false,
        }
    }

    fn weights<'a>(&self, config: &'a Puzzle3dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setObjectKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }



}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref()?;
        let count = if self.tool_id == "setObjectKindWeight" { catalogs.objects.len() } else { catalogs.vortices.len() };
        let items = count.checked_mul(4)?.checked_add(3)?;
        (count <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        match self.stage {
            Puzzle3dKindWeightStage::Catalog => {
                let catalogs = snapshot.typed().meta.kind_catalogs.as_ref().ok_or_else(|| Fault::from("puzzle3d-kind-weight-catalog-owner"))?;
                let id = if self.tool_id == "setObjectKindWeight" { catalogs.objects.get(self.cursor).map(|entry| entry.id.as_str()) } else { catalogs.vortices.get(self.cursor).map(|entry| entry.id.as_str()) };
                if let Some(id) = id {
                    if self.ids.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-kind-weight-catalog-capacity"));
                    }
                    self.ids.push(id.to_string());
                    self.cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
                }
                self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                let requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                self.requested = if self.tool_id == "setVortexKindWeight" {
                    if let Some(object_kind_id) = command.args().and_then(|args| args.get("objectKindId")).and_then(Value::as_str) {
                        let vortex_ids = self.ids.clone();
                        let object_weight = puzzle3d_object_joint_total(&config.vortex_kind_weights, &config.object_kind_weights, object_kind_id, &vortex_ids);
                        if object_weight <= f64::EPSILON {
                            self.ignored = true;
                            self.stage = Puzzle3dKindWeightStage::Publish;
                            return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-publish", "Ignoring zero-weight child", "Kind mit Nullgewicht wird ignoriert"));
                        }
                        requested
                    } else {
                        requested
                    }
                } else {
                    requested
                };
                self.cursor = 0;
                self.stage = Puzzle3dKindWeightStage::Validate;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"))
            }
            Puzzle3dKindWeightStage::Validate => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle3dKindWeightStage::SumOthers;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle3dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle3dKindWeightStage::Changed;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle3dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    self.changed_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-kind-weight-changed-owner"))?;
                }
                self.stage = Puzzle3dKindWeightStage::Build;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle3dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.stage = Puzzle3dKindWeightStage::Publish;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                if self.ids.len() >= 2 && self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle3dKindWeightStage::Publish => {
                self.stage = Puzzle3dKindWeightStage::Complete;
                if self.ignored {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let catalogs = snapshot.typed().meta.kind_catalogs.as_ref().ok_or_else(|| Fault::from("puzzle3d-kind-weight-catalog-owner"))?;
                let object_ids: Vec<String> = catalogs.objects.iter().map(|entry| entry.id.clone()).collect();
                let vortex_ids: Vec<String> = catalogs.vortices.iter().map(|entry| entry.id.clone()).collect();
                let kind_id = self.changed_id.as_deref().unwrap_or("");
                let object_kind_id = command.args().and_then(|args| args.get("objectKindId")).and_then(Value::as_str);
                let mut object_weights = config.object_kind_weights.clone();
                let mut joint_weights = config.vortex_kind_weights.clone();
                puzzle3d_apply_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, self.tool_id, kind_id, object_kind_id, self.requested);
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    config_mutations: vec![
                        Puzzle3dConfigMutation::SetObjectKindWeights(Puzzle3dConfigSetObjectKindWeights{ value: object_weights }),
                        Puzzle3dConfigMutation::SetVortexKindWeights(Puzzle3dConfigSetVortexKindWeights{ value: joint_weights }),
                    ],
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class(self.tool_id)),
                    ..Default::default()
                }))
            }
            Puzzle3dKindWeightStage::Complete => Err(Fault::from("puzzle3d-kind-weight-complete-repolled")),
            Puzzle3dKindWeightStage::Closing => Err(Fault::from("puzzle3d-kind-weight-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dKindWeightStage::Closing;
        self.close_owners.stage(Puzzle3dKindWeightWorkOwners { ids: std::mem::take(&mut self.ids), changed_id: std::mem::take(&mut self.changed_id) });
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
        self.stage == Puzzle3dKindWeightStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
