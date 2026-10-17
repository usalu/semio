//! 🔄️ Transform work: the staged selection transform (gumball, target volume relocation, Relocate drop, inspector nudge) that reads the gesture, scans a drop page by page and commits one transform tool transaction.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dTransformStage {
    Read,
    Scan,
    Commit,
    Complete,
    Closing,
}

/// 🎞️ What `Read` states for one transform: the finished record, or the drop whose proximity scan `Scan` pages.
enum Puzzle3dTransformGesture {
    Stated(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord),
    Scanning(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan),
}

/// 🔖️ A refusal notice selector: a plain function pointer owns nothing, so it retires as a copy leaf.
#[derive(Clone, Copy)]
struct Puzzle3dRefusal(fn(&Puzzle3dLabels) -> &'static str);

semio_framework_value::artifact_retire_leaf!(Puzzle3dRefusal);

/// 🛠️ The ONE retained work of every selection transform — a gumball translate/rotate/scale, a target-volume
/// gumball relocate, a Relocate-utility drop, an inspector origin nudge. `Read` states the gesture as ONE
/// [`utilities::transform::Puzzle3dSelectionRecord`] (the gesture's own ids, else the live selection); a drop's
/// proximity scan then runs page by page in `Scan` (one progress report per page, cancellable between pages), and
/// `Commit` runs the record through the transform tool: one `ToolTransaction` whose parametric leaves publish as ONE
/// edit stamped with the ref minted from the admission's `authoring_seed`. A gesture that moves nothing leaves
/// zero trace; nothing addressed, or everything addressed locked, is one localized refusal.
/// ♻️ The owners one `Puzzle3dTransformWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dTransformWorkOwners {
    record: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord>,
    scan: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan>,
    refusal: Option<Puzzle3dRefusal>,
}

pub struct Puzzle3dTransformWork {
    pub(super) tool_id: &'static str,
    authoring_seed: String,
    pub(super) stage: Puzzle3dTransformStage,
    record: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord>,
    scan: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan>,
    refusal: Option<fn(&Puzzle3dLabels) -> &'static str>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dTransformWorkOwners>,
}

