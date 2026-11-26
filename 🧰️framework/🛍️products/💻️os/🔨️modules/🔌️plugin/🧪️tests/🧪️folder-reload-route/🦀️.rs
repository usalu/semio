//! 📁️ The folder reload route (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, e2e R2-2 and R2-4, goal gap 3), driven by the
//! language-agnostic fixture `🧫️fixtures/🧫️folder-reload-route/🔣️.json` whose expectations the TypeScript twin (`🟦️.ts`
//! beside this file) derives with its own fold. It walks what a host does with a document bound to a local folder, on the
//! program's side of the channel: the document port is bound hot (`bindDocumentPort`), every published batch must name the
//! identity `ReadDocumentIdentity` answers (the folder actor refuses any other as `local.backbone-scope-mismatch`), every
//! published batch makes the host save `ReadDocumentArchive` as the folder's content, and after a detach a fresh program
//! restores that content through the kernel's own stepped load driver (`DocumentArchiveLoadHost`) and re-reads its history.
//! A read-back of the document a program already shows is a MERGE (design §22.22, live fault F4): the two-programs law
//! walks two programs on one folder through an open history edit.
//! A child of the history-edit session laws: it drives their toy app and their verbs. The folder actor itself (admission,
//! own-write echo, read-back) is the store layer's law (`🏪️store/🔄️sync`, `💻️os/🧪️tests/🧪️folder-archive-restore`).
#[cfg(test)]
use semio_plugin_ownership::bounded_ownership::plugin_demand_grant;

use semio_framework::kernel::{ActorId, InvocationResult};
use semio_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_plugin_decl_app::{ArtifactApp, NaturalFileDecodeCursor};
use store::os_store::store_schema::ArtifactCommand;
use semio_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use pack::snapshot::ArtifactPack;
use store::{ArtifactStoreInitializationAuthority, MemberFactory, Mutation, SpaceMember};
use semio_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use std::collections::BTreeSet;
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
use semio_ui_contract::{Buildable, HasBase, HasChildren, *};
use semio_framework_value::DslValue;
use semio_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use std::future::Future;
use semio_plugin_window_kits::{LabelAxes, WindowKit};
#[cfg(test)]
use semio_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_plugin_program::PluginProgram;
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_plugin_tests_06::time_travel_support_1::{ToyApp, ToyHistoryApp, fixture, head, rejected, seed_op, text, toy_manifest};
use semio_plugin_window_config::WindowConfigOwner;
use semio_plugin_window_transient::window_transient::WindowTransientOwner;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_plugin_testing_laws::{artifact_app_laws, with_authoring_identity};
use semio_plugin_actor_host as host;
use semio_plugin_time_travel as time_travel;
use semio_framework::*;
use serde_json::Value;

const FOLDER_RELOAD_ROUTE_FIXTURE_JSON: &str = include_str!("../🧫️fixtures/🧫️folder-reload-route/🔣️.json");

