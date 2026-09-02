//! 🦀️ Semio BREP exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file is the subject half only. The oracle is `🐍️component.py` beside it: an independent
//! Python implementation of the same carrier and the same thirteen-verb vocabulary, written from the
//! committed grammar, protocol and JSON schema, registered as `semio-brep-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️brep/🔣️oracle.json`. Registering oracle handlers
//! here as well would put this repository's answer on both sides of the comparison, which is the
//! exact failure the platform exists to prevent — so it registers none.
//!
//! Every input this file reads comes from the PLAN: the real concrete-forest structure through
//! `local://` and the tiny committed solid through `asset://`, the `prepare` list and the mutation
//! payload from the scenario's doc string, and the specification-vector paths from the step text. The feature is the single place any of
//! them is written down, so neither implementation can hold a transcription that drifts from what
//! the other one read.
//!
//! `s.stdio.semio.brep` is the id-keyed topology graph — vertices, edges, loops, faces, shells and
//! solids — and the one subset in this artifact whose records embed data-carrying TAGGED UNIONS:
//! `BrepCurve` has four arms and `BrepSurface` six, each with its own field list. `create-loop` and
//! `delete-loop` are deliberately absent from the vocabulary; `delete-vertex` is the only verb that
//! cascades, into every edge incident on the vertex it removes.

