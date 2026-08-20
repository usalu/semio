//! 🧬️ PngMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `PngDiff` directly via the `schema::diff` builders — apply-and-capture
//! is banned); `inverse()` is handcrafted per variant, index-aware, reading the pre-state it
//! needs from `base`. `apply_png_mutation` follows csv's proven single-source-of-truth shape:
//! `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`.

use crate::artifacts::png::schema::diff::{
    self, dec_background, dec_chromaticities, dec_chunk, dec_chunk_marker, dec_color_type, dec_list, dec_physical_dims, dec_rgb, dec_srgb_intent, dec_str, dec_text_chunk, dec_timestamp, dec_transparency, decode_option, enc_background,
    enc_chromaticities, enc_chunk, enc_chunk_marker, enc_color_type, enc_list, enc_physical_dims, enc_rgb, enc_srgb_intent, enc_str, enc_text_chunk, enc_timestamp, enc_transparency, encode_option, hex_decode, hex_encode, parse_u32, parse_u8,
    split_top_level, strip_brackets, PngDiff,
};
use crate::artifacts::png::schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTimestamp, PngTransparency};
use crate::artifacts::png::PngSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.png`.
/// 🧪️ F6 CONFIRMED (real `cargo check -p semio-s-plugin-stdio --lib` run, `f6-png-mutation-derive-check.txt`
/// in the ticket folder): `#[derive(dsl::DslOps)]` fails with 42 `DslField`-not-satisfied errors —
/// `SetTransparency{trns: Option<PngTransparency>}`/`SetBackground{bkgd: Option<PngBackground>}`
/// carry the data-carrying enums directly, and `SetSnapshot{snapshot: PngSnapshot}` carries them
/// transitively (`PngSnapshot.trns`/`.bkgd`). Same root cause as `PngDiff`'s hand-rolled
/// `DiffCodec` (see `🔺️diff/🦀️component.rs`'s file-header doc comment) — `OpText`/`OpBinary`
/// hand-rolled below, reusing `PngDiff`'s `pub(crate)` grammar primitives instead of duplicating.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PngMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PngSnapshot,
    },
    /// 🧾️ Replaces all five typed IHDR fields at once.
    SetHeader {
        width: u32,
        height: u32,
        bit_depth: u8,
        color_type: PngColorType,
        interlace: bool,
    },
    /// 🎨️ Replaces the `PLTE` palette wholesale (`None` removes the chunk entirely).
    SetPalette {
        plte: Option<Vec<PngRgb>>,
    },
    /// 👁️ Replaces `tRNS` wholesale.
    SetTransparency {
        trns: Option<PngTransparency>,
    },
    SetGamma {
        gama: Option<u32>,
    },
    SetChromaticities {
        chrm: Option<PngChromaticities>,
    },
    SetSrgbIntent {
        srgb: Option<PngSrgbIntent>,
    },
    SetPhysicalDims {
        phys: Option<PngPhysicalDims>,
    },
    SetTimestamp {
        time: Option<PngTimestamp>,
    },
    SetBackground {
        bkgd: Option<PngBackground>,
    },
    /// ➕️ Inserts a whole text chunk at `index` (final position, clamped to `len`).
    InsertTextChunk {
        index: usize,
        chunk: PngTextChunk,
    },
    /// ➖️ Removes the text chunk at `index` (no-op if out of range).
    RemoveTextChunk {
        index: usize,
    },
    /// ✏️ Replaces an existing text chunk's fields wholesale (no-op if out of range).
    SetTextChunk {
        index: usize,
        chunk: PngTextChunk,
    },
    /// 🖼️ Replaces the decoded canonical RGBA8 raster wholesale.
    SetPixels {
        pixels: Vec<u8>,
    },
    /// ➕️ Inserts a verbatim-retained unknown chunk at `index`.
    InsertUnknownChunk {
        index: usize,
        chunk: PngChunk,
    },
    /// ➖️ Removes the unknown chunk at `index` (no-op if out of range).
    RemoveUnknownChunk {
        index: usize,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source (csv precedent).
pub async fn apply_png_mutation(snapshot: &mut PngSnapshot, mutation: &PngMutation) -> protocol::MutationOutcome<PngDiff> {
    let outcome = <PngMutation as Mutation<PngSnapshot>>::diff(mutation, snapshot).await;
    match MutationDiff::apply(outcome.diff().await, snapshot).await {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).await.absorb_messages(outcome.messages().await.to_vec()).await,
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PngSnapshot> for PngMutation {
    type Diff = PngDiff;

    async fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            PngMutation::NoMutation => PngDiff::default(),
            PngMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            PngMutation::SetHeader { width, height, bit_depth, color_type, interlace } => diff::diff_set_header(base, *width, *height, *bit_depth, *color_type, *interlace),
            PngMutation::SetPalette { plte } => diff::diff_set_palette(base, plte),
            PngMutation::SetTransparency { trns } => diff::diff_set_transparency(base, trns),
            PngMutation::SetGamma { gama } => diff::diff_set_gamma(base, *gama),
            PngMutation::SetChromaticities { chrm } => diff::diff_set_chromaticities(base, *chrm),
            PngMutation::SetSrgbIntent { srgb } => diff::diff_set_srgb_intent(base, *srgb),
            PngMutation::SetPhysicalDims { phys } => diff::diff_set_physical_dims(base, *phys),
            PngMutation::SetTimestamp { time } => diff::diff_set_timestamp(base, *time),
            PngMutation::SetBackground { bkgd } => diff::diff_set_background(base, bkgd),
            PngMutation::InsertTextChunk { index, chunk } => diff::diff_insert_text_chunk(base, *index, chunk.clone()),
            PngMutation::RemoveTextChunk { index } => diff::diff_remove_text_chunk(base, *index),
            PngMutation::SetTextChunk { index, chunk } => diff::diff_set_text_chunk(base, *index, chunk.clone()),
            PngMutation::SetPixels { pixels } => diff::diff_set_pixels(base, pixels.clone()),
            PngMutation::InsertUnknownChunk { index, chunk } => diff::diff_insert_unknown_chunk(base, *index, chunk.clone()),
            PngMutation::RemoveUnknownChunk { index } => diff::diff_remove_unknown_chunk(base, *index),
        })
    }

    /// ↩️ Handcrafted, index-aware mutation-level inverses. Out-of-range targets invert to
    /// `NoMutation` (nothing to undo).
    async fn inverse(&self, base: &PngSnapshot) -> Vec<Self> {
        match self {
            PngMutation::NoMutation => vec![PngMutation::NoMutation],
            PngMutation::SetSnapshot { .. } => vec![PngMutation::SetSnapshot { snapshot: base.clone() }],
            PngMutation::SetHeader { .. } => vec![PngMutation::SetHeader { width: base.width, height: base.height, bit_depth: base.bit_depth, color_type: base.color_type, interlace: base.interlace }],
            PngMutation::SetPalette { .. } => vec![PngMutation::SetPalette { plte: base.plte.clone() }],
            PngMutation::SetTransparency { .. } => vec![PngMutation::SetTransparency { trns: base.trns.clone() }],
            PngMutation::SetGamma { .. } => vec![PngMutation::SetGamma { gama: base.gama }],
            PngMutation::SetChromaticities { .. } => vec![PngMutation::SetChromaticities { chrm: base.chrm }],
            PngMutation::SetSrgbIntent { .. } => vec![PngMutation::SetSrgbIntent { srgb: base.srgb }],
            PngMutation::SetPhysicalDims { .. } => vec![PngMutation::SetPhysicalDims { phys: base.phys }],
            PngMutation::SetTimestamp { .. } => vec![PngMutation::SetTimestamp { time: base.time }],
            PngMutation::SetBackground { .. } => vec![PngMutation::SetBackground { bkgd: base.bkgd.clone() }],
            PngMutation::InsertTextChunk { index, .. } => {
                vec![PngMutation::RemoveTextChunk { index: (*index).min(base.text_chunks.len()) }]
            }
            PngMutation::RemoveTextChunk { index } => match base.text_chunks.get(*index) {
                Some(chunk) => vec![PngMutation::InsertTextChunk { index: *index, chunk: chunk.clone() }],
                None => vec![PngMutation::NoMutation],
            },
            PngMutation::SetTextChunk { index, .. } => match base.text_chunks.get(*index) {
                Some(chunk) => vec![PngMutation::SetTextChunk { index: *index, chunk: chunk.clone() }],
                None => vec![PngMutation::NoMutation],
            },
            PngMutation::SetPixels { .. } => vec![PngMutation::SetPixels { pixels: base.pixels.clone() }],
            PngMutation::InsertUnknownChunk { index, .. } => {
                vec![PngMutation::RemoveUnknownChunk { index: (*index).min(base.unknown_chunks.len()) }]
            }
            PngMutation::RemoveUnknownChunk { index } => match base.unknown_chunks.get(*index) {
                Some(chunk) => vec![PngMutation::InsertUnknownChunk { index: *index, chunk: chunk.clone() }],
                None => vec![PngMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `PngMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `PngDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_rgb`/`enc_transparency`/`split_top_level`/`encode_option`/...) rather than
/// duplicating them a second time in this file (same intra-artifact reuse svg's mutations file
/// uses off its own diff file). Grammar: `keyword arg=value ...` (space-separated, matches the
/// derive's own handcrafted-wrapper convention, `f6-recon-report.md` §2), one match arm per
/// variant (no `DslVariants` scaffolding available since nothing here derives it). `SetSnapshot`'s
/// whole-snapshot payload is a single positional `[schema,width,height,...]` tuple reusing every
/// per-field value codec already written for the diff side.
async fn enc_png_snapshot(s: &PngSnapshot) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
        enc_str(&s.schema),
        s.width,
        s.height,
        s.bit_depth,
        enc_color_type(s.color_type),
        if s.interlace { 1 } else { 0 },
        encode_option(&s.plte, |v: &Vec<PngRgb>| enc_list(v, enc_rgb)),
        encode_option(&s.trns, enc_transparency),
        encode_option(&s.gama, |x: &u32| x.to_string()),
        encode_option(&s.chrm, enc_chromaticities),
        encode_option(&s.srgb, |v: &PngSrgbIntent| enc_srgb_intent(*v)),
        encode_option(&s.phys, enc_physical_dims),
        encode_option(&s.time, enc_timestamp),
        encode_option(&s.bkgd, enc_background),
        enc_list(&s.text_chunks, enc_text_chunk),
        hex_encode(&s.pixels),
        enc_list(&s.chunk_order, enc_chunk_marker),
        enc_list(&s.unknown_chunks, enc_chunk),
    )
}
async fn dec_png_snapshot(s: &str) -> Result<PngSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, width, height, bit_depth, color_type, interlace, plte, trns, gama, chrm, srgb, phys, time, bkgd, text_chunks, pixels, chunk_order, unknown_chunks] = parts.as_slice() else {
        return Err(format!("png snapshot: expected 18 fields, got {}", parts.len()));
    };
    Ok(PngSnapshot {
        schema: dec_str(schema)?,
        width: parse_u32(width)?,
        height: parse_u32(height)?,
        bit_depth: parse_u8(bit_depth)?,
        color_type: dec_color_type(color_type)?,
        interlace: *interlace == "1",
        plte: decode_option(plte, |s| dec_list(s, dec_rgb))?,
        trns: decode_option(trns, dec_transparency)?,
        gama: decode_option(gama, parse_u32)?,
        chrm: decode_option(chrm, dec_chromaticities)?,
        srgb: decode_option(srgb, dec_srgb_intent)?,
        phys: decode_option(phys, dec_physical_dims)?,
        time: decode_option(time, dec_timestamp)?,
        bkgd: decode_option(bkgd, dec_background)?,
        text_chunks: dec_list(text_chunks, dec_text_chunk)?,
        pixels: hex_decode(pixels)?,
        chunk_order: dec_list(chunk_order, dec_chunk_marker)?,
        unknown_chunks: dec_list(unknown_chunks, dec_chunk)?,
    })
}

async fn print_png_mutation(m: &PngMutation) -> String {
    match m {
        PngMutation::NoMutation => "no-mutation".to_string(),
        PngMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_png_snapshot(snapshot)),
        PngMutation::SetHeader { width, height, bit_depth, color_type, interlace } => format!("set-header width={width} height={height} bit-depth={bit_depth} color-type={} interlace={}", enc_color_type(*color_type), if *interlace { 1 } else { 0 },),
        PngMutation::SetPalette { plte } => format!("set-palette plte={}", encode_option(plte, |v: &Vec<PngRgb>| enc_list(v, enc_rgb))),
        PngMutation::SetTransparency { trns } => format!("set-transparency trns={}", encode_option(trns, enc_transparency)),
        PngMutation::SetGamma { gama } => format!("set-gamma gama={}", encode_option(gama, |x: &u32| x.to_string())),
        PngMutation::SetChromaticities { chrm } => format!("set-chromaticities chrm={}", encode_option(chrm, enc_chromaticities)),
        PngMutation::SetSrgbIntent { srgb } => format!("set-srgb-intent srgb={}", encode_option(srgb, |v: &PngSrgbIntent| enc_srgb_intent(*v))),
        PngMutation::SetPhysicalDims { phys } => format!("set-physical-dims phys={}", encode_option(phys, enc_physical_dims)),
        PngMutation::SetTimestamp { time } => format!("set-timestamp time={}", encode_option(time, enc_timestamp)),
        PngMutation::SetBackground { bkgd } => format!("set-background bkgd={}", encode_option(bkgd, enc_background)),
        PngMutation::InsertTextChunk { index, chunk } => format!("insert-text-chunk index={index} chunk={}", enc_text_chunk(chunk)),
        PngMutation::RemoveTextChunk { index } => format!("remove-text-chunk index={index}"),
        PngMutation::SetTextChunk { index, chunk } => format!("set-text-chunk index={index} chunk={}", enc_text_chunk(chunk)),
        PngMutation::SetPixels { pixels } => format!("set-pixels pixels={}", hex_encode(pixels)),
        PngMutation::InsertUnknownChunk { index, chunk } => format!("insert-unknown-chunk index={index} chunk={}", enc_chunk(chunk)),
        PngMutation::RemoveUnknownChunk { index } => format!("remove-unknown-chunk index={index}"),
    }
}
async fn parse_png_mutation(line: &str) -> Result<PngMutation, String> {
    if line == "no-mutation" {
        return Ok(PngMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("png mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("png mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(PngMutation::SetSnapshot { snapshot: dec_png_snapshot(arg("snapshot")?).await? }),
        "set-header" => {
            Ok(PngMutation::SetHeader { width: parse_u32(arg("width")?)?, height: parse_u32(arg("height")?)?, bit_depth: parse_u8(arg("bit-depth")?)?, color_type: dec_color_type(arg("color-type")?)?, interlace: arg("interlace")? == "1" })
        }
        "set-palette" => Ok(PngMutation::SetPalette { plte: decode_option(arg("plte")?, |s| dec_list(s, dec_rgb))? }),
        "set-transparency" => Ok(PngMutation::SetTransparency { trns: decode_option(arg("trns")?, dec_transparency)? }),
        "set-gamma" => Ok(PngMutation::SetGamma { gama: decode_option(arg("gama")?, parse_u32)? }),
        "set-chromaticities" => Ok(PngMutation::SetChromaticities { chrm: decode_option(arg("chrm")?, dec_chromaticities)? }),
        "set-srgb-intent" => Ok(PngMutation::SetSrgbIntent { srgb: decode_option(arg("srgb")?, dec_srgb_intent)? }),
        "set-physical-dims" => Ok(PngMutation::SetPhysicalDims { phys: decode_option(arg("phys")?, dec_physical_dims)? }),
        "set-timestamp" => Ok(PngMutation::SetTimestamp { time: decode_option(arg("time")?, dec_timestamp)? }),
        "set-background" => Ok(PngMutation::SetBackground { bkgd: decode_option(arg("bkgd")?, dec_background)? }),
        "insert-text-chunk" => Ok(PngMutation::InsertTextChunk { index: usize_arg("index")?, chunk: dec_text_chunk(arg("chunk")?)? }),
        "remove-text-chunk" => Ok(PngMutation::RemoveTextChunk { index: usize_arg("index")? }),
        "set-text-chunk" => Ok(PngMutation::SetTextChunk { index: usize_arg("index")?, chunk: dec_text_chunk(arg("chunk")?)? }),
        "set-pixels" => Ok(PngMutation::SetPixels { pixels: hex_decode(arg("pixels")?)? }),
        "insert-unknown-chunk" => Ok(PngMutation::InsertUnknownChunk { index: usize_arg("index")?, chunk: dec_chunk(arg("chunk")?)? }),
        "remove-unknown-chunk" => Ok(PngMutation::RemoveUnknownChunk { index: usize_arg("index")? }),
        other => Err(format!("png mutation: unknown keyword {other:?}")),
    }
}

impl OpText for PngMutation {
    async fn print_op(&self) -> String {
        print_png_mutation(self).await
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_png_mutation(line).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️RealBinaryOpFrame
/// 🧪️ P2-P2: **real binary op-frame** for `PngMutation` — upgraded from the F6-era
/// `print_op().into_bytes()` text-as-binary shortcut. `tag u8` ordinal (hand-assigned, this
/// enum cannot use `#[derive(dsl::DslOps)]`, see the doc comment above) + per-variant fields,
/// via `dsl::ByteWriter`/`dsl::ByteReader`, reusing `🔺️diff/🦀️component.rs`'s own
/// `RealBinaryPrimitives` region (`write_bin_option`/`write_bin_snapshot`/`write_bin_rgb`/…)
/// instead of duplicating them a second time — same intra-artifact reuse direction the text
/// codecs above already establish. Matches `../💾️binary/📡️component.protocol.semio`'s real
/// `repeat`/`arm` shape exactly — see that file's own doc comment for why the nested
/// `PngSnapshot`/`PngTransparency`/`PngBackground`/… payload inside most arms is one honest
/// opaque tail blob rather than individually walked at the protocol-description level (the
/// Rust encoding below IS genuinely, fully structured real binary).
async fn op_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "png op binary", offset: 0, detail: e.to_string() }
}

impl OpBinary for PngMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new().await;
        match self {
            PngMutation::NoMutation => {
                w.write_u8(0).await;
            }
            PngMutation::SetSnapshot { snapshot } => {
                w.write_u8(1).await;
                diff::write_bin_snapshot(&mut w, snapshot);
            }
            PngMutation::SetHeader { width, height, bit_depth, color_type, interlace } => {
                w.write_u8(2).await;
                w.write_u32_le(*width).await;
                w.write_u32_le(*height).await;
                w.write_u8(*bit_depth).await;
                w.write_u8(color_type.to_u8()).await;
                w.write_u8(if *interlace { 1 } else { 0 }).await;
            }
            PngMutation::SetPalette { plte } => {
                w.write_u8(3).await;
                diff::write_bin_option(&mut w, plte, |w, v: &Vec<PngRgb>| diff::write_bin_vec(w, v, diff::write_bin_rgb));
            }
            PngMutation::SetTransparency { trns } => {
                w.write_u8(4).await;
                diff::write_bin_option(&mut w, trns, diff::write_bin_transparency);
            }
            PngMutation::SetGamma { gama } => {
                w.write_u8(5).await;
                diff::write_bin_option(&mut w, gama, |w, v: &u32| w.write_u32_le(*v));
            }
            PngMutation::SetChromaticities { chrm } => {
                w.write_u8(6).await;
                diff::write_bin_option(&mut w, chrm, diff::write_bin_chromaticities);
            }
            PngMutation::SetSrgbIntent { srgb } => {
                w.write_u8(7).await;
                diff::write_bin_option(&mut w, srgb, |w, v: &PngSrgbIntent| w.write_u8(v.to_u8()));
            }
            PngMutation::SetPhysicalDims { phys } => {
                w.write_u8(8).await;
                diff::write_bin_option(&mut w, phys, diff::write_bin_physical_dims);
            }
            PngMutation::SetTimestamp { time } => {
                w.write_u8(9).await;
                diff::write_bin_option(&mut w, time, diff::write_bin_timestamp);
            }
            PngMutation::SetBackground { bkgd } => {
                w.write_u8(10).await;
                diff::write_bin_option(&mut w, bkgd, diff::write_bin_background);
            }
            PngMutation::InsertTextChunk { index, chunk } => {
                w.write_u8(11).await;
                w.write_varint_u64(*index as u64).await;
                diff::write_bin_text_chunk(&mut w, chunk);
            }
            PngMutation::RemoveTextChunk { index } => {
                w.write_u8(12).await;
                w.write_varint_u64(*index as u64).await;
            }
            PngMutation::SetTextChunk { index, chunk } => {
                w.write_u8(13).await;
                w.write_varint_u64(*index as u64).await;
                diff::write_bin_text_chunk(&mut w, chunk);
            }
            PngMutation::SetPixels { pixels } => {
                w.write_u8(14).await;
                diff::write_bin_blob(&mut w, pixels);
            }
            PngMutation::InsertUnknownChunk { index, chunk } => {
                w.write_u8(15).await;
                w.write_varint_u64(*index as u64).await;
                diff::write_bin_chunk(&mut w, chunk);
            }
            PngMutation::RemoveUnknownChunk { index } => {
                w.write_u8(16).await;
                w.write_varint_u64(*index as u64).await;
            }
        }
        Ok(w.into_bytes().await)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes).await;
        let ordinal = r.read_u8().await.map_err(op_pack_err)?;
        let mutation = match ordinal {
            0 => PngMutation::NoMutation,
            1 => PngMutation::SetSnapshot { snapshot: diff::read_bin_snapshot(&mut r).map_err(op_pack_err)? },
            2 => PngMutation::SetHeader {
                width: r.read_u32_le().await.map_err(op_pack_err)?,
                height: r.read_u32_le().await.map_err(op_pack_err)?,
                bit_depth: r.read_u8().await.map_err(op_pack_err)?,
                color_type: PngColorType::from_u8(r.read_u8().await.map_err(op_pack_err)?).map_err(|e| protocol::ProtocolError::Malformed { what: "png op color type", offset: 0, detail: e })?,
                interlace: r.read_u8().await.map_err(op_pack_err)? != 0,
            },
            3 => PngMutation::SetPalette { plte: diff::read_bin_option(&mut r, |r| diff::read_bin_vec(r, diff::read_bin_rgb)).map_err(op_pack_err)? },
            4 => PngMutation::SetTransparency { trns: diff::read_bin_option(&mut r, diff::read_bin_transparency).map_err(op_pack_err)? },
            5 => PngMutation::SetGamma { gama: diff::read_bin_option(&mut r, |r| r.read_u32_le()).map_err(op_pack_err)? },
            6 => PngMutation::SetChromaticities { chrm: diff::read_bin_option(&mut r, diff::read_bin_chromaticities).map_err(op_pack_err)? },
            7 => PngMutation::SetSrgbIntent { srgb: diff::read_bin_option(&mut r, |r| PngSrgbIntent::from_u8(r.read_u8()?).map_err(|e| dsl::PackError::Malformed { what: "png op srgb intent", offset: 0, detail: e })).map_err(op_pack_err)? },
            8 => PngMutation::SetPhysicalDims { phys: diff::read_bin_option(&mut r, diff::read_bin_physical_dims).map_err(op_pack_err)? },
            9 => PngMutation::SetTimestamp { time: diff::read_bin_option(&mut r, diff::read_bin_timestamp).map_err(op_pack_err)? },
            10 => PngMutation::SetBackground { bkgd: diff::read_bin_option(&mut r, diff::read_bin_background).map_err(op_pack_err)? },
            11 => {
                let index = r.read_varint_u64().await.map_err(op_pack_err)? as usize;
                let chunk = diff::read_bin_text_chunk(&mut r).map_err(op_pack_err)?;
                PngMutation::InsertTextChunk { index, chunk }
            }
            12 => PngMutation::RemoveTextChunk { index: r.read_varint_u64().await.map_err(op_pack_err)? as usize },
            13 => {
                let index = r.read_varint_u64().await.map_err(op_pack_err)? as usize;
                let chunk = diff::read_bin_text_chunk(&mut r).map_err(op_pack_err)?;
                PngMutation::SetTextChunk { index, chunk }
            }
            14 => PngMutation::SetPixels { pixels: diff::read_bin_blob(&mut r).map_err(op_pack_err)? },
            15 => {
                let index = r.read_varint_u64().await.map_err(op_pack_err)? as usize;
                let chunk = diff::read_bin_chunk(&mut r).map_err(op_pack_err)?;
                PngMutation::InsertUnknownChunk { index, chunk }
            }
            16 => PngMutation::RemoveUnknownChunk { index: r.read_varint_u64().await.map_err(op_pack_err)? as usize },
            other => {
                return Err(protocol::ProtocolError::Malformed { what: "png op ordinal", offset: 0, detail: format!("unknown ordinal {other}") });
            }
        };
        Ok(mutation)
    }
}
//#endregion 🔖️RealBinaryOpFrame
//#endregion OpCodecs

