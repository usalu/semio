//! 🔬️ Standalone `gif` 0.13 before/after recipe codec for `s.stdio.gif@89a/✳️base`'s per-kind
//! reader corpus. Sibling to `main.rs` (which builds the untouched `pattern-strip.gif` behind
//! the pre-existing `gif-89a-any-mutate` CROSS-SEMIO oracle) — this binary is a SEPARATE,
//! independent implementation backing the NEW `gif-89a-any-mutate-reader` THIRD-PARTY-LIBRARY
//! oracle, never sharing code with `🦀️oracle.rs`.
//!
//! Two subcommands:
//!   build   <recipe-id> <out-dir>   — writes <out-dir>/<recipe-id>/before.gif [and after.gif]
//!   project <path-to-gif>           — decodes a real GIF89a file and prints a typed JSON
//!                                     projection on stdout (opaque payloads as `*Hex`, the
//!                                     caller/probe hashes them into size+digest, per this
//!                                     artifact's own opaque-payload convention)
//!   list-recipes                    — one recipe id per line
//!
//! Every recipe's BEFORE and (where witnessable — see below) AFTER document is authored directly
//! as typed Rust values in [`recipe`] and handed to [`encode_gif`], which calls nothing but
//! `gif::Encoder`'s own public API (plus the one byte-patch `gif::Encoder::new` itself requires,
//! documented at [`encode_gif`]) — never this repository's own `GifMutation` dispatch.
//!
//! # Witnessability — decided from the crate's OWN public surface, not assumed
//!
//! [`project_gif`] uses only `gif::Decoder`'s public getters: `width`/`height`/`global_palette`/
//! `bg_color`/`repeat`, and per frame `next_frame_info` + `read_into_buffer`. Checked directly
//! against `gif` 0.13.3's source (`reader/mod.rs`, `reader/decoder.rs`, `reader/converter.rs`) —
//! not assumed from `🦀️oracle.rs`'s own (different) technique:
//!
//! - **Readable**: width, height, global palette, background colour index (`Decoder::bg_color`
//!   — a real public getter; only the ENCODE side has no setter, which is a write gap this
//!   binary's own byte-patch works around for fixture construction, not a read gap), loop count,
//!   every per-frame geometry/palette/delay/disposal/transparency/user-input field, and — this is
//!   the one non-obvious finding — the per-frame **interlace flag**, IF read the right way (see
//!   below). 16 of the 21 declared kinds move one of these.
//! - **Not readable, confirmed by grepping the crate's public API for a getter and finding
//!   none**: the pixel-aspect-ratio header byte, comment extension text, and application
//!   extension payloads. `gif::Encoder::write_raw_extension` can WRITE all three; nothing in
//!   `gif::Decoder`'s public surface can read any of them back. `set-pixel-aspect-ratio`,
//!   `insert-comment`, `remove-comment`, `add-app-extension`, `remove-app-extension` are therefore
//!   genuinely un-witnessable by a reader built on this crate's public API alone — registered
//!   `<capability>-uncarried` in `../../../🔣️oracle.json`, never routed around with the same
//!   raw-block scan `component.rs` uses (that would just be a second hand-rolled GIF parser
//!   wearing a reader's name).
//!
//! ## The interlace flag IS publicly readable — `component.rs`'s own doc comment overstates the gap
//!
//! `component.rs` and the shared `raster::gif_image_interlace_flags` both state flatly that
//! "`gif::Decoder` cannot answer this" because `Decoder::read_next_frame` always de-interlaces and
//! resets `Frame::interlaced` to `false` — true of THAT method. But `Decoder::next_frame_info`
//! returns the `Frame` BEFORE pixel decoding, at which point `interlaced` still holds the real
//! flag from the Image Descriptor (`reader/decoder.rs:598`, set well before the
//! `Decoded::FrameMetadata` event `next_frame_info` waits for). Capture `interlaced` from that
//! call, THEN separately fetch pixels via `Decoder::read_into_buffer` (which takes `&self` for the
//! frame it deinterlaces against, so it does not itself reset the flag — only the higher-level
//! `read_frame`/`decode_lzw_encoded_frame` convenience wrappers do that, and neither is called
//! here). `read_into_buffer` itself checks `frame.interlaced` and deinterlaces correctly either
//! way (`reader/converter.rs:165`), so the returned pixel bytes are natural-order regardless —
//! meaning `set-frame-interlace` moves ONLY the `interlaced` flag in this projection, never the
//! pixel digest, exactly like every other geometry-preserving mutation here. This is a real,
//! source-verified finding, not a reproduction of `component.rs`'s own (broader) claim.
//!
//! @see ../../../🔣️oracle.json — the `gif-89a-any-mutate-reader` oracle and the recipe fixtures.
//! @see ../../../🦀️oracle.rs — the CROSS-SEMIO computing oracle this binary shares
//!      nothing with (same crate, different mechanism, registered separately and untouched).

