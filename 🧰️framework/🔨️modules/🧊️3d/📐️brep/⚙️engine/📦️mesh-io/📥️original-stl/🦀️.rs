//! 📥️ Bounded STL decoding directly from original base64 text.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueRefusalKind, retained_clone::retained_clone_grant_admits};
use std::mem::{size_of, take};

type Progress = RetainedCloneProgress;
const MAX_VERTICES: usize = 300_000;
const SCAN: usize = 16;
const TEXT_SCAN: usize = 8;
const TOKEN: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Leading, Trailing, Validate, CountAscii, Reserve(usize), Binary, Ascii, Ready, Taken }

#[derive(Clone, Copy)]
enum Row { Start, Facet, Ignore, Values { normal: bool, count: usize, axes: [f64; 3] } }

struct Bytes {
    offset: usize,
    cache_group: usize,
    cache: [u8; 3],
}

struct Ascii {
    bytes: Bytes,
    row: Row,
    token: [u8; TOKEN],
    token_len: usize,
    normal: Vec3,
}

impl Default for Ascii {
    fn default() -> Self {
        Self { bytes: Bytes::at(0), row: Row::Start, token: [0; TOKEN], token_len: 0, normal: Vec3::Z }
    }
}

/// 🧾️ Validates an original base64 source and emits typed binary or ASCII STL vertices.
pub struct StlDecode {
    phase: Phase,
    address: usize,
    length: usize,
    span: std::ops::Range<usize>,
    tolerance: f64,
    group: usize,
    decoded_length: usize,
    header: [u8; 84],
    ascii: Ascii,
    is_ascii: bool,
    vertices: usize,
    mesh: TriangleMesh,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "STL requires canonical base64 with finite complete binary or ASCII facets")
}

impl StlDecode {
    /// 🎟️ Opens a decoder bound to one borrowed text owner.
    pub fn new(source: &str, tolerance: f64) -> Result<Self, ValueError> {
        if source.is_empty() || !tolerance.is_finite() || tolerance <= 0.0 { return Err(invalid()); }
        Ok(Self { phase: Phase::Leading, address: source.as_ptr() as usize, length: source.len(), span: 0..source.len(),
            tolerance, group: 0, decoded_length: 0, header: [0; 84], ascii: Ascii::default(), is_ascii: false,
            vertices: 0, mesh: TriangleMesh::default() })
    }

    pub fn ready(&self) -> bool { self.phase == Phase::Ready }

    fn backing_bytes(&self, index: usize) -> usize {
        match index {
            0 => self.mesh.positions.capacity() * size_of::<Pnt3>(),
            1 => self.mesh.normals.capacity() * size_of::<Vec3>(),
            _ => self.mesh.indices.capacity() * size_of::<u32>(),
        }
    }

