//! 🦀️ Semio TEXT exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-text-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️text/🧪️oracle/🔣️component.json`): `s.stdio.semio.text` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️text/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_text_mutation`, the entry point this ticket added, over the full 7-kind
//! `SemioTextMutation` vocabulary. Both sides project the snapshot to structural JSON and
//! `ordered-json-v1` compares them. The oracle-only build must never link the subject crate (fleet
//! brief §5.3), so the fixtures' BEFORE snapshot and MUTATION payload are transcribed once, by
//! hand, as `SemioTextSnapshot`/`SemioTextMutation` Rust literals inside the `sut`-gated `subject`
//! module below — mechanically identical to the committed JSON, never independently invented
//! (compare against the JSON embedded via `include_str!` in `oracle_fixture`). The generated
//! test-host crate carries no `serde_json` dependency (only `semio-repo-test-host` and, behind
//! `sut`, this subset's own crate), so parsing committed JSON straight into typed structs is not an
//! option here; the framework's own dependency-free `protocol::Json`/`parse_json` carries the
//! oracle side instead. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation; the Rust SUBJECT phase is blocked this
//! wave by a concurrent os-kernel refactor (see the fleet brief), so it is written and gated but
//! not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioTextMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["insert-run", "remove-run", "edit-run", "change-run-language", "reorder-runs", "add-mark", "remove-mark"];
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🧫️ The committed `(before, after)` snapshot JSON for one kind, read literally — this IS the
/// independently handcrafted specification vector the no-oracle decision rests on, never
/// recomputed.
fn oracle_fixture(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "insert-run" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🧪️tests/inserts-a-german-run-between-two-english-runs/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🧪️tests/inserts-a-german-run-between-two-english-runs/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-run" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🧪️tests/removes-the-middle-run/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🧪️tests/removes-the-middle-run/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "edit-run" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🧪️tests/rewrites-the-marked-runs-content/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🧪️tests/rewrites-the-marked-runs-content/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-run-language" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🧪️tests/retags-the-second-run-as-german/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🧪️tests/retags-the-second-run-as-german/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "reorder-runs" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🧪️tests/moves-the-first-run-to-the-end/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🧪️tests/moves-the-first-run-to-the-end/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-mark" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🧪️tests/adds-a-link-mark-ahead-of-the-bold-mark/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🧪️tests/adds-a-link-mark-ahead-of-the-bold-mark/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-mark" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🧪️tests/detaches-the-italic-mark-from-the-run/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🧪️tests/detaches-the-italic-mark-from-the-run/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-text: no fixture registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::mutations::{add_mark, change_run_language, edit_run, insert_run, remove_mark, remove_run, reorder_runs, SemioTextMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun, SemioTextSnapshot};
    use protocol::Mutation;

    //#region 🔖️HandcraftedFixtures
    /// 🧫️ The SAME specification vector `../🦀️component.rs::oracle_fixture` embeds as JSON,
    /// transcribed once by hand into real `SemioTextSnapshot`/`SemioTextMutation` values — the
    /// oracle-only build must never link this crate, so there is no way to share one physical
    /// source between the two roles; committed side by side under the same kind's `🧪️tests/`
    /// directory, so a drift between them is a one-file diff away from being caught by eye.
    fn run(language: &str, content: &str, marks: Vec<SemioTextMark>) -> SemioTextRun {
        SemioTextRun { language: language.into(), content: content.into(), marks }
    }
    fn mark(kind: SemioTextMarkKind, href: &str) -> SemioTextMark {
        SemioTextMark { kind, href: href.into() }
    }
    fn snapshot(runs: Vec<SemioTextRun>) -> SemioTextSnapshot {
        SemioTextSnapshot { schema: "s.stdio.semio.text".into(), runs }
    }

    fn fixture_for(kind: &str) -> (SemioTextSnapshot, SemioTextMutation) {
        match kind {
            "insert-run" => (
                snapshot(vec![run("en", "Good morning", vec![]), run("en", "Goodbye", vec![])]),
                SemioTextMutation::InsertRun(insert_run::mutation::InsertRun { index: 1, run: run("de", "Guten Morgen", vec![]) }),
            ),
            "remove-run" => (snapshot(vec![run("en", "Alpha", vec![]), run("de", "Beta", vec![]), run("en", "Gamma", vec![])]), SemioTextMutation::RemoveRun(remove_run::mutation::RemoveRun { index: 1 })),
            "edit-run" => (
                snapshot(vec![run("en", "Hello", vec![]), run("en", "world", vec![mark(SemioTextMarkKind::Bold, "")])]),
                SemioTextMutation::EditRun(edit_run::mutation::EditRun { index: 1, new_content: "planet".into() }),
            ),
            "change-run-language" => (snapshot(vec![run("en", "Hello", vec![]), run("en", "Welt", vec![])]), SemioTextMutation::ChangeRunLanguage(change_run_language::mutation::ChangeRunLanguage { index: 1, new_language: "de".into() })),
            "reorder-runs" => (snapshot(vec![run("en", "one", vec![]), run("en", "two", vec![]), run("en", "three", vec![])]), SemioTextMutation::ReorderRuns(reorder_runs::mutation::ReorderRuns { from: 0, to: 2 })),
            "add-mark" => (snapshot(vec![run("en", "semio", vec![mark(SemioTextMarkKind::Bold, "")])]), SemioTextMutation::AddMark(add_mark::mutation::AddMark { run_index: 0, index: 0, mark: mark(SemioTextMarkKind::Link, "https://semio.tech") })),
            "remove-mark" => (snapshot(vec![run("en", "emphasis", vec![mark(SemioTextMarkKind::Bold, ""), mark(SemioTextMarkKind::Italic, "")])]), SemioTextMutation::RemoveMark(remove_mark::mutation::RemoveMark { run_index: 0, index: 1 })),
            other => panic!("mutate-semio-text: no fixture registered for kind {other:?}"),
        }
    }
    //#endregion 🔖️HandcraftedFixtures

    //#region 🔖️Projection
    fn mark_kind_str(kind: SemioTextMarkKind) -> &'static str {
        match kind {
            SemioTextMarkKind::Bold => "bold",
            SemioTextMarkKind::Italic => "italic",
            SemioTextMarkKind::Code => "code",
            SemioTextMarkKind::Link => "link",
        }
    }
    fn mark_json(mark: &SemioTextMark) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(mark_kind_str(mark.kind).to_string())), ("href".to_string(), Json::String(mark.href.clone()))])
    }
    fn run_json(run: &SemioTextRun) -> Json {
        Json::Object(vec![("language".to_string(), Json::String(run.language.clone())), ("content".to_string(), Json::String(run.content.clone())), ("marks".to_string(), Json::Array(run.marks.iter().map(mark_json).collect()))])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field.
    fn snapshot_json(snapshot: &SemioTextSnapshot) -> Json {
        Json::Object(vec![("schema".to_string(), Json::String(snapshot.schema.clone())), ("runs".to_string(), Json::Array(snapshot.runs.iter().map(run_json).collect()))])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (mut base, mutation) = fixture_for(kind);
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::mutations::apply_semio_text_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            let projection = snapshot_json(&base);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (base, mutation) = fixture_for(kind);
            let mut current = base.clone();
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::mutations::apply_semio_text_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            let undo = mutation.inverse(&base);
            for step in &undo {
                let step_outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::mutations::apply_semio_text_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            let projection = snapshot_json(&current);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
