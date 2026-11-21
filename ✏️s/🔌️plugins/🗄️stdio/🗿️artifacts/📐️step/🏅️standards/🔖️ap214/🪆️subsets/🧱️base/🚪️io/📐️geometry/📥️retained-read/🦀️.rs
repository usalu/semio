//! 🔎 Budgeted certification of a single authored planar solid in a STEP Part 21 document.

use std::mem::size_of;
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};

#[path = "🔺️prism-proof.rs"]
mod prism_proof;
#[path = "🧊️orthogonal-proof.rs"]
mod orthogonal_proof;
use orthogonal_proof::OrthogonalProof;
#[path = "🛢️cylinder-proof.rs"]
mod cylinder_proof;
use cylinder_proof::CylinderFace;
#[path = "〰️spline-proof.rs"]
mod spline_proof;
use spline_proof::Chart;
#[path = "🟠️rounded-proof.rs"]
mod rounded_proof;
use rounded_proof::RoundedProof;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits};

const CHUNK: usize = 64;
const MAX_ROWS: usize = 16_384;
const MAX_ROW: usize = 1024;
const MAX_PROBES: usize = 32;
const MAX_FACES: usize = 64;

#[derive(Clone, Copy, Default)]
struct Span { id: u64, start: usize, end: usize }

#[derive(Clone, Copy, Default)]
struct Edge { id: u64, from: u64, to: u64, point: [f64; 3] }

#[derive(Clone, Copy, Default)]
struct Face { origin: [f64; 3], normal: [f64; 3], edges: [Edge; 4], count: usize }

#[derive(Clone, Copy)]
struct Placement { origin: [f64; 3], axis: [f64; 3], radial: Option<[f64; 3]> }

struct Bound { surface: u64, same: bool, forward: bool, members: [u64; 4], count: usize }

#[derive(Clone, Copy)]
struct AuthoredCoedge { edge: u64, from: u64, to: u64, curve: u64, same: bool, forward: bool }

#[derive(Clone, Copy)]
enum Stage { Count, Allocate, Fill, Index, ClassifyShell, RoundedHeaders, RoundedFaces, Faces, Verify, VerifySplineFace, OrthogonalFace, OrthogonalRay, OrthogonalFinish, Release, Ready }

/// 📦 Exact lower corner and three positive lengths certified from STEP topology.
pub type StepBox = ([f64; 3], [f64; 3]);

/// 🧱 Exact solid geometry certified against all authored STEP boundary instances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StepSolid {
    Box(StepBox),
    NurbsCuboid(StepBox),
    Cylinder { base: [f64; 3], axis: [f64; 3], radial: [f64; 3], radius: f64, height: f64 },
    TriangularPrism { cap: [[f64; 3]; 3], extrusion: [f64; 3] },
    Orthogonal { axes: [[f64; 4]; 3], cells: u32 },
    RoundedBox { origin: [f64;3], size: [f64;3], radius: f64 },
}

impl StepSolid {
    /// 🧊 Converts a certified analytic STEP solid into its native construction recipe.
    pub fn analytic_primitive(self) -> Option<SolidPrimitive> {
        match self {
            Self::Box(bounds) => Some(cuboid_primitive(bounds)),
            Self::Cylinder { base, axis, radial, radius, height } => {
                let frame = Frame3::from_x_z(Pnt3::from_array(base), Vec3::from_array(radial), Vec3::from_array(axis))?;
                Some(SolidPrimitive::Cylinder { frame, radius, height })
            }
            _ => None,
        }
    }

    /// 〰️ Recovers the cuboid dimensions with the original spline encoding.
    pub fn nurbs_cuboid(self) -> Option<SolidPrimitive> {
        let Self::NurbsCuboid(bounds) = self else { return None; };
        Some(cuboid_primitive(bounds))
    }

    /// 🟠️ Recovers a certified shell with plane, quarter-cylinder and octant faces.
    pub fn rounded_box(self) -> Option<semio_framework_3d_brep_representation::contract::RoundedBox> {
        let Self::RoundedBox { origin,size,radius } = self else { return None; };
        rounded_proof::recipe(origin,size,radius)
    }

