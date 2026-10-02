//! ⏪️ The cross-plugin history-edit acceptance law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §16.3, gap G12):
//! for a representative editable leaf of any app, read from the app's own committed mutation fixtures (on their own documents,
//! else on the documents the app ships: its initial document and its examples), the real verbs run
//! `historyEditBegin` → `historyEditInput` (a schema-valid change derived from the leaf's input descriptors) → `historyEditAccept`
//! → the Report replay → `historyEditFinalize` → `historyEditCommit` as an overwrite and, on a second instance, as a new
//! alternative; each head must equal a fresh fold of the edited log, and the edited mutation's history row must carry its leaf
//! label in every locale. One macro call, [`history_edit_acceptance_law!`](crate::history_edit_acceptance_law), wires it into a
//! plugin crate's test target; every failure names the plugin, the leaf and the fixture case.

use super::*;
use semio_framework::kernel::{HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework::{ActionArgDef, ArgSchema, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID};

/// 🧑‍💻️ The actor every acceptance instance authors and dispatches as.
const ACCEPTANCE_ACTOR: &str = "history-edit-acceptance";
/// 🌿️ The alternative a new-alternative finalize names.
const ACCEPTANCE_ALTERNATIVE: &str = "History edit acceptance";
/// 🎛️ Changes tried per leaf before it counts as having no derivable schema-valid change.
const ACCEPTANCE_CHANGES_PER_LEAF: usize = 16;
/// 🌿️ Downstream operations tried per leaf before the leaf runs alone.
const ACCEPTANCE_DOWNSTREAM_PER_LEAF: usize = 2;

/// 🧫️ One case the law can seed: where it comes from, the DSL text of the document before it, the key of that document (cases
/// sharing it can follow each other downstream) and its operation.
pub struct AcceptanceCase<Mu> {
    pub directory: String,
    pub base: String,
    pub base_key: String,
    pub op: Mu,
}

/// ⚖️ The outcome of one acceptance scenario: it passed (with its summary), its data cannot exercise the flow (`Skip`: the next
/// variant is tried; `SkipCase`: the case's own operation does not seed, so no variant of it can; `SkipBase`: its document does not
/// load, so no case on it can) or the generic mechanism broke (`Fail`, the law fails at once).
enum AcceptanceVerdict {
    Pass(String),
    Skip(String),
    SkipCase(String),
    SkipBase(String),
    Fail(String),
}

/// 🌱️ Why a document cannot be seeded: its base does not load, or the operation at that index of the seed does not apply cleanly.
enum AcceptanceSeedFault {
    Base(String),
    Operation(usize, String),
}

impl AcceptanceSeedFault {
    /// 💬️ The fault in words.
    fn reason(&self) -> String {
        match self {
            Self::Base(reason) => reason.clone(),
            Self::Operation(index, reason) => format!("seed operation #{index}: {reason}"),
        }
    }
}

/// 🔍️ Where two document texts first disagree (line number and both lines, clipped), or that only their values disagree.
fn acceptance_text_difference(left: &str, right: &str) -> String {
    if left == right {
        return "the canonical value (the document texts agree)".into();
    }
    let clip = |line: Option<&str>| line.map(|line| line.chars().take(240).collect::<String>());
    let (mut lefts, mut rights) = (left.lines(), right.lines());
    let mut number = 1;
    loop {
        match (lefts.next(), rights.next()) {
            (Some(one), Some(other)) if one == other => number += 1,
            (one, other) => return format!("document text line {number}: {:?} vs {:?}", clip(one), clip(other)),
        }
    }
}

/// 🗂️ Every directory under `root` that commits a mutation wire (`🦠️mutation/🔣️.json`), in path order.
fn acceptance_fixture_directories(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    fn walk(directory: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if !path.is_dir() || name.starts_with('.') || ["target", "dist", "node_modules", "🗑️generated"].contains(&name.as_str()) {
                continue;
            }
            if path.join("🦠️mutation").join("🔣️.json").is_file() {
                found.push(path.clone());
            }
            walk(&path, found);
        }
    }
    let mut directories = Vec::new();
    walk(root, &mut directories);
    directories.sort();
    directories
}