//#region 🔖️DemoMutationCases
/// 🧪️ P2-P2: shared demo mutation fixtures — `⚙️engine/🦀️component.rs`'s `conformance_laws`
/// module calls `demo_mutation_cases()` directly (`ops_grammar_conformance_law`/
/// `protocol_walk_law`) instead of duplicating the literal case list; `mod tests` below now
/// calls it too (single source of truth, per CLAUDE.md — moved out of `mod tests` verbatim,
/// only the `pub(crate)`/`#[cfg(test)]` visibility changed).
#[cfg(test)]
async fn demo_text_chunk(keyword: &str, value: &str) -> PngTextChunk {
    PngTextChunk { keyword: keyword.into(), value: value.into(), compressed: false, kind: crate::artifacts::png::schema::snapshot::PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() }
}

#[cfg(test)]
pub(crate) async fn demo_base_snapshot() -> PngSnapshot {
    use crate::artifacts::png::schema::snapshot::PngChunkMarker;
    PngSnapshot {
        schema: "stdio.png".into(),
        width: 4,
        height: 4,
        bit_depth: 8,
        color_type: PngColorType::Rgba,
        interlace: false,
        plte: None,
        trns: None,
        gama: None,
        chrm: None,
        srgb: None,
        phys: None,
        time: None,
        bkgd: None,
        text_chunks: vec![demo_text_chunk("Title", "demo")],
        pixels: vec![0u8; 4 * 4 * 4],
        chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Idat, PngChunkMarker::Text { index: 0 }, PngChunkMarker::Iend],
        unknown_chunks: vec![],
    }
}

