//! 🪟️ Funded preview of rectangular UV trims on original cylinders and cones.

use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, SurfaceId};
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_mesh_engine::{ComponentReferenceTable, MeshData};
use semio_framework_value::{RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::{f64::consts::TAU, mem::{size_of, take}};

type Uv = [f64; 2];
const DOMAINS: [&str; 3] = ["face", "edge", "vertex"];
const MAXIMUM_TRIANGLES: usize = 24_000;

#[derive(Clone, Copy, Default)]
struct Trim {
    uv: [Uv; 2],
    vertices: [u64; 2],
    edge: u64,
    samples: usize,
}

#[derive(Clone, Copy)]
enum Turn {
    Begin,
    Trim,
    Reserve(usize),
    Label { domain: usize, index: usize },
    Triangle(usize),
    Edge { trim: usize, segment: usize },
}

/// 🪧️ Progress, preallocation fallback, or the completed original preview.
pub enum AxialFacePreviewStep {
    Working,
    Unsupported,
    Ready(MeshData),
}

/// 🪟️ Owns one retained native axial face tessellation until publication or cancellation.
pub struct AxialFacePreview {
    face: FaceId,
    revision: u64,
    deflection: f64,
    turn: Turn,
    surface: Option<SurfaceId>,
    ring: Option<LoopId>,
    first: Option<CoedgeId>,
    current: Option<CoedgeId>,
    flipped: bool,
    face_label: u64,
    trims: [Trim; 4],
    count: usize,
    bounds: [Uv; 2],
    divisions: [usize; 2],
    apex: bool,
    periodic: bool,
    edges: [u64; 4],
    vertices: [u64; 4],
    edge_count: usize,
    vertex_count: usize,
    mesh: MeshData,
    references: Vec<(String, Vec<String>)>,
}

semio_framework_value::artifact_retire_leaf!(Trim, Turn);
semio_framework_value::artifact_retire_struct!(AxialFacePreview { face, revision, deflection, turn, surface, ring, first, current, flipped, face_label, trims, count, bounds, divisions, apex, periodic, edges, vertices, edge_count, vertex_count, mesh, references });

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "axial face preview requires finite original topology")
}

fn limit() -> ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit, "axial face preview exceeds its triangle limit")
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.0)
}

fn same(a: Uv, b: Uv) -> bool { close(a[0], b[0]) && close(a[1], b[1]) }

fn lerp(a: f64, b: f64, i: usize, n: usize) -> f64 {
    if i == n { b } else { a + (b - a) * (i as f64 / n as f64) }
}

fn reserve<T>(buffer: &mut Vec<T>, count: usize) -> Result<usize, ValueError> {
    buffer.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "axial face preview allocation failed"))?;
    Ok(buffer.capacity() * size_of::<T>())
}

fn label(text: &str) -> Result<String, ValueError> {
    let mut value = String::new();
    value.try_reserve_exact(text.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "axial face preview label allocation failed"))?;
    value.push_str(text);
    Ok(value)
}

fn insert_label(labels: &mut [u64; 4], count: &mut usize, label: u64) {
    if !labels[..*count].contains(&label) {
        labels[*count] = label;
        *count += 1;
    }
}

impl AxialFacePreview {
    /// 🎟️ Pins the requested face and revision without allocating a result.
    pub fn new(face: FaceId, revision: u64, deflection: f64) -> Self {
        Self {
            face, revision, deflection, turn: Turn::Begin, surface: None, ring: None, first: None, current: None,
            flipped: false, face_label: 0, trims: [Trim::default(); 4], count: 0, bounds: [[0.; 2]; 2],
            divisions: [1; 2], apex: false, periodic: false, edges: [0; 4], vertices: [0; 4],
            edge_count: 0, vertex_count: 0, mesh: MeshData::default(), references: Vec::new(),
        }
    }

    /// 🧭️ Reports whether the native trim has been accepted for meshing.
    pub fn is_meshing(&self) -> bool { !matches!(self.turn, Turn::Begin | Turn::Trim) }

    fn check_source(&self, id: FaceId, revision: u64) -> Result<(), ValueError> {
        if self.face == id && self.revision == revision { Ok(()) }
        else { Err(ValueError::invariant("axial face preview source changed")) }
    }

