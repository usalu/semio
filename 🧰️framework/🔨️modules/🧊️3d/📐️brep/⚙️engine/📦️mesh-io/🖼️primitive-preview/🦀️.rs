//! 🖼️ Bounded mesh generation from constructor-certified analytic solids.

use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge,SolidPrimitive};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::SolidId,contract::BorePreviewSource,topology::history::PersistentLabel};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{matrix::Frame3,Pnt3,Vec3 as NativeVec3};
use semio_framework_mesh_engine::{ComponentReferenceTable, MeshData};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::mem::{size_of, take};

#[path="🕳️bore-sampling/🦀️.rs"]
mod bore_sampling;
use bore_sampling::BoreSampling;

const MAX_SEGMENTS: usize = 96;
const POSITION_SCALARS_PER_TRIANGLE: usize = 9;
const INDEX_SCALARS_PER_TRIANGLE: usize = 3;
const EDGE_SCALARS_PER_SEGMENT: usize = 6;
const TRIANGLE_BYTES: usize = 2 * POSITION_SCALARS_PER_TRIANGLE * size_of::<f32>() + 7 * size_of::<u32>();
const EDGE_BYTES: usize = EDGE_SCALARS_PER_SEGMENT * size_of::<f32>() + size_of::<u32>();
const POINT_BYTES: usize = 6 * size_of::<f32>() + size_of::<u32>();
const STORAGE_STAGES: usize = 8;
const DOMAINS: [&str; 3] = ["edge", "face", "vertex"];

struct Vertex {
    point: Pnt3,
    normal: NativeVec3,
}

#[derive(Clone, Copy)]
enum Chart {
    Rectangle,
    Bore,
    Cap { radius: f64 },
    Wrapped { first: f64, end: f64, rows: usize, low_pole: bool, high_pole: bool },
}

impl Chart {
    fn slots(self, columns: usize) -> usize {
        match self {
            Self::Rectangle => 2,
            Self::Bore => 2*columns,
            Self::Cap { .. } => columns,
            Self::Wrapped { rows, .. } => 2 * columns * rows,
        }
    }
}

#[derive(Clone, Copy)]
struct CertifiedPrimitive {
    solid: SolidPrimitive,
    first_label: u64,
    bore:Option<BoreSampling>,
}

impl CertifiedPrimitive {
    fn face_count(self)->usize{self.solid.ring_sizes().len()+usize::from(self.bore.is_some())}

    fn vertex_count(self)->usize{self.solid.vertex_count()+2*usize::from(self.bore.is_some())}

    fn edge_count(self)->usize{self.solid.edge_count()+3*usize::from(self.bore.is_some())}

    fn tool(self)->SolidPrimitive{
        let bore=self.bore.expect("certified bore").source();
        SolidPrimitive::Cylinder {
            frame:Frame3 { origin:Pnt3::new(bore.center[0],bore.center[1],bore.origin[2]),..Frame3::WORLD },
            radius:bore.radius,
            height:bore.dimensions[2],
        }
    }

    fn edge(self,index:usize)->PrimitiveEdge {
        if index<self.solid.edge_count() { return self.solid.edge(index); }
        let mut edge=self.tool().edge(index-self.solid.edge_count());
        edge.vertices=edge.vertices.map(|id|id+self.solid.vertex_count());
        edge
    }

    fn vertex(self,index:usize)->Pnt3 {
        if index<self.solid.vertex_count() { return self.solid.vertex(index); }
        self.tool().vertex(index-self.solid.vertex_count())
    }
}

semio_framework_value::artifact_retire_leaf!(CertifiedPrimitive);

/// 📐️ Holds exact primitive provenance and the four separately admitted preview buffers.
pub struct PrimitivePreviewJob {
    source: SolidId,
    primitive: CertifiedPrimitive,
    segments: usize,
    tube_segments: usize,
    triangle_slots: usize,
    edge_segments: usize,
    storage: usize,
    slot: usize,
    face: usize,
    edge: usize,
    segment: usize,
    units_done: usize,
    vertex: usize,
    edge_pick: u32,
    references: Vec<(String, Vec<String>)>,
    reference_key: String,
    reference_labels: Vec<String>,
    output: MeshData,
    done: bool,
}

