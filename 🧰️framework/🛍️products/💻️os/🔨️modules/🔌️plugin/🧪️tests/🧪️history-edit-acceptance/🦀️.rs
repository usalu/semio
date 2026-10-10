//! ⏪️ The cross-plugin history-edit acceptance law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §16.3, gap G12):
//! for a representative editable leaf of any app, read from the app's own committed mutation fixtures (on their own documents,
//! else on the documents the app ships: its initial document and its examples), the real verbs run
//! `historyEditWithdraw` → `historyEditAccept` → `historyEditRestore` (a withdrawal taken back leaves no trace), then
//! `historyEditBegin` (the preview is the document as of the edited mutation: downstream is not applied) → `historyEditInput` (a
//! schema-valid change derived from the leaf's input descriptors) → `historyEditAccept` → the Report replay →
//! `historyEditFinalize` → `historyEditCommit` as an overwrite and, on a second instance, as a new alternative; each head must
//! equal a fresh fold of the edited log, and the edited mutation's history row must carry its leaf label in every locale. A leaf
//! whose descriptor declares `"editable": false` is withdraw-only (design §22.20) and never a case. One macro call, [`history_edit_acceptance_law!`](crate::history_edit_acceptance_law), wires it into a
//! plugin crate's test target; every failure names the plugin, the leaf and the fixture case.

use super::*;
use semio_framework::kernel::{HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework::{ActionArgDef, ArgSchema, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};

/// 🧑‍💻️ The actor every acceptance instance authors and dispatches as.
const ACCEPTANCE_ACTOR: &str = "history-edit-acceptance";
/// 🌿️ The alternative a new-alternative finalize names.
const ACCEPTANCE_ALTERNATIVE: &str = "History edit acceptance";
/// 🔢️ Changes tried per leaf before it counts as having no derivable schema-valid change.
const ACCEPTANCE_CHANGES_PER_LEAF: usize = 16;
/// 🌊️ Downstream operations tried per leaf before the leaf runs alone.
const ACCEPTANCE_DOWNSTREAM_PER_LEAF: usize = 2;
/// 🧰️ The input-control kinds a change is derived for, in the order of [`acceptance_change_buckets`].
const ACCEPTANCE_CONTROL_KINDS: [&str; 5] = ["number", "boolean", "option", "vector", "text"];
/// 🗃️ Committed cases the census tries per leaf before the leaf counts as not exercised.
const ACCEPTANCE_CASES_PER_LEAF: usize = 3;
/// 🪙️ Further cases tried for one control kind nobody proved in the leaf sweep.
const ACCEPTANCE_KIND_ATTEMPTS: usize = 12;
/// 🌋️ Seeded histories the conflict law tries before the editor counts as having no dependents.
const ACCEPTANCE_CASCADE_CASES: usize = 12;
/// 🪜️ Blockers resolved in turn before a blocked review counts as never reaching ready.
const ACCEPTANCE_CASCADE_BLOCKERS: usize = 8;
/// 🧱️ Cases whose history cannot be built that the conflict law passes over before it stops looking.
const ACCEPTANCE_CASCADE_UNBUILT: usize = 60;

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
/// that shows at least one input row — design §22.20: an editor with zero rows never opens — and no foreign-step capability).
fn acceptance_operation<A: ArtifactApp>(wire: &DslValue) -> Option<A::Mutation> {
    let value = if acceptance_record(wire) { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire.clone() };
    let op = <A::Mutation as semio_framework_value::FromValue>::from_value(value).ok()?;
    if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none_or(|schema| !time_travel::time_travel_schema_shows_inputs(schema)) || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {
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

/// 🎛️ The schema-valid changes the law tries for one draft, most robust first ([`acceptance_change_buckets`] flattened): each a
/// JSON pointer and the value to draft there.
pub fn acceptance_changes(inputs: &[ActionArgDef], value: &DslValue) -> Vec<(String, DslValue)> {
    acceptance_change_buckets(inputs, value).into_iter().flatten().take(ACCEPTANCE_CHANGES_PER_LEAF).collect()
}

/// 🪣️ The schema-valid changes of one draft per input-control kind ([`ACCEPTANCE_CONTROL_KINDS`]): every number input moved by its
/// step (then to its bounds and their midpoint), every boolean flipped, every option switched, every vector's first axis moved,
/// every free text extended — each a JSON pointer and the value to draft there. Existing array slots expose their item controls;
/// reference and opaque inputs are never changed.
pub fn acceptance_change_buckets(inputs: &[ActionArgDef], value: &DslValue) -> [Vec<(String, DslValue)>; 5] {
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
            if input.presentation == Some(semio_framework::ArgPresentation::Hidden) { continue; }
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
                ArgSchema::Array { items, .. } => {
                    if let Some(values) = current.and_then(DslValue::as_array) {
                        for index in 0..values.len().min(ACCEPTANCE_CHANGES_PER_LEAF) {
                            let item = ActionArgDef { id: format!("{}/{index}", input.id), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() };
                            collect(std::slice::from_ref(&item), value, prefix, buckets);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    let mut buckets: [Vec<(String, DslValue)>; 5] = Default::default();
    collect(inputs, value, "", &mut buckets);
    buckets
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
    Err(format!(
        "the instance never settled within 60 s; time travel rests at {:?} (pending history work: {}, pending typed operations: {})",
        app.time_travel.status().map(|status| status.stage),
        app.time_travel.has_pending_work(),
        app.has_pending_typed_operations()
    ))
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

/// 🪴️ A registered instance whose document is `base` (loaded through the document text path unless the instance boots on exactly
/// that text: its own initial document keeps the members it owns) followed by one clean edit per op
/// of `ops` (each leaving an applied operation when `strict`); `Err` names why the data cannot be seeded (the base, or the index
/// of the first op that does not apply cleanly), after closing the instance.
async fn acceptance_seeded<A, M>(manifest: fn() -> App, base: &str, ops: &[&A::Mutation], strict: bool) -> Result<VcsArtifactApp<A, M>, AcceptanceSeedFault>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into())).await;
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        assert_eq!(app.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.to_string()));
        let mut files = app.document_text().await.map_err(|fault| AcceptanceSeedFault::Base(format!("the document does not print: {fault:?}")))?;
        if files.dsl != base {
            files.dsl = base.to_string();
            artifact_app_laws::load_document_text(&mut app, &files).await.map_err(|fault| AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}")))?;
        }
        for (index, op) in ops.iter().enumerate() {
            let applied = app.store.mutation_ops().map_or(0, |applied| applied.len());
            let receipt =
                crate::with_authoring_identity!(|identity| app.store.dispatch(ArtifactCommand::Apply { mutations: vec![(*op).clone()], transaction: None }, &mut identity).await).map_err(|error| AcceptanceSeedFault::Operation(index, format!("the seed edit is refused: {error:?}")))?;
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
/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts AND that changes the operation:
/// accepting a draft that rebuilds the same operation closes time travel without a trace, by design, and the next one is tried on a
/// re-opened session), accept, replay to a clean
/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. With `as_of` (the document folded up to
/// and including `target`), what every render seam reads right after the begin must be exactly that: the preview applies no
/// downstream mutation. With `kind` (and no `change`) only changes of that control kind are drafted. Answers the change drafted.
#[allow(clippy::too_many_arguments)]
async fn acceptance_drive<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    target: &str,
    store: Option<&str>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: Option<&(DslValue, String)>,
    kind: Option<usize>,
) -> Result<(String, DslValue), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.to_string()))];
    begin.extend(store.map(|store| (HISTORY_EDIT_ARG_STORE, DslValue::String(store.to_string()))));
    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
    let (inputs, original) = match app.time_travel.editor() {
        None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
        Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail(format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused))),
        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    if let Some(as_of) = as_of {
        let committed = app.store.snapshot_owner();
        let shown = app.time_travel.render_snapshot_or(&app.tool_runs, &committed);
        let shown = (semio_framework_value::ToValue::to_value(shown.as_ref()), store::ArtifactDsl::print_dsl(shown.as_ref()));
        if shown != *as_of {
            let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
            return Err(AcceptanceVerdict::Fail(format!("while editing, the preview is not the document as of the edited mutation (downstream not applied) at {}", acceptance_text_difference(&shown.1, &as_of.1))));
        }
    }
    let changes = match (change, kind) {
        (Some(change), _) => vec![change.clone()],
        (None, Some(kind)) => acceptance_change_buckets(&inputs, &original).into_iter().nth(kind).unwrap_or_default().into_iter().take(ACCEPTANCE_CHANGES_PER_LEAF).collect(),
        (None, None) => acceptance_changes(&inputs, &original),
    };
    let (mut drafted, mut unchanged) = (None, 0);
    for candidate in changes {
        if app.time_travel.status().is_none() {
            acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
        }
        if acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_err()
            || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
        {
            continue;
        }
        acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
        acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
        if change.is_none() && app.time_travel.status().is_some_and(|status| status.review == Some(HistoryTimeTravelReview::Blocked)) {
            acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
            acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
            if store.is_none() && app.store.supersessions().iter().any(|(id, _)| id.0 == target) {
                return Err(AcceptanceVerdict::Fail("discarding a blocked trial left a supersession of the mutation".into()));
            }
            continue;
        }
        if app.time_travel.status().is_some() {
            drafted = Some(candidate);
            break;
        }
        if store.is_none() && app.store.supersessions().iter().any(|(id, _)| id.0 == target) {
            return Err(AcceptanceVerdict::Fail(format!("accepting {} = {:?}, which rebuilds the same operation, closed the session but left a supersession of the mutation", candidate.0, candidate.1)));
        }
        unchanged += 1;
    }
    let Some(drafted) = drafted else {
        if app.time_travel.status().is_some() {
            let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        }
        return Err(AcceptanceVerdict::Skip(format!(
            "no schema-valid{} change of its {} input(s) is accepted ({unchanged} left the operation unchanged)",
            kind.map_or(String::new(), |kind| format!(" {}", ACCEPTANCE_CONTROL_KINDS[kind])),
            inputs.len()
        )));
    };
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

/// 🧯️ The withdraw → restore round trip over `app`'s first document mutation (design §22.1, §22.16): the row action
/// `historyEditWithdraw{mutationId}` opens the session on a withdrawn draft, accepting it replays to a review (ready, or blocked by
/// a downstream mutation that needed it), and `historyEditRestore{mutationId}` takes the withdrawal back — the session closes, the
/// head is the one before and the store holds no supersession of the mutation. `Ok(false)`: the store's supersede law admits no
/// withdrawal of this operation (`timeTravel.not-withdrawable`), nothing was changed.
async fn acceptance_withdraw_restore<A, M>(app: &mut VcsArtifactApp<A, M>) -> Result<bool, AcceptanceVerdict>
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
    let before = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    let named = || vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.clone()))];
    match acceptance_verb(app, HISTORY_EDIT_WITHDRAW_ACTION_ID, named()).await {
        Ok(()) => {}
        Err(refusal) if refusal.ends_with(semio_framework_time_travel::TIME_TRAVEL_NOT_WITHDRAWABLE_CODE) => return Ok(false),
        Err(refusal) => return Err(AcceptanceVerdict::Fail(format!("the row's withdraw does not open: {refusal}"))),
    }
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawn draft is not accepted: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_some_and(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting the withdrawal closed the session".into()))?;
    if status.stage != HistoryTimeTravelStage::Reviewing || !matches!(status.review, Some(HistoryTimeTravelReview::Ready | HistoryTimeTravelReview::Blocked)) {
        return Err(AcceptanceVerdict::Fail(format!("the replay of the withdrawal ends at {:?}/{:?}, not a review with a report", status.stage, status.review)));
    }
    acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, named()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal is not restored: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    if app.store.supersessions().iter().any(|(id, _)| id.0 == target) {
        return Err(AcceptanceVerdict::Fail("restoring the only withdrawal left a supersession of the mutation in the store".into()));
    }
    let after = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    if after != before {
        return Err(AcceptanceVerdict::Fail(format!("after withdraw and restore the head differs from the one before at {}", acceptance_text_difference(&after.1, &before.1))));
    }
    Ok(true)
}

