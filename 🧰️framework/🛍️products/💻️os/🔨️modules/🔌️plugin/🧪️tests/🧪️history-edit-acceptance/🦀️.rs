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
use semio_framework::{
    ActionArgDef, ArgSchema, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID,
    HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID,
};

/// 🧑‍💻️ The actor every acceptance instance authors and dispatches as.
const ACCEPTANCE_ACTOR: &str = "history-edit-acceptance";
/// 🌿️ The alternative a new-alternative finalize names.
const ACCEPTANCE_ALTERNATIVE: &str = "History edit acceptance";
/// 🔢️ Changes tried per leaf before it counts as having no derivable schema-valid change.
const ACCEPTANCE_CHANGES_PER_LEAF: usize = 16;
/// 🌊️ Downstream operations tried per leaf before the leaf runs alone.
const ACCEPTANCE_DOWNSTREAM_PER_LEAF: usize = 2;

/// 🧫️ One case the law can seed: where it comes from, the DSL text of the document before it, the key of that document (cases
/// sharing it can follow each other downstream) and its operation.
pub struct AcceptanceCase<Mu> {
    pub directory: String,
    pub base: String,
    pub base_key: String,
    pub op: Mu,
}

/// 🏁️ The outcome of one acceptance scenario: it passed (with its summary), its data cannot exercise the flow (`Skip`: the next
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

impl AcceptanceVerdict {
    /// 🗯️ The verdict's reason or summary in words.
    fn text(self) -> String {
        match self {
            Self::Pass(text) | Self::Skip(text) | Self::SkipCase(text) | Self::SkipBase(text) | Self::Fail(text) => text,
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
    semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().map(|json| semio_framework_pack_json::to_dsl_value(&json))
}

/// 📇️ Whether a committed wire is a `{mutation, before, after}` case record (which carries its own document).
fn acceptance_record(wire: &DslValue) -> bool {
    wire.get("before").is_some() && wire.get("after").is_some() && wire.get("mutation").and_then(DslValue::as_object).is_some()
}

/// 🦠️ The operation a committed wire names, when it decodes as `A::Mutation` and the generic editor edits it (an input schema
/// and no foreign-step capability).
fn acceptance_operation<A: ArtifactApp>(wire: &DslValue) -> Option<A::Mutation> {
    let value = if acceptance_record(wire) { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire.clone() };
    let op = <A::Mutation as semio_framework_value::FromValue>::from_value(value).ok()?;
    if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none() || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
        return None;
    }
    Some(op)
}

/// 🪆️ Whether a document value names a composed child by its handle (`{childId, target}`): the child's owned content lives
/// outside the value, so a document decoded from the value alone is not materialized and cannot seed an edit.
fn acceptance_composed(value: &DslValue) -> bool {
    match value {
        DslValue::Object(entries) => (entries.len() == 2 && value.get("childId").is_some() && value.get("target").is_some()) || entries.iter().any(|(_, entry)| acceptance_composed(entry)),
        DslValue::Array(items) => items.iter().any(acceptance_composed),
        _ => false,
    }
}

/// 🗣️ The DSL text of a self-contained document given as a JSON value (none for a value that names a composed child — see
/// [`acceptance_composed`]); the snapshot is never dropped, so a fail-closed root never trips.
fn acceptance_dsl_of<A: ArtifactApp>(document: DslValue) -> Option<String> {
    if acceptance_composed(&document) {
        return None;
    }
    let snapshot = <A::Snapshot as semio_framework_value::FromValue>::from_value(document).ok()?;
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    std::mem::forget(snapshot);
    Some(text)
}

/// 🗄️ Every committed fixture case under `root` whose outcome is no refusal (applied, or a no-op an edit can turn into a change),
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
        let text = if body.trim_start().starts_with('{') {
            semio_framework_pack_json::parse(&body, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|json| acceptance_dsl_of::<A>(semio_framework_pack_json::to_dsl_value(&json)))
        } else {
            Some(body)
        };
        if let Some(text) = text.filter(|text| documents.iter().all(|(_, known)| known != text)) {
            documents.push((format!("example {}", example.id()), text));
        }
    }
    let operations: Vec<(String, A::Mutation)> =
        acceptance_fixture_directories(root).into_iter().filter_map(|directory| Some((directory.to_string_lossy().into_owned(), acceptance_operation::<A>(&acceptance_json(&directory.join("🦠️mutation").join("🔣️.json"))?)?))).collect();
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
    ActionMeta { view_state: Some(ViewModel::new(Locale::En, Terminology::Native)), ..artifact_app_laws::meta(ACCEPTANCE_ACTOR) }
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

