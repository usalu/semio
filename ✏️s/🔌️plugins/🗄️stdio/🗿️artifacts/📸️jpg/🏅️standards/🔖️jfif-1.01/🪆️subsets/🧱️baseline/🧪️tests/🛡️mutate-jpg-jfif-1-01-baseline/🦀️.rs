//! 🦀️ JFIF 1.01 🧱️baseline conformance-class mutation case — Rust adapter.
//!
//! The oracle role reads the real scan's baseline axes with the registered libjpeg-turbo CLIs
//! (`libjpeg-jpg-jfif-1-01-baseline-marker-cli`: `djpeg -v -v` for the SOFn code, the component sampling
//! factors, the DHT tables and a DAC segment, `rdjpgcom -verbose` for the sample precision), applies
//! each kind to those axes as ITU-T T.81 defines the field it names, and reads the class off the
//! specification's own tables (`../../🔮️oracles/🦀️.rs`); it never consults this repository's decoder,
//! snapshot or checker. The subject applies the same row to the decoded snapshot through
//! `JpgBaselineMutation`. Both answer in the same conformance projection, compared field for field.
//!
//! The comparison is on axes, not bytes: `encode_jpg` writes a conforming baseline file and nothing
//! else, so every axis is normalized away on re-serialization. The decode/re-encode law is its own
//! case, `../🔁️round-trip-jpg-jfif-1-01-baseline`.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::baseline::{apply, project, read_axes, Axes};

//#region 🔖️Kinds

/// 🖼️ The real 2275x2560 architectural scan, shared with the `🧾️document` case rather than copied: two
/// DQT, an SOF0 with three components, four DHT and SOS.
const SCAN: &str = "shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg";
//#endregion 🔖️Kinds

//#region 🔖️Oracle
/// 📖️ The scan's axes as libjpeg-turbo reads them from a copy in the work directory.
fn scan_axes(ctx: &Context) -> Result<Axes, String> {
    read_axes(&ctx.copy_fixture(SCAN, Some("scan.jpg"))?)
}

/// 🎯️ The reference answer for one row: the axes after the kind, which must have moved.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = scan_axes(ctx)?;
    let next = apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?;
    let projection = project(&next);
    if projection == project(&base) {
        return Err(format!("mutate-{kind}: the reference reading of the scan did not move"));
    }
    Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
}

/// ↩️ The reference inverse: every axis the kind touched goes back to the value libjpeg-turbo read.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = scan_axes(ctx)?;
    if project(&apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?) == project(&base) {
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
    use semio_s_artifact_stdio_jpg::io::decode_jpg;
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_jpg::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::{apply_jpg_baseline_mutation, encode_jpg_baseline_projection_json, jpg_baseline_conformance_codes, JpgBaselineMutation};
    use semio_s_artifact_stdio_jpg::JpgSnapshot;
    use semio_repo_test_host::law;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(row: &Json) -> Result<JpgBaselineMutation, String> {
        wire_operation(&row.str("kind"), &row.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Decode
    /// 🖼️ The real scan, decoded into the snapshot the whole case reasons about. The decode is
    /// required to have retained a frame header: `check_baseline_conformance` reports
    /// `stdio.jpg.baseline.no-frame` and certifies nothing without one, so a case that let a
    /// frameless snapshot through would be measuring the absence of the document.
    fn decoded(ctx: &Context) -> Result<JpgSnapshot, String> {
        let snapshot = decode_jpg(&ctx.fixture_bytes(super::SCAN)?).map_err(|error| format!("mutate-jpg-jfif-1-01-baseline: the committed scan must decode: {error:?}"))?;
        let frame = snapshot.frame.as_ref().ok_or("mutate-jpg-jfif-1-01-baseline: the decode retained no SOF0 frame header, so no conformance axis exists to move")?;
        if frame.components.len() != 3 || snapshot.huffman_tables.len() != 4 {
            return Err(format!(
                "mutate-jpg-jfif-1-01-baseline: this case's params are addressed at the committed scan's own SOF0 (three components) and four DHT tables, but the decode read {} component(s) and {} table(s)",
                frame.components.len(),
                snapshot.huffman_tables.len()
            ));
        }
        Ok(snapshot)
    }

    fn projection(snapshot: &JpgSnapshot) -> Result<Json, String> {
        parse_json(&encode_jpg_baseline_projection_json(snapshot))
    }
    //#endregion 🔖️Decode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the real decoded scan and asserts, in role, that the class verdict
    /// moved exactly as the feature's `code` column declares and that the kind's own axis moved. An
    /// empty `code` is a positive claim, not an absence: three kinds move their axis in the
    /// direction that stays INSIDE the class, and those must raise nothing while still being
    /// observable.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let row = ctx.doc_json()?;
        let kind = row.str("kind");
        let kind = kind.as_str();
        let base = decoded(ctx)?;
        let before = jpg_baseline_conformance_codes(&base);
        if !before.is_empty() {
            return Err(format!("mutate-{kind}: the committed scan must start INSIDE the baseline class for a departure from it to mean anything, but it already reports {before:?}"));
        }
        let mutation = mutation_from_spec(&row)?;
        let mut current = base.clone();
        apply_jpg_baseline_mutation(&mut current, &mutation);
        let after = jpg_baseline_conformance_codes(&current);
        let expected = row.str("code");
        if expected.is_empty() {
            if !after.is_empty() {
                return Err(format!("mutate-{kind}: this row moves its axis in the direction that stays inside the class, so the verdict must stay clean, but it reports {after:?}"));
            }
        } else if !after.contains(&expected) {
            return Err(format!("mutate-{kind}: the class verdict must gain {expected:?}, but it reports {after:?} — the mutation did not reach the axis its own diagnostic guards"));
        }
        let (was, now) = (projection(&base)?, projection(&current)?);
        law::mutation_is_observable(kind, &now, &was, &[])?;
        Ok(Outcome::with_raw(now.to_string().into_bytes(), now))
    }

    /// ↩️ The metamorphic inverse law over the real scan: applying the kind and then its OWN
    /// computed inverse must land back on the original conformance projection — every axis, and the
    /// verdict with them. The two counting kinds carry the weight, because their inverse has to
    /// restore a table or a component at the INDEX it was removed from, which is why both variants
    /// carry an index at all.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let row = ctx.doc_json()?;
        let kind = row.str("kind");
        let kind = kind.as_str();
        let base = decoded(ctx)?;
        let original = projection(&base)?;
        let mutation = mutation_from_spec(&row)?;
        let mut current = base.clone();
        apply_jpg_baseline_mutation(&mut current, &mutation);
        if projection(&current)? == original {
            return Err(format!("inverse-{kind}: the forward mutation left the conformance projection untouched, so restoring it proves nothing"));
        }
        for step in mutation_inverse(&mutation, &base) {
            apply_jpg_baseline_mutation(&mut current, &step);
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