/// ✏️ [`acceptance_drive`] over `app`'s first document mutation (its preview checked against `as_of`), then the store's own account
/// of it: the superseding input decoded as the edited operation, unscoped for an overwrite, scoped to the new active alternative
/// otherwise. Answers the change drafted and the edited operation the store now folds.
async fn acceptance_session<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: &(DslValue, String),
    kind: Option<usize>,
) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>
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
    let drafted = acceptance_drive(app, &target, None, change, alternative, Some(as_of), kind).await?;
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
/// sessions run on the seeded document after it was saved and loaded into a fresh instance ([`acceptance_reloaded`]). With `kind`
/// the overwrite drafts an input of that control kind only ([`ACCEPTANCE_CONTROL_KINDS`]).
async fn acceptance_scenario<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, downstream: Option<&A::Mutation>, reload: bool, kind: Option<usize>) -> AcceptanceVerdict
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
    let as_of = match downstream {
        None => acceptance_head(&overwrite),
        Some(_) => match acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true).await {
            Ok(mut alone) => {
                let head = acceptance_head(&alone);
                acceptance_close(&mut alone).await;
                head
            }
            Err(fault) => Err(format!("the edited mutation alone does not seed: {}", fault.reason())),
        },
    };
    let as_of = match as_of {
        Ok(head) => head,
        Err(reason) => {
            acceptance_close(&mut overwrite).await;
            return AcceptanceVerdict::Fail(reason);
        }
    };
    let (trail, session) = match acceptance_withdraw_restore(&mut overwrite).await {
        Ok(withdrawn) => (if withdrawn { "withdrawn and restored without a trace" } else { "withdrawal not admitted by the store's law" }, acceptance_session(&mut overwrite, None, None, &as_of, kind).await),
        Err(verdict) => ("", Err(verdict)),
    };
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
        (Err(reason), _, _, _) | (_, Err(reason), _, _) | (_, _, Err(reason), _) => AcceptanceVerdict::Fail(reason),
        (_, _, _, Err(reason)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
            Err(fault) => AcceptanceVerdict::Fail(format!("the edited document does not survive save and load ({reason}) and its control does not seed: {}", fault.reason())),
            Ok(mut control) => {
                let unedited = acceptance_reloaded_head(&control, manifest).await;
                acceptance_close(&mut control).await;
                match unedited {
                    Err(seeded) => AcceptanceVerdict::SkipCase(format!("the seeded document does not survive save and load before any edit ({seeded}): a seed through the store alone does not complete what this operation owns")),
                    Ok(_) => AcceptanceVerdict::Fail(format!("after {} = {:?} the edited document does not survive save and load although the seeded one does: {reason}", change.0, change.1)),
                }
            }
        },
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
                        let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some(), &as_of, trail).await;
                        acceptance_close(&mut branched).await;
                        verdict
                    }
                }
            }
            Ok(mut branched) => {
                let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some(), &as_of, trail).await;
                acceptance_close(&mut branched).await;
                verdict
            }
        },
    };
    ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
    verdict
}

