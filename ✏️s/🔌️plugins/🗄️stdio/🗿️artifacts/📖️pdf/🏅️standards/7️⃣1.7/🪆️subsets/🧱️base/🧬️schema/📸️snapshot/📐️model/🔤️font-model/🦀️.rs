//! 🔤️ Fonts of the PDF document model: the encodings, descriptors, programs, ToUnicode maps, CMaps, CID fonts and the font kinds. See [`super`].

use super::content_operators::PdfOp;
use super::object_model::{PdfDictEntry, PdfMatrix, PdfRect};

//#region 🔖️Fonts
/// 🔡 A predefined simple-font base encoding (§9.6.6, Annex D).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfBaseEncoding {
    Standard,
    WinAnsi,
    MacRoman,
    MacExpert,
}

/// 🔡 One `/Differences` entry: `code` shows the glyph named `glyph`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfEncodingDifference {
    pub code: u32,
    pub glyph: String,
}

/// 🔡 A simple font's encoding: an optional base encoding (`None` = the font's built-in encoding)
/// plus `/Differences` (§9.6.6.1).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfSimpleEncoding {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<PdfBaseEncoding>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub differences: Vec<PdfEncodingDifference>,
}

impl PdfSimpleEncoding {
    pub fn win_ansi() -> Self {
        Self { base: Some(PdfBaseEncoding::WinAnsi), differences: Vec::new() }
    }
}

/// 📏 A font descriptor (§9.8, Table 122). Widths live on the font itself.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfFontDescriptor {
    pub font_name: String,
    #[value(default)]
    pub flags: u32,
    #[value(default)]
    pub font_bbox: PdfRect,
    #[value(default)]
    pub italic_angle: f64,
    #[value(default)]
    pub ascent: f64,
    #[value(default)]
    pub descent: f64,
    #[value(default)]
    pub cap_height: f64,
    #[value(default)]
    pub stem_v: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stem_h: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x_height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub leading: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub avg_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub missing_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_stretch: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub char_set: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 💾 An embedded font program (§9.9, Table 126).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFontProgram {
    Type1 { reference: semio_framework_artifact_reference::ArtifactRef },
    TrueType { reference: semio_framework_artifact_reference::ArtifactRef },
    Cff { reference: semio_framework_artifact_reference::ArtifactRef },
    CidCff { reference: semio_framework_artifact_reference::ArtifactRef },
    OpenType { reference: semio_framework_artifact_reference::ArtifactRef },
}

impl PdfFontProgram {
    /// 🔗️ Borrows the admitted foreign program identity without its physical body.
    pub fn reference(&self)->&semio_framework_artifact_reference::ArtifactRef {match self {Self::Type1 {reference}|Self::TrueType {reference}|Self::Cff {reference}|Self::CidCff {reference}|Self::OpenType {reference}=>reference}}
}

/// 🈴 One `ToUnicode` mapping (§9.10.3): a character code (of `byte_width` bytes) to a Unicode
/// string, or a contiguous range whose destinations increment from `text`'s last code point.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfToUnicodeMapping {
    Char { code: u32, text: String },
    Range { low: u32, high: u32, text: String },
}

/// 🈴 A `ToUnicode` CMap in typed form.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfToUnicode {
    pub byte_width: u32,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<PdfToUnicodeMapping>,
}

/// 🗺️ One code-space range of a CMap (§9.7.5.2).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCodespaceRange {
    pub byte_width: u32,
    pub low: u32,
    pub high: u32,
}

/// 🗺️ One `cidchar`/`cidrange` entry of an embedded CMap.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfCidMapping {
    Char { code: u32, cid: u32 },
    Range { low: u32, high: u32, cid: u32 },
}

/// 🗺️ An embedded CMap stream (§9.7.5.3) in typed form.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfEmbeddedCMap {
    pub name: String,
    #[value(default)]
    pub vertical: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub codespace: Vec<PdfCodespaceRange>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<PdfCidMapping>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub use_cmap: Option<String>,
}

/// 🗺️ The `/Encoding` of a Type 0 font (§9.7.5): a predefined CMap by name, or an embedded one.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfCMap {
    Predefined { name: String },
    Embedded { cmap: PdfEmbeddedCMap },
}

