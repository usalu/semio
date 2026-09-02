//! 🦀️ DWG AC1018 (R2004) `✳️any` mutation case — Rust adapter.
//!
//! ⚠️ This case has a problem its sibling does not, and the whole file is written around stating it
//! rather than papering over it: **no AC1018-stamped file exists in this repository.** Both `.dwg`
//! files committed outside `./compose` — the 148,638-byte `📚️examples/🏛️architectural/🖼️assets/
//! 📄️architectural.dwg` and the 22-byte `📚️examples/🎬️demo/🖼️assets/🖊️example.dwg` — begin with the
//! six characters `AC1024`, and the architectural example's own docstring says so. So this case
//! reads an R2010 container, and what it can honestly claim is narrower than a native-fixture case:
//! it demonstrates that the R2004 stamp is PRODUCIBLE and READABLE at the published offsets, not
//! that an R2004 container was parsed.
//!
//! That is why this adapter's expectations are the mirror image of the AC1024 one's. There, the
//! native `AC1024` stamp must SURVIVE a decode/re-encode. Here, every `set-version-info` and
//! `set-snapshot` row drives the stamp TO `AC1018` and the handler asserts the R2004 label is what
//! an independent preamble reader then reads back — the one AC1018-specific fact this fixture can
//! carry. The identity round trip asserts the complementary thing: the reader is version-agnostic,
//! reporting the stamp the file actually has (`AC1024`) rather than the standard the case is filed
//! under. A case that claimed `AC1018` there would be asserting a fiction.
//!
//! The vocabulary is shared with AC1024 by CONSTRUCTION, not by copy: the plain file-header
//! PREAMBLE — `0x00`..`0x15`: six ASCII version characters at `0x00`, the application
//! maintenance-release byte at `0x12`, the codepage `RS` at `0x13`-`0x14` — is shared by every
//! AC1015+ DWG file, which is what this repository's own production conformance code says in
//! `DwgSnapshot`'s doc comments, sourced there to LibreDWG's `header.spec` field order. So
//! `../../🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` re-exports the
//! AC1024 `DwgMutation` rather than restating it, and the AC1024 oracle module's
//! `every_ac1018_facet_is_a_re_export_of_this_one` test fails the moment that stops being true.
//! What is NOT shared is what the two standards' containers hold behind that header, and this
//! repository has one decoder for both — recorded as a real gap, not resolved here.
//!
//! No differential oracle is registered (`../../🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/
//! 🔣️.json`'s `noOracleDecisions`), so the platform never dispatches the oracle role for
//! this feature; the handlers are registered in the shape every stdio case has, compute a real
//! answer from an independently hand-written preamble reader that never calls this repository's
//! R2004+ decoder, and assert their laws in role.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::dwg::standards::v_ac1018::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_dwg};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, divergence, inverse_restores, round_trip_preserves};

//#region 🔖️Kinds
/// 🧾️ Case-local mirror of the `dwg-ac1018-any` catalog. Duplicated rather than imported: `KINDS`
/// lives in the SUBJECT crate, which the oracle role must never link. The contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list drifts from the catalog.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-version-info"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 🖊️ The only real, non-stub DWG committed to this repository. It is stamped `AC1024`; see the
/// module header for what that costs this case and what it does not.
const INPUT: &str = "asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg";

/// 🧫️ Copies the immutable committed drawing into the work directory and returns the mutable copy's
/// bytes; the committed file is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.dwg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Expectation
/// 🎯️ The version stamp THIS standard is named for — the one every mutating row in this case's
/// Examples table drives the container to, and the only AC1018-specific fact the fixture can carry.
const R2004_VERSION: &str = "AC1018";

/// 🎯️ The stamp the committed container actually has. Named here so the identity scenario can
/// assert the reader reports the file's own version rather than the case's filing.
const FIXTURE_VERSION: &str = "AC1024";

/// 🧭️ What the published offsets predict the projection must read after `kind` is applied with
/// `params`: a stated field wins, an omitted one keeps what the input carried, and `set-snapshot`
/// collapses the container to the 22-byte preamble-only document this artifact's own demo example
/// already has. Derived from the specification's rules, never from the result being judged.
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

