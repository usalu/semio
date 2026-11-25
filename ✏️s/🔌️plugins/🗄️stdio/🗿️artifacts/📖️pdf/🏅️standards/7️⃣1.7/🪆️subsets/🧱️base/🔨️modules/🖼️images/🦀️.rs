//! 🖼️ Image XObjects (ISO 32000-1 §8.9.5): lifting a stream into [`PdfImage`] — packed samples
//! or a retained image codec — and lowering it back, masks and soft masks included.

use crate::standards::v1_7::subsets::base::modules::colour::{extra_entries, lift_colour_space, lower_colour_space, numbers_of, push_opt};
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_i64, dict_name};
use crate::standards::v1_7::subsets::base::modules::xref::ObjectSink;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{PdfArtifactResourcePort,NativePdfArtifactResources};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::{packed_sample_geometry, PdfDictEntry, PdfImage, PdfImageBody, PdfImageMask, PdfObject, PdfPackedSamples, PdfStreamFilter};

#[path = "🎭️sampled-alpha/🦀️.rs"]
mod sampled_alpha;
pub use sampled_alpha::{ConvertColour, PdfImagePreviewSamples};

#[path = "🎞️controlled/🦀️.rs"]
mod controlled;
pub use controlled::{ControlledImageBinding, ControlledImageResources};
#[cfg(test)]
use controlled::ControlledImageCursor;

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
        body:PdfImageBody::Samples {packed:Vec::new()},
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
        Some(filter)=>{let kind=match filter {PdfStreamFilter::Dct {..}=>"s.stdio.jpeg",PdfStreamFilter::Jpx=>"s.stdio.jpeg2000",PdfStreamFilter::Ccitt {..}=>"s.stdio.ccitt",PdfStreamFilter::Jbig2 {..}=>"s.stdio.jbig2",_=>unreachable!()};let reference=NativePdfArtifactResources::default().admit(kind,PdfObject::Stream {dict:dict.to_vec(),data:crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(data),filters:filters.to_vec()})?;PdfImageBody::Artifact {reference}},
        None=>if let Some(reference)=sampled_artifact_reference(dict,data,filters)? {PdfImageBody::Artifact {reference}} else {PdfImageBody::Samples {packed:admit_packed_samples(image.width,image.height,image.components(),image.depth(),data.to_vec())?}},
    };Ok(image)

}

fn sampled_artifact_reference(dict: &[PdfDictEntry], data: &[u8], filters: &[PdfStreamFilter]) -> PResult<Option<semio_framework_artifact_reference::ArtifactRef>> {
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{
        INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES, PdfSampledResourceIdentity, encoded_sampled_image, qualified_image_dictionary,
    };
    if filters.is_empty() && encoded_sampled_image(dict) {
        let Some(mut identity) = PdfSampledResourceIdentity::encoded(dict) else { return Ok(None); };
        identity.append(data);
        return Ok(Some(identity.reference()));
    }
    if !matches!(filters, [] | [PdfStreamFilter::Flate { .. }]) || !qualified_image_dictionary(dict) {
        return Ok(None);
    }
    let Some(components) = dict_name(dict, "ColorSpace").and_then(|name| match name {
        "DeviceGray" => Some(1u32),
        "DeviceRGB" => Some(3u32),
        _ => None,
    }) else { return Ok(None); };
    let dimension = |key| match dict_get(dict, key) {
        Some(PdfObject::Int(raw)) => u32::try_from(*raw).ok().filter(|size| *size != 0),
        _ => None,
    };
    let width = dimension("Width").ok_or_else(|| PdfEngineError::Unsupported("invalid PDF image width".into()))?;
    let height = dimension("Height").ok_or_else(|| PdfEngineError::Unsupported("invalid PDF image height".into()))?;
    let (_, packed_length) = packed_image_geometry(width, height, components, 8)?;
    if packed_length != data.len() {
        return Err(PdfEngineError::Unsupported("image packed body does not match its declared geometry".into()));
    }
    if packed_length <= INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES {
        return Ok(None);
    }
    let Some(mut identity) = PdfSampledResourceIdentity::new(filters) else { return Ok(None); };
    identity.append(data);
    Ok(Some(identity.reference()))
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
        PdfImageBody::Samples {packed}=>{packed_samples_of(image)?;(crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(packed),vec![PdfStreamFilter::Flate {predictor:None}])},
        PdfImageBody::Artifact {reference}=>{
            use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{
                png_source::PngPlane, require_pdf_artifact_kind, SAMPLED_IMAGE_KIND,
            };
            let value = sink.resolve_artifact(reference)?;
            let sourced_png = PngPlane::match_reference(reference);
            let sampled = sourced_png.is_none() && require_pdf_artifact_kind(reference, SAMPLED_IMAGE_KIND).is_ok();
            if let Some(role) = sourced_png {
                let number = |key| value.dict_get(key).and_then(PdfObject::as_i64);
                if image.bits_per_component != 8 || role.channels() != image.components() as usize
                    || number("Width") != Some(i64::from(image.width)) || number("Height") != Some(i64::from(image.height))
                {
                    return Err(PdfEngineError::Malformed("PDF PNG resource geometry changed before image lowering".into()));
                }
            }
            let PdfObject::Stream {data,filters,..} = value else {
                return Err(PdfEngineError::Unsupported("image artifact is not a native stream".into()));
            };
            if sampled {
                let (_, expected) = packed_image_geometry(image.width, image.height, image.components(), image.depth())?;
                if data.len() != expected || !matches!(filters.as_slice(), [] | [PdfStreamFilter::Flate { .. }]) {
                    return Err(PdfEngineError::Malformed("PDF sampled image resource geometry changed before image lowering".into()));
                }
            } else if sourced_png.is_none() && !filters.iter().any(|filter|filter.is_image_codec()) {
                return Err(PdfEngineError::Unsupported("referenced image artifact has no native image codec".into()));
            }
            (data,filters)
        },
    };
    Ok(PdfObject::Stream {dict,data,filters})
}

