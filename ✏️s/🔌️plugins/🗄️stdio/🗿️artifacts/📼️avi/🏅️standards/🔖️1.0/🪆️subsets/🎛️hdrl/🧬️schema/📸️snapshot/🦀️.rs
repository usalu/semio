//! 🧬️ AviSnapshot — RIFF/AVI 1.0: `avih` (MainAVIHeader) typed, per-stream `strh` typed + `strf`
//! discriminated by `fccType` (`BitmapInfo` for `vids`, `WaveFormat` for `auds`, `Raw` otherwise),
//! `movi` chunks assigned to their owning stream with `idx1`-derived keyframe flags, everything
//! else (non-`hdrl`/`movi`/`idx1` top-level RIFF children) typed-raw retained (`unknown_chunks`).
//! Nested auxiliary children real encoders also write are retained the same typed-raw way, one
//! level down: `hdrl`'s own non-`avih`/`strl` children (e.g. `JUNK` padding) in `hdrl_extra`, and
//! each `strl`'s non-`strh`/`strf` children (e.g. `vprp`, `JUNK`) in that stream's `strl_extra` —
//! both real, both present in ffmpeg's own AVI-1.0 output, neither addressable by a dedicated
//! mutation kind (see `AviMutation`'s module doc comment for why).
//! Snapshot DSL/pack retain the complete owned model; native RIFF serialization belongs to I/O.

use framework_schema::ArtifactSchema;





//#region 🔖️Ids
pub const STDIO_AVI_DOCUMENT_SCHEMA: &str = "stdio.avi";
//#endregion 🔖️Ids

//#region 🔖️MainHeader
/// 🏷️ `avih` — MainAVIHeader, all 14 DWORDs typed (56 bytes). <https://learn.microsoft.com/🪟️windows/win32/directshow/avimainheader>
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct AviMainHeader {
    pub micro_sec_per_frame: u32,
    pub max_bytes_per_sec: u32,
    pub padding_granularity: u32,
    pub flags: u32,
    pub total_frames: u32,
    pub initial_frames: u32,
    pub streams: u32,
    pub suggested_buffer_size: u32,
    pub width: u32,
    pub height: u32,
    /// 🕳️ `dwReserved[4]` — verbatim, never fabricated.
    #[value(default)]
    pub reserved: Vec<u32>,
}
//#endregion 🔖️MainHeader

//#region 🔖️StreamHeader
/// 🏷️ `strh` — AVISTREAMHEADER. The 13 DWORD/WORD fields up to `dwSampleSize` (48 bytes) are fixed;
/// the trailing `rcFrame` rectangle is NOT: real encoders (ffmpeg's own AVI-1.0 muxer included)
/// still write the classic pre-Win32 form with `rcFrame` as 4 16-bit `SHORT`s (56 bytes total), not
/// only the modern 4 `LONG`s form (64 bytes) most docs describe. <https://learn.microsoft.com/🪟️windows/win32/directshow/avistreamheader>
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct AviStreamHeader {
    pub fcc_type: String,
    pub fcc_handler: String,
    pub flags: u32,
    pub priority: u16,
    pub language: u16,
    pub initial_frames: u32,
    pub scale: u32,
    pub rate: u32,
    pub start: u32,
    pub length: u32,
    pub suggested_buffer_size: u32,
    pub quality: i32,
    pub sample_size: u32,
    /// 🖼️ `rcFrame`, always widened to `i32` regardless of the wire width it was read at.
    pub rc_frame_left: i32,
    pub rc_frame_top: i32,
    pub rc_frame_right: i32,
    pub rc_frame_bottom: i32,
    /// 📏 The wire width `encode_avi` re-serializes `rcFrame` as — `0` (omitted; a bare 48-byte
    /// `strh`), `8` (4 `SHORT`s; the classic 56-byte form), or `16` (4 `LONG`s; the modern 64-byte
    /// form). `decode_avi` records whichever width the source actually used so a real 56-byte
    /// `strh` round-trips byte-for-byte instead of being silently promoted to 64 bytes. Hand-built
    /// headers default to `16`, the complete/preferred form.
    #[value(default = "default_rc_frame_width")]
    pub rc_frame_width: u8,
    /// 📎 Any bytes beyond the documented 64-byte `AVISTREAMHEADER`, verbatim — rare, retained so
    /// an unusually padded real `strh` round-trips losslessly rather than being silently truncated.
    #[value(default)]
    pub strh_extra: Vec<u8>,
}

