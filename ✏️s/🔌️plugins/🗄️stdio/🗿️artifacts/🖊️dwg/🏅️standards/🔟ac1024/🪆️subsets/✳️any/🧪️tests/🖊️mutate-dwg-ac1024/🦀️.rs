//! 🦀️ DWG AC1024 (R2010) `✳️any` mutation case — Rust adapter.
//!
//! This standard is the one the committed drawing is actually stamped with: the first six bytes of
//! `📚️examples/🏛️architectural/🖼️assets/🏛️architectural.dwg` read `AC1024`. So this case asks the
//! question that only an AC1024 case can ask — does the R2010 stamp SURVIVE, exactly, a full
//! decode/re-encode of the container it labels — and its expectation table is written against the
//! real values the published offsets carry in that file (`AC1024`, `maint_version` 0x02,
//! codepage 30 = ANSI_1252), not against invented ones.
//!
//! The oracle role applies each kind with an independently hand-written preamble reader/writer that
//! never calls this repository's R2004+ decoder, and every document it produces is then read by the
//! registered third-party reference, LibreDWG's `dwgread` (`libredwg-dwg-preamble-cli`, run as a
//! separate process): its reading of the version code, `maint_version` and `codepage` — or, for a
//! preamble-only document, the byte length it refuses — must agree with the projection. When
//! `dwgread` is not installed the platform records the case as `oracle-unavailable` instead of
//! dispatching it.
//!
//! ⚠️ The oracle implementation is shared with the sibling AC1018 case, and that is by
//! CONSTRUCTION rather than by copy: the plain file-header PREAMBLE — `0x00`..`0x15`: six ASCII
//! version characters at `0x00`, the application maintenance-release byte at `0x12`, the codepage
//! `RS` at `0x13`-`0x14` — is shared by every AC1015+ DWG file, which is what this repository's own
//! production conformance code says in `DwgSnapshot`'s doc comments, sourced there to LibreDWG's
//! `header.spec` field order. So one reader serves both, and `4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` re-exports THIS
//! standard's `DwgMutation` instead of restating it. What differs between the two cases is what
//! each one claims and asserts, which is why this file and the AC1018 adapter carry different
//! expectation tables rather than one text under two names.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_dwg_test_oracle::standards::v_ac1024::subsets::any::{dwgread_agrees, oracle_apply_mutation, oracle_refusal, oracle_restore, oracle_round_trip, project_dwg};
use semio_repo_test_host::law::{carrier_is_exact, inverse_restores, round_trip_preserves};


//#region 🔖️Input
/// 🖊️ The one real DWG committed to this repository — 148,638 bytes of a genuine architectural
/// drawing. It is filed under the ac1018 example tree; its version stamp is `AC1024`, so for THIS
/// case it is a native fixture and no relabelling caveat applies.
const INPUT: &str = "asset://🏛️architectural/🏛️architectural.dwg";

/// 🧫️ Copies the immutable committed drawing into the work directory and returns the mutable copy's
/// bytes; the committed file is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.dwg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Expectation
/// 🎯️ The version stamp the committed container carries, and the one this standard is named for.
const NATIVE_VERSION: &str = "AC1024";

/// 🧭️ What the published offsets predict the projection must read after `kind` is applied with
/// `params` — computed from the spec's own rules (a stated field wins, an omitted one keeps what the
/// input carried) rather than from the result being judged. `@mode-conformance` means each role
/// answers to the specification, so this table is the thing the handler is measured against.
fn predicted(kind: &str, params: &Json, input: &[u8]) -> Result<Json, String> {
    let before = project_dwg(input)?;
    let stated = if kind == "set-snapshot" { params.get("snapshot").cloned().unwrap_or(Json::Null) } else { params.clone() };
    let field = |key: &str| stated.get(key).cloned().unwrap_or_else(|| before.get(key).cloned().unwrap_or(Json::Null));
    let triple = vec![("version".to_string(), field("version")), ("maintenanceVersion".to_string(), field("maintenanceVersion")), ("codepage".to_string(), field("codepage"))];
    let length = match kind {
        "set-snapshot" => Json::Number(22.0),
        _ => before.get("byteLength").cloned().unwrap_or(Json::Null),
    };
    Ok(Json::Object(triple.into_iter().chain(std::iter::once(("byteLength".to_string(), length))).collect()))
}

