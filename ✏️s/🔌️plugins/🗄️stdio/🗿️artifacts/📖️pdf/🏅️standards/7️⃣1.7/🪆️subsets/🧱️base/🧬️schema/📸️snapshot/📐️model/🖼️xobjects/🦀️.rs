//! 🖼️ XObjects of the PDF document model: the image bodies and masks, images, transparency groups and form XObjects. See [`super`].

use super::colour_and_graphics_state::PdfColorSpace;
use super::content_operators::PdfOp;
use super::object_model::{PdfDictEntry, PdfMatrix, PdfRect, PDF_IDENTITY_MATRIX};

//#region 🔖️XObjects
/// 🖼️ An image body is its native packed rows (§8.9.3) or an admitted foreign image.
///
/// `packed` holds height rows of width × components samples at the image's bit depth, most
/// significant bit first, every row padded to a byte boundary — exactly a decoded image stream,
/// so the body costs what the file costs. Read logical samples through [`PdfPackedSamples`].
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone,Debug,PartialEq,Eq,value_derive::ToValue,value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
pub enum PdfImageBody {
    Samples { #[value(with="pack::value::bytes", serialize_controlled_with="pack::value::bytes::to_value_controlled", deserialize_controlled_with="pack::value::bytes::from_value_controlled", retire_with="std::mem::drop")] packed:Vec<u8> },
    Artifact { reference:semio_framework_artifact_reference::ArtifactRef },
}

/// 📏️ The samples per row and packed byte length of an image grid (§8.9.3).
pub fn packed_sample_geometry(width:u32,height:u32,components:u32,bits:u32)->Result<(usize,usize),&'static str> {
    if !matches!(bits,1|2|4|8|16)||components==0 {return Err("image sample depth or components are invalid");}
    let count=(width as usize).checked_mul(components as usize).ok_or("image row sample count overflow")?;
    let row=count.checked_mul(bits as usize).ok_or("image row width overflow")?.div_ceil(8);
    let length=row.checked_mul(height as usize).filter(|length|*length<=isize::MAX as usize).ok_or("image body width overflow")?;
    Ok((count,length))
}

/// 🧮 A read view of packed native rows: logical sample `index` in row order, without unpacking.
#[derive(Clone,Copy,Debug)]
pub struct PdfPackedSamples<'a> {
    packed:&'a [u8],
    bits:usize,
    row_samples:usize,
    row_bytes:usize,
    count:usize,
}

impl<'a> PdfPackedSamples<'a> {
    /// 🔍️ Views `packed` as rows of `width × components` samples of `bits` bits; refuses a body whose length differs from its geometry.
    pub fn new(packed:&'a [u8],width:u32,height:u32,components:u32,bits:u32)->Result<Self,&'static str> {
        let (row_samples,length)=packed_sample_geometry(width,height,components,bits)?;
        if packed.len()!=length {return Err("image packed body does not match its declared geometry");}
        Ok(Self {packed,bits:bits as usize,row_samples,row_bytes:length.checked_div(height as usize).unwrap_or(0),count:row_samples*height as usize})
    }
    /// 🔢️ The number of logical samples.
    pub fn len(&self)->usize {self.count}
    /// 🫙️ Whether the grid holds no sample.
    pub fn is_empty(&self)->bool {self.count==0}
    /// 📍️ The logical sample at `index` in row order.
    pub fn get(&self,index:usize)->Option<u32> {
        if index>=self.count {return None;}
        let bit=(index%self.row_samples)*self.bits;
        let byte=(index/self.row_samples)*self.row_bytes+bit/8;
        Some(match self.bits {16=>u32::from(u16::from_be_bytes([self.packed[byte],self.packed[byte+1]])),8=>u32::from(self.packed[byte]),bits=>u32::from((self.packed[byte]>>(8-bit%8-bits))&((1u8<<bits)-1))})
    }
    /// 🔁️ Every logical sample in row order.
    pub fn iter(&self)->impl Iterator<Item=u32>+'a {let view=*self;(0..view.count).map(move|index|view.get(index).expect("sample inside its grid"))}
    /// 📦️ The packed rows.
    pub fn packed(&self)->&'a [u8] {self.packed}
}

/// 🛫️ Packs logical samples into native rows; refuses samples that do not fit the grid or the depth.
pub fn pack_samples(width:u32,height:u32,components:u32,bits:u32,values:&[u32])->Result<Vec<u8>,&'static str> {
    let (count,length)=packed_sample_geometry(width,height,components,bits)?;
    if values.len()!=count*height as usize||values.iter().any(|value|u64::from(*value)>=1u64<<bits) {return Err("image samples do not match declared geometry and depth");}
    let row=length.checked_div(height as usize).unwrap_or(0);
    let mut packed=vec![0u8;length];
    for (index,value) in values.iter().enumerate() {
        let start=(index/count.max(1))*row*8+(index%count.max(1))*bits as usize;
        for offset in 0..bits as usize {let bit=start+offset;packed[bit/8]|=(((value>>(bits as usize-1-offset))&1) as u8)<<(7-bit%8);}
    }
    Ok(packed)
}

/// 🎭 An image's explicit mask (§8.9.6): a stencil image id or colour-key ranges.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfImageMask {
    Stencil { image: String },
    ColorKey { ranges: Vec<u32> },
}

/// 🖼️ An image XObject (§8.9.5, Table 89), keyed by `id`.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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
    /// 🖼️ Creates 8-bit RGB component samples in row order.
    pub fn rgb8(id:impl Into<String>,width:u32,height:u32,packed:Vec<u8>)->Self {Self::packed(id,width,height,PdfColorSpace::DeviceRgb,8,packed)}
    /// 🖼️ Creates 8-bit grayscale component samples in row order.
    pub fn gray8(id:impl Into<String>,width:u32,height:u32,packed:Vec<u8>)->Self {Self::packed(id,width,height,PdfColorSpace::DeviceGray,8,packed)}
    /// 🖼️ Creates an image from its native packed rows.
    pub fn packed(id:impl Into<String>,width:u32,height:u32,color_space:PdfColorSpace,bits_per_component:u32,packed:Vec<u8>)->Self {
        Self {id:id.into(),width,height,color_space:Some(color_space),bits_per_component,image_mask:false,decode:Vec::new(),interpolate:false,body:PdfImageBody::Samples {packed},soft_mask:None,soft_mask_in_data:None,mask:None,matte:None,intent:None,optional_content:None,struct_parent:None,extra:Vec::new()}
    }
    /// 🎨️ The colour components of one pixel: one for a stencil mask, else its colour space's.
    pub fn components(&self)->u32 {if self.image_mask {1} else {self.color_space.as_ref().and_then(PdfColorSpace::components).unwrap_or(1)}}
    /// 🎚️ The sample depth: one bit for a stencil mask, else its `BitsPerComponent`.
    pub fn depth(&self)->u32 {if self.image_mask {1} else {self.bits_per_component}}
    /// 🧮 Views the packed body as logical samples; `None` for a foreign artifact body.
    pub fn packed_samples(&self)->Option<Result<PdfPackedSamples<'_>,&'static str>> {
        match &self.body {PdfImageBody::Samples {packed}=>Some(PdfPackedSamples::new(packed,self.width,self.height,self.components(),self.depth())),PdfImageBody::Artifact {..}=>None}
    }
}

/// 🫧 A transparency group attribute dictionary (§11.6.6).
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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