/// 📄️ The JSON document at `path` as a DSL value.
fn acceptance_json(path: &std::path::Path) -> Option<DslValue> {
    let text = std::fs::read_to_string(path).ok()?;
    dsl::os_pack::json::parse(&text).ok().map(|json| dsl::os_pack::json::to_dsl_value(&json))
}

/// 📇️ Whether a committed wire is a `{mutation, before, after}` case record (which carries its own document).
fn acceptance_record(wire: &DslValue) -> bool {
    wire.get("before").is_some() && wire.get("after").is_some() && wire.get("mutation").and_then(DslValue::as_object).is_some()
}

/// 🦠️ The operation a committed wire names, when it decodes as `A::Mutation` and the generic editor edits it (an input schema
/// and no foreign-step capability).
fn acceptance_operation<A: ArtifactApp>(wire: &DslValue) -> Option<A::Mutation> {
    let value = if acceptance_record(wire) { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire.clone() };
    let op = <A::Mutation as protocol::FromValue>::from_value(value).ok()?;
    if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none() || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
        return None;
    }
    Some(op)
}

/// 🗣️ The DSL text of a document given as a JSON value; the snapshot is never dropped, so a fail-closed root never trips.
fn acceptance_dsl_of<A: ArtifactApp>(document: DslValue) -> Option<String> {
    let snapshot = <A::Snapshot as protocol::FromValue>::from_value(document).ok()?;
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    std::mem::forget(snapshot);
    Some(text)
}

/// 🧫️ Every committed fixture case under `root` whose outcome is no refusal (applied, or a no-op an edit can turn into a change),
/// whose document is given (`📸️snapshot/⬅️before/🗣️.dsl.semio` verbatim — the only form a composed parent's owned children
/// survive in — else its JSON twin decoded as `A::Snapshot`) and whose operation the generic editor edits — in path order. A
/// `{mutation, before, after}` case record carries its own document.
pub fn acceptance_cases<A: ArtifactApp>(root: &std::path::Path) -> Vec<AcceptanceCase<A::Mutation>> {
    let mut cases = Vec::new();
    for directory in acceptance_fixture_directories(root) {
        let Some(wire) = acceptance_json(&directory.join("🦠️mutation").join("🔣️.json")) else { continue };
        let record = acceptance_record(&wire);
        let applied = record || acceptance_json(&directory.join("🎯️outcome").join("🔣️.json")).is_some_and(|outcome| outcome.get("status").and_then(DslValue::as_str).is_some_and(|status| status != "rejected"));
        if !applied {
            continue;
        }
        let before = directory.join("📸️snapshot").join("⬅️before");
        let base = match std::fs::read_to_string(before.join("🗣️.dsl.semio")) {
            Ok(text) => Some(text),
            Err(_) => (if record { wire.get("before").cloned() } else { acceptance_json(&before.join("🔣️.json")) }).and_then(acceptance_dsl_of::<A>),
        };
        let Some(base) = base else { continue };
        let Some(op) = acceptance_operation::<A>(&wire) else { continue };
        cases.push(AcceptanceCase { directory: directory.to_string_lossy().into_owned(), base_key: base.clone(), base, op });
    }
    cases
}

/// 📚️ The derived cases the law falls back to when no committed case exercises the flow: every committed operation under `root`
/// (whatever its own outcome — a refusal on its fixture document may apply elsewhere) on every document the app itself ships —
/// its initial document and each of `examples` (a DSL body verbatim, a JSON body decoded as `A::Snapshot`). Cases on one
/// document share its key, so they can follow each other downstream.
pub async fn acceptance_example_cases<A: ArtifactApp>(root: &std::path::Path, examples: &[ExampleSource]) -> Vec<AcceptanceCase<A::Mutation>> {
    let initial = A::initial_snapshot().await;
    let mut documents = vec![("initial".to_string(), store::ArtifactDsl::print_dsl(&initial))];
    std::mem::forget(initial);
    for example in examples {
        let body = example.document();
        let text = if body.trim_start().starts_with('{') { dsl::os_pack::json::parse(&body).ok().and_then(|json| acceptance_dsl_of::<A>(dsl::os_pack::json::to_dsl_value(&json))) } else { Some(body) };
        if let Some(text) = text.filter(|text| documents.iter().all(|(_, known)| known != text)) {
            documents.push((format!("example {}", example.id()), text));
        }
    }
    let operations: Vec<(String, A::Mutation)> = acceptance_fixture_directories(root).into_iter().filter_map(|directory| Some((directory.to_string_lossy().into_owned(), acceptance_operation::<A>(&acceptance_json(&directory.join("🦠️mutation").join("🔣️.json"))?)?))).collect();
    let mut cases = Vec::new();
    for (name, base) in &documents {
        for (directory, op) in &operations {
            cases.push(AcceptanceCase { directory: format!("{directory} on the {name} document"), base: base.clone(), base_key: name.clone(), op: op.clone() });
        }
    }
    for (_, op) in operations {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
    }
    cases
}

