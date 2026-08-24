//! 🦀️ Semio DRAWING exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-drawing-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️drawing/🧪️oracle/🔣️component.json`, which also records why `usvg`/`resvg` and
//! `lyon`/`kurbo` were surveyed and DELIBERATELY declined rather than merely absent):
//! `s.stdio.semio.drawing` is a semio-NATIVE format with no third-party reader or writer, so
//! `oracle` here reads the committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_semio_drawing_mutation` over the full 17-kind `SemioDrawingMutation`
//! vocabulary.
//!
//! What distinguishes this subset is that its scene graph is ANONYMOUS. `DrawNode` is a recursive
//! `#[serde(tag = "kind")]` union (group/path/text/image) with no id field at all, so every
//! node-addressed verb is keyed by a structural `NodePath` — `{"layer": 0, "path": [2]}` — rather
//! than by an identity, and a verb that changed a node's POSITION among its siblings would silently
//! re-target every later verb. Only layers and styles are named: `delete-layer` addresses `id`, and
//! `replace-fill`/`change-stroke-color`/`change-stroke-width` address `style_name` in a table the
//! nodes reference by name, so a style edit must reach every node using it without touching the
//! node tree at all. Four of the seventeen kinds are hierarchy rewrites — `group`/`ungroup` and
//! `flatten`/`unflatten` — which are declared inverses of each other, and `unflatten` is the one
//! payload here that carries a whole captured subtree so the flattening can be undone exactly.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never gets two sides to compare. Every law this case claims is therefore
//! asserted INSIDE the subject handler: `mutate-<kind>` checks the applied snapshot IS the committed
//! after-snapshot, `inverse-<kind>` checks the mutation's own computed inverse restores the
//! committed before-snapshot, and `identity-round-trip` crosses the two committed encodings of the
//! real sketch artifact against each other. A handler that merely returned `Ok` would report a pass
//! having checked nothing at all.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`), so neither
//! `protocol::Mutation` nor a `serde` derive is nameable from here, and hand-transcribing a
//! recursive scene graph into a Rust literal would be a second copy of the fixture free to drift
//! away from it. The subset's own production code therefore exports the bridges this adapter needs,
//! whose signatures name only reachable types: `decode_semio_drawing_snapshot_json`/
//! `encode_semio_drawing_snapshot_json` and the DSL/pack pass-throughs (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️drawing/🧬️schema/📸️snapshot/🦀️component.rs`), `decode_semio_drawing_mutation_json`/
//! `inverse_semio_drawing_mutation` (`…/🧬️mutations/🦀️component.rs`). The subject half is gated
//! behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation; the Rust SUBJECT phase is blocked this wave by concurrent framework refactors
//! (see 📓️w7-fleet-brief.md), so it is written and gated but not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioDrawingMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/
/// 🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate. The contract's mutation-coverage gate keeps this list honest
/// against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it
/// honest against both the enum and the manifest.
const KINDS: &[&str] = &[
    "create-layer",
    "delete-layer",
    "create-node",
    "delete-node",
    "move-node",
    "drag-nodes",
    "rotate",
    "scale",
    "reorder-nodes",
    "group",
    "ungroup",
    "flatten",
    "unflatten",
    "replace-path",
    "replace-fill",
    "change-stroke-color",
    "change-stroke-width",
];