    /// 📏️ Quotes a bounded borrowed byte window, typed allocation or buffer-header transfer.
    pub fn demand(&self) -> RetirementDemand {
        let vertex_bytes = size_of::<Pnt3>() + size_of::<Vec3>() + size_of::<u32>();
        let (copy_bytes, capacity_bytes) = match self.phase {
            Phase::Leading | Phase::Trailing => (4, 0),
            Phase::Validate => (SCAN * 7, 0),
            Phase::CountAscii | Phase::Ascii => (TOKEN + vertex_bytes + 12, 0),
            Phase::Binary => (36 + vertex_bytes, 0),
            Phase::Reserve(index) => (0, self.vertices * [size_of::<Pnt3>(), size_of::<Vec3>(), size_of::<u32>()][index]),
            Phase::Ready => (size_of::<TriangleMesh>(), 0),
            Phase::Taken => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    /// ⏱️ Spends one event on text validation, an STL token or vertex, or typed reservation.
    pub fn step(&mut self, source: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        if source.as_ptr() as usize != self.address || source.len() != self.length {
            return Err(ValueError::invariant("original STL source changed"));
        }
        let grant = effect.grant();
        let demand = self.demand();
        if matches!(self.phase, Phase::Ready | Phase::Taken) || grant.maximum_items == 0
            || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok(Progress::default()); }
        let mut receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        self.advance(source, &mut receipt).map_err(|error| error.with_retained_progress(receipt))?;
        if !receipt.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "STL typed buffer exceeded grant").with_retained_progress(receipt));
        }
        Ok(receipt)
    }

    fn advance(&mut self, source: &str, receipt: &mut Progress) -> Result<(), ValueError> {
        let text = &source[self.span.clone()];
        match self.phase {
            Phase::Leading | Phase::Trailing => {
                let leading = self.phase == Phase::Leading;
                let ch = (if leading { text.chars().next() } else { text.chars().next_back() }).ok_or_else(invalid)?;
                if ch.is_whitespace() {
                    if leading { self.span.start += ch.len_utf8(); } else { self.span.end -= ch.len_utf8(); }
                } else if leading {
                    self.phase = Phase::Trailing;
                } else {
                    if text.len() % 4 != 0 { return Err(invalid()); }
                    self.phase = Phase::Validate;
                }
            }
            Phase::Validate => self.validate(text.as_bytes())?,
            Phase::CountAscii | Phase::Ascii => {
                let counting = self.phase == Phase::CountAscii;
                let (done, vertex) = self.ascii.scan(text.as_bytes(), self.decoded_length)?;
                if let Some((position, normal)) = vertex {
                    if counting {
                        self.vertices += 1;
                        if self.vertices > MAX_VERTICES { return Err(invalid()); }
                    } else { self.push(position, normal)?; }
                }
                if done {
                    if counting { self.start_reserve()?; }
                    else if self.mesh.indices.len() == self.vertices { self.phase = Phase::Ready; }
                    else { return Err(invalid()); }
                }
            }
            Phase::Reserve(index) => {
                let count = self.vertices;
                match index {
                    0 => self.mesh.positions.try_reserve_exact(count),
                    1 => self.mesh.normals.try_reserve_exact(count),
                    _ => self.mesh.indices.try_reserve_exact(count),
                }.map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "STL typed buffer allocation failed"))?;
                receipt.retained_capacity_bytes = self.backing_bytes(index);
                self.phase = if index < 2 { Phase::Reserve(index + 1) }
                    else if self.is_ascii { self.ascii = Ascii::default(); Phase::Ascii } else { Phase::Binary };
            }
            Phase::Binary => {
                let index = self.mesh.indices.len();
                let facet = 84 + (index / 3) * 50;
                let normal = binary_vector(text.as_bytes(), facet)?;
                let position = binary_vector(text.as_bytes(), facet + 12 + index % 3 * 12)?;
                self.push(Pnt3::from_array(position), Vec3::from_array(normal))?;
                if self.mesh.indices.len() == self.vertices { self.phase = Phase::Ready; }
            }
            _ => unreachable!("completed STL transfers separately"),
        }
        Ok(())
    }

    fn validate(&mut self, bytes: &[u8]) -> Result<(), ValueError> {
        let groups = bytes.len() / 4;
        for _ in 0..SCAN {
            if self.group == groups {
                let prefix = self.decoded_length >= 5 && self.header[..5].eq_ignore_ascii_case(b"solid");
                let faces = u32::from_le_bytes(self.header[80..84].try_into().unwrap());
                let expected = 84 + u64::from(faces) * 50;
                self.is_ascii = prefix && expected != self.decoded_length as u64;
                if self.is_ascii { self.phase = Phase::CountAscii; }
                else {
                    if expected > self.decoded_length as u64 || faces > (MAX_VERTICES / 3) as u32 { return Err(invalid()); }
                    self.vertices = faces as usize * 3;
                    self.start_reserve()?;
                }
                return Ok(());
            }
            let (decoded, length) = quartet(bytes, self.group)?;
            for &byte in &decoded[..length] {
                if let Some(slot) = self.header.get_mut(self.decoded_length) { *slot = byte; }
                self.decoded_length += 1;
            }
            self.group += 1;
        }
        Ok(())
    }

    fn start_reserve(&mut self) -> Result<(), ValueError> {
        if self.vertices == 0 || self.vertices % 3 != 0 || self.vertices > MAX_VERTICES { return Err(invalid()); }
        self.phase = Phase::Reserve(0);
        Ok(())
    }

    fn push(&mut self, position: Pnt3, normal: Vec3) -> Result<(), ValueError> {
        let index = self.mesh.indices.len();
        if index == self.vertices || !position.to_array().into_iter().chain(normal.to_array()).all(f64::is_finite) { return Err(invalid()); }
        self.mesh.positions.push(position);
        self.mesh.normals.push(normal);
        self.mesh.indices.push(index as u32);
        Ok(())
    }

    /// 🫴️ Transfers the three reserved typed buffers without cloning their contents.
    pub fn into_cursor(&mut self) -> Result<MeshImportCursor, ValueError> {
        if !self.ready() { return Err(ValueError::invariant("original STL decoder has not completed")); }
        self.phase = Phase::Taken;
        MeshImportCursor::from_triangle_mesh(take(&mut self.mesh), self.tolerance)
            .map_err(|_| ValueError::invariant("decoded STL counts changed before transfer"))
    }

    pub fn terminal_is_empty(&self) -> bool { (0..3).all(|index| self.backing_bytes(index) == 0) }

    /// 🧹️ Quotes one untransferred STL buffer's retirement.
    pub fn close_demand(&self) -> RetirementDemand {
        RetirementDemand { depth: 1, release_bytes: (0..3).map(|index| self.backing_bytes(index)).find(|&bytes| bytes > 0).unwrap_or(0), ..Default::default() }
    }

    /// ♻️ Retires one typed buffer while leaving the original text with its owner.
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.close_demand();
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(RetainedCloneStep::Progress(Progress::default()));
        }
        if self.backing_bytes(0) != 0 { drop(take(&mut self.mesh.positions)); }
        else if self.backing_bytes(1) != 0 { drop(take(&mut self.mesh.normals)); }
        else { drop(take(&mut self.mesh.indices)); }
        let receipt = Progress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }
}

