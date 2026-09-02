//! 🦀️ GIF87a mutation-oracle case — Rust adapter.
//!
//! `oracle` drives the registered `gif` reference crate (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/
//! 🦀️oracle.rs`), `subject` drives this repository's own `decode_gif`/`apply_gif_mutation`/
//! `encode_gif` round trip, and both results are read back by that same module's independent
//! `project_gif_87a` reader before the `semantic-raster-v1` profile compares them. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.
//!
//! The projection is this subset's own, not the shared `raster::project_gif`: that one reports
//! screen geometry, per-frame rectangles and an opaque-sample count only, so the Global Color
//! Table, the background-colour index, the pixel-aspect-ratio byte, the interlace flag and the raw
//! index buffers all fell outside the compared surface — five of the twelve declared kinds could
//! not move it at all.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gif::standards::v87a::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, project_gif_87a};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Input
/// 🖼️ The document every mutation row runs on: a genuine GIF87a of 117 704 bytes, derived ONCE from
/// the real animated `💃️dancing` fixture — three real frames of it (0, 20 and 40), cropped to
/// 400×400, 400×400 and 32×32 rectangles of real already-decoded palette indices, with frame 0's real
/// 256-colour local table promoted to the file's own Global Color Table.
const INPUT: &str = "shared://🖼️dancing-87a-large.gif";
/// 🖼️ The 2 936-byte 16×16 derivation this case used to rest on, kept for `identity-round-trip`: it
/// is the smallest genuine GIF87a committed here and the one whose whole index buffer a scenario can
/// still name literally, so nothing it proved is given up.
const SMALL_INPUT: &str = "shared://🖼️dancing-87a.gif";

/// 🏷️ Mirrors `GifMutation::KINDS` (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
/// 🦀️.rs`) as a literal, like the OBJ/PDF adapters' own `SCENARIOS` constants — the
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
/// 👁️ `@id-mutate`: applies the row's kind with the reference `gif` codec and ASSERTS the result is
/// distinguishable from the untouched fixture. Every one of this vocabulary's twelve kinds reaches
/// the projection — nothing is exempt — so the exemption list is empty and stays empty: a kind that
/// stops moving it is a regression in the oracle or the projection, not a fact about GIF87a.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.fixture_bytes(INPUT)?;
    let forward = spec(ctx)?;
    let before = project_gif_87a(&input)?;
    let bytes = oracle_apply_mutation(&input, &forward)?;
    let projection = project_gif_87a(&bytes)?;
    law::mutation_is_observable(&forward.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ `@id-inverse`: applies the row's kind with the reference `gif` codec, applies the reference's
/// OWN computed inverse on top, and ASSERTS the semantic projection is back to the pristine
/// original's. The law is checkable without a subject, so it is checked here rather than left for
/// the parity phase: a scenario that only re-serializes and returns would pass whenever `gif` did
/// not error.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.fixture_bytes(INPUT)?;
    let before = project_gif_87a(&input)?;
    let forward = spec(ctx)?;
    let kind = forward.str("kind");
    let params = forward.get("params").cloned().unwrap_or_else(empty_params);
    let mutated = oracle_apply_mutation(&input, &forward)?;
    let inverse = oracle_inverse_spec(&input, &kind, &params)?;
    let restored = oracle_apply_mutation(&mutated, &inverse)?;
    let projection = project_gif_87a(&restored)?;
    law::inverse_restores(&kind, &projection, &before)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ `@id-identity-round-trip`: the no-byte-pass-through law, asserted on the ORACLE side too.
/// The reference `gif` codec fully parses the real GIF87a and re-serializes it from its own model
/// alone, so the bytes must change (its own LZW writer and block layout are not the fixture's) while
/// the semantic projection must not.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let small = round_trip_oracle_once(&ctx.fixture_bytes(SMALL_INPUT)?)?;
    let large = round_trip_oracle_once(&ctx.fixture_bytes(INPUT)?)?;
    Ok(Outcome::with_raw(large.0, Json::Object(vec![("small".to_string(), small.1), ("large".to_string(), large.1)])))
}