/// 🎛️ The schema-valid changes the law tries for one draft, most robust first: every number input moved by its step (then to
/// its bounds and their midpoint), every boolean flipped, every option switched, every vector's first axis moved, every free text
/// extended — each a JSON pointer and the value to draft there. Reference, array and opaque inputs are never changed.
pub fn acceptance_changes(inputs: &[ActionArgDef], value: &DslValue) -> Vec<(String, DslValue)> {
    fn at<'a>(value: &'a DslValue, pointer: &str) -> Option<&'a DslValue> {
        pointer.split('/').skip(1).try_fold(value, |current, segment| current.get(&segment.replace("~1", "/").replace("~0", "~")))
    }
    fn numbers(current: Option<f64>, min: Option<f64>, max: Option<f64>, step: Option<f64>, integer: bool) -> Vec<f64> {
        let step = step.filter(|step| *step > 0.0).map_or(1.0, |step| if integer { step.round().max(1.0) } else { step });
        let base = current.unwrap_or_else(|| min.unwrap_or(0.0));
        let mut values = vec![base + step, base - step];
        values.extend(max);
        values.extend(min);
        values.extend(min.zip(max).map(|(min, max)| (min + max) / 2.0));
        let mut kept: Vec<f64> = Vec::new();
        for value in values.into_iter().map(|value| if integer { value.round() } else { value }) {
            if value.is_finite() && Some(value) != current && min.is_none_or(|min| value >= min) && max.is_none_or(|max| value <= max) && !kept.contains(&value) {
                kept.push(value);
            }
        }
        kept
    }
    fn collect(inputs: &[ActionArgDef], value: &DslValue, prefix: &str, buckets: &mut [Vec<(String, DslValue)>; 5]) {
        for input in inputs {
            let pointer = format!("{prefix}{}", input.id);
            let current = at(value, &pointer);
            match &input.schema {
                ArgSchema::Number { min, max, step, integer, .. } => {
                    let integer = *integer;
                    buckets[0].extend(numbers(current.and_then(DslValue::as_f64), *min, *max, *step, integer).into_iter().map(|number| (pointer.clone(), if integer { DslValue::int(number as i64) } else { DslValue::float(number) })));
                }
                ArgSchema::Boolean => buckets[1].push((pointer.clone(), DslValue::Bool(!current.and_then(DslValue::as_bool).unwrap_or(false)))),
                ArgSchema::String { options, .. } if !options.is_empty() => {
                    let selected = current.and_then(DslValue::as_str);
                    buckets[2].extend(options.iter().filter(|option| Some(option.value.as_str()) != selected).map(|option| (pointer.clone(), DslValue::String(option.value.clone()))));
                }
                ArgSchema::Vector { min, max, step, .. } => {
                    if let Some(axes) = current.and_then(DslValue::as_array).filter(|axes| !axes.is_empty()) {
                        for first in numbers(axes[0].as_f64(), *min, *max, *step, false) {
                            let mut moved = axes.to_vec();
                            moved[0] = DslValue::float(first);
                            buckets[3].push((pointer.clone(), DslValue::Array(moved)));
                        }
                    }
                }
                ArgSchema::String { pattern: None, format: None, .. } => {
                    if let Some(text) = current.and_then(DslValue::as_str) {
                        buckets[4].push((pointer.clone(), DslValue::String(format!("{text} edited"))));
                    }
                }
                ArgSchema::Object { fields } => collect(fields, value, &pointer, buckets),
                _ => {}
            }
        }
    }
    let mut buckets: [Vec<(String, DslValue)>; 5] = Default::default();
    collect(inputs, value, "", &mut buckets);
    buckets.into_iter().flatten().take(ACCEPTANCE_CHANGES_PER_LEAF).collect()
}