use std::borrow::Cow;
use std::env;
use std::fs;
use std::path::Path;

//#region 🔖️Types
#[derive(Clone)]
struct FrameDoc {
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    interlaced: bool,
    palette: Option<Vec<u8>>,
    indices: Vec<u8>,
    delay: u16,
    dispose: gif::DisposalMethod,
    transparent: Option<u8>,
    needs_user_input: bool,
}

#[derive(Clone)]
struct GifDoc {
    width: u16,
    height: u16,
    global_palette: Vec<u8>,
    bg_color_index: u8,
    /// `None` = no NETSCAPE2.0 loop extension; `Some(0)` = loop forever; `Some(n)` = `n` more times.
    loop_count: Option<u16>,
    frames: Vec<FrameDoc>,
}
//#endregion 🔖️Types

//#region 🔖️Interlace
/// 🔀️ GIF §20's own four-pass row visiting order — written independently here (not shared with
/// `🧪️oracle/🖼️raster/🦀️.rs`'s `gif_interlace_row_order`, on the same "a fixed grammar
/// rule is worth restating over adding a dependency" reasoning the 87a sibling generator already
/// documents for itself).
fn interlace_row_order(height: usize) -> Vec<usize> {
    (0..height).step_by(8).chain((4..height).step_by(8)).chain((2..height).step_by(4)).chain((1..height).step_by(2)).collect()
}

/// 🔀️ `gif::Encoder::write_frame` writes `frame.buffer` verbatim and only flips the interlace bit
/// — it does not itself reorder rows to match. A caller that sets `interlaced: true` must perform
/// this reordering itself, or the written flag and the written rows disagree.
fn reorder_rows_to_interlaced(indices: &[u8], width: usize, height: usize) -> Vec<u8> {
    if width == 0 || height == 0 || indices.len() != width * height {
        return indices.to_vec();
    }
    let mut out = vec![0u8; indices.len()];
    let mut cursor = 0usize;
    for row in interlace_row_order(height) {
        out[cursor..cursor + width].copy_from_slice(&indices[row * width..row * width + width]);
        cursor += width;
    }
    out
}
//#endregion 🔖️Interlace