impl Puzzle3dTransformWork {
    pub fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { close_owners: Default::default(), tool_id, authoring_seed, stage: Puzzle3dTransformStage::Read, record: None, scan: None, refusal: None }
    }

    /// 🕹️ The ids a selection-scoped verb addresses: the command's own `ids`, else the live selection at
    /// `granularity`.
    fn addressed(command: &Puzzle3dCommand, interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        let explicit: Vec<String> = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        if !explicit.is_empty() {
            return explicit;
        }
        Self::selected(interaction, granularity)
    }

    pub(super) fn selected(interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or_else(Vec::new, |selection| selection.ids.clone())
    }

    /// 🎬️ The gesture this command states on `document`, or the refusal it answers with instead.
    fn read(&self, command: &Puzzle3dCommand, document: &Puzzle3dSnapshot, interaction: &protocol::InteractionState, window_config: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Result<Option<Puzzle3dTransformGesture>, fn(&Puzzle3dLabels) -> &'static str> {
        let args = command.args();
        match self.tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" => {
                let targets = [Self::addressed(command, interaction, PUZZLE3D_GRANULARITY_OBJECT), Self::selected(interaction, PUZZLE3D_GRANULARITY_TARGET_VOLUME)].concat();
                Ok(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::from_gumball(self.tool_id, args, targets).map(Puzzle3dTransformGesture::Stated))
            }
            "relocateTargetVolume" => Ok(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::from_pose_delta(args).map(Puzzle3dTransformGesture::Stated)),
            "worldRelocate" => {
                let object_id = args.and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let Some(position) = args.and_then(|args| args.get("position")).and_then(value_as_vec3) else { return Ok(None) };
                if document.objects.iter().any(|object| object.id == object_id && (object.locked || object.hidden)) {
                    return Err(|labels| labels.selection_locked.as_str());
                }
                let radius = window_ownership::config_from_snapshot(window_config).proximity_radius;
                Ok(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan::begin(document, object_id, position, radius).map(Puzzle3dTransformGesture::Scanning))
            }
            _ => {
                let Some(offset) = puzzle3d_inspector_origin_nudge(args) else { return Ok(None) };
                let granularity = if args.and_then(|args| args.get("entity")).and_then(Value::as_str) == Some("targetVolume") { PUZZLE3D_GRANULARITY_TARGET_VOLUME } else { PUZZLE3D_GRANULARITY_OBJECT };
                Ok(Some(Puzzle3dTransformGesture::Stated(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::new(Self::addressed(command, interaction, granularity), semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionMotion::Drag { offset }))))
            }
        }
    }

    /// 🏁️ The ONE terminal emit: the committed transaction, a refusal, or nothing at all.
    fn commit(&mut self, snapshot: &Puzzle3dPlaySnapshot, view_state: Option<&semio_framework_plugin::ViewModel>) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
        self.stage = Puzzle3dTransformStage::Complete;
        if let Some(refusal) = self.refusal.take() {
            return puzzle3d_notice_emit(view_state, refusal);
        }
        let Some(record) = self.record.take() else { return Emit { ui_scope: UiDirtyScope::None, ..Default::default() } };
        let base = snapshot.typed_arc();
        let known = record.targets.iter().any(|id| base.objects.iter().any(|object| &object.id == id) || base.target_volumes.iter().any(|volume| &volume.id == id));
        if !known {
            return puzzle3d_notice_emit(view_state, |labels| labels.nothing_selected.as_str());
        }
        if record.refused_as_locked(&base) {
            return puzzle3d_notice_emit(view_state, |labels| labels.selection_locked.as_str());
        }
        let request = semio_s_artifact_puzzle_3d_editor_transform::TransformToolRequest { base, records: vec![record] };
        match semio_s_artifact_puzzle_3d_editor_transform::puzzle3d_transform_tool_commit(self.tool_id, &self.authoring_seed, request) {
            Some((transaction, mutations)) => Emit { artifact_mutations: mutations, transaction: (!self.authoring_seed.is_empty()).then_some(transaction), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class(self.tool_id)), ..Default::default() },
            None => Emit { ui_scope: UiDirtyScope::None, ..Default::default() },
        }
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🔢️ `Read` + `Commit`, plus one `Scan` step per page of objects a drop measures.
    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let addressed = Self::addressed(command, interaction, PUZZLE3D_GRANULARITY_OBJECT).len().checked_add(Self::selected(interaction, PUZZLE3D_GRANULARITY_TARGET_VOLUME).len())?;
        let pages = if self.tool_id == "worldRelocate" { snapshot.typed().objects.len().div_ceil(semio_s_artifact_puzzle_3d_editor_transform::PUZZLE3D_RELOCATE_SCAN_PAGE).max(1) } else { 0 };
        (addressed <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS).then_some(2 + pages)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        match self.stage {
            Puzzle3dTransformStage::Read => {
                match self.read(command, snapshot.typed(), interaction, context.and_then(|context| context.window_config.as_ref())) {
                    Ok(Some(Puzzle3dTransformGesture::Stated(record))) => self.record = Some(record),
                    Ok(Some(Puzzle3dTransformGesture::Scanning(scan))) => self.scan = Some(scan),
                    Ok(None) => {}
                    Err(refusal) => self.refusal = Some(refusal),
                }
                self.stage = if self.scan.is_some() { Puzzle3dTransformStage::Scan } else { Puzzle3dTransformStage::Commit };
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-transform-read", "Reading the gesture", "Geste wird gelesen"))
            }
            Puzzle3dTransformStage::Scan => {
                let scan = self.scan.as_mut().ok_or_else(|| Fault::from("puzzle3d-transform-scan-owner"))?;
                if scan.step(snapshot.typed(), semio_s_artifact_puzzle_3d_editor_transform::PUZZLE3D_RELOCATE_SCAN_PAGE) {
                    self.record = self.scan.take().map(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan::finish);
                    self.stage = Puzzle3dTransformStage::Commit;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-transform-scan", "Measuring nearby vortices", "Nahe Vortices werden gemessen"))
            }
            Puzzle3dTransformStage::Commit => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(self.commit(snapshot, context.and_then(|context| context.view_state.as_ref())))),
            Puzzle3dTransformStage::Complete => Err(Fault::from("puzzle3d-transform-complete-repolled")),
            Puzzle3dTransformStage::Closing => Err(Fault::from("puzzle3d-transform-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dTransformStage::Closing;
        self.close_owners.stage(Puzzle3dTransformWorkOwners { record: std::mem::take(&mut self.record), scan: std::mem::take(&mut self.scan), refusal: std::mem::take(&mut self.refusal).map(Puzzle3dRefusal) });
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
        self.stage == Puzzle3dTransformStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