/// 🪪️ The acceptance actor's action meta on the registered instance.
fn acceptance_meta() -> ActionMeta {
    ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(ACCEPTANCE_ACTOR) }
}

/// ⏯️ Runs one framework history-edit verb; `Err` carries its refusal code or fault.
async fn acceptance_verb<A, M>(app: &mut VcsArtifactApp<A, M>, action: &str, args: Vec<(&str, DslValue)>) -> Result<(), String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let args = DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let result = app.handle_action(action, Some(&args), &acceptance_meta()).await.map_err(|fault| format!("{action} faulted: {fault:?}"))?;
    match result.output.get("rejected").and_then(DslValue::as_str) {
        Some(code) => Err(format!("{action} was refused: {code}")),
        None => Ok(()),
    }
}

/// ⏱️ Drives reactor turns until `done` holds for the instance's session, at most 60 s.
async fn acceptance_pump<A, M>(app: &mut VcsArtifactApp<A, M>, done: impl Fn(&VcsArtifactApp<A, M>) -> bool) -> Result<(), String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return Ok(());
        }
        app.advance_typed_operation_publication().await.map_err(|fault| format!("a driver turn faulted: {fault:?}"))?;
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    Err(format!("the session never settled; it rests at {:?}", app.time_travel.status().map(|status| status.stage)))
}

/// 🧹️ Lets the session retire its owners, then closes the instance to its terminal-empty witness.
async fn acceptance_close<A, M>(app: &mut VcsArtifactApp<A, M>)
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let _ = acceptance_pump(app, |app| !app.time_travel.has_pending_work()).await;
    artifact_app_laws::close_registered_fixture_app(app);
}

/// 🌱️ A registered instance whose document is `base` (loaded through the document text path) followed by one clean edit per op
/// of `ops` (each leaving an applied operation when `strict`); `Err` names why the data cannot be seeded (the base, or the index
/// of the first op that does not apply cleanly), after closing the instance.
async fn acceptance_seeded<A, M>(manifest: fn() -> App, base: &str, ops: &[&A::Mutation], strict: bool) -> Result<VcsArtifactApp<A, M>, AcceptanceSeedFault>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        app.store.set_local_actor_id(Some(ACCEPTANCE_ACTOR.to_string())).map_err(|error| AcceptanceSeedFault::Base(format!("the actor is refused: {error:?}")))?;
        let mut files = app.document_text().await.map_err(|fault| AcceptanceSeedFault::Base(format!("the document does not print: {fault:?}")))?;
        files.dsl = base.to_string();
        app.load_document_text(&files).await.map_err(|fault| AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}")))?;
        for (index, op) in ops.iter().enumerate() {
            let applied = app.store.mutation_ops().map_or(0, |applied| applied.len());
            let receipt = app.store.dispatch(ArtifactCommand::Apply { mutations: vec![(*op).clone()], description: None, transaction: None }).await.map_err(|error| AcceptanceSeedFault::Operation(index, format!("the seed edit is refused: {error:?}")))?;
            if matches!(receipt.worst, Some(dsl::Severity::Error | dsl::Severity::Fatal)) {
                return Err(AcceptanceSeedFault::Operation(index, format!("the seed edit does not apply cleanly: {:?}", receipt.messages)));
            }
            if strict && app.store.mutation_ops().map_or(0, |applied| applied.len()) <= applied {
                return Err(AcceptanceSeedFault::Operation(index, "the seed edit leaves no applied operation".into()));
            }
        }
        app.refresh_cache().await.map_err(|fault| AcceptanceSeedFault::Operation(ops.len().saturating_sub(1), format!("the history does not backfill: {fault:?}")))
    }
    .await;
    match seeded {
        Ok(()) => Ok(app),
        Err(reason) => {
            acceptance_close(&mut app).await;
            Err(reason)
        }
    }
}

