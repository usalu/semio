//! 🧵 The `Effect::SpawnJob` → `Event::JobCompleted` drive law, driven by the language-agnostic
//! fixture `🧫️fixtures/🧵️spawned-job-drive/🔣️.json` that `🎠️kernel/🟦️.ts`'s TypeScript twin drives
//! too.
//!
//! The defect this pins: a host may admit the effect and never pump it. On the wgpu target every
//! `interactionSelect`/`interactionHover` published exactly the right wire message and was applied
//! never, because the browser bridge had no `spawn-job` case at all — 18 `unmapped effect
//! "spawn-job" dropped` lines per run (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-world3d-interaction-2026-09-13.md` §7).

use super::*;

#[derive(serde::Deserialize)]
struct BudgetFixture {
    #[serde(rename = "stepCeiling")]
    step_ceiling: u32,
    fuel: String,
    #[serde(rename = "deadlineMs")]
    deadline_ms: u32,
}

#[derive(serde::Deserialize)]
struct PlacementFixture {
    wire: Vec<String>,
    refusals: Vec<String>,
}

#[derive(serde::Deserialize)]
struct TranscriptFixture {
    id: String,
    #[serde(default, rename = "runningPrefix")]
    running_prefix: u32,
    steps: Vec<SpawnedJobStep>,
    completion: CompletionFixture,
}