#[doc(hidden)]
pub async fn pump_until(app: &mut ToyApp, what: &str, done: impl Fn(&ToyApp) -> bool, cx: &mut semio_framework_job::StepContext<'_>) {
    semio_plugin_tests_06::time_travel_support_1::pump_until(app, what, done, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await
}

#[doc(hidden)]
pub async fn verb(app: &mut ToyApp, fixture: &Value, action: &str, args: Vec<(String, DslValue)>, cx: &mut semio_framework_job::StepContext<'_>) -> InvocationResult {
    semio_plugin_tests_06::time_travel_support_1::verb(app, fixture, action, args, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await
}

#[doc(hidden)]
pub async fn run_step(app: &mut ToyApp, fixture: &Value, step: &Value, cx: &mut semio_framework_job::StepContext<'_>) -> Option<InvocationResult> {
    semio_plugin_tests_06::time_travel_support_1::run_step(app, fixture, step, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await
}

#[doc(hidden)]
pub async fn render_body(app: &mut ToyApp, cx: &mut semio_framework_job::StepContext<'_>) -> String {
    semio_plugin_tests_06::time_travel_support_1::render_body(app, cx).await
}

#[doc(hidden)]
pub fn close(app: &mut ToyApp, cx: &mut semio_framework_job::StepContext<'_>) {
    semio_plugin_tests_06::time_travel_support_1::close(app, cx)
}

/// 🛠️ One renewed host turn on which the program holds its cancellation issuer and pays exactly one maintenance step; answers the step's receipt and the effects the step posted on the turn's context.
async fn maintain_turn(app: &mut ToyApp, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(semio_framework_value::RetainedCloneProgress, semio_framework_value::RetainedCloneProgress), Fault> {
    let refused = |failure: &dyn std::fmt::Debug| Fault::from(format!("the load poll's turn was refused: {failure:?}"));
    let (grant, _) = cx.original_turn_ceiling();
    artifact_app_laws::fixture_renew_turn(cx, semio_plugin_testing_laws::OriginalFixtureCaller::turn_fuel(), grant).map_err(|failure| refused(&failure))?;
    artifact_app_laws::ensure_fixture_cancellation_issuer(app, cx).map_err(|failure| refused(&failure))?;
    cx.consume_fuel(1);
    let loan = cx.retained_grant();
    let before = cx.retained_progress();
    let step = PluginApp::maintenance_step(app, cx, loan).map_err(|failure| refused(&failure))?;
    let posted = semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, cx.retained_progress()).map_err(|failure| refused(&failure))?;
    Ok((step.progress().unwrap_or_default(), posted))
}

/// 🔁️ One host poll of a load on its original turn: the host renews the turn, the program pays one maintenance step and the load answers its status.
async fn poll_load(app: &mut ToyApp, operation: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
    maintain_turn(app, cx).await?;
    PluginApp::poll_document_archive_load(app, operation, &mut artifact_app_laws::fixture_identity(), cx).await
}

/// 🧫️ The shared route fixture.
fn route_fixture() -> Value {
    serde_json::from_str(FOLDER_RELOAD_ROUTE_FIXTURE_JSON).expect("folder-reload-route fixture parses")
}

/// 📋️ Everything one run of the law found, reported together: a run costs a whole test build, so it names every departure.
#[derive(Default)]
struct RouteFindings(Vec<String>);

impl RouteFindings {
    /// ✅️ Records `finding` unless `holds`.
    fn holds(&mut self, holds: bool, finding: impl FnOnce() -> String) {
        if !holds {
            self.0.push(finding());
        }
    }

    /// 🟰️ Records a departure of `actual` from `expected`.
    pub fn same<T: PartialEq + std::fmt::Debug>(&mut self, actual: &T, expected: &T, what: &str) {
        self.holds(actual == expected, || format!("{what}: {actual:?}, expected {expected:?}"));
    }
}

/// 🔹️ One mutation row as the history shows it, in every locale.
#[derive(Debug, PartialEq)]
struct RouteMutation {
    pub id: String,
    pub label: (String, String),
    pub superseded: bool,
    pub withdrawn: bool,
    pub editable: bool,
    pub worst: Option<semio_framework_diagnostic::Severity>,
    pub codes: Vec<String>,
}

/// 🗂️ One document row of the history: an edit or a finalized history edit, never a row of the running program alone.
#[derive(Debug, PartialEq)]
struct RouteRow {
    pub edit: Option<String>,
    pub transition: Option<String>,
    pub kind: String,
    pub label: (String, String),
    pub applied: bool,
    pub mutations: Vec<RouteMutation>,
}

/// 🧾️ What a host reads from a program to paint its document and history, less what a reload renews (row sequence numbers).
#[derive(Debug, PartialEq)]
struct RouteView {
    pub head: (i32, String),
    pub line: String,
    pub alternatives: Vec<String>,
    pub rows: Vec<RouteRow>,
}

/// 👁️ The [`RouteView`] of `app`, read the way a host does after a load: the history snapshot `ReadHistory` answers.
async fn route_view(app: &mut ToyApp) -> RouteView {
    let patch = PluginApp::history_snapshot(app).await.expect("the host reads the program's history");
    let both = |label: &LocalizedLabel| (label.resolve(Terminology::Native, Locale::En).to_string(), label.resolve(Terminology::Native, Locale::De).to_string());
    let rows = patch
        .upserts
        .iter()
        .filter(|entry| entry.edit_id.is_some() || entry.transition_id.is_some())
        .map(|entry| RouteRow {
            edit: entry.edit_id.clone(),
            transition: entry.transition_id.clone(),
            kind: entry.kind.clone(),
            label: both(&entry.label),
            applied: entry.applied,
            mutations: entry
                .mutations
                .iter()
                .map(|mutation| RouteMutation {
                    id: mutation.mutation_id.clone(),
                    label: both(&mutation.label),
                    superseded: mutation.superseded,
                    withdrawn: mutation.withdrawn,
                    editable: mutation.editable,
                    worst: mutation.worst,
                    codes: mutation.messages.iter().map(|message| message.code.clone()).collect(),
                })
                .collect(),
        })
        .collect();
    let envelope = app.stores.store.envelope();
    let named: Vec<String> = envelope.vcs.alternatives.iter().map(|alternative| alternative.name.clone()).collect();
    let alternatives = if named.is_empty() { named } else { std::iter::once(String::new()).chain(named).collect() };
    let line = patch.active_alternative_id.as_deref().and_then(|active| envelope.vcs.alternatives.iter().find(|alternative| alternative.id == active)).map(|alternative| alternative.name.clone()).unwrap_or_default();
    RouteView { head: head(app), line, alternatives, rows }
}

/// 🔎️ The mutation rows of `view`'s document edits in applied order.
fn applied_mutations(view: &RouteView) -> Vec<&RouteMutation> {
    view.rows.iter().rev().filter(|row| row.edit.is_some()).flat_map(|row| row.mutations.iter()).collect()
}

/// 🎯️ A fixture head.
fn expected_head(value: &Value) -> (i32, String) {
    (value["count"].as_i64().expect("head count") as i32, text(&value["label"]).to_string())
}

/// 🧍️ Holds `view` against what the fixture expects of any program: its head, its viewed alternative, the alternatives
/// it lists and the mutations that read superseded.
fn hold_program(findings: &mut RouteFindings, view: &RouteView, expected: &Value, who: &str) {
    findings.same(&view.head, &expected_head(&expected["head"]), &format!("{who}: the head"));
    findings.same(&view.line.as_str(), &text(&expected["line"]), &format!("{who}: the viewed alternative"));
    let alternatives: Vec<&str> = expected["alternatives"].as_array().expect("alternatives").iter().map(text).collect();
    findings.same(&view.alternatives.iter().map(String::as_str).collect::<Vec<_>>(), &alternatives, &format!("{who}: the alternatives listed"));
    let superseded: Vec<u64> = applied_mutations(view).iter().enumerate().filter(|(_, mutation)| mutation.superseded).map(|(position, _)| position as u64).collect();
    let expected_superseded: Vec<u64> = expected["superseded"].as_array().expect("superseded").iter().map(|position| position.as_u64().expect("position")).collect();
    findings.same(&superseded, &expected_superseded, &format!("{who}: the mutations that read superseded"));
}

/// 🖋️ Holds `view` against what the fixture expects of the author alone: the warnings by mutation, the history-edit rows
/// newest first in every locale, one row per document edit, and a label in every locale on every row.
fn hold_author(findings: &mut RouteFindings, view: &RouteView, expected: &Value, who: &str) {
    let mutations = applied_mutations(view);
    let warned: Vec<(u64, Vec<String>)> =
        mutations.iter().enumerate().filter(|(_, mutation)| mutation.worst.is_some_and(|worst| worst >= semio_framework_diagnostic::Severity::Warning)).map(|(position, mutation)| (position as u64, mutation.codes.clone())).collect();
    let expected_warned: Vec<(u64, Vec<String>)> = expected["warnings"].as_array().expect("warnings").iter().map(|warning| (warning["mutation"].as_u64().expect("mutation"), vec![text(&warning["code"]).to_string()])).collect();
    findings.same(&warned, &expected_warned, &format!("{who}: the warnings by mutation"));
    let history: Vec<(String, String)> = view.rows.iter().filter(|row| row.transition.is_some()).map(|row| row.label.clone()).collect();
    let expected_history: Vec<(String, String)> = expected["historyRows"].as_array().expect("historyRows").iter().map(|label| (text(&label["en"]).to_string(), text(&label["de"]).to_string())).collect();
    findings.same(&history, &expected_history, &format!("{who}: the history-edit rows, newest first"));
    findings.same(&(view.rows.iter().filter(|row| row.edit.is_some()).count() as u64), &expected["editRows"].as_u64().expect("editRows"), &format!("{who}: one row per document edit"));
    for row in &view.rows {
        findings.holds(!row.label.0.trim().is_empty() && !row.label.1.trim().is_empty(), || format!("{who}: a row lacks its label in a locale: {row:?}"));
    }
}

/// 🗄️ The folder a document is bound to, as its host keeps it: the newest archive the program's publications made the
/// host save, and how many of its batches the folder's actor admitted.
#[derive(Default)]
struct RouteFolder {
    pub archive: Option<Vec<u8>>,
    pub admitted: usize,
}

/// 🔌️ Binds `app`'s document port the way `bindDocumentPort` does — a hot attach, which announces nothing already in
/// the log — and answers the host's end of it.
async fn bind_document_port(app: &mut ToyApp, uri: &str) -> MemoryBackbone {
    let (guest, host) = MemoryBackbone::pair(uri, uri).await;
    PluginApp::attach_hot_backbone(app, store::os_store::backbone::Backbones::Memory(guest)).await.expect("the document port binds");
    host
}

/// 🧩️ A fresh registered program of the toy app authoring as `actor`.
async fn fresh_program(actor: &str) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into()), semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.stores.store.local_actor_id().0.as_str(), actor);
    app
}

/// ✏️ One document edit authored through the program's store, as a committed gesture leaves it.
async fn author_edit(app: &mut ToyApp, op: &Value) {
    semio_plugin_testing_laws::with_authoring_identity!(|identity| app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], transaction: None }, &mut identity).await).expect("the edit applies");
    app.render_ctx().refresh_cache().await.expect("the history backfills the edit");
}

/// ⏯️ Runs one fixture step: a document edit, or a history-edit verb through the session laws' own runner; a finalized
/// session is driven until it retired.
async fn route_step(findings: &mut RouteFindings, app: &mut ToyApp, fixture: &Value, step: &Value, cx: &mut semio_framework_job::StepContext<'_>) {
    if let Some(op) = step.get("edit") {
        return author_edit(app, op).await;
    }
    if let Some(result) = run_step(app, fixture, step, cx).await {
        findings.holds(rejected(&result).is_none(), || format!("step {step} was refused: {:?}", result.output));
    }
    if step.get("commit").is_some() {
        pump_until(app, "the finalize retires", |app| !app.time_travel.has_pending_work(), cx).await;
        app.render_ctx().refresh_cache().await.expect("the history backfills the finalized history edit");
    }
}

/// 📤️ What the host does with everything `app` published since the last call: each batch must name the bound document's
/// own identity — the folder's actor admits no other — and any admitted batch makes the host save the program's whole
/// archive as the folder's content. Answers the admitted batches.
#[doc(hidden)]
pub async fn publish(findings: &mut RouteFindings, app: &ToyApp, port: &mut MemoryBackbone, folder: &mut RouteFolder, who: &str) -> Vec<BackboneMessage> {
    let identity = PluginApp::document_identity(app).expect("a bound program holds a document");
    let mut batches = Vec::new();
    for message in port.receive().await.expect("the document port drains") {
        let admitted = match &message {
            BackboneMessage::Mutations { envelopes } => {
                for envelope in protocol::decode_envelopes(envelopes).expect("a published batch decodes") {
                    findings.holds(envelope.document_id.0 == identity, || format!("{who}: a batch names document {:?} while the folder is bound to {identity:?} (`local.backbone-scope-mismatch`)", envelope.document_id.0));
                }
                true
            }
            BackboneMessage::Ack { .. } => false,
            other => {
                findings.0.push(format!("{who}: a folder-bound program published {other:?}"));
                false
            }
        };
        if admitted {
            batches.push(message);
        }
    }
    if !batches.is_empty() {
        save_archive(findings, app, folder, who).await;
        folder.admitted += batches.len();
    }
    batches
}