/// 🪴️ A registered instance whose document is `base` (loaded through the document text path) followed by one clean edit per op
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
        artifact_app_laws::load_document_text(&mut app, &files).await.map_err(|fault| AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}")))?;
        for (index, op) in ops.iter().enumerate() {
            let applied = app.store.mutation_ops().map_or(0, |applied| applied.len());
            let receipt =
                app.store.dispatch(ArtifactCommand::Apply { mutations: vec![(*op).clone()], description: None, transaction: None }).await.map_err(|error| AcceptanceSeedFault::Operation(index, format!("the seed edit is refused: {error:?}")))?;
            if matches!(receipt.worst, Some(semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
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
    let value = (semio_framework_value::ToValue::to_value(&head), store::ArtifactDsl::print_dsl(&head));
    std::mem::forget(head);
    Ok(value)
}

/// 🚗️ One session over the mutation `target` of the member `store` (`<slot>/<childId>`, design §12; `None`: the document's own
/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts), accept, replay to a clean
/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. Answers the change drafted.
async fn acceptance_drive<A, M>(app: &mut VcsArtifactApp<A, M>, target: &str, store: Option<&str>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<(String, DslValue), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.to_string()))];
    begin.extend(store.map(|store| (HISTORY_EDIT_ARG_STORE, DslValue::String(store.to_string()))));
    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin).await.map_err(AcceptanceVerdict::Fail)?;
    let (inputs, original) = match app.time_travel.editor() {
        None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
        Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail(format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused))),
        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    let changes = change.map_or_else(|| acceptance_changes(&inputs, &original), |change| vec![change.clone()]);
    let mut drafted = None;
    for candidate in changes {
        if acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_ok()
            && app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
        {
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
    Ok(drafted)
}

/// ✏️ [`acceptance_drive`] over `app`'s first document mutation, then the store's own account of it: the superseding input decoded
/// as the edited operation, unscoped for an overwrite, scoped to the new active alternative otherwise. Answers the change drafted
/// and the edited operation the store now folds.
async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let target = app
        .store
        .mutation_ops()
        .map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?
        .first()
        .map(|op| op.mutation_id.0.clone())
        .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    let drafted = acceptance_drive(app, &target, None, change, alternative).await?;
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
        Some(Some(id)) if envelope.active_alternative_id.as_deref() != Some(id.as_str()) || scope.as_deref() != Some(id.as_str()) => {
            return Err(AcceptanceVerdict::Fail(format!("the new alternative {id} is not active and scoping the supersession ({scope:?})")))
        }
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

/// 🎭️ One full scenario for `case` (with an optional `downstream` edit after it): overwrite on one instance, new alternative on
/// another, both heads against a fresh fold of the edited log, the edited row's label in every locale. With `reload` both
/// sessions run on the seeded document after it was saved and loaded into a fresh instance ([`acceptance_reloaded`]).
async fn acceptance_scenario<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, downstream: Option<&A::Mutation>, reload: bool) -> AcceptanceVerdict
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
    if reload {
        let reloaded = acceptance_reloaded(&overwrite, manifest).await;
        acceptance_close(&mut overwrite).await;
        overwrite = match reloaded {
            Ok(app) => app,
            Err(reason) => return AcceptanceVerdict::Fail(format!("the seeded document does not reload: {reason}")),
        };
    }
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
    let reloaded_head = acceptance_reloaded_head(&overwrite, manifest).await;
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
    let verdict = match (label, head, fold, reloaded_head) {
        (Err(reason), _, _, _) | (_, Err(reason), _, _) | (_, _, Err(reason), _) | (_, _, _, Err(reason)) => AcceptanceVerdict::Fail(reason),
        (Ok(_), Ok(head), Ok(fold), _) if head.0 != fold.0 => AcceptanceVerdict::Fail(format!("the overwrite head differs from a fresh fold of the edited log after {} = {:?}", change.0, change.1)),
        (Ok(_), Ok(head), Ok(fold), _) if head.1 != fold.1 => {
            AcceptanceVerdict::Fail(format!("the overwrite head's document text differs from a fresh fold of the edited log after {} = {:?} at {}", change.0, change.1, acceptance_text_difference(&head.1, &fold.1)))
        }
        (Ok(_), Ok(_), Ok(fold), Ok(reloaded)) if reloaded != fold => AcceptanceVerdict::Fail(format!(
            "the edited document reloads to a head differing from a fresh fold of the edited log (store initializer ignores the supersession) after {} = {:?} at {}",
            change.0,
            change.1,
            acceptance_text_difference(&reloaded.1, &fold.1)
        )),
        (Ok(label), Ok(_), Ok(fold), Ok(_)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
            Err(fault) => AcceptanceVerdict::Fail(format!("the alternative instance does not seed: {}", fault.reason())),
            Ok(mut branched) if reload => {
                let reloaded = acceptance_reloaded(&branched, manifest).await;
                acceptance_close(&mut branched).await;
                match reloaded {
                    Err(reason) => AcceptanceVerdict::Fail(format!("the alternative instance does not reload: {reason}")),
                    Ok(mut branched) => {
                        let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some()).await;
                        acceptance_close(&mut branched).await;
                        verdict
                    }
                }
            }
            Ok(mut branched) => {
                let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some()).await;
                acceptance_close(&mut branched).await;
                verdict
            }
        },
    };
    ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
    verdict
}

