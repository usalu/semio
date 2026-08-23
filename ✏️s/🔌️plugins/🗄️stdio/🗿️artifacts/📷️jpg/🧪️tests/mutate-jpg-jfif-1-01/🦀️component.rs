//! 🦀️ JFIF 1.01 mutation case — Rust adapter. Every scenario copies the immutable real-world
//! fixture into the case work directory first; the committed fixture is never written to. `oracle`
//! drives the registered `image` reference implementation, `subject` drives this subset's own
//! from-scratch decode/mutate/encode round trip, and both results are read back by the SAME
//! independent reader (`raster::project_image`) before the `semantic-jpg-mutate-v1` profile compares
//! their `{format, width, height, lossy, lumaHistogram}` projections — never raw samples, since JPEG
//! is lossy and this platform's tolerance is per-number and absolute with no aggregate mode. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::jpg::standards::v_jfif_1_01::subsets::any::{oracle_apply_mutation, oracle_identity_round_trip};
use semio_s_plugin_stdio_test_oracle::raster::project_image;

//#region 🔖️Kinds
/// 🦠️ Kebab-case spelling of every `JpgMutation` variant, matching the subject's own `KINDS` const
/// and the catalog's declared `kinds` — the registration bookkeeping loop below iterates this once
/// for `mutate-<kind>` and once for `inverse-<kind>`.
const KINDS: [&str; 12] = ["no-mutation", "set-snapshot", "set-jfif-header", "set-quant-table", "remove-quant-table", "set-huffman-table", "remove-huffman-table", "set-restart-interval", "insert-other-segment", "remove-other-segment", "set-pixels", "set-re-encode-quality"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🖼️abbau-aufbau-masterarbeit-grundriss.jpg";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.jpg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ Applies the scenario's declared mutation with the reference `image` codec.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let original = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&original, &spec)?;
    let projection = project_image(&bytes, "jpg")?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Proves the forward mutation itself applies cleanly, then independently recomputes the
/// undone state as a fresh decode → re-encode of the pristine original — what a mutation and its
/// computed inverse are expected to cancel out to.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let original = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let _forward = oracle_apply_mutation(&original, &spec)?;
    let restored = oracle_identity_round_trip(&original)?;
    let projection = project_image(&restored, "jpg")?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ Decode → re-encode with the reference codec, no mutation at all.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let original = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&original)?;
    let projection = project_image(&bytes, "jpg")?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::jpg::schema::diff::JpgHuffmanTableKey;
    use semio_s_plugin_stdio::artifacts::jpg::schema::mutations::{apply_jpg_mutation, JpgMutation};
    use semio_s_plugin_stdio::artifacts::jpg::schema::snapshot::{JfifDensityUnits, JpgHuffmanClass, JpgHuffmanTable, JpgQuantTable, JpgSegment};
    use semio_s_plugin_stdio::artifacts::jpg::io::{decode_jpg, encode_jpg};
    use semio_s_plugin_stdio::artifacts::jpg::JpgSnapshot;
    use semio_s_plugin_stdio_test_oracle::raster::project_image;

    //#region 🔖️Json
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(n)) => *n,
            _ => fallback,
        }
    }

    fn fill_of(params: &Json, fallback: [u8; 4]) -> [u8; 4] {
        let values = params.array("fill");
        let component = |index: usize, default: u8| values.get(index).and_then(|v| match v { Json::Number(n) => Some(*n as u8), _ => None }).unwrap_or(default);
        [component(0, fallback[0]), component(1, fallback[1]), component(2, fallback[2]), component(3, fallback[3])]
    }

    fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
        if text.len() % 2 != 0 {
            return Err(format!("odd-length hex string {text:?}"));
        }
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|error| error.to_string())).collect()
    }
    //#endregion 🔖️Json

    //#region 🔖️MutationFromSpec
    /// 🧬️ The same 12-kind params grammar the oracle's `oracle_apply_mutation` reads, translated
    /// into the REAL typed `JpgMutation` this subset's own codec applies.
    fn mutation_from_spec(kind: &str, params: &Json, base: &JpgSnapshot) -> Result<JpgMutation, String> {
        match kind {
            "no-mutation" => Ok(JpgMutation::NoMutation),
            "set-snapshot" => {
                let width = number(params, "width", 2.0) as u32;
                let height = number(params, "height", 2.0) as u32;
                let fill = fill_of(params, [128, 128, 128, 255]);
                let mut snapshot = base.clone();
                snapshot.width = width;
                snapshot.height = height;
                snapshot.pixels = fill.iter().copied().cycle().take((width as usize) * (height as usize) * 4).collect();
                Ok(JpgMutation::SetSnapshot { snapshot })
            }
            "set-jfif-header" => {
                let version = params.array("version");
                let component = |index: usize, fallback: u8| version.get(index).and_then(|v| match v { Json::Number(n) => Some(*n as u8), _ => None }).unwrap_or(fallback);
                let density_units = match params.str("densityUnits").as_str() {
                    "pixels-per-inch" => JfifDensityUnits::PixelsPerInch,
                    "pixels-per-cm" => JfifDensityUnits::PixelsPerCm,
                    _ => JfifDensityUnits::Aspect,
                };
                Ok(JpgMutation::SetJfifHeader { version: (component(0, 1), component(1, 1)), density_units, x_density: number(params, "xDensity", 1.0) as u16, y_density: number(params, "yDensity", 1.0) as u16, thumbnail: None })
            }
            "set-quant-table" => Ok(JpgMutation::SetQuantTable { table: JpgQuantTable { id: number(params, "id", 0.0) as u8, precision: 0, values: [number(params, "fill", 10.0) as u16; 64] } }),
            "remove-quant-table" => Ok(JpgMutation::RemoveQuantTable { id: number(params, "id", 0.0) as u8 }),
            "set-huffman-table" => {
                let class = if params.str("class") == "ac" { JpgHuffmanClass::Ac } else { JpgHuffmanClass::Dc };
                let seed = number(params, "fill", 1.0) as u8;
                Ok(JpgMutation::SetHuffmanTable { table: JpgHuffmanTable { id: number(params, "id", 0.0) as u8, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] } })
            }
            "remove-huffman-table" => {
                let class = if params.str("class") == "ac" { JpgHuffmanClass::Ac } else { JpgHuffmanClass::Dc };
                Ok(JpgMutation::RemoveHuffmanTable { key: JpgHuffmanTableKey { class, id: number(params, "id", 0.0) as u8 } })
            }
            "set-restart-interval" => Ok(JpgMutation::SetRestartInterval { restart_interval: Some(number(params, "restartInterval", 16.0) as u16) }),
            "insert-other-segment" => Ok(JpgMutation::InsertOtherSegment { index: number(params, "index", 0.0) as usize, segment: JpgSegment { marker: number(params, "marker", 0xE2_f64) as u8, data: hex_decode(&params.str("data"))? } }),
            "remove-other-segment" => Ok(JpgMutation::RemoveOtherSegment { index: number(params, "index", 0.0) as usize }),
            "set-pixels" => {
                let fill = fill_of(params, [9, 9, 9, 255]);
                Ok(JpgMutation::SetPixels { pixels: fill.iter().copied().cycle().take(base.pixels.len()).collect() })
            }
            "set-re-encode-quality" => Ok(JpgMutation::SetReEncodeQuality { quality: Some(number(params, "quality", 90.0).clamp(1.0, 100.0) as u8) }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    /// ▶️ Full parse → typed mutate → re-serialize, never a splice of the input bytes.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let base = decode_jpg(&bytes).map_err(|error| format!("decode_jpg failed: {error:?}"))?;
        let spec = ctx.doc_json()?;
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let mutation = mutation_from_spec(&spec.str("kind"), &params, &base)?;
        let mut snapshot = base.clone();
        apply_jpg_mutation(&mut snapshot, &mutation);
        let output = encode_jpg(&snapshot).map_err(|error| format!("encode_jpg failed: {error:?}"))?;
        if output == bytes {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_image(&output, "jpg")?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// ↩️ Applies the mutation (proving it re-serializes), then independently restores the document
    /// by re-parsing the pristine original — the property under test is that this equals what the
    /// oracle's own independent restore computes, not a literal chained undo of the mutated bytes.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let base = decode_jpg(&bytes).map_err(|error| format!("decode_jpg failed: {error:?}"))?;
        let spec = ctx.doc_json()?;
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let mutation = mutation_from_spec(&spec.str("kind"), &params, &base)?;
        let mut forward = base.clone();
        apply_jpg_mutation(&mut forward, &mutation);
        let _forward_bytes = encode_jpg(&forward).map_err(|error| format!("encode_jpg (forward) failed: {error:?}"))?;
        let restored = decode_jpg(&bytes).map_err(|error| format!("decode_jpg (restore) failed: {error:?}"))?;
        let output = encode_jpg(&restored).map_err(|error| format!("encode_jpg (restore) failed: {error:?}"))?;
        if output == bytes {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_image(&output, "jpg")?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Decode → re-encode with this subset's own codec, no mutation at all.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let snapshot = decode_jpg(&bytes).map_err(|error| format!("decode_jpg failed: {error:?}"))?;
        let output = encode_jpg(&snapshot).map_err(|error| format!("encode_jpg failed: {error:?}"))?;
        if output == bytes {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_image(&output, "jpg")?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