/// 🚫️ The projection of a REFUSED row: the frozen outcome code, and the preamble of the drawing the refusal left exactly as
/// it was. Both roles build it from what their own side observed, so parity holds only when both refused AND neither
/// touched a byte.
fn refused(code: &str, untouched: Json) -> Json {
    Json::Object(vec![("refusal".to_string(), Json::String(code.to_string())), ("preamble".to_string(), untouched)])
}

/// ⚖️ Fails the scenario unless the projection reads exactly what [`predicted`] says it must, naming
/// the first field that broke. Without this the handler would pass whenever the writer merely
/// declined to error.
fn conforms(kind: &str, projection: &Json, expected: &Json) -> Result<(), String> {
    match semio_repo_test_host::law::divergence(projection, expected) {
        None => Ok(()),
        Some(first) => Err(format!("{kind:?} did not produce the preamble the published offsets predict — {first}")),
    }
}
//#endregion 🔖️Expectation

//#region 🔖️Oracle
fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Object(vec![]))
}

/// 🔮️ Every `mutate-<kind>` scenario id, asserted IN ROLE against the specification: the
/// independent preamble writer applies the kind and the result must read back exactly the triple
/// the published offsets predict, and must have MOVED the projection at all. A row whose params leave the projection where it was is not a test.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let before = project_dwg(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_dwg(&bytes)?;
    conforms(&kind, &projection, &predicted(&kind, &params_of(&spec), &input)?)?;
    if projection == before {
        return Err(format!("{kind:?} left the preamble projection unchanged — a mutation that is not observable proves nothing"));
    }
    dwgread_agrees(&ctx.work_dir, "mutated.dwg", &bytes, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🚫️ Every `refuse-<kind>` scenario id: the row asks a drawing that carries content for a stamp the writer does not lay
/// its object streams out as. The refusal is derived from the writer contract (`oracle_refusal`), never from this
/// repository's dispatch, and `dwgread` must still read the untouched preamble.
fn refuse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    if oracle_refusal(&input, &spec)?.is_none() {
        return Err(format!("{:?} is applicable to the R2010 container — a refusal row must ask for a stamp the writer refuses", spec.str("kind")));
    }
    let preamble = project_dwg(&input)?;
    dwgread_agrees(&ctx.work_dir, "refused.dwg", &input, &preamble)?;
    Ok(Outcome::with_raw(input, refused("mutation.invariant", preamble)))
}

