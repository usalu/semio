//! 📤️ Budgeted OBJ and base64 STL output over one owned triangle mesh.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueRefusalKind, retained_clone::retained_clone_grant_admits};
use std::fmt::Write;

type Progress = RetainedCloneProgress;
const OUTPUT_LIMIT: usize = 64 << 20;
const LETTERS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 💾️ Registered triangle exchange outputs.
#[derive(Clone, Copy)]
pub enum MeshTextFormat { Obj, Stl }
semio_framework_value::artifact_retire_leaf!(MeshTextFormat);

impl MeshTextFormat {
    pub fn channel(self) -> &'static str { match self { Self::Obj => "obj", Self::Stl => "stl" } }
}

#[derive(Clone, Copy)]
enum Phase { Count, Reserve, Record, Finish, Ready, Taken }
semio_framework_value::artifact_retire_leaf!(Phase);

struct Line { bytes: [u8; 96], len: usize }
impl Line {
    fn new() -> Self { Self { bytes: [0; 96], len: 0 } }
    fn text(&self) -> &str { std::str::from_utf8(&self.bytes[..self.len]).expect("mesh text is ASCII") }
}
impl Write for Line {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len + text.len();
        self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?.copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// 🧾️ Produces bounded text records and holds all buffers for paid retirement.
pub struct MeshTextExport {
    mesh: MeshData,
    format: MeshTextFormat,
    phase: Phase,
    record: usize,
    expected: usize,
    output: String,
    carry: [u8; 3],
    carry_len: usize,
}
semio_framework_value::artifact_retire_struct!(MeshTextExport {mesh,format,phase,record,expected,output,carry,carry_len});

impl MeshTextExport {
    /// 🎟️ Admits an owned mesh header without losing its backing on refusal.
    pub fn new(mesh: &mut Option<MeshData>, format: MeshTextFormat) -> Result<Self, ValueError> {
        let source = mesh.as_ref().ok_or_else(invalid)?;
        let count = source.indices.len() / 3;
        if source.positions.len() < 9 || source.positions.len() % 3 != 0 || count == 0 || source.indices.len() % 3 != 0 {
            return Err(invalid());
        }
        let (phase, expected) = match format {
            MeshTextFormat::Obj => (Phase::Count, 0),
            MeshTextFormat::Stl => (Phase::Reserve, stl_length(count)?),
        };
        Ok(Self { mesh: mesh.take().unwrap(), format, phase, record: 0, expected, output: String::new(), carry: [0; 3], carry_len: 0 })
    }

    pub fn ready(&self) -> bool { matches!(self.phase, Phase::Ready) }
    pub fn take_output(&mut self) -> Option<String> {
        if !self.ready() { return None; }
        self.phase = Phase::Taken;
        Some(std::mem::take(&mut self.output))
    }

    /// 📏️ Quotes one line, facet, final base64 quantum or exact output reservation.
    pub fn demand(&self) -> RetirementDemand {
        let (copy_bytes, capacity_bytes) = match (self.format, self.phase) {
            (_, Phase::Reserve) => (0, self.expected),
            (_, Phase::Ready) => (size_of::<String>(), 0),
            (_, Phase::Taken) => (0, 0),
            (MeshTextFormat::Obj, _) => (96 + 3 * size_of::<u32>(), 0),
            (MeshTextFormat::Stl, Phase::Record) if self.record == 0 => (112, 0),
            (MeshTextFormat::Stl, Phase::Record) => (68 + 9 * size_of::<f32>() + 3 * size_of::<u32>(), 0),
            (MeshTextFormat::Stl, _) => (4, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    /// ⏱️ Writes or counts one original mesh record per retained work event.
    pub fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand();
        if matches!(self.phase, Phase::Ready | Phase::Taken) || grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let mut receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        self.advance(&mut receipt).map_err(|error| error.with_retained_progress(receipt))?;
        if !receipt.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "mesh export output exceeded its grant").with_retained_progress(receipt));
        }
        Ok(receipt)
    }

    fn advance(&mut self, receipt: &mut Progress) -> Result<(), ValueError> {
        match self.phase {
            Phase::Reserve => {
                self.output.try_reserve_exact(self.expected).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mesh export text allocation failed"))?;
                receipt.retained_capacity_bytes = self.output.capacity();
                self.record = 0;
                self.phase = Phase::Record;
            }
            Phase::Count => {
                if let Some(line) = self.obj_record()? {
                    self.expected = self.expected.checked_add(line.len).filter(|&length| length <= OUTPUT_LIMIT).ok_or_else(invalid)?;
                    self.record += 1;
                } else { self.phase = Phase::Reserve; }
            }
            Phase::Record => match self.format {
                MeshTextFormat::Obj => {
                    if let Some(line) = self.obj_record()? {
                        if self.output.len() + line.len > self.expected { return Err(invalid()); }
                        self.output.push_str(line.text());
                        self.record += 1;
                    } else { self.phase = Phase::Finish; }
                }
                MeshTextFormat::Stl => self.stl_record()?,
            },
            Phase::Finish => {
                if self.carry_len > 0 { self.write_quartet(self.carry_len)?; }
                if self.output.len() != self.expected { return Err(ValueError::invariant("mesh export length changed after admission")); }
                self.phase = Phase::Ready;
            }
            Phase::Ready | Phase::Taken => unreachable!("terminal mesh text is transferred separately"),
        }
        Ok(())
    }