    /// 🔻 Converts a certified triangular STEP shell into an exact native prism.
    pub fn prism_boundary(self) -> Option<semio_framework_3d_brep_construction::prism_boundary::PrismBoundary> {
        let Self::TriangularPrism { cap, extrusion } = self else { return None; };
        prism_proof::prism_recipe(cap, Vec3::from_array(extrusion))
    }
}

/// 📄 Source-borrowing retained STEP solid decoder.
pub struct StepSolidDecode {
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
    shell: [u64; MAX_FACES],
    face_count: usize,
    faces: [Face; MAX_FACES],
    orthogonal: OrthogonalProof,
    rounded: Option<RoundedProof>,
    cylinder: [CylinderFace; 3],
    spline: bool,
    charts: [Chart; 6],
    face: usize,
    result: Option<StepSolid>,
}

impl StepSolidDecode {
    /// 📎 Binds the decoder to the immutable original text.
    pub fn new(source: &str) -> Self {
        Self { source: source_key(source), stage: Stage::Count, input: 0, statement: 0, quoted: false, comment: false, section: 0, count: 0, table: Vec::new(), solid: 0, shell: [0; MAX_FACES], face_count: 0, faces: [Face::default(); MAX_FACES], orthogonal: OrthogonalProof::new(), rounded: None, cylinder: [CylinderFace::Empty; 3], spline: false, charts: [Chart::default(); 6], face: 0, result: None }
    }