//#region 🔖️Encode
/// ✍️ Every byte comes from `gif::Encoder`'s own public API, with exactly one patch:
/// `Encoder::new`'s `write_screen_desc` hard-codes the background-colour-index byte to 0 with no
/// setter (`gif` 0.13.3 `src/encoder.rs`) — this binary patches byte 11 of the fixed 13-byte
/// screen descriptor afterward, the same technique `component.rs` uses for the same documented
/// gap. This is a WRITE-side workaround for fixture construction only; it has no bearing on
/// [`project_gif`]'s witnessability, since `Decoder::bg_color` reads that same byte back through a
/// real public getter.
fn encode_gif(doc: &GifDoc) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut out, doc.width, doc.height, &doc.global_palette).expect("gif header");
        if let Some(loop_count) = doc.loop_count {
            let repeat = if loop_count == 0 { gif::Repeat::Infinite } else { gif::Repeat::Finite(loop_count) };
            encoder.set_repeat(repeat).expect("loop extension");
        }
        for frame in &doc.frames {
            let stored = if frame.interlaced { reorder_rows_to_interlaced(&frame.indices, frame.width as usize, frame.height as usize) } else { frame.indices.clone() };
            let gif_frame = gif::Frame {
                delay: frame.delay,
                dispose: frame.dispose,
                transparent: frame.transparent,
                needs_user_input: frame.needs_user_input,
                top: frame.top,
                left: frame.left,
                width: frame.width,
                height: frame.height,
                interlaced: frame.interlaced,
                palette: frame.palette.clone(),
                buffer: Cow::Owned(stored),
            };
            encoder.write_frame(&gif_frame).expect("gif frame");
        }
    }
    assert!(out.len() >= 13, "gif encoder produced a truncated stream");
    out[11] = doc.bg_color_index;
    out
}
//#endregion 🔖️Encode

//#region 🔖️Project
fn disposal_to_str(dispose: gif::DisposalMethod) -> &'static str {
    match dispose {
        gif::DisposalMethod::Any => "unspecified",
        gif::DisposalMethod::Keep => "doNotDispose",
        gif::DisposalMethod::Background => "restoreToBackground",
        gif::DisposalMethod::Previous => "restoreToPrevious",
    }
}

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn json_str(s: &str) -> String {
    format!("{:?}", s)
}

fn opt_num(v: Option<u64>) -> String {
    v.map(|n| n.to_string()).unwrap_or_else(|| "null".to_string())
}

struct ProjectedFrame {
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    interlaced: bool,
    palette_hex: Option<String>,
    delay: u16,
    dispose: gif::DisposalMethod,
    transparent: Option<u8>,
    needs_user_input: bool,
    indices_hex: String,
}

fn frame_json(f: &ProjectedFrame) -> String {
    format!(
        "{{\"left\":{},\"top\":{},\"width\":{},\"height\":{},\"interlaced\":{},\"paletteHex\":{},\"delayCs\":{},\"disposal\":{},\"transparentIndex\":{},\"userInput\":{},\"indicesHex\":{}}}",
        f.left,
        f.top,
        f.width,
        f.height,
        f.interlaced,
        f.palette_hex.as_deref().map(json_str).unwrap_or_else(|| "null".to_string()),
        f.delay,
        json_str(disposal_to_str(f.dispose)),
        opt_num(f.transparent.map(|t| t as u64)),
        f.needs_user_input,
        json_str(&f.indices_hex)
    )
}

/// 📥️ Decodes with the real `gif::Decoder`, using ONLY its public API — see this module's own
/// header docstring for exactly which fields are/are not reachable this way and why.
fn project_gif(bytes: &[u8]) -> Result<String, String> {
    let mut decoder = gif::DecodeOptions::new().read_info(bytes).map_err(|error| format!("independent reader could not parse the GIF: {error}"))?;
    let width = decoder.width();
    let height = decoder.height();
    let global_palette_hex = decoder.global_palette().map(to_hex);
    let bg_color_index = decoder.bg_color();
    let loop_count = match decoder.repeat() {
        gif::Repeat::Infinite => Some(0u16),
        gif::Repeat::Finite(0) => None,
        gif::Repeat::Finite(n) => Some(n),
    };

    let mut frames: Vec<ProjectedFrame> = Vec::new();
    loop {
        // 🧭️ `next_frame_info` returns metadata BEFORE pixel decoding — `interlaced` still holds
        // the real Image Descriptor flag here. Captured into owned locals so the borrow ends
        // before the mutable `read_into_buffer` call below (see module docstring).
        let (left, top, w, h, interlaced, palette, delay, dispose, transparent, needs_user_input) = {
            match decoder.next_frame_info().map_err(|error| format!("independent reader could not read frame metadata: {error}"))? {
                None => break,
                Some(frame) => (frame.left, frame.top, frame.width, frame.height, frame.interlaced, frame.palette.clone(), frame.delay, frame.dispose, frame.transparent, frame.needs_user_input),
            }
        };
        let mut buf = vec![0u8; decoder.buffer_size()];
        decoder.read_into_buffer(&mut buf).map_err(|error| format!("independent reader could not read frame pixels: {error}"))?;
        frames.push(ProjectedFrame { left, top, width: w, height: h, interlaced, palette_hex: palette.as_deref().map(to_hex), delay, dispose, transparent, needs_user_input, indices_hex: to_hex(&buf) });
    }

    let frames_json: Vec<String> = frames.iter().map(frame_json).collect();
    Ok(format!(
        "{{\"width\":{},\"height\":{},\"backgroundColorIndex\":{},\"loopCount\":{},\"globalPaletteHex\":{},\"frameCount\":{},\"frames\":[{}]}}",
        width,
        height,
        opt_num(bg_color_index.map(|n| n as u64)),
        opt_num(loop_count.map(|n| n as u64)),
        global_palette_hex.as_deref().map(json_str).unwrap_or_else(|| "null".to_string()),
        frames.len(),
        frames_json.join(",")
    ))
}
//#endregion 🔖️Project

