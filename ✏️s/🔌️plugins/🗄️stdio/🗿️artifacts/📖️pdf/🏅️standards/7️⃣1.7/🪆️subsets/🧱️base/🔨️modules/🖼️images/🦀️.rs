//! 🖼️ Image XObjects (ISO 32000-1 §8.9.5): lifting a stream into [`PdfImage`] — packed samples
//! or a retained image codec — and lowering it back, masks and soft masks included.

use super::colour::{extra_entries, lift_colour_space, lower_colour_space, numbers_of, push_opt, raw_stream};
use super::lexer::{dict_get, dict_i64, dict_name};
use super::xref::ObjectSink;
use super::lexer::{PResult,PdfEngineError};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{PdfArtifactResourcePort,NativePdfArtifactResources};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfImage, PdfImageBody, PdfImageMask, PdfObject, PdfStreamFilter};

/// 📋 The image dictionary keys the typed model owns; everything else is `extra`.
const IMAGE_KEYS: &[&str] = &["Type", "Subtype", "Width", "Height", "ColorSpace", "BitsPerComponent", "ImageMask", "Decode", "Interpolate", "SMask", "SMaskInData", "Mask", "Matte", "Intent", "OC", "StructParent", "Length", "Filter", "DecodeParms"];

/// ⬇️ Lifts an image XObject stream. `id_of` maps the `/SMask`/stencil `/Mask` reference to the
/// image id it was lifted under; `oc_id_of` maps an optional-content reference to its group id.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_image(id: &str, dict: &[PdfDictEntry], data: &[u8], filters: &[PdfStreamFilter], source: &mut dyn ObjectSource, id_of: &mut dyn FnMut(&PdfObject) -> Option<String>, oc_id_of: &mut dyn FnMut(&PdfObject) -> Option<String>) -> PResult<PdfImage> {
    let codec=filters.iter().find(|filter|filter.is_image_codec());
    let image_mask = dict_get(dict, "ImageMask").and_then(PdfObject::as_bool).unwrap_or(false);
    let mask = match dict_get(dict, "Mask") {
        Some(PdfObject::Array(ranges)) => Some(PdfImageMask::ColorKey { ranges: ranges.iter().filter_map(PdfObject::as_i64).map(|v| v.max(0) as u32).collect() }),
        Some(reference @ PdfObject::Ref(_)) => id_of(reference).map(|image| PdfImageMask::Stencil { image }),
        _ => None,
    };
    let mut image=PdfImage {
        id: id.to_string(),
        width: dict_i64(dict, "Width").unwrap_or(0).max(0) as u32,
        height: dict_i64(dict, "Height").unwrap_or(0).max(0) as u32,
        color_space: if image_mask { None } else { dict_get(dict, "ColorSpace").map(|cs| lift_colour_space(cs, source)) },
        bits_per_component: if image_mask { 1 } else { dict_i64(dict, "BitsPerComponent").unwrap_or(if codec.is_some() { 8 } else { 1 }).max(0) as u32 },
        image_mask,
        decode: numbers_of(dict_get(dict, "Decode")),
        interpolate: dict_get(dict, "Interpolate").and_then(PdfObject::as_bool).unwrap_or(false),
        body:PdfImageBody::Samples {values:Vec::new()},
        soft_mask: dict_get(dict, "SMask").and_then(|reference| id_of(reference)),
        soft_mask_in_data: dict_i64(dict, "SMaskInData").map(|v| v as u32),
        mask,
        matte: dict_get(dict, "Matte").map(|v| numbers_of(Some(v))),
        intent: dict_name(dict, "Intent").map(str::to_string),
        optional_content: dict_get(dict, "OC").and_then(|reference| oc_id_of(reference)),
        struct_parent: dict_i64(dict, "StructParent").map(|v| v as u32),
        extra: extra_entries(dict, IMAGE_KEYS),
    };
    image.body=match codec {
        Some(filter)=>{let kind=match filter {PdfStreamFilter::Dct {..}=>"s.stdio.jpeg",PdfStreamFilter::Jpx=>"s.stdio.jpeg2000",PdfStreamFilter::Ccitt {..}=>"s.stdio.ccitt",PdfStreamFilter::Jbig2 {..}=>"s.stdio.jbig2",_=>unreachable!()};let reference=NativePdfArtifactResources::default().admit(kind,PdfObject::Stream {dict:dict.to_vec(),data:data.to_vec(),filters:filters.to_vec()})?;PdfImageBody::Artifact {reference}},
        None=>PdfImageBody::Samples {values:unpack_image_samples(image.width,image.height,image_components(&image),image.bits_per_component,data)?},
    };Ok(image)

}

