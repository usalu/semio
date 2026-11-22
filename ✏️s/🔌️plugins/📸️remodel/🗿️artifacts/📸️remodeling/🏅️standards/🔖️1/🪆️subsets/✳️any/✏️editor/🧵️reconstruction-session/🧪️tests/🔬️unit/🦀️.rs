use super::*;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::create_gcp::create_gcp;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::add_stream_frame::add_stream_frame;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_asset::create_asset;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::checker_data_url;
use semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::frames;
use semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::FRAME_MIME;
use semio_framework_job::{StepBudget, OriginalJobFixtureCaller, OriginalJobReturnOrigin, OriginalJobReturn, InteractiveStage};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_tool_run::{ToolRunId, ToolRunTick, ToolRunTraceOp};
use std::time::{Duration, Instant};

const RUN_SCHEMA: &str = include_str!("../../🔣️.json");
const RUN_FIXTURE: &str = include_str!("../🧫️fixtures/🔣️.json");

fn schema_table() -> serde_json::Value {
    serde_json::from_str::<serde_json::Value>(RUN_SCHEMA).expect("run schema parses")["x-semio-toolRun"].clone()
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(RUN_FIXTURE).expect("run fixture parses")
}

fn label_of(entry: &serde_json::Value) -> LocalizedLabel {
    LocalizedLabel::native(entry["en"].as_str().expect("en label"), entry["de"].as_str().expect("de label"))
}

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [7; 32])
}

fn never() -> Option<u64> {
    Some(0)
}

/// 🎞️ Retained views in the short orbit document.
const ORBIT_SHORT_FRAMES: usize = 8;

/// 📥️ The committed `synthetic-orbit` document (calibrated camera, parameters, frame table) with its
/// thirty-six frames bound through `create-asset`; `orbit-short`, the same document cut to its first
/// [`ORBIT_SHORT_FRAMES`] views — a capture that
/// reconstructs (cameras, points, a mesh) in a fraction of the full orbit's time, for the laws that
/// need a published result; or four identical 24×24 checker frames on one stream sampled at stride 1
/// — a capture no two-view geometry can be solved from (zero baseline), for the laws about a run
/// that completes without cameras.
async fn imported_document(example: &str) -> Arc<RemodelingSnapshot> {
    if example == "synthetic-orbit" || example == "orbit-short" {
        let mut scene = <RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_remodel_remodeling_examples::examples::synthetic_orbit::PRIMARY_TEXT).expect("synthetic-orbit document parses");
        let kept = if example == "orbit-short" { ORBIT_SHORT_FRAMES } else { frames().count() };
        for (asset_id, bytes) in frames().take(kept) {
            let image = semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::decode_still_image(FRAME_MIME, bytes).expect("frame decodes");
            let asset = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ImageAsset { mime: FRAME_MIME.into(), data: base64_codec::base64_standard_encode(bytes), width: image.width, height: image.height };
            scene = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&scene, &create_asset((*asset_id).into(), asset)).expect("frame asset applies");
        }
        if example == "orbit-short" {
            for stream in &mut scene.streams {
                stream.frames.truncate(kept);
            }
        }
        return Arc::new(scene);
    }
    let payload = checker_data_url(24, 24, 3).await;
    let bytes = base64_codec::base64_standard_decode(payload.split_once(',').expect("data url").1).expect("checker bytes");
    let mut scene = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene();
    scene = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&scene, &semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_ingest_params::update_ingest_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams { frame_sample_stride: 1, ..scene.params.ingest.clone() })).expect("every frame is sampled");
    for index in 0..4u32 {
        let asset_id = format!("checker-frame-{index}");
        let frame = FrameRef { index, timestamp_ms: f64::from(index) * 500.0, asset_id: asset_id.clone() };
        let stream = if index == 0 {
            semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_stream::create_stream(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaStream { id: "checker".into(), name: "checker".into(), kind: semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind::ImageSequence, camera_id: None, sync_offset_ms: 0.0, fps_hint: 2.0, frames: vec![frame], source: None })
        } else {
            add_stream_frame("checker".into(), frame, semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind::ImageSequence)
        };
        for mutation in [create_asset(asset_id, semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ImageAsset { mime: "image/png".into(), data: base64_codec::base64_standard_encode(&bytes), width: 24, height: 24 }), stream] {
            scene = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&scene, &mutation).expect("frame import applies");
        }
    }
    Arc::new(scene)
}

/// 🚦️ What one job turn handed its driver, with every retained page closed.
enum Turn {
    Tick(ToolRunTick),
    Checkpoint(Vec<u8>),
    Complete,
    Fault,
    Yield,
}

const FUNDED_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 16 };

fn budget(fuel: u64, deadline_us: u64) -> StepBudget {
    StepBudget::new(fuel, deadline_us, FUNDED_GRANT)
}

fn whole_turns() -> StepBudget {
    budget(u64::MAX, u64::MAX)
}

/// 🧮️ Uses regular trace batching for the full orbit's numerical outcome.
fn fixture_budget(document: &str) -> StepBudget {
    let fuel = if document == "synthetic-orbit" { semio_framework_job::INTERACTIVE_LANE_FUEL } else { 1 };
    budget(fuel, u64::MAX)
}

fn view_to_turn(view: JobOutcomeView<'_>) -> Turn {
    match view {
        JobOutcomeView::PreviewReady { payload, .. } => Turn::Tick(ToolRunTick::decode(payload.single_page().expect("a tick is one payload page")).expect("tick decodes")),
        JobOutcomeView::CheckpointReady { state, .. } => Turn::Checkpoint(state.single_page().expect("checkpoint page").to_vec()),
        JobOutcomeView::Complete { .. } => Turn::Complete,
        JobOutcomeView::Fault { .. } => Turn::Fault,
        JobOutcomeView::Cancelled { .. } => panic!("the run was never cancelled"),
        JobOutcomeView::Yield { .. } => Turn::Yield,
    }
}


const SOURCE_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1_000_000_000, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: 1_000_000_000, maximum_release_bytes: 1_000_000_000, maximum_depth: 128 };