/// ✅️ Every `PngMutation` variant (incl. two out-of-range no-op cases) built off
/// `demo_base_snapshot()` — the single case list `mutation_diff_law`/`inverse_law`/
/// `op_text_binary_roundtrip_law` (this file) AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`⚙️engine/🦀️component.rs`) all exercise.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<PngMutation> {
    let base = demo_base_snapshot();
    vec![
        PngMutation::NoMutation,
        PngMutation::SetSnapshot {
            snapshot: {
                let mut s = base.clone();
                s.width = 99;
                s
            },
        },
        PngMutation::SetHeader { width: 8, height: 8, bit_depth: 16, color_type: PngColorType::Grayscale, interlace: true },
        PngMutation::SetPalette { plte: Some(vec![PngRgb { r: 1, g: 2, b: 3 }]) },
        PngMutation::SetTransparency { trns: Some(PngTransparency::Grayscale { gray: 7 }) },
        PngMutation::SetGamma { gama: Some(45455) },
        PngMutation::SetChromaticities { chrm: Some(PngChromaticities { white_x: 1, white_y: 2, red_x: 3, red_y: 4, green_x: 5, green_y: 6, blue_x: 7, blue_y: 8 }) },
        PngMutation::SetSrgbIntent { srgb: Some(PngSrgbIntent::Saturation) },
        PngMutation::SetPhysicalDims { phys: Some(PngPhysicalDims { ppu_x: 96, ppu_y: 96, unit_is_meter: false }) },
        PngMutation::SetTimestamp { time: Some(PngTimestamp { year: 2024, month: 6, day: 1, hour: 12, minute: 0, second: 0 }) },
        PngMutation::SetBackground { bkgd: Some(PngBackground::Rgb { r: 1, g: 2, b: 3 }) },
        PngMutation::InsertTextChunk { index: 1, chunk: demo_text_chunk("Comment", "hi") },
        PngMutation::RemoveTextChunk { index: 0 },
        PngMutation::SetTextChunk { index: 0, chunk: demo_text_chunk("Title", "updated") },
        PngMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
        PngMutation::InsertUnknownChunk { index: 1, chunk: PngChunk { kind: *b"zTXt", data: vec![4, 5] } },
        PngMutation::RemoveUnknownChunk { index: 0 },
        // Out-of-range targets: graceful no-ops, still law-compliant.
        PngMutation::RemoveTextChunk { index: 99 },
        PngMutation::RemoveUnknownChunk { index: 99 },
    ]
}
//#endregion 🔖️DemoMutationCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::png::schema::snapshot::{PngChunkMarker, PngTextKind};
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    /// 🔁️ Thin aliases of the module-level `demo_text_chunk`/`demo_base_snapshot` (P2-P2 —
    /// single source of truth, per CLAUDE.md) — kept as short LOCAL names since both are used
    /// pervasively for ad hoc per-test values below (`absorb_law` etc.), not just the mutation
    /// case list.
    async fn text_chunk(keyword: &str, value: &str) -> PngTextChunk {
        demo_text_chunk(keyword, value)
    }

    async fn base_snapshot() -> PngSnapshot {
        demo_base_snapshot()
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ `sweep_a`/`sweep_b` differ in EVERY mutable field. Every index-keyed collection
    /// (`plte`, `text_chunks`, `chunk_order`, `unknown_chunks`) is deliberately DIFFERENT
    /// length (2 vs 1) with the "surviving/modified" item at position 0 and the
    /// "removed-in-forward / added-in-backward" item as the tail at position 1 — the recipe's
    /// own documented workaround for the structural "same-length between() can show removed
    /// XOR added, never both from one call" trap (see `f1-closer-report.md` §4.4).
    async fn sweep_a() -> PngSnapshot {
        PngSnapshot {
            schema: "stdio.png".into(),
            width: 10,
            height: 20,
            bit_depth: 8,
            color_type: PngColorType::Rgba,
            interlace: false,
            plte: Some(vec![PngRgb { r: 1, g: 1, b: 1 }, PngRgb { r: 2, g: 2, b: 2 }]),
            trns: Some(PngTransparency::Grayscale { gray: 5 }),
            gama: Some(45455),
            chrm: Some(PngChromaticities { white_x: 1, white_y: 2, red_x: 3, red_y: 4, green_x: 5, green_y: 6, blue_x: 7, blue_y: 8 }),
            srgb: Some(PngSrgbIntent::Perceptual),
            phys: Some(PngPhysicalDims { ppu_x: 100, ppu_y: 100, unit_is_meter: true }),
            time: Some(PngTimestamp { year: 2020, month: 1, day: 1, hour: 0, minute: 0, second: 0 }),
            bkgd: Some(PngBackground::Grayscale { gray: 255 }),
            text_chunks: vec![text_chunk("Author", "orig"), text_chunk("Trash", "gone")],
            pixels: vec![0u8, 0, 0, 255, 255, 255, 255, 255],
            chunk_order: vec![PngChunkMarker::Gama, PngChunkMarker::Chrm],
            unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![1, 2, 3] }, PngChunk { kind: *b"gone", data: vec![9, 9] }],
        }
    }

    async fn sweep_b() -> PngSnapshot {
        PngSnapshot {
            schema: "stdio.png".into(),
            width: 11,
            height: 21,
            bit_depth: 16,
            color_type: PngColorType::Palette,
            interlace: true,
            plte: Some(vec![PngRgb { r: 9, g: 9, b: 9 }]),
            trns: None,
            gama: None,
            chrm: None,
            srgb: Some(PngSrgbIntent::AbsoluteColorimetric),
            phys: None,
            time: None,
            bkgd: None,
            text_chunks: vec![PngTextChunk { keyword: "Creator".into(), value: "changed".into(), compressed: true, kind: PngTextKind::IText, language_tag: "en".into(), translated_keyword: "Auteur".into() }],
            pixels: vec![1u8, 1, 1, 255],
            chunk_order: vec![PngChunkMarker::Srgb],
            unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![4, 5, 6] }],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    async fn assert_mutation_diff_law(base: &PngSnapshot, mutation: PngMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_png_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_png_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.diff().apply(base).expect("diff must apply to base"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    /// 🔁️ Thin alias of the module-level `demo_mutation_cases()` (P2-P2 — single source of
    /// truth) — kept as a local name taking the SAME `&PngSnapshot` signature every call site
    /// below already uses; `demo_mutation_cases()` builds its own (structurally identical)
    /// base internally, so the passed-in `base` is intentionally unused here.
    async fn all_variants(base: &PngSnapshot) -> Vec<PngMutation> {
        let _ = base;
        demo_mutation_cases()
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            assert_mutation_diff_law(&base, m);
        }
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_png_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_png_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.diff().apply(&base).expect("diff must apply to base");
            let inv_d = d.diff().inverse(&base);
            assert_eq!(inv_d.apply(&mutated).expect("inverse diff must apply to mutated"), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    async fn assert_absorb_law(base: &PngSnapshot, m1: PngMutation, m2: PngMutation) {
        let d1 = m1.diff(base);
        let mid = d1.diff().apply(base).expect("d1 must apply to base");
        let d2 = m2.diff(&mid);
        let sequential = d2.diff().apply(&mid).expect("d2 must apply to mid");

        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(base).expect("merged diff must apply to base"), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before: base has [Title] at 0; insert "New" at 1 -> [Title,New]; then
        // remove index 0 ("Title") -> [New] lands at final index 0 (the recipe's own canonical
        // shift case, on text_chunks' bespoke field-aware absorb path).
        assert_absorb_law(&base, PngMutation::InsertTextChunk { index: 1, chunk: text_chunk("New", "n") }, PngMutation::RemoveTextChunk { index: 0 });

        // Insert+Insert-same-index: both survive, later insert lands at the lower final index.
        assert_absorb_law(&base, PngMutation::InsertTextChunk { index: 1, chunk: text_chunk("F", "f") }, PngMutation::InsertTextChunk { index: 1, chunk: text_chunk("G", "g") });

        // Add+SetField: the second mutation patches directly into the still-pending added chunk.
        assert_absorb_law(&base, PngMutation::InsertTextChunk { index: 0, chunk: text_chunk("X", "orig") }, PngMutation::SetTextChunk { index: 0, chunk: text_chunk("X", "patched") });

        // Modify+Remove: a pending field patch on a since-removed base item vanishes.
        assert_absorb_law(&base, PngMutation::SetTextChunk { index: 0, chunk: text_chunk("Title", "will-vanish") }, PngMutation::RemoveTextChunk { index: 0 });

        // Insert then annihilate the very same insert — on `unknown_chunks`, exercising the
        // SHARED weak-value index transport (`absorb_weak_index_triple`) instead of
        // text_chunks' bespoke field-aware variant.
        assert_absorb_law(&base, PngMutation::InsertUnknownChunk { index: 0, chunk: PngChunk { kind: *b"abcd", data: vec![1] } }, PngMutation::RemoveUnknownChunk { index: 0 });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, PngMutation::SetGamma { gama: Some(1) }, PngMutation::SetGamma { gama: Some(2) });

        // Tri-state set-then-clear: the later clear wins outright over the pending set.
        assert_absorb_law(&base, PngMutation::SetTransparency { trns: Some(PngTransparency::Grayscale { gray: 1 }) }, PngMutation::SetTransparency { trns: None });
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = PngMutation::InsertTextChunk { index: 0, chunk: text_chunk("A", "a") }.diff(&base);
        let s1 = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = PngMutation::SetTextChunk { index: 0, chunk: text_chunk("A", "a2") }.diff(&s1);
        let s2 = d2.diff().apply(&s1).expect("d2 must apply to s1");
        let d3 = PngMutation::RemoveTextChunk { index: 1 }.diff(&s2);
        let s3 = d3.diff().apply(&s2).expect("d3 must apply to s2");

        // (d1∘d2)∘d3
        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).expect("left must apply to base"), s3);
        assert_eq!(right.apply(&base).expect("right must apply to base"), s3);
        assert_eq!(left.apply(&base).expect("left must apply to base"), right.apply(&base).expect("right must apply to base"), "absorb must associate");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.width = 8;
        b.text_chunks.push(text_chunk("Extra", "v"));
        b.pixels = vec![5u8; a.pixels.len()];

        let d = PngDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("d must apply to a"), b, "between(a,b).apply(a) must equal b");
        let d_rev = PngDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b).expect("d_rev must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(PngDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🗿️artifacts/📷️png/📚️examples/🎬️demo/🖼️assets/📷️example.png"));
        let bytes = match bytes {
            Ok(b) if !b.is_empty() => b,
            // No usable fixture on disk at test time (or a different workspace layout) — fall
            // back to a synthetic encode -> decode -> re-encode -> re-decode identity check.
            _ => crate::artifacts::png::engine::encode_png(&base_snapshot()).expect("encode synthetic fallback"),
        };
        let decoded = crate::artifacts::png::engine::decode_png(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::png::engine::encode_png(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::png::engine::decode_png(&reencoded).expect("re-decode fixture");
        // Engine's EncodeScopeNote: encode always canonicalizes to color type 6 / bit depth 8 /
        // interlace 0 — pixel CONTENT is the retained invariant, not the original header/chunks.
        assert_eq!(decoded.width, redecoded.width);
        assert_eq!(decoded.height, redecoded.height);
        assert_eq!(decoded.pixels, redecoded.pixels);
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = PngDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("forward must apply to a"), b, "between(a,b).apply(a) must equal b");
        let backward = PngDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("backward must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(PngDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // IHDR scalars.
        assert!(forward.width.is_some());
        assert!(forward.height.is_some());
        assert!(forward.bit_depth.is_some());
        assert!(forward.color_type.is_some());
        assert!(forward.interlace.is_some());

        // Tri-state clears (forward: Some -> None).
        assert_eq!(forward.trns, Some(None), "trns tri-state clear must show Some(None)");
        assert_eq!(forward.gama, Some(None), "gama tri-state clear must show Some(None)");
        assert_eq!(forward.chrm, Some(None), "chrm tri-state clear must show Some(None)");
        assert_eq!(forward.phys, Some(None), "phys tri-state clear must show Some(None)");
        assert_eq!(forward.time, Some(None), "time tri-state clear must show Some(None)");
        assert_eq!(forward.bkgd, Some(None), "bkgd tri-state clear must show Some(None)");
        assert!(matches!(forward.srgb, Some(Some(_))), "srgb value-only change must stay Some(Some(_))");

        // Tri-state recreates (backward: None -> Some) — the same six fields, other direction.
        assert!(matches!(backward.trns, Some(Some(_))));
        assert!(matches!(backward.gama, Some(Some(_))));
        assert!(matches!(backward.chrm, Some(Some(_))));
        assert!(matches!(backward.phys, Some(Some(_))));
        assert!(matches!(backward.time, Some(Some(_))));
        assert!(matches!(backward.bkgd, Some(Some(_))));

        // plte: forward shows modified+removed, backward shows modified+added (the
        // recipe's split-across-both-directions workaround for the removed-XOR-added trap).
        let plte_fwd = forward.plte.as_ref().expect("plte diff present").as_ref().expect("plte still present");
        assert_eq!(plte_fwd.removed, vec![1]);
        assert_eq!(plte_fwd.modified.len(), 1);
        assert!(plte_fwd.added.is_empty());
        let plte_bwd = backward.plte.as_ref().expect("plte diff present").as_ref().expect("plte still present");
        assert!(plte_bwd.removed.is_empty());
        assert_eq!(plte_bwd.modified.len(), 1);
        assert_eq!(plte_bwd.added.len(), 1);

        // text_chunks: same split; every field of the modified entry's diff populated.
        let tc_fwd = forward.text_chunks.as_ref().expect("text_chunks diff present");
        assert_eq!(tc_fwd.removed, vec![1]);
        assert_eq!(tc_fwd.modified.len(), 1);
        assert!(tc_fwd.added.is_empty());
        let md = &tc_fwd.modified[0].diff;
        assert!(md.keyword.is_some(), "keyword must be diffed");
        assert!(md.value.is_some(), "value must be diffed");
        assert!(md.compressed.is_some(), "compressed must be diffed");
        assert!(md.kind.is_some(), "kind must be diffed");
        assert!(md.language_tag.is_some(), "language_tag must be diffed");
        assert!(md.translated_keyword.is_some(), "translated_keyword must be diffed");
        let tc_bwd = backward.text_chunks.as_ref().expect("text_chunks diff present");
        assert!(tc_bwd.removed.is_empty());
        assert_eq!(tc_bwd.modified.len(), 1);
        assert_eq!(tc_bwd.added.len(), 1);

        // pixels.
        assert!(forward.pixels.is_some(), "pixels must be diffed");

        // chunk_order: same split.
        let co_fwd = forward.chunk_order.as_ref().expect("chunk_order diff present");
        assert_eq!(co_fwd.removed, vec![1]);
        assert_eq!(co_fwd.modified.len(), 1);
        assert!(co_fwd.added.is_empty());
        let co_bwd = backward.chunk_order.as_ref().expect("chunk_order diff present");
        assert!(co_bwd.removed.is_empty());
        assert_eq!(co_bwd.modified.len(), 1);
        assert_eq!(co_bwd.added.len(), 1);

        // unknown_chunks: same split.
        let uc_fwd = forward.unknown_chunks.as_ref().expect("unknown_chunks diff present");
        assert_eq!(uc_fwd.removed, vec![1]);
        assert_eq!(uc_fwd.modified.len(), 1);
        assert!(uc_fwd.added.is_empty());
        let uc_bwd = backward.unknown_chunks.as_ref().expect("unknown_chunks diff present");
        assert!(uc_bwd.removed.is_empty());
        assert_eq!(uc_bwd.modified.len(), 1);
        assert_eq!(uc_bwd.added.len(), 1);
    }
    //#endregion 🔖️field_sweep

    #[semio_framework_async_macros::async_test]
    async fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_png_mutation(&mut snap, &PngMutation::RemoveTextChunk { index: 42 });
        assert_eq!(snap, base);
        apply_png_mutation(&mut snap, &PngMutation::RemoveUnknownChunk { index: 42 });
        assert_eq!(snap, base);
        apply_png_mutation(&mut snap, &PngMutation::SetTextChunk { index: 42, chunk: text_chunk("x", "y") });
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PngMutation` grammar —
    /// exercises every variant via `all_variants` (incl. every ancillary Setter's `Some(_)`
    /// payload) plus two extra `SetSnapshot` cases (`sweep_a`/`sweep_b`) so the whole-snapshot
    /// positional codec's `Some` AND `None` branches for every one of its 8 optional fields, plus
    /// its `text_chunks`/`chunk_order`/`unknown_chunks` lists, both get covered.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mut mutations = all_variants(&base);
        mutations.push(PngMutation::SetSnapshot { snapshot: sweep_a() });
        mutations.push(PngMutation::SetSnapshot { snapshot: sweep_b() });
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = PngMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = PngMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
