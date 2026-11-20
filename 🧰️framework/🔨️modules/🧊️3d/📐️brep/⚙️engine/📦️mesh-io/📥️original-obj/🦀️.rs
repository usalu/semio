//! 📥️ Retained OBJ decoding over borrowed text and bounded indexed triangle buffers.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueRefusalKind, retained_clone::retained_clone_grant_admits};
use std::mem::{size_of, take};

const MAX_VERTICES: usize = 600_000;
const MAX_TRIANGLES: usize = 100_000;
const MAX_TOKEN: usize = 64;
const SCAN_BYTES: usize = 48;

#[derive(Clone, Copy)]
enum Row {
    Start,
    Ignore,
    Vertex { xyz: [f32; 3], next: usize },
    Face { first: u32, last: u32, corners: usize },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pass { Count, Positions, Indices, Write, Ready, Taken }

/// 🧾️ Counts and emits one borrowed OBJ token or row transition per retained event.
pub struct ObjDecode {
    pass: Pass,
    offset: usize,
    token: Option<usize>,
    row: Row,
    source_address: usize,
    source_length: usize,
    tolerance: f64,
    vertex_count: usize,
    triangle_count: usize,
    seen_vertices: usize,
    positions: Vec<f32>,
    indices: Vec<u32>,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "OBJ requires valid finite vertices and indices within the triangle import bounds")
}

impl ObjDecode {
    /// 🎟️ Opens a bounded decoder before borrowing or copying document tokens.
    pub fn new(source: &str, tolerance: f64) -> Result<Self, ValueError> {
        if source.is_empty() || !tolerance.is_finite() || tolerance <= 0.0 { return Err(invalid()); }
        Ok(Self { pass: Pass::Count, offset: 0, token: None, row: Row::Start,
            source_address: source.as_ptr() as usize, source_length: source.len(), tolerance,
            vertex_count: 0, triangle_count: 0, seen_vertices: 0, positions: Vec::new(), indices: Vec::new() })
    }

    pub fn ready(&self) -> bool { self.pass == Pass::Ready }

