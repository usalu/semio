//! 🔎 Budgeted certification of a single axis-aligned solid in a STEP Part 21 document.

use std::mem::size_of;
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits};

const CHUNK: usize = 64;
const MAX_ROWS: usize = 16_384;
const MAX_ROW: usize = 1024;
const MAX_PROBES: usize = 32;

#[derive(Clone, Copy, Default)]
struct Span { id: u64, start: usize, end: usize }

#[derive(Clone, Copy, Default)]
struct Edge { id: u64, from: u64, to: u64, point: [f64; 3] }

#[derive(Clone, Copy, Default)]
struct Face { axis: usize, positive: bool, edges: [Edge; 4] }

#[derive(Clone, Copy)]
enum Stage { Count, Allocate, Fill, Index, Faces, Verify, Release, Ready }

/// 📦 Exact lower corner and three positive lengths certified from STEP topology.
pub type StepBox = ([f64; 3], [f64; 3]);

/// 📄 Source-borrowing retained STEP box decoder.
pub struct StepBoxDecode {
    source: (usize, usize),
    stage: Stage,
    input: usize,
    statement: usize,
    quoted: bool,
    comment: bool,
    section: u8,
    count: usize,
    table: Vec<Span>,
    solid: u64,
    shell: [u64; 6],
    faces: [Face; 6],
    face: usize,
    result: Option<StepBox>,
}

impl StepBoxDecode {
    /// 📎 Binds the decoder to the immutable original text.
    pub fn new(source: &str) -> Self {
        Self { source: source_key(source), stage: Stage::Count, input: 0, statement: 0, quoted: false, comment: false, section: 0, count: 0, table: Vec::new(), solid: 0, shell: [0; 6], faces: [Face::default(); 6], face: 0, result: None }
    }

    /// 🪙 Quotes the next bounded read, index transition, or release.
    pub fn demand(&self) -> RetirementDemand {
        let (copy_bytes, capacity_bytes, release_bytes) = match self.stage {
            Stage::Count | Stage::Index => ((self.source.1 - self.input).min(CHUNK + 1), 0, 0),
            Stage::Allocate => (0, self.table_len() * size_of::<Span>(), 0),
            Stage::Fill => (size_of::<Span>(), 0, 0),
            Stage::Faces | Stage::Verify => (size_of::<Face>(), 0, 0),
            Stage::Release => (0, 0, self.retained_bytes()),
            Stage::Ready => (size_of::<SolidPrimitive>(), 0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, release_bytes, depth: 1, ..Default::default() }
    }

    /// 🧺 Quotes the full index release when cancelled.
    pub fn close_demand(&self) -> RetirementDemand {
        RetirementDemand { release_bytes: self.retained_bytes(), depth: 1, ..Default::default() }
    }

    /// 🫙 Indicates that no retained index allocation remains.
    pub fn terminal_is_empty(&self) -> bool { self.table.capacity() == 0 }

    /// 🧹 Releases the plain index allocation without traversing authored rows.
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let quote = self.close_demand();
        if !retained_clone_grant_admits(effect.grant(), quote) || !effect.performed(1)? { return Ok(Progress::default()); }
        self.table = Vec::new();
        Ok(Progress { copied_items: 1, released_bytes: quote.release_bytes, ..Default::default() })
    }

    /// ✅ Reports whether a certified box can be read.
    pub fn is_ready(&self) -> bool { matches!(self.stage, Stage::Ready) }

    /// 🧊 Constructs the exact native box recipe from the authored certificate.
    pub fn primitive(&self) -> Option<SolidPrimitive> {
        let (origin, [width, depth, height]) = self.ready()?;
        Some(SolidPrimitive::Box { origin: Pnt3::from_array(origin), width, depth, height })
    }

    /// 📤 Supplies the certificate after decoder storage has been retired.
    pub fn ready(&self) -> Option<StepBox> {
        self.is_ready().then(|| self.result.unwrap())
    }