/// 🌳️ The new-alternative half of a scenario on `branched`: the change the overwrite accepted, committed as a new alternative, must
/// reach the same head as the fresh fold `fold`, live and after the document reloads through the app's own store initializer; its
/// preview is checked against `as_of` like the overwrite's, and `trail` (the overwrite's withdraw round trip) joins the summary.
#[allow(clippy::too_many_arguments)]
async fn acceptance_alternative<A, M>(
    branched: &mut VcsArtifactApp<A, M>,
    manifest: fn() -> App,
    change: &(String, DslValue),
    fold: &(DslValue, String),
    label: &str,
    downstream: bool,
    as_of: &(DslValue, String),
    trail: &str,
) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE), as_of, None).await {
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
                (Ok(_), Ok(_)) => AcceptanceVerdict::Pass(format!(
                    "{} = {:?}, row \"{label}\"{}, previewed as of the mutation, {trail}, both reloads fold alike",
                    change.0,
                    change.1,
                    if downstream { ", one downstream edit replayed" } else { "" }
                )),
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
            match acceptance_scenario::<A, M>(manifest, case, next, reload, None).await {
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

/// 🚫️ The semantic kinds whose leaf descriptor under `root` (a `🔣️.json` naming `semanticKind` and `aggregateVariant`, fixtures and
/// tests aside) declares `"editable": false`: withdraw-only by declaration (design §22.20, §22.23).
pub fn acceptance_withdraw_only_kinds(root: &std::path::Path) -> BTreeSet<String> {
    fn walk(directory: &std::path::Path, found: &mut BTreeSet<String>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if !path.is_dir() || name.starts_with('.') || ["target", "dist", "node_modules", "🗑️generated", "🧫️fixtures", "🧪️tests"].contains(&name.as_str()) {
                continue;
            }
            if let Some(descriptor) = acceptance_json(&path.join("🔣️.json")).filter(|descriptor| descriptor.get("aggregateVariant").is_some()) {
                if descriptor.get("editable").and_then(DslValue::as_bool) == Some(false) {
                    found.extend(descriptor.get("semanticKind").and_then(DslValue::as_str).map(str::to_string));
                }
                continue;
            }
            walk(&path, found);
        }
    }
    let mut kinds = BTreeSet::new();
    walk(root, &mut kinds);
    kinds
}

/// 🧑‍⚖️ LAW (design §16.3): a representative editable leaf of the app is edited in history end to end through the generic
/// mechanism — overwrite and new alternative, each head (value and document text) a fresh fold of the edited log, the row
/// labelled in every locale. The committed fixture cases under `fixtures` are tried first ([`acceptance_cases`]); when none
/// exercises the flow, every committed operation on every document the app ships ([`acceptance_example_cases`] over its initial
/// document and `examples`). The first passing case is the representative; a broken mechanism fails at once. Answers the passing
/// case's summary; an app whose aggregate has no leaf at all says so, and so does one whose every leaf is declared withdraw-only
/// (`"editable": false`, design §22.20); panics naming `plugin`, the leaf and the case otherwise. After the representative, every
/// editable leaf is swept ([`acceptance_census`]): one passing case per input-control kind and no broken case are law, the number
/// of leaves exercised is a census line appended to the summary.
pub async fn assert_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let descriptors = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS;
    if descriptors.is_empty() {
        return format!("{plugin}: no parent-lane leaf (a composed parent edits its content on the child lane)");
    }
    let withdraw_only = acceptance_withdraw_only_kinds(fixtures);
    if descriptors.iter().all(|descriptor| withdraw_only.contains(descriptor.semantic_kind)) {
        return format!("{plugin}: withdraw-only artifact (all {} leaves declare editable: false, design §22.20)", descriptors.len());
    }
    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false).await {
        AcceptanceSearch::Passed(summary) => format!("{summary}\n[history-edit-census] {}", acceptance_census::<A, M>(plugin, manifest, fixtures, examples, &withdraw_only).await),
        AcceptanceSearch::Failed(failure) => panic!("{failure}"),
        AcceptanceSearch::Exhausted(report) => panic!("{report}"),
    }
}

/// 🔭️ Which input-control kinds ([`ACCEPTANCE_CONTROL_KINDS`]) the draft editor of `case`'s operation offers a change for: the case
/// seeded alone, `historyEditBegin`, the editor's inputs read, `historyEditExit`. `Err` is the verdict that ends the case (its
/// document or operation does not seed: a skip; the editor of an editable operation does not open or close: a broken mechanism).
async fn acceptance_offered<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>) -> Result<[bool; 5], AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = match acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true).await {
        Ok(app) => app,
        Err(AcceptanceSeedFault::Base(reason)) => return Err(AcceptanceVerdict::SkipBase(reason)),
        Err(AcceptanceSeedFault::Operation(_, reason)) => return Err(AcceptanceVerdict::SkipCase(reason)),
    };
    let offered = async {
        let target = app
            .store
            .mutation_ops()
            .map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?
            .first()
            .map(|op| op.mutation_id.0.clone())
            .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
        acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target))]).await.map_err(AcceptanceVerdict::Fail)?;
        let offered = match app.time_travel.editor() {
            None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
            Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail(format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused))),
            Some(editor) => acceptance_change_buckets(&editor.inputs, &editor.value).map(|bucket| !bucket.is_empty()),
        };
        acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
        Ok::<_, AcceptanceVerdict>(offered)
    }
    .await;
    acceptance_close(&mut app).await;
    offered
}

