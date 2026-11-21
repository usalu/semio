//! 🛢️ Native circular trims and the paired parameter branches of a full cylinder.

use super::*;
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_representation::arena::EdgeId;
use semio_framework_3d_brep_vector::{Pnt3,matrix::Frame3};

const TURN: f64 = std::f64::consts::TAU;

#[derive(Clone, Copy)]
enum ParametricCurve {
    Line { at: [f64;2], direction: [f64;2] },
    Circle { at: [f64;2], radius: f64 },
}

#[derive(Clone, Copy)]
struct ParameterUse { coedge: CoedgeId, face: FaceId, curve: ParametricCurve }

#[derive(Clone, Copy)]
struct WallUse { edge: EdgeId, forward: bool, parameter: ParameterUse }

/// 🧵 Bounded original coedge evidence for a closed analytic cylinder.
pub(super) struct CylinderExchange {
    pub(super) wall: FaceId,
    seam: EdgeId,
    seam_uses: [ParameterUse;2],
    circles: [(EdgeId,[ParameterUse;2]);2],
}

impl CylinderExchange {
    pub(super) fn read(body: &Body, faces: &[FaceId]) -> Result<(Self,usize), ValueError> {
        let mut depth = 1;
        let mut wall = None;
        let mut caps = [None;2];
        let mut cap_count = 0;
        for &face in faces {
            include_depth(&mut depth,body.faces.next_remove_depth_demand(face)?);
            let native = body.faces.get(face).ok_or_else(missing)?;
            include_depth(&mut depth,body.surfaces.next_remove_depth_demand(native.surface)?);
            match body.surfaces.get(native.surface).ok_or_else(missing)? {
                Surface::Cylinder { frame,radius } if wall.is_none() && !native.flipped => wall = Some((face,*frame,*radius)),
                Surface::Plane { frame } if cap_count < 2 => {
                    caps[cap_count] = Some((face,*frame,native.flipped));
                    cap_count += 1;
                }
                _ => return Err(cylinder_required()),
            }
        }
        let (wall,frame,radius) = wall.ok_or_else(cylinder_required)?;
        if cap_count != 2 || !radius.is_finite() || radius <= 0.0 { return Err(cylinder_required()); }
        let wall_ring = ring::<4>(body,wall,&mut depth)?;
        let mut circles = [None;2];
        let mut seams = [None;2];
        let mut endpoints = [[None;2];4];
        for (member,id) in wall_ring.into_iter().enumerate() {
            let (usage, range) = parameter_use(body,id,wall,&mut depth)?;
            let edge = body.edges.get(usage.edge).ok_or_else(missing)?;
            endpoints[member] = if usage.forward { [Some(edge.v0),Some(edge.v1)] } else { [Some(edge.v1),Some(edge.v0)] };
            include_depth(&mut depth,body.curves3.next_remove_depth_demand(edge.curve)?);
            let ParametricCurve::Line { at, direction } = usage.parameter.curve else { return Err(cylinder_required()); };
            match body.curves3.get(edge.curve).ok_or_else(missing)? {
                Curve3::Circle { frame: circle,radius: size } => {
                    let index = usize::from(at[1] != 0.0);
                    if edge.v0 != edge.v1 || range != (0.0,TURN) || edge.range != range || *size != radius
                        || at[0] != 0.0 || direction != [1.0,0.0] || usage.forward != (index == 0)
                        || *circle != placed(frame,at[1]) || circles[index].is_some() {
                        return Err(cylinder_required());
                    }
                    circles[index] = Some((usage,at[1],edge.v0));
                }
                Curve3::Line { origin,dir } => {
                    let index = usize::from(at[0] != 0.0);
                    let branch = if index == 0 { 0.0 } else { TURN };
                    if at[0] != branch { return Err(cylinder_required()); }
                    if edge.v0 == edge.v1 || range != (0.0,1.0) || edge.range != range
                        || at[1] != 0.0 || direction[0] != 0.0 || !direction[1].is_finite() || direction[1] <= 0.0
                        || usage.forward != (index == 1) || seams[index].is_some() {
                        return Err(cylinder_required());
                    }
                    include_depth(&mut depth,body.vertices.next_remove_depth_demand(edge.v0)?);
                    include_depth(&mut depth,body.vertices.next_remove_depth_demand(edge.v1)?);
                    let a = body.vertices.get(edge.v0).ok_or_else(missing)?.position;
                    let b = body.vertices.get(edge.v1).ok_or_else(missing)?.position;
                    if *origin != a || *dir != b-a { return Err(cylinder_required()); }
                    seams[index] = Some((usage,direction[1],edge.v0,edge.v1));
                }
                _ => return Err(cylinder_required()),
            }
        }
        if (0..4).any(|member| endpoints[member][1] != endpoints[(member+1)%4][0]) { return Err(cylinder_required()); }
        let [Some((low,_,bottom)),Some((high,height,top))] = circles else { return Err(cylinder_required()); };
        let [Some((seam0,len,v0,v1)),Some((seam1,len2,_,_))] = seams else { return Err(cylinder_required()); };
        if seam0.edge != seam1.edge || len != height || len2 != height || v0 != bottom || v1 != top {
            return Err(cylinder_required());
        }
        for (id,elevation) in [(bottom,0.0),(top,height)] {
            let point = body.vertices.get(id).ok_or_else(missing)?.position;
            if point != frame.to_world(Pnt3::new(radius,0.0,elevation)) { return Err(cylinder_required()); }
        }
        let mut cap_uses = [None;2];
        for (face,plane,reversed) in caps.into_iter().flatten() {
            let [id] = ring::<1>(body,face,&mut depth)?;
            let (cap,range) = parameter_use(body,id,face,&mut depth)?;
            let index = usize::from(cap.edge != low.edge);
            let (wall_use, elevation, axis) = if index == 0 { (low,0.0,-frame.z) } else { (high,height,frame.z) };
            let sense = if reversed { -plane.z } else { plane.z };
            let ParametricCurve::Circle { at,radius: cap_radius } = cap.parameter.curve else { return Err(cylinder_required()); };
            if cap.edge != wall_use.edge || cap.forward == wall_use.forward || cap_uses[index].is_some()
                || plane.origin != placed(frame,elevation).origin || plane.x != frame.x || sense != axis || at != [0.0,0.0]
                || cap_radius != radius || range != (0.0, if cap.forward { TURN } else { -TURN }) {
                return Err(cylinder_required());
            }
            cap_uses[index] = Some(cap.parameter);
        }
        let [Some(low_cap),Some(high_cap)] = cap_uses else { return Err(cylinder_required()); };
        Ok((Self {
            wall, seam: seam0.edge, seam_uses: [seam1.parameter,seam0.parameter],
            circles: [(low.edge,[low.parameter,low_cap]),(high.edge,[high.parameter,high_cap])],
        },depth))
    }