/// 🌳️ The new-alternative half of a scenario on `branched`: the change the overwrite accepted, committed as a new alternative, must
/// reach the same head as the fresh fold `fold`, live and after the document reloads through the app's own store initializer.
async fn acceptance_alternative<A, M>(branched: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, change: &(String, DslValue), fold: &(DslValue, String), label: &str, downstream: bool) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE)).await {
        Err(AcceptanceVerdict::Skip(reason)) => AcceptanceVerdict::Fail(format!("the change the overwrite accepted fails as a new alternative: {reason}")),
        Err(verdict) => verdict,
        Ok((_, alternative_edit)) => {
            let head = acceptance_head(branched);
            let reloaded = acceptance_reloaded_head(branched, manifest).await;
            ::protocol::Mutation::<A::Snapshot>::retire_cold(alternative_edit);
            match (head, reloaded) {
                (Err(reason), _) | (_, Err(reason)) => AcceptanceVerdict::Fail(reason),
                (Ok(head), _) if head != *fold => AcceptanceVerdict::Fail(format!("the new alternative's head differs from a fresh fold of the edited log at {}", acceptance_text_difference(&head.1, &fold.1))),
                (Ok(_), Ok(reloaded)) if reloaded != *fold => AcceptanceVerdict::Fail(format!(
                    "the new alternative reloads to a head differing from a fresh fold of the edited log (store initializer ignores the supersession) at {}",
                    acceptance_text_difference(&reloaded.1, &fold.1)
                )),
                (Ok(_), Ok(_)) => AcceptanceVerdict::Pass(format!("{} = {:?}, row \"{label}\"{}, both reloads fold alike", change.0, change.1, if downstream { ", one downstream edit replayed" } else { "" })),
            }
        }
    }
}

