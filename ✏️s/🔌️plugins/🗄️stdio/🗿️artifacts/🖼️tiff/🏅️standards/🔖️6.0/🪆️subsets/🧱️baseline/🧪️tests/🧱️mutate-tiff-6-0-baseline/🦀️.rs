//! 🦀️ TIFF 6.0 🧱️baseline conformance-class mutation case — Rust adapter.
//!
//! The oracle role reads the real scan's five Baseline axes with the registered `tiff` crate reader
//! (`tiff-tiff-6-0-baseline-mutate-reader`, `../../🔮️oracles/🦀️.rs`), applies each kind to those axes as
//! Adobe TIFF 6.0 Part 1 defines the field it names, and reads the class verdict off the
//! specification's own tables; it never consults this repository's decoder, snapshot or checker. The
//! subject applies the same row to the decoded snapshot through `TiffBaselineMutation`. Both answer in
//! the same conformance projection, compared field for field.
//!
//! The comparison is on axes, not bytes: `encode_tiff` regenerates every strip tag from the raster it
//! writes, so four kinds are not byte-observable at all. The decode/re-encode law lives in its own case,
//! `../🔁️round-trip-tiff-6-0-baseline`.
//!
//! **The `setup` column.** `remove-tile-tags` cannot be exercised against a strip-organized scan as
//! it stands — there are no tile tags to remove — so its row names the mutation that makes the
//! removal meaningful, and both its observability and its inverse law are measured from THAT state.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_tiff_test_oracle::standards::v6_0::subsets::baseline::{apply, project, read_axes, Axes};

//#region 🔖️Kinds

/// 🖼️ The real scanned TIFF, shared with the `✳️any` case rather than copied.
const SCAN: &str = "shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff";
//#endregion 🔖️Kinds

//#region 🔖️Oracle
/// 📖️ The scan's axes as the third-party reader sees them, carried through the row's `setup` step.
fn prepared_axes(ctx: &Context, row: &Json) -> Result<Axes, String> {
    let axes = read_axes(&ctx.fixture_bytes(SCAN)?)?;
    let setup = row.get("setup").cloned().unwrap_or(Json::Object(Vec::new()));
    match setup.str("kind").as_str() {
        "" => Ok(axes),
        kind => apply(&axes, kind, &setup.get("params").cloned().unwrap_or(Json::Object(Vec::new()))),
    }
}

/// 🎯️ The reference answer for one row: the axes after the kind, which must have moved.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = prepared_axes(ctx, &row)?;
    let next = apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?;
    let projection = project(&next);
    if projection == project(&base) {
        return Err(format!("mutate-{kind}: the reference reading of the scan did not move"));
    }
    Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
}

/// ↩️ The reference inverse: every axis the kind touched goes back to the value the reader read, which
/// is the prepared document's own projection.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = prepared_axes(ctx, &row)?;
    let next = apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?;
    if project(&next) == project(&base) {
        return Err(format!("inverse-{kind}: the forward kind left the reference reading untouched, so restoring it proves nothing"));
    }
    let restored = project(&base);
    Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::decode_tiff;
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_tiff::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::{apply_tiff_baseline_mutation, encode_tiff_baseline_projection_json, tiff_baseline_conformance_codes, TiffBaselineMutation};
    use semio_repo_test_host::law;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes one `{kind, params}` step — the row itself or its `setup` — through the vocabulary's
    /// derive-generated decoder: `params` is the leaf's own wire payload.
    fn mutation_from_spec(step: &Json) -> Result<TiffBaselineMutation, String> {
        wire_operation(&step.str("kind"), &step.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Decode
    /// 🖼️ The real scan, decoded and then carried through the row's `setup` step when it names one.
    /// The decode is required to have retained an IFD 0 with the strip organization these params are
    /// addressed at: `check_tiff_baseline_conformance` certifies nothing without an IFD, and a case
    /// that let an IFD-less snapshot through would be measuring the absence of the document.
    fn prepared(ctx: &Context, row: &Json) -> Result<TiffSnapshot, String> {
        let mut snapshot = decode_tiff(&ctx.fixture_bytes(super::SCAN)?).map_err(|error| format!("mutate-tiff-6-0-baseline: the committed scan must decode: {error:?}"))?;
        if snapshot.ifds.is_empty() {
            return Err("mutate-tiff-6-0-baseline: the decode retained no IFD, so no conformance axis exists to move".to_string());
        }
        let setup = row.get("setup").cloned().unwrap_or(Json::Object(Vec::new()));
        if !setup.str("kind").is_empty() {
            apply_tiff_baseline_mutation(&mut snapshot, &mutation_from_spec(&setup)?);
        }
        Ok(snapshot)
    }

    fn projection(snapshot: &TiffSnapshot) -> Result<Json, String> {
        parse_json(&encode_tiff_baseline_projection_json(snapshot))
    }
    //#endregion 🔖️Decode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the prepared document and asserts, in role, that the class verdict
    /// moved exactly as the feature's `code` column declares and that the kind's own axis moved. An
    /// empty `code` is a positive claim, not an absence: `remove-tile-tags` and `set-strip-offsets`
    /// move their axis in the direction that stays INSIDE the class, so the document must certify
    /// CLEAN afterwards while still being observable. Reading the empty column as "the verdict is
    /// unchanged" instead would be wrong in exactly the row that carries the most weight:
    /// `remove-tile-tags` runs from the `setup` state, where the document is already OUT of the
    /// class with `tiled-not-baseline` raised, and putting it back inside is the whole point of the
    /// row — the verdict is REQUIRED to change there, from one code to none.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let row = ctx.doc_json()?;
        let kind = row.str("kind");
        let kind = kind.as_str();
        let base = prepared(ctx, &row)?;
        let mutation = mutation_from_spec(&row)?;
        let mut current = base.clone();
        apply_tiff_baseline_mutation(&mut current, &mutation);
        let after = tiff_baseline_conformance_codes(&current);
        let expected = row.str("code");
        if expected.is_empty() {
            if !after.is_empty() {
                return Err(format!("mutate-{kind}: this row moves its axis in the direction that stays INSIDE the class, so the document must certify clean afterwards, but the verdict reports {after:?}"));
            }
        } else if !after.contains(&expected) {
            return Err(format!("mutate-{kind}: the class verdict must gain {expected:?}, but it reports {after:?} — the mutation did not reach the axis its own diagnostic guards"));
        }
        let (was, now) = (projection(&base)?, projection(&current)?);
        law::mutation_is_observable(kind, &now, &was, &[])?;
        Ok(Outcome::with_raw(now.to_string().into_bytes(), now))
    }

    /// ↩️ The metamorphic inverse law over the prepared document: applying the kind and then its OWN
    /// computed inverse must land back on the pre-mutation conformance projection — every tag, and
    /// the verdict with them. `remove-tile-tags` carries the weight, because its payload is a bare
    /// variant with no fields at all, so the undo has to restore both tile values out of the base.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let row = ctx.doc_json()?;
        let kind = row.str("kind");
        let kind = kind.as_str();
        let base = prepared(ctx, &row)?;
        let original = projection(&base)?;
        let mutation = mutation_from_spec(&row)?;
        let mut current = base.clone();
        apply_tiff_baseline_mutation(&mut current, &mutation);
        if projection(&current)? == original {
            return Err(format!("inverse-{kind}: the forward mutation left the conformance projection untouched, so restoring it proves nothing"));
        }
        for step in mutation_inverse(&mutation, &base) {
            apply_tiff_baseline_mutation(&mut current, &step);
        }
        let restored = projection(&current)?;
        law::inverse_restores(kind, &restored, &original)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
