use super::*;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};

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
    grant: Grant,
    expected: Expected,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Command { Increment, CoalescedIncrement, IncrementAndNotify }

#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Completion { Empty, PendingOwner, PendingExternal }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Grant { items: usize, bytes: GrantBytes }

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum GrantBytes { Zero, CommandMinusOne, ExactCommand }

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expected {
    step: String,
    command: String,
    completion: String,
    released_items: usize,
    released_bytes: ReleasedBytes,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ReleasedBytes { Zero, CommandLayout }
//#endregion 🧬️CommandCloseVectors

//#region 🧪️CommandCloseLaws
fn check(id: &str) {
    let document: Cases = serde_json::from_str(include_str!("🔣️.json")).expect("command-close neutral vectors");
    assert_eq!(document.schema_version, 1);
    assert_eq!(document.layout_policy, "command-value-layout-excludes-allocator-overhead");
    assert_eq!(document.cases.len(), 6);
    let expected_ids = ["before-begin-close", "zero-items", "zero-bytes", "short-bytes", "exact-external-completion", "exact-pending-completion"].into_iter().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(document.cases.iter().map(|case| case.id.as_str()).collect::<std::collections::BTreeSet<_>>(), expected_ids);
    assert_eq!(document.cases.iter().filter(|case| case.id == id).count(), 1);
    let case = document.cases.into_iter().find(|case| case.id == id).expect("exact named command-close case");
    let command = Box::new(match case.command {
        Command::Increment => TxnCommand::Increment,
        Command::CoalescedIncrement => TxnCommand::CoalescedIncrement,
        Command::IncrementAndNotify => TxnCommand::IncrementAndNotify,
    });
    let command_bytes = std::mem::size_of_val(command.as_ref());
    assert!(command_bytes > 0, "the three-variant command has a nonzero layout");
    let command_identity = std::ptr::from_ref(command.as_ref());
    let completion = ArtifactToolCompletion::<TxnApp>::new();
    if case.completion != Completion::Empty {
        completion.complete(Ok(Emit::mutations(vec![SetTransactionCount { value: 7 }.into()])), crate::app::EphemeralEmit::default()).expect("one real pending mutation output");
    }
    let external = (case.completion == Completion::PendingExternal).then(|| completion.clone());
    let grant_bytes = match case.grant.bytes {
        GrantBytes::Zero => 0,
        GrantBytes::CommandMinusOne => command_bytes - 1,
        GrantBytes::ExactCommand => command_bytes,
    };
    let mut job = TxnFixtureJob { command: Some(command), completion: Some(completion), count: 0, closing: false };
    if case.begin_close { job.begin_close(); }
    let (step, released_items, released_bytes) = match job.close_step(case.grant.items, grant_bytes) {
        InteractiveJobCloseStep::Blocked => ("blocked", 0, 0),
        InteractiveJobCloseStep::Pending { released_items, released_bytes } => ("pending", released_items, released_bytes),
        InteractiveJobCloseStep::Complete => ("complete", 0, 0),
    };
    let command_after = job.command.as_deref().map(std::ptr::from_ref);
    let completion_retained = job.completion.is_some();
    let external_still_shared = external.as_ref().map(ArtifactToolCompletion::has_mounted_consumer);
    let received = match external.as_ref().or(job.completion.as_ref()) {
        Some(consumer) => consumer.take_emit().expect("real completion consumer drains test output"),
        None => None,
    };
    eprintln!("[DEBUG] txn-command-close id={id} commandBytes={command_bytes} grantItems={} grantBytes={grant_bytes} step={step} releasedItems={released_items} releasedBytes={released_bytes}", case.grant.items);
    assert_eq!(step, case.expected.step, "{id}");
    assert_eq!(released_items, case.expected.released_items, "{id}");
    assert_eq!(released_bytes, match case.expected.released_bytes { ReleasedBytes::Zero => 0, ReleasedBytes::CommandLayout => command_bytes }, "{id}");
    assert_eq!(if command_after.is_some() { "retained" } else { "released" }, case.expected.command, "{id}");
    if command_after.is_some() { assert_eq!(command_after, Some(command_identity), "{id}: exact original Box must remain"); }
    assert!(completion_retained && case.expected.completion == "retained", "{id}: command close must not take completion");
    if case.completion == Completion::Empty {
        assert!(received.is_none(), "{id}: empty completion must remain empty");
    } else {
        let (emit, _) = received.expect("pending output remains reachable through its exact completion consumer");
        assert_eq!(emit.expect("pending mutation output").artifact_mutations, vec![TxnMutation::from(SetTransactionCount { value: 7 })], "{id}");
    }
    if external.is_some() { assert_eq!(external_still_shared, Some(true), "{id}: external completion clone retains its exact shared cell"); }
}

#[test]
fn txn_command_close_requires_begin_close() { check("before-begin-close"); }
#[test]
fn txn_command_close_zero_items_preserves_owners() { check("zero-items"); }
#[test]
fn txn_command_close_zero_bytes_preserves_owners() { check("zero-bytes"); }
#[test]
fn txn_command_close_short_bytes_preserves_owners() { check("short-bytes"); }
#[test]
fn txn_command_close_exact_grant_retains_external_completion() { check("exact-external-completion"); }
#[test]
fn txn_command_close_exact_grant_retains_pending_completion() { check("exact-pending-completion"); }
//#endregion 🧪️CommandCloseLaws