semio_framework_value::artifact_retire_struct!(PrimitivePreviewJob {
    source, primitive, segments, tube_segments, triangle_slots, edge_segments, storage, slot, face, edge, segment, units_done, vertex, edge_pick, references, reference_key, reference_labels, output, done
});

fn segment_count(radius: f64, tolerance: f64, sweep: f64) -> usize {
    let angle = 2.0 * (1.0 - tolerance / radius).clamp(-1.0, 1.0).acos();
    ((sweep / angle).ceil() as usize).clamp(8, MAX_SEGMENTS)
}

fn point_components(point: Pnt3) -> [f32; 3] { [point.x as f32, point.y as f32, point.z as f32] }
fn normal_components(normal: NativeVec3) -> [f32; 3] { [normal.x as f32, normal.y as f32, normal.z as f32] }

fn invalid_preview() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "primitive dimensions or preview tolerance exceed the finite preview range")
}

fn allocation_refusal() -> ValueError {
    ValueError::literal(ValueRefusalKind::AllocationFailed, "original primitive preview allocation failed")
}

fn allocate<T>(values: &mut Vec<T>, extent: usize) -> Result<usize, ValueError> {
    values.try_reserve_exact(extent).map_err(|_| allocation_refusal())?;
    Ok(values.capacity() * size_of::<T>())
}

fn allocate_text(text: &str) -> Result<String, ValueError> {
    let mut owned = String::new();
    owned.try_reserve_exact(text.len()).map_err(|_| allocation_refusal())?;
    owned.push_str(text);
    Ok(owned)
}

impl PrimitivePreviewJob {
    /// 🎬️ Starts sampling the supplied primitive and its stable component label range.
    pub fn new(source: SolidId, primitive: SolidPrimitive, first_label: u64, tolerance: f64) -> Result<Self, ValueError> {
        let last_offset = primitive.vertex_count() + primitive.edge_count() + primitive.ring_sizes().len() - 1;
        if primitive.validate().is_err() || first_label.checked_add(last_offset as u64).is_none() {
            return Err(invalid_preview());
        }
        let (radius, tube_radius, extent) = match primitive {
            SolidPrimitive::Box { origin, width, depth, height } => (1.0, 1.0, origin.x.abs().max(origin.y.abs()).max(origin.z.abs()) + width.max(depth).max(height)),
            SolidPrimitive::Sphere { frame, radius } => {
                let center = frame.origin.x.abs().max(frame.origin.y.abs()).max(frame.origin.z.abs());
                (radius, radius, center + radius)
            },
            SolidPrimitive::Cylinder { frame, radius, height } => {
                let origin = frame.origin.x.abs().max(frame.origin.y.abs()).max(frame.origin.z.abs());
                (radius, radius, origin + radius + height)
            }
            SolidPrimitive::Cone { frame, radius, height } => {
                let center = frame.origin.x.abs().max(frame.origin.y.abs()).max(frame.origin.z.abs());
                (radius, radius, center + radius + height)
            },
            SolidPrimitive::Torus { frame, major, minor } => {
                let center=frame.origin.x.abs().max(frame.origin.y.abs()).max(frame.origin.z.abs());
                (major+minor,minor,center+major+minor)
            },
        };
        if !tolerance.is_finite() || tolerance <= 0.0 || extent > f64::from(f32::MAX) / 4.0 { return Err(invalid_preview()); }
        let segments = segment_count(radius, tolerance, TAU);
        let tube_segments = segment_count(tube_radius, tolerance, TAU);
        let mut job = Self {
            source, primitive: CertifiedPrimitive { solid: primitive, first_label, bore:None }, segments, tube_segments, triangle_slots: 0, edge_segments: 0, storage: 0, slot: 0, face: 0,
            edge: 0, segment: 0, units_done: 0, vertex: 0, edge_pick: 0, references: Vec::new(),
            reference_key: String::new(), reference_labels: Vec::new(), output: MeshData::default(), done: false,
        };
        job.triangle_slots = (0..primitive.ring_sizes().len()).map(|face| job.chart(face).slots(segments)).sum();
        job.edge_segments = (0..primitive.edge_count()).map(|edge| job.segments_on_edge(edge)).sum();
        Ok(job)
    }