/// 🧾️ The document head as its canonical value and its DSL text (the text also carries what a composed parent's value form
/// only references — its owned children); the clone is never dropped, so a fail-closed root never trips.
fn acceptance_head<A, M>(app: &VcsArtifactApp<A, M>) -> Result<(DslValue, String), String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let head = app.store.snapshot().map_err(|error| format!("the head does not fold: {error:?}"))?;
    let value = (protocol::ToValue::to_value(&head), store::ArtifactDsl::print_dsl(&head));
    std::mem::forget(head);
    Ok(value)
}

/// ✏️ One session on `app` over its first mutation: begin, draft `change` (else the first change of
/// [`acceptance_changes`] the editor accepts), accept, replay to a clean review, finalize, and commit as an overwrite or as the
/// new alternative `alternative`. Answers the change drafted and the edited operation the store now folds.
async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let target = app.store.mutation_ops().map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?.first().map(|op| op.mutation_id.0.clone()).ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, vec![("mutationId", DslValue::String(target.clone()))]).await.map_err(AcceptanceVerdict::Fail)?;
    let (inputs, original) = match app.time_travel.editor() {
        None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
        Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail(format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused))),
        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    let changes = change.map_or_else(|| acceptance_changes(&inputs, &original), |change| vec![change.clone()]);
    let mut drafted = None;
    for candidate in changes {
        if acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_ok() && app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original) {
            drafted = Some(candidate);
            break;
        }
    }
    let Some(drafted) = drafted else {
        let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        return Err(AcceptanceVerdict::Skip(format!("no schema-valid change of its {} input(s) is accepted", inputs.len())));
    };
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
    acceptance_pump(app, |app| app.time_travel.status().is_some_and(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting closed the session".into()))?;
    if status.review == Some(HistoryTimeTravelReview::Blocked) {
        let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        return Err(AcceptanceVerdict::Skip(format!("drafting {} = {:?} blocks the replay ({:?})", drafted.0, drafted.1, status.worst)));
    }
    if (status.stage, status.review) != (HistoryTimeTravelStage::Reviewing, Some(HistoryTimeTravelReview::Ready)) {
        return Err(AcceptanceVerdict::Fail(format!("the replay ends at {:?}/{:?}, not a ready review", status.stage, status.review)));
    }
    acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
    let commit = match alternative {
        Some(name) => vec![("name", DslValue::String(name.to_string()))],
        None => vec![("choice", DslValue::String(HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))],
    };
    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, commit).await.map_err(AcceptanceVerdict::Fail)?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    let (_, supersession) = app.store.supersessions().iter().find(|(id, _)| id.0 == target).ok_or_else(|| AcceptanceVerdict::Fail("the finalize superseded nothing".into()))?;
    let scope = supersession.scope.clone();
    let protocol::InputReplacement::Input { payload, .. } = &supersession.replacement else {
        return Err(AcceptanceVerdict::Fail("the finalize withdrew the edited operation".into()));
    };
    let edited = <A::Mutation as ::protocol::OpBinary>::decode_op(payload).map_err(|error| AcceptanceVerdict::Fail(format!("the superseding input does not decode: {error:?}")))?;
    let envelope = app.store.envelope();
    let expected = alternative.map(|name| envelope.vcs.alternatives.iter().find(|candidate| candidate.name == name).map(|candidate| candidate.id.clone()));
    match expected {
        None if scope.is_some() => return Err(AcceptanceVerdict::Fail(format!("the overwrite is scoped to {scope:?}"))),
        Some(None) => return Err(AcceptanceVerdict::Fail("the new alternative is not listed".into())),
        Some(Some(id)) if envelope.active_alternative_id.as_deref() != Some(id.as_str()) || scope.as_deref() != Some(id.as_str()) => return Err(AcceptanceVerdict::Fail(format!("the new alternative {id} is not active and scoping the supersession ({scope:?})"))),
        _ => {}
    }
    Ok((drafted, edited))
}