/// 🧮 Checks a packed body against its image grid before it reaches a writer.
pub fn packed_samples_of(image: &PdfImage) -> PResult<Option<PdfPackedSamples<'_>>> {
    image.packed_samples().transpose().map_err(|reason| PdfEngineError::Unsupported(reason.into()))
}

/// 📏️ Returns the sample count per row and packed byte length for an image grid.
pub fn packed_image_geometry(width: u32, height: u32, components: u32, bits: u32) -> PResult<(usize, usize)> {
    packed_sample_geometry(width, height, components, bits).map_err(|reason| PdfEngineError::Unsupported(reason.into()))
}

/// 📦️ Admits a decoded image stream as its packed rows after checking them against the declared grid.
pub fn admit_packed_samples(width: u32, height: u32, components: u32, bits: u32, data: Vec<u8>) -> PResult<Vec<u8>> {
    PdfPackedSamples::new(&data, width, height, components, bits).map_err(|reason| PdfEngineError::Unsupported(reason.into()))?;
    Ok(data)
}

use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosReceipt, CosStatus};

enum SampleStage { Active, Complete, Cancelled, Closing, Closed }

/// 🎚️ Copies packed native rows in bounded runs while preserving partial output for bounded retirement.
pub struct PdfImagePackedCursor {
    packed_length: usize,
    output: Vec<u8>,
    stage: SampleStage,
}

impl PdfImagePackedCursor {
    pub fn new(width: u32, height: u32, components: u32, bits: u32) -> PResult<Self> {
        Self::checked_new(width, height, components, bits).map_err(|reason| PdfEngineError::Unsupported(reason.into()))
    }

