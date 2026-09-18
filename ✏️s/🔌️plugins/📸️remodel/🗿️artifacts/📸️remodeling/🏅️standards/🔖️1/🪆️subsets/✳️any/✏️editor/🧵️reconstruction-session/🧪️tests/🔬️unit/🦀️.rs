use super::*;
use crate::editor::remodeling::commands::import_frame_payload::checker_data_url;
use crate::examples::synthetic_orbit::{FRAMES, FRAME_MIME};
use semio_framework_job::{root_cancel_token, StepBudget, JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_tool_run::{ToolRunId, ToolRunTick, ToolRunTraceOp};
use std::time::{Duration, Instant};

const RUN_SCHEMA: &str = include_str!("../../🔣️.json");
const RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

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

/// 🎞️ Views of the committed orbit the short document keeps, and the long edge it ingests them at.
const ORBIT_SHORT_FRAMES: usize = 8;
const ORBIT_SHORT_LONG_EDGE_PX: u32 = 160;

/// 📥️ The committed `synthetic-orbit` document (calibrated camera, parameters, frame table) with its
/// thirty-six frames bound through `create-asset`; `orbit-short`, the same document cut to its first
/// [`ORBIT_SHORT_FRAMES`] views and ingested at [`ORBIT_SHORT_LONG_EDGE_PX`] — a capture that
/// reconstructs (cameras, points, a mesh) in a fraction of the full orbit's time, for the laws that
/// need a published result; or four identical 24×24 checker frames on one stream sampled at stride 1
/// — a capture no two-view geometry can be solved from (zero baseline), for the laws about a run
/// that completes without cameras.
async fn imported_document(example: &str) -> Arc<RemodelingSnapshot> {
    if example == "synthetic-orbit" || example == "orbit-short" {
        let mut scene = <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::synthetic_orbit::PRIMARY_TEXT).expect("synthetic-orbit document parses");
        let kept = if example == "orbit-short" { ORBIT_SHORT_FRAMES } else { FRAMES.len() };
        for (asset_id, bytes) in FRAMES.iter().take(kept) {
            let image = crate::editor::remodeling::decode_still_image(FRAME_MIME, bytes).expect("frame decodes");
            let asset = crate::ImageAsset { mime: FRAME_MIME.into(), data: base64_codec::base64_standard_encode(bytes), width: image.width, height: image.height };
            scene = crate::mutations::apply_remodeling_mutation(&scene, &crate::mutations::create_asset((*asset_id).into(), asset)).expect("frame asset applies");
        }
        if example == "orbit-short" {
            for stream in &mut scene.streams {
                stream.frames.truncate(kept);
            }
            scene = crate::mutations::apply_remodeling_mutation(&scene, &crate::mutations::update_ingest_params(crate::IngestParams { downscale_long_edge_px: ORBIT_SHORT_LONG_EDGE_PX, ..scene.params.ingest.clone() })).expect("ingest params apply");
        }
        return Arc::new(scene);
    }
    let payload = checker_data_url(24, 24, 3).await;
    let bytes = base64_codec::base64_standard_decode(payload.split_once(',').expect("data url").1).expect("checker bytes");
    let mut scene = crate::default_remodeling_scene();
    scene = crate::mutations::apply_remodeling_mutation(&scene, &crate::mutations::update_ingest_params(crate::IngestParams { frame_sample_stride: 1, ..scene.params.ingest.clone() })).expect("every frame is sampled");
    for index in 0..4u32 {
        let asset_id = format!("checker-frame-{index}");
        let frame = FrameRef { index, timestamp_ms: f64::from(index) * 500.0, asset_id: asset_id.clone() };
        let stream = if index == 0 {
            crate::mutations::create_stream(crate::MediaStream { id: "checker".into(), name: "checker".into(), kind: crate::MediaKind::ImageSequence, camera_id: None, sync_offset_ms: 0.0, fps_hint: 2.0, frames: vec![frame], source: None })
        } else {
            crate::mutations::add_stream_frame("checker".into(), frame, crate::MediaKind::ImageSequence)
        };
        for mutation in [crate::mutations::create_asset(asset_id, crate::ImageAsset { mime: "image/png".into(), data: base64_codec::base64_standard_encode(&bytes), width: 24, height: 24 }), stream] {
            scene = crate::mutations::apply_remodeling_mutation(&scene, &mutation).expect("frame import applies");
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

fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

fn settle(outcome: StepOutcome) -> Turn {
    match outcome {
        StepOutcome::PreviewReady(payload) => {
            let page = payload.single_page().expect("a tick is one payload page").to_vec();
            close_payload(payload);
            Turn::Tick(ToolRunTick::decode(&page).expect("tick decodes"))
        }
        StepOutcome::CheckpointReady(checkpoint) => {
            let bytes = checkpoint.state.single_page().expect("checkpoint page").to_vec();
            close_payload(checkpoint.state);
            Turn::Checkpoint(bytes)
        }
        StepOutcome::Complete(candidate) => {
            close_payload(candidate.state);
            close_payload(candidate.output);
            Turn::Complete
        }
        StepOutcome::Fault(fault) => {
            close_payload(fault.detail);
            Turn::Fault
        }
        StepOutcome::Cancelled => panic!("the run was never cancelled"),
        StepOutcome::Yield => Turn::Yield,
    }
}

fn turn(job: &mut impl InteractiveJob, budget: StepBudget, sequence: &mut u64) -> Turn {
    let mut context = StepContext::new(semio_framework_job::OperationId(91), semio_framework_job::Generation(1), budget, root_cancel_token(), never, sequence);
    settle(job.step(&mut context))
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
    let mut sequence = 0;
    for _ in 0..10_000_000 {
        match turn(job, budget, &mut sequence) {
            Turn::Tick(tick) => mirror.fold(tick),
            Turn::Checkpoint(bytes) => mirror.checkpoints.push(bytes),
            Turn::Complete => {
                mirror.completed = true;
                return mirror;
            }
            Turn::Fault => {
                mirror.faulted = true;
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

const WHOLE_TURNS: StepBudget = StepBudget { fuel: u64::MAX, deadline_us: u64::MAX };

#[semio_framework_async_macros::async_test]
async fn the_run_matches_the_language_neutral_fixture() {
    for case in fixture()["cases"].as_array().expect("cases") {
        let document = case["document"].as_str().expect("document");
        let mut job = fresh_run(imported_document(document).await);
        let mirror = run_to_end(&mut job, StepBudget::new(1, u64::MAX));
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

/// 🔬️ Recomputes one match candidate's verdict with `bitvec` Hamming distances over the descriptor words.
fn oracle_match_verdict(descriptors: &[Vec<[u64; 4]>], frame_a: usize, frame_b: usize, query: usize, ratio: f32, mutual: bool) -> Option<ReconstructionRunReason> {
    use bitvec::prelude::{BitVec, Lsb0};
    let bits = |words: &[u64; 4]| BitVec::<u64, Lsb0>::from_vec(words.to_vec());
    let distance = |left: &[u64; 4], right: &[u64; 4]| (bits(left) ^ bits(right)).count_ones() as u32;
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
    let mirror = run_to_end(&mut job, WHOLE_TURNS);
    let (descriptors, pairs, ratio, mutual) = job.engine.as_ref().expect("the engine stays resident until close").match_oracle_inputs();
    let mut decisive = [0usize; 3];
    for (frame_a, frame_b) in pairs {
        for query in 0..descriptors[frame_a].len() {
            let expected = oracle_match_verdict(&descriptors, frame_a, frame_b, query, ratio, mutual);
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
}

#[semio_framework_async_macros::async_test]
async fn a_single_unit_of_fuel_is_one_visible_decision() {
    let law = &fixture()["fuel"];
    let mut job = fresh_run(imported_document(law["document"].as_str().expect("document")).await);
    let mirror = run_to_end(&mut job, StepBudget::new(law["fuelPerStep"].as_u64().expect("fuel"), u64::MAX));
    assert!(mirror.completed);
    assert_eq!(mirror.evidence_ticks, mirror.ticks, "every tick of a single-unit step shows at least one trace record, step or provisional op");
    let whole = run_to_end(&mut fresh_run(imported_document(law["document"].as_str().expect("document")).await), WHOLE_TURNS);
    assert_eq!(mirror.verdicts, whole.verdicts, "slicing by fuel never changes the decisions");
    assert_eq!(mirror.ops, whole.ops, "slicing by fuel never changes the provisional result");
}

#[semio_framework_async_macros::async_test]
async fn a_resumed_run_replays_silently_to_its_checkpoint_and_ends_with_the_same_result() {
    let law = &fixture()["resume"];
    let document = imported_document(law["document"].as_str().expect("document")).await;
    let whole = run_to_end(&mut fresh_run(Arc::clone(&document)), WHOLE_TURNS);
    let wanted = law["checkpointIndex"].as_u64().expect("checkpoint index") as usize;
    let mut first = fresh_run(Arc::clone(&document));
    let mut mirror = Mirror::default();
    let mut sequence = 0;
    let checkpoint = loop {
        match turn(&mut first, WHOLE_TURNS, &mut sequence) {
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
    let decoded = ReconstructionRunCheckpoint::decode(&checkpoint).expect("checkpoint decodes");
    assert!(decoded.decisions > 0);
    let steps_before = mirror.steps.len();
    let mut resumed = ReconstructionRunJob::new(identity(), Arc::clone(&document), Some(&checkpoint), mirror.ops.len() as u32);
    loop {
        match turn(&mut resumed, WHOLE_TURNS, &mut sequence) {
            Turn::Tick(tick) => mirror.fold(tick),
            Turn::Checkpoint(_) | Turn::Yield => {}
            Turn::Complete => break,
            Turn::Fault => panic!("the resumed run faulted"),
        }
    }
    assert_eq!(reason_id(mirror.steps[steps_before].1), law["firstResumedStep"], "the resumed job emits nothing before it is live again");
    assert_eq!(mirror.ops, whole.ops, "the resumed run publishes exactly the uninterrupted provisional result");
    assert_eq!(mirror.resident.keys().collect::<Vec<_>>(), whole.resident.keys().collect::<Vec<_>>(), "the resumed trace holds exactly the uninterrupted records");
    for counter in ReconstructionRunCounter::ALL {
        assert_eq!(mirror.counter(counter), whole.counter(counter), "counter {}", counter.id());
    }
}

#[semio_framework_async_macros::async_test]
async fn the_revalidate_job_keeps_the_result_on_an_unchanged_head_and_withdraws_it_when_inputs_changed() {
    let document = imported_document("orbit-short").await;
    let mut run = fresh_run(Arc::clone(&document));
    let whole = run_to_end(&mut run, WHOLE_TURNS);
    let checkpoint = whole.checkpoints.last().expect("a final checkpoint").clone();
    let provisional = whole.ops.len() as u32;
    let unrelated = Arc::new(crate::mutations::apply_remodeling_mutation(&document, &crate::mutations::create_gcp(crate::GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [0.0, 0.0, 0.0], observations: Vec::new() })).expect("gcp applies"));
    let changed = Arc::new(crate::mutations::apply_remodeling_mutation(&document, &crate::mutations::update_ingest_params(crate::IngestParams { frame_sample_stride: 2, ..document.params.ingest.clone() })).expect("params apply"));
    for (head, withdrawn) in [(unrelated, false), (changed, true)] {
        let mut job = ReconstructionRevalidateJob::new(identity(), head, Some(&checkpoint), provisional);
        let mut sequence = 0;
        let Turn::Tick(tick) = turn(&mut job, WHOLE_TURNS, &mut sequence) else { panic!("revalidation reports one tick") };
        assert_eq!(tick.retract_to, withdrawn.then_some(0), "withdrawn = {withdrawn}");
        assert_eq!(tick.steps.iter().any(|step| step.reason == ReconstructionRunReason::InputsChanged.code() && step.kind == ToolRunStepKind::Danger), withdrawn);
        assert!(matches!(turn(&mut job, WHOLE_TURNS, &mut sequence), Turn::Complete));
    }
}

#[semio_framework_async_macros::async_test]
async fn the_provisional_result_applies_onto_its_base_and_its_inverse_restores_the_base() {
    let document = imported_document("orbit-short").await;
    let mirror = run_to_end(&mut fresh_run(Arc::clone(&document)), WHOLE_TURNS);
    let mut inverses = Vec::new();
    let mut current = (*document).clone();
    for op in &mirror.ops {
        let outcome = <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(op, &current);
        assert!(outcome.messages().iter().all(|message| message.level != protocol::Severity::Error), "every provisional op applies onto the overlay it was planned against: {:?}", outcome.messages());
        inverses.push(crate::mutations::inverse_remodeling_mutation(&current, op));
        current = crate::mutations::apply_remodeling_mutation(&current, op).expect("provisional op applies");
    }
    assert_ne!(current.results.mesh.source, MeshSource::Placeholder, "the committed result replaces the placeholder mesh");
    assert!(crate::resolve_bounded_remodeling_mesh(&current.durable_artifacts, &current.results.mesh.mesh).is_some(), "the committed mesh handle resolves from the appended content");
    for steps in inverses.into_iter().rev() {
        for step in steps {
            current = crate::mutations::apply_remodeling_mutation(&current, &step).expect("inverse step applies");
        }
    }
    assert_eq!(current, *document, "undoing the run's single edit restores its base byte for byte");
}

/// ⏱️ A clock whose deadline has always passed, so every step performs exactly one bounded unit.
fn expired_clock() -> Option<u64> {
    Some(u64::MAX)
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
            let mut sequence = 0;
            let cancel = root_cancel_token();
            for index in 0.. {
                let stage = job.stage.id();
                let unit = job.engine.as_ref().map(|engine| engine.unit_label());
                let started = Instant::now();
                let outcome = InteractiveJob::step(&mut job, &mut StepContext::new(semio_framework_job::OperationId(91), semio_framework_job::Generation(1), StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, 0), cancel.clone(), expired_clock, &mut sequence));
                let elapsed = started.elapsed();
                if run == 0 {
                    best.push((elapsed, stage));
                    labels.push(unit);
                } else {
                    assert!(index < best.len(), "{name}: cold run {run} took more units than the recorded run");
                    best[index].0 = best[index].0.min(elapsed);
                }
                if matches!(settle(outcome), Turn::Complete | Turn::Fault) {
                    assert_eq!(index + 1, best.len(), "{name}: every cold run slices into the same units");
                    break;
                }
            }
        }
        let (worst, stage) = best.iter().copied().max_by_key(|(elapsed, _)| *elapsed).expect("units");
        let over_target = best.iter().filter(|(elapsed, _)| *elapsed >= target).count();
        let over_ceiling = best.iter().filter(|(elapsed, _)| *elapsed >= ceiling).count();
        // The longest run of consecutive overruns and the index of its last unit.
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
    for document in ["checker", "orbit-short", "synthetic-orbit"] {
        let mut job = fresh_run(imported_document(document).await);
        let mirror = run_to_end(&mut job, StepBudget::new(1, u64::MAX));
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