struct RunDriver {
    caller: Option<OriginalJobFixtureCaller>,
    origin: OriginalJobReturnOrigin,
}

impl RunDriver {
    fn new(clock: fn() -> Option<u64>) -> Self {
        let mut caller = OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(91), semio_framework_job::Generation(1), SOURCE_STOCK, clock, u64::MAX, 1_000_000_000);
        caller.admit_return_table();
        Self { caller: Some(caller), origin: OriginalJobReturnOrigin::empty() }
    }

    fn close_job(&mut self, job: &mut impl InteractiveJob) {
        for _ in 0..4096 {
            if self.origin.outcome().is_none() { break; }
            self.caller.as_mut().expect("original caller").refreshed_turn(FUNDED_GRANT, |cx| self.origin.acknowledge_outcome(cx)).expect("funded result acknowledgement");
        }
        assert!(self.origin.outcome().is_none(), "original outcome acknowledged");
        let mut began = false;
        self.caller.as_mut().expect("original caller").refreshed_turn(FUNDED_GRANT, |cx| semio_framework_job::close_original_granted_step(cx, |_| {
            job.begin_close();
            began = true;
            semio_framework_value::RetainedWorkOutcome::<_, ValueError> { result: Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }), work: 1 }
        })).expect("funded job close admission");
        assert!(began, "original caller funded close admission");
        let mut stalled = 0;
        for _ in 0..10_000_000 {
            let step = self.caller.as_mut().expect("original caller").refreshed_turn(FUNDED_GRANT, |cx| job.close_step(cx)).expect("funded job close");
            if matches!(step, InteractiveJobCloseStep::Complete { .. }) {
                assert!(job.terminal_is_empty(), "closed job has no retained owner");
                return;
            }
            stalled = if step.progress() == RetainedCloneProgress::default() { stalled + 1 } else { 0 };
            assert!(stalled < 4_096, "original job retirement stalled before reporting Complete");
        }
        panic!("job did not close");
    }
}

impl Drop for RunDriver {
    fn drop(&mut self) {
        if std::thread::panicking() { return; }
        let caller = self.caller.as_mut().expect("original caller");
        for _ in 0..4096 {
            if self.origin.terminal_is_empty() { break; }
            caller.refreshed_turn(FUNDED_GRANT, |cx| self.origin.close_step(cx)).expect("funded return close");
        }
        assert!(self.origin.terminal_is_empty(), "original return closed");
        self.caller.take().expect("original caller").retire(SOURCE_STOCK);
    }
}

thread_local! { static UNIT_CLOCK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }

fn unit_clock() -> Option<u64> { Some(UNIT_CLOCK.with(std::cell::Cell::get)) }

struct DeadlineUnitJob<'a, J>(&'a mut J, Option<u64>);

impl<J: InteractiveJob> InteractiveJob for DeadlineUnitJob<'_, J> {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>, claim: &mut semio_framework_job::OriginalJobReturnClaim<'_>) -> semio_framework_job::InteractiveJobStep<'a> {
        if let Some(deadline) = self.1 {
            UNIT_CLOCK.with(|clock| clock.set(deadline));
            cx.observe_step_clock(Some(deadline));
        }
        self.0.step(cx, claim)
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> { self.0.borrow_outcome(descriptor) }
    fn begin_close(&mut self) { self.0.begin_close(); }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { self.0.close_step(cx) }
    fn terminal_is_empty(&self) -> bool { self.0.terminal_is_empty() }
}

/// 🚦️ One funded caller turn receives and acknowledges its actual returned ownership.
fn step_once(job: &mut impl InteractiveJob, step_budget: StepBudget, driver: &mut RunDriver) -> Option<Turn> {
    let origin = &mut driver.origin;
    driver.caller.as_mut().expect("original caller").refreshed_turn(step_budget.retained, |cx| {
        if origin.outcome().is_some() {
            origin.acknowledge_outcome(cx)?;
            return Ok(None);
        }
        cx.acknowledge_return_rows_locally();
        if !origin.admit(cx)? {
            return Ok(None);
        }
        let deadline = if step_budget.deadline_us == 0 { unit_clock().expect("unit clock") + 1 } else { step_budget.deadline_us };
        let mut outcome = None;
        cx.with_original_work_loan(step_budget.fuel, deadline, |cx| {
            let mut verdict = None;
            let mut unit = DeadlineUnitJob(job, (step_budget.deadline_us == 0).then_some(deadline));
            let returned = semio_framework_job::drive_step(&mut unit, cx, origin.receiver()?, "remodeling.run", InteractiveStage::InteractiveStep, &mut verdict)?;
            match returned {
                OriginalJobReturn::Preparing | OriginalJobReturn::Pending => Ok(()),
                OriginalJobReturn::Outcome(reply) => {
                    let mut returned = Some(reply.into_descriptor());
                    origin.receive_outcome(cx, &mut returned)?;
                    outcome = origin.outcome().map(|descriptor| view_to_turn(unit.borrow_outcome(descriptor).expect("remodeling job outcome")));
                    Ok(())
                }
                OriginalJobReturn::Continuation(reply) => {
                    let failure = format!("{:?} {:?} {:?}", reply.view().callback, reply.view().qualification, reply.view().receipt_qualification);
                    let mut returned = Some(reply);
                    origin.receive_continuation(cx, &mut returned)?;
                    panic!("remodeling job continuation: {failure}");
                }
            }
        })?;
        Ok(outcome)
    }).expect("original remodeling caller turn")
}

fn try_turn(job: &mut impl InteractiveJob, step_budget: StepBudget, driver: &mut RunDriver) -> Option<Turn> {
    for _ in 0..1_000_000 {
        if let Some(turn) = step_once(job, StepBudget::new(step_budget.fuel, step_budget.deadline_us, step_budget.retained), driver) { return Some(turn); }
    }
    None
}

fn turn(job: &mut impl InteractiveJob, step_budget: StepBudget, driver: &mut RunDriver) -> Turn {
    try_turn(job, step_budget, driver).expect("the job never delivered an outcome")
}