    pub(super) fn curve(&self, piece: &mut Piece, edge: EdgeId, geometry: u64) -> Result<u64,ValueError> {
        let (entity, uses) = if edge == self.seam { ("SEAM_CURVE",&self.seam_uses) }
            else if let Some((_,uses)) = self.circles.iter().find(|(id,_)| *id == edge) { ("SURFACE_CURVE",uses) }
            else { return Err(cylinder_required()); };
        let a = uses[0].write(piece)?;
        let b = uses[1].write(piece)?;
        let id = row(edge,23)?;
        piece.put(format_args!("#{id} = {entity}('',#{geometry},(#{a},#{b}),.CURVE_3D.);\n"))?;
        Ok(id)
    }
}

impl ParameterUse {
    fn write(self, piece: &mut Piece) -> Result<u64,ValueError> {
        let pc = row(self.coedge,17)?;
        let definition = row(self.coedge,18)?;
        let curve = row(self.coedge,19)?;
        let point = row(self.coedge,20)?;
        let support = row(self.coedge,21)?;
        let surface = row(self.face,2)?;
        let [u,v] = match self.curve { ParametricCurve::Line { at,.. } | ParametricCurve::Circle { at,.. } => at };
        piece.put(format_args!("#{point} = CARTESIAN_POINT('',({u:.16E},{v:.16E}));\n"))?;
        match self.curve {
            ParametricCurve::Line { direction: [du,dv],.. } => {
                let d = row(self.coedge,22)?;
                let magnitude = du.hypot(dv);
                if !magnitude.is_finite() || magnitude <= 0.0 { return Err(cylinder_required()); }
                piece.put(format_args!("#{d} = DIRECTION('',({du:.16E},{dv:.16E}));\n#{support} = VECTOR('',#{d},{magnitude:.16E});\n#{curve} = LINE('',#{point},#{support});\n"))?;
            }
            ParametricCurve::Circle { radius,.. } => {
                piece.put(format_args!("#{support} = AXIS2_PLACEMENT_2D('',#{point},$);\n#{curve} = CIRCLE('',#{support},{radius:.16E});\n"))?;
            }
        }
        piece.put(format_args!("#{definition} = DEFINITIONAL_REPRESENTATION('',(#{curve}),#21);\n#{pc} = PCURVE('',#{surface},#{definition});\n"))?;
        Ok(pc)
    }
}

