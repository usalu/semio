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

use super::*;

const FOLDER_RELOAD_ROUTE_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧫️folder-reload-route/🔣️.json");

async fn pump_until(app: &mut ToyApp, what: &str, done: impl Fn(&ToyApp) -> bool) {
    super::pump_until(app, what, done, crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await
}

async fn verb(app: &mut ToyApp, fixture: &Value, action: &str, args: Vec<(String, DslValue)>) -> InvocationResult {
    super::verb(app, fixture, action, args, crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await
}

async fn run_step(app: &mut ToyApp, fixture: &Value, step: &Value) -> Option<InvocationResult> {
    super::run_step(app, fixture, step, crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await
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
    fn same<T: PartialEq + std::fmt::Debug>(&mut self, actual: &T, expected: &T, what: &str) {
        self.holds(actual == expected, || format!("{what}: {actual:?}, expected {expected:?}"));
    }
}

/// 🔹️ One mutation row as the history shows it, in every locale.
#[derive(Debug, PartialEq)]
struct RouteMutation {
    id: String,
    label: (String, String),
    superseded: bool,
    withdrawn: bool,
    editable: bool,
    worst: Option<semio_framework_diagnostic::Severity>,
    codes: Vec<String>,
}

/// 🗂️ One document row of the history: an edit or a finalized history edit, never a row of the running program alone.
#[derive(Debug, PartialEq)]
struct RouteRow {
    edit: Option<String>,
    transition: Option<String>,
    kind: String,
    label: (String, String),
    applied: bool,
    mutations: Vec<RouteMutation>,
}

/// 🧾️ What a host reads from a program to paint its document and history, less what a reload renews (row sequence numbers).
#[derive(Debug, PartialEq)]
struct RouteView {
    head: (i32, String),
    line: String,
    alternatives: Vec<String>,
    rows: Vec<RouteRow>,
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
    let envelope = app.store.envelope();
    let alternatives: Vec<String> = envelope.vcs.alternatives.iter().map(|alternative| alternative.name.clone()).collect();
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
    archive: Option<Vec<u8>>,
    admitted: usize,
}

/// 🔌️ Binds `app`'s document port the way `bindDocumentPort` does — a hot attach, which announces nothing already in
/// the log — and answers the host's end of it.
async fn bind_document_port(app: &mut ToyApp, uri: &str) -> MemoryBackbone {
    let (guest, host) = MemoryBackbone::pair(uri, uri).await;
    PluginApp::attach_hot_backbone(app, store::Backbones::Memory(guest)).await.expect("the document port binds");
    host
}

/// 🧩️ A fresh registered program of the toy app authoring as `actor`.
async fn fresh_program(actor: &str) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into()), crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.store.local_actor_id().0.as_str(), actor);
    app
}

/// ✏️ One document edit authored through the program's store, as a committed gesture leaves it.
async fn author_edit(app: &mut ToyApp, op: &Value) {
    crate::with_authoring_identity!(|identity| app.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], transaction: None }, &mut identity).await).expect("the edit applies");
    app.refresh_cache().await.expect("the history backfills the edit");
}

/// ⏯️ Runs one fixture step: a document edit, or a history-edit verb through the session laws' own runner; a finalized
/// session is driven until it retired.
async fn route_step(findings: &mut RouteFindings, app: &mut ToyApp, fixture: &Value, step: &Value) {
    if let Some(op) = step.get("edit") {
        return author_edit(app, op).await;
    }
    if let Some(result) = run_step(app, fixture, step).await {
        findings.holds(rejected(&result).is_none(), || format!("step {step} was refused: {:?}", result.output));
    }
    if step.get("commit").is_some() {
        pump_until(app, "the finalize retires", |app| !app.time_travel.has_pending_work()).await;
        app.refresh_cache().await.expect("the history backfills the finalized history edit");
    }
}