fn sextet(byte: u8) -> Result<u8, ValueError> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(invalid()),
    }
}

fn quartet(bytes: &[u8], index: usize) -> Result<([u8; 3], usize), ValueError> {
    let offset = index * 4;
    let [a, b, c, d] = bytes.get(offset..offset + 4).and_then(|row| <[u8; 4]>::try_from(row).ok()).ok_or_else(invalid)?;
    let (a, b) = (sextet(a)?, sextet(b)?);
    let first = a << 2 | b >> 4;
    if c == b'=' {
        if offset + 4 != bytes.len() || d != b'=' || b & 15 != 0 { return Err(invalid()); }
        return Ok(([first, 0, 0], 1));
    }
    let c = sextet(c)?;
    let second = b << 4 | c >> 2;
    if d == b'=' {
        if offset + 4 != bytes.len() || c & 3 != 0 { return Err(invalid()); }
        return Ok(([first, second, 0], 2));
    }
    Ok(([first, second, c << 6 | sextet(d)?], 3))
}

fn binary_vector(source: &[u8], offset: usize) -> Result<[f64; 3], ValueError> {
    let mut reader = Bytes::at(offset);
    let mut axes = [0.0; 3];
    for axis in &mut axes {
        let mut bytes = [0; 4];
        for byte in &mut bytes { *byte = reader.byte(source)?; }
        *axis = f64::from(f32::from_le_bytes(bytes));
    }
    Ok(axes)
}

impl Bytes {
    fn at(offset: usize) -> Self { Self { offset, cache_group: usize::MAX, cache: [0; 3] } }

    fn byte(&mut self, source: &[u8]) -> Result<u8, ValueError> {
        let group = self.offset / 3;
        if group != self.cache_group {
            self.cache = quartet(source, group)?.0;
            self.cache_group = group;
        }
        let value = self.cache[self.offset % 3];
        self.offset += 1;
        Ok(value)
    }

    fn scalar(&mut self, source: &[u8], length: usize) -> Result<char, ValueError> {
        let first = self.byte(source)?;
        if first.is_ascii() { return Ok(first as char); }
        let width = match first {
            0xc2..=0xdf => 2,
            0xe0..=0xef => 3,
            0xf0..=0xf4 => 4,
            _ => return Err(invalid()),
        };
        if self.offset + width - 1 > length { return Err(invalid()); }
        let mut bytes = [first; 4];
        for byte in &mut bytes[1..width] { *byte = self.byte(source)?; }
        std::str::from_utf8(&bytes[..width]).ok().and_then(|value| value.chars().next()).ok_or_else(invalid)
    }
}

impl Ascii {
    fn scan(&mut self, source: &[u8], length: usize) -> Result<(bool, Option<(Pnt3, Vec3)>), ValueError> {
        for _ in 0..TEXT_SCAN {
            let done = self.bytes.offset == length;
            let ch = if done { '\n' } else { self.bytes.scalar(source, length)? };
            if ch.is_whitespace() {
                if self.token_len != 0 {
                    self.word()?;
                    if ch != '\n' { return Ok((false, None)); }
                }
                if ch == '\n' { return Ok((done, self.end_row()?)); }
            } else if !matches!(self.row, Row::Ignore) {
                let mut utf8 = [0; 4];
                let encoded = ch.encode_utf8(&mut utf8).as_bytes();
                let end = self.token_len + encoded.len();
                self.token.get_mut(self.token_len..end).ok_or_else(invalid)?.copy_from_slice(encoded);
                self.token_len = end;
            }
        }
        Ok((false, None))
    }

    fn word(&mut self) -> Result<(), ValueError> {
        let token = std::str::from_utf8(&self.token[..take(&mut self.token_len)]).map_err(|_| invalid())?;
        let values = |normal| Row::Values { normal, count: 0, axes: [0.0; 3] };
        match &mut self.row {
            Row::Start => self.row = match token { "vertex" => values(false), "facet" => Row::Facet, _ => Row::Ignore },
            Row::Facet => self.row = if token == "normal" { values(true) } else { Row::Ignore },
            Row::Values { count, axes, .. } => {
                if let Some(axis) = axes.get_mut(*count) {
                    *axis = token.parse().ok().filter(|value: &f64| value.is_finite()).ok_or_else(invalid)?;
                    *count += 1;
                }
            }
            Row::Ignore => {}
        }
        Ok(())
    }

    fn end_row(&mut self) -> Result<Option<(Pnt3, Vec3)>, ValueError> {
        match std::mem::replace(&mut self.row, Row::Start) {
            Row::Values { normal: true, count: 3, axes } => { self.normal = Vec3::from_array(axes); Ok(None) }
            Row::Values { normal: false, count: 3, axes } => Ok(Some((Pnt3::from_array(axes), self.normal))),
            Row::Values { .. } => Err(invalid()),
            _ => Ok(None),
        }
    }
}