/// 🪞️ Everything a run handed the ledger, folded the way the ledger folds it.
#[derive(Default)]
struct Mirror {
    verdicts: Vec<(u64, ToolRunVerdict, u16)>,
    resident: std::collections::BTreeMap<u64, (ToolRunVerdict, u16, ToolRunTraceSubject)>,
    steps: Vec<(ToolRunStepKind, u16, Vec<u64>)>,
    ops: Vec<RemodelingMutation>,
    stages: Vec<u16>,
    ticks: usize,
    evidence_ticks: usize,
    checkpoints: Vec<Vec<u8>>,
    last: Option<ToolRunProgress>,
    completed: bool,
    faulted: bool,
}

impl Mirror {
    fn fold(&mut self, tick: ToolRunTick) {
        self.ticks += 1;
        if let Some(retract) = tick.retract_to {
            self.ops.truncate(retract as usize);
        }
        let mut evidence = !tick.steps.is_empty() || !tick.append_ops.is_empty();
        for page in &tick.trace {
            for op in &page.ops {
                match op {
                    ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                        evidence = true;
                        self.verdicts.push((*key, *verdict, *reason));
                        self.resident.insert(*key, (*verdict, *reason, *subject));
                    }
                    ToolRunTraceOp::Retire { key } => {
                        self.resident.remove(key);
                    }
                    ToolRunTraceOp::Clear => self.resident.clear(),
                }
            }
        }
        self.evidence_ticks += usize::from(evidence);
        for step in tick.steps {
            self.steps.push((step.kind, step.reason, step.args.iter().map(|arg| if let ToolRunStepArg::Unsigned(value) = arg { *value } else { 0 }).collect()));
        }
        for bytes in tick.append_ops {
            self.ops.push(<RemodelingMutation as protocol::OpBinary>::decode_op(&bytes).expect("provisional op decodes"));
        }
        if let Some(progress) = tick.progress {
            if self.stages.last() != Some(&progress.stage) {
                self.stages.push(progress.stage);
            }
            self.last = Some(progress);
        }
    }

    fn counter(&self, counter: ReconstructionRunCounter) -> u64 {
        self.last.as_ref().and_then(|progress| progress.counters.iter().find(|entry| entry.counter == counter.index())).map_or(0, |entry| entry.value)
    }
}

/// ▶️ Drives `job` to its terminal outcome under `budget`, folding every tick.
fn run_to_end(job: &mut ReconstructionRunJob, budget: StepBudget) -> Mirror {
    let mut mirror = Mirror::default();
    let mut sequence = RunDriver::new(never);
    for _ in 0..10_000_000 {
        let turn = try_turn(job, budget, &mut sequence).unwrap_or_else(|| {
            let caller = sequence.caller.as_ref().expect("original caller");
            panic!(
                "no outcome at {} after {} decisions: engine unit {:?}, publication {:?}, retained grant {:?}, source fuel {}",
                job.stage.id(), job.decisions, job.engine.as_ref().map(|engine| engine.unit_label()), job.outbox.kind, caller.remaining_grant(), caller.fuel_remaining()
            );
        });
        match turn {
            Turn::Tick(tick) => mirror.fold(tick),
            Turn::Checkpoint(bytes) => mirror.checkpoints.push(bytes),
            terminal @ (Turn::Complete | Turn::Fault) => {
                mirror.completed = matches!(terminal, Turn::Complete);
                mirror.faulted = !mirror.completed;
                sequence.close_job(job);
                return mirror;
            }
            Turn::Yield => {}
        }
    }
    panic!("the reconstruction run did not settle")
}

fn reason_id(code: u16) -> &'static str {
    ReconstructionRunReason::from_code(code).map_or("framework", ReconstructionRunReason::id)
}

fn verdict_id(verdict: ToolRunVerdict) -> &'static str {
    match verdict {
        ToolRunVerdict::Testing => "testing",
        ToolRunVerdict::Success => "success",
        ToolRunVerdict::Warning => "warning",
        ToolRunVerdict::Danger => "danger",
    }
}

#[test]
fn the_run_definition_matches_its_source_of_record_table() {
    let table = schema_table();
    let definition = reconstruction_run_definition();
    definition.validate().expect("the run definition is valid");
    assert_eq!(table["mutating"], definition.mutating);
    assert_eq!(table["rebase"], "revalidate");
    assert_eq!(definition.rebase, ToolRunRebasePolicy::Revalidate);
    assert_eq!(table["reconfigure"], "resume");
    assert_eq!(definition.reconfigure, ToolRunReconfigurePolicy::Resume);
    assert_eq!(table["trace"], "instance3d");
    assert_eq!(definition.trace, ToolRunTraceKind::Instance3d);
    assert_eq!(definition.unit, label_of(&table["unit"]));
    assert_eq!(table["runJob"], RECONSTRUCTION_RUN_JOB_KIND);
    assert_eq!(table["revalidateJob"], RECONSTRUCTION_REVALIDATE_JOB_KIND);
    assert_eq!(definition.revalidate_job.as_ref().map(JobKindId::as_str), Some(RECONSTRUCTION_REVALIDATE_JOB_KIND));
    assert_eq!(table["traceMeshLane"], serde_json::json!(RECONSTRUCTION_TRACE_MESH_LANE));
    let stages = table["stages"].as_array().expect("stages");
    assert_eq!(stages.len(), definition.stages.len());
    for (entry, stage) in stages.iter().zip(&definition.stages) {
        assert_eq!(entry["id"], stage.id.as_str());
        assert_eq!(label_of(entry), stage.label);
    }
    let counters = table["counters"].as_array().expect("counters");
    assert_eq!(counters.len(), definition.counters.len());
    for (entry, counter) in counters.iter().zip(&definition.counters) {
        assert_eq!(entry["id"], counter.id.as_str());
        assert_eq!(label_of(entry), counter.label);
    }
    let reasons = table["reasons"].as_array().expect("reasons");
    assert_eq!(reasons.len(), definition.reasons.len());
    for (entry, reason) in reasons.iter().zip(&definition.reasons) {
        assert_eq!(entry["code"], reason.code);
        assert_eq!(entry["id"], reason.id.as_str());
        assert_eq!(entry["verdict"], verdict_id(reason.verdict));
        assert_eq!(label_of(entry), reason.template);
        let placeholders = (0..4).filter(|index| entry["en"].as_str().unwrap().contains(&format!("{{{index}}}"))).count();
        assert_eq!(entry["args"].as_array().expect("args").len(), placeholders, "reason {} names every template argument", reason.id);
    }
    assert_eq!(table["checkpoint"]["bytes"], ReconstructionRunCheckpoint::BYTES);
}

