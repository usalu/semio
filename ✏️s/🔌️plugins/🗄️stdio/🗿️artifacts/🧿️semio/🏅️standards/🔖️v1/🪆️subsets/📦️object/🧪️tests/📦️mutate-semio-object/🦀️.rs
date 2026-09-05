//! 🦀️ Semio OBJECT exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file is the subject half only. The oracle is `🐍️component.py` beside it: an independent
//! Python implementation of the same carrier and the same nine-verb vocabulary, written from the
//! committed grammar, protocol and JSON schema, registered as `semio-object-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🔣️oracle.json`. Registering oracle
//! handlers here as well would put this repository's answer on both sides of the comparison, which
//! is the exact failure the platform exists to prevent — so it registers none.
//!
//! Every input this file reads comes from the PLAN: the real committed crate object through
//! `asset://`, the `prepare` list and the mutation payload from the scenario's doc string, and the
//! specification-vector paths from the scenario's step text. The feature is the single place any of
//! them is written down, so neither implementation can hold a transcription that drifts from what
//! the other one read.
//!
//! `s.stdio.semio.object` was the first COMPOSITE subset: alongside the composite `transform` value
//! field (`move`/`rotate`/`scale`) it carries three optional owned CHILD slots — `brep`, `mesh` and
//! `properties` — each an `Option<store::ArtifactChild<S>>` holding a two-string handle, never
//! embedded content. `store` is this crate's PRIVATE `extern crate semio_framework_os_kernel as
//! store;`, unnameable from a test host, which is why every handle here is DESERIALIZED through this
//! subset's own `decode_semio_object_snapshot_json`/`decode_semio_object_mutation_json` rather than
//! constructed: those signatures name only `&str`/`SemioObjectSnapshot`/`SemioObjectMutation`.

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{Context, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioObjectMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because registration happens before the
/// subject crate is necessarily linked. `kinds_match_the_enum_and_the_catalog` in that production
/// file keeps the list honest against the enum, and the contract's mutation-coverage gate keeps it
/// honest against the catalog and this feature.
const KINDS: &[&str] = &["move-object", "rotate-object", "scale-object", "create-brep", "delete-brep", "create-mesh", "delete-mesh", "create-properties", "delete-properties"];

/// 🗣️ The real committed crate object — a non-identity translation with ALL THREE child slots
/// populated, the only committed document that exercises the `ArtifactChild` codec three slots at a
/// time rather than one at a time.
#[cfg(feature = "sut")]
const CRATE_DSL: &str = "asset://📚️examples/📦️crate/🖼️assets/🗣️.dsl.semio";
/// 🎒️ The same object in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const CRATE_PACK: &str = "asset://📚️examples/📦️crate/🖼️assets/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Plan
/// 🧫️ Every `asset://` URI the scenario's steps name, in step order — the specification-vector
/// paths live in the feature, never in this file.
#[cfg(feature = "sut")]
fn step_assets(ctx: &Context) -> Vec<String> {
    let mut found = Vec::new();
    for (_, text) in &ctx.scenario.steps {
        for candidate in text.split_whitespace() {
            if candidate.starts_with("asset://") {
                found.push(candidate.to_string());
            }
        }
    }
    found
}

/// 📜️ The scenario's `{"prepare": [...], "mutation": {…}}` doc string, split into its two halves.
#[cfg(feature = "sut")]
fn plan_mutations(ctx: &Context) -> Result<(Vec<Json>, Json), String> {
    let document = ctx.doc_json()?;
    let mutation = document.get("mutation").cloned().ok_or_else(|| format!("scenario {} carries no `mutation` in its doc string", ctx.scenario.id))?;
    Ok((document.array("prepare"), mutation))
}
//#endregion 🔖️Plan

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::{apply_semio_object_mutation, decode_semio_object_mutation_json, inverse_semio_object_mutation, SemioObjectMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{decode_semio_object_pack, decode_semio_object_snapshot_json, encode_semio_object_pack, encode_semio_object_snapshot_json, parse_semio_object_dsl, print_semio_object_dsl, SemioObjectSnapshot};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

    //#region 🔖️Bridges
    /// 🦠️ One planned mutation payload, decoded through this subset's own JSON bridge.
    fn mutation_of(value: &Json, scenario: &str) -> Result<SemioObjectMutation, String> {
        decode_semio_object_mutation_json(&value.to_string()).map_err(|error| format!("{scenario}: the planned mutation payload must decode: {error}"))
    }

    /// 🧬️ Applies one mutation, turning a refusal into a failure rather than a silent no-op.
    fn apply(snapshot: &mut SemioObjectSnapshot, mutation: &SemioObjectMutation, scenario: &str) -> Result<(), String> {
        let outcome = apply_semio_object_mutation(snapshot, mutation);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{scenario}: the mutation was rejected: {refusals:?}"))
    }

    /// 📦️ The real committed crate object, put into the state the scenario's verb is defined for by
    /// the doc string's own `prepare` list.
    fn prepared(ctx: &Context) -> Result<(SemioObjectSnapshot, SemioObjectMutation), String> {
        let (prepare, mutation) = super::plan_mutations(ctx)?;
        let text = String::from_utf8(ctx.fixture_bytes(super::CRATE_DSL)?).map_err(|error| format!("the committed crate artifact is not UTF-8: {error}"))?;
        let mut snapshot = parse_semio_object_dsl(&text)?;
        for step in &prepare {
            let step = mutation_of(step, &ctx.scenario.id)?;
            apply(&mut snapshot, &step, &ctx.scenario.id)?;
        }
        Ok((snapshot, mutation_of(&mutation, &ctx.scenario.id)?))
    }

    fn projection(snapshot: &SemioObjectSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_object_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioObjectSnapshot, expected: &SemioObjectSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_object_snapshot_json(got), encode_semio_object_snapshot_json(expected))
    }
    //#endregion 🔖️Bridges

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed crate object. The projection is the whole resulting
    /// snapshot, so a `delete-<slot>` that also cleared a sibling handle diverges from the oracle
    /// here rather than passing quietly.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (mut snapshot, mutation) = prepared(ctx)?;
        apply(&mut snapshot, &mutation, &ctx.scenario.id)?;
        let projection = projection(&snapshot)?;
        Ok(Outcome::with_raw(print_semio_object_dsl(&snapshot).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law, asserted in role: applying the verb and then its OWN computed
    /// inverse must restore the prepared object exactly — a re-attached child's `child_id` AND the
    /// artifact id and dialect its `ArtifactRef` named, not merely the slot becoming occupied again.
    /// The projection carries the mutated snapshot too, so the seven rows do not all project the
    /// same value and the comparison cannot go vacuous.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (base, mutation) = prepared(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &mutation, &ctx.scenario.id)?;
        let mutated = projection(&current)?;
        for step in inverse_semio_object_mutation(&mutation, &base) {
            apply(&mut current, &step, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the prepared object", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)])))
    }

    /// 🧫️ The committed handcrafted `(before, mutation, after)` vector for one kind, applied and
    /// checked against the committed after-snapshot in role — a THIRD statement of what the verb
    /// means, independent of both implementations, kept from the case this one replaces.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let assets = super::step_assets(ctx);
        if assets.len() < 3 {
            return Err(format!("{}: expected three committed vector assets, found {}", ctx.scenario.id, assets.len()));
        }
        let before = String::from_utf8(ctx.fixture_bytes(&assets[0])?).map_err(|error| error.to_string())?;
        let payload = String::from_utf8(ctx.fixture_bytes(&assets[1])?).map_err(|error| error.to_string())?;
        let after = String::from_utf8(ctx.fixture_bytes(&assets[2])?).map_err(|error| error.to_string())?;
        let mut current = decode_semio_object_snapshot_json(&before).map_err(|error| format!("{}: the committed before-snapshot must decode: {error}", ctx.scenario.id))?;
        let expected = decode_semio_object_snapshot_json(&after).map_err(|error| format!("{}: the committed after-snapshot must decode: {error}", ctx.scenario.id))?;
        let mutation = decode_semio_object_mutation_json(&payload).map_err(|error| format!("{}: the committed mutation payload must decode: {error}", ctx.scenario.id))?;
        apply(&mut current, &mutation, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        projection(&current).map(Outcome::projection)
    }

    /// 🔁️ Both committed encodings of the crate object, each re-emitted from the parsed document.
    /// Byte-identical re-emission IS expected here — the committed files are this codec's own output,
    /// not a foreign writer's — so the wave's "output must not equal input" tripwire would be exactly
    /// backwards and its MIRROR law is asserted in its place through `law::carrier_is_exact`, which
    /// fails with the offset of the first differing byte. What keeps that from being a codec agreeing
    /// with itself is the oracle: the Python implementation reproduces the same two files from the
    /// grammar and the protocol alone, and the digests of both sides' emitted bytes are compared.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let dsl_bytes = ctx.fixture_bytes(super::CRATE_DSL)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: the committed crate artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_object_dsl(&text)?;
        if parsed.brep.is_none() || parsed.mesh.is_none() || parsed.properties.is_none() {
            return Err("identity-round-trip: the committed crate object is the all-three-children artifact this case describes, but at least one child slot decoded as absent".to_string());
        }
        let printed = print_semio_object_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_semio_object_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(super::CRATE_PACK)?;
        let unpacked = decode_semio_object_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different object than the committed text artifact", &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_object_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_object_pack(&repacked_bytes)?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
        }
        Ok(Outcome::projection(Json::Object(vec![
            ("document".to_string(), projection(&parsed)?),
            ("dslDigest".to_string(), Json::String(semio_repo_test_host::protocol::digest(printed.as_bytes()))),
            ("packDigest".to_string(), Json::String(semio_repo_test_host::protocol::digest(&repacked_bytes))),
            ("dslLength".to_string(), Json::Number(printed.as_bytes().len() as f64)),
            ("packLength".to_string(), Json::Number(repacked_bytes.len() as f64)),
        ])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id — the loop
/// mirrors the feature's `Examples` tables exactly. Subject only: the reference answer comes from
/// the Python adapter, in the oracle role, and nothing here may answer for it.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = KINDS;
    }
    built
}
//#endregion 🔖️Registration