    fn triangle(&self, index: usize) -> Result<Option<[u32; 3]>, ValueError> {
        let Some(indices) = self.mesh.indices.as_chunks::<3>().0.get(index).copied() else { return Ok(None); };
        if indices.into_iter().any(|index| index as usize >= self.mesh.positions.len() / 3 || index == u32::MAX) { return Err(invalid()); }
        Ok(Some(indices))
    }

    fn obj_record(&self) -> Result<Option<Line>, ValueError> {
        let mut line = Line::new();
        let count = self.mesh.positions.len() / 3;
        let has_normals = self.mesh.normals.len() == self.mesh.positions.len();
        if self.record == 0 {
            line.write_str("o mesh\n").map_err(|_| invalid())?;
        } else {
            let index = self.record - 1;
            if index < count {
                vector_line(&mut line, "v", &self.mesh.positions, index)?;
            } else if has_normals && index - count < count {
                vector_line(&mut line, "vn", &self.mesh.normals, index - count)?;
            } else {
                let start = count * (1 + usize::from(has_normals));
                let Some(indices) = self.triangle(index - start)? else { return Ok(None); };
                let [a, b, c] = indices.map(|index| index + 1);
                if has_normals { writeln!(line, "f {a}//{a} {b}//{b} {c}//{c}").map_err(|_| invalid())?; }
                else { writeln!(line, "f {a} {b} {c}").map_err(|_| invalid())?; }
            }
        }
        Ok(Some(line))
    }

    fn stl_record(&mut self) -> Result<(), ValueError> {
        if self.record == 0 {
            let mut header = [0; 84];
            header[80..].copy_from_slice(&((self.mesh.indices.len() / 3) as u32).to_le_bytes());
            self.write_binary(&header)?;
        } else if let Some(triangle) = self.triangle(self.record - 1)? {
            let [a, b, c] = triangle.map(|index| self.mesh.positions.as_chunks::<3>().0[index as usize]);
            if ![a, b, c].into_iter().flatten().all(f32::is_finite) { return Err(invalid()); }
            let [pa, pb, pc] = [a, b, c].map(|point| Pnt3::from_array(point.map(f64::from)));
            let normal = (pb - pa).cross(pc - pa).normalized().unwrap_or(Vec3::ZERO).to_array()
                .map(|component| if component == 0.0 { 0.0 } else { component as f32 });
            let mut bytes = [0; 50];
            for (target, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip([normal, a, b, c].into_iter().flatten()) {
                *target = value.to_le_bytes();
            }
            self.write_binary(&bytes)?;
        } else {
            self.phase = Phase::Finish;
            return Ok(());
        }
        self.record += 1;
        Ok(())
    }

    fn write_binary(&mut self, bytes: &[u8]) -> Result<(), ValueError> {
        for &byte in bytes {
            self.carry[self.carry_len] = byte;
            self.carry_len += 1;
            if self.carry_len == 3 { self.write_quartet(3)?; }
        }
        Ok(())
    }

    fn write_quartet(&mut self, count: usize) -> Result<(), ValueError> {
        if self.output.len() + 4 > self.expected { return Err(invalid()); }
        let [a, b, c] = self.carry;
        let encoded = [LETTERS[(a >> 2) as usize], LETTERS[((a & 3) << 4 | b >> 4) as usize],
            if count >= 2 { LETTERS[((b & 15) << 2 | c >> 6) as usize] } else { b'=' },
            if count == 3 { LETTERS[(c & 63) as usize] } else { b'=' }];
        self.output.push_str(std::str::from_utf8(&encoded).expect("base64 is ASCII"));
        self.carry = [0; 3];
        self.carry_len = 0;
        Ok(())
    }
}

fn stl_length(triangles: usize) -> Result<usize, ValueError> {
    triangles.checked_mul(50).and_then(|size| size.checked_add(84))
        .and_then(|size| size.div_ceil(3).checked_mul(4)).filter(|&size| size <= OUTPUT_LIMIT).ok_or_else(invalid)
}

fn vector_line(line: &mut Line, tag: &str, values: &[f32], index: usize) -> Result<(), ValueError> {
    let [x, y, z] = values.as_chunks::<3>().0[index];
    if ![x, y, z].into_iter().all(f32::is_finite) { return Err(invalid()); }
    writeln!(line, "{tag} {x:e} {y:e} {z:e}").map_err(|_| invalid())
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "mesh text export requires a finite indexed triangle mesh within its output bounds")
}
