//! ⏪️ The cross-plugin history-edit acceptance law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §16.3, gap G12):
//! for a representative editable leaf of any app, read from the app's own committed mutation fixtures, the real verbs run
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

/// 🧫️ One committed fixture case the law can seed: its directory, the DSL text of the document before it, the canonical JSON
/// of that document (cases sharing it can follow each other downstream) and its operation.
pub struct AcceptanceCase<Mu> {
    pub directory: String,
    pub base: String,
    pub base_key: String,
    pub op: Mu,
}

/// ⚖️ The outcome of one acceptance scenario: it passed (with its summary), its data cannot exercise the flow (`Skip`, the next
/// case is tried) or the generic mechanism broke (`Fail`, the law fails at once).
enum AcceptanceVerdict {
    Pass(String),
    Skip(String),
    Fail(String),
}

/// 🧫️ Every committed fixture case under `root` whose outcome applies, whose document decodes as `A::Snapshot`, whose operation
/// decodes as `A::Mutation`, and whose operation the generic editor edits (an input schema and no foreign-step capability) — in
/// path order. A `{mutation, before, after}` case record carries its own document.
pub fn acceptance_cases<A: ArtifactApp>(root: &std::path::Path) -> Vec<AcceptanceCase<A::Mutation>> {
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
    fn json(path: &std::path::Path) -> Option<DslValue> {
        let text = std::fs::read_to_string(path).ok()?;
        dsl::os_pack::json::parse(&text).ok().map(|json| dsl::os_pack::json::to_dsl_value(&json))
    }
    let mut directories = Vec::new();
    walk(root, &mut directories);
    directories.sort();
    let mut cases = Vec::new();
    for directory in directories {
        let Some(wire) = json(&directory.join("🦠️mutation").join("🔣️.json")) else { continue };
        let record = wire.get("before").is_some() && wire.get("after").is_some() && wire.get("mutation").and_then(DslValue::as_object).is_some();
        let applied = record || json(&directory.join("🎯️outcome").join("🔣️.json")).is_some_and(|outcome| outcome.get("status").and_then(DslValue::as_str) == Some("applied"));
        let before = if record { wire.get("before").cloned() } else { json(&directory.join("📸️snapshot").join("⬅️before").join("🔣️.json")) };
        let (Some(before), true) = (before, applied) else { continue };
        let Ok(op) = <A::Mutation as protocol::FromValue>::from_value(if record { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire }) else { continue };
        if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none() || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
            continue;
        }
        let base_key = format!("{before:?}");
        let Ok(snapshot) = <A::Snapshot as protocol::FromValue>::from_value(before) else {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
            continue;
        };
        let base = store::ArtifactDsl::print_dsl(&snapshot);
        std::mem::forget(snapshot);
        cases.push(AcceptanceCase { directory: directory.to_string_lossy().into_owned(), base, base_key, op });
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
    ActionMeta { view_state: Some(ViewModel::default()), ..artifact_app_laws::meta(ACCEPTANCE_ACTOR) }
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
/// of `ops`; `Err` names why the data cannot be seeded, after closing the instance.
async fn acceptance_seeded<A, M>(manifest: fn() -> App, base: &str, ops: &[&A::Mutation]) -> Result<VcsArtifactApp<A, M>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        app.store.set_local_actor_id(Some(ACCEPTANCE_ACTOR.to_string())).map_err(|error| format!("the actor is refused: {error:?}"))?;
        let mut files = app.document_text().await.map_err(|fault| format!("the document does not print: {fault:?}"))?;
        files.dsl = base.to_string();
        app.load_document_text(&files).await.map_err(|fault| format!("the base document does not load: {fault:?}"))?;
        for op in ops {
            let receipt = app.store.dispatch(ArtifactCommand::Apply { mutations: vec![(*op).clone()], description: None, transaction: None }).await.map_err(|error| format!("the seed edit is refused: {error:?}"))?;
            if matches!(receipt.worst, Some(dsl::Severity::Error | dsl::Severity::Fatal)) {
                return Err(format!("the seed edit does not apply cleanly: {:?}", receipt.messages));
            }
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

/// 🧾️ The document head as its canonical value; the clone is never dropped, so a fail-closed root never trips.
fn acceptance_head<A, M>(app: &VcsArtifactApp<A, M>) -> Result<DslValue, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let head = app.store.snapshot().map_err(|error| format!("the head does not fold: {error:?}"))?;
    let value = protocol::ToValue::to_value(&head);
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
    let mut overwrite = match acceptance_seeded::<A, M>(manifest, &case.base, &seed).await {
        Ok(app) => app,
        Err(reason) => return AcceptanceVerdict::Skip(reason),
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
    let mut fresh = match acceptance_seeded::<A, M>(manifest, &case.base, &edited_seed).await {
        Ok(app) => app,
        Err(reason) => {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
            return AcceptanceVerdict::Fail(format!("a fresh fold of the edited log does not seed: {reason}"));
        }
    };
    let fold = acceptance_head(&fresh);
    acceptance_close(&mut fresh).await;
    let verdict = match (label, head, fold) {
        (Err(reason), _, _) | (_, Err(reason), _) | (_, _, Err(reason)) => AcceptanceVerdict::Fail(reason),
        (Ok(_), Ok(head), Ok(fold)) if head != fold => AcceptanceVerdict::Fail(format!("the overwrite head differs from a fresh fold of the edited log after {} = {:?}", change.0, change.1)),
        (Ok(label), Ok(_), Ok(fold)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed).await {
            Err(reason) => AcceptanceVerdict::Fail(format!("the alternative instance does not seed: {reason}")),
            Ok(mut branched) => {
                let verdict = match acceptance_session(&mut branched, Some(&change), Some(ACCEPTANCE_ALTERNATIVE)).await {
                    Err(AcceptanceVerdict::Skip(reason)) => AcceptanceVerdict::Fail(format!("the change the overwrite accepted fails as a new alternative: {reason}")),
                    Err(verdict) => verdict,
                    Ok((_, alternative_edit)) => {
                        let head = acceptance_head(&branched);
                        ::protocol::Mutation::<A::Snapshot>::retire_cold(alternative_edit);
                        match head {
                            Ok(head) if head == fold => AcceptanceVerdict::Pass(format!("{} = {:?}, row \"{label}\"{}", change.0, change.1, if downstream.is_some() { ", one downstream edit replayed" } else { "" })),
                            Ok(_) => AcceptanceVerdict::Fail("the new alternative's head differs from a fresh fold of the edited log".into()),
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

/// ⚖️ LAW (design §16.3): the app's committed fixtures under `fixtures` hold a representative editable leaf whose history edit
/// runs end to end through the generic mechanism — overwrite and new alternative, each head a fresh fold of the edited log, the
/// row labelled in every locale. Cases whose data cannot exercise the flow are skipped (a downstream edit is tried first, then
/// the leaf alone); the first passing case is the representative; a broken mechanism fails at once. Answers the passing case's
/// summary; panics naming `plugin`, the leaf and the case otherwise.
pub async fn assert_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let cases = acceptance_cases::<A>(fixtures);
    let mut skipped = Vec::new();
    let mut passed = None;
    'cases: for case in &cases {
        let leaf = protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
        let downstream: Vec<&A::Mutation> = cases.iter().filter(|other| other.base_key == case.base_key && other.directory != case.directory).map(|other| &other.op).take(ACCEPTANCE_DOWNSTREAM_PER_LEAF).collect();
        for next in downstream.into_iter().map(Some).chain(std::iter::once(None)) {
            match acceptance_scenario::<A, M>(manifest, case, next).await {
                AcceptanceVerdict::Pass(summary) => {
                    passed = Some(format!("{plugin}: {leaf} ({}) — {summary}", case.directory));
                    break 'cases;
                }
                AcceptanceVerdict::Skip(reason) => skipped.push(format!("{leaf} ({}): {reason}", case.directory)),
                AcceptanceVerdict::Fail(reason) => panic!("{plugin}: the history edit of {leaf} ({}) breaks the generic mechanism: {reason}", case.directory),
            }
        }
    }
    let count = cases.len();
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    passed.unwrap_or_else(|| panic!("{plugin}: none of the {count} editable fixture case(s) under {} exercises a history edit end to end:\n{}", fixtures.display(), skipped.join("\n")))
}

/// ⏪️ Wires [`assert_history_edits_end_to_end`] into a plugin crate's test target for one editor: `plugin` names the failure,
/// `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$fixtures` the fixture root relative to the crate's
/// manifest directory (the artifact tree: `"../.."`).
#[macro_export]
macro_rules! history_edit_acceptance_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $fixtures:literal) => {
        /// ⏪️ LAW (design §16.3): a representative editable leaf of this editor is edited in history end to end.
        #[semio_framework_async_macros::async_test]
        async fn history_edits_end_to_end() {
            let summary = $crate::app::history_edit_acceptance::assert_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest, &::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures)).await;
            ::std::println!("[history-edit-acceptance] {summary}");
        }
    };
}
