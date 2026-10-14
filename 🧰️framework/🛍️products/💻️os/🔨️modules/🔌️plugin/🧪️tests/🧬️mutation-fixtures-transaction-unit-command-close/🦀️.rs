use super::*;
use crate::app_testing::mutation_fixture::job_close::{job_quote, retire_completion};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
use semio_framework_value::retained_clone::RetainedCloneGrant;

//#region 🧬️CommandCloseVectors
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Cases {
    schema_version: u32,
    layout_policy: String,
    caller_grant: [usize; 5],
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
fn turn(job:&mut TxnFixtureJob,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,released:&mut usize) -> InteractiveJobCloseStep {
    let step=cx.with_retained_loan(grant,|same|job.close_step(same)).expect("original command close");
    assert!(step.progress().fits(grant), "a close receipt never exceeds its grant on any axis");
    *released += step.progress().released_bytes;
    step
}

fn label(step: InteractiveJobCloseStep) -> &'static str {
    match step {
        InteractiveJobCloseStep::Blocked => "blocked",
        InteractiveJobCloseStep::Pending { .. } => "pending",
        InteractiveJobCloseStep::Complete { .. } => "complete",
    }
}

fn drain(job:&mut TxnFixtureJob,cx:&mut semio_framework_job::StepContext<'_>,original:RetainedCloneGrant) {
    job.begin_close();
    for _ in 0..256 {
        if job.terminal_is_empty() {
            return;
        }
        let mut released = 0;
        turn(job,cx,original,&mut released);
    }
    panic!("the fixture job did not reach its terminal empty shell");
}