/// 🔁️ The probe itself, over one GIF87a document.
fn round_trip_oracle_once(input: &[u8]) -> Result<(Vec<u8>, Json), String> {
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), empty_params())]);
    let output = oracle_apply_mutation(input, &no_mutation)?;
    law::reparsed_not_copied(&output, input)?;
    let before = project_gif_87a(input)?;
    let after = project_gif_87a(&output)?;
    law::round_trip_preserves(&after, &before)?;
    Ok((output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{empty_params, spec, INPUT, SMALL_INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::artifacts::gif::standards::v87a::subsets::any::project_gif_87a;
    use semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::io::{decode_gif, encode_gif};
    use semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::schema::mutations::{
        apply_gif_mutation, insert_image, move_image, remove_image, set_background_color_index, set_global_color_table, set_image_geometry, set_image_interlace, set_image_pixels, set_pixel_aspect_ratio, set_screen_size, set_snapshot, GifMutation,
    };
    use semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb, GifSnapshot};
    use semio_s_plugin_stdio::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA;

    //#region 🔖️JsonBridge
    /// 🌉️ Mirrors the oracle's own JSON bridge (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🧪️oracle/
    /// 🦀️.rs`) but builds the REAL `GifMutation`/`GifSnapshot` this repository's own codec
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

    fn mutation_from_spec(base: &GifSnapshot, spec: &Json) -> Result<GifMutation, String> {
        let kind = spec.str("kind");
        let empty = empty_params();
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            "no-mutation" => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            "set-snapshot" => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_from_json(params.get("snapshot").ok_or("set-snapshot: missing snapshot")?)? }),
            "set-screen-size" => GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: num(params, "width").ok_or("set-screen-size: missing width")? as u32, height: num(params, "height").ok_or("set-screen-size: missing height")? as u32 }),
            "set-global-color-table" => GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable {
                gct: match params.get("gct") {
                    Some(Json::Null) | None => None,
                    Some(value) => Some(color_table_from_json(value)?),
                },
            }),
            "set-background-color-index" => GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: num(params, "index").ok_or("set-background-color-index: missing index")? as u8 }),
            "set-pixel-aspect-ratio" => GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: num(params, "ratio").ok_or("set-pixel-aspect-ratio: missing ratio")? as u8 }),
            "insert-image" => GifMutation::InsertImage(insert_image::InsertImage { index: num(params, "index").ok_or("insert-image: missing index")? as usize, image: image_from_json(params.get("image").ok_or("insert-image: missing image")?)? }),
            "remove-image" => GifMutation::RemoveImage(remove_image::RemoveImage { index: num(params, "index").ok_or("remove-image: missing index")? as usize }),
            "move-image" => GifMutation::MoveImage(move_image::MoveImage { from: num(params, "from").ok_or("move-image: missing from")? as usize, to: num(params, "to").ok_or("move-image: missing to")? as usize }),
            "set-image-geometry" => GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: num(params, "index").ok_or("set-image-geometry: missing index")? as usize, left: num(params, "left").ok_or("set-image-geometry: missing left")? as u32, top: num(params, "top").ok_or("set-image-geometry: missing top")? as u32, width: num(params, "width").ok_or("set-image-geometry: missing width")? as u32, height: num(params, "height").ok_or("set-image-geometry: missing height")? as u32 }),
            "set-image-pixels" => GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: num(params, "index").ok_or("set-image-pixels: missing index")? as usize, indices: params.array("indices").iter().map(|v| match v { Json::Number(n) => *n as u8, _ => 0 }).collect() }),
            "set-image-interlace" => GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: num(params, "index").ok_or("set-image-interlace: missing index")? as usize, interlace: bool_field(params, "interlace").ok_or("set-image-interlace: missing interlace")? }),
            other => return Err(format!("mutation kind {:?} is not recognised", other)),
        })
    }

    /// ↩️ The real inverse of `mutation`, relative to `original` (the PRE-mutation snapshot) —
    /// transcribed from `GifMutation::inverse` (`../../🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/
    /// 🧬️mutations/🦀️.rs`) rather than calling it, so this adapter needs no dependency on
    /// the `protocol::Mutation` trait beyond what the plugin crate already re-exports through
    /// `apply_gif_mutation`.
    fn inverse_mutation(original: &GifSnapshot, mutation: &GifMutation) -> GifMutation {
        match mutation {
            GifMutation::SetSnapshot(_) => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: original.clone() }),
            GifMutation::SetScreenSize(_) => GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: original.width, height: original.height }),
            GifMutation::SetGlobalColorTable(_) => GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: original.gct.clone() }),
            GifMutation::SetBackgroundColorIndex(_) => GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: original.background_color_index }),
            GifMutation::SetPixelAspectRatio(_) => GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: original.pixel_aspect_ratio }),
            GifMutation::InsertImage(insert_image::InsertImage { index, .. }) => GifMutation::RemoveImage(remove_image::RemoveImage { index: (*index).min(original.images.len()) }),
            GifMutation::RemoveImage(remove_image::RemoveImage { index }) => match original.images.get(*index) {
                Some(image) => GifMutation::InsertImage(insert_image::InsertImage { index: *index, image: image.clone() }),
                None => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: original.clone() }),
            },
            GifMutation::MoveImage(move_image::MoveImage { from, to }) => {
                let mut images = original.images.clone();
                let landed_at = if *from < images.len() {
                    let item = images.remove(*from);
                    let at = (*to).min(images.len());
                    images.insert(at, item);
                    at
                } else {
                    *from
                };
                GifMutation::MoveImage(move_image::MoveImage { from: landed_at, to: *from })
            }
            GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index, .. }) => match original.images.get(*index) {
                Some(image) => GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: *index, left: image.left, top: image.top, width: image.width, height: image.height }),
                None => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: original.clone() }),
            },
            GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index, .. }) => match original.images.get(*index) {
                Some(image) => GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: *index, indices: image.indices.clone() }),
                None => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: original.clone() }),
            },
            GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index, .. }) => match original.images.get(*index) {
                Some(image) => GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: *index, interlace: image.interlace }),
                None => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: original.clone() }),
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
        let mutation = mutation_from_spec(&snapshot, &spec(ctx)?)?;
        apply_gif_mutation(&mut snapshot, &mutation);
        let bytes = encode_gif(&snapshot)?;
        let projection = project_gif_87a(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let original = original_snapshot(ctx)?;
        let mutation = mutation_from_spec(&original, &spec(ctx)?)?;
        let mut restored = original.clone();
        apply_gif_mutation(&mut restored, &mutation);
        let undo = inverse_mutation(&original, &mutation);
        apply_gif_mutation(&mut restored, &undo);
        let bytes = encode_gif(&restored)?;
        let projection = project_gif_87a(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let small = round_trip_once(ctx, SMALL_INPUT, "small-input.gif")?;
        let large = round_trip_once(ctx, INPUT, "input.gif")?;
        Ok(Outcome::with_raw(large.0, Json::Object(vec![("small".to_string(), small.1), ("large".to_string(), large.1)])))
    }

    /// 🔁️ The probe itself, over one GIF87a document.
    fn round_trip_once(ctx: &Context, uri: &str, name: &str) -> Result<(Vec<u8>, Json), String> {
        let copy = ctx.copy_fixture(uri, Some(name))?;
        let input = std::fs::read(&copy).map_err(|error| error.to_string())?;
        let snapshot = decode_gif(&input)?;
        let output = encode_gif(&snapshot)?;
        if output == input {
            return Err(format!("byte pass-through on {uri}: output is bit-identical to the input"));
        }
        let projection = project_gif_87a(&output)?;
        Ok((output, projection))
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