    /// 🕳️ Admits a box with a fully penetrating circular inner wall.
    pub fn new_bore(source:SolidId,bore:BorePreviewSource,tolerance:f64)->Result<Self,ValueError>{
        let [x,y,z]=bore.origin;
        let [width,depth,height]=bore.dimensions;
        let stock=SolidPrimitive::Box{origin:Pnt3::new(x,y,z),width,depth,height};
        if bore.first_label.checked_add(31).is_none(){return Err(invalid_preview());}
        let mut job=Self::new(source,stock,bore.first_label,tolerance)?;
        let sampling=BoreSampling::new(bore,tolerance)?;
        job.segments=sampling.segments();
        job.primitive.bore=Some(sampling);
        job.triangle_slots=(0..job.primitive.face_count()).map(|face|job.chart(face).slots(job.segments)).sum();
        job.edge_segments=(0..job.primitive.edge_count()).map(|edge|job.segments_on_edge(edge)).sum();
        Ok(job)
    }

    pub fn is_bore(&self)->bool{self.primitive.bore.is_some()}

    /// 🔗️ Returns the arena identity whose liveness the owning engine checks before each step.
    pub fn source(&self) -> SolidId {
        self.source
    }

    /// 🏁️ Reports whether the final mesh has been transferred.
    pub fn is_done(&self) -> bool {
        self.done
    }

    fn chart(&self, face: usize) -> Chart {
        if self.primitive.bore.is_some() && matches!(face,0|1|BorePreviewSource::BORE_WALL_FACE) { return Chart::Bore; }
        match self.primitive.solid {
            SolidPrimitive::Box { .. } => Chart::Rectangle,
            SolidPrimitive::Sphere { .. } => Chart::Wrapped { first: -FRAC_PI_2, end: FRAC_PI_2, rows: self.segments.div_ceil(2), low_pole: true, high_pole: true },
            SolidPrimitive::Cylinder { height, .. } if face == 0 => Chart::Wrapped { first: 0.0, end: height, rows: 1, low_pole: false, high_pole: false },
            SolidPrimitive::Cone { height, .. } if face == 0 => Chart::Wrapped { first: 0.0, end: height, rows: 1, low_pole: true, high_pole: false },
            SolidPrimitive::Cylinder { radius, .. } | SolidPrimitive::Cone { radius, .. } => Chart::Cap { radius },
            SolidPrimitive::Torus { .. } => Chart::Wrapped { first: 0.0, end: TAU, rows: self.tube_segments, low_pole: false, high_pole: false },
        }
    }

    fn segments_on_edge(&self, edge: usize) -> usize {
        let spec = self.primitive.edge(edge);
        match spec.curve {
            Curve3::Line { dir, .. } => usize::from(dir != NativeVec3::ZERO),
            Curve3::Circle { .. } => {
                let turns = if spec.range.1 - spec.range.0 < TAU { self.segments.div_ceil(2) } else if matches!(self.primitive.solid, SolidPrimitive::Torus { .. }) && edge == 0 { self.tube_segments } else { self.segments };
                turns.max(1)
            }
            _ => unreachable!("analytic primitive edges have closed forms"),
        }
    }