/// 🏷️ The edited mutation's history row names it in every locale with its leaf label, never with its text line.
async fn acceptance_row_label<A, M>(app: &mut VcsArtifactApp<A, M>, edited: &A::Mutation) -> Result<String, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let target = app.store.mutation_ops().map_err(|error| format!("{error:?}"))?.first().map(|op| op.mutation_id.0.clone()).unwrap_or_default();
    let patch = app.history_patch(true).await.map_err(|fault| format!("the history does not project: {fault:?}"))?;
    let row = patch.upserts.iter().flat_map(|entry| entry.mutations.iter()).find(|mutation| mutation.mutation_id == target).ok_or_else(|| "the edited mutation has no history row".to_string())?;
    let (en, de) = (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string());
    if en.trim().is_empty() || de.trim().is_empty() || en == edited.print_op() || de == edited.print_op() {
        return Err(format!("the edited mutation's row label is {en:?} / {de:?}, not its leaf label in every locale"));
    }
    Ok(format!("{en} / {de}"))
}

/// ⚖️ One full scenario for `case` (with an optional `downstream` edit after it): overwrite on one instance, new alternative on
/// another, both heads against a fresh fold of the edited log, the edited row's label in every locale.
async fn acceptance_scenario<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, downstream: Option<&A::Mutation>) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let seed: Vec<&A::Mutation> = std::iter::once(&case.op).chain(downstream).collect();
    let mut overwrite = match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
        Ok(app) => app,
        Err(AcceptanceSeedFault::Base(reason)) => return AcceptanceVerdict::SkipBase(reason),
        Err(AcceptanceSeedFault::Operation(0, reason)) => return AcceptanceVerdict::SkipCase(reason),
        Err(AcceptanceSeedFault::Operation(_, reason)) => return AcceptanceVerdict::Skip(format!("the downstream edit does not seed: {reason}")),
    };
    let session = acceptance_session(&mut overwrite, None, None).await;
    let (change, edited) = match session {
        Ok(done) => done,
        Err(verdict) => {
            acceptance_close(&mut overwrite).await;
            return verdict;
        }
    };
    let label = acceptance_row_label(&mut overwrite, &edited).await;
    let head = acceptance_head(&overwrite);
    acceptance_close(&mut overwrite).await;
    let edited_seed: Vec<&A::Mutation> = std::iter::once(&edited).chain(downstream).collect();
    let mut fresh = match acceptance_seeded::<A, M>(manifest, &case.base, &edited_seed, false).await {
        Ok(app) => app,
        Err(fault) => {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
            return AcceptanceVerdict::Fail(format!("a fresh fold of the edited log does not seed: {}", fault.reason()));
        }
    };
    let fold = acceptance_head(&fresh);
    acceptance_close(&mut fresh).await;
    let verdict = match (label, head, fold) {
        (Err(reason), _, _) | (_, Err(reason), _) | (_, _, Err(reason)) => AcceptanceVerdict::Fail(reason),
        (Ok(_), Ok(head), Ok(fold)) if head.0 != fold.0 => AcceptanceVerdict::Fail(format!("the overwrite head differs from a fresh fold of the edited log after {} = {:?}", change.0, change.1)),
        (Ok(_), Ok(head), Ok(fold)) if head.1 != fold.1 => AcceptanceVerdict::Fail(format!("the overwrite head's document text differs from a fresh fold of the edited log after {} = {:?} at {}", change.0, change.1, acceptance_text_difference(&head.1, &fold.1))),
        (Ok(label), Ok(_), Ok(fold)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
            Err(fault) => AcceptanceVerdict::Fail(format!("the alternative instance does not seed: {}", fault.reason())),
            Ok(mut branched) => {
                let verdict = match acceptance_session(&mut branched, Some(&change), Some(ACCEPTANCE_ALTERNATIVE)).await {
                    Err(AcceptanceVerdict::Skip(reason)) => AcceptanceVerdict::Fail(format!("the change the overwrite accepted fails as a new alternative: {reason}")),
                    Err(verdict) => verdict,
                    Ok((_, alternative_edit)) => {
                        let head = acceptance_head(&branched);
                        ::protocol::Mutation::<A::Snapshot>::retire_cold(alternative_edit);
                        match head {
                            Ok(head) if head == fold => AcceptanceVerdict::Pass(format!("{} = {:?}, row \"{label}\"{}", change.0, change.1, if downstream.is_some() { ", one downstream edit replayed" } else { "" })),
                            Ok(head) => AcceptanceVerdict::Fail(format!("the new alternative's head differs from a fresh fold of the edited log at {}", acceptance_text_difference(&head.1, &fold.1))),
                            Err(reason) => AcceptanceVerdict::Fail(reason),
                        }
                    }
                };
                acceptance_close(&mut branched).await;
                verdict
            }
        },
    };
    ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
    verdict
}