fn include_depth(depth: &mut usize, original: usize) { *depth = (*depth).max(original); }

fn placed(frame: Frame3, height: f64) -> Frame3 { Frame3 { origin: frame.origin + frame.z * height, ..frame } }

fn ring<const N:usize>(body: &Body, face: FaceId, depth: &mut usize) -> Result<[CoedgeId;N],ValueError> {
    let record = body.faces.get(face).ok_or_else(missing)?;
    let id = record.outer.filter(|_| record.inners.is_empty()).ok_or_else(cylinder_required)?;
    include_depth(depth,body.loops.next_remove_depth_demand(id)?);
    let first = body.loops.get(id).ok_or_else(missing)?.first;
    let mut next = first;
    let mut members = [first;N];
    for (index, member) in members.iter_mut().enumerate() {
        if index > 0 && next == first { return Err(cylinder_required()); }
        include_depth(depth,body.coedges.next_remove_depth_demand(next)?);
        let coedge = body.coedges.get(next).ok_or_else(missing)?;
        if coedge.loop_id != id { return Err(cylinder_required()); }
        *member = next;
        next = coedge.next;
    }
    if next != first { return Err(cylinder_required()); }
    Ok(members)
}

fn parameter_use(body: &Body, id: CoedgeId, face: FaceId, depth: &mut usize) -> Result<(WallUse,(f64,f64)),ValueError> {
    let coedge = body.coedges.get(id).ok_or_else(missing)?;
    let planar = coedge.pcurve.ok_or_else(cylinder_required)?;
    include_depth(depth,body.curves2.next_remove_depth_demand(planar)?);
    include_depth(depth,body.edges.next_remove_depth_demand(coedge.edge)?);
    let curve = match body.curves2.get(planar).ok_or_else(missing)? {
        Curve2::Line { origin,dir } => ParametricCurve::Line { at:[origin.x,origin.y],direction:[dir.x,dir.y] },
        Curve2::Circle { center,radius } => ParametricCurve::Circle { at:[center.x,center.y],radius:*radius },
        _ => return Err(cylinder_required()),
    };
    Ok((WallUse { edge:coedge.edge,forward:coedge.forward,parameter:ParameterUse { coedge:id,face,curve } },coedge.prange))
}

fn cylinder_required() -> ValueError { unsupported("STEP export requires the original exact full cylinder topology") }
