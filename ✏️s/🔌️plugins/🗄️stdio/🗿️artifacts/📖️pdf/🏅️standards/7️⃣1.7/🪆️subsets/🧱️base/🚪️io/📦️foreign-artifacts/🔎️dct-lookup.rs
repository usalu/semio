//! 🔎️ Re-loans an admitted native DCT stream from its revision-pinned PDF graph.
use super::{dct_view::{PdfDctImageColor, PdfDctImageView}, PdfDctResourceIdentity};
use crate::standards::v1_7::subsets::base::schema::{
    snapshot::{ObjRef, PdfColorSpace, PdfImageBody, PdfObject, PdfSnapshot, PdfStreamBytes, PdfStreamFilter},
    stream_roles::PdfStreamRoleValue,
};
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::{paged::PAGED_BYTES_CHUNK_BYTES, ByteReadSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DctLookupError {
    InvalidReference,
    StaleRevision,
    MissingRole,
    MissingObject,
    UnsupportedShape,
    DigestMismatch,
}

#[derive(Clone, Copy)]
struct DctGeometry {
    dimensions: (u16, u16),
    gray: bool,
}

#[derive(Clone, Copy)]
struct DctBinding {
    slot: usize,
    object: ObjRef,
    dimensions: (u16, u16),
    color: PdfDctImageColor,
    length: usize,
}

impl DctBinding {
    fn encoded(self, snapshot: &PdfSnapshot) -> Result<&PdfStreamBytes, DctLookupError> {
        let retained = snapshot.objects.get(self.slot).filter(|retained| retained.id == self.object)
            .ok_or(DctLookupError::MissingObject)?;
        match &retained.value {
            PdfObject::Stream { data, filters, .. }
                if data.len() == self.length && matches!(filters.as_slice(),
                    [PdfStreamFilter::Dct { color_transform }] if *color_transform == self.color.color_transform()) => Ok(data),
            _ => Err(DctLookupError::UnsupportedShape),
        }
    }
}

enum Frontier {
    Role(usize),
    Object { index: usize, owner: ObjRef, geometry: DctGeometry },
    Hash { binding: DctBinding, cursor: PdfDctResourceIdentity },
    Ready(DctBinding),
}

pub struct NativePdfDctLookup {
    revision: [u8; 32],
    digest: [u8; 32],
    frontier: Frontier,
}

impl NativePdfDctLookup {
    /// 🧷️ Captures graph authority and the exact typed reference independently of any snapshot borrow.
    pub fn new(revision: [u8; 32], reference: &ArtifactRef) -> Result<Self, DctLookupError> {
        let digest = PdfDctResourceIdentity::parse_reference(reference).ok_or(DctLookupError::InvalidReference)?;
        Ok(Self { revision, digest, frontier: Frontier::Role(0) })
    }

    /// 🪜️ Observes one role, object row, or at most one page of original DCT bytes.
    pub fn step(
        &mut self,
        revision: [u8; 32],
        snapshot: &PdfSnapshot,
        work: usize,
    ) -> Result<usize, DctLookupError> {
        if revision != self.revision { return Err(DctLookupError::StaleRevision); }
        if work == 0 { return Ok(0); }
        match &mut self.frontier {
            Frontier::Role(index) => {
                let role = snapshot.admitted_stream_roles.get(*index).ok_or(DctLookupError::MissingRole)?;
                *index += 1;
                let PdfStreamRoleValue::Image { image } = &role.value else { return Ok(1); };
                let PdfImageBody::Artifact { reference } = &image.body else { return Ok(1); };
                if !role.identity.path.is_empty() || PdfDctResourceIdentity::parse_reference(reference) != Some(self.digest) {
                    return Ok(1);
                }
                let gray = match &image.color_space {
                    Some(PdfColorSpace::DeviceGray) => true,
                    Some(PdfColorSpace::DeviceRgb) => false,
                    _ => return Err(DctLookupError::UnsupportedShape),
                };
                let dimensions = (u16::try_from(image.width).ok().filter(|width| *width > 0),
                    u16::try_from(image.height).ok().filter(|height| *height > 0));
                let (Some(width), Some(height)) = dimensions else { return Err(DctLookupError::UnsupportedShape); };
                if image.bits_per_component != 8 || image.image_mask {
                    return Err(DctLookupError::UnsupportedShape);
                }
                self.frontier = Frontier::Object {
                    index: 0, owner: role.identity.owner, geometry: DctGeometry { dimensions: (width, height), gray },
                };
                Ok(1)
            }
            Frontier::Object { index, owner, geometry } => {
                let slot = *index;
                let retained = snapshot.objects.get(slot).ok_or(DctLookupError::MissingObject)?;
                *index += 1;
                if retained.id != *owner { return Ok(1); }
                let binding = object_binding(slot, *owner, &retained.value, *geometry)?;
                let cursor = PdfDctResourceIdentity::new(binding.color.color_transform())
                    .ok_or(DctLookupError::UnsupportedShape)?;
                self.frontier = Frontier::Hash { binding, cursor };
                Ok(1)
            }
            Frontier::Hash { binding, cursor } => {
                let data = binding.encoded(snapshot)?;
                let spent = cursor.advance(data, work.min(PAGED_BYTES_CHUNK_BYTES));
                if cursor.complete(binding.length) {
                    if cursor.digest() != self.digest { return Err(DctLookupError::DigestMismatch); }
                    self.frontier = Frontier::Ready(*binding);
                }
                Ok(spent)
            }
            Frontier::Ready(_) => Ok(0),
        }
    }

    /// 🪟️ Borrows only the exact verified native row in the same graph revision.
    pub fn view<'a>(&self, revision: [u8; 32], snapshot: &'a PdfSnapshot) -> Result<Option<PdfDctImageView<'a>>, DctLookupError> {
        if revision != self.revision { return Err(DctLookupError::StaleRevision); }
        let Frontier::Ready(binding) = self.frontier else { return Ok(None); };
        let encoded = binding.encoded(snapshot)?;
        PdfDctImageView::from_admitted(encoded, binding.dimensions, binding.color, self.digest)
            .map(Some).ok_or(DctLookupError::UnsupportedShape)
    }
}