use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
use semio_repo_test_host::{Context, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioBrepMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because registration happens before the
/// subject crate is necessarily linked. `kinds_match_the_enum_and_the_catalog` in that production
/// file keeps the list honest against the enum, and the contract's mutation-coverage gate keeps it
/// honest against the catalog and this feature.
const KINDS: &[&str] = &["create-vertex", "delete-vertex", "create-edge", "delete-edge", "create-face", "delete-face", "create-shell", "delete-shell", "create-solid", "delete-solid", "replace-curve", "replace-surface", "move-vertex"];

/// 🌲️ The document every mutation row runs on: the real "hexagonal cut concrete forest" structure,
/// 167 vertices / 270 B-spline edges / 127 loops / 127 planar faces / 12 shells / 12 solids, derived
/// ONCE from the real committed Rhino BIM export
/// `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left-bim.stp` by
/// `🐍️derive-brep-fixture.py` in the ticket folder. Every semio id carries the STEP entity number it
/// came from.
#[cfg(feature = "sut")]
const FOREST_DSL: &str = "local://🗣️hexagonal-cut-concrete-forest-left.dsl.semio";
/// 🎒️ The same structure in its binary envelope, written by the PYTHON implementation — so this
/// codec reproducing it is a cross-language byte agreement, not a codec agreeing with itself.
#[cfg(feature = "sut")]
const FOREST_PACK: &str = "local://🎒️.pack.semio";
/// 🧊️ The tiny committed solid — the one that carries a line, a circle, a rational NURBS curve and
/// a NURBS surface at once. It is committed under `✳️any`'s example set, because `✳️brep` commits no
/// example of its own, and it is kept for the BYTE half of the identity law: its two files were
/// written by THIS codec, so the Python side reproducing them is the other direction of the same
/// cross-language agreement.
#[cfg(feature = "sut")]
const SOLID_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🧊️solid/🖼️assets/🗣️.dsl.semio";
/// 🎒️ The same solid in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const SOLID_PACK: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🧊️solid/🖼️assets/🎒️.pack.semio";
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
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{apply_semio_brep_mutation, decode_semio_brep_mutation_json, inverse_semio_brep_mutation, SemioBrepMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{decode_semio_brep_pack, decode_semio_brep_snapshot_json, encode_semio_brep_pack, encode_semio_brep_snapshot_json, parse_semio_brep_dsl, print_semio_brep_dsl, SemioBrepSnapshot};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

    //#region 🔖️Bridges
    /// 🦠️ One planned mutation payload, decoded through this subset's own JSON bridge.
    fn mutation_of(value: &Json, scenario: &str) -> Result<SemioBrepMutation, String> {
        decode_semio_brep_mutation_json(&value.to_string()).map_err(|error| format!("{scenario}: the planned mutation payload must decode: {error}"))
    }

    /// 🧬️ Applies one mutation, turning a refusal into a failure rather than a silent no-op.
    fn apply(snapshot: &mut SemioBrepSnapshot, mutation: &SemioBrepMutation, scenario: &str) -> Result<(), String> {
        let outcome = apply_semio_brep_mutation(snapshot, mutation);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{scenario}: the mutation was rejected: {refusals:?}"))
    }

    /// 🌲️ The real concrete-forest structure, put into the state the scenario's verb is aimed at by
    /// the doc string's own `prepare` list.
    fn prepared(ctx: &Context) -> Result<(SemioBrepSnapshot, SemioBrepMutation), String> {
        let (prepare, mutation) = super::plan_mutations(ctx)?;
        let text = String::from_utf8(ctx.fixture_bytes(super::FOREST_DSL)?).map_err(|error| format!("the concrete-forest artifact is not UTF-8: {error}"))?;
        let mut snapshot = parse_semio_brep_dsl(&text)?;
        for step in &prepare {
            let step = mutation_of(step, &ctx.scenario.id)?;
            apply(&mut snapshot, &step, &ctx.scenario.id)?;
        }
        Ok((snapshot, mutation_of(&mutation, &ctx.scenario.id)?))
    }

    fn projection(snapshot: &SemioBrepSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_brep_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioBrepSnapshot, expected: &SemioBrepSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_brep_snapshot_json(got), encode_semio_brep_snapshot_json(expected))
    }
    //#endregion 🔖️Bridges

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed solid. The projection is the whole resulting
    /// snapshot, so a `delete-vertex` that severed the wrong edges — or a `replace-surface` that
    /// dropped a sphere's radius — diverges from the oracle here rather than passing quietly.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (mut snapshot, mutation) = prepared(ctx)?;
        apply(&mut snapshot, &mutation, &ctx.scenario.id)?;
        let projection = projection(&snapshot)?;
        Ok(Outcome::with_raw(print_semio_brep_dsl(&snapshot).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law, asserted in role: applying the verb and then its OWN computed
    /// inverse must restore the prepared solid exactly — for `delete-vertex` that means the vertex
    /// AND both incident edges with their curves, in the order they were in. The projection carries
    /// the mutated snapshot too, so the thirteen rows do not all project the same value and the
    /// comparison cannot go vacuous.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (base, mutation) = prepared(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &mutation, &ctx.scenario.id)?;
        let mutated = projection(&current)?;
        for step in inverse_semio_brep_mutation(&mutation, &base) {
            apply(&mut current, &step, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the prepared solid", ctx.scenario.id), &current, &base));
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
        let mut current = decode_semio_brep_snapshot_json(&before).map_err(|error| format!("{}: the committed before-snapshot must decode: {error}", ctx.scenario.id))?;
        let expected = decode_semio_brep_snapshot_json(&after).map_err(|error| format!("{}: the committed after-snapshot must decode: {error}", ctx.scenario.id))?;
        let mutation = decode_semio_brep_mutation_json(&payload).map_err(|error| format!("{}: the committed mutation payload must decode: {error}", ctx.scenario.id))?;
        apply(&mut current, &mutation, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        projection(&current).map(Outcome::projection)
    }

    /// 🔁️ One document's two encodings, each re-emitted from the parsed document and required back
    /// byte for byte. Byte-identical re-emission IS expected here — `.dsl.semio` is a fixed-layout
    /// record grammar and `.pack.semio` is its binary twin — so the wave's "output must not equal
    /// input" tripwire would be exactly backwards and its MIRROR law is asserted in its place through
    /// `law::carrier_is_exact`, which fails with the offset of the first differing byte.
    fn carrier_pair(ctx: &Context, dsl_uri: &str, pack_uri: &str, what: &str) -> Result<(SemioBrepSnapshot, Json), String> {
        let dsl_bytes = ctx.fixture_bytes(dsl_uri)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: {what} is not UTF-8: {error}"))?;
        let parsed = parse_semio_brep_dsl(&text)?;
        let printed = print_semio_brep_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_semio_brep_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement(&format!("identity-round-trip: printing {what} back to DSL and reparsing it lost content"), &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(pack_uri)?;
        let unpacked = decode_semio_brep_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement(&format!("identity-round-trip: the binary twin of {what} decodes to a different document than its text"), &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_brep_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_brep_pack(&repacked_bytes)?;
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

    /// 🔁️ Both documents, in both encodings — four files, all four reproduced byte for byte. The
    /// committed solid's two files are this codec's own output and the Python implementation
    /// reproduces them from the grammar and the protocol alone; the concrete forest's two files are
    /// the PYTHON implementation's output and this codec has to reproduce THOSE, 2 466 real `f64`
    /// among them, 98 of which have no exponent-free shortest lexeme and are written positionally.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let (solid, solid_report) = carrier_pair(ctx, super::SOLID_DSL, super::SOLID_PACK, "the committed solid")?;
        if solid.vertices.len() != 3 || solid.edges.len() != 3 || solid.faces.len() != 1 || solid.solids.len() != 1 {
            return Err(format!(
                "identity-round-trip: the committed solid is the three-vertex three-edge one-face one-solid artifact this case describes, but decoded as {}/{}/{}/{}",
                solid.vertices.len(),
                solid.edges.len(),
                solid.faces.len(),
                solid.solids.len()
            ));
        }
        let (forest, forest_report) = carrier_pair(ctx, super::FOREST_DSL, super::FOREST_PACK, "the concrete forest")?;
        if forest.vertices.len() != 167 || forest.edges.len() != 270 || forest.loops.len() != 127 || forest.faces.len() != 127 || forest.shells.len() != 12 || forest.solids.len() != 12 {
            return Err(format!(
                "identity-round-trip: the concrete forest is the 167/270/127/127/12/12 structure this case describes, but decoded as {}/{}/{}/{}/{}/{}",
                forest.vertices.len(),
                forest.edges.len(),
                forest.loops.len(),
                forest.faces.len(),
                forest.shells.len(),
                forest.solids.len()
            ));
        }
        Ok(Outcome::projection(Json::Object(vec![("solid".to_string(), solid_report), ("forest".to_string(), forest_report)])))
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