/// ⬆️ Lowers an image to its stream object. `ref_of` resolves the ids of masks/soft masks and
/// optional-content groups to references.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_image(image: &PdfImage, sink: &mut dyn ObjectSink, ref_of: &mut dyn FnMut(&str) -> Option<PdfObject>, oc_ref_of: &mut dyn FnMut(&str) -> Option<PdfObject>) -> PResult<PdfObject> {
    let mut dict = vec![PdfDictEntry::new("Type", PdfObject::name("XObject")), PdfDictEntry::new("Subtype", PdfObject::name("Image")), PdfDictEntry::new("Width", PdfObject::Int(image.width as i64)), PdfDictEntry::new("Height", PdfObject::Int(image.height as i64))];
    if image.image_mask {
        dict.push(PdfDictEntry::new("ImageMask", PdfObject::Bool(true)));
    } else {
        push_opt(&mut dict, "ColorSpace", image.color_space.as_ref().map(|cs| lower_colour_space(cs, sink)).transpose()?);
        dict.push(PdfDictEntry::new("BitsPerComponent", PdfObject::Int(image.bits_per_component.max(1) as i64)));
    }
    if !image.decode.is_empty() {
        dict.push(PdfDictEntry::new("Decode", PdfObject::numbers(&image.decode)));
    }
    if image.interpolate {
        dict.push(PdfDictEntry::new("Interpolate", PdfObject::Bool(true)));
    }
    push_opt(&mut dict, "SMask", image.soft_mask.as_deref().and_then(|id| ref_of(id)));
    push_opt(&mut dict, "SMaskInData", image.soft_mask_in_data.map(|v| PdfObject::Int(v as i64)));
    match &image.mask {
        Some(PdfImageMask::ColorKey { ranges }) => dict.push(PdfDictEntry::new("Mask", PdfObject::Array(ranges.iter().map(|v| PdfObject::Int(*v as i64)).collect()))),
        Some(PdfImageMask::Stencil { image: stencil }) => push_opt(&mut dict, "Mask", ref_of(stencil)),
        None => {}
    }
    push_opt(&mut dict, "Matte", image.matte.as_ref().map(|v| PdfObject::numbers(v)));
    push_opt(&mut dict, "Intent", image.intent.as_ref().map(PdfObject::name));
    push_opt(&mut dict, "OC", image.optional_content.as_deref().and_then(|id| oc_ref_of(id)));
    push_opt(&mut dict, "StructParent", image.struct_parent.map(|v| PdfObject::Int(v as i64)));
    dict.extend(image.extra.iter().cloned());
    let (data,filters)=match &image.body {
        PdfImageBody::Samples {values}=>(pack_image_samples(image.width,image.height,image_components(image),image.bits_per_component,values)?,vec![PdfStreamFilter::Flate {predictor:None}]),
        PdfImageBody::Artifact {reference}=>{let PdfObject::Stream {data,filters,..}=sink.resolve_artifact(reference)? else{return Err(PdfEngineError::Unsupported("image artifact is not a native stream".into()));};if !filters.iter().any(|filter|filter.is_image_codec()){return Err(PdfEngineError::Unsupported("referenced image artifact has no native image codec".into()));}(data,filters)},
    };
    let mut object=raw_stream(dict,data);if let PdfObject::Stream {filters:slot,..}=&mut object {*slot=filters;}

    Ok(object)
}

fn image_components(image:&PdfImage)->u32 {if image.image_mask{1}else{image.color_space.as_ref().and_then(|color|color.components()).unwrap_or(1)}}
fn geometry(width:u32,height:u32,components:u32,bits:u32)->PResult<(usize,usize)> {
    if !matches!(bits,1|2|4|8|16)||components==0{return Err(PdfEngineError::Unsupported("image sample depth or components are invalid".into()));}
    let count=(width as usize).checked_mul(components as usize).ok_or_else(||PdfEngineError::Unsupported("image row exceeds native address space".into()))?;
    let row=count.checked_mul(bits as usize).ok_or_else(||PdfEngineError::Unsupported("image row width overflow".into()))?.div_ceil(8);
    count.checked_mul(height as usize).ok_or_else(||PdfEngineError::Unsupported("image sample count overflow".into()))?;
    let length=row.checked_mul(height as usize).ok_or_else(||PdfEngineError::Unsupported("image body width overflow".into()))?;Ok((count,length))
}
/// 🛬️ Admits packed native rows as explicit logical sample values.
pub fn unpack_image_samples(width:u32,height:u32,components:u32,bits:u32,data:&[u8])->PResult<Vec<u32>> {
    let (count,length)=geometry(width,height,components,bits)?;if data.len()!=length{return Err(PdfEngineError::Unsupported("image packed body does not match its declared geometry".into()));}
    let row=if height==0{0}else{length/height as usize};let mut values=Vec::with_capacity(count*height as usize);
    for y in 0..height as usize {for x in 0..count {let start=y*row*8+x*bits as usize;let mut value=0;for offset in 0..bits as usize {let bit=start+offset;value=(value<<1)|u32::from((data[bit/8]>>(7-bit%8))&1);}values.push(value);}}Ok(values)
}
/// 🛫️ Packs explicit logical samples only at native image emission.
pub fn pack_image_samples(width:u32,height:u32,components:u32,bits:u32,values:&[u32])->PResult<Vec<u8>> {
    let (count,length)=geometry(width,height,components,bits)?;if values.len()!=count*height as usize||values.iter().any(|value|*value>=(1u32<<bits)){return Err(PdfEngineError::Unsupported("image samples do not match declared geometry and depth".into()));}
    let row=if height==0{0}else{length/height as usize};let mut data=vec![0;length];for y in 0..height as usize {for x in 0..count {let start=y*row*8+x*bits as usize;let value=values[y*count+x];for offset in 0..bits as usize {let bit=start+offset;data[bit/8]|=(((value>>(bits as usize-1-offset))&1)as u8)<<(7-bit%8);}}}Ok(data)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