#[test]
fn the_checkpoint_round_trips_its_little_endian_layout_and_refuses_other_lengths() {
    let mut inputs = RemodelingContentDigest::default();
    inputs.record(b"frames, params and calibration");
    let checkpoint = ReconstructionRunCheckpoint { base_revision: [3; 32], inputs, decisions: 0x0102_0304_0506_0708, provisional_ops: 0x0a0b_0c0d };
    let bytes = checkpoint.encode();
    assert_eq!(bytes.len(), 84);
    assert_eq!(&bytes[72..80], &0x0102_0304_0506_0708u64.to_le_bytes());
    assert_eq!(ReconstructionRunCheckpoint::decode(&bytes), Some(checkpoint));
    assert_eq!(ReconstructionRunCheckpoint::decode(&bytes[..83]), None);
}

fn step_kind_id(kind: ToolRunStepKind) -> &'static str {
    match kind {
        ToolRunStepKind::Info => "info",
        ToolRunStepKind::Success => "success",
        ToolRunStepKind::Warning => "warning",
        ToolRunStepKind::Danger => "danger",
    }
}

fn fresh_run(document: Arc<RemodelingSnapshot>) -> ReconstructionRunJob {
    ReconstructionRunJob::new(identity(), document, None, 0)
}

#[test]
fn bounded_resize_matches_shared_fixture_and_image_oracle() {
    let case: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️resize.json")).unwrap();
    for row in case["extents"].as_array().unwrap() {
        let values: Vec<u32> = row.as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
        assert_eq!(frame_extent(values[0], values[1], values[2]), [values[3], values[4]]);
    }
    let width = case["width"].as_u64().unwrap() as u32;
    let height = case["height"].as_u64().unwrap() as u32;
    let pixels: Vec<u8> = serde_json::from_value(case["pixels"].clone()).unwrap();
    let expected: Vec<u8> = serde_json::from_value(case["expected"].clone()).unwrap();
    let [target_width, target_height] = frame_extent(width, height, case["longEdge"].as_u64().unwrap() as u32);
    let reference = image::RgbaImage::from_raw(width, height, pixels.clone()).unwrap();
    let oracle = image::imageops::resize(&reference, target_width, target_height, image::imageops::FilterType::Triangle);
    let mut job = semio_framework_pixels::editing::PixelEditJob::new(semio_framework_pixels::RasterImage { width, height, pixels }, semio_framework_pixels::editing::PixelOperation::Resize { width: target_width, height: target_height, bilinear: true }, None).unwrap();
    while !job.advance_bounded(1, 4).unwrap().done {}
    assert_eq!(job.result().unwrap().pixels, expected);
    assert_eq!(oracle.into_raw(), expected);
}


#[semio_framework_async_macros::async_test]
async fn ingest_binds_the_source_calibration_and_keeps_duplicate_frame_indices_distinct() {
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{CameraCalibration, ImageAsset, MediaKind, VideoSource};
    use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation;

    let mut document = Arc::unwrap_or_clone(imported_document("checker").await);
    document.params.ingest.min_sharpness = 0.0;
    document.params.ingest.downscale_long_edge_px = 16;
    document.streams[0].frames.truncate(1);
    document.streams[0].camera_id = Some("left".into());
    let mut right = document.streams[0].clone();
    right.id = "right-stream".into();
    right.camera_id = Some("right".into());
    right.kind = MediaKind::Video;
    right.sync_offset_ms = -10.0;
    right.source = Some(VideoSource { width: 640, height: 320, ..VideoSource::default() });
    right.frames[0].asset_id = "right-frame-0".into();
    document.streams.push(right);

    let data_url = checker_data_url(32, 16, 3).await;
    let (_, data) = data_url.split_once(',').expect("checker data URL");
    document = apply_remodeling_mutation(&document, &create_asset("right-frame-0".into(), ImageAsset { mime: "image/png".into(), data: data.into(), width: 32, height: 16 })).expect("right frame asset");
    document.calibration.cameras = vec![
        CameraCalibration { id: "right".into(), model: "fisheye".into(), fx: 18.0, fy: 21.0, cx: 17.0, cy: 7.0, skew: -0.5, distortion: [0.0625, 0.0, 0.0, 0.25, 0.0], ..CameraCalibration::default() },
        CameraCalibration { id: "left".into(), model: "brownConrady".into(), fx: 25.0, fy: 22.0, cx: 11.0, cy: 13.0, skew: 0.25, distortion: [0.03125, 0.0, 0.0, 0.125, -0.25], ..CameraCalibration::default() },
    ];

    let inputs = reconstruction_inputs(&document);
    let mut changed = document.clone();
    changed.streams[0].camera_id = Some("right".into());
    assert_ne!(reconstruction_inputs(&changed), inputs);
    changed.streams[0].camera_id = Some("left".into());
    changed.streams[1].sync_offset_ms = 0.0;
    assert_ne!(reconstruction_inputs(&changed), inputs);

    let mut job = fresh_run(Arc::new(document));
    let mut verdicts = Vec::new();
    for _ in 0..1_000 {
        if let Some(verdict) = job.ingest().expect("ingest step") {
            verdicts.push(verdict);
        }
        if job.scene.is_none() {
            break;
        }
    }
    assert_eq!(verdicts, vec![(0, 0, ReconstructionRunReason::FrameAccepted), (1 << 32, 0, ReconstructionRunReason::FrameAccepted)]);
    let engine = job.engine.as_mut().expect("reconstruction engine");
    engine.start().expect("mixed-size captures");
    let [right, left] = engine.frames.as_slice() else { panic!("two accepted frames") };
    assert_eq!((right.image.width, right.image.height, right.capture.stream, right.timestamp_ms), (16, 8, 1, -10.0));
    assert_eq!((left.image.width, left.image.height, left.capture.stream, left.timestamp_ms), (16, 16, 0, 0.0));
    let right_intrinsics = right.capture.intrinsics.expect("right calibration");
    let left_intrinsics = left.capture.intrinsics.expect("left calibration");
    assert_eq!([right_intrinsics.fx, right_intrinsics.fy, right_intrinsics.cx, right_intrinsics.cy, right_intrinsics.skew], [9.0, 10.5, 8.25, 3.25, -0.25]);
    let scale = 16.0 / 24.0;
    assert_eq!([left_intrinsics.fx, left_intrinsics.fy, left_intrinsics.cx, left_intrinsics.cy, left_intrinsics.skew], [25.0 * scale, 22.0 * scale, 11.5 * scale - 0.5, 13.5 * scale - 0.5, 0.25 * scale]);
    assert_eq!(right_intrinsics.project([0.0, 0.0, 1.0]), Some([8.25, 3.25]));
    assert_eq!(left_intrinsics.project([0.0, 0.0, 1.0]), Some([11.5 * scale - 0.5, 13.5 * scale - 0.5]));
}