/// 🔁️ Runs `cases` in order until one passes: each case with up to [`ACCEPTANCE_DOWNSTREAM_PER_LEAF`] downstream edits from cases
/// on the same document, then alone; a case whose own operation does not seed, and every case on a document that does not load,
/// is skipped at once. `Ok(Some)` names the passing case, `Ok(None)` means every case was skipped (reasons appended to `skipped`),
/// `Err` is a broken mechanism.
async fn acceptance_first_pass<A, M>(plugin: &str, manifest: fn() -> App, cases: &[AcceptanceCase<A::Mutation>], skipped: &mut Vec<String>) -> Result<Option<String>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut unloadable = BTreeSet::new();
    for case in cases {
        if unloadable.contains(&case.base_key) {
            continue;
        }
        let leaf = protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
        let downstream: Vec<&A::Mutation> = cases.iter().filter(|other| other.base_key == case.base_key && other.directory != case.directory).map(|other| &other.op).take(ACCEPTANCE_DOWNSTREAM_PER_LEAF).collect();
        for next in downstream.into_iter().map(Some).chain(std::iter::once(None)) {
            match acceptance_scenario::<A, M>(manifest, case, next).await {
                AcceptanceVerdict::Pass(summary) => return Ok(Some(format!("{plugin}: {leaf} ({}) — {summary}", case.directory))),
                AcceptanceVerdict::Skip(reason) => skipped.push(format!("{leaf} ({}): {reason}", case.directory)),
                AcceptanceVerdict::SkipCase(reason) => {
                    skipped.push(format!("{leaf} ({}): {reason}", case.directory));
                    break;
                }
                AcceptanceVerdict::SkipBase(reason) => {
                    skipped.push(format!("{leaf} ({}): {reason}", case.directory));
                    unloadable.insert(case.base_key.clone());
                    break;
                }
                AcceptanceVerdict::Fail(reason) => return Err(format!("{plugin}: the history edit of {leaf} ({}) breaks the generic mechanism: {reason}", case.directory)),
            }
        }
    }
    Ok(None)
}

/// ⚖️ LAW (design §16.3): a representative editable leaf of the app is edited in history end to end through the generic
/// mechanism — overwrite and new alternative, each head (value and document text) a fresh fold of the edited log, the row
/// labelled in every locale. The committed fixture cases under `fixtures` are tried first ([`acceptance_cases`]); when none
/// exercises the flow, every committed operation on every document the app ships ([`acceptance_example_cases`] over its initial
/// document and `examples`). The first passing case is the representative; a broken mechanism fails at once. Answers the passing
/// case's summary; panics naming `plugin`, the leaf and the case otherwise.
pub async fn assert_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut skipped = Vec::new();
    let committed = acceptance_cases::<A>(fixtures);
    let mut counts = vec![committed.len()];
    let mut outcome = acceptance_first_pass::<A, M>(plugin, manifest, &committed, &mut skipped).await;
    for case in committed {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    if matches!(outcome, Ok(None)) {
        let derived = acceptance_example_cases::<A>(fixtures, examples).await;
        counts.push(derived.len());
        outcome = acceptance_first_pass::<A, M>(plugin, manifest, &derived, &mut skipped).await;
        for case in derived {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
        }
    }
    match outcome {
        Ok(Some(summary)) => summary,
        Err(failure) => panic!("{failure}"),
        Ok(None) => {
            let shown = skipped.len().min(40);
            panic!("{plugin}: none of the {} committed and {} derived editable case(s) under {} exercises a history edit end to end ({} skip(s), first {shown}):\n{}", counts[0], counts.get(1).copied().unwrap_or(0), fixtures.display(), skipped.len(), skipped[..shown].join("\n"))
        }
    }
}

