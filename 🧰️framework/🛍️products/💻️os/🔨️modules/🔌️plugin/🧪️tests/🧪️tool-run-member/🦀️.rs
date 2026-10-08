//! 🪆️ Runtime laws of the child-target tool run (design §12, §20.14, §20.15; `📓️s3-wires-report.md` §1.1) on a two-store
//! fixture — the toy parent and one owned `TestSnapshot` member in its `slot`: the provisional member ops compose the child
//! on read while both stores stay untouched, a paused run steps exactly one unit per step, an abort leaves zero trace, and a
//! finalize publishes ONE member edit carrying the run's `TransactionRef` — one editable history row, its state exactly the
//! fresh fold of the run's ops on the member base, one member undo removing it. Expectations: the `member` section of
//! `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json`.

use super::*;
use crate::test_app_mutation_fixture::SetSlotChildren;

store::space_members! {
    pub enum ToolRunMembers, ToolRunMembersOpen {
        Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation),
    }
}

type MemberApp = VcsArtifactApp<ToyRunApp, ToolRunMembers>;

//#region 🧸️ToyMemberJob
/// 🧸️ One unit appends `SetCount(base + unit)` and `SetLabel("unit-<unit>")` to the member, `base` being the member's count
/// in the composed read the job starts from; one unit per step, so progress spans driver turns and a single step is one unit.
pub(super) struct ToyMemberJob {
    writer: ToolRunTickWriter,
    base_count: i32,
    target: u32,
    done: u32,
    closing: bool,
}

/// 🏭️ The member job a request describes, continuing after the member ops the run already holds.
pub(super) fn toy_member_job(request: ToolRunJobRequest<'_, ToyRunApp>) -> Result<ToyMemberJob, Fault> {
    let member = &fixture()["member"];
    let base_count = request.children.typed_read::<TestSnapshot>(text(&member["slot"]), text(&member["childId"]))?.count;
    let done = (request.member_ops.len() / number(&fixture()["opsPerUnit"]) as usize) as u32;
    Ok(ToyMemberJob { writer: ToolRunTickWriter::with_provisional_base(request.identity, request.member_ops.len() as u32), base_count, target: toy_target(&request.config), done, closing: false })
}

impl semio_framework_job::InteractiveJob for ToyMemberJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if self.done >= self.target {
            return ToyRunJob::complete();
        }
        self.done += 1;
        cx.consume_fuel(1);
        self.writer.append_op(encoded(SetCount { value: self.base_count + self.done as i32 }.into())).expect("the member op fits the member cap");
        self.writer.append_op(encoded(SetLabel { value: format!("unit-{}", self.done) }.into())).expect("the member op fits the member cap");
        self.writer.append_entity(u64::from(self.done));
        self.writer.progress(ToolRunProgress {
            identity: self.writer.identity(),
            sequence: 0,
            state: ToolRunState::Running,
            stage: 0,
            completed: u64::from(self.done),
            total: Some(u64::from(self.target)),
            counters: Vec::new(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        });
        let tick = self.writer.finish().expect("a pending member tick");
        ToyRunJob::emit(cx, tick)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.closing {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Blocked
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}
//#endregion 🧸️ToyMemberJob

//#region 🧰️MemberHarness
fn member_fixture() -> Value {
    fixture()["member"].clone()
}

fn member_dialect() -> ArtifactDialect {
    let dialect = &member_fixture()["childDialect"];
    ArtifactDialect { artifact_kind: text(&dialect["artifactKind"]).into(), standard: text(&dialect["standard"]).into(), subset: text(&dialect["subset"]).into() }
}

fn member_key() -> (String, String) {
    let member = member_fixture();
    (text(&member["slot"]).to_string(), text(&member["childId"]).to_string())
}

/// 🧒️ The live owned member store at its default state, under the member owner catalog every product member adopts.
async fn member_store() -> ToolRunMembers {
    let mut envelope = store::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", text(&member_fixture()["childId"]), TestSnapshot::default(), None);
    envelope.dialect = Some(member_dialect());
    let mut child = ArtifactStore::new(envelope, protocol::ActorId(text(&fixture()["actor"]).into())).await.expect("the member store opens");
    child.install_document_store_owners_exact(<TestSnapshot as store::MemberStoreOwner<TestMutation>>::member_store_owners());
    ToolRunMembers::Child(Box::new(child))
}

/// 🏗️ The toy app over the member roster with the parent declaring and owning its one member, run target `target`.
async fn member_app(target: u64) -> MemberApp {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut app = artifact_app_laws::new_registered_app_with_members::<ToyRunApp, ToolRunMembers, _>(toy_manifest(), protocol::ActorId(text(&fixture()["actor"]).into())).await;
    app.config_store.dispatch(ArtifactCommand::Apply { mutations: vec![ChangeTestConfigSelection { selected: Some(target.to_string()) }.into()], transaction: None }).await.expect("the member target config applies");
    let (slot, child_id) = member_key();
    let declared = ArtifactRef { artifact_id: child_id.clone(), dialect: member_dialect() }.to_uri();
    app.store.dispatch(ArtifactCommand::Apply { mutations: vec![TestMutation::SetSlotChildren(SetSlotChildren { children: vec![declared] })], transaction: None }).await.expect("the parent declares its member");
    app.refresh_cache().await.expect("the parent view follows its declaration");
    app.register_child(slot, child_id, member_dialect(), member_store().await).await.expect("the owned member registers");
    app
}

fn member_child(app: &MemberApp) -> &ArtifactStore<TestSnapshot, TestMutation> {
    let ToolRunMembers::Child(child) = &app.children.get(&member_key()).expect("the live member").member;
    child
}

fn member_child_mut(app: &mut MemberApp) -> &mut ArtifactStore<TestSnapshot, TestMutation> {
    let ToolRunMembers::Child(child) = &mut app.children.get_mut(&member_key()).expect("the live member").member;
    child
}

/// 🔭️ The member as the render seams read it: the run's composed overlay while it holds ops, else the live member.
fn composed_member(app: &MemberApp) -> TestSnapshot {
    let (slot, child_id) = member_key();
    app.tool_runs.children_or(&app.child_content_root).typed_read::<TestSnapshot>(&slot, &child_id).expect("the composed member reads").clone()
}

/// 🧮️ The fresh fold of `ops` (the member's own wire) on `base` — the independent replay every member state must equal.
fn fresh_fold(base: &TestSnapshot, ops: &[Vec<u8>]) -> TestSnapshot {
    let mut state = base.clone();
    for bytes in ops {
        let op = <TestMutation as ::protocol::OpBinary>::decode_op(bytes).expect("a member op decodes");
        let outcome = <TestMutation as Mutation<TestSnapshot>>::diff(&op, &state);
        state = protocol::apply_diff(outcome.diff(), &state).expect("a member op applies");
    }
    state
}

async fn member_action(app: &mut MemberApp, action: &str, arguments: Vec<(String, DslValue)>) -> DslValue {
    app.handle_action(action, Some(&DslValue::Object(arguments)), &toy_meta()).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}")).output
}