    /// 🪙 Quotes the next bounded read, index transition, or release.
    pub fn demand(&self) -> RetirementDemand {
        let (copy_bytes, capacity_bytes, release_bytes) = match self.stage {
            Stage::Count | Stage::Index => ((self.source.1 - self.input).min(CHUNK + 1), 0, 0),
            Stage::Allocate => (0, self.table_len() * size_of::<Span>(), 0),
            Stage::Fill => (size_of::<Span>(), 0, 0),
            Stage::ClassifyShell => (size_of::<u64>(), 0, 0),
            Stage::RoundedHeaders | Stage::RoundedFaces => (2*size_of::<RoundedProof>()+384,0,0),
            Stage::Faces | Stage::Verify if self.face_count == 3 => (size_of::<CylinderFace>(), 0, 0),
            Stage::Faces if self.spline => (size_of::<Face>() + size_of::<Chart>(), 0, 0),
            Stage::VerifySplineFace => (size_of::<Face>() + size_of::<Chart>(), 0, 0),
            Stage::Faces | Stage::Verify | Stage::OrthogonalFace => (size_of::<Face>(), 0, 0),
            Stage::OrthogonalRay | Stage::OrthogonalFinish => (size_of::<StepSolid>(), 0, 0),
            Stage::Release => (0, 0, self.retained_bytes()),
            Stage::Ready => (size_of::<StepSolid>().max(size_of::<SolidPrimitive>()), 0, 0),
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

    /// ✅ Reports whether a certified solid can be read.
    pub fn is_ready(&self) -> bool { matches!(self.stage, Stage::Ready) }

    /// 📤 Supplies the certificate after decoder storage has been retired.
    pub fn ready(&self) -> Option<StepSolid> {
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
                        self.face_count = list_references(faces, &mut self.shell)?;
                        if self.face_count < 3 || self.face_count == 4 { return Err(unsupported("STEP import requires a supported closed planar shell")); }
                        self.stage = match self.face_count {
                            6 => Stage::ClassifyShell,
                            26 => { self.rounded = Some(RoundedProof::new()); Stage::RoundedHeaders },
                            _ => Stage::Faces,
                        };
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
            Stage::ClassifyShell => {
                let [_, _, support, _] = fields(self.row(source, self.shell[0], "ADVANCED_FACE")?)?;
                self.spline = self.row(source, reference(support)?, "B_SPLINE_SURFACE_WITH_KNOTS").is_ok();
                self.stage = Stage::Faces;
            }
            Stage::RoundedHeaders | Stage::RoundedFaces => {
                let mut proof = self.rounded.take().ok_or_else(|| invalid("STEP rounded box proof is missing"))?;
                let id = self.shell[self.face];
                if matches!(self.stage,Stage::RoundedHeaders) { proof.observe(self,source,id)?; }
                else { proof.verify(self,source,id,self.result.and_then(StepSolid::rounded_box).ok_or_else(|| invalid("STEP rounded box shape is missing"))?)?; }
                self.face += 1;
                if self.face == self.face_count {
                    self.face = 0;
                    if matches!(self.stage,Stage::RoundedHeaders) {
                        self.result = proof.recipe()?;
                        self.stage = if self.result.is_some() { Stage::RoundedFaces } else { Stage::Faces };
                    } else { proof.complete()?; self.stage = Stage::Release; }
                }
                self.rounded = Some(proof);
            }
            Stage::Faces => {
                let id = self.shell[self.face];
                if self.face_count == 3 {
                    self.cylinder[self.face] = self.cylinder_face(source, id)?;
                } else if self.spline {
                    let (face, chart) = Chart::read(self, source, id)?;
                    self.faces[self.face] = face;
                    self.charts[self.face] = chart;
                } else {
                    let face = self.face(source, id)?;
                    if self.face_count > 6 { self.orthogonal.coordinates(&face)?; }
                    self.faces[self.face] = face;
                }
                self.face += 1;
                if self.face == self.face_count { self.stage = Stage::Verify; }
            }
            Stage::Verify => {
                let faces = &self.faces[..self.face_count];
                match faces.len() {
                    3 => self.result = Some(cylinder_proof::certify_cylinder(&self.cylinder)?),
                    5 => self.result = Some(prism_proof::certify_prism(faces)?),
                    6 => {
                        let bounds = certify_box(faces)?;
                        if self.spline {
                            self.result = Some(StepSolid::NurbsCuboid(bounds));
                            self.face = 0;
                            self.stage = Stage::VerifySplineFace;
                            return Ok(());
                        }
                        self.result = Some(StepSolid::Box(bounds));
                    }
                    _ => {
                        self.orthogonal.prepare()?;
                        self.face = 0;
                        self.stage = Stage::OrthogonalFace;
                        return Ok(());
                    }
                }
                self.stage = Stage::Release;
            }
            Stage::VerifySplineFace => {
                let primitive = self.result.and_then(StepSolid::nurbs_cuboid).ok_or_else(|| invalid("STEP cuboid proof is missing"))?;
                self.charts[self.face].certify(primitive, &self.faces[self.face])?;
                self.face += 1;
                if self.face == self.face_count { self.stage = Stage::Release; }
            }
            Stage::OrthogonalFace => {
                self.orthogonal.face(&self.faces[self.face])?;
                self.face += 1;
                if self.face == self.face_count { self.stage = Stage::OrthogonalRay; }
            }
            Stage::OrthogonalRay => {
                if self.orthogonal.ray()? { self.stage = Stage::OrthogonalFinish; }
            }
            Stage::OrthogonalFinish => {
                self.result = Some(self.orthogonal.finish()?);
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
                    if let Some((kind, _)) = body.split_once('(') {
                        let kind = kind.trim();
                        if kind.eq_ignore_ascii_case("MANIFOLD_SOLID_BREP") {
                            if self.solid != 0 { return Err(unsupported("STEP import requires one authored manifold solid")); }
                            self.solid = id;
                        } else if ["BREP_WITH_VOIDS", "FACETED_BREP", "SHELL_BASED_SURFACE_MODEL",
                            "MAPPED_ITEM", "NEXT_ASSEMBLY_USAGE_OCCURRENCE"].iter().any(|known| kind.eq_ignore_ascii_case(known)) {
                            return Err(unsupported("STEP import requires a single direct planar solid representation"));
                        }
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
            return Err(unsupported("STEP entity is outside the retained solid subset"));
        }
        attributes.trim_end().strip_suffix(')').ok_or_else(|| invalid("STEP instance has an invalid attribute list"))
    }

    fn bound(&self, source: &str, id: u64) -> Result<Bound, ValueError> {
        let [_, bounds, surface, same] = fields(self.row(source, id, "ADVANCED_FACE")?)?;
        let [bound] = refs(bounds)?;
        let [_, ring, forward] = fields(self.row(source, bound, "FACE_OUTER_BOUND")?)?;
        let [_, edges] = fields(self.row(source, reference(ring)?, "EDGE_LOOP")?)?;
        let mut members = [0; 4];
        let count = list_references(edges, &mut members)?;
        Ok(Bound { surface: reference(surface)?, same: boolean(same)?, forward: boolean(forward)?, members, count })
    }

    fn placement(&self, source: &str, id: u64) -> Result<Placement, ValueError> {
        let [_, origin, axis, radial] = fields(self.row(source, id, "AXIS2_PLACEMENT_3D")?)?;
        let origin = self.vector(source, reference(origin)?, "CARTESIAN_POINT")?;
        let axis = if axis == "$" { [0.0, 0.0, 1.0] } else { self.vector(source, reference(axis)?, "DIRECTION")? };
        if axis == [0.0; 3] { return Err(invalid("STEP placement has no axis direction")); }
        let radial = if radial == "$" { None } else { Some(self.vector(source, reference(radial)?, "DIRECTION")?) };
        if let Some(radial) = radial {
            let cross = Vec3::from_array(axis).cross(Vec3::from_array(radial));
            if cross == Vec3::ZERO || !cross.to_array().into_iter().all(f64::is_finite) {
                return Err(invalid("STEP placement reference direction is degenerate"));
            }
        }
        Ok(Placement { origin, axis, radial })
    }

    fn face(&self, source: &str, id: u64) -> Result<Face, ValueError> {
        let Bound { surface, same, forward, members, count } = self.bound(source,id)?;
        let [_, axis] = fields(self.row(source, surface, "PLANE")?)?;
        let Placement { origin, axis, .. } = self.placement(source,reference(axis)?)?;
        let normal = if same { axis } else { axis.map(|value| -value) };
        let mut ring = [Edge::default();4];
        if !matches!(count, 3 | 4) { return Err(unsupported("STEP import requires triangle or quadrilateral face bounds")); }
        for (slot, id) in ring.iter_mut().zip(members[..count].iter()) {
            let authored = self.coedge(source,*id)?;
            let line = self.wrapped_curve(source,authored.curve,"SURFACE_CURVE")?.unwrap_or(authored.curve);
            *slot = self.line_edge(source,authored,line)?;
        }
        self.close_ring(source, &mut ring[..count], forward)?;
        Ok(Face { origin, normal, edges: ring, count })
    }

    fn close_ring(&self, source: &str, ring: &mut [Edge], forward: bool) -> Result<(), ValueError> {
        if !forward {
            ring.reverse();
            for edge in &mut *ring {
                std::mem::swap(&mut edge.from, &mut edge.to);
                edge.point = self.vertex(source, edge.from)?;
            }
        }
        for (index, edge) in ring.iter().enumerate() {
            if edge.to != ring[(index + 1) % ring.len()].from { return Err(invalid("STEP face bound is discontinuous")); }
        }
        Ok(())
    }

    fn coedge(&self, source: &str, id: u64) -> Result<AuthoredCoedge, ValueError> {
        let [_, start, end, edge, forward] = fields(self.row(source,id,"ORIENTED_EDGE")?)?;
        if start != "*" || end != "*" { return Err(unsupported("STEP oriented edge requires derived endpoint attributes")); }
        let edge = reference(edge)?;
        let [_, from, to, curve, same] = fields(self.row(source,edge,"EDGE_CURVE")?)?;
        Ok(AuthoredCoedge { edge, from: reference(from)?, to: reference(to)?, curve: reference(curve)?, same: boolean(same)?, forward: boolean(forward)? })
    }

    fn wrapped_curve(&self, source: &str, curve: u64, kind: &str) -> Result<Option<u64>, ValueError> {
        if let Ok(attributes) = self.row(source,curve,kind) {
            let [_, curve, _, _] = fields(attributes)?;
            return reference(curve).map(Some);
        }
        Ok(None)
    }

    fn line_edge(&self, source: &str, authored: AuthoredCoedge, line: u64) -> Result<Edge, ValueError> {
        let AuthoredCoedge { edge, from, to, same, forward, .. } = authored;
        let (a, b) = (self.vertex(source,from)?, self.vertex(source,to)?);
        let delta = std::array::from_fn(|axis| b[axis] - a[axis]);
        if delta == [0.0; 3] || !delta.into_iter().all(f64::is_finite) { return Err(invalid("STEP edge extent is degenerate")); }
        let [_, origin, vector] = fields(self.row(source, line, "LINE")?)?;
        let origin = self.vector(source, reference(origin)?, "CARTESIAN_POINT")?;
        let [_, direction, length] = fields(self.row(source, reference(vector)?, "VECTOR")?)?;
        let direction = self.vector(source, reference(direction)?, "DIRECTION")?;
        let exact_trim = if let Some(axis) = only_axis(delta) {
            only_axis(direction) == Some(axis) && (0..3).all(|i| i == axis || origin[i] == a[i])
                && ((direction[axis] > 0.0) == (delta[axis] > 0.0)) == same
        } else {
            (origin == a || origin == b) && direction == if same { delta } else { delta.map(|value| -value) }
        };
        if number(length)? <= 0.0 || !exact_trim { return Err(invalid("STEP LINE does not support its authored edge trim")); }
        if forward { Ok(Edge { id: edge, from, to, point: a }) }
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

fn cuboid_primitive((origin, [width, depth, height]): StepBox) -> SolidPrimitive {
    SolidPrimitive::Box { origin: Pnt3::from_array(origin), width, depth, height }
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

fn list_references(text: &str, output: &mut [u64]) -> Result<usize, ValueError> {
    let mut count = 0;
    for field in list(text)?.split(',') {
        *output.get_mut(count).ok_or_else(|| unsupported("STEP authored shell or ring exceeds its retained arity"))? = reference(field)?;
        count += 1;
    }
    if output[..count].iter().enumerate().any(|(index,id)| output[..index].contains(id)) {
        return Err(invalid("STEP aggregate repeats an authored member"));
    }
    Ok(count)
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

fn axis_side(face: &Face) -> Result<(usize, bool), ValueError> {
    let axis = only_axis(face.normal).ok_or_else(|| unsupported("STEP face is not axis aligned"))?;
    let positive = face.normal[axis] > 0.0;
    if face.count != 4 || face.edges.iter().any(|edge| edge.point[axis] != face.origin[axis]) {
        return Err(invalid("STEP box bound leaves its planar quadrilateral"));
    }
    let (i,j) = ((axis+1)%3, (axis+2)%3);
    let [a,b,c,_] = face.edges.map(|edge| edge.point);
    let orientation = if a[i] != b[i] && b[j] != c[j] { Some((a[i] < b[i]) == (b[j] < c[j])) }
        else if a[j] != b[j] && b[i] != c[i] { Some((a[j] < b[j]) != (b[i] < c[i])) }
        else { None };
    if orientation != Some(positive) { return Err(invalid("STEP face and outer bound orientations disagree")); }
    Ok((axis, positive))
}

fn certify_box(faces: &[Face]) -> Result<StepBox, ValueError> {
    let mut sides = 0_u8;
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for face in faces {
        let (axis, positive) = axis_side(face)?;
        let bit = 1 << (2 * axis + usize::from(positive));
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
        let (axis, positive) = axis_side(face)?;
        let mut used = 0_u8;
        for edge in face.edges {
            let mut corner = 0;
            for axis in 0..3 {
                if edge.point[axis] == max[axis] { corner |= 1 << axis; }
                else if edge.point[axis] != min[axis] { return Err(unsupported("STEP face contains a vertex between box corners")); }
            }
            if ((corner >> axis) & 1 != 0) != positive || used & (1 << corner) != 0 {
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
