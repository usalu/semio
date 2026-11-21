//! 🎞️ Admits Image XObjects and their independent native stream roles.

use super::{dict_get, dict_name, PdfDictEntry, PdfImage, PdfImageSampleCursor, PdfObject, PdfStreamFilter};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfDctResourceIdentity;
use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosReceipt, CosStatus};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfImageBody};
use super::sample_receipt;

const DEPTH: u32 = 8;

enum ImageBacking {
    Samples { cursors: [PdfImageSampleCursor; 2], next: usize },
    Dct { identity: PdfDctResourceIdentity, bytes: usize },
}

/// 🎬️ Resolves sampled or encoded native content into two independently owned image roles.
pub(super) struct ControlledImageCursor {
    width: u32,
    height: u32,
    gray: bool,
    interpolate: bool,
    backing: ImageBacking,
}

impl ControlledImageCursor {
    /// 🔍️ Qualifies a typed native stream without acquiring sample backing.
    pub(super) fn new(value: &PdfObject) -> Result<Option<Self>, &'static str> {
        let PdfObject::Stream { dict, data, filters } = value else { return Ok(None); };
        if !matches!(filters.as_slice(), [] | [PdfStreamFilter::Flate { .. } | PdfStreamFilter::Dct { .. }]) || !qualified_dictionary(dict) { return Ok(None); }
        let Some(gray) = dict_name(dict, "ColorSpace").and_then(|name| match name {
            "DeviceGray" => Some(true),
            "DeviceRGB" => Some(false),
            _ => None,
        }) else { return Ok(None); };
        let dimension = |key| match dict_get(dict, key) {
            Some(PdfObject::Int(raw)) => u32::try_from(*raw).ok().filter(|size| *size != 0),
            _ => None,
        };
        let width = dimension("Width").ok_or("invalid PDF image width")?;
        let height = dimension("Height").ok_or("invalid PDF image height")?;
        let interpolate = dict_get(dict, "Interpolate").and_then(PdfObject::as_bool).unwrap_or(false);
        let backing = match filters.as_slice() {
            [] | [PdfStreamFilter::Flate { .. }] => {
                let components = if gray { 1 } else { 3 };
                let sample_cursor = || PdfImageSampleCursor::checked_new(width, height, components, DEPTH);
                let first = sample_cursor()?;
                if first.packed_length != data.len() { return Err("image packed body does not match its declared geometry"); }
                ImageBacking::Samples { cursors: [first, sample_cursor()?], next: 0 }
            }
            [PdfStreamFilter::Dct { color_transform }] => {
                if !data.starts_with(&[0xff, 0xd8]) || !data.ends_with(&[0xff, 0xd9]) { return Ok(None); }
                let Some(identity) = PdfDctResourceIdentity::new(*color_transform) else { return Ok(None); };
                ImageBacking::Dct { identity, bytes: data.len() }
            }
            _ => return Ok(None),
        };
        Ok(Some(Self { width, height, gray, interpolate, backing }))
    }

    fn done(&self) -> bool {
        match &self.backing {
            ImageBacking::Samples { cursors, next } => *next == cursors.len(),
            ImageBacking::Dct { identity, bytes } => identity.complete(*bytes),
        }
    }

    fn reference_bytes(&self) -> usize {
        if matches!(&self.backing, ImageBacking::Dct { .. }) { 2 * PdfDctResourceIdentity::REFERENCE_BYTES } else { 0 }
    }

    /// 📐️ Distinguishes allocation of empty sample capacity from writes into that capacity.
    pub(super) fn reserving_samples(&self) -> bool {
        match &self.backing {
            ImageBacking::Samples { cursors, next } => cursors.get(*next).is_some_and(|cursor| cursor.output.capacity() == 0),
            ImageBacking::Dct { .. } => false,
        }
    }

    /// ▶️ Advances one lane while borrowing native stream bytes from the graph.
    pub(super) fn advance(&mut self, data: &[u8], grant: CosGrant) -> CosReceipt {
        match &mut self.backing {
            ImageBacking::Samples { cursors, next } => {
                let Some(cursor) = cursors.get_mut(*next) else {
                    return sample_receipt(Ok(CosStatus::Complete { end: data.len() }), 0);
                };
                let mut turn = cursor.advance(data, grant, false);
                if matches!(turn.status, Ok(CosStatus::Complete { .. })) {
                    *next += 1;
                    if *next < cursors.len() { turn.status = Ok(CosStatus::More); }
                }
                turn
            }
            ImageBacking::Dct { identity, bytes } => {
                if *bytes != data.len() { return sample_receipt(Err("qualified JPEG stream changed extent"), 1); }
                let work = identity.advance(data, grant.work);
                sample_receipt(Ok(CosStatus::More), work)
            }
        }
    }

    /// 🫴️ Transfers both role bodies after backing and identifier work is fully admitted.
    pub(super) fn take_images(&mut self, id: &str) -> Option<[PdfImage; 2]> {
        if !self.done() { return None; }
        let bodies = match &mut self.backing {
            ImageBacking::Samples { cursors: [first, second], .. } => {
                [first.take_samples()?, second.take_samples().expect("complete replayed PDF image")]
                    .map(|values| PdfImageBody::Samples { values })
            }
            ImageBacking::Dct { identity, .. } => {
                let reference = identity.reference();
                [reference.clone(), reference].map(|reference| PdfImageBody::Artifact { reference })
            }
        };
        Some(bodies.map(|body| {
            let color_space = if self.gray { PdfColorSpace::DeviceGray } else { PdfColorSpace::DeviceRgb };
            let mut image = PdfImage::samples(id, self.width, self.height, color_space, DEPTH, Vec::new());
            image.interpolate = self.interpolate;
            image.body = body;
            image
        }))
    }

    /// ♻️ Prepares any active sample capacities for retirement.
    pub(super) fn begin_close(&mut self) {
        if let ImageBacking::Samples { cursors, next } = &mut self.backing {
            for cursor in cursors { cursor.begin_close(); }
            *next = 0;
        }
    }

    /// 🧹️ Releases sample owners under explicit grants. The DCT hash owns no backing.
    pub(super) fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        let ImageBacking::Samples { cursors, next } = &mut self.backing else {
            return sample_receipt(Ok(CosStatus::Closed), 0);
        };
        let Some(cursor) = cursors.get_mut(*next) else { return sample_receipt(Ok(CosStatus::Closed), 0); };
        let mut turn = cursor.close_step(grant);
        if turn.status == Ok(CosStatus::Closed) {
            *next += 1;
            turn.status = Ok(CosStatus::More);
        }
        turn
    }
}

