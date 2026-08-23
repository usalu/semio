//! 🦀️ GIF87a mutation-oracle case — Rust adapter.
//!
//! `oracle` drives the registered `gif` reference crate (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/
//! 🧪️oracle/🦀️component.rs`), `subject` drives this repository's own `decode_gif`/`apply_gif_mutation`/
//! `encode_gif` round trip, and both results are read back by the shared, independent
//! `raster::project_gif` reader before the `semantic-raster-v1` profile compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles
//! the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gif::standards::v87a::subsets::any::{oracle_apply_mutation, oracle_inverse_spec};
use semio_s_plugin_stdio_test_oracle::raster::project_gif;

//#region 🔖️Input
const INPUT: &str = "shared://🖼️dancing-87a.gif";

/// 🏷️ Mirrors `GifMutation::KINDS` (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
/// 🦀️component.rs`) as a literal, like the OBJ/PDF adapters' own `SCENARIOS` constants — the
/// oracle-only host never links the production plugin crate, so this loop cannot import the
/// constant from it. `kinds_match_enum_variants_and_manifest_catalog` (in that mutations module)
/// is what keeps this list honest against the enum.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-screen-size", "set-global-color-table", "set-background-color-index", "set-pixel-aspect-ratio", "insert-image", "remove-image", "move-image", "set-image-geometry", "set-image-pixels", "set-image-interlace"];

fn empty_params() -> Json {
    Json::Object(Vec::new())
}