//#region 🔖️BaseDocument
/// 🧬️ The shared starting document every recipe clones from — 8-colour global palette, 3 frames
/// spanning distinct geometry/local-palette/disposal/transparency/user-input/interlace states, a
/// NETSCAPE2.0 loop extension, and a deliberately non-zero background-colour-index (byte-patched,
/// see [`encode_gif`]) — enough to exercise every one of the 16 witnessable kinds meaningfully.
fn base_doc() -> GifDoc {
    GifDoc {
        width: 8,
        height: 6,
        global_palette: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 255, 0, 255, 0, 0, 0, 255, 255, 255],
        bg_color_index: 2,
        loop_count: Some(3),
        frames: vec![
            FrameDoc { left: 0, top: 0, width: 4, height: 3, interlaced: false, palette: None, indices: (0..12u8).map(|i| i % 4).collect(), delay: 10, dispose: gif::DisposalMethod::Keep, transparent: None, needs_user_input: false },
            FrameDoc { left: 4, top: 0, width: 4, height: 3, interlaced: false, palette: Some(vec![10, 10, 10, 90, 90, 90, 170, 170, 170, 250, 250, 250]), indices: vec![0, 1, 2, 3, 1, 2, 3, 0, 2, 3, 0, 1], delay: 20, dispose: gif::DisposalMethod::Background, transparent: Some(1), needs_user_input: false },
            FrameDoc { left: 0, top: 3, width: 8, height: 3, interlaced: false, palette: None, indices: (0..24u8).rev().map(|i| i % 4).collect(), delay: 30, dispose: gif::DisposalMethod::Previous, transparent: None, needs_user_input: true },
        ],
    }
}
//#endregion 🔖️BaseDocument

