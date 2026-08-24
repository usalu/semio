//! 🦀️ Semio OBJECT exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-object-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️object/🧪️oracle/🔣️component.json`): `s.stdio.semio.object` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed, independently
//! handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/✳️object/
//! 🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_object_mutation` over the full 9-kind `SemioObjectMutation` vocabulary.
//!
//! What distinguishes this subset is that it was the first COMPOSITE one: alongside the composite
//! `transform` value field (`move`/`rotate`/`scale`) it carries three optional owned CHILD slots —
//! `brep`, `mesh` and `properties` — each an `Option<store::ArtifactChild<S>>` holding a two-string
//! handle (`child_id` plus an `ArtifactRef` naming an artifact id and its dialect), never embedded
//! content. Attaching one is `create-<slot>`, detaching it is `delete-<slot>`, and a `delete` must
//! leave the OTHER two slots untouched, which is exactly what the committed fixtures are arranged
//! to catch: `delete-brep` runs against a snapshot carrying both a brep and a mesh child, and
//! `delete-properties` against one carrying both a properties and a mesh child.
//!
//! **The `ArtifactRef` wall, and how it is gone.** An earlier draft of this adapter registered real
//! handlers for only the 3 transform kinds and, for the other 6, a handler that returned a
//! self-documenting `Err`: a `create-*` payload carries a `store::os_io::ArtifactRef` `target` and a
//! `delete-*` before-snapshot carries a populated `store::ArtifactChild<S>`, and `store` is this
//! crate's PRIVATE `extern crate semio_framework_os_kernel as store;` (`📦️glue.rs`), unnameable
//! from a test host that links only `semio-repo-test-host` and this plugin. Constructing one by hand
//! was impossible; DESERIALIZING one never needed a nameable path at all. This subset's own
//! production code now exports `decode_semio_object_snapshot_json`/`encode_semio_object_snapshot_json`
//! (`../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/🦀️component.rs`) and
//! `decode_semio_object_mutation_json`/`inverse_semio_object_mutation` (`…/🧬️mutations/
//! 🦀️component.rs`) — thin, permanent `serde_json`/`protocol::Mutation`-backed wrappers whose
//! SIGNATURES name only `&str`/`String`/`Vec`/`SemioObjectSnapshot`/`SemioObjectMutation`, all
//! reachable. The same fix `✳️kit` established for the same wall. All 9 kinds are therefore real
//! here, and the transform kinds now run against the committed before-snapshot itself rather than an
//! all-identity snapshot hand-built beside it.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never gets two sides to compare. Every law this case claims is therefore
//! asserted INSIDE the subject handler. The subject half is gated behind the generated host's `sut`
//! feature so the oracle-only run never compiles the local implementation; the Rust SUBJECT phase is
//! blocked this wave by concurrent framework refactors (see 📓️w7-fleet-brief.md), so it is written
//! and gated but not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioObjectMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["move-object", "rotate-object", "scale-object", "create-brep", "delete-brep", "create-mesh", "delete-mesh", "create-properties", "delete-properties"];