/// 🧾️ `{"kind": <id>, "params": <params>}` from the scenario's own doc string.
fn spec(ctx: &Context) -> Result<Json, String> {
    ctx.doc_json()
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.fixture_bytes(INPUT)?;
    let bytes = oracle_apply_mutation(&input, &spec(ctx)?)?;
    let projection = project_gif(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.fixture_bytes(INPUT)?;
    let forward = spec(ctx)?;
    let kind = forward.str("kind");
    let params = forward.get("params").cloned().unwrap_or_else(empty_params);
    let mutated = oracle_apply_mutation(&input, &forward)?;
    let inverse = oracle_inverse_spec(&input, &kind, &params)?;
    let restored = oracle_apply_mutation(&mutated, &inverse)?;
    let projection = project_gif(&restored)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity round trip's oracle side: a correct decode/re-encode must project identically
/// to the untouched input, so the oracle simply IS that input — the subject supplies the actual
/// decode/re-encode and enforces the byte-pass-through tripwire on its own side.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.fixture_bytes(INPUT)?;
    let projection = project_gif(&input)?;
    Ok(Outcome::with_raw(input, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{empty_params, spec, INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::raster::project_gif;
    use semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::io::{decode_gif, encode_gif};
    use semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::schema::mutations::{apply_gif_mutation, GifMutation};
    use semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb, GifSnapshot};
    use semio_s_plugin_stdio::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA;

    //#region 🔖️JsonBridge
    /// 🌉️ Mirrors the oracle's own JSON bridge (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🧪️oracle/
    /// 🦀️component.rs`) but builds the REAL `GifMutation`/`GifSnapshot` this repository's own codec
    /// consumes, independently of that mirror — the two are never allowed to call into each other.
    fn num(json: &Json, key: &str) -> Option<f64> {
        match json.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }

    fn bool_field(json: &Json, key: &str) -> Option<bool> {
        match json.get(key) {
            Some(Json::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    fn color_table_from_json(json: &Json) -> Result<GifColorTable, String> {
        let mut colors = Vec::new();
        for color in json.array("colors") {
            colors.push(GifRgb { r: num(&color, "r").ok_or("color table entry missing r")? as u8, g: num(&color, "g").ok_or("color table entry missing g")? as u8, b: num(&color, "b").ok_or("color table entry missing b")? as u8 });
        }
        Ok(GifColorTable { sorted: bool_field(json, "sorted").unwrap_or(false), colors })
    }

    fn image_from_json(json: &Json) -> Result<GifImage, String> {
        let lct = match json.get("lct") {
            Some(Json::Null) | None => None,
            Some(value) => Some(color_table_from_json(value)?),
        };
        let indices = json.array("indices").iter().map(|v| match v { Json::Number(n) => *n as u8, _ => 0 }).collect();
        Ok(GifImage {
            left: num(json, "left").unwrap_or(0.0) as u32,
            top: num(json, "top").unwrap_or(0.0) as u32,
            width: num(json, "width").ok_or("image missing width")? as u32,
            height: num(json, "height").ok_or("image missing height")? as u32,
            interlace: bool_field(json, "interlace").unwrap_or(false),
            lct,
            indices,
        })
    }

    fn snapshot_from_json(json: &Json) -> Result<GifSnapshot, String> {
        let gct = match json.get("gct") {
            Some(Json::Null) | None => None,
            Some(value) => Some(color_table_from_json(value)?),
        };
        let images = json.array("images").iter().map(image_from_json).collect::<Result<Vec<_>, _>>()?;
        Ok(GifSnapshot {
            schema: STDIO_GIF_DOCUMENT_SCHEMA.to_string(),
            width: num(json, "width").ok_or("snapshot missing width")? as u32,
            height: num(json, "height").ok_or("snapshot missing height")? as u32,
            gct,
            background_color_index: num(json, "backgroundColorIndex").unwrap_or(0.0) as u8,
            pixel_aspect_ratio: num(json, "pixelAspectRatio").unwrap_or(0.0) as u8,
            images,
        })
    }

    fn mutation_from_spec(spec: &Json) -> Result<GifMutation, String> {
        let kind = spec.str("kind");
        let empty = empty_params();
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            "no-mutation" => GifMutation::NoMutation,
            "set-snapshot" => GifMutation::SetSnapshot { snapshot: snapshot_from_json(params.get("snapshot").ok_or("set-snapshot: missing snapshot")?)? },
            "set-screen-size" => GifMutation::SetScreenSize { width: num(params, "width").ok_or("set-screen-size: missing width")? as u32, height: num(params, "height").ok_or("set-screen-size: missing height")? as u32 },
            "set-global-color-table" => GifMutation::SetGlobalColorTable {
                gct: match params.get("gct") {
                    Some(Json::Null) | None => None,
                    Some(value) => Some(color_table_from_json(value)?),
                },
            },
            "set-background-color-index" => GifMutation::SetBackgroundColorIndex { index: num(params, "index").ok_or("set-background-color-index: missing index")? as u8 },
            "set-pixel-aspect-ratio" => GifMutation::SetPixelAspectRatio { ratio: num(params, "ratio").ok_or("set-pixel-aspect-ratio: missing ratio")? as u8 },
            "insert-image" => GifMutation::InsertImage { index: num(params, "index").ok_or("insert-image: missing index")? as usize, image: image_from_json(params.get("image").ok_or("insert-image: missing image")?)? },
            "remove-image" => GifMutation::RemoveImage { index: num(params, "index").ok_or("remove-image: missing index")? as usize },
            "move-image" => GifMutation::MoveImage { from: num(params, "from").ok_or("move-image: missing from")? as usize, to: num(params, "to").ok_or("move-image: missing to")? as usize },
            "set-image-geometry" => GifMutation::SetImageGeometry { index: num(params, "index").ok_or("set-image-geometry: missing index")? as usize, left: num(params, "left").ok_or("set-image-geometry: missing left")? as u32, top: num(params, "top").ok_or("set-image-geometry: missing top")? as u32, width: num(params, "width").ok_or("set-image-geometry: missing width")? as u32, height: num(params, "height").ok_or("set-image-geometry: missing height")? as u32 },
            "set-image-pixels" => GifMutation::SetImagePixels { index: num(params, "index").ok_or("set-image-pixels: missing index")? as usize, indices: params.array("indices").iter().map(|v| match v { Json::Number(n) => *n as u8, _ => 0 }).collect() },
            "set-image-interlace" => GifMutation::SetImageInterlace { index: num(params, "index").ok_or("set-image-interlace: missing index")? as usize, interlace: bool_field(params, "interlace").ok_or("set-image-interlace: missing interlace")? },
            other => return Err(format!("mutation kind {:?} is not recognised", other)),
        })
    }

    /// ↩️ The real inverse of `mutation`, relative to `original` (the PRE-mutation snapshot) —
    /// transcribed from `GifMutation::inverse` (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/
    /// 🧬️mutations/🦀️component.rs`) rather than calling it, so this adapter needs no dependency on
    /// the `protocol::Mutation` trait beyond what the plugin crate already re-exports through
    /// `apply_gif_mutation`.
    fn inverse_mutation(original: &GifSnapshot, mutation: &GifMutation) -> GifMutation {
        match mutation {
            GifMutation::NoMutation => GifMutation::NoMutation,
            GifMutation::SetSnapshot { .. } => GifMutation::SetSnapshot { snapshot: original.clone() },
            GifMutation::SetScreenSize { .. } => GifMutation::SetScreenSize { width: original.width, height: original.height },
            GifMutation::SetGlobalColorTable { .. } => GifMutation::SetGlobalColorTable { gct: original.gct.clone() },
            GifMutation::SetBackgroundColorIndex { .. } => GifMutation::SetBackgroundColorIndex { index: original.background_color_index },
            GifMutation::SetPixelAspectRatio { .. } => GifMutation::SetPixelAspectRatio { ratio: original.pixel_aspect_ratio },
            GifMutation::InsertImage { index, .. } => GifMutation::RemoveImage { index: (*index).min(original.images.len()) },
            GifMutation::RemoveImage { index } => match original.images.get(*index) {
                Some(image) => GifMutation::InsertImage { index: *index, image: image.clone() },
                None => GifMutation::NoMutation,
            },
            GifMutation::MoveImage { from, to } => {
                let mut images = original.images.clone();
                let landed_at = if *from < images.len() {
                    let item = images.remove(*from);
                    let at = (*to).min(images.len());
                    images.insert(at, item);
                    at
                } else {
                    *from
                };
                GifMutation::MoveImage { from: landed_at, to: *from }
            }
            GifMutation::SetImageGeometry { index, .. } => match original.images.get(*index) {
                Some(image) => GifMutation::SetImageGeometry { index: *index, left: image.left, top: image.top, width: image.width, height: image.height },
                None => GifMutation::NoMutation,
            },
            GifMutation::SetImagePixels { index, .. } => match original.images.get(*index) {
                Some(image) => GifMutation::SetImagePixels { index: *index, indices: image.indices.clone() },
                None => GifMutation::NoMutation,
            },
            GifMutation::SetImageInterlace { index, .. } => match original.images.get(*index) {
                Some(image) => GifMutation::SetImageInterlace { index: *index, interlace: image.interlace },
                None => GifMutation::NoMutation,
            },
        }
    }
    //#endregion 🔖️JsonBridge

    /// 🧫️ Copies the immutable fixture into the work directory and decodes the mutable copy through
    /// the repository's own, complete GIF87a parser.
    fn original_snapshot(ctx: &Context) -> Result<GifSnapshot, String> {
        let copy = ctx.copy_fixture(INPUT, Some("input.gif"))?;
        let bytes = std::fs::read(&copy).map_err(|error| error.to_string())?;
        decode_gif(&bytes)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = original_snapshot(ctx)?;
        let mutation = mutation_from_spec(&spec(ctx)?)?;
        apply_gif_mutation(&mut snapshot, &mutation);
        let bytes = encode_gif(&snapshot)?;
        let projection = project_gif(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let original = original_snapshot(ctx)?;
        let mutation = mutation_from_spec(&spec(ctx)?)?;
        let mut restored = original.clone();
        apply_gif_mutation(&mut restored, &mutation);
        let undo = inverse_mutation(&original, &mutation);
        apply_gif_mutation(&mut restored, &undo);
        let bytes = encode_gif(&restored)?;
        let projection = project_gif(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let copy = ctx.copy_fixture(INPUT, Some("input.gif"))?;
        let input = std::fs::read(&copy).map_err(|error| error.to_string())?;
        let snapshot = decode_gif(&input)?;
        let output = encode_gif(&snapshot)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".into());
        }
        let projection = project_gif(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registers by the FULL expanded scenario
/// id (`mutate-<kind>`/`inverse-<kind>`), one loop iteration per declared `KINDS` entry, plus the
/// standalone identity round trip.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