    /// ⏭️ Advances one bounded source or verification transition.
    pub fn step(&mut self, source: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        if self.source != source_key(source) { return Err(invalid("original STEP source changed")); }
        let quote = self.demand();
        if !retained_clone_grant_admits(effect.grant(), quote) || !effect.performed(1)? { return Ok(Progress::default()); }
        let mut receipt = Progress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() };
        self.advance(source, &mut receipt).map_err(|error| error.with_retained_progress(receipt))?;
        Ok(receipt)
    }

    fn advance(&mut self, source: &str, receipt: &mut Progress) -> Result<(), ValueError> {
        match self.stage {
            Stage::Count | Stage::Index => {
                if self.input == self.source.1 {
                    if self.section != 6 || self.quoted || self.comment || self.statement != self.input {
                        return Err(invalid("STEP exchange envelope is incomplete"));
                    }
                    if matches!(self.stage, Stage::Count) {
                        if self.count == 0 { return Err(unsupported("STEP DATA has no authored instances")); }
                        self.stage = Stage::Allocate;
                    } else {
                        let [_, shell] = fields(self.row(source, self.solid, "MANIFOLD_SOLID_BREP")?)?;
                        let [_, faces] = fields(self.row(source, reference(shell)?, "CLOSED_SHELL")?)?;
                        self.shell = refs(faces)?;
                        self.stage = Stage::Faces;
                    }
                } else if let Some((start, end)) = self.scan(source.as_bytes()) {
                    self.statement(source, start, end)?;
                }
            }
            Stage::Allocate => {
                self.table.try_reserve_exact(self.table_len()).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "STEP instance table could not be allocated"))?;
                receipt.retained_capacity_bytes = self.retained_bytes();
                self.stage = Stage::Fill;
            }
            Stage::Fill => {
                self.table.push(Span::default());
                if self.table.len() == self.table_len() {
                    self.input = 0;
                    self.statement = 0;
                    self.section = 0;
                    self.stage = Stage::Index;
                }
            }
            Stage::Faces => {
                self.faces[self.face] = self.face(source, self.shell[self.face])?;
                self.face += 1;
                if self.face == self.faces.len() { self.stage = Stage::Verify; }
            }
            Stage::Verify => {
                self.result = Some(certify(&self.faces)?);
                self.stage = Stage::Release;
            }
            Stage::Release => {
                receipt.released_bytes = self.retained_bytes();
                self.table = Vec::new();
                self.stage = Stage::Ready;
            }
            Stage::Ready => return Err(invalid("STEP decoder is already complete")),
        }
        Ok(())
    }

    fn scan(&mut self, source: &[u8]) -> Option<(usize, usize)> {
        let limit = (self.input + CHUNK).min(source.len());
        while self.input < limit {
            let at = self.input;
            let byte = source[at];
            let next = source.get(at + 1).copied();
            self.input += 1;
            if self.comment {
                if byte == b'*' && next == Some(b'/') { self.comment = false; self.input += 1; }
            } else if self.quoted {
                if byte == b'\'' {
                    if next == Some(b'\'') { self.input += 1; }
                    else { self.quoted = false; }
                }
            } else if byte == b'\'' {
                self.quoted = true;
            } else if byte == b'/' && next == Some(b'*') {
                self.comment = true;
                self.input += 1;
            } else if byte == b';' {
                let start = std::mem::replace(&mut self.statement, self.input);
                return Some((start, at));
            } else if self.statement == at && byte.is_ascii_whitespace() {
                self.statement = self.input;
            }
        }
        None
    }

    fn statement(&mut self, source: &str, start: usize, end: usize) -> Result<(), ValueError> {
        let row = &source[start..end];
        if row.len() > MAX_ROW { return Err(unsupported("STEP statement exceeds the retained parser limit")); }
        let row = trivia(row)?;
        let marker = ["ISO-10303-21", "HEADER", "ENDSEC", "DATA", "ENDSEC", "END-ISO-10303-21"];
        if self.section == 5 && row.eq_ignore_ascii_case(marker[5]) { self.section = 6; return Ok(()); }
        if self.section < 5 && row.eq_ignore_ascii_case(marker[usize::from(self.section)]) {
            self.section += 1;
            return Ok(());
        }
        match self.section {
            2 => Ok(()),
            4 if row.starts_with('#') => {
                if matches!(self.stage, Stage::Count) {
                    self.count += 1;
                    if self.count > MAX_ROWS { return Err(unsupported("STEP instance count exceeds the retained parser limit")); }
                } else {
                    let (id, body) = row.split_once('=').ok_or_else(|| invalid("STEP instance has no definition"))?;
                    let id = reference(id)?;
                    let body = trivia(body)?;
                    if body.split_once('(').is_some_and(|(kind, _)| kind.trim().eq_ignore_ascii_case("MANIFOLD_SOLID_BREP")) {
                        if self.solid != 0 { return Err(unsupported("STEP import requires one authored manifold solid")); }
                        self.solid = id;
                    }
                    let slot = self.slot(id)?;
                    if self.table[slot].id != 0 { return Err(invalid("STEP instance identifier is repeated")); }
                    self.table[slot] = Span { id, start: start + (body.as_ptr() as usize - source[start..end].as_ptr() as usize), end };
                }
                Ok(())
            }
            _ => Err(invalid("unexpected STEP exchange statement")),
        }
    }

    fn table_len(&self) -> usize { (self.count * 2).next_power_of_two() }
    fn retained_bytes(&self) -> usize { self.table.capacity() * size_of::<Span>() }

    fn slot(&self, id: u64) -> Result<usize, ValueError> {
        let start = id.wrapping_mul(0x9e3779b97f4a7c15);
        let mixed = start ^ (start >> 32);
        for probe in 0..MAX_PROBES.min(self.table.len()) {
            let slot = (mixed as usize).wrapping_add(probe) & (self.table.len() - 1);
            if self.table[slot].id == 0 || self.table[slot].id == id { return Ok(slot); }
        }
        Err(unsupported("STEP instance index probe limit was exceeded"))
    }

    fn row<'a>(&self, source: &'a str, id: u64, kind: &str) -> Result<&'a str, ValueError> {
        if id == 0 { return Err(unsupported("STEP import requires one authored manifold solid")); }
        let span = self.table[self.slot(id)?];
        if span.id != id { return Err(invalid("STEP topology references an absent instance")); }
        let (actual, attributes) = source[span.start..span.end].split_once('(').ok_or_else(|| invalid("STEP instance has no attributes"))?;
        let actual = actual.trim();
        if !actual.eq_ignore_ascii_case(kind) && !(kind == "FACE_OUTER_BOUND" && actual.eq_ignore_ascii_case("FACE_BOUND")) {
            return Err(unsupported("STEP entity is outside the planar cuboid subset"));
        }
        attributes.trim_end().strip_suffix(')').ok_or_else(|| invalid("STEP instance has an invalid attribute list"))
    }

    fn face(&self, source: &str, id: u64) -> Result<Face, ValueError> {
        let [_, bounds, surface, same] = fields(self.row(source, id, "ADVANCED_FACE")?)?;
        let [bound] = refs(bounds)?;
        let [_, ring, forward] = fields(self.row(source, bound, "FACE_OUTER_BOUND")?)?;
        let [_, edges] = fields(self.row(source, reference(ring)?, "EDGE_LOOP")?)?;
        let [_, axis] = fields(self.row(source, reference(surface)?, "PLANE")?)?;
        let [_, origin, normal, tangent] = fields(self.row(source, reference(axis)?, "AXIS2_PLACEMENT_3D")?)?;
        let plane = self.vector(source, reference(origin)?, "CARTESIAN_POINT")?;
        let direction = if normal == "$" { [0.0, 0.0, 1.0] } else { self.vector(source, reference(normal)?, "DIRECTION")? };
        let axis = only_axis(direction).ok_or_else(|| unsupported("STEP face is not axis aligned"))?;
        if tangent != "$" {
            let tangent = self.vector(source, reference(tangent)?, "DIRECTION")?;
            if tangent[axis] != 0.0 || tangent == [0.0; 3] { return Err(invalid("STEP plane reference direction is not tangent")); }
        }
        let positive = (direction[axis] > 0.0) == boolean(same)?;
        let forward = boolean(forward)?;
        let mut ring = [Edge::default(); 4];
        for (index, edge) in refs::<4>(edges)?.into_iter().enumerate() {
            ring[index] = self.edge(source, edge)?;
        }
        if !forward {
            ring.reverse();
            for edge in &mut ring {
                std::mem::swap(&mut edge.from, &mut edge.to);
                edge.point = self.vertex(source, edge.from)?;
            }
        }
        for (index, edge) in ring.iter().enumerate() {
            if edge.to != ring[(index + 1) % 4].from || edge.point[axis] != plane[axis] {
                return Err(invalid("STEP bound is discontinuous or leaves its authored plane"));
            }
        }
        let i = (axis + 1) % 3;
        let j = (axis + 2) % 3;
        let [a, b, c, _] = ring.map(|edge| edge.point);
        let cross = (b[i] - a[i]) * (c[j] - b[j]) - (b[j] - a[j]) * (c[i] - b[i]);
        if !cross.is_finite() || cross == 0.0 || (cross > 0.0) != positive {
            return Err(invalid("STEP face and outer bound orientations disagree"));
        }
        Ok(Face { axis, positive, edges: ring })
    }

    fn edge(&self, source: &str, id: u64) -> Result<Edge, ValueError> {
        let [_, start, end, edge, forward] = fields(self.row(source, id, "ORIENTED_EDGE")?)?;
        if start != "*" || end != "*" { return Err(unsupported("STEP oriented edge requires derived endpoint attributes")); }
        let edge = reference(edge)?;
        let [_, from, to, curve, same] = fields(self.row(source, edge, "EDGE_CURVE")?)?;
        let (from, to) = (reference(from)?, reference(to)?);
        let (a, b) = (self.vertex(source, from)?, self.vertex(source, to)?);
        let delta = std::array::from_fn(|axis| b[axis] - a[axis]);
        let axis = only_axis(delta).ok_or_else(|| unsupported("STEP trim is not axis aligned"))?;
        let curve = reference(curve)?;
        let line = match self.row(source, curve, "SURFACE_CURVE") {
            Ok(attributes) => {
                let [_, curve, _, _] = fields(attributes)?;
                reference(curve)?
            }
            Err(_) => curve,
        };
        let [_, origin, vector] = fields(self.row(source, line, "LINE")?)?;
        let origin = self.vector(source, reference(origin)?, "CARTESIAN_POINT")?;
        let [_, direction, length] = fields(self.row(source, reference(vector)?, "VECTOR")?)?;
        let direction = self.vector(source, reference(direction)?, "DIRECTION")?;
        if !(number(length)? > 0.0 && only_axis(direction) == Some(axis)
            && (0..3).all(|i| i == axis || origin[i] == a[i])
            && ((direction[axis] > 0.0) == (delta[axis] > 0.0)) == boolean(same)?) {
            return Err(invalid("STEP LINE does not support its authored edge trim"));
        }
        if boolean(forward)? { Ok(Edge { id: edge, from, to, point: a }) }
        else { Ok(Edge { id: edge, from: to, to: from, point: b }) }
    }

    fn vertex(&self, source: &str, id: u64) -> Result<[f64; 3], ValueError> {
        let [_, point] = fields(self.row(source, id, "VERTEX_POINT")?)?;
        self.vector(source, reference(point)?, "CARTESIAN_POINT")
    }

    fn vector(&self, source: &str, id: u64, kind: &str) -> Result<[f64; 3], ValueError> {
        let [_, coords] = fields(self.row(source, id, kind)?)?;
        let [x, y, z] = fields(list(coords)?)?;
        Ok([number(x)?, number(y)?, number(z)?])
    }
}