/// 🗣️ The real committed crate object — a non-identity translation with ALL THREE child slots
/// populated, which is the only committed document that exercises the `ArtifactChild` codec end to
/// end rather than one slot at a time.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio";
/// 🎒️ The same object in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` fixture TEXT for one kind, read literally via
/// `include_str!` — this IS the independently handcrafted specification vector the no-oracle
/// decision rests on, never recomputed. One `include_str!` per file for the whole adapter: `oracle`
/// answers with `before`/`after`, `subject` decodes all three.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "move-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🧪️tests/moves-the-object-to-a-new-translation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🧪️tests/moves-the-object-to-a-new-translation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚚move-object/🧪️tests/moves-the-object-to-a-new-translation/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "rotate-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🧪️tests/rotates-the-object-a-half-turn-about-z/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🧪️tests/rotates-the-object-a-half-turn-about-z/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🔄rotate-object/🧪️tests/rotates-the-object-a-half-turn-about-z/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "scale-object" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/scales-the-object-non-uniformly/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/scales-the-object-non-uniformly/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/scales-the-object-non-uniformly/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-brep" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🧪️tests/attaches-a-brep-child-to-an-object-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🧪️tests/attaches-a-brep-child-to-an-object-that-has-none/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧱create-brep/🧪️tests/attaches-a-brep-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-brep" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🧪️tests/detaches-the-brep-child-and-leaves-the-mesh-child-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🧪️tests/detaches-the-brep-child-and-leaves-the-mesh-child-alone/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/💥delete-brep/🧪️tests/detaches-the-brep-child-and-leaves-the-mesh-child-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-mesh" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/attaches-a-mesh-child-to-an-object-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/attaches-a-mesh-child-to-an-object-that-has-none/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/attaches-a-mesh-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-mesh" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/detaches-the-mesh-child-and-leaves-the-brep-child-alone/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-properties" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-an-object-that-has-none/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-an-object-that-has-none/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🏷️create-properties/🧪️tests/attaches-a-properties-child-to-an-object-that-has-none/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-properties" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-the-mesh-child-alone/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-the-mesh-child-alone/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🚫delete-properties/🧪️tests/detaches-the-properties-child-and-leaves-the-mesh-child-alone/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-object: no fixture registered for kind {other:?}"),
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
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::{apply_semio_object_mutation, decode_semio_object_mutation_json, inverse_semio_object_mutation, SemioObjectMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{decode_semio_object_pack, decode_semio_object_snapshot_json, encode_semio_object_pack, encode_semio_object_snapshot_json, parse_semio_object_dsl, print_semio_object_dsl, SemioObjectSnapshot};

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridges — real deserialization of the committed bytes,
    /// including the `store::ArtifactChild`/`ArtifactRef` handles no caller outside the plugin can
    /// construct by hand.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<SemioObjectSnapshot, String> {
        decode_semio_object_snapshot_json(text).map_err(|error| format!("mutate-semio-object: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<SemioObjectMutation, String> {
        decode_semio_object_mutation_json(text).map_err(|error| format!("mutate-semio-object: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &SemioObjectSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_object_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both fixtures are written
    /// in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioObjectSnapshot, expected: &SemioObjectSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_object_snapshot_json(got), encode_semio_object_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — the two untouched child slots included, so a `delete-brep` that also cleared
    /// the mesh handle fails here. The assertion lives in the handler because a recorded no-oracle
    /// case runs no oracle role: one that merely returned `Ok` would report a pass having checked
    /// nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after) = super::fixture_text(kind);
            let mut current = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let outcome = apply_semio_object_mutation(&mut current, &mutation);
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
    /// restore the committed before-snapshot exactly — a detached child's `child_id` AND the
    /// artifact id and dialect its `ArtifactRef` named, not merely the slot becoming occupied again.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let outcome = apply_semio_object_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {:?}", outcome.messages()));
            }
            for step in inverse_semio_object_mutation(&mutation, &base) {
                let step_outcome = apply_semio_object_mutation(&mut current, &step);
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

    /// 🔁️ The real committed crate object through both of its committed encodings — the one
    /// document that carries all three child handles at once, so a child codec that dropped a slot
    /// shows up here rather than in a per-kind fixture that only ever populates one. The DSL text and
    /// the pack envelope are separate committed files produced by separate codecs, so agreeing on one
    /// snapshot cannot be achieved by smuggling bytes from either. Byte-identical re-emission IS
    /// expected — the committed text is this codec's own output, not a foreign writer's — so the
    /// wave's usual "output must not equal input" tripwire does not apply and the text/binary
    /// cross-check carries that evidence instead.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed crate artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_object_dsl(&text)?;
        if parsed.brep.is_none() || parsed.mesh.is_none() || parsed.properties.is_none() {
            return Err("identity-round-trip: the committed crate object is the all-three-children fixture this case describes, but at least one child slot decoded as absent".to_string());
        }
        let reparsed = parse_semio_object_dsl(&print_semio_object_dsl(&parsed))?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let unpacked = decode_semio_object_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different object than the committed text artifact", &unpacked, &parsed));
        }
        let repacked = decode_semio_object_pack(&encode_semio_object_pack(&parsed))?;
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
/// snapshot the oracle role can read literally, but the crate object is committed as DSL and pack
/// bytes ONLY, and turning those into a snapshot needs this subset's own codec — which the
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
