//! 🦀️ DWG AC1024 (R2010) `✳️any` mutation case — Rust adapter.
//!
//! This standard is the one the committed drawing is actually stamped with: the first six bytes of
//! `📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg` read `AC1024`. So this case asks the
//! question that only an AC1024 case can ask — does the R2010 stamp SURVIVE, exactly, a full
//! decode/re-encode of the container it labels — and its expectation table is written against the
//! real values the published offsets carry in that file (`AC1024`, `maint_version` 0x02,
//! codepage 30 = ANSI_1252), not against invented ones.
//!
//! No differential oracle is registered (`../../🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧪️oracle/
//! 🔣️.json`'s `noOracleDecisions`: DWG is proprietary, LibreDWG is GPL-3.0 C, and no
//! permissively licensed Rust DWG reader exists), so the platform never dispatches the oracle role
//! for this feature. The handlers below are registered anyway — the shape every stdio case has — and
//! they compute a REAL answer from an independently hand-written preamble reader/writer that never
//! calls this repository's 12,000-line R2004+ decoder, so they assert their laws today through that
//! module's own `cargo test --features oracles --lib` suite and become dispatchable the moment a
//! reference exists.
//!
//! ⚠️ The oracle implementation is shared with the sibling AC1018 case, and that is by
//! CONSTRUCTION rather than by copy: the plain file-header PREAMBLE — `0x00`..`0x15`: six ASCII
//! version characters at `0x00`, the application maintenance-release byte at `0x12`, the codepage
//! `RS` at `0x13`-`0x14` — is shared by every AC1015+ DWG file, which is what this repository's own
//! production conformance code says in `DwgSnapshot`'s doc comments, sourced there to LibreDWG's
//! `header.spec` field order. So one reader serves both, and `🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` re-exports THIS
//! standard's `DwgMutation` instead of restating it. What differs between the two cases is what
//! each one claims and asserts, which is why this file and the AC1018 adapter carry different
//! expectation tables rather than one text under two names.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::dwg::standards::v_ac1024::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_dwg};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves};

//#region 🔖️Kinds
/// 🧾️ Case-local mirror of the `dwg-ac1024-any` catalog. Duplicated rather than imported: `KINDS`
/// itself lives in the SUBJECT crate, which the oracle role must never link. The contract phase
/// fails with `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list drifts from the
/// catalog, and an unregistered scenario id is a hard runtime error, so the duplication is checked.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-version-info"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🖊️ The one real DWG committed to this repository — 148,638 bytes of a genuine architectural
/// drawing. It is filed under the ac1018 example tree; its version stamp is `AC1024`, so for THIS
/// case it is a native fixture and no relabelling caveat applies.
const INPUT: &str = "asset://📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg";

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
    let field = |key: &str| params.get(key).cloned().unwrap_or_else(|| before.get(key).cloned().unwrap_or(Json::Null));
    let triple = vec![("version".to_string(), field("version")), ("maintenanceVersion".to_string(), field("maintenanceVersion")), ("codepage".to_string(), field("codepage"))];
    let length = match kind {
        "set-snapshot" => Json::Number(22.0),
        _ => before.get("byteLength").cloned().unwrap_or(Json::Null),
    };
    Ok(Json::Object(triple.into_iter().chain(std::iter::once(("byteLength".to_string(), length))).collect()))
}

/// ⚖️ Fails the scenario unless the projection reads exactly what [`predicted`] says it must, naming
/// the first field that broke. Without this the handler would pass whenever the writer merely
/// declined to error.
fn conforms(kind: &str, projection: &Json, expected: &Json) -> Result<(), String> {
    match semio_s_plugin_stdio_test_oracle::law::divergence(projection, expected) {
        None => Ok(()),
        Some(first) => Err(format!("{kind:?} did not produce the preamble the published offsets predict — {first}")),
    }
}
//#endregion 🔖️Expectation