/// 💾️ Saves `app`'s whole archive as the folder's content, as the host does after a published batch, for a folder found
/// empty at the bind and after a merge that left the program ahead.
async fn save_archive(findings: &mut RouteFindings, app: &ToyApp, folder: &mut RouteFolder, who: &str) {
    let archive = PluginApp::document_archive(app).await.expect("the program archives its document");
    let bytes = protocol::encode_document_archive_bytes(&archive).expect("the archive encodes");
    findings.holds(bytes.len() <= protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES, || format!("{who}: the archive ({} bytes) exceeds the folder's bound", bytes.len()));
    folder.archive = Some(bytes);
}

/// 🔀️ Delivers the author's admitted batches to the reader, as a shared transport would, and lets the reader adopt them.
#[doc(hidden)]
pub async fn deliver(batches: Vec<BackboneMessage>, port: &mut MemoryBackbone, to: &mut ToyApp, cx: &mut semio_framework_job::StepContext<'_>) {
    if batches.is_empty() {
        return;
    }
    for message in batches {
        port.send(message).await.expect("a batch reaches the reader");
    }
    to.tick_backbone(&mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("the reader ingests the batch");
    pump_until(to, "the reader adopts the batch", |app| app.stores.store.reprojection_progress().is_none(), cx).await;
    to.render_ctx().refresh_cache().await.expect("the reader's history backfills");
}

/// 🧭️ Drives one host load driver against `app`: the kernel's driver decides each command, and the program answers it as its
/// channel does (admit a load or a merge, poll to a terminal status, acknowledge). Answers the outcome, the polls it took
/// and the terminal status.
async fn drive_archive_host(app: &mut ToyApp, mut host: protocol::DocumentArchiveLoadHost, cx: &mut semio_framework_job::StepContext<'_>) -> (protocol::DocumentArchiveLoadOutcome, usize, Option<protocol::DocumentArchiveLoadStatus>) {
    let mut sequence = 0u64;
    let mut polls = 0usize;
    let mut terminal = None;
    loop {
        let (sent, command) = match host.step(|| {
            sequence += 1;
            sequence
        }) {
            protocol::DocumentArchiveLoadStep::Finished(outcome) => return (outcome, polls, terminal),
            protocol::DocumentArchiveLoadStep::Send { seq, command } => (seq, command),
        };
        let frame = match command {
            protocol::AppCommand::LoadDocumentArchive { seq, archive } => semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(app, seq, archive, cx).map(|()| protocol::AppFrame::Done { in_reply_to: seq }).map_err(|failure| failure.primary_fault().cloned().expect("the archive admission refuses with a fault")),
            protocol::AppCommand::MergeDocumentArchive { seq, archive } => semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_merge(app, seq, archive, cx).map(|()| protocol::AppFrame::Done { in_reply_to: seq }).map_err(|failure| failure.primary_fault().cloned().expect("the archive admission refuses with a fault")),
            protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } => {
                polls += 1;
                assert!(polls < 1_000_000, "the archive operation never reaches a terminal status");
                poll_load(app, operation, cx).await.map(|status| protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status })
            }
            protocol::AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation } => PluginApp::acknowledge_document_archive_load(app, operation).map(|()| protocol::AppFrame::Done { in_reply_to: seq }),
            other => panic!("the load driver sends admit, poll and acknowledge only, not {other:?}"),
        }
        .unwrap_or_else(|fault| panic!("the program refused the archive operation: {fault:?}"));
        if let Some(status) = host.answer(sent, &frame).expect("the driver takes the program's answer") {
            terminal = Some(status);
        }
    }
}

/// 🗃️ Loads `archive` into `app` the way every host does (the first archive read after an attach): answers the outcome and
/// the polls it took.
async fn load_through_the_host(app: &mut ToyApp, archive: protocol::DocumentArchivePack, cx: &mut semio_framework_job::StepContext<'_>) -> (protocol::DocumentArchiveLoadOutcome, usize) {
    let (outcome, polls, _) = drive_archive_host(app, protocol::DocumentArchiveLoadHost::new(archive), cx).await;
    (outcome, polls)
}

/// 🔀️ Merges the folder's archive into `app` the way a host does for a read-back of the document the program already shows,
/// and lets the program adopt what it took: answers the outcome, the events merged and the events `app` is ahead by.
async fn merge_through_the_host(app: &mut ToyApp, folder: &RouteFolder, cx: &mut semio_framework_job::StepContext<'_>) -> (protocol::DocumentArchiveLoadOutcome, u64, u64) {
    let archive = protocol::decode_document_archive_bytes(folder.archive.as_ref().expect("the folder holds an archive")).await.expect("the folder's archive decodes");
    let (outcome, _, terminal) = drive_archive_host(app, protocol::DocumentArchiveLoadHost::merging(archive), cx).await;
    pump_until(app, "the merged events are adopted", |app| app.stores.store.reprojection_progress().is_none(), cx).await;
    app.render_ctx().refresh_cache().await.expect("the history backfills the merged events");
    let terminal = terminal.expect("a merge is polled to its terminal status");
    (outcome, terminal.completed, terminal.ahead)
}

/// 🔁️ Detaches `app` from its folder and closes it, then walks the route a page reload and a re-attach take: a fresh
/// program holding its own example document binds its port (publishing nothing, so the folder
/// is not overwritten before it is read back), has the port retired for the load, restores the folder's archive, which restores the remembered document identity, and is
/// bound again. Answers the fresh program and the host's end of its port.
async fn reload_from_folder(findings: &mut RouteFindings, mut app: ToyApp, port: MemoryBackbone, folder: &RouteFolder, fixture: &Value, actor: &str, uri: &str, cx: &mut semio_framework_job::StepContext<'_>) -> (ToyApp, MemoryBackbone) {
    let identity = PluginApp::document_identity(&app).expect("a bound program holds a document");
    PluginApp::detach_backbone(&mut app).await.expect("the program detaches from its folder");
    drop(port);
    pump_until(&mut app, "the detached program settles", |app| !app.time_travel.has_pending_work(), cx).await;
    close(&mut app, cx);
    let mut fresh = fresh_program(actor).await;
    author_edit(&mut fresh, &fixture["example"]).await;
    let mut bound = bind_document_port(&mut fresh, uri).await;
    findings.holds(bound.receive().await.expect("the fresh port drains").is_empty(), || format!("{actor}: binding the port published the fresh program's own document, which would overwrite the folder before its read-back"));
    PluginApp::detach_backbone(&mut fresh).await.expect("the host retires the port for the load");
    drop(bound);
    let Some(bytes) = folder.archive.as_ref() else {
        findings.0.push(format!("{actor}: the folder holds no archive to read back"));
        return (fresh, MemoryBackbone::pair(uri, uri).await.1);
    };
    let archive = protocol::decode_document_archive_bytes(bytes).await.expect("the folder's archive decodes");
    let (outcome, polls) = load_through_the_host(&mut fresh, archive, cx).await;
    findings.same(&outcome, &protocol::DocumentArchiveLoadOutcome::Ready, &format!("{actor}: the folder's archive loads"));
    findings.holds(polls >= 1, || format!("{actor}: the load reached its terminal status without one poll"));
    findings.same(&PluginApp::document_identity(&fresh), &Some(identity), &format!("{actor}: the restored program holds the document identity its folder was remembered under"));
    let mut rebound = bind_document_port(&mut fresh, uri).await;
    findings.holds(rebound.receive().await.expect("the rebound port drains").is_empty(), || format!("{actor}: binding the port after the load published the restored document again"));
    (fresh, rebound)
}