/// 📤️ What the host does with everything `app` published since the last call: each batch must name the bound document's
/// own identity — the folder's actor admits no other — and any admitted batch makes the host save the program's whole
/// archive as the folder's content. Answers the admitted batches.
async fn publish(findings: &mut RouteFindings, app: &ToyApp, port: &mut MemoryBackbone, folder: &mut RouteFolder, who: &str) -> Vec<BackboneMessage> {
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
async fn deliver(batches: Vec<BackboneMessage>, port: &mut MemoryBackbone, to: &mut ToyApp) {
    if batches.is_empty() {
        return;
    }
    for message in batches {
        port.send(message).await.expect("a batch reaches the reader");
    }
    to.tick_backbone(&mut crate::app::artifact_app_laws::fixture_identity()).await.expect("the reader ingests the batch");
    pump_until(to, "the reader adopts the batch", |app| app.store.reprojection_progress().is_none()).await;
    to.refresh_cache().await.expect("the reader's history backfills");
}

/// 🧭️ Drives one host load driver against `app`: the kernel's driver decides each command, and the program answers it as its
/// channel does (admit a load or a merge, poll to a terminal status, acknowledge). Answers the outcome, the polls it took
/// and the terminal status.
async fn drive_archive_host(app: &mut ToyApp, mut host: protocol::DocumentArchiveLoadHost) -> (protocol::DocumentArchiveLoadOutcome, usize, Option<protocol::DocumentArchiveLoadStatus>) {
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
            protocol::AppCommand::LoadDocumentArchive { seq, archive } => PluginApp::begin_document_archive_load(app, seq, archive).map(|()| protocol::AppFrame::Done { in_reply_to: seq }),
            protocol::AppCommand::MergeDocumentArchive { seq, archive } => PluginApp::begin_document_archive_merge(app, seq, archive).map(|()| protocol::AppFrame::Done { in_reply_to: seq }),
            protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } => {
                polls += 1;
                assert!(polls < 1_000_000, "the archive operation never reaches a terminal status");
                PluginApp::poll_document_archive_load(app, operation, &mut crate::app::artifact_app_laws::fixture_identity()).await.map(|status| protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status })
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
async fn load_through_the_host(app: &mut ToyApp, archive: protocol::DocumentArchivePack) -> (protocol::DocumentArchiveLoadOutcome, usize) {
    let (outcome, polls, _) = drive_archive_host(app, protocol::DocumentArchiveLoadHost::new(archive)).await;
    (outcome, polls)
}

/// 🔀️ Merges the folder's archive into `app` the way a host does for a read-back of the document the program already shows,
/// and lets the program adopt what it took: answers the outcome, the events merged and the events `app` is ahead by.
async fn merge_through_the_host(app: &mut ToyApp, folder: &RouteFolder) -> (protocol::DocumentArchiveLoadOutcome, u64, u64) {
    let archive = protocol::decode_document_archive_bytes(folder.archive.as_ref().expect("the folder holds an archive")).await.expect("the folder's archive decodes");
    let (outcome, _, terminal) = drive_archive_host(app, protocol::DocumentArchiveLoadHost::merging(archive)).await;
    pump_until(app, "the merged events are adopted", |app| app.store.reprojection_progress().is_none()).await;
    app.refresh_cache().await.expect("the history backfills the merged events");
    let terminal = terminal.expect("a merge is polled to its terminal status");
    (outcome, terminal.completed, terminal.ahead)
}

/// 🔁️ Detaches `app` from its folder and closes it, then walks the route a page reload and a re-attach take: a fresh
/// program holding its own example document reports the same identity, binds its port (publishing nothing, so the folder
/// is not overwritten before it is read back), has the port retired for the load, restores the folder's archive and is
/// bound again. Answers the fresh program and the host's end of its port.
async fn reload_from_folder(findings: &mut RouteFindings, mut app: ToyApp, port: MemoryBackbone, folder: &RouteFolder, fixture: &Value, actor: &str, uri: &str) -> (ToyApp, MemoryBackbone) {
    let identity = PluginApp::document_identity(&app).expect("a bound program holds a document");
    PluginApp::detach_backbone(&mut app).await.expect("the program detaches from its folder");
    drop(port);
    pump_until(&mut app, "the detached program settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
    let mut fresh = fresh_program(actor).await;
    author_edit(&mut fresh, &fixture["example"]).await;
    findings.same(&PluginApp::document_identity(&fresh), &Some(identity), &format!("{actor}: a fresh program of the same app holds the document identity its folder was remembered under"));
    let mut bound = bind_document_port(&mut fresh, uri).await;
    findings.holds(bound.receive().await.expect("the fresh port drains").is_empty(), || format!("{actor}: binding the port published the fresh program's own document, which would overwrite the folder before its read-back"));
    PluginApp::detach_backbone(&mut fresh).await.expect("the host retires the port for the load");
    drop(bound);
    let Some(bytes) = folder.archive.as_ref() else {
        findings.0.push(format!("{actor}: the folder holds no archive to read back"));
        return (fresh, MemoryBackbone::pair(uri, uri).await.1);
    };
    let archive = protocol::decode_document_archive_bytes(bytes).await.expect("the folder's archive decodes");
    let (outcome, polls) = load_through_the_host(&mut fresh, archive).await;
    findings.same(&outcome, &protocol::DocumentArchiveLoadOutcome::Ready, &format!("{actor}: the folder's archive loads"));
    findings.holds(polls >= 1, || format!("{actor}: the load reached its terminal status without one poll"));
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
        route_step(&mut findings, &mut author, &fixture, step).await;
        let batches = publish(&mut findings, &author, &mut author_port, &mut author_folder, "author").await;
        let publishes = step.get("edit").is_some() || step.get("commit").is_some();
        batch_shape.push((step.to_string(), batches.len(), usize::from(publishes)));
        deliver(batches, &mut reader_port, &mut reader).await;
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

    let (mut author, mut author_port) = reload_from_folder(&mut findings, author, author_port, &author_folder, &fixture, &author_actor, author_uri).await;
    let (mut reader, reader_port) = reload_from_folder(&mut findings, reader, reader_port, &reader_folder, &fixture, &reader_actor, reader_uri).await;
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
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work()).await;
        close(app);
    }
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
        match PluginApp::begin_document_archive_load(&mut app, 70 + index as u64, archive) {
            Ok(()) => findings.0.push(format!("{what}: the archive was admitted")),
            Err(fault) => hold_refusal(&mut findings, what, &fault, code, &values),
        }
    }
    PluginApp::begin_document_archive_load(&mut app, 90, own.clone()).expect("the program's own archive is admitted");
    match PluginApp::begin_document_archive_load(&mut app, 90, own) {
        Ok(()) => findings.0.push("a second load under a live operation was admitted".to_string()),
        Err(fault) => hold_refusal(&mut findings, "a second load under a live operation", &fault, "plugin.document-load.busy", &[]),
    }
    match PluginApp::poll_document_archive_load(&mut app, 4_242, &mut crate::app::artifact_app_laws::fixture_identity()).await {
        Ok(status) => findings.0.push(format!("a poll of an operation that does not run answered {status:?}")),
        Err(fault) => hold_refusal(&mut findings, "a poll of an operation that does not run", &fault, "plugin.document-load.operation-unknown", &[]),
    }
    PluginApp::cancel_document_archive_load(&mut app, 90).expect("the admitted load is cancelled");
    let mut polls = 0usize;
    let ended = loop {
        let status = PluginApp::poll_document_archive_load(&mut app, 90, &mut crate::app::artifact_app_laws::fixture_identity()).await.expect("the cancelled load is polled");
        polls += 1;
        assert!(polls < 1_000_000, "the cancelled load never reaches a terminal status");
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status.state;
        }
    };
    PluginApp::acknowledge_document_archive_load(&mut app, 90).expect("the terminal load is acknowledged");
    findings.same(&ended, &protocol::DocumentArchiveLoadState::Cancelled, "the cancelled load's terminal state");
    findings.same(&head(&app), &before, "the document after every refusal and the cancel");
    pump_until(&mut app, "the program settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
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
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut target = fresh_program("author").await;
    PluginApp::begin_document_archive_load(&mut target, 61, archive).expect("the archive is admitted");
    let mut statuses: Vec<(u64, u64)> = Vec::new();
    let ended = loop {
        let status = PluginApp::poll_document_archive_load(&mut target, 61, &mut crate::app::artifact_app_laws::fixture_identity()).await.expect("the load is polled");
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
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work()).await;
        close(app);
    }
    assert!(findings.0.is_empty(), "the load's progress departs in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}

/// ⚖️ LAW (live fault F6): a load the person cancels in the history body stops reading as loading at once — no loading
/// row, no verb waits for it — and then retires in the program's own maintenance turns with not one further host poll; the
/// host's next poll finds it cancelled, and the document is the one the program held.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_load_stops_reading_as_loading_at_once_and_retires_without_a_host_poll() {
    let fixture = route_fixture();
    let mut findings = RouteFindings::default();
    let (mut source, archive) = program_with_history(24).await;
    let mut target = fresh_program("author").await;
    author_edit(&mut target, &fixture["example"]).await;
    let before = head(&target);
    PluginApp::begin_document_archive_load(&mut target, 62, archive).expect("the archive is admitted");
    let first = PluginApp::poll_document_archive_load(&mut target, 62, &mut crate::app::artifact_app_laws::fixture_identity()).await.expect("the load is polled once");
    findings.holds(matches!(first.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running), || format!("the load ended within one poll ({:?}), so the cancel has nothing to cancel", first.state));
    findings.holds(target.reprojection_status().is_some_and(|status| status.kind == semio_framework::kernel::HistoryReprojectionKind::Load), || "a running load shows no loading row".to_string());
    let cancelled = verb(&mut target, &fixture, semio_framework::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Vec::new()).await;
    findings.holds(rejected(&cancelled).is_none(), || format!("the cancel was refused: {:?}", cancelled.output));
    findings.holds(target.reprojection_status().is_none(), || format!("the cancelled load still reads as loading: {:?}", target.reprojection_status()));
    findings.holds(target.document_loading_refusal("undo").is_none(), || "a verb still waits for the cancelled load".to_string());
    let mut turns = 0usize;
    while target.document_archive_loads.get(62).is_some_and(|load| !load.terminal()) && turns < 100_000 {
        let demand = PluginApp::maintenance_retirement_demands(&target, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).expect("a quoted maintenance demand");
        PluginApp::maintenance_step(&mut target, plugin_demand_grant(demand)).expect("a maintenance turn");
        turns += 1;
    }
    findings.holds(target.document_archive_loads.get(62).is_some_and(|load| load.terminal()), || format!("the cancelled load did not retire in {turns} maintenance turns without a host poll"));
    let status = PluginApp::poll_document_archive_load(&mut target, 62, &mut crate::app::artifact_app_laws::fixture_identity()).await.expect("the host's next poll");
    findings.same(&status.state, &protocol::DocumentArchiveLoadState::Cancelled, "what the host's next poll finds");
    PluginApp::acknowledge_document_archive_load(&mut target, 62).expect("the cancelled load is acknowledged");
    findings.same(&head(&target), &before, "the document after the cancelled load");
    for app in [&mut source, &mut target] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work()).await;
        close(app);
    }
    assert!(findings.0.is_empty(), "the cancelled load departs in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}
//#endregion ⏳️LoadProgressAndCancel

/// 🕒️ The positions, in applied order, of the document mutations the history lists as not applied while a history edit is open.
async fn pending_positions(app: &mut ToyApp) -> Vec<u64> {
    let patch = app.history_patch(true).await.expect("history patch");
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
        route_step(&mut findings, &mut author, &fixture, step).await;
        publish(&mut findings, &author, &mut author_port, &mut folder, "author").await;
    }

    let mut peer = fresh_program(&peer_actor).await;
    author_edit(&mut peer, &fixture["example"]).await;
    let archive = protocol::decode_document_archive_bytes(folder.archive.as_ref().expect("the folder holds the author's archive")).await.expect("the folder's archive decodes");
    let (loaded, _) = load_through_the_host(&mut peer, archive).await;
    findings.same(&loaded, &protocol::DocumentArchiveLoadOutcome::Ready, "the peer loads the folder's archive at its attach");
    let mut peer_port = bind_document_port(&mut peer, "folder-two-peers-peer").await;
    findings.same(&head(&peer), &head(&author), "the peer shows the author's document");

    for step in scenario["session"].as_array().expect("session steps") {
        route_step(&mut findings, &mut author, &fixture, step).await;
    }
    let preview = render_body(&mut author).await;
    findings.holds(preview.contains(text(&expected["previewBody"])), || format!("the author's preview while editing reads {preview}, expected {}", expected["previewBody"]));
    let target = author.time_travel.editor().map(|editor| editor.value.clone());

    author_edit(&mut peer, &scenario["peerEdit"]).await;
    let written = publish(&mut findings, &peer, &mut peer_port, &mut folder, "peer").await;
    findings.same(&written.len(), &1, "the peer's edit is one published batch, saved to the folder");

    let generation = author.store.generation();
    let (outcome, merged, ahead) = merge_through_the_host(&mut author, &folder).await;
    findings.same(&outcome, &protocol::DocumentArchiveLoadOutcome::Ready, "the author's read-back merges");
    findings.same(&(merged, ahead), &expected_merge(&expected["authorMerge"]), "the author's read-back (merged, ahead)");
    findings.holds(author.store.generation() != generation, || "the merge moved nothing the author's open history edit could see".to_string());
    let status = author.time_travel.status();
    findings.holds(status.as_ref().is_some_and(|status| serde_json::to_value(status.stage).ok() == Some(Value::from("editing"))), || format!("the author's history edit did not stay open in editing: {:?}", status.map(|status| status.stage)));
    findings.same(&author.time_travel.editor().map(|editor| editor.value.clone()), &target, "the author's draft after the read-back");
    let preview = render_body(&mut author).await;
    findings.holds(preview.contains(text(&expected["previewBody"])), || format!("the read-back changed the author's preview to {preview}, expected {}", expected["previewBody"]));
    findings.same(&pending_positions(&mut author).await, &expected["pendingWhileEditing"].as_array().expect("pending").iter().map(|position| position.as_u64().expect("position")).collect(), "the mutations listed as not applied while editing, the peer's included");
    findings.holds(author.store.backbone_ref().is_some(), || "the read-back left the author without a bound document port".to_string());
    findings.holds(author_port.receive().await.expect("the author's port drains").iter().all(|message| matches!(message, BackboneMessage::Ack { .. })), || "the read-back made the author publish although it holds nothing the folder lacks".to_string());

    for step in scenario["finish"].as_array().expect("finish steps") {
        route_step(&mut findings, &mut author, &fixture, step).await;
        let batches = publish(&mut findings, &author, &mut author_port, &mut folder, "author").await;
        findings.same(&batches.len(), &usize::from(step.get("commit").is_some()), &format!("batches the author published by step {step}"));
    }
    let author_view = route_view(&mut author).await;
    hold_settled(&mut findings, &author_view, &expected["settled"], "the author");

    let peer_line = route_view(&mut peer).await.line;
    let (outcome, merged, ahead) = merge_through_the_host(&mut peer, &folder).await;
    findings.same(&outcome, &protocol::DocumentArchiveLoadOutcome::Ready, "the peer's read-back merges");
    findings.same(&(merged, ahead), &expected_merge(&expected["peerMerge"]), "the peer's read-back (merged, ahead)");
    let peer_view = route_view(&mut peer).await;
    hold_settled(&mut findings, &peer_view, &expected["settled"], "the peer");
    findings.same(&peer_view.line, &peer_line, "the peer's viewed alternative after the read-back");
    let (_, merged, ahead) = merge_through_the_host(&mut author, &folder).await;
    findings.same(&(merged, ahead), &(0, 0), "a further read-back of the author takes nothing and leaves it ahead of nothing");

    drop((author_port, peer_port));
    for app in [&mut author, &mut peer] {
        pump_until(app, "the program settles", |app| !app.time_travel.has_pending_work()).await;
        close(app);
    }
    assert!(findings.0.is_empty(), "two programs on one folder depart from the fixture in {} place(s):\n{}", findings.0.len(), findings.0.join("\n"));
}
