use super::*;
use crate::app::mutation_fixture::job_close::{job_grant as grant, job_quote, retire_completion};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
use semio_framework_value::retained_clone::RetainedCloneGrant;

//#region 🧬️CommandCloseVectors
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Cases {
    schema_version: u32,
    layout_policy: String,
    cases: Vec<Case>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Case {
    id: String,
    begin_close: bool,
    command: Command,
    completion: Completion,
    advance: Advance,
    probe: Option<Probe>,
    expected: Expected,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Command {
    Increment,
    StreamedIncrement,
    IncrementAndNotify,
}

#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Completion {
    Empty,
    PendingOwner,
    PendingExternal,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Advance {
    None,
    UntilReleaseQuote,
    UntilCommandReleased,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Probe {
    items: usize,
    axis: Axis,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Axis {
    ExactQuote,
    DepthMinusOne,
    ReleaseMinusOne,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    step: String,
    command: String,
    completion: String,
    released_bytes: ReleasedBytes,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ReleasedBytes {
    Zero,
    CommandLayoutAtLeast,
}
//#endregion 🧬️CommandCloseVectors

//#region 🧪️CommandCloseLaws
fn turn(job: &mut TxnFixtureJob, grant: RetainedCloneGrant, released: &mut usize) -> InteractiveJobCloseStep {
    let step = job.close_step(grant);
    assert!(step.progress().fits(grant), "a close receipt never exceeds its grant on any axis");
    *released += step.progress().released_bytes;
    step
}

fn label(step: InteractiveJobCloseStep) -> &'static str {
    match step {
        InteractiveJobCloseStep::Blocked => "blocked",
        InteractiveJobCloseStep::Pending { .. } => "pending",
        InteractiveJobCloseStep::Complete { .. } => "complete",
        InteractiveJobCloseStep::Refused(_) => "refused",
    }
}

fn drain(job: &mut TxnFixtureJob) {
    job.begin_close();
    for _ in 0..256 {
        if job.terminal_is_empty() {
            return;
        }
        let mut released = 0;
        let demand = job_quote(job);
        turn(job, grant(1, demand), &mut released);
    }
    panic!("the fixture job did not reach its terminal empty shell");
}

fn check(id: &str) {
    let document: Cases = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️transaction-command-close/🔣️.json")).expect("command-close neutral vectors");
    assert_eq!(document.schema_version, 2);
    assert_eq!(document.layout_policy, "command-value-layout-excludes-allocator-overhead");
    assert_eq!(document.cases.len(), 6);
    let expected_ids = ["before-begin-close", "zero-items", "short-depth", "short-release", "exact-external-completion", "exact-pending-completion"].into_iter().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(document.cases.iter().map(|case| case.id.as_str()).collect::<std::collections::BTreeSet<_>>(), expected_ids);
    assert_eq!(document.cases.iter().filter(|case| case.id == id).count(), 1);
    let case = document.cases.into_iter().find(|case| case.id == id).expect("exact named command-close case");
    let command = Box::new(match case.command {
        Command::Increment => TxnCommand::Increment,
        Command::StreamedIncrement => TxnCommand::StreamedIncrement,
        Command::IncrementAndNotify => TxnCommand::IncrementAndNotify,
    });
    let command_bytes = size_of_val(command.as_ref());
    assert!(command_bytes > 0, "the three-variant command has a nonzero layout");
    let command_identity = std::ptr::from_ref(command.as_ref());
    let completion = ArtifactToolCompletion::<TxnApp>::new();
    if case.completion != Completion::Empty {
        completion.complete(Ok(Emit::mutations(vec![SetTransactionCount { value: 7 }.into()])), crate::app::EphemeralEmit::default()).expect("one real pending mutation output");
    }
    let external = (case.completion == Completion::PendingExternal).then(|| completion.clone());
    let mut job = TxnFixtureJob { owners: crate::app::mutation_fixture::job_close::FixtureJobOwners::new(command, completion), count: 0, page: 0 };
    if case.begin_close {
        job.begin_close();
    }
    let mut released = 0;
    let mut step = None;
    for _ in 0..64 {
        let advanced = match case.advance {
            Advance::None => false,
            Advance::UntilReleaseQuote => job_quote(&job).release_bytes == 0,
            Advance::UntilCommandReleased => !job.owners.command_is_released(),
        };
        if !advanced {
            break;
        }
        let demand = job_quote(&job);
        step = Some(turn(&mut job, grant(1, demand), &mut released));
    }
    if let Some(probe) = case.probe {
        let mut demand = job_quote(&job);
        match probe.axis {
            Axis::ExactQuote => {}
            Axis::DepthMinusOne => demand.depth -= 1,
            Axis::ReleaseMinusOne => demand.release_bytes -= 1,
        }
        let before = job_quote(&job);
        let probed = turn(&mut job, grant(probe.items, demand), &mut released);
        assert_eq!(probed.progress(), Default::default(), "{id}: an unfunded, refused or blocked probe moves nothing");
        step = Some(probed);
        assert_eq!(job_quote(&job), before, "{id}: an unfunded probe leaves the exact quote unchanged");
    }
    let command_state = if job.owners.command.is_some() {
        "retained"
    } else if job.owners.command_is_released() {
        "released"
    } else {
        "framed"
    };
    assert_eq!(label(step.expect("every case takes at least one close turn")), case.expected.step, "{id}");
    assert_eq!(
        released == 0,
        matches!(case.expected.released_bytes, ReleasedBytes::Zero),
        "{id}"
    );
    if let ReleasedBytes::CommandLayoutAtLeast = case.expected.released_bytes {
        assert!(released >= command_bytes, "{id}: the command box layout is released on the release axis");
    }
    assert_eq!(command_state, case.expected.command, "{id}");
    if command_state == "retained" {
        assert_eq!(job.owners.command.as_deref().map(std::ptr::from_ref), Some(command_identity), "{id}: exact original Box must remain");
    }
    assert!(job.owners.completion.is_some() && case.expected.completion == "retained", "{id}: command close must not take completion");
    let external_still_shared = external.as_ref().map(ArtifactToolCompletion::has_mounted_consumer);
    let received = match external.as_ref().or(job.owners.completion.as_ref()) {
        Some(consumer) => consumer.take_emit().expect("real completion consumer drains test output"),
        None => None,
    };
    if case.completion == Completion::Empty {
        assert!(received.is_none(), "{id}: empty completion must remain empty");
    } else {
        let (emit, _) = received.expect("pending output remains reachable through its exact completion consumer");
        assert_eq!(emit.expect("pending mutation output").artifact_mutations, vec![TxnMutation::from(SetTransactionCount { value: 7 })], "{id}");
    }
    if external.is_some() {
        assert_eq!(external_still_shared, Some(true), "{id}: external completion clone retains its exact shared cell");
    }
    drain(&mut job);
    if let Some(external) = external {
        retire_completion(external);
    }
}

#[test]
fn txn_command_close_requires_begin_close() {
    check("before-begin-close");
}
#[test]
fn txn_command_close_zero_items_preserves_owners() {
    check("zero-items");
}
#[test]
fn txn_command_close_short_depth_is_refused_and_preserves_owners() {
    check("short-depth");
}
#[test]
fn txn_command_close_short_release_preserves_the_framed_command() {
    check("short-release");
}
#[test]
fn txn_command_close_exact_grant_retains_external_completion() {
    check("exact-external-completion");
}
#[test]
fn txn_command_close_exact_grant_retains_pending_completion() {
    check("exact-pending-completion");
}
//#endregion 🧪️CommandCloseLaws