//#region 🔖️Oracle
/// 🧾️ The `no-mutation` spec, spelled once. Only the subject half needs it: the oracle side reaches
/// its identity through `oracle_round_trip`, which zeroes the preamble first.
#[cfg(feature = "sut")]
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))])
}

fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Object(vec![]))
}

/// 🔮️ Every `mutate-<kind>` scenario id, asserted IN ROLE against the specification: the
/// independent preamble writer applies the kind and the result must read back exactly the triple
/// the published offsets predict, and — for anything but `no-mutation` — must have MOVED the
/// projection at all. A row whose params leave the projection where it was is not a test.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let before = project_dwg(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_dwg(&bytes)?;
    conforms(&kind, &projection, &predicted(&kind, &params_of(&spec), &input)?)?;
    if kind != "no-mutation" && projection == before {
        return Err(format!("{kind:?} left the preamble projection unchanged — a mutation that is not observable proves nothing"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Every `inverse-<kind>` scenario id, and the ORACLE side of the inverse law: the forward
/// mutation is applied, the inverse is computed independently against the UNTOUCHED original
/// (`oracle_inverse_spec`, mirroring `DwgMutation::inverse()`'s own base-relative semantics), and
/// the restored drawing must project exactly as the original does. `no-mutation` runs the same two
/// steps as every other kind rather than being short-circuited, so the trivial case is evidence.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let original = project_dwg(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&input, &spec)?)?;
    let projection = project_dwg(&restored)?;
    inverse_restores(&kind, &projection, &original)?;
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
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{conforms, mutable_input, no_mutation, params_of, predicted, NATIVE_VERSION};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::dwg::standards::v_ac1024::subsets::any::schema::mutations::{apply_dwg_mutation_checked, inverse_dwg_mutation, set_snapshot, set_version_info, DwgMutation};
    use semio_s_plugin_stdio::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{decode_dwg, encode_dwg, DwgSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::dwg::standards::v_ac1024::subsets::any::project_dwg;
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves};

    /// 🦠️ The `(kind, params)` wire shape read into a real `DwgMutation`. `set-snapshot` builds a
    /// preamble-only document — a genuine whole-document replacement, the same thing the reference
    /// side builds — rather than a field set wearing a different verb's name.
    fn mutation_from_spec(spec: &Json, base: &DwgSnapshot) -> Result<DwgMutation, String> {
        let params = params_of(spec);
        let version = match params.get("version") {
            Some(Json::String(text)) => text.clone(),
            _ => base.version.clone(),
        };
        let number = |key: &str, fallback: f64| match params.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        };
        let maintenance_version = number("maintenanceVersion", f64::from(base.maintenance_version)) as u8;
        let codepage = number("codepage", f64::from(base.codepage)) as u16;
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            "set-version-info" => DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version, maintenance_version, codepage }),
            "set-snapshot" => DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: DwgSnapshot { version, maintenance_version, codepage, ..DwgSnapshot::default() } }),
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }

    /// 📐️ Full parse into the typed `DwgSnapshot` and re-serialization from the model alone — never
    /// a splice of the input's own bytes.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let mut snapshot = decode_dwg(input)?;
        let mutation = mutation_from_spec(spec, &snapshot.clone())?;
        apply_dwg_mutation_checked(&mut snapshot, &mutation)?;
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
        let mutated = apply_and_encode(&input, &spec)?;
        let mut snapshot = decode_dwg(&mutated)?;
        for step in inverse_dwg_mutation(&base, &mutation_from_spec(&spec, &base)?) {
            apply_dwg_mutation_checked(&mut snapshot, &step)?;
        }
        let restored = encode_dwg(&snapshot).map_err(|error| format!("encode_dwg failed: {error}"))?;
        let projection = project_dwg(&restored)?;
        inverse_restores(&kind, &projection, &original)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let before = project_dwg(&input)?;
        let bytes = apply_and_encode(&input, &no_mutation())?;
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
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the outline's rows are enumerated here rather than its base id.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