    fn checked_new(width: u32, height: u32, components: u32, bits: u32) -> Result<Self, &'static str> {
        let (_, packed_length) = packed_sample_geometry(width, height, components, bits)?;
        Ok(Self { packed_length, output: Vec::new(), stage: SampleStage::Active })
    }

    pub fn advance(&mut self, data: &(impl semio_framework_value::ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> CosReceipt {
        if cancelled && matches!(self.stage, SampleStage::Active | SampleStage::Complete) { self.stage = SampleStage::Cancelled; }
        match self.stage {
            SampleStage::Complete => return sample_receipt(Ok(CosStatus::Complete { end: self.packed_length }), 0),
            SampleStage::Cancelled => return sample_receipt(Ok(CosStatus::Cancelled), 0),
            SampleStage::Closed => return sample_receipt(Ok(CosStatus::Closed), 0),
            SampleStage::Closing => return sample_receipt(Ok(CosStatus::More), 0),
            _ if grant.work == 0 => return sample_receipt(Ok(CosStatus::More), 0),
            _ => {}
        }
        if data.byte_len() != self.packed_length {
            return sample_receipt(Err("image packed body does not match its declared geometry"), 1);
        }
        if self.packed_length != 0 && self.output.capacity() == 0 {
            if self.packed_length > grant.allocation_bytes { return sample_receipt(Ok(CosStatus::NeedsAllocation { bytes: self.packed_length }), 0); }
            if self.output.try_reserve_exact(self.packed_length).is_err() {
                return sample_receipt(Err("unable to allocate image sample backing"), 1);
            }
            return CosReceipt { allocated_bytes: self.output.capacity(), ..sample_receipt(Ok(CosStatus::More), 1) };
        }
        let start = self.output.len();
        let count = (self.packed_length - start).min(grant.work);
        if count == 0 {
            self.stage = SampleStage::Complete;
            return sample_receipt(Ok(CosStatus::Complete { end: 0 }), 1);
        }
        self.output.resize(start + count, 0);
        let mut filled = start;
        while filled < start + count {
            let read = data.read_bytes(filled, &mut self.output[filled..start + count]);
            if read == 0 {
                self.output.truncate(start);
                return sample_receipt(Err("image sample source lost a byte"), 1);
            }
            filled += read;
        }
        if self.output.len() == self.packed_length {
            self.stage = SampleStage::Complete;
            sample_receipt(Ok(CosStatus::Complete { end: self.packed_length }), count)
        } else { sample_receipt(Ok(CosStatus::More), count) }
    }

    pub fn take_packed(&mut self) -> Option<Vec<u8>> {
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
        let bytes = self.output.capacity();
        if bytes > grant.release_bytes { return sample_receipt(Ok(CosStatus::NeedsRelease { bytes }), 0); }
        self.output = Vec::new();
        self.stage = SampleStage::Closed;
        CosReceipt { released_bytes: bytes, ..sample_receipt(Ok(CosStatus::Closed), 1) }
    }
}

fn sample_receipt(status: Result<CosStatus, &'static str>, work: usize) -> CosReceipt {
    CosReceipt { status, work, allocated_bytes: 0, released_bytes: 0 }
}

//#region 🧪️Tests

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn controlled_sampled_artifact_reference_agrees_with_the_native_resource_port() {
        use crate::standards::v1_7::subsets::base::{
            io::foreign_artifacts::{PdfSampledResourceIdentity, SAMPLED_IMAGE_KIND},
            schema::snapshot::PdfPredictor,
        };

        let data = [37u8; 5_000];
        let predictor = PdfPredictor { predictor: 12, colors: 3, bits_per_component: 8, columns: 16 };
        for filters in [
            Vec::new(),
            vec![PdfStreamFilter::Flate { predictor: None }],
            vec![PdfStreamFilter::Flate { predictor: Some(predictor) }],
        ] {
            let stream = PdfObject::Stream { dict: Vec::new(), data: crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(&data), filters: filters.clone() };
            let reference = NativePdfArtifactResources::default().admit(SAMPLED_IMAGE_KIND, stream).unwrap();
            let mut incremental = PdfSampledResourceIdentity::new(&filters).unwrap();
            assert_eq!(incremental.advance(&data, 1), 1);
            assert!(!incremental.complete(data.len()));
            assert_eq!(incremental.advance(&data, 1), 1);
            assert!(incremental.complete(data.len()));
            assert_eq!(incremental.reference(), reference);
            let bytes = reference.artifact_id.capacity() + reference.dialect.artifact_kind.capacity()
                + reference.dialect.standard.capacity() + reference.dialect.subset.capacity();
            assert_eq!(bytes, PdfSampledResourceIdentity::REFERENCE_BYTES);
        }
    }

    #[test]
    fn controlled_dct_reference_agrees_with_the_native_resource_port() {
        use crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfDctResourceIdentity;

        let data = b"physical DCT custody";
        for color_transform in [None, Some(0), Some(1)] {
            let stream = PdfObject::Stream {
                dict: Vec::new(), data: crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(data), filters: vec![PdfStreamFilter::Dct { color_transform }],
            };
            let reference = NativePdfArtifactResources::default().admit("s.stdio.jpeg", stream).unwrap();
            let mut incremental = PdfDctResourceIdentity::new(color_transform).unwrap();
            for _ in 0..data.len() {
                assert_eq!(incremental.advance(data, 1), 1);
            }
            assert!(incremental.complete(data.len()));
            assert_eq!(incremental.reference(), reference);
            let bytes = reference.artifact_id.capacity() + reference.dialect.artifact_kind.capacity()
                + reference.dialect.standard.capacity() + reference.dialect.subset.capacity();
            assert_eq!(bytes, PdfDctResourceIdentity::REFERENCE_BYTES);
        }
    }

    const ONE: CosGrant = CosGrant { work: 1, allocation_bytes: 64, release_bytes: 64 };

    #[test]
    fn packed_rows_copy_in_bounded_runs_and_read_back_through_row_padding_and_big_endian_components() {
        for (width, height, bits, packed, expected) in [
            (3, 2, 1, &[160, 64][..], &[1, 0, 1, 0, 1, 0][..]),
            (2, 1, 16, &[0x13, 0x57, 0xff, 0xff][..], &[0x1357, 65535][..]),
        ] {
            let mut cursor = PdfImagePackedCursor::new(width, height, 1, bits).unwrap();
            let denied = CosGrant { allocation_bytes: 0, ..ONE };
            assert_eq!(cursor.advance(packed, denied, false).status, Ok(CosStatus::NeedsAllocation { bytes: packed.len() }));
            for _ in 0..100 {
                let turn = cursor.advance(packed, ONE, false);
                assert!(turn.work <= 1);
                if let Ok(CosStatus::Complete { .. }) = turn.status { break; }
            }
            let copied = cursor.take_packed().unwrap();
            assert_eq!(copied, packed);
            let view = PdfPackedSamples::new(&copied, width, height, 1, bits).unwrap();
            assert_eq!(view.iter().collect::<Vec<_>>(), expected);
            assert_eq!(crate::standards::v1_7::subsets::base::schema::snapshot::pack_samples(width, height, 1, bits, expected).unwrap(), packed);
        }
    }


    #[test]
    fn controlled_xobject_replays_samples_under_two_capacity_grants() {
        let name = |key, value| PdfDictEntry::new(key, PdfObject::name(value));
        let mut stream = PdfObject::Stream {
            dict: vec![
                name("Type", "XObject"), name("Subtype", "Image"), name("ColorSpace", "DeviceRGB"),
                PdfDictEntry::new("Width", PdfObject::Int(2)), PdfDictEntry::new("Height", PdfObject::Int(1)),
                PdfDictEntry::new("BitsPerComponent", PdfObject::Int(8)),
                PdfDictEntry::new("ImageMask", PdfObject::Bool(false)), PdfDictEntry::new("Interpolate", PdfObject::Bool(true)),
            ],
            data: crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(&[255, 0, 4, 0, 128, 5]), filters: Vec::new(),
        };
        let mut image = ControlledImageCursor::new(&stream).unwrap().unwrap();
        let PdfObject::Stream { data, .. } = &stream else { unreachable!() };
        let denied = CosGrant { allocation_bytes: 0, ..ONE };
        for _ in 0..2 {
            assert!(image.reserving_samples());
            assert_eq!(image.advance(data, denied).status, Ok(CosStatus::NeedsAllocation { bytes: 6 }));
            assert_eq!(image.advance(data, ONE).allocated_bytes, 6);
            for _ in 0..6 {
                assert!(!image.reserving_samples());
                let step = image.advance(data, ONE);
                assert_eq!((step.work, step.allocated_bytes), (1, 0));
            }
        }
        let [image, role] = image.take_images("Im0", None).unwrap();
        assert_eq!(image, role);
        assert!(image.interpolate && !image.image_mask);
        assert_eq!(image.body, PdfImageBody::Samples { packed: vec![255, 0, 4, 0, 128, 5] });

        let mut partial = ControlledImageCursor::new(&stream).unwrap().unwrap();
        for _ in 0..10 { let _ = partial.advance(data, ONE); }
        partial.begin_close();
        let denied = CosGrant { release_bytes: 0, ..ONE };
        assert_eq!(partial.close_step(denied).status, Ok(CosStatus::NeedsRelease { bytes: 6 }));
        assert_eq!(partial.close_step(ONE).released_bytes, 6);
        assert_eq!(partial.close_step(ONE).released_bytes, 6);
        assert_eq!(partial.close_step(ONE).status, Ok(CosStatus::Closed));

        let PdfObject::Stream { data, .. } = &mut stream else { unreachable!() };
        { let mut bytes = data.to_vec_owner(); bytes.pop(); *data = crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(&bytes); }
        assert!(ControlledImageCursor::new(&stream).is_err());
        let PdfObject::Stream { dict, .. } = &mut stream else { unreachable!() };
        dict.push(PdfDictEntry::new("SMask", PdfObject::Null));
        assert!(ControlledImageCursor::new(&stream).unwrap().is_none());
    }

    #[test]
    fn cancelled_packed_rows_hold_the_allocated_rows_for_granted_close() {
        let mut cursor = PdfImagePackedCursor::new(4, 1, 1, 8).unwrap();
        let packed = [0, 4, 5, 76];
        assert_eq!(cursor.advance(&packed, ONE, false).allocated_bytes, 4);
        cursor.advance(&packed, ONE, false);
        assert_eq!(cursor.advance(&packed, ONE, true).status, Ok(CosStatus::Cancelled));
        assert!(cursor.take_packed().is_none());
        cursor.begin_close();
        let denied = CosGrant { release_bytes: 0, ..ONE };
        assert_eq!(cursor.close_step(denied).status, Ok(CosStatus::NeedsRelease { bytes: 4 }));
        let closed = cursor.close_step(ONE);
        assert_eq!(closed.status, Ok(CosStatus::Closed));
        assert_eq!(closed.released_bytes, 4);
    }
}

//#endregion 🧪️Tests