/// 🔁️ Runs `cases` in order until one passes: each case with up to [`ACCEPTANCE_DOWNSTREAM_PER_LEAF`] downstream edits from cases
/// on the same document, then alone; a case whose own operation does not seed, and every case on a document that does not load,
/// is skipped at once. `Ok(Some)` names the passing case, `Ok(None)` means every case was skipped (reasons appended to `skipped`),
/// `Err` is a broken mechanism.
async fn acceptance_first_pass<A, M>(plugin: &str, manifest: fn() -> App, cases: &[AcceptanceCase<A::Mutation>], skipped: &mut Vec<String>, reload: bool) -> Result<Option<String>, String>
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
            match acceptance_scenario::<A, M>(manifest, case, next, reload).await {
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

/// 🧑‍⚖️ LAW (design §16.3): a representative editable leaf of the app is edited in history end to end through the generic
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
    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false).await {
        AcceptanceSearch::Passed(summary) => summary,
        AcceptanceSearch::Failed(failure) => panic!("{failure}"),
        AcceptanceSearch::Exhausted(report) => panic!("{report}"),
    }
}

/// 📊️ The outcome of a representative-leaf search: a passing case's summary, a broken mechanism, or every case skipped (with the
/// skip report; an app with no editable leaf at all reports zero cases).
enum AcceptanceSearch {
    Passed(String),
    Failed(String),
    Exhausted(String),
}

/// 🔎️ The search behind [`assert_history_edits_end_to_end`] and [`assert_documents_reload_identically`]: committed cases first, then
/// derived ones, each scenario on a reloaded document when `reload`.
async fn acceptance_search<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource], reload: bool) -> AcceptanceSearch
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut skipped = Vec::new();
    let committed = acceptance_cases::<A>(fixtures);
    let mut counts = vec![committed.len()];
    let mut outcome = acceptance_first_pass::<A, M>(plugin, manifest, &committed, &mut skipped, reload).await;
    for case in committed {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    if matches!(outcome, Ok(None)) {
        let derived = acceptance_example_cases::<A>(fixtures, examples).await;
        counts.push(derived.len());
        outcome = acceptance_first_pass::<A, M>(plugin, manifest, &derived, &mut skipped, reload).await;
        for case in derived {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
        }
    }
    match outcome {
        Ok(Some(summary)) => AcceptanceSearch::Passed(summary),
        Err(failure) => AcceptanceSearch::Failed(failure),
        Ok(None) => {
            let shown = skipped.len().min(40);
            AcceptanceSearch::Exhausted(format!(
                "{plugin}: none of the {} committed and {} derived editable case(s) under {} exercises a history edit end to end ({} skip(s), first {shown}):\n{}",
                counts[0],
                counts.get(1).copied().unwrap_or(0),
                fixtures.display(),
                skipped.len(),
                skipped[..shown].join("\n")
            ))
        }
    }
}

/// 🎟️ The archive operation id a reload uses on its fresh instance.
const ACCEPTANCE_ARCHIVE_OPERATION: u64 = 1;

/// 📀️ `app`'s document saved as its recursive archive (`PluginApp::document_archive`: the parent and every owned member) and loaded
/// into a fresh registered instance through the archive load (`begin_document_archive_load`, polled to `Ready`); `Err` names why,
/// after closing the fresh instance.
async fn acceptance_reloaded<A, M>(app: &VcsArtifactApp<A, M>, manifest: fn() -> App) -> Result<VcsArtifactApp<A, M>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let archive = PluginApp::document_archive(app).await.map_err(|fault| format!("the document does not archive: {fault:?}"))?;
    let mut reloaded = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
    reloaded.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let loaded = async {
        reloaded.store.set_local_actor_id(Some(ACCEPTANCE_ACTOR.to_string())).map_err(|error| format!("the actor is refused: {error:?}"))?;
        PluginApp::begin_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION, archive).map_err(|fault| format!("the archive is refused: {fault:?}"))?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        loop {
            let status = PluginApp::poll_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION).await.map_err(|fault| format!("the archive load faulted: {fault:?}"))?;
            match status.state {
                protocol::DocumentArchiveLoadState::Ready => break,
                protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault => return Err(format!("the archive load ends {:?}: {}", status.state, String::from_utf8_lossy(&status.fault))),
                _ if std::time::Instant::now() > deadline => return Err(format!("the archive load never settles ({}/{})", status.completed, status.total)),
                _ => {
                    PluginApp::maintenance_step(&mut reloaded, 1, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).map_err(|fault| format!("an archive maintenance step faulted: {fault:?}"))?;
                }
            }
        }
        PluginApp::acknowledge_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION).map_err(|fault| format!("the archive load is not acknowledged: {fault:?}"))?;
        reloaded.refresh_cache().await.map_err(|fault| format!("the reloaded history does not backfill: {fault:?}"))
    }
    .await;
    match loaded {
        Ok(()) => Ok(reloaded),
        Err(reason) => {
            acceptance_close(&mut reloaded).await;
            Err(reason)
        }
    }
}

