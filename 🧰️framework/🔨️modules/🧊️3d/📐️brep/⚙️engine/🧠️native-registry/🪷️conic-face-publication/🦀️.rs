//! 🪷️ Original topology and handle publication for one closed circular or elliptical boundary.

use super::*;
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_representation::topology::{Coedge, Edge, Face, Loop, Vertex, dummy_coedge};
use semio_framework_3d_brep_vector::{Pnt2, Vec2};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use solid_publication::{remove_topology_slot, store_demand, store_insert, topology_slot_depth};

const RANGE: (f64, f64) = (0.0, std::f64::consts::TAU);
const ENTRIES: usize = 8;
const READY: usize = 11;

#[derive(Clone, Copy)]
enum Conic {
    Circle(f64),
    Ellipse(f64, f64),
}

impl Conic {
    fn curve(self, frame: Frame3) -> Curve3 {
        match self {
            Self::Circle(radius) => Curve3::Circle { frame, radius },
            Self::Ellipse(major_radius, minor_radius) => Curve3::Ellipse { frame, major_radius, minor_radius },
        }
    }

    fn trim(self) -> Curve2 {
        let center = Pnt2::new(0.0, 0.0);
        match self {
            Self::Circle(radius) => Curve2::Circle { center, radius },
            Self::Ellipse(major_radius, minor_radius) => Curve2::Ellipse { center, x_axis: Vec2::new(1.0, 0.0), major_radius, minor_radius },
        }
    }

    fn radii(self) -> [f64; 2] {
        match self {
            Self::Circle(radius) => [radius; 2],
            Self::Ellipse(major, minor) => [major, minor],
        }
    }
}

/// 🎟️ Owns the unpublished topology slots of an exact conic face.
pub struct ConicFacePublication {
    owner: usize,
    frame: Frame3,
    conic: Conic,
    flipped: bool,
    first_label: u64,
    handle: [u8; 64],
    inserted: [Option<EntityRef>; ENTRIES],
    count: usize,
    stage: usize,
    open: bool,
}

macro_rules! entity {
    ($owner:expr, $index:literal, $kind:ident) => {
        match $owner.inserted[$index] {
            Some(EntityRef::$kind(id)) => id,
            _ => unreachable!("conic topology references an admitted arena record"),
        }
    };
}

impl ConicFacePublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("the digest uses lowercase hexadecimal") }
    pub fn topology_ready(&self) -> bool { self.stage == READY }
    pub fn terminal_is_empty(&self) -> bool { !self.open }

    fn check_owner(&self, kernel: &BrepRegistry) -> Result<(), ValueError> {
        if self.open && self.owner == std::ptr::from_ref(kernel) as usize { Ok(()) }
        else { Err(ValueError::invariant("conic face requires its original unpublished session")) }
    }

    fn label(&self, offset: u64) -> PersistentLabel { PersistentLabel(self.first_label + offset) }

    fn topology_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let entity = match self.stage {
            0 => return store_demand(&body.vertices),
            1 => return store_demand(&body.curves3),
            2 => return store_demand(&body.edges),
            3 => return store_demand(&body.surfaces),
            4 => return store_demand(&body.faces),
            5 => return store_demand(&body.loops),
            6 => return store_demand(&body.curves2),
            7 => return store_demand(&body.coedges),
            8 => self.inserted[7],
            9 => self.inserted[5],
            10 => self.inserted[4],
            _ => return Err(ValueError::invariant("conic topology is already complete")),
        }.ok_or_else(|| ValueError::invariant("conic topology requires its original arena slot"))?;
        Ok(RetirementDemand { depth: topology_slot_depth(body, entity)?, ..Default::default() })
    }

    fn insert(&mut self, body: &mut Body, grant: Grant) -> Result<Progress, ValueError> {
        macro_rules! put {
            ($store:ident, $kind:ident, $record:expr) => {{
                let (id, receipt) = store_insert(&mut body.$store, grant, || $record)?;
                (id.map(EntityRef::$kind), receipt)
            }};
        }
        let (entry, receipt) = match self.stage {
            0 => put!(vertices, Vertex, Vertex { position: self.conic.curve(self.frame).eval(0.0), label: self.label(0), tol: Tol::DEFAULT }),
            1 => put!(curves3, Curve3, self.conic.curve(self.frame)),
            2 => put!(edges, Edge, Edge {
                curve: entity!(self, 1, Curve3), range: RANGE, v0: entity!(self, 0, Vertex), v1: entity!(self, 0, Vertex),
                tol: Tol::DEFAULT, label: self.label(1),
            }),
            3 => put!(surfaces, Surface, Surface::Plane { frame: self.frame }),
            4 => put!(faces, Face, Face {
                surface: entity!(self, 3, Surface), outer: None, inners: Vec::new(), flipped: self.flipped, tol: Tol::DEFAULT, label: self.label(2),
            }),
            5 => put!(loops, Loop, Loop { first: dummy_coedge(), face: entity!(self, 4, Face) }),
            6 => put!(curves2, Curve2, self.conic.trim()),
            7 => put!(coedges, Coedge, Coedge {
                edge: entity!(self, 2, Edge), forward: true, pcurve: Some(entity!(self, 6, Curve2)), prange: RANGE,
                loop_id: entity!(self, 5, Loop), next: dummy_coedge(), prev: dummy_coedge(),
            }),
            _ => return Err(ValueError::invariant("conic topology insert is outside its recipe")),
        };
        if let Some(entry) = entry {
            self.inserted[self.stage] = Some(entry);
            self.stage += 1;
            self.count += 1;
        }
        Ok(receipt)
    }

    fn link(&mut self, body: &mut Body) -> Result<Progress, ValueError> {
        let coedge = entity!(self, 7, Coedge);
        let ring = entity!(self, 5, Loop);
        match self.stage {
            8 => {
                let use_ = body.coedges.get_mut(coedge).expect("original conic coedge");
                use_.next = coedge;
                use_.prev = coedge;
            }
            9 => body.loops.get_mut(ring).expect("original conic loop").first = coedge,
            10 => body.faces.get_mut(entity!(self, 4, Face)).expect("original conic face").outer = Some(ring),
            _ => return Err(ValueError::invariant("conic topology link is outside its recipe")),
        }
        self.stage += 1;
        Ok(Progress { copied_items: 1, ..Default::default() })
    }
}