//#region 🔖️Recipes
/// 🧪️ One recipe: BEFORE always, AFTER only when the kind is legal (every kind here always is —
/// `gif@89a`'s real dispatch (`../../../🧬️schema/🧬️mutations/🦀️.rs:288`,
/// `MutationOutcome::new(match self {...})`) wraps every one of the 21 kinds uniformly; there is no
/// per-kind rejection branch, only `set-snapshot`'s own documented no-op path — see
/// `set-snapshot-no-op` below, the one recipe whose "after" is BYTE-IDENTICAL to its "before" by
/// design, exactly like `no-mutation-no-op`).
fn recipe(id: &str) -> Option<(GifDoc, GifDoc)> {
    let base = base_doc();
    match id {
        "no-mutation-no-op" => Some((base.clone(), base)),

        "set-snapshot-applied" => {
            let after = GifDoc {
                width: 10,
                height: 8,
                global_palette: vec![20, 20, 20, 200, 200, 200, 40, 40, 200],
                bg_color_index: 1,
                loop_count: Some(0),
                frames: vec![FrameDoc { left: 0, top: 0, width: 5, height: 4, interlaced: false, palette: None, indices: vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1], delay: 5, dispose: gif::DisposalMethod::Any, transparent: None, needs_user_input: false }],
            };
            Some((base, after))
        }
        // 🧭️ The one recipe exercising `set-snapshot`'s documented no-op branch
        // (`../../../🧬️schema/🧬️mutations/📄set-snapshot/🦀️.rs:19` — `base == snapshot` warns
        // `mutation.no-op` and returns `GifDiff::default()`): the replacement is byte-for-byte the
        // same document, so before and after are identical, same convention as `no-mutation-no-op`.
        "set-snapshot-no-op" => Some((base.clone(), base)),

        "set-screen-size-applied" => {
            let mut after = base.clone();
            after.width = 12;
            after.height = 9;
            Some((base, after))
        }

        "set-global-color-table-applied" => {
            let mut after = base.clone();
            after.global_palette = vec![0, 0, 0, 64, 64, 64, 128, 128, 128, 192, 192, 192];
            Some((base, after))
        }

        "set-background-color-index-applied" => {
            let mut after = base.clone();
            after.bg_color_index = 5;
            Some((base, after))
        }

        "set-loop-count-applied" => {
            let mut after = base.clone();
            after.loop_count = Some(7);
            Some((base, after))
        }

        "insert-frame-applied" => {
            let mut after = base.clone();
            after.frames.push(FrameDoc { left: 2, top: 1, width: 4, height: 3, interlaced: false, palette: None, indices: (0..12u8).map(|i| (i + 1) % 4).collect(), delay: 15, dispose: gif::DisposalMethod::Keep, transparent: None, needs_user_input: false });
            Some((base, after))
        }

        "remove-frame-applied" => {
            let mut after = base.clone();
            after.frames.remove(1);
            Some((base, after))
        }

        "move-frame-applied" => {
            let mut after = base.clone();
            let frame = after.frames.remove(0);
            after.frames.push(frame);
            Some((base, after))
        }

        "set-frame-geometry-applied" => {
            let mut after = base.clone();
            after.frames[0].left = 2;
            after.frames[0].top = 1;
            Some((base, after))
        }

        "set-frame-pixels-applied" => {
            let mut after = base.clone();
            after.frames[0].indices = vec![3, 2, 1, 0, 3, 2, 1, 0, 3, 2, 1, 0];
            Some((base, after))
        }

        "set-frame-interlace-applied" => {
            let mut after = base.clone();
            after.frames[0].interlaced = true;
            Some((base, after))
        }

        "set-frame-delay-applied" => {
            let mut after = base.clone();
            after.frames[0].delay = 99;
            Some((base, after))
        }

        "set-frame-disposal-applied" => {
            let mut after = base.clone();
            after.frames[0].dispose = gif::DisposalMethod::Background;
            Some((base, after))
        }

        "set-frame-transparency-applied" => {
            let mut after = base.clone();
            after.frames[0].transparent = Some(0);
            Some((base, after))
        }

        "set-frame-user-input-applied" => {
            let mut after = base.clone();
            after.frames[0].needs_user_input = true;
            Some((base, after))
        }

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &[
    "no-mutation-no-op",
    "set-snapshot-applied",
    "set-snapshot-no-op",
    "set-screen-size-applied",
    "set-global-color-table-applied",
    "set-background-color-index-applied",
    "set-loop-count-applied",
    "insert-frame-applied",
    "remove-frame-applied",
    "move-frame-applied",
    "set-frame-geometry-applied",
    "set-frame-pixels-applied",
    "set-frame-interlace-applied",
    "set-frame-delay-applied",
    "set-frame-disposal-applied",
    "set-frame-transparency-applied",
    "set-frame-user-input-applied",
];
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[gif-89a-reader] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(id);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("before.gif"), encode_gif(&before)).expect("write before.gif");
    fs::write(dir.join("after.gif"), encode_gif(&after)).expect("write after.gif");
    eprintln!("[gif-89a-reader] {id}: before.gif + after.gif -> {}", dir.display());
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(error) => {
            eprintln!("[gif-89a-reader] cannot read {path}: {error}");
            return 1;
        }
    };
    match project_gif(&bytes) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(error) => {
            eprintln!("[gif-89a-reader] cannot project {path}: {error}");
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: reader build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: reader project <path-to-gif>");
                std::process::exit(2);
            };
            cmd_project(path)
        }
        Some("list-recipes") => {
            for id in RECIPE_IDS {
                println!("{id}");
            }
            0
        }
        _ => {
            eprintln!("usage: reader build <recipe-id> <out-dir> | project <path-to-gif> | list-recipes");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_recipe_id_resolves() {
        for id in RECIPE_IDS {
            assert!(recipe(id).is_some(), "recipe {id} must resolve");
        }
    }

    #[test]
    fn no_op_recipes_have_byte_identical_before_and_after() {
        for id in ["no-mutation-no-op", "set-snapshot-no-op"] {
            let (before, after) = recipe(id).unwrap();
            assert_eq!(encode_gif(&before), encode_gif(&after), "recipe {id} must be byte-identical");
        }
    }

    #[test]
    fn applied_recipes_change_the_bytes() {
        for id in RECIPE_IDS {
            if id.ends_with("-no-op") {
                continue;
            }
            let (before, after) = recipe(id).unwrap();
            assert_ne!(encode_gif(&before), encode_gif(&after), "recipe {id} must change the bytes");
        }
    }

    #[test]
    fn project_round_trips_the_base_document() {
        let bytes = encode_gif(&base_doc());
        let json = project_gif(&bytes).expect("project the base document");
        assert!(json.contains("\"width\":8"));
        assert!(json.contains("\"frameCount\":3"));
        assert!(json.contains("\"backgroundColorIndex\":2"));
        assert!(json.contains("\"loopCount\":3"));
    }

    #[test]
    fn interlace_flag_is_readable_via_next_frame_info_before_pixel_decode() {
        let mut doc = base_doc();
        doc.frames[0].interlaced = true;
        let bytes = encode_gif(&doc);
        let json = project_gif(&bytes).expect("project an interlaced document");
        let first_frame = json.split("\"frames\":[").nth(1).unwrap();
        assert!(first_frame.starts_with("{\"left\":0,\"top\":0,\"width\":4,\"height\":3,\"interlaced\":true"), "got: {first_frame}");
    }

    #[test]
    fn interlaced_and_natural_encodings_project_the_same_pixel_bytes() {
        let mut interlaced_doc = base_doc();
        interlaced_doc.frames[0].interlaced = true;
        let natural_bytes = encode_gif(&base_doc());
        let interlaced_bytes = encode_gif(&interlaced_doc);
        assert_ne!(natural_bytes, interlaced_bytes, "the stored row order must actually differ");
        let natural_json = project_gif(&natural_bytes).unwrap();
        let interlaced_json = project_gif(&interlaced_bytes).unwrap();
        let extract_indices = |json: &str| json.split("\"indicesHex\":\"").nth(1).unwrap().split('"').next().unwrap().to_string();
        assert_eq!(extract_indices(&natural_json), extract_indices(&interlaced_json), "de-interlaced pixel bytes must be identical regardless of storage order");
    }
}
//#endregion 🔖️Tests

#[cfg(test)]
mod process_local_determinism {
    use super::*;
    #[test]
    fn encoding_every_recipe_twice_in_one_process_is_byte_identical() {
        for id in RECIPE_IDS {
            let (b1, a1) = recipe(id).unwrap();
            let (b2, a2) = recipe(id).unwrap();
            assert_eq!(encode_gif(&b1), encode_gif(&b2), "before bytes for {id} differ across two in-process encodes");
            assert_eq!(encode_gif(&a1), encode_gif(&a2), "after bytes for {id} differ across two in-process encodes");
        }
    }
}
