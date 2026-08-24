//! 🦀️ Semio KIT exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-kit-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️kit/🧪️oracle/🔣️component.json`): `s.stdio.semio.kit` is a semio-NATIVE format
//! with no third-party reader or writer, so `oracle` here reads the committed, independently
//! handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/
//! 🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no reimplementation of
//! mutation semantics. `subject` drives this repository's own `apply_semio_kit_mutation`, the
//! entry point this ticket added, over the full 15-kind `SemioKitMutation` vocabulary.
//!
//! Unlike `✳️text` (this wave's first case), `kit` is a COMPOSITE subset: `SemioKitSnapshot` embeds
//! `store::ArtifactChild<S>`/`store::ArtifactLink`/`store::LinkPin` fields directly (owned
//! objects/models/properties children, an independent-lifecycle representations LINK pool), and
//! several mutation payloads (`create-object`/`create-model`/`create-properties`/`bind-
//! representation`/`change-representation-pin`) carry `store::os_io::ArtifactRef`/`store::LinkPin`
//! fields too. `store` (like `dsl`/`protocol`) is a PRIVATE `extern crate semio_framework_os_kernel
//! as store;` item declared in this crate's own `📦️glue.rs` — confirmed by grep across the whole
//! plugin: no `pub use`/`pub extern crate` re-export of it, or of `serde`/`serde_json`, exists
//! anywhere. The generated test-host crate (`materializeRustHost`, `🧰️framework/…/🧪️test/
//! 📜️script.ts`) links only `semio-repo-test-host` (dependency-free) and, behind `sut`, THIS crate
//! with `default-features = false` — `semio_framework_os_kernel` is never a direct dependency of
//! the adapter crate itself, so it never enters that crate's extern prelude, so nothing here can
//! write `store::…`/`protocol::…` to hand-construct a `SemioKitSnapshot`/`SemioKitMutation`
//! literal the way `✳️text`'s adapter constructs `SemioTextSnapshot`/`SemioTextMutation` literals
//! (`✳️text` has no `store`-typed fields at all, so it never meets this wall). `mutate-binary-raw`'s
//! own adapter independently discovered and documented the identical `store`-unreachability
//! constraint for `ArtifactPack`. The same wall blocks `protocol::Mutation` — `✳️text`'s own
//! `subject::inverse` writes `use protocol::Mutation;` to call `mutation.inverse(&base)`, which is
//! a TRAIT method, not an inherent one; that import cannot resolve from outside this crate either,
//! so that precedent has an equivalent latent gap, currently masked only because (a) the real
//! `--features sut` build is itself blocked by the unrelated os-kernel refactor and (b) the
//! standalone `rustc` sanity check has no `--extern` flags at all, so a genuine "cannot resolve
//! `protocol`" failure is indistinguishable there from an expected "crate not linked in this
//! standalone check" failure. Both problems share one fix, applied to THIS subset's own production
//! code (in scope — `🧿️semio/✳️kit` is not the "other subsets" the brief says to leave alone):
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/🦀️component.rs` now exports
//! `decode_kit_snapshot_json`/`encode_kit_snapshot_json`, and `…/🧬️mutations/🦀️component.rs` now
//! exports `decode_kit_mutation_json`/`inverse_semio_kit_mutation` — four thin, PERMANENT
//! `serde_json`/`protocol::Mutation`-backed wrappers (`serde_json` is already a direct dependency
//! of this crate; using it behind an interface is exactly CLAUDE.md's "external libraries behind an
//! interface" rule, never a new dependency) whose SIGNATURES only name `&str`/`String`/`Vec`/
//! `SemioKitSnapshot`/`SemioKitMutation` — all reachable from outside, unlike `store`/`protocol`
//! themselves. `subject` below calls those four helpers on the SAME committed fixture text `oracle`
//! embeds via `include_str!` — real `serde_json` deserialization of the REAL committed bytes, not
//! hand-transcription, so there is no separate Rust literal that could ever drift out of sync with
//! the fixture it claims to mirror (the exact drift risk a peer session flagged for `✳️graph`'s 33
//! hand-transcribed fixtures). This also happens to be a strictly stronger fix than switching to
//! `Context::fixture_json` alone would have been: that helper only yields the framework's
//! dependency-free `Json` (untyped), which still could not satisfy `apply_semio_kit_mutation`'s
//! `SemioKitSnapshot`/`SemioKitMutation` parameters without hand-written field-by-field
//! reconstruction — the same drift surface this design eliminates entirely. Both roles project to
//! `protocol::Json` (`semio_repo_test_host`'s dependency-free type, NOT the subject crate's private
//! `protocol` alias) and `ordered-json-v1` compares them structurally. The Rust SUBJECT phase is
//! blocked this wave by a concurrent os-kernel refactor (see the fleet brief), so it is written and
//! gated but not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioKitMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &[
    "create-object",
    "delete-object",
    "create-model",
    "delete-model",
    "create-properties",
    "delete-properties",
    "bind-representation",
    "unbind-representation",
    "change-representation-pin",
    "add-type",
    "remove-type",
    "rename-type",
    "add-design",
    "remove-design",
    "edit-design",
];

/// 🗣️ The real committed furniture kit — one type bound to one representation link, one design with
/// two pieces and a connection, and all three owned child slots populated.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🗣️example.dsl.semio";
/// 🎒️ The same kit in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🧫️ The committed `(before, mutation, after)` fixture TEXT for one kind, read literally via
/// `include_str!` — this IS the independently handcrafted specification vector the no-oracle
/// decision rests on, never recomputed. `mutation` is unused by `oracle` (which only ever answers
/// with `before`/`after`) but shared with `subject` below so there is exactly one `include_str!`
/// call per fixture file in this whole adapter.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "create-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏗️create-object/🧪️tests/attaches-a-second-object-child/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏗️create-object/🧪️tests/attaches-a-second-object-child/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏗️create-object/🧪️tests/attaches-a-second-object-child/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🪓delete-object/🧪️tests/detaches-the-only-object-child-and-keeps-the-model-child/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🪓delete-object/🧪️tests/detaches-the-only-object-child-and-keeps-the-model-child/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🪓delete-object/🧪️tests/detaches-the-only-object-child-and-keeps-the-model-child/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-model" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏛️create-model/🧪️tests/attaches-a-second-model-child/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏛️create-model/🧪️tests/attaches-a-second-model-child/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏛️create-model/🧪️tests/attaches-a-second-model-child/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-model" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💣delete-model/🧪️tests/detaches-the-only-model-child-and-keeps-the-object-child/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💣delete-model/🧪️tests/detaches-the-only-model-child-and-keeps-the-object-child/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/💣delete-model/🧪️tests/detaches-the-only-model-child-and-keeps-the-object-child/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-properties" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-a-kit-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-a-kit-that-has-none/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-a-kit-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-properties" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-every-other-collection-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-every-other-collection-alone/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-every-other-collection-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "bind-representation" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🔗bind-representation/🧪️tests/binds-a-second-representation-to-an-existing-type/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🔗bind-representation/🧪️tests/binds-a-second-representation-to-an-existing-type/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🔗bind-representation/🧪️tests/binds-a-second-representation-to-an-existing-type/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "unbind-representation" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🧪️tests/unbinds-the-leading-representation-and-keeps-the-trailing-one/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🧪️tests/unbinds-the-leading-representation-and-keeps-the-trailing-one/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🧪️tests/unbinds-the-leading-representation-and-keeps-the-trailing-one/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-representation-pin" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🧪️tests/repins-the-representation-from-head-to-a-checkpoint/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🧪️tests/repins-the-representation-from-head-to-a-checkpoint/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🧪️tests/repins-the-representation-from-head-to-a-checkpoint/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-type" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➕add-type/🧪️tests/appends-a-slab-type-to-the-catalogue/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➕add-type/🧪️tests/appends-a-slab-type-to-the-catalogue/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➕add-type/🧪️tests/appends-a-slab-type-to-the-catalogue/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-type" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➖remove-type/🧪️tests/removes-the-column-type-and-keeps-the-beam-type/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➖remove-type/🧪️tests/removes-the-column-type-and-keeps-the-beam-type/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/➖remove-type/🧪️tests/removes-the-column-type-and-keeps-the-beam-type/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "rename-type" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✏️rename-type/🧪️tests/renames-the-beam-type-without-recategorising-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✏️rename-type/🧪️tests/renames-the-beam-type-without-recategorising-it/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/✏️rename-type/🧪️tests/renames-the-beam-type-without-recategorising-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-design" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🆕add-design/🧪️tests/adds-an-empty-roof-design/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🆕add-design/🧪️tests/adds-an-empty-roof-design/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🆕add-design/🧪️tests/adds-an-empty-roof-design/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-design" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🗑️remove-design/🧪️tests/removes-the-only-design-together-with-its-pieces/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🗑️remove-design/🧪️tests/removes-the-only-design-together-with-its-pieces/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🗑️remove-design/🧪️tests/removes-the-only-design-together-with-its-pieces/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "edit-design" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🖊️edit-design/🧪️tests/replaces-the-designs-pieces-and-connections-in-one-step/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🖊️edit-design/🧪️tests/replaces-the-designs-pieces-and-connections-in-one-step/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/🖊️edit-design/🧪️tests/replaces-the-designs-pieces-and-connections-in-one-step/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-kit: no fixture registered for kind {other:?}"),
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
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{apply_semio_kit_mutation, decode_kit_mutation_json, inverse_semio_kit_mutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{decode_kit_snapshot_json, decode_semio_kit_pack, encode_kit_snapshot_json, encode_semio_kit_pack, parse_semio_kit_dsl, print_semio_kit_dsl};

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds via
    /// `include_str!` into real `SemioKitSnapshot`/`SemioKitMutation` values, through this
    /// subset's own production `decode_kit_snapshot_json`/`decode_kit_mutation_json` — real
    /// `serde_json` deserialization of the committed bytes, never a hand-transcribed Rust literal,
    /// so there is nothing here that could drift out of sync with the fixture it mirrors.
    fn decode_base(kind: &str) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot {
        let (before, _mutation, _after) = super::fixture_text(kind);
        decode_kit_snapshot_json(before).unwrap_or_else(|error| panic!("mutate-semio-kit: committed before-snapshot fixture for {kind:?} must decode: {error}"))
    }

    fn decode_mutation(kind: &str) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation {
        let (_before, mutation, _after) = super::fixture_text(kind);
        decode_kit_mutation_json(mutation).unwrap_or_else(|error| panic!("mutate-semio-kit: committed mutation fixture for {kind:?} must decode: {error}"))
    }

    fn decode_after(kind: &str) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot {
        let (_before, _mutation, after) = super::fixture_text(kind);
        decode_kit_snapshot_json(after).unwrap_or_else(|error| panic!("mutate-semio-kit: committed after-snapshot fixture for {kind:?} must decode: {error}"))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both fixtures are written
    /// in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot, expected: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_kit_snapshot_json(got), encode_kit_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Projection
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: this subset's own
    /// `serde_json`-derived JSON text, re-parsed into the framework's dependency-free `Json` so it
    /// compares structurally against `oracle`'s literal fixture text the same way.
    fn project(snapshot: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot) -> (Vec<u8>, Json) {
        let text = encode_kit_snapshot_json(snapshot);
        let json = parse_json(&text).unwrap_or_else(|error| panic!("mutate-semio-kit: subject's own encoded snapshot must parse as JSON: {error}"));
        (text.into_bytes(), json)
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot. The assertion lives here rather than in the comparison because a recorded
    /// no-oracle case runs no oracle role: a handler that merely returned `Ok` would report a pass
    /// having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let mut base = decode_base(kind);
            let mutation = decode_mutation(kind);
            let expected = decode_after(kind);
            let outcome = apply_semio_kit_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            if base != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &base, &expected));
            }
            let (bytes, json) = project(&base);
            Ok(Outcome::with_raw(bytes, json))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let base = decode_base(kind);
            let mutation = decode_mutation(kind);
            let mut current = base.clone();
            let outcome = apply_semio_kit_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            let undo = inverse_semio_kit_mutation(&mutation, &base);
            for step in &undo {
                let step_outcome = apply_semio_kit_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let (bytes, json) = project(&current);
            Ok(Outcome::with_raw(bytes, json))
        }
    }

    /// 🔁️ The real committed furniture kit through both of its committed encodings — the one
    /// document that carries a type with a representation LINK, a design with pieces and a
    /// connection, and all three owned child slots at once, so a codec that dropped either the link
    /// pool or a child handle shows up here rather than in a per-kind fixture that only exercises
    /// one of them. The DSL text and the pack envelope are separate committed files produced by
    /// separate codecs, so agreeing on one snapshot cannot be achieved by smuggling bytes from
    /// either. Byte-identical re-emission IS expected — the committed text is this codec's own
    /// output, not a foreign writer's — so the wave's usual "output must not equal input" tripwire
    /// does not apply and the text/binary cross-check carries that evidence instead.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed furniture artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_kit_dsl(&text)?;
        if parsed.types.len() != 1 || parsed.designs.len() != 1 || parsed.representations.len() != 1 {
            return Err(format!(
                "identity-round-trip: the committed furniture kit is the one-type, one-design, one-representation fixture this case describes, but parsed {}/{}/{}",
                parsed.types.len(),
                parsed.designs.len(),
                parsed.representations.len()
            ));
        }
        let reparsed = parse_semio_kit_dsl(&print_semio_kit_dsl(&parsed))?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let unpacked = decode_semio_kit_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different kit than the committed text artifact", &unpacked, &parsed));
        }
        let repacked = decode_semio_kit_pack(&encode_semio_kit_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let (bytes, json) = project(&parsed);
        Ok(Outcome::with_raw(bytes, json))
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
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