/// 💽️ The head `app`'s document reloads to: saved as its recursive archive and loaded into a fresh instance through the app's own
/// store initializer ([`acceptance_reloaded`]).
async fn acceptance_reloaded_head<A, M>(app: &VcsArtifactApp<A, M>, manifest: fn() -> App) -> Result<(DslValue, String), String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut reloaded = acceptance_reloaded(app, manifest).await?;
    let head = acceptance_head(&reloaded);
    acceptance_close(&mut reloaded).await;
    head
}

/// 🖼️ What a reload must preserve of a document: its head (value and document text), its owned children (slot, child id, dialect,
/// envelope bytes, sorted) and the projected render of every declared window body (a refusal compared as its fault text).
struct AcceptanceDocumentView {
    head: (DslValue, String),
    children: Vec<(String, String, String, Vec<u8>)>,
    renders: Vec<(String, Result<String, String>)>,
}

/// 📸️ The [`AcceptanceDocumentView`] of `app`.
async fn acceptance_document_view<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App) -> Result<AcceptanceDocumentView, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let head = acceptance_head(app)?;
    let mut children: Vec<(String, String, String, Vec<u8>)> =
        PluginApp::child_packs(app).await.map_err(|fault| format!("the owned children do not pack: {fault:?}"))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.dialect, entry.envelope_pack)).collect();
    children.sort();
    let view = ViewModel::new(Locale::En, Terminology::Native);
    let mut renders = Vec::new();
    let bodies: Vec<String> = (&manifest().definition.window_kinds).into_iter().map(|window| window.body_key.clone()).collect();
    for body in bodies {
        let render = match PluginApp::render(app, &body, None, &view).await {
            Ok(tree) => artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string),
            Err(fault) => Err(format!("{fault:?}")),
        };
        renders.push((body, render));
    }
    Ok(AcceptanceDocumentView { head, children, renders })
}

/// 🧐️ Where a reloaded document's view first departs from the saved one, or `None` when it reloads identically.
fn acceptance_view_difference(saved: &AcceptanceDocumentView, reloaded: &AcceptanceDocumentView) -> Option<String> {
    if saved.head != reloaded.head {
        return Some(format!("the head differs at {}", acceptance_text_difference(&saved.head.1, &reloaded.head.1)));
    }
    if saved.children != reloaded.children {
        let names = |children: &[(String, String, String, Vec<u8>)]| children.iter().map(|(slot, child, dialect, bytes)| format!("{slot}/{child} {dialect} ({} bytes)", bytes.len())).collect::<Vec<_>>();
        return Some(format!("the owned children differ: {:?} saved, {:?} reloaded", names(&saved.children), names(&reloaded.children)));
    }
    saved.renders.iter().zip(&reloaded.renders).find(|(left, right)| left != right).map(|((body, left), (_, right))| match (left, right) {
        (Ok(left), Ok(right)) => format!("window body {body} renders differently at {}", acceptance_text_difference(left, right)),
        (left, right) => format!("window body {body} renders {left:?} saved, {right:?} reloaded"),
    })
}