impl BrepRegistry {

    /// 🌀️ Borrows a certified exact conic boundary from a live planar face.
    pub fn conic_face_boundary(&self, id:FaceId) -> Option<(&Curve3,EVec3,bool)> {
        let face=self.body.faces.get(id)?;
        if !face.inners.is_empty() { return None; }
        let Surface::Plane{frame}=self.body.surfaces.get(face.surface)? else { return None; };
        let loop_id=face.outer?;
        let ring=self.body.loops.get(loop_id)?;
        let coedge=self.body.coedges.get(ring.first)?;
        if ring.face!=id || coedge.loop_id!=loop_id || coedge.next!=ring.first || coedge.prev!=ring.first || coedge.pcurve.is_none() { return None; }
        use semio_framework_3d_brep_curve::Curve2;
        let range=(0.0,std::f64::consts::TAU);
        let edge=self.body.edges.get(coedge.edge)?;
        if !coedge.forward || edge.v0!=edge.v1 || edge.range!=range || coedge.prange!=range { return None; }
        let curve=self.body.curves3.get(edge.curve)?;
        let trim=self.body.curves2.get(coedge.pcurve?)?;
        match (curve,trim) {
            (Curve3::Circle{frame:source,radius},Curve2::Circle{center,radius:trim_radius})
                if source==frame && trim_radius==radius && center.x==0.0 && center.y==0.0 => {}
            (Curve3::Ellipse{frame:source,major_radius,minor_radius},Curve2::Ellipse{center,x_axis,major_radius:major,minor_radius:minor})
                if source==frame && major_radius==major && minor_radius==minor && center.x==0.0 && center.y==0.0 && x_axis.x==1.0 && x_axis.y==0.0 => {}
            _=>return None,
        }
        let normal=frame.x.cross(frame.y).normalized()?;
        let normal=if face.flipped {-normal} else {normal};
        Some((curve,[normal.x,normal.y,normal.z],face.flipped))
    }

