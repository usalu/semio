//! 🦀️ DAG exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.
//! Recorded no-oracle decision `dag-1-port-directed-graph-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`): `dag.dag` is a semio-NATIVE
//! port-directed computation graph with no third-party reader or writer, so `oracle` here reads the
//! committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_dag_mutation_reporting` over the full fourteen-kind `DagMutation`
//! vocabulary.
//!
//! **The one fact everything here turns on.** `DagSnapshot` persists NO nodes and NO edges — only
//! `schema` and one composed `s.stdio.semio.graph` child handle whose `childId` is a content digest
//! of the child. So the persisted projection moves if and only if the working scene moved, which
//! makes it an exact observability surface; and a committed `➡️after` for an APPLIED mutation would
//! have to carry a hand-forged `DefaultHasher` digest, which is why all fourteen committed vectors
//! are REJECTION vectors. Each `mutate-<kind>` handler therefore runs BOTH halves: the committed
//! vector for its exact `(code, severity, target)` triple with the handle required to stay put, and
//! the feature's own real-effect payload against the real committed five-node pipeline with the
//! handle required to move.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the dag artifact would make one plugin's
//! test tree a build dependency of another's. The laws are stated inline, in the same words and with
//! the same strictness.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`). The subset's
//! own production code exports the bridges instead: `decode_dag_snapshot_json`/
//! `encode_dag_snapshot_json`/`parse_dag_dsl`/`print_dag_dsl`/`dag_scene_summary`
//! (`…/🧬️schema/📸️snapshot/🦀️component.rs`) and `decode_dag_mutation_json`/
//! `apply_dag_mutation_reporting`/`inverse_dag_mutation_steps`/`seed_dag_working_scene_with`
//! (`…/🧬️schema/🧬️mutations/🦀️component.rs`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `DagMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
/// 🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not link the
/// subject crate. The contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "rename-node",
    "change-node-name",
    "move-node",
    "resize-node",
    "change-node-icon",
    "change-node-abbreviation",
    "change-node-operator-kind",
    "replace-node-kind",
    "replace-node-properties",
    "reorder-nodes",
    "connect-nodes",
    "disconnect-nodes",
];

/// 🗣️ The real committed pipeline — `slider → scale → combine → screen` with `mode` feeding
/// `combine`'s second input, five nodes over four edges.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` rejection vector TEXT for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted specification vector the
/// no-oracle decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "create-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/rejects-a-duplicate-node-id/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/rejects-a-duplicate-node-id/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/rejects-a-duplicate-node-id/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/rejects-a-duplicate-node-id/🎯️outcome/🔣️component.json"),
        ),
        "delete-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "rename-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🧪️tests/rejects-renaming-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🧪️tests/rejects-renaming-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🧪️tests/rejects-renaming-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🧪️tests/rejects-renaming-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "change-node-name" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🧪️tests/rejects-renaming-the-label-of-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🧪️tests/rejects-renaming-the-label-of-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🧪️tests/rejects-renaming-the-label-of-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-node-name/🧪️tests/rejects-renaming-the-label-of-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "move-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🧪️tests/rejects-moving-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🧪️tests/rejects-moving-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🧪️tests/rejects-moving-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-node/🧪️tests/rejects-moving-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "resize-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🧪️tests/rejects-resizing-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🧪️tests/rejects-resizing-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🧪️tests/rejects-resizing-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🧪️tests/rejects-resizing-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "change-node-icon" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🧪️tests/rejects-reiconing-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🧪️tests/rejects-reiconing-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🧪️tests/rejects-reiconing-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-icon/🧪️tests/rejects-reiconing-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "change-node-abbreviation" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🧪️tests/rejects-reabbreviating-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🧪️tests/rejects-reabbreviating-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🧪️tests/rejects-reabbreviating-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🧪️tests/rejects-reabbreviating-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "change-node-operator-kind" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🧪️tests/rejects-rebinding-the-operator-of-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🧪️tests/rejects-rebinding-the-operator-of-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🧪️tests/rejects-rebinding-the-operator-of-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-node-operator-kind/🧪️tests/rejects-rebinding-the-operator-of-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "replace-node-kind" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🧪️tests/rejects-rekinding-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🧪️tests/rejects-rekinding-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🧪️tests/rejects-rekinding-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-node-kind/🧪️tests/rejects-rekinding-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "replace-node-properties" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧪️tests/rejects-repropertying-a-missing-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧪️tests/rejects-repropertying-a-missing-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧪️tests/rejects-repropertying-a-missing-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧪️tests/rejects-repropertying-a-missing-node/🎯️outcome/🔣️component.json"),
        ),
        "reorder-nodes" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/rejects-a-duplicate-id-in-the-order/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/rejects-a-duplicate-id-in-the-order/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/rejects-a-duplicate-id-in-the-order/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/rejects-a-duplicate-id-in-the-order/🎯️outcome/🔣️component.json"),
        ),
        "connect-nodes" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-nodes/🧪️tests/rejects-a-missing-source-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-nodes/🧪️tests/rejects-a-missing-source-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-nodes/🧪️tests/rejects-a-missing-source-node/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-nodes/🧪️tests/rejects-a-missing-source-node/🎯️outcome/🔣️component.json"),
        ),
        "disconnect-nodes" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-disconnecting-a-missing-edge/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-disconnecting-a-missing-edge/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-disconnecting-a-missing-edge/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-disconnecting-a-missing-edge/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-dag-1: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally. For every kind in