/// 🔂️ LAW (design §20.15): a document survives save → fresh load. Every document the app ships (its initial document and each of
/// `examples`) saved as its recursive archive and loaded into a fresh instance has the same head (value and text), the same owned
/// children and the same render of every declared window body; and a representative history edit runs end to end on a reloaded
/// document (the [`assert_history_edits_end_to_end`] search, each scenario on the reloaded instance; an app without a parent-lane
/// leaf says so). Answers the summary; panics naming `plugin` and the document otherwise.
pub async fn assert_documents_reload_identically<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut documents = vec![("initial".to_string(), None)];
    for example in examples {
        let body = example.document();
        let text = if body.trim_start().starts_with('{') {
            semio_framework_pack_json::parse(&body, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|json| acceptance_dsl_of::<A>(semio_framework_pack_json::to_dsl_value(&json)))
        } else {
            Some(body)
        };
        documents.push((format!("example {}", example.id()), text));
    }
    let mut reloaded_documents = 0;
    for (name, text) in documents {
        let mut saved = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
        saved.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
        let outcome = async {
            if let Some(text) = text {
                let mut files = saved.document_text().await.map_err(|fault| format!("the document does not print: {fault:?}"))?;
                files.dsl = text;
                artifact_app_laws::load_document_text(&mut saved, &files).await.map_err(|fault| format!("the document does not load: {fault:?}"))?;
            }
            saved.refresh_cache().await.map_err(|fault| format!("the history does not backfill: {fault:?}"))?;
            let before = acceptance_document_view(&mut saved, manifest).await?;
            let mut reloaded = acceptance_reloaded(&saved, manifest).await?;
            let after = acceptance_document_view(&mut reloaded, manifest).await;
            acceptance_close(&mut reloaded).await;
            Ok::<_, String>(acceptance_view_difference(&before, &after?))
        }
        .await;
        acceptance_close(&mut saved).await;
        match outcome {
            Ok(None) => reloaded_documents += 1,
            Ok(Some(difference)) => panic!("{plugin}: the {name} document does not reload identically: {difference}"),
            Err(reason) => panic!("{plugin}: the {name} document does not survive save and load: {reason}"),
        }
    }
    let edit = match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, true).await {
        AcceptanceSearch::Passed(summary) => summary,
        AcceptanceSearch::Failed(failure) => panic!("{failure} (on a reloaded document)"),
        AcceptanceSearch::Exhausted(report) if report.contains("none of the 0 committed and 0 derived") => format!("{plugin}: no parent-lane leaf to edit on a reloaded document"),
        AcceptanceSearch::Exhausted(report) => panic!("{report} (on a reloaded document)"),
    };
    format!("{reloaded_documents} document(s) reload identically; {edit}")
}

/// 🧷️ The first applied, editable, unedited child-lane mutation of `app`'s history (`HistoryMutationEntry.store` names its owned
/// member, `<slot>/<childId>`): its id, its store, and its label in every locale (refused when a locale resolves empty).
async fn acceptance_child_row<A, M>(app: &mut VcsArtifactApp<A, M>) -> Result<Option<(String, String, String)>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let patch = app.history_patch(true).await.map_err(|fault| format!("the history does not project: {fault:?}"))?;
    let Some(row) = patch.upserts.iter().filter(|entry| entry.applied).flat_map(|entry| entry.mutations.iter()).find(|row| row.store.is_some() && row.editable && !row.superseded && !row.withdrawn && !row.pending) else {
        return Ok(None);
    };
    let (en, de) = (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string());
    if en.trim().is_empty() || de.trim().is_empty() {
        return Err(format!("the child-lane row {} is labelled {en:?} / {de:?}, not in every locale", row.mutation_id));
    }
    Ok(Some((row.mutation_id.clone(), row.store.clone().unwrap_or_default(), format!("{en} / {de}"))))
}

