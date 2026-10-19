//! 🖼️ XObjects of the PDF document model: the image bodies and masks, images, transparency groups and form XObjects. See [`super`].

use super::colour_and_graphics_state::PdfColorSpace;
use super::content_operators::PdfOp;
use super::object_model::{PdfDictEntry, PdfMatrix, PdfRect, PDF_IDENTITY_MATRIX};

//#region 🔖️XObjects
/// 🖼️ A logical image body is explicit component samples or an admitted foreign image.
#[derive(semio_framework_value::RetireOwned, Clone,Debug,PartialEq,Eq,value_derive::ToValue,value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
pub enum PdfImageBody {
    Samples { values:Vec<u32> },
    Artifact { reference:semio_framework_artifact_reference::ArtifactRef },
}

/// 🎭 An image's explicit mask (§8.9.6): a stencil image id or colour-key ranges.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfImageMask {
    Stencil { image: String },
    ColorKey { ranges: Vec<u32> },
}

/// 🖼️ An image XObject (§8.9.5, Table 89), keyed by `id`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfImage {
    pub id: String,
    pub width: u32,
    pub height: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<PdfColorSpace>,
    #[value(default)]
    pub bits_per_component: u32,
    #[value(default)]
    pub image_mask: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub decode: Vec<f64>,
    #[value(default)]
    pub interpolate: bool,
    pub body:PdfImageBody,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub soft_mask: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub soft_mask_in_data: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<PdfImageMask>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub matte: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parent: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfImage {
    /// 🖼️ Creates explicit 8-bit RGB component samples in row order.
    pub fn rgb8(id:impl Into<String>,width:u32,height:u32,values:Vec<u8>)->Self {Self::samples(id,width,height,PdfColorSpace::DeviceRgb,8,values.into_iter().map(u32::from).collect())}
    /// 🖼️ Creates explicit 8-bit grayscale component samples in row order.
    pub fn gray8(id:impl Into<String>,width:u32,height:u32,values:Vec<u8>)->Self {Self::samples(id,width,height,PdfColorSpace::DeviceGray,8,values.into_iter().map(u32::from).collect())}
    /// 🖼️ Creates an image from logical samples rather than a packed representation.
    pub fn samples(id:impl Into<String>,width:u32,height:u32,color_space:PdfColorSpace,bits_per_component:u32,values:Vec<u32>)->Self {
        Self {id:id.into(),width,height,color_space:Some(color_space),bits_per_component,image_mask:false,decode:Vec::new(),interpolate:false,body:PdfImageBody::Samples {values},soft_mask:None,soft_mask_in_data:None,mask:None,matte:None,intent:None,optional_content:None,struct_parent:None,extra:Vec::new()}
    }
}

/// 🫧 A transparency group attribute dictionary (§11.6.6).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfTransparencyGroup {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<PdfColorSpace>,
    #[value(default)]
    pub isolated: bool,
    #[value(default)]
    pub knockout: bool,
}

/// 📄 A form XObject (§8.10), keyed by `id`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfFormXObject {
    pub id: String,
    pub bbox: PdfRect,
    #[value(default = "PdfFormXObject::identity")]
    pub matrix: PdfMatrix,
    #[value(default)]
    pub content: Vec<PdfOp>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfTransparencyGroup>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parent: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfFormXObject {
    pub(super) fn identity() -> PdfMatrix {
        PDF_IDENTITY_MATRIX
    }
    pub fn new(id: impl Into<String>, bbox: PdfRect, content: Vec<PdfOp>) -> Self {
        Self { id: id.into(), bbox, matrix: PDF_IDENTITY_MATRIX, content, group: None, optional_content: None, struct_parent: None, extra: Vec::new() }
    }
}
//#endregion 🔖️XObjects
