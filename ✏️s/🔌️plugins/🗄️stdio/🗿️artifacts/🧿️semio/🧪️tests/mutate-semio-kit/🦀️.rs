//! 🦀️ Semio KIT exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file is the subject half only. The oracle is `🐍️component.py` beside it: an independent
//! Python implementation of the same carrier and the same fifteen-verb vocabulary, written from the
//! committed grammar, protocol and JSON schema, registered as `semio-kit-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🔣️oracle.json`. Registering oracle handlers
//! here as well would put this repository's answer on both sides of the comparison, which is the
//! exact failure the platform exists to prevent — so it registers none.
//!
//! Every input this file reads comes from the PLAN: the real committed furniture kit through
//! `asset://`, the `prepare` list and the mutation payload from the scenario's doc string, and the
//! specification-vector paths from the scenario's step text. The feature is the single place any of
//! them is written down, so neither implementation can hold a transcription that drifts from what
//! the other one read.
//!
//! `s.stdio.semio.kit` was the second COMPOSITE subset and is the one that carries every composition
//! shape the platform has: `objects`/`models` are repeated owned CHILD pools, `properties` is a
//! single owned child SLOT, `representations` is a LINK pool whose entries carry a `LinkPin`, and
//! `designs` nests pieces and connections inside a record. All of those live behind this crate's
//! PRIVATE `extern crate semio_framework_os_kernel as store;`, unnameable from a test host, which is
//! why every handle here is DESERIALIZED through this subset's own
//! `decode_kit_snapshot_json`/`decode_kit_mutation_json` rather than constructed.

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{Context, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioKitMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because registration happens before the
/// subject crate is necessarily linked. `kinds_match_the_enum_and_the_catalog` in that production
/// file keeps the list honest against the enum, and the contract's mutation-coverage gate keeps it
/// honest against the catalog and this feature.
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

/// 🪑️ The real committed furniture kit — a type catalogue, a design of two pieces and a connection,
/// both child pools, the single properties slot and a representation link, all occupied at once.
#[cfg(feature = "sut")]
/// 🏗️ The kit every mutation row runs on: the real Nakagin Capsule Tower as a kit of parts — 12 real
/// element types, one design carrying 180 real capsule pieces with their real placement transforms
/// and 179 real port-to-port connections — derived ONCE from the real committed IFC 4 file with
/// IfcOpenShell 0.8.4 by `🐍️derive-kit-fixture.py` in the ticket folder.
#[cfg(feature = "sut")]
const TOWER_DSL: &str = "local://🗣️nakagin-capsule-tower.dsl.semio";
/// 🎒️ The same kit in its binary envelope, written by the PYTHON implementation — so this codec
/// reproducing it is a cross-language byte agreement, not a codec agreeing with itself.
#[cfg(feature = "sut")]
const TOWER_PACK: &str = "local://🎒️nakagin-capsule-tower.pack.semio";
const FURNITURE_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🗣️.dsl.semio";
/// 🎒️ The same kit in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const FURNITURE_PACK: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🎒️example.pack.semio";
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
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{apply_semio_kit_mutation, decode_kit_mutation_json, inverse_semio_kit_mutation, SemioKitMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{decode_kit_snapshot_json, decode_semio_kit_pack, encode_kit_snapshot_json, encode_semio_kit_pack, parse_semio_kit_dsl, print_semio_kit_dsl, SemioKitSnapshot};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

    //#region 🔖️Bridges
    /// 🦠️ One planned mutation payload, decoded through this subset's own JSON bridge.
    fn mutation_of(value: &Json, scenario: &str) -> Result<SemioKitMutation, String> {
        decode_kit_mutation_json(&value.to_string()).map_err(|error| format!("{scenario}: the planned mutation payload must decode: {error}"))
    }

    /// 🧬️ Applies one mutation, turning a refusal into a failure rather than a silent no-op.
    fn apply(snapshot: &mut SemioKitSnapshot, mutation: &SemioKitMutation, scenario: &str) -> Result<(), String> {
        let outcome = apply_semio_kit_mutation(snapshot, mutation);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{scenario}: the mutation was rejected: {refusals:?}"))
    }

    /// 🪑️ The real committed furniture kit, put into the state the scenario's verb is defined for by
    /// the doc string's own `prepare` list.
    fn prepared(ctx: &Context) -> Result<(SemioKitSnapshot, SemioKitMutation), String> {
        let (prepare, mutation) = super::plan_mutations(ctx)?;
        let text = String::from_utf8(ctx.fixture_bytes(super::TOWER_DSL)?).map_err(|error| format!("the capsule tower kit artifact is not UTF-8: {error}"))?;
        let mut snapshot = parse_semio_kit_dsl(&text)?;
        for step in &prepare {
            let step = mutation_of(step, &ctx.scenario.id)?;
            apply(&mut snapshot, &step, &ctx.scenario.id)?;
        }
        Ok((snapshot, mutation_of(&mutation, &ctx.scenario.id)?))
    }

    fn projection(snapshot: &SemioKitSnapshot) -> Result<Json, String> {
        parse_json(&encode_kit_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioKitSnapshot, expected: &SemioKitSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_kit_snapshot_json(got), encode_kit_snapshot_json(expected))
    }
    //#endregion 🔖️Bridges

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed furniture kit. The projection is the whole resulting
    /// snapshot, so a `delete-object` that also emptied the model pool — or a `bind-representation`
    /// that dropped a checkpoint pin back to head — diverges from the oracle here rather than passing
    /// quietly.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (mut snapshot, mutation) = prepared(ctx)?;
        apply(&mut snapshot, &mutation, &ctx.scenario.id)?;
        let projection = projection(&snapshot)?;
        Ok(Outcome::with_raw(print_semio_kit_dsl(&snapshot).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law, asserted in role: applying the verb and then its OWN computed
    /// inverse must restore the prepared kit exactly — for `remove-design` that means the design's
    /// pieces and its connection, which `add-design` alone cannot put back. The projection carries the
    /// mutated snapshot too, so the fifteen rows do not all project the same value and the comparison
    /// cannot go vacuous.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (base, mutation) = prepared(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &mutation, &ctx.scenario.id)?;
        let mutated = projection(&current)?;
        for step in inverse_semio_kit_mutation(&mutation, &base) {
            apply(&mut current, &step, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the prepared kit", ctx.scenario.id), &current, &base));
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
        let mut current = decode_kit_snapshot_json(&before).map_err(|error| format!("{}: the committed before-snapshot must decode: {error}", ctx.scenario.id))?;
        let expected = decode_kit_snapshot_json(&after).map_err(|error| format!("{}: the committed after-snapshot must decode: {error}", ctx.scenario.id))?;
        let mutation = decode_kit_mutation_json(&payload).map_err(|error| format!("{}: the committed mutation payload must decode: {error}", ctx.scenario.id))?;
        apply(&mut current, &mutation, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        projection(&current).map(Outcome::projection)
    }

    /// 🔁️ Both committed encodings of the furniture kit, each re-emitted from the parsed document.
    /// Byte-identical re-emission IS expected here — the committed files are this codec's own output,
    /// not a foreign writer's — so the wave's "output must not equal input" tripwire would be exactly
    /// backwards and its MIRROR law is asserted in its place through `law::carrier_is_exact`, which
    /// fails with the offset of the first differing byte. What keeps that from being a codec agreeing
    /// with itself is the oracle: the Python implementation reproduces the same two files from the
    /// grammar and the protocol alone, and the digests of both sides' emitted bytes are compared.
    fn carrier_pair(ctx: &Context, dsl_uri: &str, pack_uri: &str, what: &str) -> Result<(SemioKitSnapshot, Json), String> {
        let dsl_bytes = ctx.fixture_bytes(dsl_uri)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: {what} is not UTF-8: {error}"))?;
        let parsed = parse_semio_kit_dsl(&text)?;
        let printed = print_semio_kit_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_semio_kit_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement(&format!("identity-round-trip: printing {what} back to DSL and reparsing it lost content"), &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(pack_uri)?;
        let unpacked = decode_semio_kit_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement(&format!("identity-round-trip: the binary twin of {what} decodes to a different kit than its text"), &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_kit_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_kit_pack(&repacked_bytes)?;
        if repacked != parsed {
            return Err(disagreement(&format!("identity-round-trip: encoding {what} to a pack and decoding it back lost content"), &repacked, &parsed));
        }
        let report = Json::Object(vec![
            ("document".to_string(), projection(&parsed)?),
            ("dslDigest".to_string(), Json::String(semio_repo_test_host::protocol::digest(printed.as_bytes()))),
            ("packDigest".to_string(), Json::String(semio_repo_test_host::protocol::digest(&repacked_bytes))),
            ("dslLength".to_string(), Json::Number(printed.as_bytes().len() as f64)),
            ("packLength".to_string(), Json::Number(repacked_bytes.len() as f64)),
        ]);
        Ok((parsed, report))
    }

    /// 🔁️ Both kits, in both encodings — four files, all four reproduced byte for byte. The
    /// committed furniture kit's two files are this codec's own output and the Python implementation
    /// reproduces them from the grammar and the protocol alone; the capsule tower's two files are the
    /// PYTHON implementation's output and this codec has to reproduce THOSE, 1 800 real `f64`
    /// transform components among them.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let (furniture, furniture_report) = carrier_pair(ctx, super::FURNITURE_DSL, super::FURNITURE_PACK, "the committed furniture kit")?;
        if furniture.properties.is_none() || furniture.representations.is_empty() || furniture.designs.is_empty() {
            return Err("identity-round-trip: the committed furniture kit is the all-shapes artifact this case describes, but its properties slot, its representation pool or its design set decoded as empty".to_string());
        }
        let (tower, tower_report) = carrier_pair(ctx, super::TOWER_DSL, super::TOWER_PACK, "the capsule tower kit")?;
        if tower.types.len() != 12 || tower.designs.len() != 1 || tower.designs[0].pieces.len() != 180 || tower.designs[0].connections.len() != 179 || tower.representations.len() != 12 {
            return Err(format!(
                "identity-round-trip: the capsule tower kit is the 12/1/180/179/12 document this case describes, but decoded as {}/{}/{}/{}/{}",
                tower.types.len(),
                tower.designs.len(),
                tower.designs.first().map(|design| design.pieces.len()).unwrap_or(0),
                tower.designs.first().map(|design| design.connections.len()).unwrap_or(0),
                tower.representations.len()
            ));
        }
        Ok(Outcome::projection(Json::Object(vec![("furniture".to_string(), furniture_report), ("tower".to_string(), tower_report)])))
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