#[semio_framework_async_macros::async_test]
async fn the_run_matches_the_language_neutral_fixture() {
    for case in fixture()["cases"].as_array().expect("cases") {
        let document = case["document"].as_str().expect("document");
        let mut job = fresh_run(imported_document(document).await);
        let mirror = run_to_end(&mut job, fixture_budget(document));
        assert_eq!(if mirror.completed { "complete" } else if mirror.faulted { "faulted" } else { "open" }, case["final"], "{document}: final outcome");
        let prefix: Vec<String> = mirror.verdicts.iter().map(|(_, verdict, reason)| format!("{}:{}", verdict_id(*verdict), reason_id(*reason))).collect();
        let expected_prefix: Vec<&str> = case["verdictPrefix"].as_array().expect("prefix").iter().map(|entry| entry.as_str().expect("verdict")).collect();
        assert_eq!(&prefix[..expected_prefix.len()], expected_prefix.as_slice(), "{document}: verdict prefix");
        assert_eq!(mirror.verdicts.len() as u64, case["verdicts"].as_u64().expect("verdicts"), "{document}: every evaluated element is traced once");
        for counter in ReconstructionRunCounter::ALL {
            assert_eq!(mirror.counter(counter), case["counters"][counter.id()].as_u64().expect("counter"), "{document}: counter {}", counter.id());
        }
        let stages: Vec<&str> = mirror.stages.iter().map(|stage| ReconstructionRunStage::ALL[usize::from(*stage)].id()).collect();
        let expected_stages: Vec<&str> = case["stagesVisited"].as_array().expect("stages").iter().map(|entry| entry.as_str().expect("stage")).collect();
        assert_eq!(stages, expected_stages, "{document}: stages visited");
        let appends = mirror.ops.iter().filter(|op| matches!(op, RemodelingMutation::AppendContent(_))).count() as u64;
        let commits = mirror.ops.iter().filter(|op| matches!(op, RemodelingMutation::CommitReconstruction(_))).count() as u64;
        assert_eq!(appends, case["appendContentOps"].as_u64().expect("appends"), "{document}: append-content ops");
        assert_eq!(commits, case["commitReconstructionOps"].as_u64().expect("commits"), "{document}: commit ops");
        assert!(mirror.ops.iter().all(|op| matches!(op, RemodelingMutation::AppendContent(_) | RemodelingMutation::CommitReconstruction(_))), "{document}: a run only appends content and commits the result");
        assert!(commits == 0 || matches!(mirror.ops.last(), Some(RemodelingMutation::CommitReconstruction(_))), "{document}: the commit is the last provisional op");
        if let Some(RemodelingMutation::CommitReconstruction(commit)) = mirror.ops.last() {
            let expected = case["registeredFrameRatio"].as_f64().expect("registered frame ratio");
            let ratio = f64::from(commit.qc.as_ref().expect("published quality report").registered_frame_ratio);
            let counters = &case["counters"];
            let registered = counters["cameras"].as_f64().expect("registered cameras");
            let accepted = counters["framesAccepted"].as_f64().expect("accepted frames");
            assert_eq!(expected, registered / accepted, "{document}: fixture registration ratio");
            assert_eq!(ratio, expected, "{document}: published registration ratio");
        }
        let (kind, reason, args) = mirror.steps.last().expect("a last step");
        assert_eq!(step_kind_id(*kind), case["lastStep"]["kind"], "{document}: last step kind");
        assert_eq!(reason_id(*reason), case["lastStep"]["reason"], "{document}: last step reason");
        assert_eq!(serde_json::json!(args), case["lastStep"]["args"], "{document}: last step args");
        for (_, (verdict, reason, _)) in &mirror.resident {
            assert_eq!(ReconstructionRunReason::from_code(*reason).map(ReconstructionRunReason::verdict), Some(*verdict), "{document}: every record carries its reason's declared verdict");
        }
        assert!(mirror.resident.values().filter(|(_, reason, _)| *reason == ReconstructionRunReason::PointsTriangulated.code() || *reason == ReconstructionRunReason::CameraRegistered.code()).all(|(_, _, subject)| matches!(subject, ToolRunTraceSubject::Instance3d { .. })), "{document}: cameras and points appear as instance3d records");
        let _ = job;
    }
}