async fn member_run_action(app: &mut MemberApp, action: &str) -> DslValue {
    let slot = app.tool_runs.slot().expect("a member run slot exists");
    member_action(app, action, vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(slot.generation.to_string()))]).await
}

async fn member_start(app: &mut MemberApp) {
    let output = member_action(app, "toolRunStart", vec![("toolId".into(), DslValue::String(text(&member_fixture()["toolId"]).into()))]).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "start spawns the member run job");
    assert_eq!(app.tool_runs.member(), Some((member_key().0.as_str(), member_key().1.as_str())), "the run targets the declared member");
}

async fn member_pump_until(app: &mut MemberApp, what: &str, done: impl Fn(&MemberApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
    }
    let reasons: Vec<u16> = app.tool_runs.steps().map(|steps| steps.iter().map(|step| step.reason).collect()).unwrap_or_default();
    let admission = app.admit_child_content_publication().err().map(|fault| fault.code.0);
    panic!("{what} never settled; state {:?}; member ops {}; ledger work {}; step reasons {reasons:?}; child-content admission refusal {admission:?}", app.tool_runs.state(), app.tool_runs.member_ops().len(), app.tool_runs.has_pending_work());
}

/// 🧾️ The applied history rows a tool transaction of the member tool keys.
async fn member_transaction_rows(app: &mut MemberApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    let tool = format!("{}#{}", ToyRunApp::APP_ID, text(&member_fixture()["toolId"]));
    let history = app.history_snapshot().await.expect("the member app projects its history");
    history.upserts.into_iter().filter(|row| row.applied && row.transaction.as_ref().is_some_and(|transaction| transaction.tool == tool)).collect()
}

fn member_close(app: &mut MemberApp) {
    artifact_app_laws::close_registered_fixture_app(app);
    assert!(app.tool_runs.terminal_is_empty(), "the tool run ledger retires every member-owned alias and read on close");
}
//#endregion 🧰️MemberHarness

