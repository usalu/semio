//! 🎛️ Selection gestures of the puzzle 5d editor: the retained selection-transform, kind-weight and focus-selection work. See [`super`].

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use crate::{editor::puzzle5d::config::Puzzle5dConfig, editor::puzzle5d::config::Puzzle5dConfigMutation, editor::puzzle5d::config::Puzzle5dConfigSetObjectKindWeights, editor::puzzle5d::config::Puzzle5dConfigSetVortexKindWeights};
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_pack_json::Value;
use std::{collections::HashMap, collections::HashSet};
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_inspector_nudge;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME, PUZZLE5D_INTERACTION_DOMAIN, puzzle5d_editor_projection, puzzle5d_value_as_f64_3};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand, puzzle5d_notice_emit};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dTransformStage {
    Read,
    Scan,
    Commit,
    Complete,
    Closing,
}

/// 🎞️ What `Read` states for one transform: the finished record, or the world drop whose proximity scan `Scan` pages.
enum Puzzle5dTransformGesture {
    Stated(crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord),
    Scanning(crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dRelocateScan),
}

/// 🛠️ The ONE retained work of every selection transform — a gumball translate/rotate/scale, a target-volume
/// gumball relocate, a world drop, an inspector `x`/`y`/origin nudge. `Read` states the gesture as ONE
/// [`crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord`] (the gesture's own ids, else the live selection);
/// a world drop's proximity scan then runs page by page in `Scan` (one progress report per page, cancellable between
/// pages), and `Commit` runs the record through the transform tool: one `ToolTransaction` whose parametric leaves
/// publish as ONE edit stamped with the ref minted from the admission's `authoring_seed`. A gesture that moves nothing leaves
/// zero trace; nothing addressed, or everything addressed locked, is one localized refusal.
/// ♻️ The owners one `Puzzle5dTransformWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dTransformWorkOwners {
    pub record: Option<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord>,
    pub scan: Option<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dRelocateScan>,
}

pub struct Puzzle5dTransformWork {
    pub tool_id: &'static str,
    pub authoring_seed: String,
    pub stage: Puzzle5dTransformStage,
    pub record: Option<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord>,
    pub scan: Option<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dRelocateScan>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dTransformWorkOwners>,
}