    fn surface<'a>(&self, body: &'a Body) -> Result<&'a Surface, ValueError> {
        self.surface.and_then(|id| body.surfaces.get(id)).ok_or_else(invalid)
    }

    fn labels(&self, domain: usize) -> &[u64] {
        match domain {
            0 => std::slice::from_ref(&self.face_label),
            1 => &self.edges[..self.edge_count],
            _ => &self.vertices[..self.vertex_count],
        }
    }

    fn elements(&self, slot: usize) -> usize {
        let triangles = 2 * self.divisions[0] * self.divisions[1];
        let edges: usize = self.trims[..self.count].iter().map(|trim| trim.samples).sum();
        match slot {
            0 | 1 => triangles * 9,
            2 | 3 => triangles * 3,
            4 => triangles,
            5 => edges * 6,
            6 => edges,
            _ => unreachable!(),
        }
    }

    fn reserve_demand(&self, slot: usize) -> (usize, usize) {
        match slot {
            0..=6 => (0, self.elements(slot) * size_of::<u32>()),
            7 => (0, 3 * size_of::<(String, Vec<String>)>()),
            8..=10 => { let n = DOMAINS[slot - 8].len(); (n, n) }
            11..=13 => (0, self.labels(slot - 11).len() * size_of::<String>()),
            _ => unreachable!(),
        }
    }

    /// 📏️ Quotes the next original trim, backing or mesh event.
    pub fn demand(&self, body: &Body, face: FaceId, revision: u64) -> Result<RetirementDemand, ValueError> {
        self.check_source(face, revision)?;
        let mut depth = body.faces.next_remove_depth_demand(face)?;
        let row = body.faces.get(face).ok_or_else(invalid)?;
        depth = depth.max(body.surfaces.next_remove_depth_demand(row.surface)?);
        let (copy_bytes, capacity_bytes) = match self.turn {
            Turn::Begin => {
                if let Some(ring) = row.outer { depth = depth.max(body.loops.next_remove_depth_demand(ring)?); }
                (size_of::<u64>() + size_of::<LoopId>() + size_of::<SurfaceId>(), 0)
            }
            Turn::Trim => {
                let id = self.current.ok_or_else(invalid)?;
                depth = depth.max(body.coedges.next_remove_depth_demand(id)?);
                let coedge = body.coedges.get(id).ok_or_else(invalid)?;
                if let Some(trim) = coedge.pcurve { depth = depth.max(body.curves2.next_remove_depth_demand(trim)?); }
                depth = depth.max(body.edges.next_remove_depth_demand(coedge.edge)?);
                let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
                depth = depth.max(body.curves3.next_remove_depth_demand(edge.curve)?)
                    .max(body.vertices.next_remove_depth_demand(edge.v0)?).max(body.vertices.next_remove_depth_demand(edge.v1)?);
                (size_of::<Trim>() + size_of::<CoedgeId>(), 0)
            }
            Turn::Reserve(slot) => self.reserve_demand(slot),
            Turn::Label { domain, index } => {
                let n = PersistentLabel(self.labels(domain)[index]).decimal(&mut [0; 20]).len();
                (n, n)
            }
            Turn::Triangle(_) => (18 * size_of::<f32>() + 7 * size_of::<u32>(), 0),
            Turn::Edge { .. } => (6 * size_of::<f32>() + size_of::<u32>(), 0),
        };
        Ok(RetirementDemand { copy_bytes, capacity_bytes, depth, ..Default::default() })
    }

    fn begin(&mut self, body: &Body) -> Result<bool, ValueError> {
        let face = body.faces.get(self.face).ok_or_else(invalid)?;
        if !face.inners.is_empty() || !matches!(body.surfaces.get(face.surface), Some(Surface::Cylinder { .. } | Surface::Cone { .. })) {
            return Ok(false);
        }
        let Some(ring) = face.outer else { return Ok(false); };
        let row = body.loops.get(ring).filter(|row| row.face == self.face).ok_or_else(invalid)?;
        self.surface = Some(face.surface);
        self.ring = Some(ring);
        self.first = Some(row.first);
        self.current = Some(row.first);
        self.flipped = face.flipped;
        self.face_label = face.label.0;
        self.turn = Turn::Trim;
        Ok(true)
    }

    fn read_trim(&mut self, body: &Body) -> Result<bool, ValueError> {
        let coedge = body.coedges.get(self.current.ok_or_else(invalid)?).ok_or_else(invalid)?;
        if Some(coedge.loop_id) != self.ring { return Ok(false); }
        let Some(trim) = coedge.pcurve.and_then(|id| body.curves2.get(id)) else { return Ok(false); };
        if !matches!(trim, Curve2::Line { .. }) { return Ok(false); }
        let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
        let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
        if !matches!(curve, Curve3::Line { .. } | Curve3::Circle { .. } | Curve3::Ellipse { .. }) { return Ok(false); }
        let get_uv = |t| { let p = trim.eval(t); [p.x, p.y] };
        let mut uv = [get_uv(coedge.prange.0), get_uv(coedge.prange.1)];
        if !uv.into_iter().flatten().all(f64::is_finite) || same(uv[0], uv[1])
            || close(uv[0][0], uv[1][0]) == close(uv[0][1], uv[1][1]) { return Ok(false); }
        let tolerance = edge.tol.value().max(1e-10);
        let surface = self.surface(body)?;
        for t in [0., 0.25, 0.5, 0.75, 1.] {
            let [u, v] = get_uv(coedge.prange.0 + (coedge.prange.1 - coedge.prange.0) * t);
            let Some(point) = surface.point_original(u, v) else { return Ok(false); };
            let actual = curve.eval(edge.range.0 + (edge.range.1 - edge.range.0) * t);
            if !(point.distance(actual) <= tolerance) { return Ok(false); }
        }
        let a = body.vertices.get(edge.v0).ok_or_else(invalid)?;
        let b = body.vertices.get(edge.v1).ok_or_else(invalid)?;
        if !(curve.eval(edge.range.0).distance(a.position) <= tolerance && curve.eval(edge.range.1).distance(b.position) <= tolerance) {
            return Ok(false);
        }
        let mut labels = [a.label.0, b.label.0];
        if !coedge.forward { uv.reverse(); labels.reverse(); }
        self.trims[self.count] = Trim { uv, vertices: labels, edge: edge.label.0, samples: 0 };
        self.count += 1;
        self.current = Some(coedge.next);
        if self.current == self.first { return self.finish_trims(body); }
        Ok(self.count < self.trims.len())
    }

    fn finish_trims(&mut self, body: &Body) -> Result<bool, ValueError> {
        let trims = &self.trims[..self.count];
        if !(3..=4).contains(&trims.len()) { return Ok(false); }
        let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
        for uv in trims.iter().flat_map(|trim| trim.uv) {
            for (axis, coord) in uv.into_iter().enumerate() {
                bounds[axis][0] = bounds[axis][0].min(coord);
                bounds[axis][1] = bounds[axis][1].max(coord);
            }
        }
        let [[ua, ub], [va, vb]] = bounds;
        let sweep = ub - ua;
        let height = vb - va;
        if close(ua, ub) || close(va, vb) || sweep > TAU && !close(sweep, TAU) { return Ok(false); }
        let (radial, mixed, apex) = match self.surface(body)? {
            Surface::Cylinder { frame, radius } if *radius > 0. =>
                (frame.x.norm().hypot(frame.y.norm()) * radius, 0., false),
            Surface::Cone { frame, half_angle } if va >= 0. && *half_angle > 0. && *half_angle < std::f64::consts::FRAC_PI_2 => {
                let slope = frame.x.norm().hypot(frame.y.norm()) * half_angle.tan();
                (slope * vb, slope, va == 0.)
            }
            _ => return Ok(false),
        };
        let periodic = close(sweep, TAU);
        let mut sides = 0;
        for (i, trim) in trims.iter().enumerate() {
            let next = &trims[(i + 1) % trims.len()];
            if trim.vertices[1] != next.vertices[0] { return Ok(false); }
            if !same(trim.uv[1], next.uv[0]) && !(apex && periodic && close(trim.uv[1][1], va) && close(next.uv[0][1], va)) {
                return Ok(false);
            }
            let [a, b] = trim.uv;
            let side = if close(a[1], b[1]) && close(a[0].min(b[0]), ua) && close(a[0].max(b[0]), ub) {
                if close(a[1], va) { 1 } else if close(a[1], vb) { 2 } else { return Ok(false); }
            } else if close(a[0], b[0]) && close(a[1].min(b[1]), va) && close(a[1].max(b[1]), vb) {
                if close(a[0], ua) { 4 } else if close(a[0], ub) { 8 } else { return Ok(false); }
            } else { return Ok(false); };
            if sides & side != 0 { return Ok(false); }
            sides |= side;
        }
        if sides != 15 && !(apex && periodic && sides == 14) { return Ok(false); }
        let cross = mixed * sweep * height;
        let bound = [radial * sweep * sweep + cross, cross];
        if !bound.into_iter().all(f64::is_finite) { return Err(invalid()); }
        let mut divisions = bound.map(|value| (2. * value / self.deflection).sqrt().ceil().max(1.) as usize);
        divisions[0] = divisions[0].max((sweep / (TAU / 4.)).ceil() as usize);
        if divisions[0].checked_mul(divisions[1]).and_then(|n| n.checked_mul(2)).is_none_or(|n| n > MAXIMUM_TRIANGLES) { return Err(limit()); }
        self.bounds = bounds;
        self.divisions = divisions;
        self.periodic = periodic;
        self.apex = apex;
        for trim in &mut self.trims[..self.count] {
            if !self.edges[..self.edge_count].contains(&trim.edge) {
                trim.samples = if close(trim.uv[0][1], trim.uv[1][1]) { divisions[0] } else { 1 };
                insert_label(&mut self.edges, &mut self.edge_count, trim.edge);
            }
            for vertex in trim.vertices { insert_label(&mut self.vertices, &mut self.vertex_count, vertex); }
        }
        self.turn = Turn::Reserve(0);
        Ok(true)
    }

    fn allocate(&mut self, slot: usize) -> Result<usize, ValueError> {
        let count = if slot <= 6 { self.elements(slot) } else { 0 };
        let bytes = match slot {
            0 => reserve(&mut self.mesh.positions, count)?,
            1 => reserve(&mut self.mesh.normals, count)?,
            2 => reserve(&mut self.mesh.indices, count)?,
            3 => reserve(&mut self.mesh.vertex_ids, count)?,
            4 => reserve(&mut self.mesh.face_ids, count)?,
            5 => reserve(&mut self.mesh.edge_positions, count)?,
            6 => reserve(&mut self.mesh.edge_ids, count)?,
            7 => reserve(&mut self.references, DOMAINS.len())?,
            8..=10 => {
                let name = label(DOMAINS[slot - 8])?;
                let bytes = name.capacity();
                self.references.push((name, Vec::new()));
                bytes
            }
            11..=13 => {
                let domain = slot - 11;
                let count = self.labels(domain).len();
                reserve(&mut self.references[domain].1, count)?
            }
            _ => unreachable!(),
        };
        self.turn = if slot == 13 { Turn::Label { domain: 0, index: 0 } } else { Turn::Reserve(slot + 1) };
        Ok(bytes)
    }

    fn position(&self, surface: &Surface, [u, v]: Uv) -> Result<[f32; 3], ValueError> {
        let u = if self.periodic && close(u, self.bounds[0][1]) { self.bounds[0][0] } else { u };
        let point = surface.point_original(u, v).ok_or_else(invalid)?;
        let xyz = point.to_array().map(|axis| axis as f32);
        if !xyz.into_iter().all(f32::is_finite) || point.distance(Pnt3::from_array(xyz.map(f64::from))) > self.deflection * 0.5 { return Err(invalid()); }
        Ok(xyz)
    }

    fn vertex_id(&self, uv: Uv) -> u32 {
        for trim in &self.trims[..self.count] {
            for (point, label) in trim.uv.into_iter().zip(trim.vertices) {
                if same(point, uv) || self.apex && close(point[1], self.bounds[1][0]) && close(uv[1], point[1]) {
                    return self.labels(2).iter().position(|&x| x == label).map_or(u32::MAX, |i| i as u32);
                }
            }
        }
        u32::MAX
    }

    fn triangle(&mut self, surface: &Surface, index: usize) -> Result<bool, ValueError> {
        let [nu, nv] = self.divisions;
        let (u, v) = (index / 2 / nv, index / 2 % nv);
        let x = |i| lerp(self.bounds[0][0], self.bounds[0][1], i, nu);
        let y = |i| lerp(self.bounds[1][0], self.bounds[1][1], i, nv);
        let mut uv = if index % 2 == 0 { [[x(u), y(v)], [x(u+1), y(v)], [x(u+1), y(v+1)]] }
            else { [[x(u), y(v)], [x(u+1), y(v+1)], [x(u), y(v+1)]] };
        if self.flipped { uv.swap(1, 2); }
        let pos = [self.position(surface, uv[0])?, self.position(surface, uv[1])?, self.position(surface, uv[2])?];
        let [a,b,c] = pos.map(|xyz| Pnt3::from_array(xyz.map(f64::from)));
        let Some(normal) = (b-a).cross(c-a).normalized() else { return Ok(false); };
        let base = (self.mesh.positions.len() / 3) as u32;
        for ([u,v], xyz) in uv.into_iter().zip(pos) {
            let oriented = surface.normal_original(u, v)
                .map(|n| if self.flipped { -n } else { n }).unwrap_or(normal);
            self.mesh.positions.extend(xyz);
            self.mesh.normals.extend(oriented.to_array().map(|x| x as f32));
            let pick = self.vertex_id([u,v]);
            self.mesh.vertex_ids.push(pick);
        }
        self.mesh.indices.extend([base, base+1, base+2]);
        self.mesh.face_ids.push(0);
        Ok(true)
    }

    /// ⏱️ Advances one funded event against the unchanged original topology.
    pub fn step(&mut self, body: &Body, face: FaceId, revision: u64, grant: Grant) -> Result<(AxialFacePreviewStep, Progress), ValueError> {
        let demand = self.demand(body, face, revision)?;
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) {
            return Ok((AxialFacePreviewStep::Working, Default::default()));
        }
        let mut progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match self.turn {
            Turn::Begin => if !self.begin(body)? { return Ok((AxialFacePreviewStep::Unsupported, progress)); },
            Turn::Trim => if !self.read_trim(body)? { return Ok((AxialFacePreviewStep::Unsupported, progress)); },
            Turn::Reserve(slot) => progress.retained_capacity_bytes = self.allocate(slot)?,
            Turn::Label { domain, index } => {
                let key = self.labels(domain)[index];
                let value = label(PersistentLabel(key).decimal(&mut [0;20]))?;
                progress.retained_capacity_bytes = value.capacity();
                self.references[domain].1.push(value);
                self.turn = if index + 1 < self.labels(domain).len() { Turn::Label { domain, index: index + 1 } }
                    else if domain < 2 { Turn::Label { domain: domain + 1, index: 0 } }
                    else { Turn::Triangle(0) };
            }
            Turn::Triangle(index) => {
                if !self.triangle(self.surface(body)?, index)? { progress.copied_bytes = 0; }
                self.turn = if index + 1 == 2 * self.divisions[0] * self.divisions[1] { Turn::Edge { trim: 0, segment: 0 } }
                    else { Turn::Triangle(index + 1) };
            }
            Turn::Edge { trim, segment } => {
                if trim == self.count {
                    if self.mesh.indices.is_empty() { return Err(invalid()); }
                    self.mesh.component_references = ComponentReferenceTable::from_entries(take(&mut self.references));
                    progress.copied_bytes = 0;
                    return Ok((AxialFacePreviewStep::Ready(take(&mut self.mesh)), progress));
                }
                let row = self.trims[trim];
                if row.samples == 0 {
                    self.turn = Turn::Edge { trim: trim + 1, segment: 0 };
                    progress.copied_bytes = 0;
                } else {
                    let point = |i| self.position(self.surface(body)?, std::array::from_fn(|axis| lerp(row.uv[0][axis], row.uv[1][axis], i, row.samples)));
                    let (a,b) = (point(segment)?, point(segment + 1)?);
                    let edge_id = self.labels(1).iter().position(|&label| label == row.edge).ok_or_else(invalid)? as u32;
                    self.mesh.edge_positions.extend(a);
                    self.mesh.edge_positions.extend(b);
                    self.mesh.edge_ids.push(edge_id);
                    self.turn = if segment + 1 == row.samples { Turn::Edge { trim: trim + 1, segment: 0 } }
                        else { Turn::Edge { trim, segment: segment + 1 } };
                }
            }
        }
        if !progress.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "axial face preview exceeded its grant").with_retained_progress(progress));
        }
        Ok((AxialFacePreviewStep::Working, progress))
    }
}