fn source_key(source: &str) -> (usize, usize) { (source.as_ptr() as usize, source.len()) }
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
fn unsupported(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::UnsupportedOwner, message) }

fn trivia(mut text: &str) -> Result<&str, ValueError> {
    loop {
        let Some(rest) = text.trim_start().strip_prefix("/*") else { return Ok(text.trim()); };
        text = rest.split_once("*/").ok_or_else(|| invalid("STEP comment is unterminated"))?.1;
    }
}

fn only_axis(values: [f64; 3]) -> Option<usize> {
    let axis = values.iter().position(|value| *value != 0.0)?;
    values.iter().enumerate().all(|(index, value)| value.is_finite() && (index == axis || *value == 0.0)).then_some(axis)
}

fn list(text: &str) -> Result<&str, ValueError> {
    text.strip_prefix('(').and_then(|text| text.strip_suffix(')')).ok_or_else(|| invalid("STEP aggregate is malformed"))
}

fn refs<const N: usize>(text: &str) -> Result<[u64; N], ValueError> {
    let mut refs = [0; N];
    for (id, token) in refs.iter_mut().zip(fields::<N>(list(text)?)?) { *id = reference(token)?; }
    Ok(refs)
}

fn reference(text: &str) -> Result<u64, ValueError> {
    text.trim().strip_prefix('#').and_then(|text| text.parse().ok()).filter(|id| *id != 0)
        .ok_or_else(|| invalid("STEP instance reference is malformed"))
}

