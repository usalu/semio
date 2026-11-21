//! 📚️ Bounded GLB buffer directory and strict virtual reads of validated data URIs.
use super::{SceneResult, allocate, field, rows};
use super::super::{GLTF_BASE64_MARKER, GLTF_DATA_SCHEME, json::Value};
use semio_framework_io_base64::decode_standard_quad;
use semio_framework_value::{RetainedAllocationEffect, retained_clone::RetainedCloneProgress};
use std::mem::size_of;

#[derive(semio_framework_value_derive::RetireOwned)]
enum Slot {
    Embedded,
    Inline { payload: usize, bytes: usize },
    Unavailable,
}

#[derive(Default, semio_framework_value_derive::RetireOwned)]
enum Phase {
    #[default]
    Directory,
    Select,
    MediaType,
    Validate,
}

#[derive(Default, semio_framework_value_derive::RetireOwned)]
pub(super) struct BufferCursor {
    slots: Vec<Slot>,
    phase: Phase,
    cursor: usize,
    payload: usize,
    receipt: [usize; 3],
}

pub(super) enum BufferStep {
    Working,
    Ready,
}

fn uri(declaration: &Value) -> Option<&[u8]> {
    field(declaration, "uri").and_then(Value::as_str).map(str::as_bytes)
}

impl BufferCursor {
    pub(super) fn phase(&self) -> &'static str {
        match self.phase {
            Phase::Directory | Phase::Select => "mesh-glb-buffers",
            Phase::MediaType => "mesh-glb-uri-header",
            Phase::Validate => "mesh-glb-uri-quartets",
        }
    }

    pub(super) fn take_progress(&mut self) -> RetainedCloneProgress {
        let [copied_items, copied_bytes, retained_capacity_bytes] = std::mem::take(&mut self.receipt);
        RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes: 0 }
    }

    pub(super) fn step(&mut self, document: &Value, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<BufferStep> {
        self.receipt = [0; 3];
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0
            || effect.remaining_work_units().map_err(|error| error.to_string())? == 0
        {
            return Ok(BufferStep::Working);
        }
        let declarations = rows(document, "buffers");
        match self.phase {
            Phase::Directory => {
                let capacity = declarations.len().checked_mul(size_of::<Slot>()).ok_or("glb: buffer directory extent overflow")?;
                if grant.maximum_capacity_bytes < capacity { return Ok(BufferStep::Working); }
                if !effect.performed(1).map_err(|error| error.to_string())? { return Ok(BufferStep::Working); }
                self.receipt[0] = 1;
                self.receipt[2] = allocate(&mut self.slots, declarations.len(), effect)?;
                self.phase = Phase::Select;
            }
            _ => {
                if grant.maximum_copy_bytes < size_of::<Slot>() { return Ok(BufferStep::Working); }
                if !effect.performed(1).map_err(|error| error.to_string())? { return Ok(BufferStep::Working); }
                self.receipt[0] = 1;
                let Some(declaration) = declarations.get(self.slots.len()) else { return Ok(BufferStep::Ready); };
                match self.phase {
                    Phase::Select => match uri(declaration) {
                        Some(data) if data.starts_with(GLTF_DATA_SCHEME.as_bytes()) => {
                            self.cursor = GLTF_DATA_SCHEME.len();
                            self.phase = Phase::MediaType;
                        }
                        None if self.slots.is_empty() => self.publish(Slot::Embedded),
                        _ => self.publish(Slot::Unavailable),
                    },
                    Phase::MediaType => self.scan_header(uri(declaration).ok_or("glb: inline buffer declaration changed")?),
                    Phase::Validate => self.validate_quad(uri(declaration).ok_or("glb: inline buffer declaration changed")?),
                    Phase::Directory => unreachable!(),
                }
            }
        }
        Ok(BufferStep::Working)
    }

    fn scan_header(&mut self, uri: &[u8]) {
        let end = self.cursor + GLTF_BASE64_MARKER.len();
        match uri.get(self.cursor..end) {
            Some(marker) if marker == GLTF_BASE64_MARKER.as_bytes() => {
                let digits = uri.len() - end;
                if digits == 0 {
                    self.publish(Slot::Inline { payload: end, bytes: 0 });
                } else if digits % 4 != 0 {
                    self.publish(Slot::Unavailable);
                } else {
                    self.payload = end;
                    self.cursor = end;
                    self.phase = Phase::Validate;
                }
            }
            Some(_) => self.cursor += 1,
            None => self.publish(Slot::Unavailable),
        }
    }

    fn validate_quad(&mut self, uri: &[u8]) {
        let end = self.cursor + 4;
        let last = end == uri.len();
        let offset = self.cursor - self.payload;
        let Some(quad) = uri.get(self.cursor..end).and_then(|bytes| bytes.try_into().ok()) else {
            self.publish(Slot::Unavailable);
            return;
        };
        match decode_standard_quad(quad, offset, last) {
            Ok((_, count)) if last => self.publish(Slot::Inline { payload: self.payload, bytes: offset / 4 * 3 + count }),
            Ok(_) => self.cursor = end,
            Err(_) => self.publish(Slot::Unavailable),
        }
    }

    fn publish(&mut self, slot: Slot) {
        self.slots.push(slot);
        self.receipt[1] = size_of::<Slot>();
        self.phase = Phase::Select;
    }

    pub(super) fn view<'a>(&'a self, document: &'a Value, bin: Option<&'a [u8]>) -> Buffers<'a> {
        Buffers { slots: &self.slots, declarations: rows(document, "buffers"), bin: bin.unwrap_or_default() }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Buffers<'a> {
    slots: &'a [Slot],
    declarations: &'a [Value],
    bin: &'a [u8],
}