    /// 📏️ Quotes one text chunk or one exact buffer reservation.
    pub fn demand(&self) -> RetirementDemand {
        let (copy_bytes, capacity_bytes) = match self.pass {
            Pass::Positions => (0, 3 * self.vertex_count * size_of::<f32>()),
            Pass::Indices => (0, 3 * self.triangle_count * size_of::<u32>()),
            Pass::Count | Pass::Write => (SCAN_BYTES + MAX_TOKEN + 3 * size_of::<u32>(), 0),
            Pass::Ready => (size_of::<(Vec<f32>, Vec<u32>)>(), 0),
            _ => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    /// ⏱️ Visits one bounded token, row terminator, or typed allocation stage.
    pub fn step(&mut self, source: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if source.as_ptr() as usize != self.source_address || source.len() != self.source_length {
            return Err(ValueError::invariant("original OBJ source changed"));
        }
        let grant = effect.grant();
        let demand = self.demand();
        if matches!(self.pass, Pass::Ready | Pass::Taken) || grant.maximum_items == 0
            || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(RetainedCloneProgress::default());
        }
        let mut receipt = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let allocation = || ValueError::literal(ValueRefusalKind::AllocationFailed, "OBJ typed buffer allocation failed");
        match self.pass {
            Pass::Count | Pass::Write => {
                receipt.copied_bytes = demand.copy_bytes;
                self.scan(source).map_err(|error| error.with_retained_progress(receipt))?;
            }
            Pass::Positions => {
                self.positions.try_reserve_exact(3 * self.vertex_count).map_err(|_| allocation().with_retained_progress(receipt))?;
                receipt.retained_capacity_bytes = self.positions.capacity() * size_of::<f32>();
                self.pass = Pass::Indices;
            }
            Pass::Indices => {
                self.indices.try_reserve_exact(3 * self.triangle_count).map_err(|_| allocation().with_retained_progress(receipt))?;
                receipt.retained_capacity_bytes = self.indices.capacity() * size_of::<u32>();
                self.offset = 0;
                self.pass = Pass::Write;
            }
            _ => unreachable!("ready OBJ is transferred separately"),
        }
        if !receipt.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "OBJ buffer reservation exceeded grant").with_retained_progress(receipt));
        }
        Ok(receipt)
    }

    fn scan(&mut self, source: &str) -> Result<(), ValueError> {
        let start = self.offset;
        for _ in 0..SCAN_BYTES {
            let byte = source.as_bytes().get(self.offset).copied();
            if let Some(token) = self.token {
                if byte.is_none_or(|byte| byte.is_ascii_whitespace() || byte == b'#') {
                    self.token = None;
                    return self.word(source.get(token..self.offset).ok_or_else(invalid)?);
                }
                if self.offset - token >= MAX_TOKEN { return Err(invalid()); }
            } else if byte.is_none_or(|byte| byte == b'\n') {
                self.end_row()?;
                if byte.is_none() { return self.end_pass(); }
                self.offset += 1;
                return Ok(());
            } else if byte == Some(b'#') {
                self.end_row()?;
                self.row = Row::Ignore;
            } else if !matches!(self.row, Row::Ignore) && byte.is_some_and(|byte| !byte.is_ascii_whitespace()) {
                self.token = Some(self.offset);
            }
            self.offset += 1;
        }
        debug_assert_eq!(self.offset - start, SCAN_BYTES);
        Ok(())
    }

    fn word(&mut self, word: &str) -> Result<(), ValueError> {
        match &mut self.row {
            Row::Start => {
                self.row = match word {
                    "v" => Row::Vertex { xyz: [0.; 3], next: 0 },
                    "f" => Row::Face { first: 0, last: 0, corners: 0 },
                    _ => Row::Ignore,
                };
            }
            Row::Vertex { xyz, next } => {
                if let Some(axis) = xyz.get_mut(*next) {
                    *axis = word.parse().ok().filter(|value: &f32| value.is_finite()).ok_or_else(invalid)?;
                    *next += 1;
                }
            }
            Row::Face { first, last, corners } => {
                let raw: i64 = word.split('/').next().and_then(|index| index.parse().ok()).ok_or_else(invalid)?;
                let index = match raw {
                    1.. => raw - 1,
                    ..=-1 => (self.seen_vertices as i64).checked_add(raw).ok_or_else(invalid)?,
                    0 => return Err(invalid()),
                };
                let index = u32::try_from(index).ok().filter(|&index| index < MAX_VERTICES as u32).ok_or_else(invalid)?;
                if self.pass == Pass::Write && index as usize >= self.vertex_count { return Err(invalid()); }
                if *corners >= 2 {
                    if self.pass == Pass::Count {
                        self.triangle_count += 1;
                        if self.triangle_count > MAX_TRIANGLES { return Err(invalid()); }
                    } else {
                        if self.indices.len() == 3 * self.triangle_count { return Err(invalid()); }
                        self.indices.extend_from_slice(&[*first, *last, index]);
                    }
                }
                if *corners == 0 { *first = index; }
                *last = index;
                *corners += 1;
            }
            Row::Ignore => {}
        }
        Ok(())
    }

    fn end_row(&mut self) -> Result<(), ValueError> {
        match std::mem::replace(&mut self.row, Row::Start) {
            Row::Vertex { xyz, next: 3 } => {
                self.seen_vertices += 1;
                if self.seen_vertices > MAX_VERTICES { return Err(invalid()); }
                if self.pass == Pass::Write {
                    if self.seen_vertices > self.vertex_count { return Err(invalid()); }
                    self.positions.extend_from_slice(&xyz);
                }
            }
            Row::Vertex { .. } => return Err(invalid()),
            _ => {}
        }
        Ok(())
    }

    fn end_pass(&mut self) -> Result<(), ValueError> {
        if self.pass == Pass::Count {
            self.vertex_count = self.seen_vertices;
            if self.vertex_count == 0 || self.triangle_count == 0 { return Err(invalid()); }
            self.seen_vertices = 0;
            self.pass = Pass::Positions;
        } else {
            if self.positions.len() != self.vertex_count * 3 || self.indices.len() != self.triangle_count * 3 { return Err(invalid()); }
            self.pass = Pass::Ready;
        }
        Ok(())
    }

    /// 🫴️ Transfers both filled typed buffers without cloning their entries.
    pub fn into_cursor(&mut self) -> Result<MeshImportCursor, ValueError> {
        if !self.ready() { return Err(ValueError::invariant("original OBJ decoder has not completed")); }
        self.pass = Pass::Taken;
        MeshImportCursor::new(take(&mut self.positions), Vec::new(), take(&mut self.indices), self.tolerance)
            .map_err(|_| ValueError::invariant("decoded OBJ counts changed before transfer"))
    }

    pub fn terminal_is_empty(&self) -> bool { self.positions.capacity() == 0 && self.indices.capacity() == 0 }

    /// 🧹️ Quotes one untransferred typed buffer.
    pub fn close_demand(&self) -> RetirementDemand {
        let release_bytes = if self.positions.capacity() != 0 { self.positions.capacity() * size_of::<f32>() }
            else { self.indices.capacity() * size_of::<u32>() };
        RetirementDemand { release_bytes, depth: 1, ..Default::default() }
    }

    /// ♻️ Releases one typed buffer before native mutation has taken custody.
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.close_demand();
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if self.positions.capacity() != 0 { drop(take(&mut self.positions)); }
        else { drop(take(&mut self.indices)); }
        let receipt = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }
}