fn object_binding(slot: usize, owner: ObjRef, object: &PdfObject, geometry: DctGeometry) -> Result<DctBinding, DctLookupError> {
    let PdfObject::Stream { dict, data, filters } = object else { return Err(DctLookupError::UnsupportedShape); };
    let [PdfStreamFilter::Dct { color_transform }] = filters.as_slice() else { return Err(DctLookupError::UnsupportedShape); };
    let transform = match color_transform {
        None => None,
        Some(0) => Some(false),
        Some(1) => Some(true),
        _ => return Err(DctLookupError::UnsupportedShape),
    };
    let (width, height) = geometry.dimensions;
    let space = if geometry.gray { "DeviceGray" } else { "DeviceRGB" };
    if !super::qualified_image_dictionary(dict)
        || object.dict_get("Width") != Some(&PdfObject::Int(i64::from(width)))
        || object.dict_get("Height") != Some(&PdfObject::Int(i64::from(height)))
        || object.dict_get("ColorSpace").and_then(PdfObject::as_name) != Some(space)
        || data.len() < 4 || [data.byte_at(0), data.byte_at(1), data.byte_at(data.len() - 2), data.byte_at(data.len() - 1)] != [Some(0xff), Some(0xd8), Some(0xff), Some(0xd9)]
    {
        return Err(DctLookupError::UnsupportedShape);
    }
    let color = if geometry.gray { PdfDctImageColor::Gray { transform } } else { PdfDctImageColor::Rgb { transform } };
    Ok(DctBinding { slot, object: owner, dimensions: geometry.dimensions, color, length: data.len() })
}