/// 🔗️ Every breach of complete input-schema resolution among `A`'s document leaves, through the resolver the history editor uses
/// (`registered_input_schema_document`): a leaf whose input descriptors the reader refuses, a `$id` its payload schema references
/// (transitively) that nothing published, or a payload validator that does not compile. One line per breach, naming the leaf.
pub fn input_schema_resolution_failures<A: ArtifactApp>() -> Vec<String> {
    fn references(value: &DslValue, into: &mut Vec<String>) {
        match value {
            DslValue::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", DslValue::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => references(value, into),
                    }
                }
            }
            DslValue::Array(items) => items.iter().for_each(|item| references(item, into)),
            _ => {}
        }
    }
    let descriptors = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS;
    let schemas = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS;
    let mut failures = Vec::new();
    for (descriptor, schema) in descriptors.iter().zip(schemas) {
        let kind = descriptor.semantic_kind;
        if let Err(error) = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document) {
            failures.push(format!("{kind}: the input reader refuses its payload schema: {error:?}"));
        }
        let Ok(root) = dsl::os_pack::json::parse(schema) else {
            failures.push(format!("{kind}: its payload schema is not JSON"));
            continue;
        };
        let mut pending = Vec::new();
        references(&dsl::os_pack::json::to_dsl_value(&root), &mut pending);
        let (mut seen, mut documents, mut unresolved) = (BTreeSet::new(), Vec::new(), Vec::new());
        while let Some(id) = pending.pop() {
            if id.is_empty() || !seen.insert(id.clone()) {
                continue;
            }
            match semio_framework::registered_input_schema_document(&id) {
                Some(document) => {
                    references(&document, &mut pending);
                    documents.push(dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(&document)));
                }
                None => unresolved.push(id),
            }
        }
        if !unresolved.is_empty() {
            failures.push(format!("{kind}: nothing publishes the referenced schema document(s) {unresolved:?}"));
        }
        if let Err(error) = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()) {
            failures.push(format!("{kind}: its payload validator does not compile: {error}"));
        }
    }
    failures
}

/// ⚖️ LAW (design §16.3, the editable-everything gate at runtime): on a registered instance — which publishes every document its
/// leaves reference, as in production — every document leaf's payload schema resolves completely. Answers the number of leaves
/// read; panics naming `plugin` and every breaching leaf otherwise.
pub async fn assert_input_schemas_resolve<A, M>(plugin: &str, manifest: fn() -> App) -> usize
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
    let failures = input_schema_resolution_failures::<A>();
    acceptance_close(&mut app).await;
    assert!(failures.is_empty(), "{plugin}: {} leaf payload schema(s) do not resolve through the runtime resolver:\n{}", failures.len(), failures.join("\n"));
    <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS.len()
}

/// ⏳️ Polls the law's future to completion on the calling thread, yielding between polls — the dependency-free executor every
/// fixture app is driven by, so a plugin crate needs no async test macro to run the law.
pub fn block_on_acceptance<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

/// ⏪️ Wires [`assert_history_edits_end_to_end`] into a plugin crate's test target for one editor: `plugin` names the failure,
/// `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$fixtures` the fixture root relative to the crate's
/// manifest directory (the artifact tree: `"../.."`).
#[macro_export]
macro_rules! history_edit_acceptance_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $fixtures:literal) => {
        /// ⏪️ LAW (design §16.3): a representative editable leaf of this editor is edited in history end to end.
        #[test]
        fn history_edits_end_to_end() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest, &fixtures, &examples));
            ::std::println!("[history-edit-acceptance] {summary}");
        }

        /// 🔗️ LAW (design §16.3): every document leaf payload schema of this editor resolves through the runtime resolver.
        #[test]
        fn history_edit_inputs_resolve() {
            let leaves = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_input_schemas_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest));
            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve", $plugin);
        }
    };
}