fn boolean(text: &str) -> Result<bool, ValueError> {
    if text.eq_ignore_ascii_case(".T.") { Ok(true) }
    else if text.eq_ignore_ascii_case(".F.") { Ok(false) }
    else { Err(invalid("STEP logical attribute is malformed")) }
}

fn number(text: &str) -> Result<f64, ValueError> {
    text.parse::<f64>().ok().filter(|v| v.is_finite()).ok_or_else(|| invalid("STEP coordinate is not a finite real"))
}

fn fields<const N: usize>(text: &str) -> Result<[&str; N], ValueError> {
    let mut result = [""; N];
    let (mut next, mut start, mut depth, mut quoted) = (0, 0, 0_i32, false);
    for (index, byte) in text.bytes().enumerate() {
        match (byte, quoted) {
            (b'\'', _) => quoted = !quoted,
            (b'(', false) => depth += 1,
            (b')', false) => depth -= 1,
            (b',', false) if depth == 0 => {
                *result.get_mut(next).ok_or_else(|| unsupported("STEP aggregate has an unsupported arity"))? = trivia(&text[start..index])?;
                next += 1;
                start = index + 1;
            }
            _ => {}
        }
        if depth < 0 { return Err(invalid("STEP aggregate is unbalanced")); }
    }
    if next + 1 != N { return Err(unsupported("STEP aggregate has an unsupported arity")); }
    if quoted || depth != 0 { return Err(invalid("STEP attributes are unbalanced")); }
    result[next] = trivia(&text[start..])?;
    Ok(result)
}