/// 📋️ LAW (design §16.3, session-5 decision): the sweep behind [`assert_history_edits_end_to_end`] over every editable leaf of the
/// aggregate (the kinds of `withdraw_only` aside) and its committed cases under `fixtures` — for a leaf that commits no case with a
/// document, its committed operations on the documents the app ships ([`acceptance_example_cases`]) — at most
/// [`ACCEPTANCE_CASES_PER_LEAF`] per leaf. Law: every input-control kind some case's editor offers ([`acceptance_offered`]) has one case that passes the whole
/// scenario drafting an input of that kind, and no case breaks the generic mechanism — it panics naming `plugin`, the kind or every
/// broken case otherwise. Census, never failing: the leaves exercised of the editable ones, those without a committed editable
/// case, and the leaf that proved each kind — the line it answers.
async fn acceptance_census<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource], withdraw_only: &BTreeSet<String>) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let leaves: Vec<&str> = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.iter().map(|descriptor| descriptor.semantic_kind).filter(|kind| !withdraw_only.contains(*kind)).collect();
    let mut cases = acceptance_cases::<A>(fixtures);
    let leaf_of = |case: &AcceptanceCase<A::Mutation>| protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
    let committed: BTreeSet<&str> = cases.iter().map(leaf_of).collect();
    for case in acceptance_example_cases::<A>(fixtures, examples).await {
        match committed.contains(leaf_of(&case)) {
            true => ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op),
            false => cases.push(case),
        }
    }
    let shipped = leaves.iter().filter(|leaf| !committed.contains(**leaf) && cases.iter().any(|case| leaf_of(case) == **leaf)).count();
    let (mut exercised, mut uncased, mut attempted) = (BTreeSet::new(), Vec::new(), BTreeSet::new());
    let mut offering: [BTreeSet<&str>; 5] = Default::default();
    let mut proven: [Option<&str>; 5] = [None; 5];
    let (mut broken, mut skipped) = (Vec::new(), Vec::new());
    for leaf in leaves.iter().copied() {
        let own: Vec<usize> = cases.iter().enumerate().filter(|(_, case)| leaf_of(case) == leaf).map(|(index, _)| index).take(ACCEPTANCE_CASES_PER_LEAF).collect();
        if own.is_empty() {
            uncased.push(leaf);
        }
        for index in own {
            let case = &cases[index];
            let offered = match acceptance_offered::<A, M>(manifest, case).await {
                Ok(offered) => offered,
                Err(AcceptanceVerdict::Fail(reason)) => {
                    broken.push(format!("{leaf} ({}): {reason}", case.directory));
                    continue;
                }
                Err(verdict) => {
                    skipped.push(format!("{leaf} ({}): {}", case.directory, verdict.text()));
                    continue;
                }
            };
            for kind in (0..5).filter(|kind| offered[*kind]) {
                offering[kind].insert(leaf);
            }
            if !offered.contains(&true) {
                skipped.push(format!("{leaf} ({}): its editor offers no number, boolean, option, vector or text input to change (references, lists or opaque values only)", case.directory));
                continue;
            }
            let mut wanted: Vec<usize> = (0..5).filter(|kind| offered[*kind] && proven[*kind].is_none()).collect();
            if wanted.is_empty() && !exercised.contains(leaf) {
                wanted.extend((0..5).find(|kind| offered[*kind]));
            }
            for kind in wanted {
                attempted.insert((index, kind));
                match acceptance_scenario::<A, M>(manifest, case, None, false, Some(kind)).await {
                    AcceptanceVerdict::Pass(_) => {
                        exercised.insert(leaf);
                        proven[kind].get_or_insert(leaf);
                    }
                    AcceptanceVerdict::Fail(reason) => broken.push(format!("{leaf} [{}] ({}): {reason}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory)),
                    verdict => skipped.push(format!("{leaf} [{}] ({}): {}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory, verdict.text())),
                }
            }
            if exercised.contains(leaf) {
                break;
            }
        }
    }
    for kind in 0..5 {
        let pending: Vec<usize> = cases.iter().enumerate().filter(|(index, case)| offering[kind].contains(leaf_of(case)) && !attempted.contains(&(*index, kind))).map(|(index, _)| index).take(ACCEPTANCE_KIND_ATTEMPTS).collect();
        for index in pending {
            if proven[kind].is_some() {
                break;
            }
            let (case, leaf) = (&cases[index], leaf_of(&cases[index]));
            match acceptance_scenario::<A, M>(manifest, case, None, false, Some(kind)).await {
                AcceptanceVerdict::Pass(_) => {
                    exercised.insert(leaf);
                    proven[kind] = Some(leaf);
                }
                AcceptanceVerdict::Fail(reason) => broken.push(format!("{leaf} [{}] ({}): {reason}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory)),
                verdict => skipped.push(format!("{leaf} [{}] ({}): {}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory, verdict.text())),
            }
        }
    }
    let gaps: Vec<String> = (0..5).filter(|kind| proven[*kind].is_none() && !offering[*kind].is_empty()).map(|kind| format!("{} (offered by {})", ACCEPTANCE_CONTROL_KINDS[kind], offering[kind].iter().copied().collect::<Vec<_>>().join(", "))).collect();
    let kinds: Vec<String> = (0..5)
        .map(|kind| match proven[kind] {
            Some(leaf) => format!("{} proven by {leaf}", ACCEPTANCE_CONTROL_KINDS[kind]),
            None if offering[kind].is_empty() => format!("{} not offered", ACCEPTANCE_CONTROL_KINDS[kind]),
            None => format!("{} UNPROVEN", ACCEPTANCE_CONTROL_KINDS[kind]),
        })
        .collect();
    let idle: Vec<String> = leaves
        .iter()
        .copied()
        .filter(|leaf| !exercised.contains(leaf) && !uncased.contains(leaf))
        .map(|leaf| {
            let reason = skipped.iter().chain(&broken).find(|entry| entry.starts_with(&format!("{leaf} "))).and_then(|entry| entry.rsplit_once("): ")).map_or("no reason recorded", |(_, reason)| reason);
            format!("{leaf} [{}]", reason.chars().take(110).collect::<String>())
        })
        .collect();
    let census = format!(
        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} swept only on a shipped document for want of a committed one, {} without any editable case{}{}, {} with cases but not exercised{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        shipped,
        uncased.len(),
        if uncased.is_empty() { "" } else { ": " },
        uncased.iter().take(12).copied().collect::<Vec<_>>().join(", "),
        idle.len(),
        if idle.is_empty() { "" } else { ": " },
        idle.iter().take(12).cloned().collect::<Vec<_>>().join("; "),
        kinds.join(", ")
    );
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    let shown = skipped.len().min(12);
    assert!(broken.is_empty(), "{plugin}: {} leaf case(s) break the generic mechanism (first {}):\n{}\n[history-edit-census] {census}", broken.len(), broken.len().min(12), broken[..broken.len().min(12)].join("\n"));
    assert!(gaps.is_empty(), "{plugin}: no committed case exercises an input of control kind {} end to end ({} skip(s), first {shown}):\n{}\n[history-edit-census] {census}", gaps.join("; "), skipped.len(), skipped[..shown].join("\n"));
    census
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
    let mut reloaded = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into())).await;
    reloaded.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let loaded = async {
        assert_eq!(reloaded.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.to_string()));
        PluginApp::begin_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION, archive).map_err(|fault| format!("the archive is refused: {fault:?}"))?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        loop {
            let status = PluginApp::poll_document_archive_load(&mut reloaded, ACCEPTANCE_ARCHIVE_OPERATION).await.map_err(|fault| format!("the archive load faulted: {fault:?}"))?;
            match status.state {
                protocol::DocumentArchiveLoadState::Ready => break,
                protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault => return Err(format!("the archive load ends {:?}: {}", status.state, String::from_utf8_lossy(&status.fault))),
                _ if std::time::Instant::now() > deadline => return Err(format!("the archive load never settles ({}/{})", status.completed, status.total)),
                _ => {
                    PluginApp::maintenance_step(&mut reloaded, crate::plugin_runtime::runtime_lifecycle_grant()).map_err(|fault| format!("an archive maintenance step faulted: {fault:?}"))?;
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
/// `examples`, loaded through the app's own example route `setActiveExample{exampleId}`) saved as its recursive archive and loaded into a fresh instance has the same head (value and text), the same owned
/// children and the same render of every declared window body; and a representative history edit runs end to end on a reloaded
/// document (the [`assert_history_edits_end_to_end`] search, each scenario on the reloaded instance; an app without a parent-lane
/// leaf says so). Answers the summary; panics naming `plugin` and the document otherwise.
pub async fn assert_documents_reload_identically<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut documents = vec![("initial".to_string(), None)];
    documents.extend(examples.iter().map(|example| (format!("example {}", example.id()), Some(example.id().to_string()))));
    let mut reloaded_documents = 0;
    for (name, example) in documents {
        let mut saved = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into())).await;
        saved.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
        let outcome = async {
            if let Some(example) = example {
                assert_eq!(saved.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.to_string()));
                acceptance_verb(&mut saved, CATALOGUE_EXAMPLE_ACTION_ID, vec![("exampleId", DslValue::String(example))]).await.map_err(|refusal| format!("the app's own example route does not load it: {refusal}"))?;
                artifact_app_laws::settle_registered_typed_operation(&mut saved, acceptance_meta().instance_id).await.map_err(|fault| format!("the example does not finish loading: {fault:?}"))?;
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
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into())).await;
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        assert_eq!(app.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.to_string()));
        for (action, args) in seed {
            let args = semio_framework_pack_json::parse(args, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the seed args of {action} are not JSON: {error:?}"))?;
            let DslValue::Object(args) = semio_framework_pack_json::to_dsl_value(&args) else {
                return Err(format!("the seed args of {action} are not a JSON object"));
            };
            acceptance_verb(&mut app, action, args.iter().map(|(key, value)| (key.as_str(), value.clone())).collect()).await?;
            artifact_app_laws::settle_registered_typed_operation(&mut app, acceptance_meta().instance_id).await.map_err(|fault| format!("the seed gesture {action} does not publish: {fault:?}"))?;
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

/// 🔄️ Where `app`'s document, saved as its recursive archive and loaded into a fresh instance through the app's own store initializer
/// ([`acceptance_reloaded`]), departs from the live one — its head, owned children or window renders ([`acceptance_view_difference`]), or its
/// member head packs (every member folds its own log, edits included, from scratch) — else `None`; the fresh instance is closed on every path.
async fn acceptance_child_reload_difference<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App) -> Result<Option<String>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let saved = acceptance_document_view(app, manifest).await?;
    let heads = acceptance_child_heads(app).await?;
    let mut reloaded = acceptance_reloaded(app, manifest).await?;
    let fresh = acceptance_document_view(&mut reloaded, manifest).await;
    let fresh_heads = acceptance_child_heads(&reloaded).await;
    acceptance_close(&mut reloaded).await;
    if let Some(difference) = acceptance_view_difference(&saved, &fresh?) {
        return Ok(Some(difference));
    }
    Ok((fresh_heads? != heads).then(|| "the member head packs differ from the live members'".to_string()))
}

/// 🎞️ The child-lane scenario behind [`assert_child_history_edits_end_to_end`]; every instance is closed on every path.
async fn acceptance_child_scenario<A, M>(manifest: fn() -> App, seed: &[(&str, &str)]) -> Result<String, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut overwrite = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the seed does not land: {reason}"))?;
    let outcome = async {
        if let Some(difference) = acceptance_child_reload_difference(&mut overwrite, manifest).await? {
            return Err(format!("the seeded document with child-lane history does not reload identically: {difference}"));
        }
        let (target, store, label) = acceptance_child_row(&mut overwrite).await?.ok_or("the seed lands no editable child-lane mutation in the history")?;
        let before = acceptance_child_heads(&overwrite).await?;
        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None, None, None).await {
            Ok(change) => change,
            Err(verdict) => return Err(format!("the overwrite of {target} in {store}: {}", verdict.text())),
        };
        let heads = acceptance_child_heads(&overwrite).await?;
        if heads == before {
            return Err(format!("the overwrite {} = {:?} of {target} leaves every member head unchanged", change.0, change.1));
        }
        if let Some(difference) = acceptance_child_reload_difference(&mut overwrite, manifest).await? {
            return Err(format!("the overwritten document does not fold freshly to the same state after save and load: {difference}"));
        }
        Ok::<_, String>((change, store, label, heads))
    }
    .await;
    acceptance_close(&mut overwrite).await;
    let (change, store, label, heads) = outcome?;
    let mut seeded = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the alternative instance does not seed: {reason}"))?;
    let reloaded = acceptance_reloaded(&seeded, manifest).await;
    acceptance_close(&mut seeded).await;
    let mut branched = reloaded.map_err(|reason| format!("the seeded document does not reload for the alternative: {reason}"))?;
    let alternative = async {
        let (target, _, _) = acceptance_child_row(&mut branched).await?.ok_or("the reloaded seeded document lists no editable child-lane mutation")?;
        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE), None, None).await {
            Ok(_) => {}
            Err(verdict) => return Err(format!("the change the overwrite accepted fails as a new alternative on the reloaded document: {}", verdict.text())),
        }
        if acceptance_child_heads(&branched).await? != heads {
            return Err("the new alternative's member heads differ from the overwrite's".into());
        }
        if let Some(difference) = acceptance_child_reload_difference(&mut branched, manifest).await? {
            return Err(format!("the new alternative does not fold freshly to the same state after save and load: {difference}"));
        }
        Ok::<(), String>(())
    }
    .await;
    acceptance_close(&mut branched).await;
    alternative?;
    Ok(format!("{store}: {} = {:?}, row \"{label}\", seeded, overwritten and alternative documents reload identically", change.0, change.1))
}

/// 👶️ LAW (design §12, §20.15 — D20, audit F22): a child-lane mutation, the edit the plugin's `seed` gestures land in an owned member
/// store, is edited in history end to end through the generic member path (`historyEditBegin{mutationId, store}`). The seeded document
/// (carrying child-lane history) saved and freshly loaded through the app's own store initializer has the same head, owned children,
/// renders and member heads; the overwrite changes the member's head and the overwritten document reloads identically; the same change
/// committed as a new alternative on a second instance — whose session runs on the seeded document after a save → fresh load — reaches the
/// same member heads and reloads identically; the row is labelled in every locale. Answers the summary; panics naming `plugin` otherwise.
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
/// (transitively) that nothing published, or a payload validator that does not compile. One line per breach, naming the leaf. The
/// kinds of `withdraw_only` (declared `"editable": false`: no editor ever opens on them, design §22.20) are not judged.
pub fn input_schema_resolution_failures<A: ArtifactApp>(withdraw_only: &BTreeSet<String>) -> Vec<String> {
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
        if withdraw_only.contains(kind) {
            continue;
        }
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
/// leaves reference, as in production — every editable document leaf's payload schema resolves completely (a leaf under `fixtures`
/// declared `"editable": false` is withdraw-only and not judged). Answers the number of leaves read and the number declared
/// withdraw-only; panics naming `plugin` and every breaching leaf otherwise.
pub async fn assert_input_schemas_resolve<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path) -> (usize, usize)
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into())).await;
    let declared = acceptance_withdraw_only_kinds(fixtures);
    let failures = input_schema_resolution_failures::<A>(&declared);
    acceptance_close(&mut app).await;
    assert!(failures.is_empty(), "{plugin}: {} leaf payload schema(s) do not resolve through the runtime resolver:\n{}", failures.len(), failures.join("\n"));
    let withdraw_only = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.iter().filter(|descriptor| declared.contains(descriptor.semantic_kind)).count();
    (<A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS.len() - withdraw_only, withdraw_only)
}