    /// 🧭️ Plans the exact fill of one closed conic with optional XY-plane restriction.
    pub fn plan_conic_face(&mut self, handle: &str, xy_only: bool) -> Result<ConicFacePublication, ValueError> {
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "conic face requires a complete circular or elliptical boundary");
        let (curve, forward) = match self.original_query_entity(handle)? {
            Entity::Curve(curve, _) => (curve, true),
            Entity::Edge(edge) => (self.full_turn_curve(*edge).ok_or_else(invalid)?, true),
            Entity::Wire(wire, _) if wire.closed && wire.members.len() == 1 => {
                let (edge, forward) = wire.members[0];
                (self.full_turn_curve(edge).ok_or_else(invalid)?, forward)
            }
            _ => return Err(invalid()),
        };
        let (mut frame, conic) = match curve {
            Curve3::Circle { frame, radius } => (*frame, Conic::Circle(*radius)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => (*frame, Conic::Ellipse(*major_radius, *minor_radius)),
            _ => return Err(invalid()),
        };
        if conic.radii().into_iter().any(|radius| !radius.is_finite() || radius <= Tol::DEFAULT.value()) {
            return Err(invalid());
        }
        if xy_only && frame.z.z.abs() < 1.0 - 1e-9 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "planar face boundary must be parallel to the XY plane"));
        }
        if !forward { frame.y = -frame.y; frame.z = -frame.z; }
        self.plan_conic_patch(frame, conic, false)
    }

    /// ↔️ Translates a complete conic and its plane along the signed original face normal.
    pub fn plan_conic_face_offset(&mut self, handle: &str, distance: f64) -> Result<ConicFacePublication, ValueError> {
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Offset Face requires one circular or elliptical planar trim");
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "face offset requires a finite distance and translation");
        if !distance.is_finite() { return Err(invalid()); }
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(unsupported()); };
        let (curve, normal, flipped) = self.conic_face_boundary(*id).ok_or_else(unsupported)?;
        let (mut frame, conic) = match curve {
            Curve3::Circle { frame, radius } => (*frame, Conic::Circle(*radius)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => (*frame, Conic::Ellipse(*major_radius, *minor_radius)),
            _ => return Err(unsupported()),
        };
        frame.origin = frame.origin + vec3(normal) * distance;
        let radius = conic.radii().into_iter().fold(0.0_f64, f64::max);
        if evec(frame.origin).into_iter().any(|coordinate| !(coordinate.abs() + radius).is_finite()) {
            return Err(invalid());
        }
        self.plan_conic_patch(frame, conic, flipped)
    }

    fn plan_conic_patch(&mut self, frame: Frame3, conic: Conic, flipped: bool) -> Result<ConicFacePublication, ValueError> {
        use semio_framework_3d_brep_representation::topology::history::LabelSource;
        let first_label = self.body.labels.next();
        let next = first_label.checked_add(3).ok_or_else(|| ValueError::invariant("conic face label range exhausted"))?;
        self.body.labels = LabelSource::from_next(next);
        self.unpublished_topology += 1;
        Ok(ConicFacePublication {
            owner: std::ptr::from_ref(&*self) as usize, frame, conic, flipped, first_label,
            handle: analytic_publication::primitive_handle(b"Face:", next - 1),
            inserted: [None; ENTRIES], count: 0, stage: 0, open: true,
        })
    }

    /// 📏️ Quotes the next original arena slot or the final live index element.
    pub fn conic_face_demand(&self, plan: &ConicFacePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        plan.check_owner(self)?;
        if !plan.topology_ready() { return plan.topology_demand(&self.body); }
        let key = plan.handle();
        Ok(RetirementDemand { capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?, depth: self.live.next_insert_depth_demand(key)?, ..Default::default() })
    }

    /// 🧱️ Advances one funded conic face topology action.
    pub fn step_conic_face_topology(&mut self, plan: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        plan.check_owner(self)?;
        let grant = effect.grant();
        if !retained_clone_grant_admits(grant, plan.topology_demand(&self.body)?) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        if plan.stage < ENTRIES { plan.insert(&mut self.body, grant) } else { plan.link(&mut self.body) }
    }

    /// 🗄️ Reserves the original live index for the closed face.
    pub fn reserve_conic_face(&mut self, plan: &ConicFacePublication, grant: Grant) -> Result<Progress, ValueError> {
        self.conic_face_demand(plan, grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(plan.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    /// 🌱️ Publishes only the completed conic face, preserving the exact original profile.
    pub fn publish_conic_face(&mut self, plan: &mut ConicFacePublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if !plan.topology_ready() || key != plan.handle() || self.live.contains_key(&key) {
            return Err((ValueError::invariant("invalid conic face publication"), key));
        }
        if let Err(error) = plan.check_owner(self) { return Err((error, key)); }
        let (previous, receipt) = self.live.insert_reserved(key, Entity::Face(entity!(plan, 4, Face)), grant).map_err(|(error, key, _)| (error, key))?;
        assert!(previous.is_none());
        plan.open = false;
        self.unpublished_topology -= 1;
        self.advance_live_revision();
        Ok(receipt)
    }

    /// 🧹️ Quotes one unpublished conic face arena removal.
    pub fn conic_face_close_demand(&self, plan: &ConicFacePublication) -> Result<RetirementDemand, ValueError> {
        plan.check_owner(self)?;
        let depth = match plan.count.checked_sub(1).and_then(|index| plan.inserted[index]) {
            Some(entity) => topology_slot_depth(&self.body, entity)?,
            None => 1,
        };
        Ok(RetirementDemand { depth, ..Default::default() })
    }

    /// ♻️ Releases one original unpublished face arena slot in reverse dependency order.
    pub fn close_conic_face(&mut self, plan: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        let demand = self.conic_face_close_demand(plan)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(Step::Progress(Progress::default()));
        }
        let progress = Progress { copied_items: 1, ..Default::default() };
        if plan.count == 0 {
            plan.open = false;
            self.unpublished_topology -= 1;
            return Ok(Step::Complete(progress));
        }
        plan.count -= 1;
        remove_topology_slot(&mut self.body, plan.inserted[plan.count].take().expect("original conic face slot"));
        Ok(Step::Progress(progress))
    }
}