impl Puzzle5dTransformWork {
    pub fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { close_owners: Default::default(), tool_id, authoring_seed, stage: Puzzle5dTransformStage::Read, record: None, scan: None }
    }

    /// 🕹️ The parts a selection-scoped verb addresses: the command's own `ids`, else the live part selection.
    fn addressed(command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Vec<String> {
        let explicit: Vec<String> = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        if !explicit.is_empty() {
            return explicit;
        }
        Self::selected(interaction, PUZZLE5D_GRANULARITY_PART)
    }

    pub fn selected(interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or_else(Vec::new, |selection| selection.ids.clone())
    }

    /// 🩹️ The parts an inspector edit names: `partIds` then `partId`, each once, empty ids dropped.
    fn patched(command: &Puzzle5dCommand) -> Vec<String> {
        let args = command.args();
        let listed = args.and_then(|args| args.get("partIds")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str);
        let single = args.and_then(|args| args.get("partId")).and_then(Value::as_str);
        listed.chain(single).filter(|id| !id.is_empty()).map(str::to_string).collect()
    }

    /// 🎬️ The gesture this command states on `document`, `None` when it states none.
    pub fn read(&self, command: &Puzzle5dCommand, document: &Puzzle5dSnapshot, config: &Puzzle5dConfig, interaction: &protocol::InteractionState) -> Option<Puzzle5dTransformGesture> {
        use semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord;
        use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::{Puzzle5dRelocateScan, Puzzle5dSelectionRecord};
        let args = command.args();
        match self.tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" => {
                let targets = [Self::addressed(command, interaction), Self::selected(interaction, PUZZLE5D_GRANULARITY_TARGET_VOLUME)].concat();
                Puzzle3dSelectionRecord::from_gumball(self.tool_id, args, targets).map(|record| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::world(record)))
            }
            "relocateTargetVolume" => Puzzle3dSelectionRecord::from_pose_delta(args).map(|record| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::world(record))),
            "worldRelocate" => {
                let part_id = args.and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let position = args.and_then(|args| args.get("position")).and_then(puzzle5d_value_as_f64_3)?;
                Puzzle5dRelocateScan::begin(document, part_id, position, config.proximity_radius).map(Puzzle5dTransformGesture::Scanning)
            }
            _ => puzzle5d_inspector_nudge(args).map(|motion| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::new(Self::patched(command), motion))),
        }
    }

    /// 🏁️ The ONE terminal emit: the committed transaction, a refusal, or nothing at all.
    fn commit(&mut self, snapshot: &Puzzle5dPlaySnapshot, view_state: Option<&semio_framework_plugin::ViewModel>) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        self.stage = Puzzle5dTransformStage::Complete;
        let Some(record) = self.record.take() else { return Emit { ui_scope: UiDirtyScope::None, ..Default::default() } };
        let base = snapshot.typed_arc();
        if !record.names_any(&base) {
            return puzzle5d_notice_emit(view_state, |labels| labels.nothing_selected.as_str());
        }
        if record.refused_as_locked(&base) {
            return puzzle5d_notice_emit(view_state, |labels| labels.selection_locked.as_str());
        }
        let request = crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::TransformToolRequest { base, records: vec![record] };
        match crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_transform_tool_commit(self.tool_id, &self.authoring_seed, request) {
            Some((transaction, mutations)) => Emit { artifact_mutations: mutations, transaction: (!self.authoring_seed.is_empty()).then_some(transaction), ui_scope: UiDirtyScope::Full, ..Default::default() },
            None => Emit { ui_scope: UiDirtyScope::None, ..Default::default() },
        }
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🔢️ `Read` + `Commit`, plus one `Scan` step per page of parts a world drop measures.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let addressed = Self::addressed(command, interaction).len().checked_add(Self::selected(interaction, PUZZLE5D_GRANULARITY_TARGET_VOLUME).len())?.checked_add(Self::patched(command).len())?;
        let pages = if self.tool_id == "worldRelocate" { snapshot.typed().parts.len().div_ceil(crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::PUZZLE5D_RELOCATE_SCAN_PAGE).max(1) } else { 0 };
        (addressed <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS).then_some(2 + pages)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        match self.stage {
            Puzzle5dTransformStage::Read => {
                match self.read(command, snapshot.typed(), config, interaction) {
                    Some(Puzzle5dTransformGesture::Stated(record)) => self.record = Some(record),
                    Some(Puzzle5dTransformGesture::Scanning(scan)) => self.scan = Some(scan),
                    None => {}
                }
                self.stage = if self.scan.is_some() { Puzzle5dTransformStage::Scan } else { Puzzle5dTransformStage::Commit };
                Ok(crate::puzzle_progress_step!("puzzle5d-transform-read", "Reading the gesture", "Geste wird gelesen"))
            }
            Puzzle5dTransformStage::Scan => {
                let scan = self.scan.as_mut().ok_or_else(|| Fault::from("puzzle5d-transform-scan-owner"))?;
                if scan.step(snapshot.typed(), crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::PUZZLE5D_RELOCATE_SCAN_PAGE) {
                    self.record = self.scan.take().map(crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dRelocateScan::finish);
                    self.stage = Puzzle5dTransformStage::Commit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-transform-scan", "Measuring nearby grips", "Nahe Griffe werden gemessen"))
            }
            Puzzle5dTransformStage::Commit => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(self.commit(snapshot, context.and_then(|context| context.view_state.as_ref())))),
            Puzzle5dTransformStage::Complete => Err(Fault::from("puzzle5d-transform-complete-repolled")),
            Puzzle5dTransformStage::Closing => Err(Fault::from("puzzle5d-transform-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dTransformStage::Closing;
        self.close_owners.stage(Puzzle5dTransformWorkOwners { record: std::mem::take(&mut self.record), scan: std::mem::take(&mut self.scan) });
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
        self.stage == Puzzle5dTransformStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dKindWeightStage {
    Catalog,
    InferParts,
    InferGrips,
    Validate,
    SumOthers,
    Changed,
    Build,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dKindWeightWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dKindWeightWorkOwners {
    pub ids: Vec<String>,
    pub seen: HashSet<String>,
    pub result: HashMap<String, f64>,
    pub changed_id: Option<String>,
}

pub struct Puzzle5dKindWeightWork {
    pub tool_id: &'static str,
    pub stage: Puzzle5dKindWeightStage,
    pub cursor: usize,
    pub part_cursor: usize,
    pub grip_cursor: usize,
    pub ids: Vec<String>,
    pub seen: HashSet<String>,
    pub result: HashMap<String, f64>,
    missing: bool,
    base_sum: f64,
    pub other_sum: f64,
    other_count: usize,
    pub changed_id: Option<String>,
    pub requested: f64,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dKindWeightWorkOwners>,
}

impl Puzzle5dKindWeightWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            stage: Puzzle5dKindWeightStage::Catalog,
            cursor: 0,
            part_cursor: 0,
            grip_cursor: 0,
            ids: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            seen: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            result: HashMap::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            missing: false,
            base_sum: 0.0,
            other_sum: 0.0,
            other_count: 0,
            changed_id: None,
            requested: 1.0,
        }
    }

    fn section(&self) -> &'static str {
        if self.tool_id == "setPartKindWeight" {
            "parts"
        } else {
            "grips"
        }
    }

    pub fn weights<'a>(&self, config: &'a Puzzle5dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setPartKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }

    fn catalog(&self, snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("kindCatalogs").and_then(|value| value.get(self.section())).and_then(Value::as_array).cloned().unwrap_or_default()
    }

    fn push_id(&mut self, id: &str, inferred: bool) -> Result<(), Fault> {
        if self.ids.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
            return Err(Fault::from("puzzle5d-kind-weight-catalog-capacity"));
        }
        if !inferred || self.seen.insert(id.to_string()) {
            self.ids.push(id.to_string());
        }
        Ok(())
    }

    fn base_weight(&self, config: &Puzzle5dConfig, id: &str) -> f64 {
        if self.ids.is_empty() {
            return 0.0;
        }
        if self.missing || self.weights(config).is_empty() {
            return 1.0 / self.ids.len() as f64;
        }
        let value = self.weights(config).get(id).copied().unwrap_or(0.0);
        if (self.base_sum - 1.0).abs() > 0.001 && self.base_sum.abs() > f64::EPSILON {
            value / self.base_sum
        } else {
            value
        }
    }

}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        Some(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dKindWeightStage::Catalog => {
                let entries = self.catalog(snapshot);
                if let Some(entry) = entries.get(self.cursor) {
                    if let Some(id) = entry.get("id").and_then(Value::as_str) {
                        self.push_id(id, false)?;
                    }
                    self.cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
                }
                self.stage = if entries.is_empty() {
                    if self.tool_id == "setPartKindWeight" {
                        Puzzle5dKindWeightStage::InferParts
                    } else {
                        Puzzle5dKindWeightStage::InferGrips
                    }
                } else {
                    Puzzle5dKindWeightStage::Validate
                };
                self.cursor = 0;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-infer", "Preparing kind validation", "Artprüfung wird vorbereitet"))
            }
            Puzzle5dKindWeightStage::InferParts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                self.part_cursor += 1;
                if let Some(id) = part.get("partKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-part", "Reading inferred part kind", "Abgeleitete Teileart wird gelesen"))
            }
            Puzzle5dKindWeightStage::InferGrips => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                self.grip_cursor += 1;
                if let Some(id) = grip.get("gripKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-grip", "Reading inferred grip kind", "Abgeleitete Griffart wird gelesen"))
            }
            Puzzle5dKindWeightStage::Validate => {
                if self.changed_id.is_none() {
                    self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                    self.requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                }
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::SumOthers;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle5dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::Changed;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle5dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    self.result.insert(self.changed_id.clone().ok_or_else(|| Fault::from("puzzle5d-kind-weight-changed-owner"))?, self.requested);
                }
                self.stage = Puzzle5dKindWeightStage::Build;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle5dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor).cloned() else {
                    self.stage = Puzzle5dKindWeightStage::Publish;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                let value = if self.ids.len() == 1 {
                    1.0
                } else if self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                } else {
                    let remainder = (1.0 - self.requested).max(0.0);
                    if self.other_sum <= f64::EPSILON {
                        remainder / self.other_count.max(1) as f64
                    } else {
                        self.base_weight(config, &id) / self.other_sum * remainder
                    }
                };
                self.result.insert(id, value);
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle5dKindWeightStage::Publish => {
                self.stage = Puzzle5dKindWeightStage::Complete;
                let mutation = if self.tool_id == "setPartKindWeight" {
                    Puzzle5dConfigMutation::SetObjectKindWeights(Puzzle5dConfigSetObjectKindWeights{ value: std::mem::take(&mut self.result) })
                } else {
                    Puzzle5dConfigMutation::SetVortexKindWeights(Puzzle5dConfigSetVortexKindWeights{ value: std::mem::take(&mut self.result) })
                };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { config_mutations: vec![mutation], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dKindWeightStage::Complete => Err(Fault::from("puzzle5d-kind-weight-complete-repolled")),
            Puzzle5dKindWeightStage::Closing => Err(Fault::from("puzzle5d-kind-weight-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dKindWeightStage::Closing;
        self.close_owners.stage(Puzzle5dKindWeightWorkOwners { ids: std::mem::take(&mut self.ids), seen: std::mem::take(&mut self.seen), result: std::mem::take(&mut self.result), changed_id: std::mem::take(&mut self.changed_id) });
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
        self.stage == Puzzle5dKindWeightStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dFocusSelectionStage {
    Selection,
    Parts,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dFocusSelectionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dFocusSelectionWorkOwners {
    pub selected: HashSet<String>,
}

pub struct Puzzle5dFocusSelectionWork {
    pub stage: Puzzle5dFocusSelectionStage,
    pub selection_cursor: usize,
    pub part_cursor: usize,
    pub selected: HashSet<String>,
    sum_2d: [f64; 2],
    sum_3d: [f64; 3],
    matched: usize,
    minimum_3d: [f64; 3],
    maximum_3d: [f64; 3],
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dFocusSelectionWorkOwners>,
}

impl Default for Puzzle5dFocusSelectionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dFocusSelectionStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            sum_2d: [0.0; 2],
            sum_3d: [0.0; 3],
            matched: 0,
            minimum_3d: [f64::MAX; 3],
            maximum_3d: [f64::MIN; 3],
        }
    }
}

impl Puzzle5dFocusSelectionWork {
    pub fn source(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_PART).map(|selection| selection.ids.as_slice()).unwrap_or_default()
    }

    /// 🎥️ Which parts this focus frames. An EMPTY selection frames the WHOLE document rather than
    /// completing with `Emit::default()` — a camera verb with nothing selected has an obvious subject
    /// (everything), and the silent no-op made the focus keybinding dead on every boot before the user
    /// had picked anything (puzzle 3d's `Puzzle3dFocusSelectionWork::frames`). Costs no extra capacity:
    /// the part scan this work already declares in `extent` runs either way.
    fn frames(&self, part_id: &str) -> bool {
        self.selected.is_empty() || self.selected.contains(part_id)
    }

    pub fn axis(row: &Value, section: &str, field: &str, index: usize) -> f64 {
        row.get(section).and_then(|section| section.get(field)).and_then(Value::as_array).and_then(|values| values.get(index)).and_then(Value::as_f64).unwrap_or(0.0)
    }

    fn scalar(row: &Value, section: &str, field: &str) -> f64 {
        row.get(section).and_then(|section| section.get(field)).and_then(Value::as_f64).unwrap_or(0.0)
    }

}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let selected = Self::source(interaction).len();
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let items = selected.checked_add(parts)?.checked_add(1)?;
        (selected <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dFocusSelectionStage::Selection => {
                if let Some(id) = Self::source(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-owner", "Reading selected part", "Ausgewähltes Teil wird gelesen"));
                }
                self.stage = Puzzle5dFocusSelectionStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-part", "Finding selected part", "Ausgewähltes Teil wird gesucht"))
            }
            Puzzle5dFocusSelectionStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dFocusSelectionStage::Publish;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str).is_some_and(|id| self.frames(id)) {
                    self.sum_2d[0] += Self::scalar(row, "2d", "x");
                    self.sum_2d[1] += Self::scalar(row, "2d", "y");
                    self.sum_3d[0] += Self::axis(row, "3d", "origin", 0);
                    self.sum_3d[1] += Self::axis(row, "3d", "origin", 1);
                    self.sum_3d[2] += Self::axis(row, "3d", "origin", 2);
                    for axis in 0..3 {
                        let value = Self::axis(row, "3d", "origin", axis);
                        self.minimum_3d[axis] = self.minimum_3d[axis].min(value);
                        self.maximum_3d[axis] = self.maximum_3d[axis].max(value);
                    }
                    self.matched += 1;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-part", "Scanning selected part", "Ausgewähltes Teil wird geprüft"))
            }
            Puzzle5dFocusSelectionStage::Publish => {
                self.stage = Puzzle5dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.nothing_selected.as_str())));
                }
                let divisor = self.matched as f64;
                let centre = [self.sum_3d[0] / divisor, self.sum_3d[1] / divisor, self.sum_3d[2] / divisor];
                // 📏️ Half the diagonal of the framed parts' own axis-aligned bound, accumulated in the ONE
                // part scan `extent` already declares — no second pass, no unbounded growth.
                let span = [self.maximum_3d[0] - self.minimum_3d[0], self.maximum_3d[1] - self.minimum_3d[1], self.maximum_3d[2] - self.minimum_3d[2]];
                let radius = (span[0] * span[0] + span[1] * span[1] + span[2] * span[2]).sqrt() / 2.0;
                let distance = radius * 3.0 + 2.0;
                // 🪟️ BOTH poses are written into the one window-config value; `addressed_config` projects
                // exactly the half the addressed pane owns (board → `camera2d`, world → `camera3d`), so a
                // focus in either pane keeps that pane's own camera coherent with the 5d dual pose.
                let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                let offset = [next.camera3d.position[0] - next.camera3d.target[0], next.camera3d.position[1] - next.camera3d.target[1], next.camera3d.position[2] - next.camera3d.target[2]];
                let orbit = (offset[0] * offset[0] + offset[1] * offset[1] + offset[2] * offset[2]).sqrt();
                let scale = if orbit > f64::EPSILON { distance / orbit } else { 1.0 };
                next.camera3d.target = centre;
                next.camera3d.position = [centre[0] + offset[0] * scale, centre[1] + offset[1] * scale, centre[2] + offset[2] * scale];
                next.camera2d.x = self.sum_2d[0] / divisor;
                next.camera2d.y = self.sum_2d[1] / divisor;
                let view = view_state.ok_or_else(|| Fault::from("puzzle5d-focus-window-context-required"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dFocusSelectionStage::Complete => Err(Fault::from("puzzle5d-focus-selection-complete-repolled")),
            Puzzle5dFocusSelectionStage::Closing => Err(Fault::from("puzzle5d-focus-selection-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dFocusSelectionStage::Closing;
        self.close_owners.stage(Puzzle5dFocusSelectionWorkOwners { selected: std::mem::take(&mut self.selected) });
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
        self.stage == Puzzle5dFocusSelectionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