/// 🌰️ A registered instance on the app's initial document after the plugin's `seed` gestures (`(action id, JSON object args)`, each
/// dispatched as the acceptance actor and published to completion) — the gestures that land a child-lane edit in an owned member.
async fn acceptance_child_seeded<A, M>(manifest: fn() -> App, seed: &[(&str, &str)]) -> Result<VcsArtifactApp<A, M>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        app.store.set_local_actor_id(Some(ACCEPTANCE_ACTOR.to_string())).map_err(|error| format!("the actor is refused: {error:?}"))?;
        for (action, args) in seed {
            let args = semio_framework_pack_json::parse(args, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the seed args of {action} are not JSON: {error:?}"))?;
            let DslValue::Object(args) = semio_framework_pack_json::to_dsl_value(&args) else {
                return Err(format!("the seed args of {action} are not a JSON object"));
            };
            acceptance_verb(&mut app, action, args.iter().map(|(key, value)| (key.as_str(), value.clone())).collect()).await?;
            acceptance_pump(&mut app, |app| !app.has_pending_typed_operations()).await?;
        }
        app.refresh_cache().await.map_err(|fault| format!("the history does not backfill: {fault:?}"))
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

/// 🧵️ The head snapshot pack of every owned member of `app`, sorted by slot and child id.
async fn acceptance_child_heads<A, M>(app: &VcsArtifactApp<A, M>) -> Result<Vec<(String, String, Vec<u8>)>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    Ok(PluginApp::child_head_packs(app).await.map_err(|fault| format!("the owned children do not pack their heads: {fault:?}"))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.head_pack)).collect())
}

/// 🎞️ The child-lane scenario behind [`assert_child_history_edits_end_to_end`]; every instance is closed on every path.
async fn acceptance_child_scenario<A, M>(manifest: fn() -> App, seed: &[(&str, &str)]) -> Result<String, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut overwrite = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the seed does not land: {reason}"))?;
    let outcome = async {
        let (target, store, label) = acceptance_child_row(&mut overwrite).await?.ok_or("the seed lands no editable child-lane mutation in the history")?;
        let before = acceptance_child_heads(&overwrite).await?;
        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None).await {
            Ok(change) => change,
            Err(verdict) => return Err(format!("the overwrite of {target} in {store}: {}", verdict.text())),
        };
        let heads = acceptance_child_heads(&overwrite).await?;
        if heads == before {
            return Err(format!("the overwrite {} = {:?} of {target} leaves every member head unchanged", change.0, change.1));
        }
        let saved = acceptance_document_view(&mut overwrite, manifest).await?;
        let mut reloaded = acceptance_reloaded(&overwrite, manifest).await?;
        let fresh = acceptance_document_view(&mut reloaded, manifest).await;
        let fresh_heads = acceptance_child_heads(&reloaded).await;
        acceptance_close(&mut reloaded).await;
        if let Some(difference) = acceptance_view_difference(&saved, &fresh?) {
            return Err(format!("the overwritten document does not fold freshly to the same state after save and load: {difference}"));
        }
        if fresh_heads? != heads {
            return Err("a fresh fold of the edited member log has other member heads than the overwrite".into());
        }
        Ok::<_, String>((change, store, label, heads))
    }
    .await;
    acceptance_close(&mut overwrite).await;
    let (change, store, label, heads) = outcome?;
    let mut branched = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the alternative instance does not seed: {reason}"))?;
    let alternative = async {
        let (target, _, _) = acceptance_child_row(&mut branched).await?.ok_or("the alternative instance lists no editable child-lane mutation")?;
        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE)).await {
            Ok(_) => {}
            Err(verdict) => return Err(format!("the change the overwrite accepted fails as a new alternative: {}", verdict.text())),
        }
        if acceptance_child_heads(&branched).await? != heads {
            return Err("the new alternative's member heads differ from the overwrite's".into());
        }
        Ok::<(), String>(())
    }
    .await;
    acceptance_close(&mut branched).await;
    alternative?;
    Ok(format!("{store}: {} = {:?}, row \"{label}\"", change.0, change.1))
}

