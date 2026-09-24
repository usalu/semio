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
use semio_s_plugin_stdio_test_oracle::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::{apply, project, read_axes, Axes};

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

/// 🎯️ The reference answer for one row: the axes after the kind, which must have moved unless the row
/// is the identity baseline.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = scan_axes(ctx)?;
    let next = apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?;
    let projection = project(&next);
    if kind != "no-mutation" && projection == project(&base) {
        return Err(format!("mutate-{kind}: the reference reading of the scan did not move"));
    }
    Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
}

/// ↩️ The reference inverse: every axis the kind touched goes back to the value libjpeg-turbo read.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = scan_axes(ctx)?;
    if kind != "no-mutation" && project(&apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?) == project(&base) {
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
    use semio_s_artifact_stdio_jpg::io::{decode_jpg, encode_jpg};
    use semio_s_artifact_stdio_jpg::schema::snapshot::{JpgFrameComponent, JpgHuffmanClass, JpgHuffmanTable};
    use semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::{apply_jpg_baseline_mutation, encode_jpg_baseline_projection_json, inverse_jpg_baseline_mutation, jpg_baseline_conformance_codes, JpgBaselineMutation};
    use semio_s_artifact_stdio_jpg::schema::diff::JpgHuffmanTableKey;
    use semio_s_artifact_stdio_jpg::JpgSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::jpg::standards::v_jfif_1_01::subsets::document::project_jpg_mutation;
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Json
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn class_of(params: &Json) -> JpgHuffmanClass {
        match params.str("class").as_str() {
            "ac" => JpgHuffmanClass::Ac,
            _ => JpgHuffmanClass::Dc,
        }
    }

    /// 🧾️ A Huffman table whose bit-length counts and values are irrelevant to every axis this
    /// vocabulary reads — the class only counts tables per class, it never inspects one. Spelled out
    /// rather than defaulted so a reader can see that the payload is deliberately inert.
    fn inert_table(class: JpgHuffmanClass, id: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [0u8; 16], values: Vec::new() }
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🧬️ The feature's ten-kind params grammar, translated into the REAL typed
    /// `JpgBaselineMutation` this subset applies. `set-snapshot` is built from the decoded document
    /// with the three hard axes stamped at once, because a conformance class is a whole-document
    /// property and that variant is the class stamp in its total form.
    fn mutation_from_spec(kind: &str, params: &Json, base: &JpgSnapshot) -> Result<JpgBaselineMutation, String> {
        match kind {
            "set-snapshot" => {
                let mut snapshot = base.clone();
                snapshot.sof_marker = number(params, "sofMarker", 194.0) as u8;
                snapshot.arithmetic = matches!(params.get("arithmetic"), Some(Json::Bool(true)));
                if let Some(frame) = snapshot.frame.as_mut() {
                    frame.precision = number(params, "precision", 12.0) as u8;
                }
                Ok(JpgBaselineMutation::SetSnapshot(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
            }
            "set-sof-marker" => Ok(JpgBaselineMutation::SetSofMarker(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_sof_marker::SetSofMarker { marker: number(params, "marker", 194.0) as u8 })),
            "set-sample-precision" => Ok(JpgBaselineMutation::SetSamplePrecision(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_sample_precision::SetSamplePrecision { precision: number(params, "precision", 12.0) as u8 })),
            "set-arithmetic" => Ok(JpgBaselineMutation::SetArithmetic(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_arithmetic::SetArithmetic { arithmetic: matches!(params.get("arithmetic"), Some(Json::Bool(true))) })),
            "insert-huffman-table" => Ok(JpgBaselineMutation::InsertHuffmanTable(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::insert_huffman_table::InsertHuffmanTable { index: number(params, "index", 0.0) as usize, table: inert_table(class_of(params), number(params, "id", 2.0) as u8) })),
            "remove-huffman-table" => Ok(JpgBaselineMutation::RemoveHuffmanTable(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::remove_huffman_table::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: class_of(params), id: number(params, "id", 0.0) as u8 } })),
            "insert-frame-component" => Ok(JpgBaselineMutation::InsertFrameComponent(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::insert_frame_component::InsertFrameComponent {
                index: number(params, "index", 0.0) as usize,
                component: JpgFrameComponent { id: number(params, "id", 4.0) as u8, h_sampling: number(params, "hSampling", 1.0) as u8, v_sampling: number(params, "vSampling", 1.0) as u8, quant_table_id: 0 },
            })),
            "remove-frame-component" => Ok(JpgBaselineMutation::RemoveFrameComponent(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::remove_frame_component::RemoveFrameComponent { id: number(params, "id", 3.0) as u8 })),
            "set-component-sampling" => Ok(JpgBaselineMutation::SetComponentSampling(semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::set_component_sampling::SetComponentSampling { id: number(params, "id", 1.0) as u8, h_sampling: number(params, "hSampling", 5.0) as u8, v_sampling: number(params, "vSampling", 1.0) as u8 })),
            other => Err(format!("mutate-jpg-jfif-1-01-baseline: no params grammar for kind {other:?}")),
        }
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
        let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let mutation = mutation_from_spec(kind, &params, &base)?;
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
        if kind != "no-mutation" {
            law::mutation_is_observable(kind, &now, &was, &[])?;
        } else if now != was {
            return Err("mutate-no-mutation: the identity element moved the projection".to_string());
        }
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
        let params = row.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let mutation = mutation_from_spec(kind, &params, &base)?;
        let mut current = base.clone();
        apply_jpg_baseline_mutation(&mut current, &mutation);
        if kind != "no-mutation" && projection(&current)? == original {
            return Err(format!("inverse-{kind}: the forward mutation left the conformance projection untouched, so restoring it proves nothing"));
        }
        for step in inverse_jpg_baseline_mutation(&mutation, &base) {
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
    built = built.oracle("mutate", mutate_oracle).oracle("no-mutation-baseline-mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("no-mutation-baseline-inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("no-mutation-baseline-mutate", subject::mutate).subject("inverse", subject::inverse).subject("no-mutation-baseline-inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