impl<'a> Buffers<'a> {
    pub(super) fn buffer(self, index: usize) -> SceneResult<Buffer<'a>> {
        match self.slots.get(index).ok_or("glb: buffer index outside document")? {
            Slot::Embedded => Ok(Buffer::Embedded(self.bin)),
            Slot::Inline { payload, bytes } => {
                let digits = uri(&self.declarations[index]).and_then(|data| data.get(*payload..)).ok_or("glb: inline buffer declaration changed")?;
                Ok(Buffer::Inline { digits, bytes: *bytes })
            }
            Slot::Unavailable => Err("glb: buffer bytes unavailable".into()),
        }
    }
}

pub(super) enum Buffer<'a> {
    Embedded(&'a [u8]),
    Inline { digits: &'a [u8], bytes: usize },
}

impl Buffer<'_> {
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Embedded(bytes) => bytes.len(),
            Self::Inline { bytes, .. } => *bytes,
        }
    }

    pub(super) fn read<'a>(&'a self, offset: usize, length: usize, scratch: &'a mut [u8; 12]) -> SceneResult<&'a [u8]> {
        let row = scratch.get_mut(..length).ok_or("glb: accessor row exceeds sample width")?;
        let end = offset.checked_add(length).filter(|end| *end <= self.len()).ok_or("glb: accessor row outside buffer")?;
        match self {
            Self::Embedded(bytes) => Ok(&bytes[offset..end]),
            Self::Inline { digits, .. } => {
                let mut cursor = offset;
                while cursor < end {
                    let group = cursor / 3 * 4;
                    let pad = cursor % 3;
                    let quad = digits.get(group..group + 4).and_then(|bytes| bytes.try_into().ok())
                        .ok_or("glb: validated inline quartet is missing")?;
                    let (bytes, count) = decode_standard_quad(quad, group, group + 4 == digits.len())
                        .map_err(|error| error.to_string())?;
                    let take = count.checked_sub(pad).filter(|take| *take > 0)
                        .ok_or("glb: validated inline quartet is too short")?.min(end - cursor);
                    row[cursor - offset..cursor - offset + take].copy_from_slice(&bytes[pad..pad + take]);
                    cursor += take;
                }
                Ok(row)
            }
        }
    }
}
