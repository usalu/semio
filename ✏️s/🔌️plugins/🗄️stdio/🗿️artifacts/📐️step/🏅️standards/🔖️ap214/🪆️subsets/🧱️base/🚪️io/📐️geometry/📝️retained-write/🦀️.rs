//! ✍️ Retained STEP Part 21 emission over one original solid boundary.

use std::fmt::{self, Write};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::{ArenaId, CoedgeId, FaceId, LoopId, SolidId}, topology::Body};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::matrix::Frame3;

#[path = "🛢️cylinder.rs"]
mod cylinder;
use cylinder::CylinderExchange;
#[path = "〰️splines.rs"]
mod splines;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits};

const PREFIX: &str = concat!(
    "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('semio planar solid'),'2;1');\n",
    "FILE_NAME('solid.step','2024-01-01T00:00:00',(''),(''),'semio','semio','');\n",
    "FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));\nENDSEC;\nDATA;\n",
    "#1 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );\n",
    "#2 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );\n",
    "#3 = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );\n",
    "#4 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#1,'distance_accuracy_value','');\n",
    "#5 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#4)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#1,#2,#3)) REPRESENTATION_CONTEXT('Context3D','3D Context') );\n",
    "#6 = APPLICATION_CONTEXT('configuration controlled 3D design of mechanical parts and assemblies');\n",
    "#7 = MECHANICAL_CONTEXT('',#6,'mechanical');\n",
    "#8 = APPLICATION_PROTOCOL_DEFINITION('international standard','config_control_design',1994,#6);\n",
    "#9 = PRODUCT('semio_solid','semio_solid','',(#7));\n",
    "#10 = PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE('','',#9,.NOT_KNOWN.);\n",
    "#11 = PRODUCT_DEFINITION_CONTEXT('part definition',#6,'design');\n",
    "#12 = PRODUCT_DEFINITION('design','',#10,#11);\n",
    "#13 = PRODUCT_DEFINITION_SHAPE('','',#12);\n",
    "#14 = SHAPE_DEFINITION_REPRESENTATION(#13,#15);\n",
    "#15 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#16,#20),#5);\n",
    "#16 = AXIS2_PLACEMENT_3D('',#17,#18,$);\n",
    "#17 = CARTESIAN_POINT('',(0.,0.,0.));\n",
    "#18 = DIRECTION('',(0.,0.,1.));\n",
    "#21 = ( GEOMETRIC_REPRESENTATION_CONTEXT(2) PARAMETRIC_REPRESENTATION_CONTEXT() REPRESENTATION_CONTEXT('2D','') );\n",
    "#20 = MANIFOLD_SOLID_BREP('',#19);\n#19 = CLOSED_SHELL('',(",
);
const SUFFIX: &str = "ENDSEC;\nEND-ISO-10303-21;\n";
const CHUNK: usize = 64;
const MAX_FACES: usize = 256;
const MAX_RINGS: usize = 128;
const MAX_EDGES_PER_RING: usize = 512;
const MAX_RECORD_BYTES: usize = 2048;
const MAX_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const INDEX_SIZE: usize = 2048;
const INDEX_PROBES: usize = 32;

fn unsupported(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::UnsupportedOwner, message) }
fn missing() -> ValueError { ValueError::invariant("certified STEP source lost its native boundary") }

struct Piece { bytes: [u8; MAX_RECORD_BYTES], len: usize }

impl Piece {
    fn put(&mut self, text: fmt::Arguments<'_>) -> Result<(), ValueError> {
        self.write_fmt(text).map_err(|_| unsupported("STEP output record exceeds its retained encoding bound"))
    }

    fn get(&self, start: usize, end: usize) -> &str { std::str::from_utf8(&self.bytes[start..end]).unwrap() }

    fn placement(&mut self, frame: Frame3, id:u64, origin:u64, normal:u64, tangent:u64) -> Result<(),ValueError> {
        self.point(origin,"CARTESIAN_POINT",frame.origin.to_array())?;
        self.point(normal,"DIRECTION",frame.z.to_array())?;
        self.point(tangent,"DIRECTION",frame.x.to_array())?;
        self.put(format_args!("#{id} = AXIS2_PLACEMENT_3D('',#{origin},#{normal},#{tangent});\n"))
    }