#[derive(serde::Deserialize)]
struct CompletionFixture {
    steps: u32,
    outcome: OutcomeFixture,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum OutcomeFixture {
    Ok(Vec<u8>),
    Fault(Vec<u8>),
}

#[derive(serde::Deserialize)]
struct RefusalFixture {
    id: String,
    #[serde(default, rename = "runningPrefix")]
    running_prefix: u32,
    steps: Vec<SpawnedJobStep>,
    error: ErrorFixture,
}

#[derive(serde::Deserialize)]
struct ErrorFixture {
    kind: String,
    steps: u32,
}

#[derive(serde::Deserialize)]
struct CompletedEventFixture {
    kind: String,
    cases: Vec<CompletedEventCase>,
}

#[derive(serde::Deserialize)]
struct CompletedEventCase {
    id: String,
    job: String,
    completion: CompletionFixture,
    #[serde(rename = "rustEventJson")]
    rust_event_json: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct DriveFixture {
    budget: BudgetFixture,
    placements: PlacementFixture,
    transcripts: Vec<TranscriptFixture>,
    refusals: Vec<RefusalFixture>,
    #[serde(rename = "completedEvent")]
    completed_event: CompletedEventFixture,
}

fn fixture() -> DriveFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧵️spawned-job-drive/🔣️.json")).expect("spawned-job-drive fixture JSON")
}

fn transcript(running_prefix: u32, tail: &[SpawnedJobStep]) -> Vec<SpawnedJobStep> {
    let mut steps = vec![SpawnedJobStep::Running; running_prefix as usize];
    steps.extend_from_slice(tail);
    steps
}

fn outcome_of(fixture: &OutcomeFixture) -> RequestOutcome {
    match fixture {
        OutcomeFixture::Ok(value) => RequestOutcome::Ok(value.clone()),
        OutcomeFixture::Fault(value) => RequestOutcome::Err(value.clone()),
    }
}

/// ⚖️ The host's static admission budget is the fixture's, to the byte — the same three numbers the
/// TypeScript twin reads, so the two renderers cannot drift into different patience.
#[test]
fn the_host_admission_budget_is_the_declared_one() {
    let fixture = fixture();
    assert_eq!(fixture.budget.step_ceiling, SPAWNED_JOB_STEP_CEILING);
    assert_eq!(fixture.budget.fuel.parse::<u64>().expect("fuel is a u64 decimal"), SPAWNED_JOB_FUEL);
    assert_eq!(fixture.budget.deadline_ms, SPAWNED_JOB_DEADLINE_MS);
}

/// 🚦 The placement vocabulary is closed, ordered and round-trips; nothing outside it resolves.
#[test]
fn the_placement_vocabulary_is_exactly_the_wit_enum() {
    let fixture = fixture();
    let declared: Vec<String> = JobPlacement::ALL.iter().map(|placement| placement.wire_name().to_string()).collect();
    assert_eq!(declared, fixture.placements.wire, "the WIT `enum job-placement` order is part of the contract");
    for name in &fixture.placements.wire {
        let placement = JobPlacement::from_wire_name(name).unwrap_or_else(|| panic!("{name} must resolve"));
        assert_eq!(placement.wire_name(), name, "{name} must round-trip");
    }
    for name in &fixture.placements.refusals {
        assert_eq!(JobPlacement::from_wire_name(name), None, "{name:?} must not resolve to a placement");
    }
}

/// 🧵 Every fixture transcript reaches exactly the completion the fixture declares.
#[test]
fn every_transcript_reaches_its_declared_completion() {
    let fixture = fixture();
    assert!(fixture.transcripts.len() >= 5, "the fixture must keep driving every terminal shape: {}", fixture.transcripts.len());
    for case in &fixture.transcripts {
        let steps = transcript(case.running_prefix, &case.steps);
        let completion = spawned_job_completion(&steps).unwrap_or_else(|error| panic!("{} must complete: {error}", case.id));
        assert_eq!(completion.steps, case.completion.steps, "{} took the wrong number of steps", case.id);
        assert_eq!(completion.outcome, outcome_of(&case.completion.outcome), "{} produced the wrong outcome", case.id);
    }
}

/// 🛑️ A guest refusal is a COMPLETION, not a drive error — the parked request resolves `Err` and the
/// actor sees the fault instead of hanging on a job that will never answer.
#[test]
fn a_failed_step_completes_the_job_with_a_fault() {
    let completion = spawned_job_completion(&[SpawnedJobStep::Running, SpawnedJobStep::Failed { value: b"nope".to_vec() }]).expect("a failed step is a completion");
    assert_eq!(completion.outcome, RequestOutcome::Err(b"nope".to_vec()));
    assert!(matches!(spawned_job_completion(&[SpawnedJobStep::Done { value: vec![] }]).expect("done").outcome, RequestOutcome::Ok(_)));
}

/// ⏳️ Every fixture refusal refuses, with the declared kind and step count.
#[test]
fn every_refusal_refuses_exactly_as_declared() {
    let fixture = fixture();
    assert!(fixture.refusals.len() >= 3, "the fixture must keep driving both refusal kinds");
    for case in &fixture.refusals {
        let steps = transcript(case.running_prefix, &case.steps);
        let error = spawned_job_completion(&steps).expect_err(&format!("{} must refuse", case.id));
        match (&error, case.error.kind.as_str()) {
            (SpawnedJobDriveError::Stalled { steps }, "stalled") | (SpawnedJobDriveError::Overrun { steps }, "overrun") => assert_eq!(*steps, case.error.steps, "{} reported the wrong step count", case.id),
            _ => panic!("{} refused as {error:?}, fixture declares {}", case.id, case.error.kind),
        }
    }
}

/// 🐛️ The exact wgpu defect as a law: a host that admits the effect and never steps it produces NO
/// completion, loudly. Before this contract the browser simply dropped the effect and nothing — not
/// a warning with a job id, not a fault — said the interaction would never apply.
#[test]
fn a_host_that_never_steps_the_job_stalls_loudly() {
    let error = spawned_job_completion(&[]).expect_err("an unpumped job must not complete");
    assert_eq!(error, SpawnedJobDriveError::Stalled { steps: 0 });
    assert!(error.to_string().contains("terminal step"), "{error}");
}

/// 📨️ The completion becomes exactly the `Event::JobCompleted` the guest's reactor correlates on,
/// cross-checked against `serde_json` — a third-party encoder that shares no line with this module.
#[test]
fn a_completion_becomes_the_declared_job_completed_event() {
    let fixture = fixture();
    assert_eq!(fixture.completed_event.kind, "job-completed");
    assert!(fixture.completed_event.cases.len() >= 2, "both outcome arms must be driven");
    for case in &fixture.completed_event.cases {
        let job: u64 = case.job.parse().expect("job is a u64 decimal");
        let completion = SpawnedJobCompletion { steps: case.completion.steps, outcome: outcome_of(&case.completion.outcome) };
        let event = spawned_job_completed_event(job, &completion);
        assert_eq!(serde_json::to_value(&event).expect("event serializes"), case.rust_event_json, "{} serialized to the wrong event", case.id);
        let Event::JobCompleted { job: correlated, result } = &event else { panic!("{} must be a JobCompleted event", case.id) };
        assert_eq!(*correlated, job, "{} must correlate on the job id alone", case.id);
        assert_eq!(*result, completion.outcome);
    }
}