/// ↩️ Every `inverse-<kind>` scenario id, and the ORACLE side of the inverse law: the forward
/// mutation is applied, the inverse is computed independently against the UNTOUCHED original
/// (`oracle_restore`, mirroring `DwgMutation::inverse()`'s own base-relative semantics), and
/// the restored drawing must project exactly as the original does.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let original = project_dwg(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_restore(&input, &mutated, &spec)?;
    let projection = project_dwg(&restored)?;
    inverse_restores(&kind, &projection, &original)?;
    dwgread_agrees(&ctx.work_dir, "restored.dwg", &restored, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The identity round trip, asserted in role under the EXACT-BYTES law rather than the
/// no-byte-pass-through one — and that is the correct law here, not a missing one. The preamble is
/// fixed-width with no writer freedom whatsoever, and the R2004+ section map behind it is a
/// compressed, checksummed, proprietary structure neither this repository nor any permissively
/// licensed Rust crate can regenerate, so it is carried through by construction. The check is still
/// not a tautology: `oracle_round_trip` ZEROES the whole 21-byte preamble region before writing it
/// back from the parsed fields, so byte equality proves those bytes were re-derived from the parse.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_dwg(&input)?;
    let bytes = oracle_round_trip(&input)?;
    let projection = project_dwg(&bytes)?;
    round_trip_preserves(&projection, &before)?;
    carrier_is_exact(&bytes, &input)?;
    if projection.get("version") != Some(&Json::String(NATIVE_VERSION.to_string())) {
        return Err(format!("the committed container is stamped {NATIVE_VERSION}; the round trip read back {:?} instead", projection.get("version")));
    }
    dwgread_agrees(&ctx.work_dir, "round-trip.dwg", &bytes, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{conforms, mutable_input, params_of, predicted, refused, NATIVE_VERSION};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::schema::mutations::DwgMutation;
    use semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::schema::snapshot::{decode_dwg, encode_dwg};
    use semio_s_artifact_stdio_dwg::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_dwg_test_oracle::standards::v_ac1024::subsets::any::project_dwg;
    use semio_repo_test_host::law::{carrier_is_exact, inverse_restores, round_trip_preserves, wire_operation};

    /// 🦠️ The spec's wire payload, decoded by `DwgMutation`'s own payload constructor through the shared stdio bridge, and
    /// held against the payload the subject re-emits from it — the row IS the leaf's wire, member for member.
    fn mutation_of(spec: &Json) -> Result<DwgMutation, String> {
        wire_operation(&spec.str("kind"), &params_of(spec), mutation_from_payload_json, mutation_payload_json)
    }

    /// 📐️ Full parse into the typed `DwgSnapshot` and re-serialization from the model alone — never
    /// a splice of the input's own bytes.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let mut snapshot = decode_dwg(input)?;
        apply_mutation_checked(&mut snapshot, &mutation_of(spec)?)?;
        encode_dwg(&snapshot).map_err(|error| format!("encode_dwg failed: {error}"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let bytes = apply_and_encode(&input, &spec)?;
        let projection = project_dwg(&bytes)?;
        conforms(&kind, &projection, &predicted(&kind, &params_of(&spec), &input)?)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let original = project_dwg(&input)?;
        let base = decode_dwg(&input)?;
        let mut snapshot = decode_dwg(&apply_and_encode(&input, &spec)?)?;
        for step in mutation_inverse(&mutation_of(&spec)?, &base) {
            apply_mutation_checked(&mut snapshot, &step)?;
        }
        let restored = encode_dwg(&snapshot).map_err(|error| format!("encode_dwg failed: {error}"))?;
        let projection = project_dwg(&restored)?;
        inverse_restores(&kind, &projection, &original)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    /// 🚫️ Production dispatch must REFUSE the row and leave the drawing exactly as it was: the projection carries the
    /// refusal's own code and the preamble of the untouched drawing re-encoded from the model, byte for byte the input.
    pub fn refuse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode_dwg(&input)?;
        let Err(refusal) = apply_mutation_checked(&mut snapshot, &mutation_of(&spec)?) else {
            return Err(format!("{:?} was APPLIED — this writer lays a drawing with content out as {NATIVE_VERSION} only", spec.str("kind")));
        };
        let bytes = encode_dwg(&snapshot).map_err(|error| format!("encode_dwg failed: {error}"))?;
        carrier_is_exact(&bytes, &input)?;
        let preamble = project_dwg(&bytes)?;
        Ok(Outcome::with_raw(bytes, refused(&refusal.code, preamble)))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let before = project_dwg(&input)?;
        let bytes = encode_dwg(&decode_dwg(&input)?).map_err(|error| format!("encode_dwg failed: {error}"))?;
        let projection = project_dwg(&bytes)?;
        round_trip_preserves(&projection, &before)?;
        carrier_is_exact(&bytes, &input)?;
        if projection.get("version") != Some(&Json::String(NATIVE_VERSION.to_string())) {
            return Err(format!("the committed container is stamped {NATIVE_VERSION}; this codec read back {:?} instead", projection.get("version")));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("refuse", refuse_oracle);
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("refuse", subject::refuse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
