//! 🖼️ Image XObjects (ISO 32000-1 §8.9.5): lifting a stream into [`PdfImage`] — packed samples
//! or a retained image codec — and lowering it back, masks and soft masks included.

use crate::standards::v1_7::subsets::base::modules::colour::{extra_entries, lift_colour_space, lower_colour_space, numbers_of, push_opt, raw_stream};
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_i64, dict_name};
use crate::standards::v1_7::subsets::base::modules::xref::ObjectSink;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{PdfArtifactResourcePort,NativePdfArtifactResources};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfImage, PdfImageBody, PdfImageMask, PdfObject, PdfStreamFilter};

#[path = "🎭️sampled-alpha/🦀️.rs"]
mod sampled_alpha;
pub use sampled_alpha::PdfImagePreviewSamples;

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
/// 📏️ Returns the sample count per row and packed byte length for an image grid.
pub fn packed_image_geometry(width: u32, height: u32, components: u32, bits: u32) -> PResult<(usize, usize)> {
    if !matches!(bits, 1 | 2 | 4 | 8 | 16) || components == 0 {
        return Err(PdfEngineError::Unsupported("image sample depth or components are invalid".into()));
    }
    let count = (width as usize).checked_mul(components as usize)
        .ok_or_else(|| PdfEngineError::Unsupported("image row sample count overflow".into()))?;
    let row = count.checked_mul(bits as usize)
        .ok_or_else(|| PdfEngineError::Unsupported("image row width overflow".into()))?.div_ceil(8);
    count.checked_mul(height as usize).filter(|count| *count <= isize::MAX as usize / std::mem::size_of::<u32>())
        .ok_or_else(|| PdfEngineError::Unsupported("image sample storage overflow".into()))?;
    let length = row.checked_mul(height as usize)
        .ok_or_else(|| PdfEngineError::Unsupported("image body width overflow".into()))?;
    Ok((count, length))
}
use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosReceipt, CosStatus};

enum SampleStage { Active, Complete, Cancelled, Closing, Closed }

/// 🎚️ Admits packed native rows while preserving partial output for bounded retirement.
pub struct PdfImageSampleCursor {
    row_samples: usize,
    row_bytes: usize,
    packed_length: usize,
    sample_count: usize,
    bits: usize,
    output: Vec<u32>,
    stage: SampleStage,
}

impl PdfImageSampleCursor {
    pub fn new(width: u32, height: u32, components: u32, bits: u32) -> PResult<Self> {
        let (row_samples, packed_length) = packed_image_geometry(width, height, components, bits)?;
        let height = height as usize;
        let row_bytes = packed_length.checked_div(height).unwrap_or(0);
        Ok(Self { row_samples, row_bytes, packed_length, sample_count: row_samples * height, bits: bits as usize, output: Vec::new(), stage: SampleStage::Active })
    }

    pub fn advance(&mut self, data: &[u8], grant: CosGrant, cancelled: bool) -> CosReceipt {
        if cancelled && matches!(self.stage, SampleStage::Active | SampleStage::Complete) { self.stage = SampleStage::Cancelled; }
        match self.stage {
            SampleStage::Complete => return sample_receipt(Ok(CosStatus::Complete { end: self.sample_count }), 0),
            SampleStage::Cancelled => return sample_receipt(Ok(CosStatus::Cancelled), 0),
            SampleStage::Closed => return sample_receipt(Ok(CosStatus::Closed), 0),
            SampleStage::Closing => return sample_receipt(Ok(CosStatus::More), 0),
            _ if grant.work == 0 => return sample_receipt(Ok(CosStatus::More), 0),
            _ => {}
        }
        if data.len() != self.packed_length {
            return sample_receipt(Err("image packed body does not match its declared geometry"), 1);
        }
        if self.sample_count != 0 && self.output.capacity() == 0 {
            let bytes = self.sample_count * std::mem::size_of::<u32>();
            if bytes > grant.allocation_bytes { return sample_receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0); }
            if self.output.try_reserve_exact(self.sample_count).is_err() {
                return sample_receipt(Err("unable to allocate image sample backing"), 1);
            }
            return CosReceipt { allocated_bytes: self.output.capacity() * std::mem::size_of::<u32>(), ..sample_receipt(Ok(CosStatus::More), 1) };
        }