/// 🧾️ Prints every fixture case's recorded quantities as JSON (`[fixture-actual] {...}`), for
/// re-recording `🧫️fixtures/🔣️.json` after an intended pipeline change. Run explicitly.
#[semio_framework_async_macros::async_test]
#[ignore = "prints the run fixture's actual values for re-recording, run explicitly"]
async fn prints_the_run_fixture_actuals() {
    for case in fixture()["cases"].as_array().expect("cases") {
        let document = case["document"].as_str().expect("document");
        let mut job = fresh_run(imported_document(document).await);
        let mirror = run_to_end(&mut job, fixture_budget(document));
        let counters: serde_json::Map<String, serde_json::Value> = ReconstructionRunCounter::ALL.iter().map(|counter| (counter.id().to_string(), serde_json::json!(mirror.counter(*counter)))).collect();
        let stages: Vec<&str> = mirror.stages.iter().map(|stage| ReconstructionRunStage::ALL[usize::from(*stage)].id()).collect();
        let appends = mirror.ops.iter().filter(|op| matches!(op, RemodelingMutation::AppendContent(_))).count();
        let commits = mirror.ops.iter().filter(|op| matches!(op, RemodelingMutation::CommitReconstruction(_))).count();
        let (kind, reason, args) = mirror.steps.last().expect("a last step");
        println!(
            "[fixture-actual] {}",
            serde_json::json!({ "document": document, "final": if mirror.completed { "complete" } else if mirror.faulted { "faulted" } else { "open" }, "verdicts": mirror.verdicts.len(), "counters": counters, "stagesVisited": stages, "appendContentOps": appends, "commitReconstructionOps": commits, "lastStep": { "kind": step_kind_id(*kind), "reason": reason_id(*reason), "args": args } })
        );
        let _ = job;
    }
}

/// 🔬️ Recomputes one match candidate's verdict with `bitvec` Hamming distances over the descriptor words.
fn oracle_match_verdict(descriptors: &[Vec<[u64; 4]>], frame_a: usize, frame_b: usize, query: usize, ratio: f32, mutual: bool) -> Option<ReconstructionRunReason> {
    use bitvec::{order::Lsb0, view::BitView};
    let distance = |left: &[u64; 4], right: &[u64; 4]| {
        let difference: [u64; 4] = std::array::from_fn(|word| left[word] ^ right[word]);
        difference.view_bits::<Lsb0>().count_ones() as u32
    };
    let (a, b) = (&descriptors[frame_a], &descriptors[frame_b]);
    let mut best = (u32::MAX, usize::MAX);
    let mut second = u32::MAX;
    for (candidate, descriptor) in b.iter().enumerate() {
        let value = distance(&a[query], descriptor);
        if value < best.0 {
            second = best.0;
            best = (value, candidate);
        } else if value < second {
            second = value;
        }
    }
    if best.1 == usize::MAX {
        return None;
    }
    if !(second == u32::MAX || (best.0 as f32) < ratio * second as f32) {
        return Some(ReconstructionRunReason::MatchAmbiguous);
    }
    if !mutual {
        return Some(ReconstructionRunReason::MatchAccepted);
    }
    let mut reverse = (u32::MAX, usize::MAX);
    for (candidate, descriptor) in a.iter().enumerate() {
        let value = distance(&b[best.1], descriptor);
        if value < reverse.0 {
            reverse = (value, candidate);
        }
    }
    Some(if reverse.1 == query { ReconstructionRunReason::MatchAccepted } else { ReconstructionRunReason::MatchAsymmetric })
}

#[semio_framework_async_macros::async_test]
async fn every_traced_match_verdict_agrees_with_the_bitvec_hamming_oracle() {
    let mut job = fresh_run(imported_document("synthetic-orbit").await);
    let mut sequence = RunDriver::new(never);
    let mut mirror = Mirror::default();
    let matching_finished = (0..10_000_000).any(|_| match turn(&mut job, whole_turns(), &mut sequence) {
        Turn::Tick(tick) => {
            mirror.fold(tick);
            mirror.stages.last().is_some_and(|stage| *stage > ReconstructionRunStage::Matching.index())
        }
        Turn::Checkpoint(_) | Turn::Yield => false,
        Turn::Complete | Turn::Fault => panic!("the orbit settled before publishing its matching results"),
    });
    assert!(matching_finished, "the full orbit published all matching decisions");
    let engine = job.engine.as_ref().expect("the engine stays resident until close");
    let descriptors: Vec<Vec<[u64; 4]>> = engine.descriptors_per_frame.iter().map(|frame| frame.iter().map(|descriptor| descriptor.0).collect()).collect();
    let mut decisive = [0usize; 3];
    for &(frame_a, frame_b) in &engine.match_pairs {
        for query in 0..descriptors[frame_a].len() {
            let expected = oracle_match_verdict(&descriptors, frame_a, frame_b, query, engine.params.match_ratio, engine.params.match_mutual);
            let traced = mirror.resident.get(&match_trace_key(frame_a, frame_b, query as u32)).map(|(_, reason, _)| ReconstructionRunReason::from_code(*reason).expect("run reason"));
            assert_eq!(traced, expected, "pair {frame_a}-{frame_b} query {query}");
            match expected {
                Some(ReconstructionRunReason::MatchAccepted) => decisive[0] += 1,
                Some(ReconstructionRunReason::MatchAmbiguous) => decisive[1] += 1,
                Some(_) => decisive[2] += 1,
                None => {}
            }
        }
    }
    assert!(decisive.iter().all(|count| *count > 0), "the oracle saw accepted, ratio-rejected and cross-check-rejected candidates: {decisive:?}");
    sequence.close_job(&mut job);
}

#[semio_framework_async_macros::async_test]
async fn a_single_unit_of_fuel_is_one_visible_decision() {
    let law = &fixture()["fuel"];
    let mut job = fresh_run(imported_document(law["document"].as_str().expect("document")).await);
    let mirror = run_to_end(&mut job, budget(law["fuelPerStep"].as_u64().expect("fuel"), u64::MAX));
    assert!(mirror.completed);
    assert_eq!(mirror.evidence_ticks, mirror.ticks, "every tick of a single-unit step shows at least one trace record, step or provisional op");
    let whole = run_to_end(&mut fresh_run(imported_document(law["document"].as_str().expect("document")).await), whole_turns());
    assert_eq!(mirror.verdicts, whole.verdicts, "slicing by fuel never changes the decisions");
    assert_eq!(mirror.ops, whole.ops, "slicing by fuel never changes the provisional result");
}