/// this vocabulary that is byte-identical to the BEFORE snapshot, because every committed vector is
/// a refusal — the reference answer to "what does a refused dag mutation produce" is "the document
/// it was handed".
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_dag::artifacts::dag::standards::v1::subsets::any::schema::mutations::{apply_dag_mutation_reporting, decode_dag_mutation_json, inverse_dag_mutation_steps, seed_dag_working_scene_with, DagMutation};
    use semio_s_plugin_dag::artifacts::dag::standards::v1::subsets::any::schema::snapshot::{dag_scene_summary, decode_dag_snapshot_json, encode_dag_snapshot_json, parse_dag_dsl, print_dag_dsl, DagSnapshot};

    //#region 🔖️FixtureDecode
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<DagSnapshot, String> {
        decode_dag_snapshot_json(text).map_err(|error| format!("mutate-dag-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, label: &str, kind: &str) -> Result<DagMutation, String> {
        decode_dag_mutation_json(text).map_err(|error| format!("mutate-dag-1: the {label} payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &DagSnapshot) -> Result<Json, String> {
        parse_json(&encode_dag_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed — the persisted handle AND the scene it
    /// resolves to, because two documents that differ only in a content digest are unreadable
    /// without the scene beside them.
    fn disagreement(what: &str, got: &DagSnapshot, expected: &DagSnapshot) -> String {
        format!("{what}\n     got: {} {}\nexpected: {} {}", encode_dag_snapshot_json(got), dag_scene_summary(got), encode_dag_snapshot_json(expected), dag_scene_summary(expected))
    }

    /// 🗣️ The real committed pipeline, with its composed child resolved by the parse itself.
    fn pipeline(ctx: &Context) -> Result<DagSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("mutate-dag-1: the committed pipeline artifact is not UTF-8: {error}"))?;
        let parsed = parse_dag_dsl(&text)?;
        let summary = dag_scene_summary(&parsed);
        if !summary.contains("slider") || !summary.contains("screen") {
            return Err(format!("mutate-dag-1: the committed pipeline must resolve its composed child to the five-node signal chain, got {summary}"));
        }
        Ok(parsed)
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 🚫️ The refusal law: the committed vector must be refused with exactly the `(code, severity,
    /// target)` triple its `🎯️outcome/🔣️component.json` declares, the document must come back
    /// byte-identical to the committed `➡️after`, and — the part only this artifact can state — the
    /// content-addressed child handle must NOT have been re-minted. A refusal that quietly rebuilt
    /// the child would produce an identical scene under a new digest, which is the shape of bug a
    /// snapshot comparison alone cannot see here.
    fn vector_is_refused(kind: &str, code: &str, declared: &Json, raised: &[(String, String, Vec<String>)], before: &DagSnapshot, after: &DagSnapshot, expected: &DagSnapshot) -> Result<(), String> {
        if declared.str("status") != "rejected" {
            return Err(format!("mutate-{kind}: this case expects every committed dag vector to be a refusal, but its outcome declares {:?}", declared.str("status")));
        }
        if declared.str("code") != code {
            return Err(format!("mutate-{kind}: the feature's Examples row names code {code:?} but the committed outcome declares {:?} — the two declarations of the same vector have drifted", declared.str("code")));
        }
        let Some((raised_code, severity, target)) = raised.first() else {
            return Err(format!("mutate-{kind}: the committed vector must be refused with {code:?}, but the implementation raised nothing at all"));
        };
        if raised_code != code {
            return Err(format!("mutate-{kind}: the committed vector must be refused with {code:?}, but the implementation raised {raised_code:?}"));
        }
        let expected_severity = if code == "mutation.target-missing" { "Error" } else { "Fatal" };
        if severity != expected_severity {
            return Err(format!("mutate-{kind}: {code:?} is {expected_severity}-level in this vocabulary — a missing target is recoverable, a duplicate id or a broken invariant is not — but the implementation raised it as {severity}"));
        }
        let declared_target: Vec<String> = declared.array("path").iter().map(|entry| match entry { Json::String(value) => value.clone(), other => other.to_string() }).collect();
        if target != &declared_target {
            return Err(format!("mutate-{kind}: the committed outcome addresses {declared_target:?} but the implementation addressed {target:?}"));
        }
        if after != expected {
            return Err(disagreement(&format!("mutate-{kind}: a refused mutation must leave the document at the committed after-snapshot"), after, expected));
        }
        if after.content != before.content {
            return Err(format!("mutate-{kind}: a refused mutation re-minted the composed child handle ({} -> {}) — the scene may be unchanged, but the document is no longer the same document", before.content.child_id, after.content.child_id));
        }
        Ok(())
    }

    /// 👁️ The observability law, in the exact form this artifact can state it: an APPLIED mutation
    /// must move the content-addressed child handle. The handle is a digest of the child, so it
    /// moves if and only if the working scene moved — a kind whose forward effect never reached the
    /// scene would pass `mutate-<kind>` and `inverse-<kind>` identically to doing nothing.
    fn application_is_observable(kind: &str, raised: &[(String, String, Vec<String>)], base: &DagSnapshot, mutated: &DagSnapshot) -> Result<(), String> {
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the real-effect payload was meant to APPLY to the committed pipeline, but the implementation raised {raised:?}"));
        }
        if mutated.content.child_id == base.content.child_id {
            return Err(format!("mutate-{kind}: applying this kind to the real pipeline left the content-addressed child handle at {} — the mutation never reached the scene, so the scenario would report a pass for a mutation it never observed ({})", base.content.child_id, dag_scene_summary(mutated)));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Both halves in one scenario: the committed refusal vector for its exact diagnostic, then
    /// the feature's own real-effect payload against the real committed pipeline for its effect.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, vector, after, outcome) = super::fixture_text(kind);
            let mut base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let vector = mutation_of(vector, "committed vector", kind)?;
            seed_dag_working_scene_with(&mut base, &vector);
            let mut refused = base.clone();
            let raised = apply_dag_mutation_reporting(&mut refused, &vector);
            vector_is_refused(kind, &row.str("code"), &parse_json(outcome)?, &raised, &base, &refused, &expected)?;

            let pipeline = pipeline(ctx)?;
            let payload = mutation_of(&row.get("params").map(Json::to_string).unwrap_or_default(), "feature real-effect", kind)?;
            let mut applied = pipeline.clone();
            let applied_messages = apply_dag_mutation_reporting(&mut applied, &payload);
            application_is_observable(kind, &applied_messages, &pipeline, &applied)?;
            let projection = projection(&applied)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law over the real committed pipeline: applying the kind and then
    /// its OWN computed inverse must restore the document exactly, content handle included. Because
    /// that handle is a digest of the child, restoring it is the strongest possible statement that
    /// the whole scene came back — node order, port strings and edge endpoints included, not merely
    /// membership.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = pipeline(ctx)?;
            let payload = mutation_of(&row.get("params").map(Json::to_string).unwrap_or_default(), "feature real-effect", kind)?;
            let mut current = base.clone();
            let raised = apply_dag_mutation_reporting(&mut current, &payload);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current.content.child_id == base.content.child_id {
                return Err(format!("inverse-{kind}: the forward mutation left the content handle untouched, so restoring it proves nothing ({})", dag_scene_summary(&current)));
            }
            for step in inverse_dag_mutation_steps(&payload, &base) {
                let undone = apply_dag_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed pipeline through its own DSL carrier. `.dag.dsl.semio` is a
    /// fixed-layout record grammar — `dsl::print` emits the declared fields in schema order under
    /// the `semio dag.dag.dsl v1` preamble, with no writer freedom — and the committed example is
    /// this codec's own output, committed as such. The wave's usual "output must not equal input"
    /// tripwire therefore does not apply and would be the wrong law here; the byte-exact law is
    /// asserted instead, together with a scene check a parser returning an unresolved child cannot
    /// satisfy.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed pipeline artifact is not UTF-8: {error}"))?;
        let parsed = parse_dag_dsl(&text)?;
        let summary = dag_scene_summary(&parsed);
        for node in ["slider", "mode", "scale", "combine", "screen"] {
            if !summary.contains(node) {
                return Err(format!("identity-round-trip: the committed pipeline carries five nodes slider/mode/scale/combine/screen, but the parse resolved {summary}"));
            }
        }
        for edge in ["e1:", "e2:", "e3:", "e4:"] {
            if !summary.contains(edge) {
                return Err(format!("identity-round-trip: the committed pipeline carries four edges e1..e4, but the parse resolved {summary}"));
            }
        }
        let printed = print_dag_dsl(&parsed);
        let reparsed = parse_dag_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.dag.dsl.semio` is a fixed-layout record grammar and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
                printed.len(),
                text.len(),
                match at {
                    Some(offset) => format!(" (first at byte {offset})"),
                    None => String::new(),
                }
            ));
        }
        let projection = projection(&parsed)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// snapshot the oracle role can read literally, but the real pipeline is committed as `.dsl.semio`
/// text ONLY and turning that into a resolved document needs this subset's own codec, which the
/// oracle-only build must not link.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