/// 🆔️ The ids of `app`'s applied document mutations, in applied order.
fn acceptance_applied_ids<A, M>(app: &VcsArtifactApp<A, M>) -> Result<Vec<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    Ok(app.store.mutation_ops().map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?.iter().map(|op| op.mutation_id.0.clone()).collect())
}

/// ⛔️ Whether a replay outcome blocks finalizing: an error or a fatal (the floor `ReplayReport::blocks_finalize` reads).
fn acceptance_blocks(worst: Option<semio_framework_diagnostic::Severity>) -> bool {
    matches!(worst, Some(semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
}

/// 🧮️ The review after a replay, as the conflict law reads it: the session reviews, its report classifies every mutation of
/// `expected` (one outcome each), the status blocks exactly when an outcome is an error or a fatal, and `nextProblem` names the
/// first such outcome. Answers the outcomes in replay order and whether the review is blocked.
fn acceptance_classified<A, M>(app: &VcsArtifactApp<A, M>, expected: &[String]) -> Result<(Vec<(String, Option<semio_framework_diagnostic::Severity>)>, bool), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("the replay closed the session".into()))?;
    if status.stage != HistoryTimeTravelStage::Reviewing {
        return Err(AcceptanceVerdict::Fail(format!("the replay ends at {:?}, not a review", status.stage)));
    }
    let outcomes: Vec<(String, Option<semio_framework_diagnostic::Severity>)> = match app.time_travel.session().report.as_ref() {
        Some(report) => report.outcomes.iter().map(|outcome| (outcome.mutation_id.0.clone(), outcome.worst)).collect(),
        None => return Err(AcceptanceVerdict::Fail(format!("the review holds no replay report (review {:?}, fault {:?})", status.review, status.fault))),
    };
    if let Some(missing) = expected.iter().find(|id| !outcomes.iter().any(|(classified, _)| classified == *id)) {
        return Err(AcceptanceVerdict::Fail(format!("the replay report classifies {} mutation(s) but not the applied mutation {missing}", outcomes.len())));
    }
    let first = outcomes.iter().find(|(_, worst)| acceptance_blocks(*worst)).map(|(id, _)| id.clone());
    if (status.review == Some(HistoryTimeTravelReview::Blocked)) != first.is_some() || status.blocking != first.is_some() {
        return Err(AcceptanceVerdict::Fail(format!("the review is {:?} (blocking: {}) although the report's first blocking outcome is {first:?}", status.review, status.blocking)));
    }
    let named = status.next_problem.as_ref().map(|problem| problem.mutation_id.clone());
    if named != first {
        return Err(AcceptanceVerdict::Fail(format!("nextProblem names {named:?} although the report's first blocking outcome is {first:?}")));
    }
    Ok((outcomes, first.is_some()))
}