#[semio_framework_async_macros::async_test]
async fn a_resumed_run_replays_silently_to_its_checkpoint_and_ends_with_the_same_result() {
    let law = &fixture()["resume"];
    let document = imported_document(law["document"].as_str().expect("document")).await;
    let whole = run_to_end(&mut fresh_run(Arc::clone(&document)), whole_turns());
    let wanted = law["checkpointIndex"].as_u64().expect("checkpoint index") as usize;
    let mut first = fresh_run(Arc::clone(&document));
    let mut mirror = Mirror::default();
    let mut sequence = RunDriver::new(never);
    let checkpoint = loop {
        match turn(&mut first, whole_turns(), &mut sequence) {
            Turn::Tick(tick) => mirror.fold(tick),
            Turn::Checkpoint(bytes) => {
                mirror.checkpoints.push(bytes.clone());
                if mirror.checkpoints.len() > wanted {
                    break bytes;
                }
            }
            Turn::Complete | Turn::Fault => panic!("the run settled before checkpoint {wanted}"),
            Turn::Yield => {}
        }
    };
    sequence.close_job(&mut first);
    drop(sequence);
    let mut sequence = RunDriver::new(never);
    let decoded = ReconstructionRunCheckpoint::decode(&checkpoint).expect("checkpoint decodes");
    assert!(decoded.decisions > 0);
    let steps_before = mirror.steps.len();
    let mut resumed = ReconstructionRunJob::new(identity(), Arc::clone(&document), Some(&checkpoint), mirror.ops.len() as u32);
    loop {
        match turn(&mut resumed, whole_turns(), &mut sequence) {
            Turn::Tick(tick) => mirror.fold(tick),
            Turn::Checkpoint(_) | Turn::Yield => {}
            Turn::Complete => break,
            Turn::Fault => panic!("the resumed run faulted"),
        }
    }
    sequence.close_job(&mut resumed);
    assert_eq!(reason_id(mirror.steps[steps_before].1), law["firstResumedStep"], "the resumed job emits nothing before it is live again");
    assert_eq!(mirror.ops, whole.ops, "the resumed run publishes exactly the uninterrupted provisional result");
    assert_eq!(mirror.resident.keys().collect::<Vec<_>>(), whole.resident.keys().collect::<Vec<_>>(), "the resumed trace holds exactly the uninterrupted records");
    for counter in ReconstructionRunCounter::ALL {
        assert_eq!(mirror.counter(counter), whole.counter(counter), "counter {}", counter.id());
    }
}

#[semio_framework_async_macros::async_test]
async fn the_revalidate_job_keeps_the_result_on_an_unchanged_head_and_withdraws_it_when_inputs_changed() {
    let law = &fixture()["revalidation"];
    let document = imported_document(law["document"].as_str().expect("revalidation example")).await;
    let mut run = fresh_run(Arc::clone(&document));
    let whole = run_to_end(&mut run, whole_turns());
    let checkpoint = whole.checkpoints.last().expect("a final checkpoint").clone();
    let provisional = whole.ops.len() as u32;
    let mut unrelated = (*document).clone();
    unrelated.id.push_str("-renamed");
    let added_gcp = Arc::new(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&document, &create_gcp(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [0.0, 0.0, 0.0], observations: Vec::new() })).expect("gcp applies"));
    let changed = Arc::new(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&document, &semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_ingest_params::update_ingest_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams { frame_sample_stride: 2, ..document.params.ingest.clone() })).expect("params apply"));
    for (edit, head) in [("documentIdentity", Arc::new(unrelated)), ("groundControlPresence", added_gcp), ("ingestStride", changed)] {
        let withdrawn = law["withdrawn"][edit].as_bool().expect("revalidation verdict");
        let mut job = ReconstructionRevalidateJob::new(identity(), head, Some(&checkpoint), provisional);
        let mut sequence = RunDriver::new(never);
        let Turn::Tick(tick) = turn(&mut job, whole_turns(), &mut sequence) else { panic!("revalidation reports one tick") };
        assert_eq!(tick.retract_to, withdrawn.then_some(0), "withdrawn = {withdrawn}");
        assert_eq!(tick.steps.iter().any(|step| step.reason == ReconstructionRunReason::InputsChanged.code() && step.kind == ToolRunStepKind::Danger), withdrawn);
        assert!(matches!(turn(&mut job, whole_turns(), &mut sequence), Turn::Complete));
        sequence.close_job(&mut job);
    }
}

#[semio_framework_async_macros::async_test]
async fn the_provisional_result_applies_onto_its_base_and_its_inverse_restores_the_base() {
    let document = imported_document("orbit-short").await;
    let mirror = run_to_end(&mut fresh_run(Arc::clone(&document)), whole_turns());
    let mut inverses = Vec::new();
    let mut current = (*document).clone();
    for op in &mirror.ops {
        let outcome = <RemodelingMutation as protocol::ColdMutation<RemodelingSnapshot>>::diff(op, &current);
        assert!(outcome.messages().iter().all(|message| message.level != semio_framework_diagnostic::Severity::Error), "every provisional op applies onto the overlay it was planned against: {:?}", outcome.messages());
        inverses.push(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::inverse_remodeling_mutation(&current, op).expect("valid retained mutation inverse fixture"));
        current = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&current, op).expect("provisional op applies");
    }
    assert_ne!(current.results.mesh.source, MeshSource::Placeholder, "the committed result replaces the placeholder mesh");
    assert!(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::resolve_bounded_remodeling_mesh(&current.durable_artifacts, &current.results.mesh.mesh).is_some(), "the committed mesh handle resolves from the appended content");
    for steps in inverses.into_iter().rev() {
        for step in steps {
            current = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::apply_remodeling_mutation(&current, &step).expect("inverse step applies");
        }
    }
    assert_eq!(current, *document, "undoing the run's single edit restores its base byte for byte");
}