/// ⚖️ Fails the scenario unless the projection reads exactly what [`predicted`] says, naming the
/// first field that broke, and — for the two kinds that carry a version — unless the stamp actually
/// landed on `AC1018`. The second half is what makes this an R2004 case rather than a rename of the
/// R2010 one.
fn conforms_as_r2004(kind: &str, projection: &Json, expected: &Json) -> Result<(), String> {
    if let Some(first) = divergence(projection, expected) {
        return Err(format!("{kind:?} did not produce the preamble the published offsets predict — {first}"));
    }
    if kind != "no-mutation" && projection.get("version") != Some(&Json::String(R2004_VERSION.to_string())) {
        return Err(format!("{kind:?} was supposed to leave this container stamped {R2004_VERSION}, the release this standard names; the preamble reads {:?}", projection.get("version")));
    }
    Ok(())
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
/// independent preamble writer applies the kind, the result must read back exactly the triple the
/// published offsets predict, the stamp must have landed on `AC1018`, and — for anything but
/// `no-mutation` — the projection must have MOVED. A row whose params leave it where it was is not
/// a test.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let before = project_dwg(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_dwg(&bytes)?;
    conforms_as_r2004(&kind, &projection, &predicted(&kind, &params_of(&spec), &input)?)?;
    if kind != "no-mutation" && projection == before {
        return Err(format!("{kind:?} left the preamble projection unchanged — a mutation that is not observable proves nothing"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Every `inverse-<kind>` scenario id, and the ORACLE side of the inverse law: the forward
/// mutation stamps the container R2004, the inverse is computed independently against the UNTOUCHED
/// original (`oracle_inverse_spec`, mirroring `DwgMutation::inverse()`'s base-relative semantics),
/// and the restored drawing must project exactly as the original does — which for the two version
/// kinds means the `AC1024` stamp comes BACK, so this scenario also proves the R2004 rewrite was a
/// real edit rather than a projection that never moved.
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

/// 🔒️ The identity round trip, asserted in role under the EXACT-BYTES law — the correct law here,
/// not a missing one: the preamble is fixed-width with no writer freedom, and the R2004+ section map
/// behind it is compressed, checksummed and proprietary, so it is carried through by construction.
/// The check is not a tautology, because `oracle_round_trip` ZEROES the whole 21-byte preamble
/// region before rewriting it from the parsed fields alone.
///
/// ⚠️ This scenario asserts `AC1024`, not `AC1018`, and that is deliberate: the reader must report
/// the stamp the FILE carries, not the standard the case is filed under. Asserting `AC1018` here
/// would be asserting a fiction about a fixture this repository does not have.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_dwg(&input)?;
    let bytes = oracle_round_trip(&input)?;
    let projection = project_dwg(&bytes)?;
    round_trip_preserves(&projection, &before)?;
    carrier_is_exact(&bytes, &input)?;
    if projection.get("version") != Some(&Json::String(FIXTURE_VERSION.to_string())) {
        return Err(format!("the preamble reader must report the stamp the file carries ({FIXTURE_VERSION}); it read {:?}", projection.get("version")));
    }
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{conforms_as_r2004, mutable_input, no_mutation, params_of, predicted, FIXTURE_VERSION};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::dwg::standards::v_ac1018::subsets::any::schema::mutations::{apply_dwg_mutation_checked, inverse_dwg_mutation, set_snapshot, set_version_info, DwgMutation};
    use semio_s_plugin_stdio::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::{decode_dwg, encode_dwg, DwgSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::dwg::standards::v_ac1018::subsets::any::project_dwg;
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves};

    /// 🦠️ The `(kind, params)` wire shape read into a real `DwgMutation`, reached through the
    /// AC1018 module path so this case drives the standard it is filed under even though that path
    /// re-exports the AC1024 vocabulary — if the re-export ever stops resolving, this case is the
    /// thing that fails.
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
    /// a splice of the input's own bytes. A rejected mutation is surfaced rather than discarded, via
    /// this subset's own reachability wrapper.
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
        conforms_as_r2004(&kind, &projection, &predicted(&kind, &params_of(&spec), &input)?)?;
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
        if projection.get("version") != Some(&Json::String(FIXTURE_VERSION.to_string())) {
            return Err(format!("this codec must report the stamp the file carries ({FIXTURE_VERSION}); it read {:?}", projection.get("version")));
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