        let count = (self.sample_count - self.output.len()).min(grant.work);
        if count == 0 {
            self.stage = SampleStage::Complete;
            return sample_receipt(Ok(CosStatus::Complete { end: 0 }), 1);
        }
        let mut x = self.output.len() % self.row_samples;
        let mut y = self.output.len() / self.row_samples;
        for _ in 0..count {
            let bit = x * self.bits;
            let byte = y * self.row_bytes + bit / 8;
            let value = match self.bits {
                16 => u32::from(u16::from_be_bytes([data[byte], data[byte + 1]])),
                8 => u32::from(data[byte]),
                bits => {
                    let shift = 8 - (bit % 8) - bits;
                    u32::from((data[byte] >> shift) & ((1u8 << bits) - 1))
                }
            };
            self.output.push(value);
            x += 1;
            if x == self.row_samples { x = 0; y += 1; }
        }
        if self.output.len() == self.sample_count {
            self.stage = SampleStage::Complete;
            sample_receipt(Ok(CosStatus::Complete { end: self.sample_count }), count)
        } else { sample_receipt(Ok(CosStatus::More), count) }
    }

    pub fn take_samples(&mut self) -> Option<Vec<u32>> {
        if !matches!(self.stage, SampleStage::Complete) { return None; }
        self.stage = SampleStage::Closed;
        Some(std::mem::take(&mut self.output))
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.stage, SampleStage::Closed) { self.stage = SampleStage::Closing; }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if matches!(self.stage, SampleStage::Closed) { return sample_receipt(Ok(CosStatus::Closed), 0); }
        if !matches!(self.stage, SampleStage::Closing) || grant.work == 0 { return sample_receipt(Ok(CosStatus::More), 0); }
        let bytes = self.output.capacity() * std::mem::size_of::<u32>();
        if bytes > grant.release_bytes { return sample_receipt(Ok(CosStatus::NeedsRelease { bytes }), 0); }
        self.output = Vec::new();
        self.stage = SampleStage::Closed;
        CosReceipt { released_bytes: bytes, ..sample_receipt(Ok(CosStatus::Closed), 1) }
    }
}

fn sample_receipt(status: Result<CosStatus, &'static str>, work: usize) -> CosReceipt {
    CosReceipt { status, work, allocated_bytes: 0, released_bytes: 0 }
}

/// 🛬️ Admits packed native rows as explicit logical sample values.
pub fn unpack_image_samples(width: u32, height: u32, components: u32, bits: u32, data: &[u8]) -> PResult<Vec<u32>> {
    let mut cursor = PdfImageSampleCursor::new(width, height, components, bits)?;
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    loop {
        match cursor.advance(data, grant, false).status {
            Ok(CosStatus::Complete { .. }) => return Ok(cursor.take_samples().expect("complete PDF image sample output")),
            Ok(CosStatus::More) => {}
            Err(reason) => return Err(PdfEngineError::Unsupported(reason.into())),
            _ => return Err(PdfEngineError::Unsupported("unexpected image sample state".into())),
        }
    }
}

/// 🛫️ Packs explicit logical samples only at native image emission.
pub fn pack_image_samples(width:u32,height:u32,components:u32,bits:u32,values:&[u32])->PResult<Vec<u8>> {
    let (count,length)=packed_image_geometry(width,height,components,bits)?;if values.len()!=count*height as usize||values.iter().any(|value|*value>=(1u32<<bits)){return Err(PdfEngineError::Unsupported("image samples do not match declared geometry and depth".into()));}
    let row=if height==0{0}else{length/height as usize};let mut data=vec![0;length];for y in 0..height as usize {for x in 0..count {let start=y*row*8+x*bits as usize;let value=values[y*count+x];for offset in 0..bits as usize {let bit=start+offset;data[bit/8]|=(((value>>(bits as usize-1-offset))&1)as u8)<<(7-bit%8);}}}Ok(data)
}

//#region 🧪️Tests

#[cfg(test)]
mod sample_tests {
    use super::*;

    const ONE: CosGrant = CosGrant { work: 1, allocation_bytes: 64, release_bytes: 64 };

    #[test]
    fn inline_sample_work_follows_row_padding_and_big_endian_components() {
        for (width, height, bits, packed, expected) in [
            (3, 2, 1, &[160, 64][..], &[1, 0, 1, 0, 1, 0][..]),
            (2, 1, 16, &[0x13, 0x57, 0xff, 0xff][..], &[0x1357, 65535][..]),
        ] {
            let mut cursor = PdfImageSampleCursor::new(width, height, 1, bits).unwrap();
            let denied = CosGrant { allocation_bytes: 0, ..ONE };
            let quote = expected.len() * std::mem::size_of::<u32>();
            assert_eq!(cursor.advance(packed, denied, false).status, Ok(CosStatus::NeedsAllocation { bytes: quote }));
            for _ in 0..100 {
                let turn = cursor.advance(packed, ONE, false);
                assert!(turn.work <= 1);
                if let Ok(CosStatus::Complete { .. }) = turn.status { break; }
            }
            assert_eq!(cursor.take_samples().unwrap(), expected);
        }
    }

    #[test]
    fn cancelled_inline_samples_hold_the_allocated_rows_for_granted_close() {
        let mut cursor = PdfImageSampleCursor::new(4, 1, 1, 8).unwrap();
        let packed = [0, 4, 5, 76];
        assert_eq!(cursor.advance(&packed, ONE, false).allocated_bytes, 16);
        cursor.advance(&packed, ONE, false);
        assert_eq!(cursor.advance(&packed, ONE, true).status, Ok(CosStatus::Cancelled));
        assert!(cursor.take_samples().is_none());
        cursor.begin_close();
        let denied = CosGrant { release_bytes: 0, ..ONE };
        assert_eq!(cursor.close_step(denied).status, Ok(CosStatus::NeedsRelease { bytes: 16 }));
        let closed = cursor.close_step(ONE);
        assert_eq!(closed.status, Ok(CosStatus::Closed));
        assert_eq!(closed.released_bytes, 16);
    }
}

//#endregion 🧪️Tests