    fn triangle(&self) -> Option<[Vertex; 3]> {
        let face = self.face;
        let primitive = self.primitive.solid;
        if let Some(bore)=self.primitive.bore {
            match face {
                0|1=>return Some(bore.cap_triangle(face==1,self.slot)),
                BorePreviewSource::BORE_WALL_FACE=>return Some(bore.wall_triangle(self.slot)),
                _=>{}
            }
        }
        let surface = primitive.surface(face);
        let angle = |column| TAU * column as f64 / self.segments as f64;
        match self.chart(face) {
            Chart::Bore=>unreachable!("certified bore chart dispatched above"),
            Chart::Rectangle => {
                let Surface::Plane { frame } = surface else { unreachable!("box face is planar") };
                let corners = if self.slot == 0 { [0, 1, 2] } else { [0, 2, 3] };
                Some(corners.map(|i| {
                    let trim = primitive.trim(face, i);
                    let edge = primitive.edge(trim.edge);
                    Vertex { point: primitive.vertex(edge.vertices[usize::from(!trim.forward)]), normal: frame.z }
                }))
            }
            Chart::Cap { radius } => {
                let Surface::Plane { frame } = surface else { unreachable!("analytic cap is planar") };
                let point = |theta: f64| frame.to_world(Pnt3::new(radius * theta.cos(), radius * theta.sin(), 0.0));
                Some([frame.origin, point(angle(self.slot)), point(angle((self.slot + 1) % self.segments))].map(|point| Vertex { point, normal: frame.z }))
            }
            Chart::Wrapped { first, end, rows, low_pole, high_pole } => {
                let side = self.slot % 2;
                let column = (self.slot / 2) % self.segments;
                let row = (self.slot / 2) / self.segments;
                if (low_pole && row == 0 && side == 0) || (high_pole && row + 1 == rows && side == 1) { return None; }
                let a = (column, row);
                let b = ((column + 1) % self.segments, row);
                let c = ((column + 1) % self.segments, row + 1);
                let d = (column, row + 1);
                Some((if side == 0 { [a, b, c] } else { [a, c, d] }).map(|(column, r)| {
                    let u = angle(column);
                    let v = if first == 0.0 && end == TAU && r == rows { 0.0 } else { first + (end - first) * r as f64 / rows as f64 };
                    if let Surface::Sphere { frame, radius } = surface {
                        if r == 0 || r == rows {
                            let normal = if r == 0 { -frame.z } else { frame.z };
                            return Vertex { point: frame.origin + normal * radius, normal };
                        }
                    }
                    let radial = NativeVec3::new(u.cos(), u.sin(), 0.0);
                    let normal = match &surface {
                        Surface::Cone { frame, half_angle } =>
                            frame.to_world_vector(radial * half_angle.cos() - NativeVec3::Z * half_angle.sin()),
                        Surface::Cylinder { frame, .. } => frame.to_world_vector(radial),
                        Surface::Sphere { frame, .. } | Surface::Torus { frame, .. } =>
                            frame.to_world_vector(radial * v.cos() + NativeVec3::Z * v.sin()),
                        _ => unreachable!("wrapped primitive surface"),
                    };
                    Vertex { point: surface.eval(u, v), normal }
                }))
            }
        }
    }

    fn reference_count(&self, domain: usize) -> usize {
        match domain {
            0 => (0..self.primitive.edge_count()).filter(|&edge| self.segments_on_edge(edge) > 0).count(),
            1 => self.primitive.face_count(),
            2 => self.primitive.vertex_count(),
            _ => unreachable!("primitive picking domain"),
        }
    }

    fn next_reference_label(&self, domain: usize) -> u64 {
        let index = self.reference_labels.len();
        let primitive = self.primitive;
        let offset = match domain {
            0 => primitive.vertex_count() + (0..primitive.edge_count()).filter(|&edge| self.segments_on_edge(edge) > 0).nth(index).expect("visible primitive edge"),
            1 => primitive.vertex_count() + primitive.edge_count() + index,
            2 => index,
            _ => unreachable!("primitive picking domain"),
        };
        self.primitive.first_label + offset as u64
    }

