//! 🦀️ Exhaustive GIF 89a mutation case — Rust adapter. Every one of the 21 declared `GifMutation`
//! kinds, applied to a real 4.4 MB / 800x800 / 54-frame animated GIF (`💃️dancing/🖼️assets/🖼️dancing.gif`,
//! committed under the 87a subset's own example directory and read here via `asset://`). The oracle
//! drives the registered `gif` reference implementation; the subject fully parses the artifact into
//! its typed snapshot and re-serializes from it alone — never splicing source bytes. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation — §5.3 of the frozen plan, not a workaround for a broken crate. `semio-s-plugin-stdio`
//! builds; the subject and parity phases both run.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gif::standards::v89a::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_arrange, project};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `GifMutation::KINDS` (`.../🧬️schema/🧬️mutations/🦀️component.rs`) as an adapter-local
/// list rather than importing the subject crate at the top level: the oracle-only host does not link
/// the subject crate at all (`sut` is off), so registration must not name it.
const KINDS: [&str; 21] = [
    "no-mutation",
    "set-snapshot",
    "set-screen-size",
    "set-global-color-table",
    "set-background-color-index",
    "set-pixel-aspect-ratio",
    "set-loop-count",
    "insert-frame",
    "remove-frame",
    "move-frame",
    "set-frame-geometry",
    "set-frame-pixels",
    "set-frame-interlace",
    "set-frame-delay",
    "set-frame-disposal",
    "set-frame-transparency",
    "set-frame-user-input",
    "insert-comment",
    "remove-comment",
    "add-app-extension",
    "remove-app-extension",
];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif";

/// 🧫️ Copies the immutable real-world asset into the work directory and returns the mutable copy's
/// bytes. The committed asset itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.gif"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

fn no_mutation_spec() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(Vec::new()))])
}