/// ⚖️ LAW (e2e R2-2 and R2-4, goal clause 9 and gap 3, design §22.3): a document bound to a folder survives a reload with
/// its whole history. The author edits, edits the history and finalizes as an overwrite, edits again, edits the history
/// and finalizes as a new alternative; a reader on the main line receives every event and edits on its own. Every batch
/// either program publishes names its own document identity before and after the reload. Each program detaches; a fresh
/// one holding an example document restores its folder through the host's stepped load and reads back exactly the
/// document rows, history-edit rows, alternatives, warnings and viewed alternative it saved — the author on the new
/// alternative, the reader on the main line — and none of the example it displaced. The reloaded author edits on, and
/// that batch is admitted and saved too. One finalize leaves as one batch.
#[semio_framework_async_macros::async_test]
async fn a_folder_bound_document_reloads_with_its_whole_history_and_each_programs_viewed_alternative() {
    let mut caller = semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original route caller");
    let mut context = caller.context();
    let cx = &mut context;
    let fixture = route_fixture();
    let expected = &fixture["expected"];
    let (author_actor, reader_actor) = (text(&fixture["actor"]).to_string(), text(&fixture["reader"]).to_string());
    let (author_uri, reader_uri) = ("folder-reload-route-author", "folder-reload-route-reader");
    let mut findings = RouteFindings::default();
    let (mut author, mut reader) = (fresh_program(&author_actor).await, fresh_program(&reader_actor).await);
    let (mut author_port, mut reader_port) = (bind_document_port(&mut author, author_uri).await, bind_document_port(&mut reader, reader_uri).await);
    let (mut author_folder, mut reader_folder) = (RouteFolder::default(), RouteFolder::default());
    let mut batch_shape = Vec::new();
    for step in fixture["steps"].as_array().expect("steps") {
        route_step(&mut findings, &mut author, &fixture, step, cx).await;
        let batches = publish(&mut findings, &author, &mut author_port, &mut author_folder, "author").await;
        let publishes = step.get("edit").is_some() || step.get("commit").is_some();
        batch_shape.push((step.to_string(), batches.len(), usize::from(publishes)));
        deliver(batches, &mut reader_port, &mut reader, cx).await;
        publish(&mut findings, &reader, &mut reader_port, &mut reader_folder, "reader").await;
    }
    author_edit(&mut reader, &fixture["readerEdit"]).await;
    publish(&mut findings, &reader, &mut reader_port, &mut reader_folder, "reader").await;
    findings.holds(author_folder.admitted > 0 && reader_folder.admitted > 0, || format!("a folder admitted no batch: author {}, reader {}", author_folder.admitted, reader_folder.admitted));

    let author_before = route_view(&mut author).await;
    let reader_before = route_view(&mut reader).await;
    hold_program(&mut findings, &author_before, &expected["author"], "the author before the reload");
    hold_author(&mut findings, &author_before, &expected["author"], "the author before the reload");
    hold_program(&mut findings, &reader_before, &expected["reader"], "the reader before the reload");

    let (mut author, mut author_port) = reload_from_folder(&mut findings, author, author_port, &author_folder, &fixture, &author_actor, author_uri, cx).await;
    let (mut reader, reader_port) = reload_from_folder(&mut findings, reader, reader_port, &reader_folder, &fixture, &reader_actor, reader_uri, cx).await;
    let author_after = route_view(&mut author).await;
    let reader_after = route_view(&mut reader).await;
    hold_program(&mut findings, &author_after, &expected["author"], "the author after the reload");
    hold_author(&mut findings, &author_after, &expected["author"], "the author after the reload");
    hold_program(&mut findings, &reader_after, &expected["reader"], "the reader after the reload");
    findings.same(&author_after, &author_before, "the author reads back another history than it saved");
    findings.same(&reader_after, &reader_before, "the reader reads back another history than it saved");

    let admitted = author_folder.admitted;
    author_edit(&mut author, &fixture["afterReload"]).await;
    publish(&mut findings, &author, &mut author_port, &mut author_folder, "the reloaded author").await;
    findings.same(&author_folder.admitted, &(admitted + 1), "the reloaded author's edit is admitted as one batch");
    let author_edited = route_view(&mut author).await;
    findings.same(&author_edited.head, &expected_head(&expected["author"]["afterReload"]), "the reloaded author's head after its edit");
    findings.same(&author_edited.line, &author_before.line, "the reloaded author stays on its alternative after its edit");
    findings.same(&author_edited.rows.len(), &(author_before.rows.len() + 1), "the reloaded author's edit is one more row");
    for (step, published, expected_batches) in &batch_shape {
        findings.same(published, expected_batches, &format!("batches published by step {step}"));
    }

    drop((author_port, reader_port));
    for app in [&mut author, &mut reader] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(app, cx);
    }
    let fuel = cx.fuel_remaining();
    drop(context);
    caller.retire(fuel).expect("original route caller paid retirement");
    assert!(findings.0.is_empty(), "the folder reload route departs from its fixture in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

//#region 🪧️NamedRefusals
/// 🧱️ One archive member that names an owned child, at `ordinal`.
fn refusal_member(ordinal: u32) -> protocol::OwnedDocumentMemberPackEntry {
    let reference = |artifact_id: &str| protocol::DocumentArchiveArtifactRef { artifact_id: artifact_id.into(), artifact_kind: "s.test.child".into(), standard: "1".into(), subset: "*".into() };
    protocol::OwnedDocumentMemberPackEntry { ordinal, reference: reference("child"), owner: protocol::DocumentArchiveOwnerRef { parent: reference("parent"), slot: "slot".into(), child_id: "child".into() }, envelope_pack: vec![1] }
}

/// 🪧️ Holds one refusal of the load surface against its code, and its framework notice in English and German against the
/// values it must name: a notice with an unfilled placeholder or without a value is a finding.
fn hold_refusal(findings: &mut RouteFindings, what: &str, fault: &Fault, code: &str, values: &[String]) {
    findings.same(&fault.code.0.as_str(), &code, &format!("{what}: the refusal's code"));
    for locale in [Locale::En, Locale::De] {
        match semio_framework::kernel::fault_notice(fault, &[], Terminology::Native, locale) {
            None => findings.0.push(format!("{what}: {} has no {locale:?} notice", fault.code.0)),
            Some(notice) => findings.holds(!notice.text.trim().is_empty() && !notice.text.contains('{') && values.iter().all(|value| notice.text.contains(value.as_str())), || format!("{what}: the {locale:?} notice reads {:?}, expected it to name {values:?}", notice.text)),
        }
    }
}

/// ⚖️ LAW (audit F16): every refusal of the whole-document load surface is named and told in every locale. An archive with
/// more members than the bound, one larger than the bound, one without its history, one whose member is out of order, a
/// second load under a live operation and a poll of an operation that does not run each answer their own
/// `plugin.document-load.*` code, whose framework notice names the counts and bounds in English and German — and none of
/// them changes the document or leaves an operation behind.
#[semio_framework_async_macros::async_test]
async fn every_refused_whole_document_load_is_named_and_told_in_every_locale() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original load caller");let mut context=caller.context();let cx=&mut context;
    let mut findings = RouteFindings::default();
    let mut app = fresh_program("author").await;
    author_edit(&mut app, &serde_json::json!({ "kind": "setCount", "value": 3 })).await;
    let before = head(&app);
    let own = PluginApp::document_archive(&app).await.expect("the program archives its document");
    let (maximum_members, maximum_bytes) = (protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES);
    let oversized = protocol::DocumentArchivePack { parent_pack: vec![0; maximum_bytes + 1], ..own.clone() };
    let oversized_bytes = oversized.parent_pack.len() + oversized.parent_spr.len();
    let refused = [
        ("too many members", protocol::DocumentArchivePack { members: (0..=maximum_members as u32).map(refusal_member).collect(), ..own.clone() }, "plugin.document-load.too-many-members", vec![(maximum_members + 1).to_string(), maximum_members.to_string()]),
        ("too large", oversized, "plugin.document-load.too-large", vec![oversized_bytes.to_string(), maximum_bytes.to_string()]),
        ("no history", protocol::DocumentArchivePack { parent_spr: Vec::new(), ..own.clone() }, "plugin.document-load.incomplete", Vec::new()),
        ("a member out of order", protocol::DocumentArchivePack { members: vec![refusal_member(3)], ..own.clone() }, "plugin.document-load.member-invalid", vec!["1".to_string()]),
    ];
    for (index, (what, archive, code, values)) in refused.into_iter().enumerate() {
        match semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut app, 70 + index as u64, archive, cx) {
            Ok(()) => findings.0.push(format!("{what}: the archive was admitted")),
            Err(failure) => hold_refusal(&mut findings, what, failure.primary_fault().unwrap_or_else(|| panic!("the refusal is a fault: {failure:?}")), code, &values),
        }
    }
    semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut app, 90, own.clone(), cx).expect("the program's own archive is admitted");
    match semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut app, 90, own, cx) {
        Ok(()) => findings.0.push("a second load under a live operation was admitted".to_string()),
        Err(failure) => hold_refusal(&mut findings, "a second load under a live operation", failure.primary_fault().expect("the refusal is a fault"), "plugin.document-load.busy", &[]),
    }
    match poll_load(&mut app, 4_242, cx).await {
        Ok(status) => findings.0.push(format!("a poll of an operation that does not run answered {status:?}")),
        Err(fault) => hold_refusal(&mut findings, "a poll of an operation that does not run", &fault, "plugin.document-load.operation-unknown", &[]),
    }
    PluginApp::cancel_document_archive_load(&mut app, 90).expect("the admitted load is cancelled");
    let mut polls = 0usize;
    let ended = loop {
        let status = poll_load(&mut app, 90, cx).await.expect("the cancelled load is polled");
        polls += 1;
        assert!(polls < 1_000_000, "the cancelled load never reaches a terminal status");
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status.state;
        }
    };
    PluginApp::acknowledge_document_archive_load(&mut app, 90).expect("the terminal load is acknowledged");
    findings.same(&ended, &protocol::DocumentArchiveLoadState::Cancelled, "the cancelled load's terminal state");
    findings.same(&head(&app), &before, "the document after every refusal and the cancel");
    pump_until(&mut app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
    close(&mut app, cx);
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "the load surface's refusals depart in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}
//#endregion 🪧️NamedRefusals

//#region ⏳️LoadProgressAndCancel
/// 🧱️ A program whose document holds `edits` edits, and its archive.
async fn program_with_history(edits: i64) -> (ToyApp, protocol::DocumentArchivePack) {
    let mut app = fresh_program("author").await;
    for value in 1..=edits {
        author_edit(&mut app, &serde_json::json!({ "kind": "setCount", "value": value })).await;
    }
    let archive = PluginApp::document_archive(&app).await.expect("the program archives its document");
    (app, archive)
}

/// ⚖️ LAW (live fault F6, `📓️api-stepped-document-load.md` §2.2): a whole-document load reports the work it really did —
/// the history records it decoded, the parent and members it admitted, the operations it folded — as a count that never
/// goes back, never exceeds its total and takes more than one value for a document with a history; the total only grows
/// as work is discovered, and a finished load has done all of it.
#[semio_framework_async_macros::async_test]
async fn a_whole_document_load_reports_the_work_it_did_and_never_goes_back() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original load caller");let mut context=caller.context();let cx=&mut context;
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut target = fresh_program("author").await;
    semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut target, 61, archive, cx).expect("the archive is admitted");
    let mut statuses: Vec<(u64, u64)> = Vec::new();
    let ended = loop {
        let status = poll_load(&mut target, 61, cx).await.expect("the load is polled");
        assert!(statuses.len() < 1_000_000, "the load never reaches a terminal status");
        statuses.push((status.completed, status.total));
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status.state;
        }
    };
    PluginApp::acknowledge_document_archive_load(&mut target, 61).expect("the terminal load is acknowledged");
    findings.same(&ended, &protocol::DocumentArchiveLoadState::Ready, "the load's terminal state");
    findings.holds(statuses.windows(2).all(|pair| pair[0].0 <= pair[1].0 && pair[0].1 <= pair[1].1), || format!("a count or a total went back: {statuses:?}"));
    findings.holds(statuses.iter().all(|(completed, total)| completed <= total), || format!("a count exceeded its total: {statuses:?}"));
    let distinct: BTreeSet<u64> = statuses.iter().map(|(completed, _)| *completed).collect();
    findings.holds(distinct.len() > 1 || statuses.len() == 1, || format!("a load of a 24-edit history reported one step only: {statuses:?}"));
    findings.holds(statuses.last().is_some_and(|(completed, total)| completed == total && *total > 1), || format!("the finished load did not do all of its work, or its work was one unit: {:?}", statuses.last()));
    findings.same(&head(&target), &head(&source), "the loaded document");
    for app in [&mut source, &mut target] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(app, cx);
    }
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "the load's progress departs in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