    fn point(&mut self, id: u64, entity: &'static str, [x,y,z]: [f64;3]) -> Result<(), ValueError> {
        if ![x,y,z].into_iter().all(f64::is_finite) { return Err(missing()); }
        self.put(format_args!("#{id} = {entity}('',({x:.16E},{y:.16E},{z:.16E}));\n"))
    }
}

impl Write for Piece {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.len + text.len();
        self.bytes.get_mut(self.len..end).ok_or(fmt::Error)?.copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Pass { Measure, Allocate, Write, Ready }

#[derive(Clone, Copy)]
enum Walk {
    Prefix,
    ShellFace(usize),
    FaceOpen(usize),
    FaceBound(FaceId, usize),
    LoopOpen(FaceId, usize),
    LoopMember { face: FaceId, index: usize, ring: LoopId, first: CoedgeId, current: CoedgeId, count: usize },
    LoopRecords { face: FaceId, index: usize, ring: LoopId, first: CoedgeId, current: CoedgeId },
    Suffix,
    Done,
}

/// 🆔 One certified original native solid borrowed under its session lock.
#[derive(Clone, Copy)]
pub struct StepWriteSource<'a> {
    pub body: &'a Body,
    pub solid: SolidId,
    pub revision: u64,
    pub handle: &'a str,
    pub depth: usize,
}

/// 🧾 Original STEP writer with exact source revision and reversible output custody.
pub struct StepWrite {
    source: (usize, usize),
    solid: SolidId,
    revision: u64,
    pass: Pass,
    walk: Walk,
    shell_index: usize,
    seen: [u64; INDEX_SIZE],
    cylinder: Option<CylinderExchange>,
    piece: Piece,
    offset: usize,
    measured: usize,
    output: Option<String>,
}

impl StepWrite {
    /// 🧷 Binds one original Solid and the exact borrowed source handle to this cursor.
    pub fn new(StepWriteSource { body, solid, revision, handle, .. }: StepWriteSource<'_>) -> Self {
        Self {
            source: (std::ptr::from_ref(body) as usize, handle.as_ptr() as usize),
            solid, revision, pass: Pass::Measure, walk: Walk::Prefix, shell_index: 0,
            seen: [0; INDEX_SIZE], cylinder: None, piece: Piece { bytes: [0; MAX_RECORD_BYTES], len: 0 }, offset: 0, measured: 0, output: None,
        }
    }

    /// 📤 Reports whether the STEP envelope and topology are complete.
    pub fn ready(&self) -> bool { matches!(self.pass, Pass::Ready) }