/// 👶️ LAW (design §12, §20.15 — D20): a child-lane mutation, the edit the plugin's `seed` gestures land in an owned member store, is
/// edited in history end to end through the generic member path (`historyEditBegin{mutationId, store}`): the overwrite changes the
/// member's head; the overwritten document saved and freshly loaded (its member folds the edited log from scratch) has the same head,
/// owned children, renders and member heads; the same change committed as a new alternative on a second instance reaches the same
/// member heads; the row is labelled in every locale. Answers the summary; panics naming `plugin` otherwise.
pub async fn assert_child_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, seed: &[(&str, &str)]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_child_scenario::<A, M>(manifest, seed).await {
        Ok(summary) => format!("{plugin}: child lane {summary}"),
        Err(reason) => panic!("{plugin}: the child-lane history edit breaks the generic mechanism: {reason}"),
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
        let Ok(root) = semio_framework_pack_json::parse(schema, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            failures.push(format!("{kind}: its payload schema is not JSON"));
            continue;
        };
        let mut pending = Vec::new();
        references(&semio_framework_pack_json::to_dsl_value(&root), &mut pending);
        let (mut seen, mut documents, mut unresolved) = (BTreeSet::new(), Vec::new(), Vec::new());
        while let Some(id) = pending.pop() {
            if id.is_empty() || !seen.insert(id.clone()) {
                continue;
            }
            match semio_framework::registered_input_schema_document(&id) {
                Some(document) => {
                    references(&document, &mut pending);
                    documents.push(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&document)));
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

/// 🩺️ LAW (design §16.3, the editable-everything gate at runtime): on a registered instance — which publishes every document its
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

/// 🔌️ Wires [`assert_history_edits_end_to_end`] into a plugin crate's test target for one editor: `plugin` names the failure,
/// `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$fixtures` the fixture root relative to the crate's
/// manifest directory (the artifact tree: `"../.."`).
#[macro_export]
macro_rules! history_edit_acceptance_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $fixtures:literal) => {
        /// ⏮️ LAW (design §16.3): a representative editable leaf of this editor is edited in history end to end.
        #[test]
        fn history_edits_end_to_end() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[history-edit-acceptance] {summary}");
        }

        /// ⛓️ LAW (design §16.3): every document leaf payload schema of this editor resolves through the runtime resolver.
        #[test]
        fn history_edit_inputs_resolve() {
            let leaves = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_input_schemas_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest));
            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve", $plugin);
        }
    };
}

/// 🛠️ Wires [`assert_documents_reload_identically`] into a composed plugin's test target for one editor (design §20.15), the same
/// arguments as [`history_edit_acceptance_law!`](crate::history_edit_acceptance_law): `plugin` names the failure, `$editor` is the
/// `ArtifactEditor` (its `Members` and `examples()` are read from it), `$manifest` its `fn() -> App`, `$fixtures` the fixture root
/// relative to the crate's manifest directory.
#[macro_export]
macro_rules! composed_reload_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $fixtures:literal) => {
        /// 💾️ LAW (design §20.15): every document this editor ships, and a history edit on it, survives save → fresh load.
        #[test]
        fn documents_reload_identically() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_documents_reload_identically::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[documents-reload] {}: {summary}", $plugin);
        }
    };
}

/// 🧩️ Wires [`assert_child_history_edits_end_to_end`] into a composed plugin's test target (design §12, §20.15): `plugin` names the
/// failure, `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$seed` the `[(action id, JSON object args)]` gestures
/// that land a child-lane edit in an owned member of the editor's initial document.
#[macro_export]
macro_rules! composed_child_history_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $seed:expr) => {
        /// 🍼️ LAW (design §12, §20.15): a child-lane mutation of this editor is edited in history end to end on its member store.
        #[test]
        fn child_history_edits_end_to_end() {
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_child_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &$seed,
            ));
            ::std::println!("[child-history-edit] {summary}");
        }
    };
}