fn certify(faces: &[Face; 6]) -> Result<StepBox, ValueError> {
    let mut sides = 0_u8;
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for face in faces {
        let bit = 1 << (2 * face.axis + usize::from(face.positive));
        if sides & bit != 0 { return Err(unsupported("STEP solid does not have six different box sides")); }
        sides |= bit;
        for edge in face.edges {
            for axis in 0..3 {
                min[axis] = min[axis].min(edge.point[axis]);
                max[axis] = max[axis].max(edge.point[axis]);
            }
        }
    }
    let lengths = std::array::from_fn(|axis| max[axis] - min[axis]);
    if !(0..3).all(|axis| lengths[axis].is_finite() && lengths[axis] > 0.0 && min[axis] + lengths[axis] == max[axis]) { return Err(invalid("STEP box extents are degenerate")); }
    let mut corners = [0_u64; 8];
    for face in faces {
        let mut used = 0_u8;
        for edge in face.edges {
            let mut corner = 0;
            for axis in 0..3 {
                if edge.point[axis] == max[axis] { corner |= 1 << axis; }
                else if edge.point[axis] != min[axis] { return Err(unsupported("STEP face contains a vertex between box corners")); }
            }
            if ((corner >> face.axis) & 1 != 0) != face.positive || used & (1 << corner) != 0 {
                return Err(invalid("STEP box face is reversed or repeats a corner"));
            }
            if corners[corner] != 0 && corners[corner] != edge.from { return Err(invalid("STEP box has unshared coincident vertices")); }
            used |= 1 << corner;
            corners[corner] = edge.from;
            let opposite = faces.iter().flat_map(|face| face.edges).filter(|other| other.id == edge.id && other.from == edge.to && other.to == edge.from).count();
            let incidence = faces.iter().flat_map(|face| face.edges).filter(|other| other.id == edge.id).count();
            if opposite != 1 || incidence != 2 { return Err(invalid("STEP box does not have shared opposite coedges")); }
        }
    }
    Ok((min, lengths))
}