    /// ✍️ Reports the retained text phase.
    pub fn phase(&self) -> &'static str {
        if matches!(self.pass, Pass::Write | Pass::Ready) { "step-export-encoding" } else { "step-export-sizing" }
    }

    /// 🧺 Reports the pending document allocation for guaranteed cancellation.
    pub fn output_capacity(&self) -> usize { self.output.as_ref().map_or(0, String::capacity) }

    /// 🪙 Quotes the next bounded source read, exact reservation, or text copy.
    pub fn demand(&self, source: StepWriteSource<'_>) -> Result<RetirementDemand, ValueError> {
        self.check(source)?;
        let mut demand = RetirementDemand { depth: source.depth, ..Default::default() };
        match self.pass {
            Pass::Allocate => demand.capacity_bytes = self.measured,
            Pass::Write if self.offset < self.piece.len => demand.copy_bytes = (self.piece.len - self.offset).min(CHUNK),
            Pass::Measure | Pass::Write => {
                demand.depth = demand.depth.max(self.walk_depth(source.body)?);
                if !matches!(self.walk, Walk::Done) {
                    demand.copy_bytes = MAX_RECORD_BYTES + self.source_copy_bytes();
                }
            }
            Pass::Ready => {}
        }
        Ok(demand)
    }

    /// ⏭️ Advances one native record, owner reservation, or bounded output chunk.
    pub fn step(&mut self, source: StepWriteSource<'_>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let demand = self.demand(source)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(Progress::default()); }
        let mut receipt = Progress { copied_items: 1, ..Default::default() };
        self.advance(source.body, demand, &mut receipt).map_err(|error| error.with_retained_progress(receipt))?;
        if !receipt.fits(effect.grant()) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "STEP export exceeded its granted capacity").with_retained_progress(receipt));
        }
        Ok(receipt)
    }

    /// 📦 Relinquishes the completed text to the already-funded registered output owner.
    pub fn take_output(&mut self, source: StepWriteSource<'_>) -> Result<String, ValueError> {
        self.check(source)?;
        if !self.ready() { return Err(ValueError::invariant("STEP writer has no completed output")); }
        self.output.take().ok_or_else(|| ValueError::invariant("STEP output was already transferred"))
    }

    fn check(&self, StepWriteSource { body, solid, revision, handle, .. }: StepWriteSource<'_>) -> Result<(), ValueError> {
        if self.solid != solid || self.revision != revision || handle.len() != 64
            || self.source != (std::ptr::from_ref(body) as usize, handle.as_ptr() as usize) {
            return Err(ValueError::invariant("original STEP export source changed"));
        }
        Ok(())
    }

    fn advance(&mut self, body: &Body, demand: RetirementDemand, receipt: &mut Progress) -> Result<(), ValueError> {
        match self.pass {
            Pass::Measure => {
                self.render_record(body, 1, receipt)?;
                self.measured += self.piece.len;
                if self.measured > MAX_OUTPUT_BYTES { return Err(unsupported("STEP document exceeds the retained output limit")); }
                if matches!(self.walk, Walk::Done) { self.pass = Pass::Allocate; }
            }
            Pass::Allocate => {
                let mut output = String::new();
                output.try_reserve_exact(self.measured).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "STEP document could not be allocated"))?;
                receipt.retained_capacity_bytes = output.capacity();
                self.output = Some(output);
                self.walk = Walk::Prefix;
                self.shell_index = 0;
                self.piece.len = 0;
                self.pass = Pass::Write;
            }
            Pass::Write => {
                if self.offset < self.piece.len {
                    let end = self.offset + demand.copy_bytes;
                    let output = self.output.as_mut().unwrap();
                    if output.len() + demand.copy_bytes > self.measured { return Err(missing()); }
                    output.push_str(self.piece.get(self.offset, end));
                    self.offset = end;
                    receipt.copied_bytes = demand.copy_bytes;
                } else if matches!(self.walk, Walk::Done) {
                    if self.output.as_ref().unwrap().len() != self.measured { return Err(missing()); }
                    self.pass = Pass::Ready;
                } else {
                    self.render_record(body, 2, receipt)?;
                }
            }
            Pass::Ready => return Err(ValueError::invariant("STEP writer is already complete")),
        }
        Ok(())
    }

    fn source_copy_bytes(&self) -> usize {
        match self.walk {
            Walk::Prefix => std::mem::size_of::<CylinderExchange>(),
            Walk::Suffix | Walk::Done => 0,
            Walk::FaceOpen(_) | Walk::LoopRecords { .. } => 160,
            _ => 32,
        }
    }

    fn render_record(&mut self, body: &Body, pass: u64, receipt: &mut Progress) -> Result<(), ValueError> {
        let source_bytes = self.source_copy_bytes();
        let result = self.render(body, pass);
        receipt.copied_bytes = source_bytes + self.piece.len;
        result
    }

    fn shell<'a>(&self, body: &'a Body) -> Result<(semio_framework_3d_brep_representation::arena::ShellId, &'a [FaceId]), ValueError> {
        let solid = body.solids.get(self.solid).ok_or_else(missing)?;
        if !solid.inners.is_empty() { return Err(unsupported("STEP export requires one outer solid shell")); }
        let shell = body.shells.get(solid.outer).ok_or_else(missing)?;
        if shell.faces.is_empty() || shell.faces.len() > MAX_FACES { return Err(unsupported("STEP shell exceeds its retained face limit")); }
        Ok((solid.outer, &shell.faces))
    }

    fn walk_depth(&self, body: &Body) -> Result<usize, ValueError> {
        let mut depth = body.solids.next_remove_depth_demand(self.solid)?;
        let (shell, faces) = self.shell(body)?;
        depth = depth.max(body.shells.next_remove_depth_demand(shell)?);
        macro_rules! include_depth { ($store:ident, $id:expr) => { depth = depth.max(body.$store.next_remove_depth_demand($id)?); }; }
        match self.walk {
            Walk::Prefix if faces.len() == 3 && self.cylinder.is_none() => {
                depth = depth.max(CylinderExchange::read(body,faces)?.1);
            }
            Walk::FaceOpen(index) => {
                let id = faces[index];
                include_depth!(faces, id);
                include_depth!(surfaces, body.faces.get(id).ok_or_else(missing)?.surface);
            }
            Walk::FaceBound(id, _) | Walk::LoopOpen(id, _) => {
                include_depth!(faces, id);
                if let Walk::LoopOpen(_, index) = self.walk {
                    include_depth!(loops, loop_id(body, id, index)?);
                }
            }
            Walk::LoopMember { current, .. } => { include_depth!(coedges, current); }
            Walk::LoopRecords { current, face, .. } => {
                include_depth!(faces, face);
                include_depth!(coedges, current);
                let id = body.coedges.get(current).ok_or_else(missing)?.edge;
                include_depth!(edges, id);
                let edge = body.edges.get(id).ok_or_else(missing)?;
                include_depth!(vertices, edge.v0);
                include_depth!(vertices, edge.v1);
                include_depth!(curves3, edge.curve);
            }
            _ => {}
        }
        Ok(depth)
    }

    fn render(&mut self, body: &Body, pass: u64) -> Result<(), ValueError> {
        self.piece.len = 0;
        self.offset = 0;
        let (_, shell_faces) = self.shell(body)?;
        self.walk = match self.walk {
            Walk::Prefix => {
                if shell_faces.len() == 3 && self.cylinder.is_none() { self.cylinder = Some(CylinderExchange::read(body,shell_faces)?.0); }
                self.piece.put(format_args!("{PREFIX}"))?;
                Walk::ShellFace(0)
            }
            Walk::ShellFace(index) => {
                let id = row(shell_faces[index], 1)?;
                let last = index + 1 == shell_faces.len();
                self.piece.put(format_args!("#{id}{}", if last { "));\n" } else { "," }))?;
                if last { Walk::FaceOpen(0) } else { Walk::ShellFace(index+1) }
            }
            Walk::FaceOpen(index) => {
                let id = shell_faces[index];
                let face = body.faces.get(id).ok_or_else(missing)?;
                if face.inners.len() >= MAX_RINGS || face.outer.is_none() { return Err(unsupported("STEP face has unsupported boundary count")); }
                let support = row(id, 2)?;
                let points = [row(id, 3)?, row(id, 4)?, row(id, 5)?, row(id, 6)?];
                let [placement, origin, normal, tangent] = points;
                let face_id = row(id, 1)?;
                match body.surfaces.get(face.surface).ok_or_else(missing)? {
                    Surface::Plane { frame } => {
                        self.piece.placement(*frame, placement, origin, normal, tangent)?;
                        self.piece.put(format_args!("#{support} = PLANE('',#{placement});\n"))?;
                    }
                    Surface::Cylinder { frame, radius } | Surface::Sphere { frame, radius } if *radius > 0.0 && radius.is_finite() => {
                        self.piece.placement(*frame, placement, origin, normal, tangent)?;
                        let entity = if matches!(body.surfaces.get(face.surface), Some(Surface::Sphere { .. })) { "SPHERICAL_SURFACE" } else { "CYLINDRICAL_SURFACE" };
                        self.piece.put(format_args!("#{support} = {entity}('',#{placement},{radius:.16E});\n"))?;
                    }
                    spline @ Surface::Nurbs { .. } => splines::surface(&mut self.piece, spline, support, points)?,
                    _ => return Err(unsupported("STEP export requires original planar, cylindrical, spherical or bilinear spline faces")),
                }
                self.piece.put(format_args!("#{face_id} = ADVANCED_FACE('',("))?;
                self.shell_index = index;
                Walk::FaceBound(id, 0)
            }
            Walk::FaceBound(face, index) => {
                let id = row(loop_id(body, face, index)?, 7)?;
                let record = body.faces.get(face).ok_or_else(missing)?;
                if index < record.inners.len() {
                    self.piece.put(format_args!("#{id},"))?;
                    Walk::FaceBound(face,index+1)
                } else {
                    let plane = row(face, 2)?;
                    let sense = sense(!record.flipped);
                    self.piece.put(format_args!("#{id}),#{plane},{sense});\n"))?;
                    Walk::LoopOpen(face,0)
                }
            }
            Walk::LoopOpen(face,index) => {
                let ring = loop_id(body,face,index)?;
                let record = body.loops.get(ring).ok_or_else(missing)?;
                if record.face != face { return Err(missing()); }
                let bound = row(ring,7)?;
                let edge_loop = row(ring,8)?;
                let entity = if index == 0 { "FACE_OUTER_BOUND" } else { "FACE_BOUND" };
                self.piece.put(format_args!("#{bound} = {entity}('',#{edge_loop},.T.);\n#{edge_loop} = EDGE_LOOP('',("))?;
                Walk::LoopMember { face, index, ring, first: record.first, current: record.first, count: 0 }
            }
            Walk::LoopMember { face,index,ring,first,current,count } => {
                let coedge = body.coedges.get(current).ok_or_else(missing)?;
                let id = row(current,9)?;
                let last = coedge.next == first;
                let closed_cap = count == 0 && self.cylinder.as_ref().is_some_and(|source| source.wall != face);
                if coedge.loop_id != ring || (last && count < 2 && !closed_cap) { return Err(missing()); }
                if count >= MAX_EDGES_PER_RING { return Err(unsupported("STEP loop exceeds its retained coedge limit")); }
                self.piece.put(format_args!("#{id}{}", if last { "));\n" } else { "," }))?;
                if last { Walk::LoopRecords { face,index,ring,first,current:first } }
                else { Walk::LoopMember { face,index,ring,first,current:coedge.next,count:count+1 } }
            }
            Walk::LoopRecords { face,index,ring,first,current } => {
                let coedge = body.coedges.get(current).ok_or_else(missing)?;
                self.edge(body, current, pass)?;
                if coedge.next != first {
                    Walk::LoopRecords { face,index,ring,first,current:coedge.next }
                } else if index < body.faces.get(face).ok_or_else(missing)?.inners.len() {
                    Walk::LoopOpen(face,index+1)
                } else if self.shell_index + 1 < shell_faces.len() {
                    Walk::FaceOpen(self.shell_index+1)
                } else { Walk::Suffix }
            }
            Walk::Suffix => {
                self.piece.put(format_args!("{SUFFIX}"))?;
                Walk::Done
            }
            Walk::Done => return Err(missing()),
        };
        Ok(())
    }

    fn edge(&mut self, body: &Body, current: CoedgeId, pass: u64) -> Result<(), ValueError> {
        let coedge = body.coedges.get(current).ok_or_else(missing)?;
        let native = body.edges.get(coedge.edge).ok_or_else(missing)?;
        let edge = row(coedge.edge,10)?;
        if put_once(&mut self.seen, edge, pass)? {
            let vertices = [native.v0, native.v1];
            let position = |id| body.vertices.get(id).map(|vertex| vertex.position).ok_or_else(missing);
            let ends = [position(vertices[0])?, position(vertices[1])?];
            for (vertex, position) in vertices.into_iter().zip(ends) {
                let id = row(vertex,11)?;
                if put_once(&mut self.seen, id, pass)? {
                    let point = row(vertex,12)?;
                    self.piece.point(point, "CARTESIAN_POINT", position.to_array())?;
                    self.piece.put(format_args!("#{id} = VERTEX_POINT('',#{point});\n"))?;
                }
            }
            let [origin,direction,vector,geometry] = [13,14,15,16].map(|kind| row(coedge.edge,kind).unwrap());
            match body.curves3.get(native.curve).ok_or_else(missing)? {
                Curve3::Line { origin: at,dir } => {
                    let [x,y,z] = dir.to_array();
                    let length = x.hypot(y).hypot(z);
                    if !(length.is_finite() && length > 0.0 && native.range.0.is_finite() && native.range.1.is_finite() && native.range.0 != native.range.1) {
                        return Err(missing());
                    }
                    self.piece.point(origin, "CARTESIAN_POINT", at.to_array())?;
                    self.piece.point(direction, "DIRECTION", dir.to_array())?;
                    self.piece.put(format_args!("#{vector} = VECTOR('',#{direction},{length:.16E});\n#{geometry} = LINE('',#{origin},#{vector});\n"))?;
                }
                curve @ Curve3::Circle { frame,radius } => {
                    let (first,last) = native.range;
                    let span = (last-first).abs();
                    let turn = std::f64::consts::TAU;
                    if !(span > 0.0 && span <= turn && *radius > 0.0 && radius.is_finite())
                        || (native.v0 == native.v1) != (span == turn)
                        || ![first,last].into_iter().zip(ends).all(|(p,end)| (curve.eval(p)-end).norm() <= native.tol.value()) {
                        return Err(unsupported("STEP circular edge requires finite range and matching vertices"));
                    }
                    let placement = row(coedge.edge,24)?;
                    self.piece.placement(*frame,placement,origin,direction,vector)?;
                    self.piece.put(format_args!("#{geometry} = CIRCLE('',#{placement},{radius:.16E});\n"))?;
                }
                spline @ Curve3::Nurbs { .. } => splines::curve(&mut self.piece, spline, geometry, [origin,direction], native.range, ends)?,
                _ => return Err(unsupported("STEP export requires original straight, clamped spline or circular edge trims")),
            }
            let curve = if let Some(cylinder) = &self.cylinder { cylinder.curve(&mut self.piece,coedge.edge,geometry)? }
                else { geometry };
            let v0 = row(native.v0,11)?;
            let v1 = row(native.v1,11)?;
            let same = sense(native.range.1 > native.range.0);
            self.piece.put(format_args!("#{edge} = EDGE_CURVE('',#{v0},#{v1},#{curve},{same});\n"))?;
        }
        let id = row(current,9)?;
        let orientation = sense(coedge.forward);
        self.piece.put(format_args!("#{id} = ORIENTED_EDGE('',*,*,#{edge},{orientation});\n"))
    }
}