fn qualified_dictionary(dict: &[PdfDictEntry]) -> bool {
    dict.len() <= 8 && dict.iter().enumerate().all(|(index, entry)| {
        !dict[..index].iter().any(|previous| previous.key == entry.key) && match entry.key.as_str() {
            "Type" => entry.value.as_name() == Some("XObject"),
            "Subtype" => entry.value.as_name() == Some("Image"),
            "BitsPerComponent" => entry.value == PdfObject::Int(i64::from(DEPTH)),
            "ImageMask" => entry.value.as_bool() == Some(false),
            "Interpolate" => entry.value.as_bool().is_some(),
            "Width" | "Height" | "ColorSpace" => true,
            _ => false,
        }
    }) && dict_name(dict, "Subtype") == Some("Image") && dict_get(dict, "BitsPerComponent").is_some()
}

//#region 🔖️Image resources

use super::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::{
    snapshot::{ObjRef, PdfSnapshot},
    stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue},
};
use std::mem::size_of;

const MAX_NAME: usize = 128;

/// 🏷️ One authored page resource with an exact native image object identity.
pub struct ControlledImageBinding {
    owner: ObjRef,
    name: [u8; MAX_NAME],
    length: usize,
}

impl ControlledImageBinding {
    /// 📃️ Reads the one indirect image binding in a qualified XObject subdictionary.
    pub fn from_xobjects(value: &PdfObject) -> Option<Self> {
        let PdfObject::Dict(entries) = value else { return None; };
        let [entry] = entries.as_slice() else { return None; };
        let owner = entry.value.as_ref()?;
        let length = entry.key.len();
        if length == 0 || length > MAX_NAME { return None; }
        let mut name = [0; MAX_NAME];
        name[..length].copy_from_slice(entry.key.as_bytes());
        Some(Self { owner, name, length })
    }