/// ✂️ A history row's Withdraw of the mutation `id` (it opens the session, or stacks into the open review), accepted and replayed to
/// its review. `Ok(false)`: the store's supersede law admits no withdrawal of it, nothing changed.
async fn acceptance_withdraw_accept<A, M>(app: &mut VcsArtifactApp<A, M>, id: &str) -> Result<bool, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_verb(app, HISTORY_EDIT_WITHDRAW_ACTION_ID, vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(id.to_string()))]).await {
        Ok(()) => {}
        Err(refusal) if refusal.ends_with(semio_framework_time_travel::TIME_TRAVEL_NOT_WITHDRAWABLE_CODE) => return Ok(false),
        Err(refusal) => return Err(AcceptanceVerdict::Fail(format!("the withdraw of {id} does not open: {refusal}"))),
    }
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal of {id} is not accepted: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    Ok(true)
}

/// 🧗️ Resolves a blocked review blocker by blocker: the mutation `nextProblem` names is withdrawn, accepted and replayed, until the
/// review is ready. Answers the ids withdrawn, in order; a blocker that cannot be withdrawn, one named twice, or more than
/// [`ACCEPTANCE_CASCADE_BLOCKERS`] of them break the mechanism (design §22.1: a blocking mutation is always resolvable).
async fn acceptance_resolve<A, M>(app: &mut VcsArtifactApp<A, M>, expected: &[String]) -> Result<Vec<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut resolved: Vec<String> = Vec::new();
    loop {
        let (_, blocked) = acceptance_classified(app, expected)?;
        if !blocked {
            return Ok(resolved);
        }
        if resolved.len() >= ACCEPTANCE_CASCADE_BLOCKERS {
            return Err(AcceptanceVerdict::Fail(format!("withdrawing {} blockers in turn does not reach a ready review", resolved.len())));
        }
        let blocker = app.time_travel.status().and_then(|status| status.next_problem).map(|problem| problem.mutation_id).ok_or_else(|| AcceptanceVerdict::Fail("a blocked review names no next problem".into()))?;
        if resolved.contains(&blocker) {
            return Err(AcceptanceVerdict::Fail(format!("nextProblem names {blocker} again after it was withdrawn")));
        }
        if !acceptance_withdraw_accept(app, &blocker).await? {
            return Err(AcceptanceVerdict::Fail(format!("the blocking mutation {blocker} cannot be withdrawn")));
        }
        resolved.push(blocker);
    }
}