/// 🗣️ The real committed sketch — one visible layer whose group root holds a path exercising every
/// segment kind (move/line/cubic/quadratic/arc/close), a text node, an image node carrying real
/// `image/png` bytes, and an empty nested group.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🖍️sketch/🖼️assets/🗣️example.dsl.semio";
/// 🎒️ The same sketch in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🖍️sketch/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` fixture TEXT for one kind, read literally via
/// `include_str!` — this IS the independently handcrafted specification vector the no-oracle
/// decision rests on, never recomputed. One `include_str!` per file for the whole adapter: `oracle`
/// answers with `before`/`after`, `subject` decodes all three.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "create-layer" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/inserts-a-second-layer-above-the-base-layer/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/inserts-a-second-layer-above-the-base-layer/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/inserts-a-second-layer-above-the-base-layer/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-layer" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-the-leading-layer-and-keeps-the-overlay/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-the-leading-layer-and-keeps-the-overlay/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-the-leading-layer-and-keeps-the-overlay/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➕create-node/🧪️tests/appends-a-caption-text-node-to-the-layer-root/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➕create-node/🧪️tests/appends-a-caption-text-node-to-the-layer-root/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➕create-node/🧪️tests/appends-a-caption-text-node-to-the-layer-root/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➖delete-node/🧪️tests/removes-the-text-node-from-the-layer-root/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➖delete-node/🧪️tests/removes-the-text-node-from-the-layer-root/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/➖delete-node/🧪️tests/removes-the-text-node-from-the-layer-root/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "move-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-text-node-to-a-new-origin/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-text-node-to-a-new-origin/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-text-node-to-a-new-origin/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "drag-nodes" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🧪️tests/drags-the-text-node-and-the-nested-group-by-the-same-offset/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🧪️tests/drags-the-text-node-and-the-nested-group-by-the-same-offset/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🧪️tests/drags-the-text-node-and-the-nested-group-by-the-same-offset/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "rotate" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔄rotate/🧪️tests/rotates-the-nested-group-a-half-turn-about-z/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔄rotate/🧪️tests/rotates-the-nested-group-a-half-turn-about-z/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔄rotate/🧪️tests/rotates-the-nested-group-a-half-turn-about-z/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "scale" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📏scale/🧪️tests/scales-the-nested-group-non-uniformly/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📏scale/🧪️tests/scales-the-nested-group-non-uniformly/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📏scale/🧪️tests/scales-the-nested-group-non-uniformly/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "reorder-nodes" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/moves-the-leading-path-node-to-the-end-of-the-layer-root/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/moves-the-leading-path-node-to-the-end-of-the-layer-root/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🧪️tests/moves-the-leading-path-node-to-the-end-of-the-layer-root/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "group" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🧷group/🧪️tests/groups-the-two-leading-children-into-a-new-group/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🧷group/🧪️tests/groups-the-two-leading-children-into-a-new-group/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🧷group/🧪️tests/groups-the-two-leading-children-into-a-new-group/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "ungroup" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💫ungroup/🧪️tests/dissolves-the-nested-group-into-its-parent/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💫ungroup/🧪️tests/dissolves-the-nested-group-into-its-parent/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/💫ungroup/🧪️tests/dissolves-the-nested-group-into-its-parent/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "flatten" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🫓flatten/🧪️tests/flattens-an-identity-nested-group-into-its-leaves/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🫓flatten/🧪️tests/flattens-an-identity-nested-group-into-its-leaves/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🫓flatten/🧪️tests/flattens-an-identity-nested-group-into-its-leaves/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "unflatten" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🎈unflatten/🧪️tests/restores-the-captured-hierarchy-over-the-flat-group/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🎈unflatten/🧪️tests/restores-the-captured-hierarchy-over-the-flat-group/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🎈unflatten/🧪️tests/restores-the-captured-hierarchy-over-the-flat-group/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-path" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🧪️tests/swaps-the-open-path-for-a-closed-triangle/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🧪️tests/swaps-the-open-path-for-a-closed-triangle/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🧪️tests/swaps-the-open-path-for-a-closed-triangle/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-fill" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🧪️tests/repaints-the-primary-styles-fill-from-red-to-blue/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🧪️tests/repaints-the-primary-styles-fill-from-red-to-blue/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🧪️tests/repaints-the-primary-styles-fill-from-red-to-blue/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-stroke-color" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🧪️tests/recolours-the-primary-styles-stroke-to-translucent-white/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🧪️tests/recolours-the-primary-styles-stroke-to-translucent-white/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🧪️tests/recolours-the-primary-styles-stroke-to-translucent-white/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-stroke-width" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🧪️tests/thickens-the-primary-styles-stroke/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🧪️tests/thickens-the-primary-styles-stroke/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🧪️tests/thickens-the-primary-styles-stroke/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-drawing: no fixture registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{apply_semio_drawing_mutation, decode_semio_drawing_mutation_json, inverse_semio_drawing_mutation, SemioDrawingMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{decode_semio_drawing_pack, decode_semio_drawing_snapshot_json, encode_semio_drawing_pack, encode_semio_drawing_snapshot_json, parse_semio_drawing_dsl, print_semio_drawing_dsl, SemioDrawingSnapshot};

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridges — real deserialization of the committed bytes,
    /// recursive `DrawNode` union and all.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<SemioDrawingSnapshot, String> {
        decode_semio_drawing_snapshot_json(text).map_err(|error| format!("mutate-semio-drawing: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<SemioDrawingMutation, String> {
        decode_semio_drawing_mutation_json(text).map_err(|error| format!("mutate-semio-drawing: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &SemioDrawingSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_drawing_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both fixtures are written
    /// in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioDrawingSnapshot, expected: &SemioDrawingSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_drawing_snapshot_json(got), encode_semio_drawing_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — the whole scene graph, so a `group` that collected the right two children
    /// but left them behind in the parent fails here. The assertion lives in the handler because a
    /// recorded no-oracle case runs no oracle role: one that merely returned `Ok` would report a
    /// pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after) = super::fixture_text(kind);
            let mut current = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let outcome = apply_semio_drawing_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: the mutation was rejected: {:?}", outcome.messages()));
            }
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &current, &expected));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly. This is the sharpest check this subset has,
    /// because nodes carry no identity: an undone `ungroup` has to put its children back at the
    /// index they came from, under a group carrying the transform it had, or the `NodePath` every
    /// other verb addresses by no longer means what it meant.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let outcome = apply_semio_drawing_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {:?}", outcome.messages()));
            }
            for step in inverse_semio_drawing_mutation(&mutation, &base) {
                let step_outcome = apply_semio_drawing_mutation(&mut current, &step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {:?}", step_outcome.messages()));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed sketch through both of its committed encodings — the one document that
    /// carries every `DrawNode` variant and every `PathSegment` variant at once, so a codec that
    /// dropped an arc or an image payload shows up here rather than in a per-kind fixture that never
    /// contains one. The DSL text and the pack envelope are separate committed files produced by
    /// separate codecs, so agreeing on one snapshot cannot be achieved by smuggling bytes from
    /// either. Byte-identical re-emission IS expected — the committed text is this codec's own
    /// output, not a foreign writer's — so the wave's usual "output must not equal input" tripwire
    /// does not apply and the text/binary cross-check carries that evidence instead.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed sketch artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_drawing_dsl(&text)?;
        if parsed.layers.len() != 1 || parsed.styles.len() != 1 {
            return Err(format!("identity-round-trip: the committed sketch is the one-layer, one-style fixture this case describes, but parsed {} layer(s) and {} style(s)", parsed.layers.len(), parsed.styles.len()));
        }
        let reparsed = parse_semio_drawing_dsl(&print_semio_drawing_dsl(&parsed))?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let unpacked = decode_semio_drawing_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different sketch than the committed text artifact", &unpacked, &parsed));
        }
        let repacked = decode_semio_drawing_pack(&encode_semio_drawing_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
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
/// deliberately subject-only: the reference answer for the other scenarios is a committed JSON
/// snapshot the oracle role can read literally, but the sketch is committed as DSL and pack bytes
/// ONLY, and turning those into a snapshot needs this subset's own codec — which the oracle-only
/// build must not link.
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