impl PdfCMap {
    pub fn identity_h() -> Self {
        Self::Predefined { name: "Identity-H".into() }
    }
}

/// 📇 `/CIDSystemInfo` (§9.7.3).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCidSystemInfo {
    pub registry: String,
    pub ordering: String,
    pub supplement: u32,
}

impl Default for PdfCidSystemInfo {
    fn default() -> Self {
        Self { registry: "Adobe".into(), ordering: "Identity".into(), supplement: 0 }
    }
}

/// 📏 One run of a CIDFont `/W` array: consecutive widths from `start_cid` (§9.7.4.3).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCidWidthRun {
    pub start_cid: u32,
    pub widths: Vec<f64>,
}

/// 📏 One `/W2` vertical-metrics run (§9.7.4.3): per-CID `[w1y vx vy]` triples.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCidVerticalRun {
    pub start_cid: u32,
    pub metrics: Vec<[f64; 3]>,
}

/// 🔢 `/CIDToGIDMap` (§9.7.4.2).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfCidToGid {
    Identity,
    Map { glyphs: Vec<u16> },
}

/// 🔤 A CIDFont (§9.7.4), the descendant of a Type 0 font.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCidFont {
    pub true_type: bool,
    pub base_font: String,
    #[value(default)]
    pub system_info: PdfCidSystemInfo,
    pub descriptor: PdfFontDescriptor,
    #[value(default = "PdfCidFont::default_width")]
    pub default_width: f64,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub widths: Vec<PdfCidWidthRun>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_vertical: Option<[f64; 2]>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub vertical_metrics: Vec<PdfCidVerticalRun>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cid_to_gid: Option<PdfCidToGid>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub program: Option<PdfFontProgram>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfCidFont {
    fn default_width() -> f64 {
        1000.0
    }
}

/// 🔠 One Type 3 glyph procedure (§9.6.5): a content stream drawing the glyph named `name`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfCharProc {
    pub name: String,
    pub content: Vec<PdfOp>,
}

/// 🔤 A font's subtype-specific data (§9.5–9.7). The standard 14 fonts are `Type1` fonts with
/// no program and no descriptor; their metrics come from the writer's built-in AFM tables.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFontKind {
    Type1 { base_font: String, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, descriptor: Option<PdfFontDescriptor>, program: Option<PdfFontProgram> },
    TrueType { base_font: String, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, descriptor: Option<PdfFontDescriptor>, program: Option<PdfFontProgram> },
    Type3 { font_matrix: PdfMatrix, font_bbox: PdfRect, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, char_procs: Vec<PdfCharProc>, descriptor: Option<PdfFontDescriptor> },
    Type0 { base_font: String, cmap: PdfCMap, descendant: PdfCidFont },
}

/// 🔤 A document font, keyed by `id` — the resource name content operators (`Tf`) use.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfFont {
    pub id: String,
    pub kind: PdfFontKind,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub to_unicode: Option<PdfToUnicode>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfFont {
    /// 🅰️ One of the standard 14 Type 1 fonts (§9.6.2.2) with WinAnsi encoding (or the built-in
    /// encoding for Symbol/ZapfDingbats), no descriptor and no program.
    pub fn standard(id: impl Into<String>, base_font: &str) -> Self {
        let symbolic = matches!(base_font, "Symbol" | "ZapfDingbats");
        Self { id: id.into(), kind: PdfFontKind::Type1 { base_font: base_font.into(), encoding: if symbolic { PdfSimpleEncoding::default() } else { PdfSimpleEncoding::win_ansi() }, first_char: 0, widths: Vec::new(), descriptor: None, program: None }, to_unicode: None, extra: Vec::new() }
    }
    pub fn base_font(&self) -> &str {
        match &self.kind {
            PdfFontKind::Type1 { base_font, .. } | PdfFontKind::TrueType { base_font, .. } | PdfFontKind::Type0 { base_font, .. } => base_font,
            PdfFontKind::Type3 { .. } => "",
        }
    }
    /// 🔢 Whether character codes of this font are two bytes wide (Type 0 with a two-byte CMap).
    pub fn is_composite(&self) -> bool {
        matches!(self.kind, PdfFontKind::Type0 { .. })
    }
}
//#endregion 🔖️Fonts
