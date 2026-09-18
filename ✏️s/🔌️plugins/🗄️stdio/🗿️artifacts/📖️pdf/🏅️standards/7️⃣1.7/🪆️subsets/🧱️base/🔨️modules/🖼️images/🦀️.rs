//! 🖼️ Image XObjects (ISO 32000-1 §8.9.5): lifting a stream into [`PdfImage`] — packed samples
//! or a retained image codec — and lowering it back, masks and soft masks included.

use super::colour::{extra_entries, lift_colour_space, lower_colour_space, numbers_of, push_opt, raw_stream};
use super::lexer::{dict_get, dict_i64, dict_name};
use super::xref::{ObjectSink, ObjectSource};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfImage, PdfImageCodec, PdfImageMask, PdfObject, PdfStreamFilter};

/// 📋 The image dictionary keys the typed model owns; everything else is `extra`.
const IMAGE_KEYS: &[&str] = &["Type", "Subtype", "Width", "Height", "ColorSpace", "BitsPerComponent", "ImageMask", "Decode", "Interpolate", "SMask", "SMaskInData", "Mask", "Matte", "Intent", "OC", "StructParent", "Length", "Filter", "DecodeParms"];

/// ⬇️ Lifts an image XObject stream. `id_of` maps the `/SMask`/stencil `/Mask` reference to the
/// image id it was lifted under; `oc_id_of` maps an optional-content reference to its group id.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_image(id: &str, dict: &[PdfDictEntry], data: &[u8], filters: &[PdfStreamFilter], source: &mut dyn ObjectSource, id_of: &mut dyn FnMut(&PdfObject) -> Option<String>, oc_id_of: &mut dyn FnMut(&PdfObject) -> Option<String>) -> PdfImage {
    let codec = filters.iter().find(|filter| filter.is_image_codec()).map(|filter| match filter {
        PdfStreamFilter::Dct { color_transform } => PdfImageCodec::Dct { color_transform: *color_transform },
        PdfStreamFilter::Jpx => PdfImageCodec::Jpx,
        PdfStreamFilter::Ccitt { parameters } => PdfImageCodec::Ccitt { parameters: parameters.clone() },
        PdfStreamFilter::Jbig2 { globals } => PdfImageCodec::Jbig2 { globals: globals.clone() },
        _ => PdfImageCodec::Raw,
    });
    let image_mask = dict_get(dict, "ImageMask").and_then(PdfObject::as_bool).unwrap_or(false);
    let mask = match dict_get(dict, "Mask") {
        Some(PdfObject::Array(ranges)) => Some(PdfImageMask::ColorKey { ranges: ranges.iter().filter_map(PdfObject::as_i64).map(|v| v.max(0) as u32).collect() }),
        Some(reference @ PdfObject::Ref(_)) => id_of(reference).map(|image| PdfImageMask::Stencil { image }),
        _ => None,
    };
    PdfImage {
        id: id.to_string(),
        width: dict_i64(dict, "Width").unwrap_or(0).max(0) as u32,
        height: dict_i64(dict, "Height").unwrap_or(0).max(0) as u32,
        color_space: if image_mask { None } else { dict_get(dict, "ColorSpace").map(|cs| lift_colour_space(cs, source)) },
        bits_per_component: if image_mask { 1 } else { dict_i64(dict, "BitsPerComponent").unwrap_or(if codec.is_some() { 8 } else { 1 }).max(0) as u32 },
        image_mask,
        decode: numbers_of(dict_get(dict, "Decode")),
        interpolate: dict_get(dict, "Interpolate").and_then(PdfObject::as_bool).unwrap_or(false),
        codec: codec.unwrap_or(PdfImageCodec::Raw),
        data: data.to_vec(),
        soft_mask: dict_get(dict, "SMask").and_then(|reference| id_of(reference)),
        soft_mask_in_data: dict_i64(dict, "SMaskInData").map(|v| v as u32),
        mask,
        matte: dict_get(dict, "Matte").map(|v| numbers_of(Some(v))),
        intent: dict_name(dict, "Intent").map(str::to_string),
        optional_content: dict_get(dict, "OC").and_then(|reference| oc_id_of(reference)),
        struct_parent: dict_i64(dict, "StructParent").map(|v| v as u32),
        extra: extra_entries(dict, IMAGE_KEYS),
    }
}

/// ⬆️ Lowers an image to its stream object. `ref_of` resolves the ids of masks/soft masks and
/// optional-content groups to references.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_image(image: &PdfImage, sink: &mut dyn ObjectSink, ref_of: &mut dyn FnMut(&str) -> Option<PdfObject>, oc_ref_of: &mut dyn FnMut(&str) -> Option<PdfObject>) -> PdfObject {
    let mut dict = vec![PdfDictEntry::new("Type", PdfObject::name("XObject")), PdfDictEntry::new("Subtype", PdfObject::name("Image")), PdfDictEntry::new("Width", PdfObject::Int(image.width as i64)), PdfDictEntry::new("Height", PdfObject::Int(image.height as i64))];
    if image.image_mask {
        dict.push(PdfDictEntry::new("ImageMask", PdfObject::Bool(true)));
    } else {
        push_opt(&mut dict, "ColorSpace", image.color_space.as_ref().map(|cs| lower_colour_space(cs, sink)));
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
    let filters = match &image.codec {
        PdfImageCodec::Raw => vec![PdfStreamFilter::Flate { predictor: None }],
        PdfImageCodec::Dct { color_transform } => vec![PdfStreamFilter::Dct { color_transform: *color_transform }],
        PdfImageCodec::Jpx => vec![PdfStreamFilter::Jpx],
        PdfImageCodec::Ccitt { parameters } => vec![PdfStreamFilter::Ccitt { parameters: parameters.clone() }],
        PdfImageCodec::Jbig2 { globals } => vec![PdfStreamFilter::Jbig2 { globals: globals.clone() }],
    };
    let mut object = raw_stream(dict, image.data.clone());
    if let PdfObject::Stream { filters: slot, .. } = &mut object {
        *slot = filters;
    }
    object
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