/// 🎬️ The document a kind actually acts on. The real animation carries a genuine comment extension
/// and a genuine NETSCAPE2.0 loop extension and nothing else, and the NETSCAPE one is the loop-count
/// axis rather than an `appExtensions` entry — so `remove-app-extension` is handed the real document
/// with its target inserted first, by the reference implementation. Every other kind gets the
/// committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ `@id-mutate`: applies the row's kind, projects the result, and ASSERTS it is distinguishable
/// from the untouched animation. One handler serves all 21 scenario ids — the kind and its params
/// come from the scenario's own doc string. The exemption list is empty: every declared kind of this
/// vocabulary reaches the projection, including `set-frame-interlace`, whose flag the projection now
/// reads off the Image Descriptor rather than from `Frame::interlaced` (which the reference decoder
/// resets to `false` on every read, making the kind invisible).
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = arranged_input(ctx, &spec)?;
    let before = project(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ `@id-inverse`: applies the row's kind, then its computed inverse, and asserts the semantic
/// projection is fully recovered — the algebraic law every mutation kind must satisfy.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = arranged_input(ctx, &spec)?;
    let original_projection = project(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let restored_projection = project(&restored)?;
    law::inverse_restores(&spec.str("kind"), &restored_projection, &original_projection)?;
    Ok(Outcome::with_raw(restored, restored_projection))
}

/// 🔁️ `@id-identity-round-trip`: the no-byte-pass-through tripwire. Decoding and re-encoding through
/// the reference codec alone must change the bytes (a different LZW writer, different block layout)
/// while leaving the semantic projection unchanged.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_apply_mutation(&input, &no_mutation_spec())?;
    law::reparsed_not_copied(&output, &input)?;
    let before = project(&input)?;
    let after = project(&output)?;
    law::round_trip_preserves(&after, &before)?;
    Ok(Outcome::with_raw(output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input, KINDS};
    use semio_repo_test_host::{Adapter, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::artifacts::gif::standards::v89a::subsets::any::project;
    use semio_s_plugin_stdio::ArtifactDsl;
    use semio_s_plugin_stdio::artifacts::gif::standards::v89a::subsets::any::io::{decode_gif, encode_gif};
    use semio_s_plugin_stdio::artifacts::gif::standards::v89a::subsets::any::schema::mutations::{
        add_app_extension, apply_gif_mutation, insert_comment, insert_frame, inverse_gif_mutation, move_frame, remove_app_extension, remove_comment, remove_frame, set_background_color_index, set_frame_delay, set_frame_disposal, set_frame_geometry,
        set_frame_interlace, set_frame_pixels, set_frame_transparency, set_frame_user_input, set_global_color_table, set_loop_count, set_pixel_aspect_ratio, set_screen_size, set_snapshot, GifMutation,
    };
    use semio_s_plugin_stdio::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifRgb, GifSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA};

    //#region 🔖️SpecToMutation
    fn disposal_from_spec(value: &str) -> GifDisposal {
        match value {
            "doNotDispose" => GifDisposal::DoNotDispose,
            "restoreToBackground" => GifDisposal::RestoreToBackground,
            "restoreToPrevious" => GifDisposal::RestoreToPrevious,
            _ => GifDisposal::Unspecified,
        }
    }
    fn indices_from(value: &Json, key: &str) -> Vec<u8> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().map(|item| match item { Json::Number(n) => *n as u8, _ => 0 }).collect(),
            _ => Vec::new(),
        }
    }
    fn gct_from(value: Option<&Json>) -> Option<GifColorTable> {
        match value {
            Some(Json::Array(items)) if !items.is_empty() => Some(GifColorTable {
                sorted: false,
                colors: items
                    .iter()
                    .filter_map(|item| match item {
                        Json::Array(rgb) if rgb.len() >= 3 => Some(GifRgb {
                            r: match &rgb[0] { Json::Number(n) => *n as u8, _ => 0 },
                            g: match &rgb[1] { Json::Number(n) => *n as u8, _ => 0 },
                            b: match &rgb[2] { Json::Number(n) => *n as u8, _ => 0 },
                        }),
                        _ => None,
                    })
                    .collect(),
            }),
            _ => None,
        }
    }
    fn frame_from(value: &Json) -> GifFrame {
        let num = |key: &str, default: f64| match value.get(key) { Some(Json::Number(n)) => *n, _ => default };
        let flag = |key: &str, default: bool| match value.get(key) { Some(Json::Bool(b)) => *b, _ => default };
        GifFrame {
            left: num("left", 0.0) as u32,
            top: num("top", 0.0) as u32,
            width: num("width", 0.0) as u32,
            height: num("height", 0.0) as u32,
            interlace: flag("interlace", false),
            lct: gct_from(value.get("palette")),
            indices: indices_from(value, "indices"),
            delay_cs: num("delayCs", 0.0) as u16,
            disposal: disposal_from_spec(&value.str("disposal")),
            transparent_index: match value.get("transparentIndex") { Some(Json::Number(n)) => Some(*n as u8), _ => None },
            user_input: flag("userInput", false),
            plain_text: None,
        }
    }
    fn app_extension_from(value: &Json) -> GifAppExtension {
        let mut identifier = [0u8; 8];
        let id_bytes = value.str("identifier").into_bytes();
        identifier[..id_bytes.len().min(8)].copy_from_slice(&id_bytes[..id_bytes.len().min(8)]);
        let mut auth_code = [0u8; 3];
        let auth_bytes = value.str("authCode").into_bytes();
        auth_code[..auth_bytes.len().min(3)].copy_from_slice(&auth_bytes[..auth_bytes.len().min(3)]);
        GifAppExtension { identifier, auth_code, data: indices_from(value, "data") }
    }
    fn snapshot_from(value: &Json) -> GifSnapshot {
        let num = |key: &str, default: f64| match value.get(key) { Some(Json::Number(n)) => *n, _ => default };
        GifSnapshot {
            schema: STDIO_GIF89A_DOCUMENT_SCHEMA.to_string(),
            width: num("width", 0.0) as u32,
            height: num("height", 0.0) as u32,
            gct: gct_from(value.get("globalPalette")),
            background_color_index: num("backgroundColorIndex", 0.0) as u8,
            pixel_aspect_ratio: num("aspectRatio", 0.0) as u8,
            loop_count: match value.get("loopCount") { Some(Json::Number(n)) => Some(*n as u16), _ => None },
            frames: value.array("frames").iter().map(frame_from).collect(),
            comments: value.array("comments").iter().map(|item| match item { Json::String(s) => s.clone(), _ => String::new() }).collect(),
            app_extensions: value.array("appExtensions").iter().map(app_extension_from).collect(),
        }
    }

    /// 🔁️ Translates the case's own `{"kind", "params"}` spec into a native `GifMutation` — the same
    /// 21-way dispatch the oracle's `apply_kind` performs, targeting the subject's typed vocabulary
    /// instead of the oracle's owned snapshot.
    fn mutation_from_spec(snapshot: &GifSnapshot, spec: &Json) -> Result<GifMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let num = |key: &str, default: f64| match params.get(key) { Some(Json::Number(n)) => *n, _ => default };
        let opt_num = |key: &str| match params.get(key) { Some(Json::Number(n)) => Some(*n), _ => None };
        let flag = |key: &str, default: bool| match params.get(key) { Some(Json::Bool(b)) => *b, _ => default };
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() }),
            "set-snapshot" => GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_from(&params) }),
            "set-screen-size" => GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: num("width", snapshot.width as f64) as u32, height: num("height", snapshot.height as f64) as u32 }),
            "set-global-color-table" => GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: gct_from(params.get("colors")) }),
            "set-background-color-index" => GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: num("index", 0.0) as u8 }),
            "set-pixel-aspect-ratio" => GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: num("ratio", 0.0) as u8 }),
            "set-loop-count" => GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: opt_num("loopCount").map(|n| n as u16) }),
            "insert-frame" => {
                let mut frame = match params.get("frame") {
                    Some(frame_json) => frame_from(frame_json),
                    None => {
                        let source = num("sourceFrame", 0.0) as usize;
                        snapshot.frames.get(source).cloned().ok_or("insert-frame: sourceFrame out of range")?
                    }
                };
                if let Some(delay) = opt_num("delayCs") {
                    frame.delay_cs = delay as u16;
                }
                GifMutation::InsertFrame(insert_frame::InsertFrame { index: num("index", snapshot.frames.len() as f64) as usize, frame })
            }
            "remove-frame" => GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: num("index", 0.0) as usize }),
            "move-frame" => GifMutation::MoveFrame(move_frame::MoveFrame { from: num("from", 0.0) as usize, to: num("to", 0.0) as usize }),
            "set-frame-geometry" => GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: num("index", 0.0) as usize, left: num("left", 0.0) as u32, top: num("top", 0.0) as u32, width: num("width", 0.0) as u32, height: num("height", 0.0) as u32 }),
            "set-frame-pixels" => {
                let index = num("index", 0.0) as usize;
                let indices = match params.get("indices") {
                    Some(Json::Array(items)) => items.iter().map(|item| match item { Json::Number(n) => *n as u8, _ => 0 }).collect(),
                    _ => {
                        let fill = num("fillIndex", 0.0) as u8;
                        vec![fill; snapshot.frames.get(index).map(|frame| frame.indices.len()).unwrap_or(0)]
                    }
                };
                GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, indices })
            }
            "set-frame-interlace" => GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index: num("index", 0.0) as usize, interlace: flag("interlace", false) }),
            "set-frame-delay" => GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: num("index", 0.0) as usize, delay_cs: num("delayCs", 0.0) as u16 }),
            "set-frame-disposal" => GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: num("index", 0.0) as usize, disposal: disposal_from_spec(&params.str("disposal")) }),
            "set-frame-transparency" => GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: num("index", 0.0) as usize, transparent_index: opt_num("transparentIndex").map(|n| n as u8) }),
            "set-frame-user-input" => GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index: num("index", 0.0) as usize, user_input: flag("userInput", false) }),
            "insert-comment" => GifMutation::InsertComment(insert_comment::InsertComment { index: num("index", snapshot.comments.len() as f64) as usize, text: params.str("text") }),
            "remove-comment" => GifMutation::RemoveComment(remove_comment::RemoveComment { index: num("index", 0.0) as usize }),
            "add-app-extension" => GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: num("index", snapshot.app_extensions.len() as f64) as usize, extension: app_extension_from(&params) }),
            "remove-app-extension" => GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: num("index", 0.0) as usize }),
            other => return Err(format!("mutation kind {:?} has no subject implementation", other)),
        })
    }
    //#endregion 🔖️SpecToMutation

    //#region 🔖️Codec
    /// 🚫️ The no-byte-pass-through channel: complete semantic parse, the subset's own text codec out
    /// and back in, then re-serialize from the model alone — never the source bytes.
    fn decode_through_text(input: &[u8]) -> Result<GifSnapshot, String> {
        let snapshot = decode_gif(input)?;
        let text = ArtifactDsl::print_dsl(&snapshot);
        <GifSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("parse_dsl failed: {error}"))
    }
    //#endregion 🔖️Codec

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode_through_text(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(&snapshot, &spec)?;
        apply_gif_mutation(&mut snapshot, &mutation);
        let bytes = encode_gif(&snapshot).map_err(|error| format!("encode_gif failed: {error}"))?;
        let projection = project(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let original = decode_through_text(&arranged_input(ctx, &spec)?)?;
        let original_projection = project(&encode_gif(&original).map_err(|error| format!("encode_gif failed: {error}"))?)?;
        let mutation = mutation_from_spec(&original, &spec)?;
        let mut mutated = original.clone();
        apply_gif_mutation(&mut mutated, &mutation);
        for inverse in inverse_gif_mutation(&mutation, &original) {
            apply_gif_mutation(&mut mutated, &inverse);
        }
        let bytes = encode_gif(&mutated).map_err(|error| format!("encode_gif failed: {error}"))?;
        let restored_projection = project(&bytes)?;
        if restored_projection != original_projection {
            return Err(format!("inverse of {:?} did not recover the original semantic projection", spec.str("kind")));
        }
        Ok(Outcome::with_raw(bytes, restored_projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_through_text(&input)?;
        let output = encode_gif(&snapshot).map_err(|error| format!("encode_gif failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let before = project(&input)?;
        let after = project(&output)?;
        if before != after {
            return Err("decode/re-encode round trip changed the semantic projection".to_string());
        }
        Ok(Outcome::with_raw(output, after))
    }

    /// 🧭️ Registers all 21 kinds' `mutate`/`inverse` scenario ids plus the round trip, mirroring
    /// `super::adapter`'s oracle registration.
    pub fn register(mut built: Adapter) -> Adapter {
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), mutate);
            built = built.subject(&format!("inverse-{kind}"), inverse);
        }
        built.subject("identity-round-trip", identity_round_trip)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle);
        built = built.oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = subject::register(built);
    }
    built
}
//#endregion 🔖️Registration