/// 🏗️ Finalizes the ready review as an overwrite and compares the result with a fresh fold of `kept` on `case`'s document: the
/// live head and the head the document reloads to must both equal it. `Err` names the departure.
async fn acceptance_overwrite_folds<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, kept: &[&A::Mutation]) -> Result<(), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the resolved review does not finalize: {refusal}")))?;
    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, vec![("choice", DslValue::String(HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))]).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the resolved review does not commit: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    let head = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    let reloaded = acceptance_reloaded_head(app, manifest).await.map_err(|reason| AcceptanceVerdict::Fail(format!("after the conflict was resolved the finalized document does not survive save and load: {reason}")))?;
    let mut fresh = acceptance_seeded::<A, M>(manifest, &case.base, kept, false).await.map_err(|fault| AcceptanceVerdict::Fail(format!("a fresh fold of the effective history does not seed: {}", fault.reason())))?;
    let fold = acceptance_head(&fresh);
    acceptance_close(&mut fresh).await;
    let fold = fold.map_err(AcceptanceVerdict::Fail)?;
    if head != fold {
        return Err(AcceptanceVerdict::Fail(format!("after the conflict was resolved the overwrite head differs from a fresh fold of the effective history at {}", acceptance_text_difference(&head.1, &fold.1))));
    }
    if reloaded != fold {
        return Err(AcceptanceVerdict::Fail(format!("after the conflict was resolved the document reloads to a head differing from a fresh fold of the effective history at {}", acceptance_text_difference(&reloaded.1, &fold.1))));
    }
    Ok(())
}

/// 🧨️ The withdraw half of the conflict law on `app`, seeded with `seed` (the case's operation, then its inverse): the root is
/// withdrawn and the report classified; a review that blocks is resolved, restored to its original report, taken back without a
/// trace, resolved again and finalized. `Err(Skip)`: withdrawing the root blocks nothing (no dependents).
async fn acceptance_cascade_withdraw<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, seed: &[&A::Mutation]) -> Result<String, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let ids = acceptance_applied_ids(app)?;
    let root = ids.first().cloned().ok_or_else(|| AcceptanceVerdict::Fail("the seeded history left no applied operation".into()))?;
    if ids.len() != seed.len() {
        return Err(AcceptanceVerdict::SkipCase(format!("its history lists {} applied operations for {} seeded ones", ids.len(), seed.len())));
    }
    let before = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    if let Err(reason) = acceptance_reloaded_head(app, manifest).await {
        return Err(AcceptanceVerdict::SkipCase(format!("its seeded history does not survive save and load before any edit ({reason})")));
    }
    if !acceptance_withdraw_accept(app, &root).await? {
        return Err(AcceptanceVerdict::SkipCase("the store's law admits no withdrawal of its operation".into()));
    }
    let restore = |id: &str| vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(id.to_string()))];
    let (original, blocked) = acceptance_classified(app, &ids)?;
    if !blocked {
        acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(root.as_str())).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal of the root is not restored: {refusal}")))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
        return Err(AcceptanceVerdict::Skip("withdrawing its operation blocks nothing downstream".into()));
    }
    if acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new()).await.is_ok() {
        return Err(AcceptanceVerdict::Fail("a blocked review lets historyEditFinalize through".into()));
    }
    let resolved = acceptance_resolve(app, &ids).await?;
    for blocker in resolved.iter().rev() {
        acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(blocker.as_str())).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawn blocker {blocker} is not restored: {refusal}")))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    }
    let (again, _) = acceptance_classified(app, &ids)?;
    if again != original {
        return Err(AcceptanceVerdict::Fail(format!("restoring the {} resolved blocker(s) does not return to the original report: {again:?} instead of {original:?}", resolved.len())));
    }
    acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(root.as_str())).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal of the root is not restored: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    if app.store.supersessions().iter().any(|(id, _)| ids.contains(&id.0)) {
        return Err(AcceptanceVerdict::Fail("restoring every withdrawal left a supersession in the store".into()));
    }
    let after = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    if after != before {
        return Err(AcceptanceVerdict::Fail(format!("after every withdrawal was restored the head differs from the one before at {}", acceptance_text_difference(&after.1, &before.1))));
    }
    if !acceptance_withdraw_accept(app, &root).await? {
        return Err(AcceptanceVerdict::Fail("the root was withdrawn once and cannot be withdrawn again".into()));
    }
    let resolved = acceptance_resolve(app, &ids).await?;
    let kept: Vec<&A::Mutation> = ids.iter().zip(seed).skip(1).filter(|(id, _)| !resolved.contains(*id)).map(|(_, op)| *op).collect();
    acceptance_overwrite_folds(app, manifest, case, &kept).await?;
    let blocking = original.iter().filter(|(_, worst)| acceptance_blocks(*worst)).count();
    Ok(format!(
        "its withdrawal is classified over {} mutation(s) and blocks {blocking}; nextProblem named each of the {} blocker(s) resolved by withdrawal; restored to the original report and then without a trace; resolved again and finalized: head and reload equal the fresh fold of the {} mutation(s) left",
        original.len(),
        resolved.len(),
        kept.len()
    ))
}

/// 🎚️ The edit half of the conflict law on a fresh instance seeded with `seed`: an input of the root is drafted to another valid
/// value until one blocks the replay downstream; that review is resolved and finalized, and the head and the reload must equal a
/// fresh fold of the edited root followed by the mutations left. An input change that makes the root itself fail, or that only
/// withdrawing the root resolves, is no downstream conflict: it is taken back and the next one drafted. `Ok(None)`: no input change of
/// the root conflicts downstream.
async fn acceptance_cascade_edit<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, seed: &[&A::Mutation]) -> Result<Option<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = acceptance_seeded::<A, M>(manifest, &case.base, seed, true).await.map_err(|fault| AcceptanceVerdict::Fail(format!("the history that seeded once does not seed again: {}", fault.reason())))?;
    let outcome = async {
        let ids = acceptance_applied_ids(&app)?;
        let root = ids.first().cloned().ok_or_else(|| AcceptanceVerdict::Fail("the seeded history left no applied operation".into()))?;
        let begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(root.clone()))];
        acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
        let (inputs, original) = match app.time_travel.editor() {
            None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
            Some(editor) => (editor.inputs.clone(), editor.value.clone()),
        };
        for candidate in acceptance_changes(&inputs, &original) {
            if app.time_travel.status().is_none() {
                acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
            }
            if acceptance_verb(&mut app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_err()
                || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
            {
                continue;
            }
            acceptance_verb(&mut app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
            acceptance_pump(&mut app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
            if app.time_travel.status().is_none() {
                continue;
            }
            let (classified, blocked) = acceptance_classified(&app, &ids)?;
            let own = app.time_travel.status().and_then(|status| status.next_problem).is_some_and(|problem| problem.mutation_id == root);
            if !blocked || own {
                acceptance_verb(&mut app, HISTORY_EDIT_RESTORE_ACTION_ID, begin.clone()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("an accepted draft of the root is not restored: {refusal}")))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
                continue;
            }
            let resolved = acceptance_resolve(&mut app, &ids).await?;
            if resolved.contains(&root) {
                acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("a review that only withdrawing the edited root resolves does not exit: {refusal}")))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
                continue;
            }
            let edited = match app.time_travel.session().accepted_draft(&protocol::MutationId(root.clone())).map(|draft| &draft.replacement) {
                Some(protocol::InputReplacement::Input { payload, .. }) => <A::Mutation as ::protocol::OpBinary>::decode_op(payload).map_err(|error| AcceptanceVerdict::Fail(format!("the root's edited input does not decode: {error:?}")))?,
                Some(_) => return Err(AcceptanceVerdict::Fail("the root's accepted draft is a withdrawal, not its edited input".into())),
                None => return Err(AcceptanceVerdict::Fail("the resolved review no longer holds the root's draft".into())),
            };
            let kept: Vec<&A::Mutation> = std::iter::once(&edited).chain(ids.iter().zip(seed).skip(1).filter(|(id, _)| !resolved.contains(*id)).map(|(_, op)| *op)).collect();
            let folded = acceptance_overwrite_folds(&mut app, manifest, case, &kept).await;
            let left = kept.len() - 1;
            ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
            folded?;
            return Ok(Some(format!(
                "editing its input {} = {:?} is classified over {} mutation(s) and blocks downstream; {} blocker(s) resolved by withdrawal; finalized: head and reload equal the fresh fold of the edited root and the {left} mutation(s) left",
                candidate.0,
                candidate.1,
                classified.len(),
                resolved.len()
            )));
        }
        if app.time_travel.status().is_some() {
            let _ = acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        }
        Ok::<Option<String>, AcceptanceVerdict>(None)
    }
    .await;
    acceptance_close(&mut app).await;
    outcome
}

