//! 🌈️ Colour and graphics state of the PDF document model: the colour spaces, functions, shadings, patterns, soft masks, extended graphics states and the named colour space and property resources. See [`super`].

use super::content_operators::{PdfLineCap, PdfLineJoin, PdfOp};
use super::object_model::{PdfDictEntry, PdfMatrix, PdfRect, PDF_IDENTITY_MATRIX};

//#region 🔖️Colour
/// 🌈 A colour space (§8.6). `Named` refers to an entry of the current resource dictionary's
/// `/ColorSpace` sub-dictionary and only appears where the spec allows a name.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", retire_with="retire_pdf_color_space")]
pub enum PdfColorSpace {
    DeviceGray,
    DeviceRgb,
    DeviceCmyk,
    CalGray { white_point: [f64; 3], black_point: Option<[f64; 3]>, gamma: Option<f64> },
    CalRgb { white_point: [f64; 3], black_point: Option<[f64; 3]>, gamma: Option<[f64; 3]>, matrix: Option<[f64; 9]> },
    Lab { white_point: [f64; 3], black_point: Option<[f64; 3]>, range: Option<[f64; 4]> },
    IccBased { components: u32, profile: semio_framework_artifact_reference::ArtifactRef, alternate: Option<Box<PdfColorSpace>>, range: Option<Vec<f64>> },
    Indexed { base: Box<PdfColorSpace>, hival: u32, palette: Vec<u8> },
    Separation { name: String, alternate: Box<PdfColorSpace>, tint_transform: PdfFunction },
    DeviceN { names: Vec<String>, alternate: Box<PdfColorSpace>, tint_transform: PdfFunction, attributes: Option<Vec<PdfDictEntry>> },
    Pattern { base: Option<Box<PdfColorSpace>> },
    Named { name: String },
}

/// ♻️ Retires recursive colour-space children through their actual value owners.
fn retire_pdf_color_space(value: PdfColorSpace) {
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            PdfColorSpace::IccBased { alternate, .. } | PdfColorSpace::Pattern { base: alternate } => {
                if let Some(child) = alternate { pending.push(*child); }
            }
            PdfColorSpace::Indexed { base, .. } => pending.push(*base),
            PdfColorSpace::Separation { alternate, tint_transform, .. } => {
                pending.push(*alternate);
                <PdfFunction as semio_framework_value::FromValue>::retire_decoded(tint_transform);
            }
            PdfColorSpace::DeviceN { alternate, tint_transform, attributes, .. } => {
                pending.push(*alternate);
                <PdfFunction as semio_framework_value::FromValue>::retire_decoded(tint_transform);
                <Option<Vec<PdfDictEntry>> as semio_framework_value::FromValue>::retire_decoded(attributes);
            }
            leaf => drop(leaf),
        }
    }
}

impl PdfColorSpace {
    /// 🔢️ Number of components a colour in this space carries (`None` for `Pattern`/`Named`).
    pub fn components(&self) -> Option<u32> {
        Some(match self {
            PdfColorSpace::DeviceGray | PdfColorSpace::CalGray { .. } | PdfColorSpace::Indexed { .. } | PdfColorSpace::Separation { .. } => 1,
            PdfColorSpace::DeviceRgb | PdfColorSpace::CalRgb { .. } | PdfColorSpace::Lab { .. } => 3,
            PdfColorSpace::DeviceCmyk => 4,
            PdfColorSpace::IccBased { components, .. } => *components,
            PdfColorSpace::DeviceN { names, .. } => names.len() as u32,
            PdfColorSpace::Pattern { .. } | PdfColorSpace::Named { .. } => return None,
        })
    }
}

/// 🧮 A PDF function (§7.10): sampled (type 0), exponential (2), stitching (3), PostScript
/// calculator (4, retained as its source text), or an array of 1-out functions.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", retire_with="retire_pdf_function")]
pub enum PdfFunction {
    Sampled { domain: Vec<f64>, range: Vec<f64>, size: Vec<u32>, bits_per_sample: u32, order: Option<u32>, encode: Option<Vec<f64>>, decode: Option<Vec<f64>>, samples: Vec<u32> },
    Exponential { domain: Vec<f64>, range: Option<Vec<f64>>, c0: Vec<f64>, c1: Vec<f64>, n: f64 },
    Stitching { domain: Vec<f64>, range: Option<Vec<f64>>, functions: Vec<PdfFunction>, bounds: Vec<f64>, encode: Vec<f64> },
    PostScript { domain: Vec<f64>, range: Vec<f64>, code: String },
    Array { functions: Vec<PdfFunction> },
}