    fn name(&self) -> &str { std::str::from_utf8(&self.name[..self.length]).expect("authored PDF image name") }
}

struct ImageAdmission {
    binding: ControlledImageBinding,
    samples: Option<ControlledImageCursor>,
    role_position: Option<usize>,
}

/// 🪟️ Coordinates page bindings and the independently owned image roles of one native lift.
#[derive(Default)]
pub struct ControlledImageResources {
    owners: [ObjRef; Self::MAX_COUNT],
    active: Option<ImageAdmission>,
    page_image: Option<usize>,
}

impl ControlledImageResources {
    /// 📏️ Limits the number of distinct image resources on the controlled path.
    pub const MAX_COUNT: usize = 8;

    /// 📑️ Binds a new page to its inherited image, if present.
    pub fn set_page(&mut self, image: Option<usize>) { self.page_image = image; }

    /// 🔗️ Resolves the active page's qualified authored name to its stable typed image ID.
    pub fn page_name<'a>(&self, snapshot: &'a PdfSnapshot) -> Option<&'a str> {
        self.page_image.map(|index| snapshot.images[index].id.as_str())
    }

    /// 🧭️ Confirms the resolved page resource at the content role's own ancestry.
    pub fn matches_page(&self, snapshot: &PdfSnapshot, binding: &ControlledImageBinding) -> bool {
        self.page_image.is_some_and(|index| self.owners[index] == binding.owner && snapshot.images[index].id == binding.name())
    }

    /// 🌱 Starts admission of a resource inspected through its exact page or branch dictionary.
    pub fn begin(&mut self, binding: ControlledImageBinding) {
        self.active = Some(ImageAdmission { binding, samples: None, role_position: None });
    }

    /// 📦️ Quotes empty owned table or sample backing separately from writes.
    pub fn reserves_empty(&self, snapshot: &PdfSnapshot) -> bool {
        self.active.as_ref().and_then(|active| active.samples.as_ref()).is_some_and(|samples| {
            snapshot.images.capacity() == 0 || samples.reserving_samples()
        })
    }

    /// 📨️ Quotes the inline typed values that the next work unit can write or move.
    pub fn inline_body_bytes(&self, snapshot: &PdfSnapshot) -> usize {
        let Some(active) = self.active.as_ref() else { return 0; };
        if let Some(position) = active.role_position {
            return if position >= snapshot.images.len() { 2 * size_of::<PdfAdmittedStreamRole>() } else { 0 };
        }
        if active.samples.is_none() { return size_of::<ControlledImageCursor>(); }
        if active.samples.as_ref().is_some_and(ControlledImageCursor::done) {
            size_of::<PdfImage>() + size_of::<PdfAdmittedStreamRole>()
        } else { 0 }
    }

    /// 🔎️ Inspects qualified images and completes their native role prefix under the current grant.
    pub fn advance(&mut self, source: &mut dyn ObjectSource, snapshot: &mut PdfSnapshot, grant: CosGrant) -> Option<CosReceipt> {
        if grant.work == 0 { return Some(sample_receipt(Ok(CosStatus::More), 0)); }
        let active = self.active.as_mut()?;
        let count = snapshot.images.len();
        if let Some(position) = active.role_position {
            if position >= count {
                snapshot.admitted_stream_roles.swap(position - 1, position);
                active.role_position = Some(position - 1);
                return Some(sample_receipt(Ok(CosStatus::More), 1));
            }
            self.active = None;
            return Some(sample_receipt(Ok(CosStatus::Complete { end: count - 1 }), 1));
        }
        let binding = &active.binding;
        if let Some(samples) = active.samples.as_mut() {
            if snapshot.images.capacity() == 0 { return Some(reserve_images(snapshot, grant)); }
            if !samples.done() { return Some(sample_image(source, binding.owner, samples, grant)); }
            let bytes = 2 * binding.length + size_of::<PdfGraphIdentity>() + samples.reference_bytes();
            if bytes > grant.allocation_bytes { return Some(sample_receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0)); }
            if snapshot.admitted_stream_roles.len() == snapshot.admitted_stream_roles.capacity() {
                return Some(sample_receipt(Err("PDF image role exceeds the retained role slots"), 1));
            }
            let [image, role] = samples.take_images(binding.name()).expect("qualified PDF image is fully sampled");
            let identity = PdfGraphIdentity { owner: binding.owner, path: Vec::new() };
            active.role_position = Some(snapshot.admitted_stream_roles.len());
            snapshot.admitted_stream_roles.push(PdfAdmittedStreamRole {
                dependencies: vec![identity.clone()], identity, value: PdfStreamRoleValue::Image { image: role },
            });
            snapshot.images.push(image);
            self.owners[count] = binding.owner;
            active.samples = None;
            return Some(CosReceipt { allocated_bytes: bytes, ..sample_receipt(Ok(CosStatus::More), 1) });
        }
        let index = self.owners[..count].iter().position(|owner| *owner == binding.owner);
        if let Some(index) = index {
            if snapshot.images[index].id != binding.name() { return None; }
            self.active = None;
            return Some(sample_receipt(Ok(CosStatus::Complete { end: index }), 1));
        }
        if count == Self::MAX_COUNT || snapshot.images.iter().any(|image| image.id == binding.name()) { return None; }
        let mut qualified = Ok(None);
        source.inspect(binding.owner, &mut |stream| {
            if let Some(stream) = stream { qualified = ControlledImageCursor::new(stream); }
        });
        match qualified {
            Ok(Some(samples)) => {
                active.samples = Some(samples);
                Some(sample_receipt(Ok(CosStatus::More), 1))
            }
            Ok(None) => None,
            Err(reason) => Some(sample_receipt(Err(reason), 1)),
        }
    }

    /// 🛑️ Parks the unpublished sample backing for retirement.
    pub fn begin_close(&mut self) {
        if let Some(samples) = self.active.as_mut().and_then(|active| active.samples.as_mut()) { samples.begin_close(); }
    }

    /// 🗑️ Releases only the active unpublished backing. Published pairs belong to the snapshot.
    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if let Some(samples) = self.active.as_mut().and_then(|active| active.samples.as_mut()) {
            let step = samples.close_step(grant);
            if step.status != Ok(CosStatus::Closed) { return step; }
        }
        self.active = None;
        sample_receipt(Ok(CosStatus::Closed), 0)
    }
}

fn reserve_images(snapshot: &mut PdfSnapshot, grant: CosGrant) -> CosReceipt {
    let bytes = ControlledImageResources::MAX_COUNT * size_of::<PdfImage>();
    if bytes > grant.allocation_bytes { return sample_receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0); }
    if snapshot.images.try_reserve_exact(ControlledImageResources::MAX_COUNT).is_err() {
        return sample_receipt(Err("unable to allocate PDF image slots"), 1);
    }
    CosReceipt { allocated_bytes: snapshot.images.capacity() * size_of::<PdfImage>(), ..sample_receipt(Ok(CosStatus::More), 1) }
}

fn sample_image(source: &mut dyn ObjectSource, owner: ObjRef, samples: &mut ControlledImageCursor, grant: CosGrant) -> CosReceipt {
    let mut result = sample_receipt(Err("qualified PDF image stream disappeared"), 1);
    source.inspect(owner, &mut |stream| {
        if let Some(PdfObject::Stream { data, .. }) = stream { result = samples.advance(data, grant); }
    });
    if matches!(result.status, Ok(CosStatus::Complete { .. })) { result.status = Ok(CosStatus::More); }
    result
}

//#endregion 🔖️Image resources