    pub fn progress(&self) -> (usize, usize, &'static str) {
        let edge_units: usize = (0..self.primitive.edge_count()).map(|edge| self.segments_on_edge(edge).max(1)).sum();
        let reference_units: usize = (0..DOMAINS.len()).map(|domain| self.reference_count(domain) + 3).sum();
        let total = self.triangle_slots + edge_units + self.primitive.vertex_count() + reference_units + STORAGE_STAGES + 1;
        let phase = if self.storage < STORAGE_STAGES { "primitive-preview-storage" }
            else if self.face < self.primitive.face_count() { "primitive-preview-faces" }
            else if self.edge < self.primitive.edge_count() { "primitive-preview-edges" }
            else if self.vertex < self.primitive.vertex_count() { "primitive-preview-vertices" }
            else { "primitive-preview-components" };
        (self.units_done, total.max(self.units_done), phase)
    }

    fn extent(&self) -> usize {
        match self.storage {
            0 | 1 => self.triangle_slots * POSITION_SCALARS_PER_TRIANGLE + self.primitive.vertex_count() * 3,
            2 => self.triangle_slots * INDEX_SCALARS_PER_TRIANGLE,
            3 => self.edge_segments * EDGE_SCALARS_PER_SEGMENT,
            4 => self.triangle_slots,
            5 => self.edge_segments,
            6 => self.triangle_slots * 3 + self.primitive.vertex_count(),
            _ => DOMAINS.len(),
        }
    }

    fn reference_demand(&self) -> (usize, usize) {
        let domain = self.references.len();
        if domain == DOMAINS.len() { return (0, 0); }
        if self.reference_key.is_empty() { return (DOMAINS[domain].len(), DOMAINS[domain].len()); }
        let count = self.reference_count(domain);
        if self.reference_labels.capacity() == 0 { return (0, count * size_of::<String>()); }
        if self.reference_labels.len() == count { return (0, 0); }
        let text = PersistentLabel(self.next_reference_label(domain)).decimal(&mut [0; 20]).len();
        (text, text)
    }