/// ♻️ Retires function forests without recursive native destruction.
fn retire_pdf_function(value: PdfFunction) {
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            PdfFunction::Stitching { functions, .. } | PdfFunction::Array { functions } => pending.extend(functions),
            leaf => drop(leaf),
        }
    }
}

/// 🌅 A shading (§8.7.4.5). Mesh types 4–7 keep their packed vertex data with the decode
/// parameters needed to read it.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfShadingKind {
    FunctionBased { domain: Option<[f64; 4]>, matrix: Option<PdfMatrix>, function: PdfFunction },
    Axial { coords: [f64; 4], domain: Option<[f64; 2]>, function: PdfFunction, extend: [bool; 2] },
    Radial { coords: [f64; 6], domain: Option<[f64; 2]>, function: PdfFunction, extend: [bool; 2] },
    Mesh { shading_type: u32, bits_per_coordinate: u32, bits_per_component: u32, bits_per_flag: Option<u32>, vertices_per_row: Option<u32>, decode: Vec<f64>, function: Option<PdfFunction>, reference: semio_framework_artifact_reference::ArtifactRef },
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfShading {
    pub id: String,
    pub color_space: PdfColorSpace,
    pub kind: PdfShadingKind,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bbox: Option<PdfRect>,
    #[value(default)]
    pub anti_alias: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 🧩 A pattern (§8.7.3): tiling patterns paint a content cell, shading patterns reference a
/// shading.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfPatternKind {
    Tiling { paint_type: u32, tiling_type: u32, bbox: PdfRect, x_step: f64, y_step: f64, content: Vec<PdfOp> },
    Shading { shading: String, ext_g_state: Option<String> },
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfPattern {
    pub id: String,
    #[value(default = "PdfPattern::identity")]
    pub matrix: PdfMatrix,
    pub kind: PdfPatternKind,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfPattern {
    pub(super) fn identity() -> PdfMatrix {
        PDF_IDENTITY_MATRIX
    }
}
//#endregion 🔖️Colour

//#region 🔖️GraphicsState
/// 🫥 The `/SMask` entry of an extended graphics state (§11.6.5): `None` (the name `/None`,
/// which switches soft masking off), or an alpha/luminosity mask drawn from the form XObject
/// `group`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfSoftMask {
    None,
    Alpha { group: String, transfer: Option<PdfFunction> },
    Luminosity { group: String, backdrop: Option<Vec<f64>>, transfer: Option<PdfFunction> },
}

/// 🎛️ An extended graphics state parameter dictionary (§8.4.5, Table 58). Every entry is optional;
/// absent means "leave as is".
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfExtGState {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<PdfLineCap>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_join: Option<PdfLineJoin>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub miter_limit: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dash: Option<(Vec<f64>, f64)>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rendering_intent: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overprint_stroke: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overprint_fill: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overprint_mode: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<(String, f64)>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub soft_mask: Option<PdfSoftMask>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_alpha: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill_alpha: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha_is_shape: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_adjust: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flatness: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub smoothness: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text_knockout: Option<bool>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️GraphicsState

//#region 🔖️Resources
/// 🌈 One `name → colour space` entry of the document's `/ColorSpace` resources. Content
/// operators (`cs`/`CS`) reference the name; every page and form binds what it uses on write.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfNamedColorSpace {
    pub name: String,
    pub color_space: PdfColorSpace,
}

/// 🏷️ One `name → property list` entry of the document's `/Properties` resources (§14.6.2),
/// referenced by `DP`/`BDC` operators.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfNamedProperties {
    pub name: String,
    pub entries: Vec<PdfDictEntry>,
}
//#endregion 🔖️Resources