/// ⚖️ LAW: a member run's ticks never touch either store; its ops are the member's own vocabulary held by the run, and the
/// composed read the render seams take shows exactly their fresh fold on the member base.
#[semio_framework_async_macros::async_test]
async fn member_run_ticks_compose_the_member_on_read_while_both_stores_stay_untouched() {
    let member = member_fixture();
    let units = number(&member["units"]);
    let mut app = member_app(units).await;
    let base = member_child(&app).snapshot_ref().clone();
    let (parent_generation, member_generation) = (app.store.generation(), member_child(&app).generation());
    member_start(&mut app).await;
    member_pump_until(&mut app, "the member run completes and its overlay shows every op", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(app.store.generation(), parent_generation, "member ticks never touch the parent store");
    assert_eq!(member_child(&app).generation(), member_generation, "member ticks never touch the member store");
    assert!(app.tool_runs.provisional().is_empty(), "a member run holds no parent op");
    assert_eq!(app.tool_runs.member_ops().len() as u64, units * number(&fixture()["opsPerUnit"]));
    assert_eq!(member_child(&app).snapshot_ref(), &base, "the live member is still the base");
    let composed = composed_member(&app);
    assert_eq!((composed.count as u64, composed.label.as_str()), (number(&member["countAfterFinalize"]), text(&member["labelAfterFinalize"])));
    assert_eq!(composed, fresh_fold(&base, app.tool_runs.member_ops()), "the composed read is the fresh fold of the run's ops");
    member_run_action(&mut app, "toolRunAbort").await;
    member_pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(composed_member(&app), base, "an aborted member run composes the live member again");
    member_close(&mut app);
}

/// ⚖️ LAW: a paused member run schedules nothing and every single step adds exactly one unit of member ops.
#[semio_framework_async_macros::async_test]
async fn member_run_pause_then_step_drives_exactly_one_unit_per_step() {
    let expected = &member_fixture()["pauseStep"];
    let mut app = member_app(number(&expected["target"])).await;
    member_start(&mut app).await;
    member_pump_until(&mut app, "member job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running)).await;
    assert_eq!(member_run_action(&mut app, "toolRunPause").await.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    for _ in 0..8 {
        app.advance_typed_operation_publication().await.expect("paused turn");
    }
    let mut held = app.tool_runs.member_ops().len() as u64;
    for _ in 0..number(&expected["steps"]) {
        assert_eq!(member_run_action(&mut app, "toolRunStep").await.get("toolRun").and_then(DslValue::as_str), Some("driveOneUnit"));
        member_pump_until(&mut app, "single member step settles", |app| !app.tool_runs.has_pending_work()).await;
        let after = app.tool_runs.member_ops().len() as u64;
        assert_eq!(after - held, number(&expected["opsPerStep"]), "one step is exactly one unit of member ops");
        held = after;
        for _ in 0..8 {
            app.advance_typed_operation_publication().await.expect("paused turn");
        }
        assert_eq!(app.tool_runs.member_ops().len() as u64, held, "a paused member run schedules nothing");
    }
    member_run_action(&mut app, "toolRunAbort").await;
    member_pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    member_close(&mut app);
}

/// ⚖️ LAW: an aborted member run leaves zero trace — both store generations and edit logs, the command log, the history and
/// the member state are exactly what they were before the run, and the run holds no op.
#[semio_framework_async_macros::async_test]
async fn member_run_abort_leaves_both_stores_the_command_log_and_the_history_untouched() {
    let expected = &member_fixture()["abort"];
    let mut app = member_app(number(&expected["target"])).await;
    app.refresh_cache().await.expect("backfill the command log before the invariant capture");
    let base = member_child(&app).snapshot_ref().clone();
    let (parent_generation, parent_edits, commands) = (app.store.generation(), app.store.envelope().vcs.edits.len(), app.command_log.len());
    let (member_generation, member_edits) = (member_child(&app).generation(), member_child(&app).envelope().vcs.edits.len());
    let rows = member_transaction_rows(&mut app).await.len();
    member_start(&mut app).await;
    member_pump_until(&mut app, "first member unit", |app| app.tool_runs.member_ops().len() as u64 >= number(&expected["unitsBeforeAbort"]) * number(&fixture()["opsPerUnit"])).await;
    assert_eq!(member_run_action(&mut app, "toolRunAbort").await.get("toolRun").and_then(DslValue::as_str), Some("closeJob"));
    member_pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    assert_eq!((app.store.generation(), app.store.envelope().vcs.edits.len(), app.command_log.len()), (parent_generation, parent_edits, commands), "abort: parent store and command log");
    assert_eq!((member_child(&app).generation(), member_child(&app).envelope().vcs.edits.len()), (member_generation, member_edits), "abort: member store");
    assert_eq!(member_child(&app).snapshot_ref(), &base, "abort: member state");
    assert_eq!(member_transaction_rows(&mut app).await.len(), rows, "abort: no history row");
    assert!(app.tool_runs.member_ops().is_empty(), "abort retires every member op");
    member_close(&mut app);
}

/// ⚖️ LAW: a member run's finalize publishes ONE member edit — every op stamped with the run's `TransactionRef` (tool
/// `<appId>#<toolId>`) and group id, the parent untouched — whose state is exactly the fresh fold of the run's ops on the
/// member base; its history row is the tool's own label in every locale with editable member mutations, and one member undo
/// removes the whole run.
#[semio_framework_async_macros::async_test]
async fn member_run_finalize_is_one_member_edit_carrying_the_run_transaction_and_one_undo_removes_it() {
    let member = member_fixture();
    let mut app = member_app(number(&member["units"])).await;
    let base = member_child(&app).snapshot_ref().clone();
    let parent_edits = app.store.envelope().vcs.edits.len();
    let member_edits = member_child(&app).envelope().vcs.edits.len();
    let rows = member_transaction_rows(&mut app).await.len();
    member_start(&mut app).await;
    member_pump_until(&mut app, "member run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work()).await;
    let ops = app.tool_runs.member_ops().to_vec();
    assert_eq!(member_run_action(&mut app, "toolRunFinalize").await.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    member_pump_until(&mut app, "member finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work()).await;
    assert_eq!((app.store.envelope().vcs.edits.len() - parent_edits) as u64, number(&member["parentEditsAdded"]), "the parent publishes nothing");
    assert_eq!((member_child(&app).envelope().vcs.edits.len() - member_edits) as u64, number(&member["memberEditsAdded"]), "ONE member edit");
    let edit = member_child(&app).envelope().vcs.edits.last().expect("the finalized member edit");
    assert_eq!(edit.forwards.len(), ops.len(), "every member op lands in the one edit");
    let transaction = edit.mutation_meta.first().and_then(|meta| meta.transaction.clone()).expect("the member edit is a tool transaction");
    assert!(edit.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&transaction) && meta.group_id.as_deref() == Some(text(&member["groupId"]))), "every member op carries the run's ref and group id");
    assert_eq!(transaction.tool, format!("{}#{}", ToyRunApp::APP_ID, text(&member["toolId"])));
    let committed = member_child(&app).snapshot_ref().clone();
    assert_eq!(committed, fresh_fold(&base, &ops), "the published member is the fresh fold of the run's ops");
    assert_eq!((committed.count as u64, committed.label.as_str()), (number(&member["countAfterFinalize"]), text(&member["labelAfterFinalize"])));
    assert!(app.tool_runs.member_ops().is_empty(), "the finalized run releases its member ops");
    let rows_after = member_transaction_rows(&mut app).await;
    assert_eq!((rows_after.len() - rows) as u64, number(&member["transactionRowsAdded"]), "ONE history row");
    let row = rows_after.iter().find(|row| row.transaction.as_ref().is_some_and(|row| row.id == transaction.id)).expect("the run's history row");
    assert_eq!(row.op_count as usize, ops.len());
    assert!(!row.mutations.is_empty() && row.mutations.iter().all(|mutation| mutation.editable), "the member mutations are editable in history");
    let label = &member["toolLabel"];
    assert_eq!((row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string()), (text(&label["en"]).to_string(), text(&label["de"]).to_string()));
    member_child_mut(&mut app).dispatch(ArtifactCommand::Undo).await.expect("undo the finalized member edit");
    let undone = member_child(&app).snapshot_ref().clone();
    assert_eq!((undone.count as u64, undone.label.as_str()), (number(&member["countAfterUndo"]), text(&member["labelAfterUndo"])), "one member undo removes the whole run");
    member_close(&mut app);
}

/// ⚖️ LAW (§20.14): a member run holds at most `TOOL_RUN_MEMBER_OPS_MAX` member ops — the ceiling that bounds its one-turn
/// finalize — and reports the cap as the localized provisional-cap step carrying that ceiling.
#[semio_framework_async_macros::async_test]
async fn member_run_holds_at_most_the_member_ceiling_and_reports_the_cap() {
    let expected = &member_fixture()["capped"];
    assert_eq!(u64::from(semio_framework_tool_run::TOOL_RUN_MEMBER_OPS_MAX), number(&expected["memberOpsMax"]));
    let mut app = member_app(number(&expected["target"])).await;
    member_start(&mut app).await;
    member_pump_until(&mut app, "the capped member run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work()).await;
    assert_eq!(app.tool_runs.member_ops().len() as u64, number(&expected["memberOpsMax"]), "the run holds exactly the member ceiling");
    let capped = app.tool_runs.steps().expect("the run's steps").iter().any(|step| step.reason == semio_framework_tool_run::TOOL_RUN_REASON_PROVISIONAL_CAP && step.args.first() == Some(&semio_framework_tool_run::ToolRunStepArg::Unsigned(number(&expected["memberOpsMax"]))));
    assert!(capped, "the cap is reported with the member ceiling");
    member_run_action(&mut app, "toolRunAbort").await;
    member_pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work()).await;
    member_close(&mut app);
}