    /// 🎟️ Quotes the next original preview allocation or one complete geometric sample.
    pub fn demand(&self) -> RetirementDemand {
        if self.done { return RetirementDemand::default(); }
        let (copy_bytes, capacity_bytes) = if self.storage < STORAGE_STAGES {
            let item = if self.storage == STORAGE_STAGES - 1 { size_of::<(String, Vec<String>)>() } else { size_of::<f32>() };
            (0, self.extent() * item)
        } else if self.face < self.primitive.face_count() { (TRIANGLE_BYTES, 0) }
        else if self.edge < self.primitive.edge_count() { (EDGE_BYTES, 0) }
        else if self.vertex < self.primitive.vertex_count() { (POINT_BYTES, 0) }
        else { self.reference_demand() };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    fn build_reference(&mut self, progress: &mut Progress) -> Result<(), ValueError> {
        let domain = self.references.len();
        let count = self.reference_count(domain);
        if self.reference_key.is_empty() {
            self.reference_key = allocate_text(DOMAINS[domain])?;
            progress.copied_bytes = self.reference_key.len();
            progress.retained_capacity_bytes = self.reference_key.capacity();
        } else if self.reference_labels.capacity() == 0 {
            progress.retained_capacity_bytes = allocate(&mut self.reference_labels, count)?;
        } else if self.reference_labels.len() < count {
            let label = self.next_reference_label(domain);
            let mut buf = [0; 20];
            let text = allocate_text(PersistentLabel(label).decimal(&mut buf))?;
            progress.copied_bytes = text.len();
            progress.retained_capacity_bytes = text.capacity();
            self.reference_labels.push(text);
        } else {
            self.references.push((take(&mut self.reference_key), take(&mut self.reference_labels)));
        }
        Ok(())
    }

    /// 🧱️ Advances the job by at most one granted geometry or buffer event.
    pub fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<MeshData>, Progress), ValueError> {
        let grant = effect.grant();
        if self.done || !retained_clone_grant_admits(grant, self.demand()) || !effect.performed(1)? {
            return Ok((None, Progress::default()));
        }
        self.units_done += 1;
        let mut progress = Progress { copied_items: 1, ..Default::default() };
        if self.storage < STORAGE_STAGES {
            let extent = self.extent();
            progress.retained_capacity_bytes = match self.storage {
                0 => allocate(&mut self.output.positions, extent)?,
                1 => allocate(&mut self.output.normals, extent)?,
                2 => allocate(&mut self.output.indices, extent)?,
                3 => allocate(&mut self.output.edge_positions, extent)?,
                4 => allocate(&mut self.output.face_ids, extent)?,
                5 => allocate(&mut self.output.edge_ids, extent)?,
                6 => allocate(&mut self.output.vertex_ids, extent)?,
                _ => allocate(&mut self.references, extent)?,
            };
            self.storage += 1;
            return Ok((None, progress));
        }
        if self.face < self.primitive.face_count() {
            if let Some(triangle) = self.triangle() {
                let base = (self.output.positions.len() / 3) as u32;
                let normal = triangle[0].normal + triangle[1].normal + triangle[2].normal;
                let cross = (triangle[1].point - triangle[0].point).cross(triangle[2].point - triangle[0].point);
                let orientation = if cross.dot(normal) < 0.0 { [base, base + 2, base + 1] } else { [base, base + 1, base + 2] };
                self.output.indices.extend_from_slice(&orientation);
                self.output.face_ids.push(self.face as u32);
                self.output.vertex_ids.extend_from_slice(&[u32::MAX; 3]);
                for vertex in triangle {
                    self.output.positions.extend_from_slice(&point_components(vertex.point));
                    self.output.normals.extend_from_slice(&normal_components(vertex.normal));
                }
                progress.copied_bytes = TRIANGLE_BYTES;
            }
            self.slot += 1;
            if self.slot == self.chart(self.face).slots(self.segments) { self.slot = 0; self.face += 1; }
            return Ok((None, progress));
        }
        if self.edge < self.primitive.edge_count() {
            let segments = self.segments_on_edge(self.edge);
            if self.segment < segments {
                let edge = self.primitive.edge(self.edge);
                let (start, end) = edge.range;
                let point = |i| {
                    if let Some(bore)=self.primitive.bore {
                        if matches!(self.edge,BorePreviewSource::BOTTOM_RING_EDGE|BorePreviewSource::TOP_RING_EDGE){
                            return bore.rim_point(self.edge==BorePreviewSource::TOP_RING_EDGE,i);
                        }
                    }
                    if i == 0 { return self.primitive.vertex(edge.vertices[0]); }
                    if i == segments { return self.primitive.vertex(edge.vertices[1]); }
                    edge.curve.eval(start + (end - start) * i as f64 / segments as f64)
                };
                self.output.edge_positions.extend_from_slice(&point_components(point(self.segment)));
                self.output.edge_positions.extend_from_slice(&point_components(point(self.segment + 1)));
                self.output.edge_ids.push(self.edge_pick);
                self.segment += 1;
                progress.copied_bytes = EDGE_BYTES;
            }
            if self.segment == segments {
                if segments > 0 { self.edge_pick += 1; }
                self.segment = 0;
                self.edge += 1;
            }
            return Ok((None, progress));
        }
        if self.vertex < self.primitive.vertex_count() {
            self.output.positions.extend_from_slice(&point_components(self.primitive.vertex(self.vertex)));
            self.output.normals.extend_from_slice(&[0.0, 0.0, 0.0]);
            self.output.vertex_ids.push(self.vertex as u32);
            self.vertex += 1;
            progress.copied_bytes = POINT_BYTES;
            return Ok((None, progress));
        }
        if self.references.len() < DOMAINS.len() {
            self.build_reference(&mut progress)?;
            return Ok((None, progress));
        }
        self.output.component_references = ComponentReferenceTable::from_entries(take(&mut self.references));
        self.done = true;
        Ok((Some(take(&mut self.output)), progress))
    }
}