/// 🌪️ The conflict scenario for `case`: its operation followed by its own inverse on its own document is the history; the withdraw
/// half ([`acceptance_cascade_withdraw`]) must find a blocked review, then the edit half ([`acceptance_cascade_edit`]) runs on a
/// fresh instance. `Pass` carries both summaries; a skip says why this case shows no conflict; `Fail` is a broken mechanism.
async fn acceptance_cascade<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut probe = match acceptance_seeded::<A, M>(manifest, &case.base, &[], true).await {
        Ok(app) => app,
        Err(fault) => return AcceptanceVerdict::SkipBase(fault.reason()),
    };
    let inverse = match probe.store.snapshot() {
        Ok(before) => {
            let inverse = ::protocol::Mutation::<A::Snapshot>::inverse(&case.op, &before).map_err(|error| format!("its operation states no inverse on its document: {error:?}"));
            std::mem::forget(before);
            inverse
        }
        Err(error) => Err(format!("its document does not fold: {error:?}")),
    };
    acceptance_close(&mut probe).await;
    let inverse = match inverse {
        Ok(inverse) if !inverse.is_empty() => inverse,
        Ok(_) => return AcceptanceVerdict::SkipCase("its operation has an empty inverse on its document".into()),
        Err(reason) => return AcceptanceVerdict::SkipCase(reason),
    };
    let seed: Vec<&A::Mutation> = std::iter::once(&case.op).chain(&inverse).collect();
    let verdict = match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
        Err(AcceptanceSeedFault::Base(reason)) => AcceptanceVerdict::SkipBase(reason),
        Err(AcceptanceSeedFault::Operation(0, reason)) => AcceptanceVerdict::SkipCase(reason),
        Err(AcceptanceSeedFault::Operation(_, reason)) => AcceptanceVerdict::SkipCase(format!("its inverse does not seed after it: {reason}")),
        Ok(mut app) => {
            let withdrawn = acceptance_cascade_withdraw(&mut app, manifest, case, &seed).await;
            acceptance_close(&mut app).await;
            match withdrawn {
                Err(verdict) => verdict,
                Ok(summary) => match acceptance_cascade_edit::<A, M>(manifest, case, &seed).await {
                    Ok(Some(edited)) => AcceptanceVerdict::Pass(format!("{summary}; {edited}")),
                    Ok(None) => AcceptanceVerdict::Pass(format!("{summary}; no input change of the root conflicts downstream")),
                    Err(AcceptanceVerdict::Fail(reason)) => AcceptanceVerdict::Fail(format!("the edit scenario: {reason}")),
                    Err(verdict) => AcceptanceVerdict::Pass(format!("{summary}; the edit scenario was skipped: {}", verdict.text())),
                },
            }
        }
    };
    drop(seed);
    for op in inverse {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
    }
    verdict
}

/// 🏛️ LAW (design §23.2, the generic conflict case): on the first history of this app — a committed operation followed by its own
/// inverse, on its own document or a shipped one — whose first mutation, withdrawn, blocks the replay downstream, the conflict is
/// resolved end to end ([`acceptance_cascade`]). Answers that case's summary; an app none of whose histories blocks answers
/// "no dependents" with its census, one without a parent-lane leaf says so; panics naming `plugin`, the leaf and the case when the
/// mechanism breaks.
pub async fn assert_history_conflicts_resolve<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    if <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.is_empty() {
        return format!("{plugin}: no parent-lane leaf (conflicts between child-lane mutations are not driven by this law)");
    }
    let mut cases = acceptance_cases::<A>(fixtures);
    cases.extend(acceptance_example_cases::<A>(fixtures, examples).await);
    let (mut tried, mut unbuilt) = (0, 0);
    let (mut found, mut broken) = (None, None);
    let mut reasons: Vec<String> = Vec::new();
    let mut unloadable = BTreeSet::new();
    for case in &cases {
        if tried >= ACCEPTANCE_CASCADE_CASES || unbuilt >= ACCEPTANCE_CASCADE_UNBUILT {
            break;
        }
        if unloadable.contains(&case.base_key) {
            continue;
        }
        let leaf = protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
        match acceptance_cascade::<A, M>(manifest, case).await {
            AcceptanceVerdict::Pass(summary) => {
                found = Some(format!("{leaf} ({}) — {summary}", case.directory));
                break;
            }
            AcceptanceVerdict::Fail(reason) => {
                broken = Some(format!("{plugin}: the conflict law on {leaf} ({}) breaks the generic mechanism: {reason}", case.directory));
                break;
            }
            AcceptanceVerdict::Skip(reason) => {
                tried += 1;
                reasons.push(format!("{leaf}: {reason}"));
            }
            AcceptanceVerdict::SkipCase(reason) => {
                unbuilt += 1;
                reasons.push(format!("{leaf}: {reason}"));
            }
            AcceptanceVerdict::SkipBase(reason) => {
                unbuilt += 1;
                unloadable.insert(case.base_key.clone());
                reasons.push(format!("{leaf}: {reason}"));
            }
        }
    }
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    if let Some(failure) = broken {
        panic!("{failure}");
    }
    match found {
        Some(summary) => format!("{plugin}: {summary}"),
        None => format!(
            "{plugin}: no dependents — {tried} histories of a committed operation and its inverse block nothing when their first mutation is withdrawn, {unbuilt} could not be built (first reasons: {})",
            reasons.iter().take(3).map(|reason| reason.chars().take(160).collect::<String>()).collect::<Vec<_>>().join(" | ")
        ),
    }
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
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let (leaves, withdraw_only) =
                $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_input_schemas_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest, &fixtures));
            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve, {withdraw_only} withdraw-only leaf(s) not judged", $plugin);
        }

        /// ⚔️ LAW (design §23.2): a conflict a history edit of this editor causes downstream is classified, named and resolved end to end.
        #[test]
        fn conflict_history_edits_end_to_end() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_history_conflicts_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[history-edit-cascade] {summary}");
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

#[cfg(test)]
#[path = "🧪️tests/🔎️array-controls/🦀️.rs"]
mod array_controls;