fn loop_id(body: &Body, id: FaceId, index: usize) -> Result<LoopId, ValueError> {
    let face = body.faces.get(id).ok_or_else(missing)?;
    match index { 0 => face.outer, _ => face.inners.get(index-1).copied() }.ok_or_else(missing)
}

fn sense(increasing: bool) -> &'static str { if increasing { ".T." } else { ".F." } }

fn row(id: impl ArenaId, field: u64) -> Result<u64, ValueError> {
    let base = u64::from(id.raw_index()) * 24 + 100;
    if base + 24 > i32::MAX as u64 { return Err(unsupported("STEP original arena exceeds the exchange identifier range")); }
    Ok(base + field)
}

fn put_once(table: &mut [u64; INDEX_SIZE], key: u64, pass: u64) -> Result<bool, ValueError> {
    let hash = key.wrapping_mul(0x9e3779b97f4a7c15);
    for probe in 0..INDEX_PROBES {
        let slot = &mut table[(((hash ^ (hash >> 32)) as usize).wrapping_add(probe)) & (INDEX_SIZE-1)];
        if *slot == 0 || *slot >> 2 == key {
            let fresh = *slot & pass == 0;
            *slot = (key << 2) | (*slot & 3) | pass;
            return Ok(fresh);
        }
    }
    Err(unsupported("STEP shared topology exceeds the retained index limit"))
}