#[semio_framework_async_macros::async_test]
async fn every_bounded_unit_stays_under_the_interactive_ceiling_on_every_example() {
    let law = &fixture()["interactive"];
    let ceiling = Duration::from_micros(law["ceilingUs"].as_u64().expect("ceiling"));
    let target = Duration::from_micros(law["targetUs"].as_u64().expect("target"));
    assert_eq!(ceiling, Duration::from_micros(semio_framework_job::INTERACTIVE_STEP_CEILING_US), "the fixture ceiling is the framework's interactive step ceiling");
    for name in law["unitDocuments"].as_array().expect("unit documents").iter().map(|entry| entry.as_str().expect("document")) {
        let document = imported_document(name).await;
        let mut best: Vec<(Duration, &'static str)> = Vec::new();
        let mut labels: Vec<Option<String>> = Vec::new();
        for run in 0..law["coldRuns"].as_u64().expect("cold runs") as usize {
            let mut job = fresh_run(Arc::clone(&document));
            let mut sequence = RunDriver::new(unit_clock);
            for index in 0.. {
                let stage = job.stage.id();
                let unit = job.engine.as_ref().map(|engine| engine.unit_label());
                let started = Instant::now();
                let deadline = if job.outbox.kind.is_none() && job.owed.is_empty() { 0 } else { u64::MAX };
                let outcome = step_once(&mut job, budget(semio_framework_job::INTERACTIVE_LANE_FUEL, deadline), &mut sequence);
                let elapsed = started.elapsed();
                if run == 0 {
                    best.push((elapsed, stage));
                    labels.push(unit);
                } else {
                    assert!(index < best.len(), "{name}: cold run {run} took more units than the recorded run");
                    best[index].0 = best[index].0.min(elapsed);
                }
                if matches!(outcome, Some(Turn::Complete | Turn::Fault)) {
                    assert_eq!(index + 1, best.len(), "{name}: every cold run slices into the same units");
                    break;
                }
            }
            sequence.close_job(&mut job);
        }
        let (worst, stage) = best.iter().copied().max_by_key(|(elapsed, _)| *elapsed).expect("units");
        let over_target = best.iter().filter(|(elapsed, _)| *elapsed >= target).count();
        let over_ceiling = best.iter().filter(|(elapsed, _)| *elapsed >= ceiling).count();
        let (mut sustained, mut sustained_end, mut run) = (0u32, 0usize, 0u32);
        for (index, (elapsed, _)) in best.iter().enumerate() {
            run = if *elapsed >= ceiling { run + 1 } else { 0 };
            if run > sustained {
                sustained = run;
                sustained_end = index;
            }
        }
        let window: Vec<String> = (sustained_end.saturating_sub(sustained as usize).saturating_add(1)..=sustained_end).map(|index| format!("{:?} {}", best[index].0, labels[index].clone().unwrap_or_default())).collect();
        assert!(
            sustained < semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS,
            "{name}: {sustained} consecutive bounded units overran the {ceiling:?} ceiling, a sustained overrun the watchdog quarantines (worst {worst:?} in stage {stage}; {over_ceiling} over the ceiling and {over_target} over the {target:?} target of {} units); the window: {window:#?}",
            best.len()
        );
    }
}

/// 🖨️ Prints every fixture case of `🧫️fixtures/🔣️.json` as the current engine produces it, to
/// re-record the language-neutral fixture after an engine change. Diagnostic, therefore `#[ignore]`:
/// `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture print_run_fixture_cases`.
#[semio_framework_async_macros::async_test]
#[ignore = "re-records the run fixture cases; run explicitly"]
async fn print_run_fixture_cases() {
    for document in ["synthetic-orbit", "checker", "orbit-short"] {
        let mut job = fresh_run(imported_document(document).await);
        let mirror = run_to_end(&mut job, fixture_budget(document));
        let prefix: Vec<String> = mirror.verdicts.iter().take(24).map(|(_, verdict, reason)| format!("{}:{}", verdict_id(*verdict), reason_id(*reason))).collect();
        let counters: serde_json::Map<String, serde_json::Value> = ReconstructionRunCounter::ALL.iter().map(|counter| (counter.id().to_string(), serde_json::json!(mirror.counter(*counter)))).collect();
        let stages: Vec<&str> = mirror.stages.iter().map(|stage| ReconstructionRunStage::ALL[usize::from(*stage)].id()).collect();
        let appends = mirror.ops.iter().filter(|op| matches!(op, RemodelingMutation::AppendContent(_))).count();
        let commits = mirror.ops.iter().filter(|op| matches!(op, RemodelingMutation::CommitReconstruction(_))).count();
        let (kind, reason, args) = mirror.steps.last().expect("a last step");
        let case = serde_json::json!({
            "document": document,
            "final": if mirror.completed { "complete" } else if mirror.faulted { "faulted" } else { "open" },
            "verdictPrefix": prefix,
            "verdicts": mirror.verdicts.len(),
            "counters": counters,
            "stagesVisited": stages,
            "appendContentOps": appends,
            "commitReconstructionOps": commits,
            "lastStep": { "kind": step_kind_id(*kind), "reason": reason_id(*reason), "args": args },
            "checkpoints": mirror.checkpoints.len(),
        });
        eprintln!("[FIXTURE] {}", serde_json::to_string_pretty(&case).expect("case json"));
    }
}

#[semio_framework_async_macros::async_test]
async fn cancellation_stops_publication_and_retires_the_run_with_its_original_caller() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️cancel.json")).expect("cancel fixture");
    let mut job = fresh_run(imported_document(law["document"].as_str().expect("document")).await);
    let mut driver = RunDriver::new(never);
    let wanted = law["decisionsBeforeCancel"].as_u64().expect("decisions");
    for _ in 0..100_000 {
        if job.decisions >= wanted { break; }
        let _ = step_once(&mut job, budget(1, u64::MAX), &mut driver);
    }
    assert_eq!(job.decisions, wanted);
    driver.caller.as_ref().expect("original caller").cancel().cancel_now();
    for _ in 0..law["turnsAfterCancel"].as_u64().expect("cancel turns") {
        assert!(step_once(&mut job, budget(1, u64::MAX), &mut driver).is_none(), "cancelled caller publishes no further outcomes");
    }
    assert_eq!(job.decisions, law["decisionsAfterCancel"].as_u64().expect("unchanged decisions"));
    driver.close_job(&mut job);
    assert_eq!(job.scene.is_some(), law["sceneRetainedAfterClose"].as_bool().expect("scene retired"));
    assert_eq!(job.engine.is_some(), law["engineRetainedAfterClose"].as_bool().expect("engine retired"));
    assert!(job.observations.is_empty());
    assert!(job.owed.is_empty());
}