/// ⚖️ LAW (live fault F6): a load the person cancels in the history body stops reading as loading at once — no loading
/// row, no verb waits for it — and then retires in the program's own maintenance turns with not one further host poll; the
/// host's next poll finds it cancelled, and the document is the one the program held.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_load_stops_reading_as_loading_at_once_and_retires_without_a_host_poll() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original maintenance caller");let mut context=caller.context();let cx=&mut context;
    let fixture = route_fixture();
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut target = fresh_program("author").await;
    author_edit(&mut target, &fixture["example"]).await;
    let before = head(&target);
    semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut target, 62, archive, cx).expect("the archive is admitted");
    let first = poll_load(&mut target, 62, cx).await.expect("the load is polled once");
    findings.holds(matches!(first.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running), || format!("the load ended within one poll ({:?}), so the cancel has nothing to cancel", first.state));
    findings.holds(target.render_ref().reprojection_status().is_some_and(|status| status.kind == semio_framework::kernel::HistoryReprojectionKind::Load), || "a running load shows no loading row".to_string());
    let cancelled = verb(&mut target, &fixture, semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Vec::new(), cx).await;
    findings.holds(rejected(&cancelled).is_none(), || format!("the cancel was refused: {:?}", cancelled.output));
    findings.holds(target.render_ref().reprojection_status().is_none(), || format!("the cancelled load still reads as loading: {:?}", target.render_ref().reprojection_status()));
    findings.holds(target.ingest.document_loading_refusal(&target.tools, "undo", false).is_none(), || "a verb still waits for the cancelled load".to_string());
    let mut turns = 0usize;
    while target.ingest.document_archive_loads.get(62).is_some_and(|load| !load.terminal()) && turns < 100_000 {
        maintain_turn(&mut target, cx).await.expect("a maintenance turn");
        turns += 1;
    }
    findings.holds(target.ingest.document_archive_loads.get(62).is_some_and(|load| load.terminal()), || format!("the cancelled load did not retire in {turns} maintenance turns without a host poll"));
    let status = poll_load(&mut target, 62, cx).await.expect("the host's next poll");
    findings.same(&status.state, &protocol::DocumentArchiveLoadState::Cancelled, "what the host's next poll finds");
    PluginApp::acknowledge_document_archive_load(&mut target, 62).expect("the cancelled load is acknowledged");
    findings.same(&head(&target), &before, "the document after the cancelled load");
    for app in [&mut source, &mut target] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(app, cx);
    }
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "the cancelled load departs in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}
/// ⚖️ LAW (live fault F7, `📓️api-stepped-document-load.md` §2.3): a cancel at ANY point of a whole-document load either restores
/// the previous document exactly or is refused because the replacement already committed, and then the load ends ready with
/// the loaded document. It never ends cancelled over the replaced document: the cancel reaches the awaited replacement in the
/// same step it reaches the load, so no maintenance unit commits the candidate in between.
#[semio_framework_async_macros::async_test]
async fn a_cancel_at_any_point_of_a_load_restores_the_previous_document_or_is_refused_after_the_commit() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original maintenance caller");let mut context=caller.context();let cx=&mut context;
    let fixture = route_fixture();
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let (mut cancelled_runs, mut refused_runs) = (0usize, 0usize);
    for polls_before in 0..2_000usize {
        let mut target = fresh_program("author").await;
        author_edit(&mut target, &fixture["example"]).await;
        let before = head(&target);
        semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut target, 63, archive.clone(), cx).expect("the archive is admitted");
        let mut passed = false;
        for _ in 0..polls_before {
            let status = poll_load(&mut target, 63, cx).await.expect("the load is polled");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                passed = true;
                break;
            }
        }
        let refused = if passed { true } else { rejected(&verb(&mut target, &fixture, semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Vec::new(), cx).await).is_some() };
        let mut polls = 0usize;
        let ended = loop {
            let status = poll_load(&mut target, 63, cx).await.expect("the load is polled to its end");
            polls += 1;

            assert!(polls < 1_000_000, "the load never reaches a terminal status");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status.state;
            }
        };
        PluginApp::acknowledge_document_archive_load(&mut target, 63).expect("the terminal load is acknowledged");
        if refused {
            findings.same(&ended, &protocol::DocumentArchiveLoadState::Ready, &format!("cancel after {polls_before} polls: a refused cancel leaves a ready load"));
            findings.same(&head(&target), &head(&source), &format!("cancel after {polls_before} polls: the document after a load whose cancel was refused"));
            refused_runs += 1;
        } else {
            findings.same(&ended, &protocol::DocumentArchiveLoadState::Cancelled, &format!("cancel after {polls_before} polls: the cancelled load's terminal state"));
            findings.same(&head(&target), &before, &format!("cancel after {polls_before} polls: the document after the cancelled load"));
            cancelled_runs += 1;
            author_edit(&mut target, &serde_json::json!({ "kind": "setCount", "value": 9_001 })).await;
            findings.holds(head(&target) != before, || format!("cancel after {polls_before} polls: the cancelled program refused a further edit"));
            maintain_turn(&mut target, cx).await.expect("a maintenance turn after the cancelled load");
            let undone = verb(&mut target, &fixture, "undo", Vec::new(), cx).await;
            findings.holds(rejected(&undone).is_none(), || format!("cancel after {polls_before} polls: the cancelled program refused a further action: {:?}", undone.output));
        }
        pump_until(&mut target, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(&mut target, cx);
        if passed { break; }
    }
    findings.holds(cancelled_runs > 0, || "no cancel in the scan took effect".to_string());
    pump_until(&mut source, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
    close(&mut source, cx);
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "{cancelled_runs} cancelled and {refused_runs} refused cancels depart in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

/// 🔁️ One live-cleanup turn of the runtime on a renewed host turn, with the program holding its cancellation issuer.
async fn live_turn(runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<ToyApp>, cell: &std::sync::Arc<semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell<ToyApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
    let refused = |failure: &dyn std::fmt::Debug| Fault::from(format!("the live cleanup turn was refused: {failure:?}"));
    let (grant, _) = cx.original_turn_ceiling();
    artifact_app_laws::fixture_renew_turn(cx, semio_plugin_testing_laws::OriginalFixtureCaller::turn_fuel(), grant).map_err(|failure| refused(&failure))?;
    artifact_app_laws::ensure_fixture_cancellation_issuer(&mut cell.instance.lock().map_err(|_| refused(&"poisoned"))?.app, cx).map_err(|failure| refused(&failure))?;
    cx.consume_fuel(1);
    semio_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(runtime, cx).map(|_| ()).map_err(|failure| refused(&failure))
}

/// 🧹️ Closes one runtime instance and pays its close ladder, turn by turn, until it leaves quarantine.
async fn retire_instance(runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<ToyApp>, cell: std::sync::Arc<semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell<ToyApp>>, id: u32, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
    let refused = |failure: &dyn std::fmt::Debug| Fault::from(format!("the close ladder was refused: {failure:?}"));
    drop(cell);
    semio_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(runtime, id).await?;
    for _ in 0..100_000 {
        if !cx.is_cancelled() {
            let (grant, _) = cx.original_turn_ceiling();
            artifact_app_laws::fixture_renew_turn(cx, semio_plugin_testing_laws::OriginalFixtureCaller::turn_fuel(), grant).map_err(|failure| refused(&failure))?;
        }
        cx.consume_fuel(1);
        semio_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(runtime, cx).map_err(|failure| refused(&failure))?;
        if runtime.close_quarantine.borrow().get(id).is_none() {
            return Ok(());
        }
    }
    Err(Fault::from("the closed instance never left quarantine"))
}

/// ⚖️ LAW (live fault F9, browser): cancelling a replaying whole-document load is a normal outcome. A replay cancelled through the
/// same action the browser uses leaves the runtime instance live: no live-cleanup turn after the cancel is refused or leaves the
/// instance faulted, the load retires cancelled over the previous document, and a further edit and action are accepted.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_replay_leaves_the_runtime_instance_live_and_accepting_actions() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original runtime caller");let mut context=caller.context();let cx=&mut context;
    let fixture = route_fixture();
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut cancelled_runs = 0usize;
    for turns_before in [0usize, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987] {
        let mut scan_caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original scan caller");let mut scan_context=scan_caller.context();let cx=&mut scan_context;
        let mut app = fresh_program("author").await;
        author_edit(&mut app, &fixture["example"]).await;
        let before = head(&app);
        let id = PluginApp::birth_identity(&app).expect("a registered program has its instance").0;
        let runtime = semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<ToyApp>::new(artifact_app_laws::fixture_mounted_policy()).expect("the runtime");
        let cell = std::sync::Arc::new(semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(semio_plugin_plugin_program::AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_owner_policy()));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let running = |cell: &std::sync::Arc<semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell<ToyApp>>| cell.instance.lock().expect("the instance").app.ingest.document_archive_loads.get(66).is_some_and(|load| !load.terminal());
        artifact_app_laws::admit_document_archive_load(&mut cell.instance.lock().expect("the instance").app, 66, archive.clone(), cx).expect("the archive is admitted");
        for _ in 0..turns_before {
            live_turn(&runtime, &cell, cx).await.expect("a live cleanup turn before the cancel");
        }
        if !running(&cell) {
            retire_instance(&runtime, cell, id, cx).await.expect("the instance retires through its close ladder");
            let fuel=cx.fuel_remaining();drop(scan_context);scan_caller.retire(fuel).expect("original scan caller paid retirement");
            break;
        }
        let cancelled = verb(&mut cell.instance.lock().expect("the instance").app, &fixture, semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Vec::new(), cx).await;
        findings.holds(rejected(&cancelled).is_none(), || format!("cancel after {turns_before} live turns was refused: {:?}", cancelled.output));
        for _ in 0..4 {
            if let Err(fault) = live_turn(&runtime, &cell, cx).await {
                findings.holds(false, || format!("cancel after {turns_before} live turns: a live turn right after the cancel was refused: {fault:?}"));
            }
        }
        let mut turns = 0usize;
        while running(&cell) && turns < 100_000 {
            if let Err(fault) = live_turn(&runtime, &cell, cx).await {
                findings.holds(false, || format!("cancel after {turns_before} live turns: live turn {turns} after the cancel was refused: {fault:?}"));
                break;
            }
            turns += 1;
        }
        let status = semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus::from_repr(cell.maintenance_status.load(std::sync::atomic::Ordering::SeqCst));
        findings.holds(!matches!(status, semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus::Fault(_)), || format!("cancel after {turns_before} live turns left the instance faulted: {status:?}"));
        {
            let mut active = cell.instance.lock().expect("the instance");
            findings.holds(active.app.ingest.document_archive_loads.get(66).is_some_and(|load| load.terminal()), || format!("cancel after {turns_before} live turns: the cancelled load did not retire"));
            let ended = PluginApp::poll_document_archive_load(&mut active.app, 66, &mut artifact_app_laws::fixture_identity(), cx).await.expect("the host's next poll").state;
            findings.same(&ended, &protocol::DocumentArchiveLoadState::Cancelled, &format!("cancel after {turns_before} live turns: the terminal state"));
            PluginApp::acknowledge_document_archive_load(&mut active.app, 66).expect("the cancelled load is acknowledged");
            findings.same(&head(&active.app), &before, &format!("cancel after {turns_before} live turns: the document after the cancelled load"));
        }
        author_edit(&mut cell.instance.lock().expect("the instance").app, &serde_json::json!({ "kind": "setCount", "value": 9_001 })).await;
        findings.holds(head(&cell.instance.lock().expect("the instance").app) != before, || format!("cancel after {turns_before} live turns: the instance refused a further edit"));
        let undone = verb(&mut cell.instance.lock().expect("the instance").app, &fixture, "undo", Vec::new(), cx).await;
        findings.holds(rejected(&undone).is_none(), || format!("cancel after {turns_before} live turns: the instance refused a further action: {:?}", undone.output));
        for _ in 0..8 {
            if let Err(fault) = live_turn(&runtime, &cell, cx).await {
                findings.holds(false, || format!("cancel after {turns_before} live turns: a live turn after the further edit was refused: {fault:?}"));
                break;
            }
        }
        retire_instance(&runtime, cell, id, cx).await.expect("the instance retires through its close ladder");
        cancelled_runs += 1;
        let fuel=cx.fuel_remaining();drop(scan_context);scan_caller.retire(fuel).expect("original scan caller paid retirement");
    }
    findings.holds(cancelled_runs > 0, || "no cancel in the scan took effect".to_string());
    pump_until(&mut source, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
    close(&mut source, cx);
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "{cancelled_runs} cancelled replays depart in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

/// ⚖️ LAW (live fault F7, receipts): a cancel at ANY point of a load, paid through every ladder stage it reaches, receipts each
/// maintenance step exactly once: a step's receipt is the effects the turn posted on its context on the items, copied bytes and
/// capacity, no release is posted beyond it, and the scan pays the ladder while a cancelled store replacement is still initializing.
/// Every other cancel goes straight to the program's live cleanup, which must pay the cancelled load and its replacement whole.
#[semio_framework_async_macros::async_test]
async fn a_cancel_at_every_ladder_stage_receipts_each_step_exactly_once() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original maintenance caller");let mut context=caller.context();let cx=&mut context;
    let fixture = route_fixture();
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut initializing_cancelled = false;
    for polls_before in 0..2_000usize {
        let mut target = fresh_program("author").await;
        author_edit(&mut target, &fixture["example"]).await;
        semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut target, 65, archive.clone(), cx).expect("the archive is admitted");
        let mut passed = false;
        for _ in 0..polls_before {
            let status = poll_load(&mut target, 65, cx).await.expect("the load is polled");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                passed = true;
                break;
            }
        }
        let cancelled = !passed && rejected(&verb(&mut target, &fixture, semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Vec::new(), cx).await).is_none();
        initializing_cancelled |= cancelled && target.ingest.store_replacement_jobs.next_id_from(0).and_then(|(_, operation)| target.ingest.store_replacement_jobs.get(operation)).is_some_and(|job| format!("{:?}", job.state) == "Initializing");
        if cancelled && polls_before % 2 == 1 {
            close(&mut target, cx);
            continue;
        }
        let mut turns = 0usize;
        while target.ingest.document_archive_loads.get(65).is_some_and(|load| !load.terminal()) && turns < 100_000 {
            let stage = target.next_maintenance_stage();
            initializing_cancelled |= cancelled && target.ingest.store_replacement_jobs.next_id_from(0).and_then(|(_, operation)| target.ingest.store_replacement_jobs.get(operation)).is_some_and(|job| format!("{:?}", job.state) == "Initializing");
            let (receipt, posted) = maintain_turn(&mut target, cx).await.expect("a maintenance turn");
            findings.same(&(receipt.copied_items, receipt.copied_bytes, receipt.retained_capacity_bytes), &(posted.copied_items, posted.copied_bytes, posted.retained_capacity_bytes), &format!("cancel after {polls_before} polls: the receipt of the step at stage {stage} equals what it posted"));
            findings.holds(posted.released_bytes >= receipt.released_bytes, || format!("cancel after {polls_before} polls: the step at stage {stage} receipted {} released bytes but posted {}", receipt.released_bytes, posted.released_bytes));
            turns += 1;
        }
        let status = poll_load(&mut target, 65, cx).await.expect("the load is polled to its end");
        PluginApp::acknowledge_document_archive_load(&mut target, 65).ok();
        findings.holds(passed || !cancelled || status.state == protocol::DocumentArchiveLoadState::Cancelled, || format!("cancel after {polls_before} polls ended {:?}", status.state));
        pump_until(&mut target, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(&mut target, cx);
        if passed { break; }
    }
    findings.holds(initializing_cancelled, || "no cancel in the scan paid the ladder while a store replacement was still initializing".to_string());
    pump_until(&mut source, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
    close(&mut source, cx);
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "the cancelled loads' receipts depart in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

/// ⚖️ LAW (live fault F8): the status of a running load counts the records its store replacement decoded, as discovered and done
/// at once, so the count keeps moving through the long phase after the fold, where neither a stage nor a member count moves;
/// it is the part of `completed` the load's own counters do not explain, it never goes back, and the finished load keeps it.
#[semio_framework_async_macros::async_test]
async fn a_whole_document_load_counts_the_records_its_replacement_decoded() {
    let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original load caller");let mut context=caller.context();let cx=&mut context;
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut target = fresh_program("author").await;
    semio_plugin_testing_laws::artifact_app_laws::admit_document_archive_load(&mut target, 64, archive, cx).expect("the archive is admitted");
    let mut decoded: Vec<u64> = Vec::new();
    let ended = loop {
        let status = poll_load(&mut target, 64, cx).await.expect("the load is polled");
        assert!(decoded.len() < 1_000_000, "the load never reaches a terminal status");
        let load = target.ingest.document_archive_loads.get(64).expect("the polled load is live");
        let explained = load.completed + load.fold.0 + load.stage + load.members_done;
        let live = load.replacement.and_then(|handle| target.ingest.store_replacement_jobs.get(handle.operation.0)).map_or(0, |job| job.decoded);
        let decoded_now = live.max(load.replacement_decoded);
        findings.same(&status.completed, &(explained + decoded_now), "completed is the load's own counters plus the replacement's decoded records");
        findings.same(&status.total, &(load.total + load.fold.1 + decoded_now), "total grows with the discovered records");
        decoded.push(decoded_now);
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status.state;
        }
    };
    PluginApp::acknowledge_document_archive_load(&mut target, 64).expect("the terminal load is acknowledged");
    findings.same(&ended, &protocol::DocumentArchiveLoadState::Ready, "the load's terminal state");
    findings.holds(decoded.windows(2).all(|pair| pair[0] <= pair[1]), || format!("the decoded count went back: {decoded:?}"));
    findings.holds(decoded.last().is_some_and(|last| *last > 0), || format!("the replacement decoded no record the load could report: {decoded:?}"));
    for app in [&mut source, &mut target] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(app, cx);
    }
    let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).expect("original load caller paid retirement");
    assert!(findings.0.is_empty(), "the load's decode progress departs in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}
//#endregion ⏳️LoadProgressAndCancel

/// 🕒️ The positions, in applied order, of the document mutations the history lists as not applied while a history edit is open.
async fn pending_positions(app: &mut ToyApp) -> Vec<u64> {
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    patch.upserts.iter().rev().filter(|entry| entry.edit_id.is_some()).flat_map(|entry| entry.mutations.iter()).enumerate().filter(|(_, mutation)| mutation.pending).map(|(position, _)| position as u64).collect()
}

/// 🧮️ What a fixture merge answer expects: the events merged and the events the reader is ahead by.
fn expected_merge(value: &Value) -> (u64, u64) {
    (value["merged"].as_u64().expect("merged"), value["ahead"].as_u64().expect("ahead"))
}

/// 🤝️ Holds `view` against what both programs show once the two-peer run settled.
fn hold_settled(findings: &mut RouteFindings, view: &RouteView, expected: &Value, who: &str) {
    findings.same(&view.head, &expected_head(&expected["head"]), &format!("{who}: the settled head"));
    let superseded: Vec<u64> = applied_mutations(view).iter().enumerate().filter(|(_, mutation)| mutation.superseded).map(|(position, _)| position as u64).collect();
    findings.same(&superseded, &expected["superseded"].as_array().expect("superseded").iter().map(|position| position.as_u64().expect("position")).collect(), &format!("{who}: the mutations that read superseded"));
    let history: Vec<(String, String)> = view.rows.iter().filter(|row| row.transition.is_some()).map(|row| row.label.clone()).collect();
    findings.same(&history, &expected["historyRows"].as_array().expect("historyRows").iter().map(|label| (text(&label["en"]).to_string(), text(&label["de"]).to_string())).collect(), &format!("{who}: the history-edit rows"));
    findings.same(&(view.rows.iter().filter(|row| row.edit.is_some()).count() as u64), &expected["editRows"].as_u64().expect("editRows"), &format!("{who}: one row per document edit"));
}

/// 📏️ The causal batch message payload maximum a replay may encode in one message.
const ONE_BATCH_PAYLOAD_BYTES: usize = 262_144;

/// ⚖️ LAW (replay extent): a document replays a history whose persisted payload is larger than one causal batch
/// message, because every replacement Store carries the app's persisted operation wire and a replay only ever encodes the
/// few operations it touches, never the history length.
#[semio_framework_async_macros::async_test]
async fn a_history_larger_than_one_batch_payload_replays_through_a_history_edit() {
    let mut caller = semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original route caller");
    let mut context = caller.context();
    let cx = &mut context;
    let fixture = route_fixture();
    let mut findings = RouteFindings::default();
    let label = "x".repeat(60_000);
    let edits = 2 * (ONE_BATCH_PAYLOAD_BYTES / label.len() + 1);
    let mut source = fresh_program("author").await;
    author_edit(&mut source, &serde_json::json!({ "kind": "setCount", "value": 100 })).await;
    for _ in 0..edits {
        author_edit(&mut source, &serde_json::json!({ "kind": "setLabel", "value": label })).await;
    }
    for step in [serde_json::json!({ "begin": 0 }), serde_json::json!({ "input": { "path": "/value", "value": 180 } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        route_step(&mut findings, &mut source, &fixture, &step, cx).await;
    }
    for app in [&mut source] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(app, cx);
    }
    let fuel = cx.fuel_remaining();
    drop(context);
    caller.retire(fuel).expect("original route caller paid retirement");
    assert!(findings.0.is_empty(), "the replay departs in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

/// ⚖️ LAW (live fault F4, design §22.22): two programs on ONE folder converge through an open history edit. The author binds
/// an empty folder, which takes its document, and edits; the peer attaches and loads the folder's archive cold. The author
/// opens a history edit and drafts; the peer edits, and the folder takes the peer's archive. The author's read-back MERGES
/// through the host's driver: it takes exactly the peer's edit and is ahead of nothing, its history edit stays open on the
/// same target and the same preview, the peer's edit is listed downstream as not applied, its port stays bound, and nothing
/// is written. The author accepts — the replay takes the peer's edit too — and overwrites: one batch is published and the
/// folder takes the author's archive. The peer's read-back merges that history edit, stays where it stood, and both programs
/// show the same head, superseded mutation and rows; a further read-back takes nothing and leaves nobody ahead.
#[semio_framework_async_macros::async_test]
async fn two_programs_on_one_folder_converge_through_an_open_history_edit() {
    let mut caller = semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original route caller");
    let mut context = caller.context();
    let cx = &mut context;
    let fixture = route_fixture();
    let scenario = &fixture["twoPeers"];
    let expected = &scenario["expected"];
    let (author_actor, peer_actor) = (text(&fixture["actor"]).to_string(), text(&fixture["reader"]).to_string());
    let mut findings = RouteFindings::default();
    let mut folder = RouteFolder::default();

    let mut author = fresh_program(&author_actor).await;
    let mut author_port = bind_document_port(&mut author, "folder-two-peers-author").await;
    save_archive(&mut findings, &author, &mut folder, "the author at an empty folder").await;
    for step in scenario["author"].as_array().expect("author steps") {
        route_step(&mut findings, &mut author, &fixture, step, cx).await;
        publish(&mut findings, &author, &mut author_port, &mut folder, "author").await;
    }

    let mut peer = fresh_program(&peer_actor).await;
    author_edit(&mut peer, &fixture["example"]).await;
    let archive = protocol::decode_document_archive_bytes(folder.archive.as_ref().expect("the folder holds the author's archive")).await.expect("the folder's archive decodes");
    let (loaded, _) = load_through_the_host(&mut peer, archive, cx).await;
    findings.same(&loaded, &protocol::DocumentArchiveLoadOutcome::Ready, "the peer loads the folder's archive at its attach");
    let mut peer_port = bind_document_port(&mut peer, "folder-two-peers-peer").await;
    findings.same(&head(&peer), &head(&author), "the peer shows the author's document");

    for step in scenario["session"].as_array().expect("session steps") {
        route_step(&mut findings, &mut author, &fixture, step, cx).await;
    }
    pump_until(&mut author, "the draft's preview settles", |app| !app.time_travel.has_pending_work(), cx).await;
    let preview = render_body(&mut author, cx).await;
    findings.holds(preview.contains(text(&expected["previewBody"])), || format!("the author's preview while editing reads {preview}, expected {}", expected["previewBody"]));
    let target = author.time_travel.editor().map(|editor| editor.value.clone());

    author_edit(&mut peer, &scenario["peerEdit"]).await;
    let written = publish(&mut findings, &peer, &mut peer_port, &mut folder, "peer").await;
    findings.same(&written.len(), &1, "the peer's edit is one published batch, saved to the folder");

    let generation = author.stores.store.generation();
    let (outcome, merged, ahead) = merge_through_the_host(&mut author, &folder, cx).await;
    findings.same(&outcome, &protocol::DocumentArchiveLoadOutcome::Ready, "the author's read-back merges");
    findings.same(&(merged, ahead), &expected_merge(&expected["authorMerge"]), "the author's read-back (merged, ahead)");
    findings.holds(author.stores.store.generation() != generation, || "the merge moved nothing the author's open history edit could see".to_string());
    let status = author.time_travel.status();
    findings.holds(status.as_ref().is_some_and(|status| serde_json::to_value(status.stage).ok() == Some(Value::from("editing"))), || format!("the author's history edit did not stay open in editing: {:?}", status.map(|status| status.stage)));
    findings.same(&author.time_travel.editor().map(|editor| editor.value.clone()), &target, "the author's draft after the read-back");
    pump_until(&mut author, "the preview over the moved base settles", |app| !app.time_travel.has_pending_work(), cx).await;
    let preview = render_body(&mut author, cx).await;
    findings.holds(preview.contains(text(&expected["previewBody"])), || format!("the read-back changed the author's preview to {preview}, expected {}", expected["previewBody"]));
    findings.same(&pending_positions(&mut author).await, &expected["pendingWhileEditing"].as_array().expect("pending").iter().map(|position| position.as_u64().expect("position")).collect(), "the mutations listed as not applied while editing, the peer's included");
    findings.holds(author.stores.store.backbone_ref().is_some(), || "the read-back left the author without a bound document port".to_string());
    findings.holds(author_port.receive().await.expect("the author's port drains").iter().all(|message| matches!(message, BackboneMessage::Ack { .. })), || "the read-back made the author publish although it holds nothing the folder lacks".to_string());

    for step in scenario["finish"].as_array().expect("finish steps") {
        route_step(&mut findings, &mut author, &fixture, step, cx).await;
        let batches = publish(&mut findings, &author, &mut author_port, &mut folder, "author").await;
        findings.same(&batches.len(), &usize::from(step.get("commit").is_some()), &format!("batches the author published by step {step}"));
    }
    let author_view = route_view(&mut author).await;
    hold_settled(&mut findings, &author_view, &expected["settled"], "the author");

    let peer_line = route_view(&mut peer).await.line;
    let (outcome, merged, ahead) = merge_through_the_host(&mut peer, &folder, cx).await;
    findings.same(&outcome, &protocol::DocumentArchiveLoadOutcome::Ready, "the peer's read-back merges");
    findings.same(&(merged, ahead), &expected_merge(&expected["peerMerge"]), "the peer's read-back (merged, ahead)");
    let peer_view = route_view(&mut peer).await;
    hold_settled(&mut findings, &peer_view, &expected["settled"], "the peer");
    findings.same(&peer_view.line, &peer_line, "the peer's viewed alternative after the read-back");
    let (_, merged, ahead) = merge_through_the_host(&mut author, &folder, cx).await;
    findings.same(&(merged, ahead), &(0, 0), "a further read-back of the author takes nothing and leaves it ahead of nothing");

    drop((author_port, peer_port));
    for app in [&mut author, &mut peer] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work(), cx).await;
        close(app, cx);
    }
    let fuel = cx.fuel_remaining();
    drop(context);
    caller.retire(fuel).expect("original route caller paid retirement");
    assert!(findings.0.is_empty(), "two programs on one folder depart from the fixture in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}