fn check(id:&str){
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=original_caller.context();let recipient=cx.original_recipient_identity();
    let document: Cases = serde_json::from_str(include_str!("../🧫️fixtures/🧾️transaction-command-close/🔣️.json")).expect("command-close neutral vectors");
    assert_eq!(document.schema_version, 2);
    assert_eq!(document.layout_policy, "command-value-layout-excludes-allocator-overhead");
    assert_eq!(document.cases.len(), 6);
    let expected_ids = ["before-begin-close", "zero-items", "short-depth", "short-release", "exact-external-completion", "exact-pending-completion"].into_iter().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(document.cases.iter().map(|case| case.id.as_str()).collect::<std::collections::BTreeSet<_>>(), expected_ids);
    assert_eq!(document.cases.iter().filter(|case| case.id == id).count(), 1);
    let [maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes, maximum_depth] = document.caller_grant;
    let original = RetainedCloneGrant { maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes, maximum_depth };
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
        completion.complete(Ok(Emit::mutations(vec![SetTransactionCount { value: 7 }.into()])), crate::app_emission::EphemeralEmit::default()).expect("one real pending mutation output");
    }
    let external = (case.completion == Completion::PendingExternal).then(|| completion.clone());
    let mut job = TxnFixtureJob { owners: crate::app_testing::mutation_fixture::job_close::FixtureJobOwners::new(command, completion), count: 0, page: 0 };
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
        step = Some(turn(&mut job,&mut cx,original,&mut released));
    }
    if let Some(probe) = case.probe {
        let demand = job_quote(&job);
        let mut denied = RetainedCloneGrant { maximum_items: probe.items, ..original };
        match probe.axis {
            Axis::ExactQuote => {}
            Axis::DepthMinusOne => denied.maximum_depth = demand.depth - 1,
            Axis::ReleaseMinusOne => denied.maximum_release_bytes = demand.release_bytes - 1,
        }
        let before = job_quote(&job);
        let probed = turn(&mut job,&mut cx,denied,&mut released);
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
    drain(&mut job,&mut cx,original);
    if let Some(external) = external {
        retire_completion(external,&mut cx,original);
    }
    assert_eq!(cx.original_recipient_identity(),recipient);let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).unwrap();
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
fn txn_command_close_short_depth_yields_and_preserves_owners() {
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

#[test]
fn original_fixture_completion_same_source_denials_whole_rejection_and_paid_close(){
    use crate::app_testing::mutation_fixture::job_close::FixtureJobOwners;
    use semio_framework_job::{close_original_retained_step,OriginalTurnFailure,RetainedCloneProgress};
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️mutation-fixtures-job-close/🧫️fixtures/🔣️.json")).unwrap();let loan=fixture["loan"].as_array().unwrap();let policy=RetainedCloneGrant{maximum_items:loan[0].as_u64().unwrap()as usize,maximum_copy_bytes:loan[1].as_u64().unwrap()as usize,maximum_capacity_bytes:loan[2].as_u64().unwrap()as usize,maximum_release_bytes:loan[3].as_u64().unwrap()as usize,maximum_depth:loan[4].as_u64().unwrap()as usize};
    for row in fixture["cases"].as_array().unwrap(){
        let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();let mut owners=FixtureJobOwners::<TxnApp>::new(Box::new(TxnCommand::Increment),ArtifactToolCompletion::new());
        assert!(cx.with_retained_loan(policy,|same|owners.prepare_emission(same,std::mem::size_of::<i32>(),0,||Ok(Emit::mutations(vec![SetTransactionCount{value:7}.into()])))).unwrap());
        let pending_pointer=owners.pending_emission().unwrap().0.as_ref().unwrap().artifact_mutations.as_ptr();
        if row["occupied"].as_bool().unwrap(){owners.completion.as_ref().unwrap().complete(Ok(Emit::default()),Default::default()).unwrap()}
        let mut busy_witness=Some(owners.completion.as_ref().unwrap().clone());let mut busy_active=None;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap();let demand=FixtureJobOwners::<TxnApp>::completion_demand();
        for denied in [RetainedCloneGrant{maximum_items:0,..policy},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..policy},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..policy},RetainedCloneGrant{maximum_depth:demand.depth-1,..policy}]{
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|owners.complete(same)));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(owners.pending_emission().unwrap().0.as_ref().unwrap().artifact_mutations.as_ptr(),pending_pointer);
        }
        let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(policy,|same|if row["busy"].as_bool().unwrap(){busy_witness.as_ref().unwrap().with_busy_test_lock(||owners.complete(same))}else{owners.complete(same)}));
        let after=cx.retained_progress();let actual=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};assert_eq!((heap.requested_bytes,heap.released_bytes),(actual.retained_capacity_bytes,actual.released_bytes));assert_eq!(cx.fuel_remaining(),fuel-1);
        let mut failure:Option<OriginalTurnFailure>=None;match result{Ok(true)=>{assert!(row["faultCode"].is_null());assert!(owners.pending_is_empty());assert!(owners.completion_is_delivered())},Err(original)=>{let fault=original.primary_fault().unwrap();assert_eq!(fault.code.0,row["faultCode"].as_str().unwrap());assert_eq!(fault.retained_progress(),actual);assert_eq!(owners.pending_emission().unwrap().0.as_ref().unwrap().artifact_mutations.as_ptr(),pending_pointer);failure=Some(original)},other=>panic!("original completion did not hand off: {other:?}")}
        let mut emitted=owners.completion.as_ref().unwrap().take_emit().unwrap();let mut emitted_active=None;let mut failure_active:Option<ControlledRetirement<OriginalTurnFailure>>=None;cx.original_cancel_token().cancel_now();owners.begin_close();
        for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            if owners.terminal_is_empty()&&emitted.is_none()&&emitted_active.is_none()&&failure.is_none()&&failure_active.is_none()&&busy_witness.is_none()&&busy_active.is_none(){break}
            let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(policy,|same|{
                if emitted.is_some()||emitted_active.is_some(){let demand=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&emitted,&emitted_active,policy.maximum_copy_bytes)?;return close_original_retained_step(same,demand,|grant|semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut emitted,&mut emitted_active,grant))}
                if failure.is_some()||failure_active.is_some(){let demand=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&failure,&failure_active,policy.maximum_copy_bytes)?;return close_original_retained_step(same,demand,|grant|semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut failure,&mut failure_active,grant))}
                if busy_witness.is_some()||busy_active.is_some(){let demand=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&busy_witness,&busy_active,policy.maximum_copy_bytes)?;return close_original_retained_step(same,demand,|grant|semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut busy_witness,&mut busy_active,grant))}
                owners.close_step(same)
            }));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(policy));
        }
        assert!(owners.terminal_is_empty()&&emitted.is_none()&&emitted_active.is_none()&&failure.is_none()&&failure_active.is_none()&&busy_witness.is_none()&&busy_active.is_none());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((owners,emitted,emitted_active,failure,failure_active,busy_witness,busy_active)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.original_recipient_identity(),recipient);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();eprintln!("[DEBUG] original fixture completion case={} samePendingSource=true wholeNativeFault=true sameRecipient=true rejectedSystem0=true paidClose=true terminalDrop0=true",row["id"]);
    }
}