impl Default for AviStreamHeader {
    fn default() -> Self {
        Self {
            fcc_type: String::new(),
            fcc_handler: String::new(),
            flags: 0,
            priority: 0,
            language: 0,
            initial_frames: 0,
            scale: 0,
            rate: 0,
            start: 0,
            length: 0,
            suggested_buffer_size: 0,
            quality: 0,
            sample_size: 0,
            rc_frame_left: 0,
            rc_frame_top: 0,
            rc_frame_right: 0,
            rc_frame_bottom: 0,
            rc_frame_width: 16,
            strh_extra: Vec::new(),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_rc_frame_width() -> u8 {
    16
}
//#endregion 🔖️StreamHeader

//#region 🔖️StreamFormat
/// 🎨️ `strf`, discriminated by the owning stream's `fccType`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_dsl_record_derive::DslEnum)]
#[value(tag = "format", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum AviStreamFormat {
    /// 🖼️ `BITMAPINFOHEADER` (40 bytes; `vids`). <https://learn.microsoft.com/🪟️windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader>
    #[dsl(keyword="bitmapInfo")]
    BitmapInfo { size: u32, width: i32, height: i32, planes: u16, bit_count: u16, compression: String, size_image: u32, x_pels_per_meter: i32, y_pels_per_meter: i32, colors_used: u32, colors_important: u32 },
    /// 🔊️ `WAVEFORMATEX`-shaped (`auds`).
    #[dsl(keyword="waveFormat")]
    WaveFormat {
        format_tag: u16,
        channels: u16,
        samples_per_sec: u32,
        avg_bytes_per_sec: u32,
        block_align: u16,
        bits_per_sample: u16,
        #[value(default)]
        extra: Vec<u8>,
    },
    /// 📦 Any other `fccType` — verbatim `strf` payload bytes.
    #[dsl(keyword="raw")]
    Raw { data: Vec<u8> },
}

impl semio_framework_dsl_record::DslField for AviStreamFormat{
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{control.step()?;let mut statements=control.allocate_vec(1)?;statements.push(<Self as semio_framework_dsl_record::DslVariants>::to_named_record_controlled(self,control)?);Ok(semio_framework_dsl_record::FieldValue::Statements(statements))}

    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        control.step()?;
        match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let(keyword,record)=&values[0];<Self as semio_framework_dsl_record::DslVariants>::from_named_record_controlled(keyword,record,control)},_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"AVI stream format requires exactly one typed choice"))}
    }
    fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants())}
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control).map(semio_framework_dsl_record::Shape::Statements)}
    fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Statements(vec![<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self)])}
    fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let(keyword,record)=&values[0];<Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword,record).map_err(|error|error.message)},_=>Err("AVI stream format requires exactly one typed choice".into())}}
}

impl Default for AviStreamFormat {
    fn default() -> Self {
        Self::Raw { data: Vec::new() }
    }
}
//#endregion 🔖️StreamFormat

//#region 🔖️Chunk
/// 🎞️ One `movi` chunk belonging to this stream — fourcc (e.g. `"00dc"`), payload bytes, and
/// whether `idx1` (or the no-`idx1` fallback, per spec: absent index ⇒ every scanned chunk is
/// treated as a sync point) marks it a keyframe.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct AviChunk {
    pub fourcc: String,
    #[value(default)]
    pub data: Vec<u8>,
    pub keyframe: bool,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct AviStream {
    pub strh: AviStreamHeader,
    #[dsl(block)]
    pub strf: AviStreamFormat,
    #[value(default)]
    pub chunks: Vec<AviChunk>,
    /// 📦️ Typed-raw retention for this stream's `strl` children besides `strh`/`strf` (e.g. a
    /// `vprp` video-properties chunk, `JUNK` padding) — verbatim fourcc + payload, replayed after
    /// `strh`/`strf` on encode. Real ffmpeg AVI-1.0 output carries both of these inside `strl`.
    #[value(default)]
    pub strl_extra: Vec<RiffChunk>,
}
//#endregion 🔖️Chunk

//#region 🔖️RawChunk
/// 📦️ Typed-raw retention for a top-level RIFF child this codec doesn't otherwise type (any
/// entry inside `AVI `'s body besides `hdrl`/`movi`/`idx1`) — verbatim fourcc + payload, replayed
/// at the same relative position (after `idx1`) on encode.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RiffChunk {
    pub fourcc: String,
    #[value(default)]
    pub data: Vec<u8>,
}
//#endregion 🔖️RawChunk

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(lines)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi")]
pub struct AviSnapshot {
    #[state(artifact)]
    #[value(default = "default_schema")]
    pub schema: String,
    #[state(artifact)]
    pub main_header: AviMainHeader,
    #[state(artifact)]
    #[value(default)]
    pub streams: Vec<AviStream>,
    #[state(artifact)]
    pub idx1_present: bool,
    #[state(artifact)]
    #[value(default)]
    pub unknown_chunks: Vec<RiffChunk>,
    /// 📦️ Typed-raw retention for `hdrl` children besides `avih`/`strl` (e.g. `JUNK` padding
    /// directly inside `hdrl`) — verbatim fourcc + payload, replayed after every `strl` on encode.
    #[state(artifact)]
    #[value(default)]
    pub hdrl_extra: Vec<RiffChunk>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_schema() -> String {
    STDIO_AVI_DOCUMENT_SCHEMA.into()
}

/// 🆕️ A new avi document: `stdio.avi`, and a main header whose `dwReserved[4]` are the four zero DWORDs every written
/// `avih` carries — the empty `reserved` list saved as four zeros and reopened as a different document.
impl Default for AviSnapshot {
    fn default() -> Self {
        Self { schema: default_schema(), main_header: AviMainHeader { reserved: vec![0; 4], ..AviMainHeader::default() }, streams: Vec::new(), idx1_present: false, unknown_chunks: Vec::new(), hdrl_extra: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
