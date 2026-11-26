//! 🎞️ Admits Image XObjects and their independent native stream roles.

use super::{dict_get, dict_name, PdfDictEntry, PdfImage, PdfImagePackedCursor, PdfObject, PdfStreamFilter};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{
    INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES, PdfDctResourceIdentity, PdfSampledResourceIdentity, encoded_sampled_image, qualified_image_dictionary,
};
use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosReceipt, CosStatus, grow_table, table_growth};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfImageBody, PdfStreamBytes};
use semio_framework_value::ByteReadSource;
use super::sample_receipt;

const DEPTH: u32 = 8;

enum ImageBacking {
    Samples { cursors: [PdfImagePackedCursor; 2], next: usize },
    Dct { identity: PdfDctResourceIdentity, bytes: usize },
    SampledArtifact { identity: PdfSampledResourceIdentity, bytes: usize },
}

/// 🎬️ Resolves sampled or encoded native content into two independently owned image roles.
pub(super) struct ControlledImageCursor {
    width: u32,
    height: u32,
    gray: bool,
    interpolate: bool,
    soft_mask: Option<ObjRef>,
    backing: ImageBacking,
}

impl ControlledImageCursor {
    /// 🔍️ Qualifies a typed native stream without acquiring sample backing.
    pub(super) fn new(value: &PdfObject) -> Result<Option<Self>, &'static str> {
        let PdfObject::Stream { dict, data, filters } = value else { return Ok(None); };
        if filters.is_empty() && encoded_sampled_image(dict) {
            let dimension = |key| match dict_get(dict, key) { Some(PdfObject::Int(raw)) => u32::try_from(*raw).ok().filter(|size| *size != 0), _ => None };
            let (Some(width), Some(height), Some(identity)) = (dimension("Width"), dimension("Height"), PdfSampledResourceIdentity::encoded(dict)) else { return Ok(None); };
            return Ok(Some(Self {
                width, height, gray: dict_name(dict, "ColorSpace") == Some("DeviceGray"),
                interpolate: dict_get(dict, "Interpolate").and_then(PdfObject::as_bool).unwrap_or(false),
                soft_mask: dict_get(dict, "SMask").and_then(PdfObject::as_ref),
                backing: ImageBacking::SampledArtifact { identity, bytes: data.len() },
            }));
        }
        if !matches!(filters.as_slice(), [] | [PdfStreamFilter::Flate { .. } | PdfStreamFilter::Dct { .. }]) || !qualified_image_dictionary(dict) { return Ok(None); }
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
        let soft_mask = dict_get(dict, "SMask").and_then(PdfObject::as_ref);
        let backing = match filters.as_slice() {
            [] | [PdfStreamFilter::Flate { .. }] => {
                let components = if gray { 1 } else { 3 };
                let sample_cursor = || PdfImagePackedCursor::checked_new(width, height, components, DEPTH);
                let first = sample_cursor()?;
                if first.packed_length != data.len() { return Err("image packed body does not match its declared geometry"); }
                if first.packed_length > INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES {
                    let Some(identity) = PdfSampledResourceIdentity::new(filters) else { return Ok(None); };
                    ImageBacking::SampledArtifact { identity, bytes: data.len() }
                } else {
                    ImageBacking::Samples { cursors: [first, sample_cursor()?], next: 0 }
                }
            }
            [PdfStreamFilter::Dct { color_transform }] => {
                let length = data.len();
                if length < 2 || [data.byte_at(0), data.byte_at(1), data.byte_at(length - 2), data.byte_at(length - 1)] != [Some(0xff), Some(0xd8), Some(0xff), Some(0xd9)] { return Ok(None); }
                let Some(identity) = PdfDctResourceIdentity::new(*color_transform) else { return Ok(None); };
                ImageBacking::Dct { identity, bytes: data.len() }
            }
            _ => return Ok(None),
        };
        Ok(Some(Self { width, height, gray, interpolate, soft_mask, backing }))
    }

    fn done(&self) -> bool {
        match &self.backing {
            ImageBacking::Samples { cursors, next } => *next == cursors.len(),
            ImageBacking::Dct { identity, bytes } => identity.complete(*bytes),
            ImageBacking::SampledArtifact { identity, bytes } => identity.complete(*bytes),
        }
    }

    fn reference_bytes(&self) -> usize {
        match &self.backing {
            ImageBacking::Samples { .. } => 0,
            ImageBacking::Dct { .. } => 2 * PdfDctResourceIdentity::REFERENCE_BYTES,
            ImageBacking::SampledArtifact { .. } => 2 * PdfSampledResourceIdentity::REFERENCE_BYTES,
        }
    }

    /// 📐️ Distinguishes allocation of empty sample capacity from writes into that capacity.
    pub(super) fn reserving_samples(&self) -> bool {
        match &self.backing {
            ImageBacking::Samples { cursors, next } => cursors.get(*next).is_some_and(|cursor| cursor.output.capacity() == 0),
            ImageBacking::Dct { .. } | ImageBacking::SampledArtifact { .. } => false,
        }
    }

    /// ▶️ Advances one lane while borrowing native stream bytes from the graph.
    pub(super) fn advance(&mut self, data: &PdfStreamBytes, grant: CosGrant) -> CosReceipt {
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
            ImageBacking::SampledArtifact { identity, bytes } => {
                if *bytes != data.len() { return sample_receipt(Err("qualified sampled image stream changed extent"), 1); }
                let work = identity.advance(data, grant.work);
                sample_receipt(Ok(CosStatus::More), work)
            }
        }
    }

    /// 🫴️ Transfers both role bodies after backing and identifier work is fully admitted.
    pub(super) fn take_images(&mut self, id: &str, soft_mask: Option<&str>) -> Option<[PdfImage; 2]> {
        if !self.done() { return None; }
        let bodies = match &mut self.backing {
            ImageBacking::Samples { cursors: [first, second], .. } => {
                [first.take_packed()?, second.take_packed().expect("complete replayed PDF image")]
                    .map(|packed| PdfImageBody::Samples { packed })
            }
            ImageBacking::Dct { identity, .. } => {
                let reference = identity.reference();
                [reference.clone(), reference].map(|reference| PdfImageBody::Artifact { reference })
            }
            ImageBacking::SampledArtifact { identity, .. } => {
                let reference = identity.reference();
                [reference.clone(), reference].map(|reference| PdfImageBody::Artifact { reference })
            }
        };
        Some(bodies.map(|body| {
            let color_space = if self.gray { PdfColorSpace::DeviceGray } else { PdfColorSpace::DeviceRgb };
            let mut image = PdfImage::packed(id, self.width, self.height, color_space, DEPTH, Vec::new());
            image.interpolate = self.interpolate;
            image.soft_mask = soft_mask.map(str::to_owned);
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
    /// 📃️ Binds one authored XObject resource name to its exact indirect native object.
    pub fn new(owner: ObjRef, authored: &[u8]) -> Option<Self> {
        let length = authored.len();
        if length == 0 || length > MAX_NAME { return None; }
        let mut name = [0; MAX_NAME];
        name[..length].copy_from_slice(authored);
        Some(Self { owner, name, length })
    }

    fn name(&self) -> &str { std::str::from_utf8(&self.name[..self.length]).expect("authored PDF image name") }
}

/// 🎭️ The soft mask an image names: its owner and typed id, and whether this admission still lifts it after the image (a mask typed before is only referenced).
struct SoftMask { owner: ObjRef, name: [u8; MAX_NAME], length: usize, fresh: bool }

struct ImageAdmission {
    binding: ControlledImageBinding,
    samples: Option<ControlledImageCursor>,
    role_position: Option<usize>,
    role_target: usize,
    mask: Option<SoftMask>,
    image: Option<usize>,
}

/// 🪟️ Coordinates page bindings and the independently owned image roles of one native lift: the owner of every typed image (in image order) and the images the active page binds grow under quotes, so a document or page may bind any number of images.
#[derive(Default)]
pub struct ControlledImageResources {
    owners: Vec<ObjRef>,
    active: Option<ImageAdmission>,
    page_images: Vec<u32>,
}

impl ControlledImageResources {
    /// 🪪️ The native owner of the image typed in `slot`.
    pub fn owner(&self, slot: usize) -> ObjRef { self.owners[slot] }

    /// 📏️ The growth the owner table needs before it can adopt `count` images typed outside this table.
    pub fn adoption_growth(&self, count: usize) -> usize { table_growth(&self.owners, count) }

    /// 🪢️ Adopts an image typed outside this table (the reference builder, inside a form or an appearance) as the next image slot after [`Self::adoption_growth`] was granted; false when the allocator refuses.
    pub fn adopt(&mut self, owner: ObjRef) -> bool {
        if !grow_table(&mut self.owners, 1) { return false; }
        self.owners.push(owner);
        true
    }

    /// 🚪️ Gives up the active admission before it acquired any backing, returning the image's owner so the reference builder can lift it.
    pub fn abandon(&mut self) -> Option<ObjRef> { self.active.take().map(|active| active.binding.owner) }

    /// 📑️ Starts a new page with no bound image resources; the page table keeps its capacity.
    pub fn clear_page(&mut self) { self.page_images.clear(); }

    /// 📏️ The growth the page table needs before it can bind one more image (zero when it has room).
    pub fn page_growth(&self) -> usize { table_growth(&self.page_images, 1) }

    /// 📥️ Grows the page table exactly as [`Self::page_growth`] quoted; false when the allocator refuses.
    pub fn grow_page(&mut self) -> bool { grow_table(&mut self.page_images, 1) }

    /// 📌️ Binds one admitted image slot to the active page; false when the table has no room left ([`Self::grow_page`] first).
    pub fn push_page(&mut self, slot: usize) -> bool {
        let Ok(slot) = u32::try_from(slot) else { return false; };
        if self.page_images.len() == self.page_images.capacity() { return false; }
        self.page_images.push(slot);
        true
    }

    /// 🔗️ Reports whether the active page binds an image whose typed ID is the authored XObject name.
    pub fn page_contains(&self, snapshot: &PdfSnapshot, name: &str) -> bool {
        self.page_images.iter().any(|slot| snapshot.images[*slot as usize].id == name)
    }

    /// 📦️ The bytes the owner and page tables hold.
    pub fn held_bytes(&self) -> usize { self.owners.capacity() * size_of::<ObjRef>() + self.page_images.capacity() * size_of::<u32>() }

    /// 🧺️ Drops both tables once the lift credited [`Self::held_bytes`].
    pub fn release(&mut self) {
        self.owners = Vec::new();
        self.page_images = Vec::new();
    }

    /// 🌱 Starts admission of a resource inspected through its exact page or branch dictionary.
    pub fn begin(&mut self, binding: ControlledImageBinding) {
        self.active = Some(ImageAdmission { binding, samples: None, role_position: None, role_target: 0, mask: None, image: None });
    }

    /// 📦️ Quotes empty sample backing separately from writes.
    pub fn reserves_empty(&self) -> bool {
        self.active.as_ref().and_then(|active| active.samples.as_ref()).is_some_and(ControlledImageCursor::reserving_samples)
    }

    /// 🏃️ Whether the next unit moves image bytes, one per unit of its grant: copying packed samples into reserved backing or hashing an encoded body.
    pub fn copies_bytes(&self) -> bool {
        self.active.as_ref().and_then(|active| active.samples.as_ref()).is_some_and(|samples| !samples.done() && !samples.reserving_samples())
    }

    /// 📨️ Quotes the inline typed values that the next work unit can write or move.
    pub fn inline_body_bytes(&self, snapshot: &PdfSnapshot) -> usize {
        let Some(active) = self.active.as_ref() else { return 0; };
        if let Some(position) = active.role_position {
            return if position > active.role_target { 2 * size_of::<PdfAdmittedStreamRole>() } else { 0 };
        }
        if active.samples.is_none() { return size_of::<ControlledImageCursor>(); }
        if active.samples.as_ref().is_some_and(ControlledImageCursor::done) {
            size_of::<PdfImage>() + size_of::<PdfAdmittedStreamRole>()
        } else { 0 }
    }

    /// 🔎️ Inspects qualified images and completes their native role prefix under the current grant.
    pub fn advance(&mut self, source: &mut dyn ObjectSource, snapshot: &mut PdfSnapshot, grant: CosGrant, content_roles: usize) -> Option<CosReceipt> {
        if grant.work == 0 { return Some(sample_receipt(Ok(CosStatus::More), 0)); }
        let active = self.active.as_mut()?;
        let count = snapshot.images.len();
        if let Some(position) = active.role_position {
            if position > active.role_target {
                snapshot.admitted_stream_roles.swap(position - 1, position);
                active.role_position = Some(position - 1);
                return Some(sample_receipt(Ok(CosStatus::More), 1));
            }
            if let Some(mask) = active.mask.take().filter(|mask| mask.fresh) {
                active.image.get_or_insert(count - 1);
                active.binding = ControlledImageBinding { owner: mask.owner, name: mask.name, length: mask.length };
                active.role_position = None;
                return Some(sample_receipt(Ok(CosStatus::More), 1));
            }
            let end = active.image.unwrap_or(count - 1);
            self.active = None;
            return Some(sample_receipt(Ok(CosStatus::Complete { end }), 1));
        }
        let binding = &active.binding;
        if let Some(samples) = active.samples.as_mut() {
            if !samples.done() { return Some(sample_image(source, binding.owner, samples, grant)); }
            let bytes = 2 * binding.length + 2 * size_of::<PdfGraphIdentity>() + samples.reference_bytes()
                + table_growth(&snapshot.images, 1) + table_growth(&snapshot.admitted_stream_roles, 1) + table_growth(&self.owners, 1);
            if bytes > grant.allocation_bytes { return Some(sample_receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0)); }
            if !grow_table(&mut snapshot.images, 1) || !grow_table(&mut snapshot.admitted_stream_roles, 1) || !grow_table(&mut self.owners, 1) {
                return Some(sample_receipt(Err("unable to grow the PDF image tables"), 1));
            }
            let mask = active.mask.as_ref().map(|mask| std::str::from_utf8(&mask.name[..mask.length]).expect("authored PDF soft mask name"));
            let [image, role] = samples.take_images(binding.name(), mask).expect("qualified PDF image is fully sampled");
            let identity = PdfGraphIdentity { owner: binding.owner, path: Vec::new() };
            let dependencies = vec![identity.clone()];
            active.role_position = Some(snapshot.admitted_stream_roles.len());
            active.role_target = snapshot.admitted_stream_roles.len() - content_roles;
            snapshot.admitted_stream_roles.push(PdfAdmittedStreamRole {
                dependencies, identity, value: PdfStreamRoleValue::Image { image: Box::new(role) },
            });
            snapshot.images.push(image);
            self.owners.push(binding.owner);
            active.samples = None;
            return Some(CosReceipt { allocated_bytes: bytes, ..sample_receipt(Ok(CosStatus::More), 1) });
        }
        let index = self.owners[..count].iter().position(|owner| *owner == binding.owner);
        if let Some(index) = index {
            if snapshot.images[index].id != binding.name() { return None; }
            self.active = None;
            return Some(sample_receipt(Ok(CosStatus::Complete { end: index }), 1));
        }
        if self.owners.len() != count || snapshot.images.iter().any(|image| image.id == binding.name()) { return None; }
        let mut qualified = Ok(None);
        source.inspect(binding.owner, &mut |stream| {
            if let Some(stream) = stream { qualified = ControlledImageCursor::new(stream); }
        });
        match qualified {
            Ok(Some(samples)) => {
                if let Some(owner) = samples.soft_mask {
                    if active.image.is_some() { return None; }
                    let mask = match self.owners[..count].iter().position(|existing| *existing == owner) {
                        Some(index) => {
                            let id = snapshot.images[index].id.as_bytes();
                            let mut name = [0; MAX_NAME];
                            name[..id.len()].copy_from_slice(id);
                            SoftMask { owner, name, length: id.len(), fresh: false }
                        }
                        None => {
                            let length = binding.length + "SMask".len();
                            if length > MAX_NAME { return None; }
                            let mut name = [0; MAX_NAME];
                            name[..binding.length].copy_from_slice(&binding.name[..binding.length]);
                            name[binding.length..length].copy_from_slice(b"SMask");
                            let id = std::str::from_utf8(&name[..length]).expect("authored PDF soft mask name");
                            if snapshot.images.iter().any(|image| image.id == id) { return None; }
                            SoftMask { owner, name, length, fresh: true }
                        }
                    };
                    active.mask = Some(mask);
                }
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

fn sample_image(source: &mut dyn ObjectSource, owner: ObjRef, samples: &mut ControlledImageCursor, grant: CosGrant) -> CosReceipt {
    let mut result = sample_receipt(Err("qualified PDF image stream disappeared"), 1);
    source.inspect(owner, &mut |stream| {
        if let Some(PdfObject::Stream { data, .. }) = stream { result = samples.advance(data, grant); }
    });
    if matches!(result.status, Ok(CosStatus::Complete { .. })) { result.status = Ok(CosStatus::More); }
    result
}

//#endregion 🔖️Image resources
