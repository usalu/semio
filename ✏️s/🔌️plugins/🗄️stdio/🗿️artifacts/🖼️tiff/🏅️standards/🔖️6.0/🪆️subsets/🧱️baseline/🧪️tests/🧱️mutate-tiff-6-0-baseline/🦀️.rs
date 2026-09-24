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
use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::baseline::{apply, project, read_axes, Axes};

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

/// 🎯️ The reference answer for one row: the axes after the kind, which must have moved unless the
/// row is the identity baseline.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = prepared_axes(ctx, &row)?;
    let next = apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?;
    let projection = project(&next);
    if kind != "no-mutation" && projection == project(&base) {
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
    if kind != "no-mutation" && project(&next) == project(&base) {
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
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::{decode_tiff, encode_tiff};
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
    use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::{apply_tiff_baseline_mutation, encode_tiff_baseline_projection_json, inverse_tiff_baseline_mutation, remove_strip_offsets, remove_tile_tags, tiff_baseline_conformance_codes, TiffBaselineMutation};
    use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::document::project_tiff;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Json
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn numbers(value: &Json, key: &str) -> Vec<f64> {
        value
            .array(key)
            .into_iter()
            .filter_map(|entry| match entry {
                Json::Number(found) => Some(found),
                _ => None,
            })
            .collect()
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🧬️ The feature's nine-kind params grammar, translated into the REAL typed
    /// `TiffBaselineMutation` this subset applies. `set-snapshot` is built from the decoded document
    /// with all three value axes stamped at once, because a conformance class is a whole-document
    /// property and that variant is the class stamp in its total form.
    fn mutation_from_spec(kind: &str, params: &Json, base: &TiffSnapshot) -> Result<TiffBaselineMutation, String> {
        match kind {
            "set-snapshot" => {
                let mut snapshot = base.clone();
                for step in [
                    TiffBaselineMutation::SetCompression(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_compression::SetCompression { compression: number(params, "compression", 5.0) as u16 }),
                    TiffBaselineMutation::SetPhotometricInterpretation(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_photometric_interpretation::SetPhotometricInterpretation { photometric: number(params, "photometric", 6.0) as u16 }),
                    TiffBaselineMutation::SetBitsPerSample(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_bits_per_sample::SetBitsPerSample { bits: numbers(params, "bits").into_iter().map(|value| value as u16).collect() }),
                ] {
                    apply_tiff_baseline_mutation(&mut snapshot, &step);
                }
                Ok(TiffBaselineMutation::SetSnapshot(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
            }
            "set-compression" => Ok(TiffBaselineMutation::SetCompression(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_compression::SetCompression { compression: number(params, "compression", 5.0) as u16 })),
            "set-photometric-interpretation" => Ok(TiffBaselineMutation::SetPhotometricInterpretation(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_photometric_interpretation::SetPhotometricInterpretation { photometric: number(params, "photometric", 6.0) as u16 })),
            "set-bits-per-sample" => Ok(TiffBaselineMutation::SetBitsPerSample(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_bits_per_sample::SetBitsPerSample { bits: numbers(params, "bits").into_iter().map(|value| value as u16).collect() })),
            "insert-tile-tags" => Ok(TiffBaselineMutation::InsertTileTags(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::insert_tile_tags::InsertTileTags { tile_width: number(params, "tileWidth", 256.0) as u32, tile_length: number(params, "tileLength", 256.0) as u32 })),
            "remove-tile-tags" => Ok(TiffBaselineMutation::RemoveTileTags(remove_tile_tags::RemoveTileTags {})),
            "set-strip-offsets" => Ok(TiffBaselineMutation::SetStripOffsets(semio_s_artifact_stdio_tiff::standards::v6_0::subsets::baseline::schema::mutations::set_strip_offsets::SetStripOffsets { offsets: numbers(params, "offsets").into_iter().map(|value| value as u32).collect() })),
            "remove-strip-offsets" => Ok(TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {})),
            other => Err(format!("mutate-tiff-6-0-baseline: no params grammar for kind {other:?}")),
        }
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
        let setup_kind = setup.str("kind");
        if !setup_kind.is_empty() {
            let params = setup.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
            let step = mutation_from_spec(&setup_kind, &params, &snapshot)?;
            apply_tiff_baseline_mutation(&mut snapshot, &step);
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
        let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let mutation = mutation_from_spec(kind, &params, &base)?;
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
        if kind != "no-mutation" {
            law::mutation_is_observable(kind, &now, &was, &[])?;
        } else if now != was {
            return Err("mutate-no-mutation: the identity element moved the projection".to_string());
        }
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
        let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let mutation = mutation_from_spec(kind, &params, &base)?;
        let mut current = base.clone();
        apply_tiff_baseline_mutation(&mut current, &mutation);
        if kind != "no-mutation" && projection(&current)? == original {
            return Err(format!("inverse-{kind}: the forward mutation left the conformance projection untouched, so restoring it proves nothing"));
        }
        for step in inverse_tiff_baseline_mutation(&mutation, &base) {
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
    built = built.oracle("mutate", mutate_oracle).oracle("no-mutation-baseline-mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("no-mutation-baseline-inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("no-mutation-baseline-mutate", subject::mutate).subject("inverse", subject::inverse).subject("no-mutation-baseline-inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
