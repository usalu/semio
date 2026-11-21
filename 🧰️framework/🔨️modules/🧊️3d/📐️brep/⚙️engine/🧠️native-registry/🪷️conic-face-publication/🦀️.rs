//! 🪷️ Certifies live conic sources and publishes their funded original topology.

use super::*;
use polygon_face_publication::FaceOpening;
use semio_framework_3d_brep_solid_topology::{PolygonOpening,complete_conic_boundary,spherical_axial_span};
use semio_framework_3d_brep_solid_transaction::{Conic,ConicBoundary};
pub use semio_framework_3d_brep_solid_transaction::ConicFacePublication;
pub use polygon_face_publication::{PolygonSectionSet, SECTION_FACE_LIMIT};
use semio_framework_3d_brep_queries::planar_section::{PlanarSection, PlanarSectionRegion, planar_section, spherical_strip_section};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,ValueError,ValueRefusalKind};

/// ✂️ Selects a staged polygon or a complete native conic section.
pub enum SectionPlan {
    Box(semio_framework_3d_brep_construction::primitives::BoxSectionDraft),
    Conic(ConicFacePublication),
    Polygon(PolygonFacePublication, semio_framework_3d_brep_construction::primitives::SectionPolygon),
    Regions(PolygonSectionSet),
}

impl BrepRegistry {

    /// 🌐️ Resolves an exact supported solid section without copying body topology.
    pub fn plan_section(&mut self, handle:&str, origin:EVec3, normal:EVec3)->Result<SectionPlan,ValueError>{
        use semio_framework_3d_brep_construction::primitives::SectionPolygon;
        use semio_framework_3d_brep_solid_transaction::{apex_section,axial_cylinder_section,conic_section,frustum_section};
        let unsupported=||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Section requires a certified solid");
        let Some(Entity::Solid(id,source))=self.live.get(handle)else{return Err(unsupported());};
        if !self.body.solids.contains(*id){return Err(ValueError::invariant("section solid is no longer live"));}
        if let Some(draft)=source.as_ref().map(|source|source.box_section(origin,normal)).transpose()?.flatten() {return Ok(SectionPlan::Box(draft));}
        if matches!(source, None | Some(SolidProvenance::PlanarBoundary(None))) {
            if let Some(section)=planar_section(&self.body,*id,origin,normal,Tol::DEFAULT.value()) {
                return match section.map_err(|message|ValueError::literal(ValueRefusalKind::InvalidValue,message))? {
                    PlanarSection::Convex(draft)=>Ok(SectionPlan::Box(draft)),
                    PlanarSection::Polygon(polygon)=>Ok(SectionPlan::Polygon(self.plan_polygon_face_points(polygon.count())?,polygon)),
                    PlanarSection::Region(region) => {
                        let (plan, polygon) = self.plan_section_region(region)?;
                        Ok(SectionPlan::Polygon(plan, polygon))
                    }
                    PlanarSection::Regions(regions) => {
                        let mut faces = PolygonSectionSet::default();
                        for region in regions.into_regions() {
                            faces.push(self.plan_section_region(region)?);
                        }
                        Ok(SectionPlan::Regions(faces))
                    }
                };
            }
        }
        let source=source.as_ref().ok_or_else(unsupported)?;
        if let SolidProvenance::SphericalCaps(shape,_)=source {
            let span=spherical_axial_span(*shape).ok_or_else(unsupported)?;
            let (frame,radius,arcs)=spherical_strip_section(
                shape.frame(0),shape.radius(0),span,origin,normal,Tol::DEFAULT.value(),
            ).map_err(|message|ValueError::literal(ValueRefusalKind::InvalidValue,message))?;
            let boundary=arcs[0].map_or(ConicBoundary::FULL,|first|ConicBoundary::clipped(first,arcs[1]));
            return self.plan_conic_boundary(frame,Conic::Circle(radius),boundary,false).map(SectionPlan::Conic);
        }
        let profile=match source{
            SolidProvenance::Frustum(shape,_)=>Some((shape.frame,shape.radii,shape.height,Some(shape.apex()))),
            SolidProvenance::Primitive(shape)=>match shape.primitive{
                SolidPrimitive::Cone{frame,radius,height}=>Some((frame,[radius,0.],height,None)),_=>None,
            },
            _=>None,
        };
        if let Some((frame,radii,height,apex))=profile{
            if let Some(polygon)=apex_section(frame,radii,height,apex,pnt(origin),vec3(normal))?{
                return Ok(SectionPlan::Polygon(self.plan_polygon_face_points(polygon.count())?,polygon));
            }
            let (frame,conic,boundary)=frustum_section(frame,radii,height,pnt(origin),vec3(normal))?;
            return self.plan_conic_boundary(frame,conic,boundary,false).map(SectionPlan::Conic);
        }
        let SolidProvenance::Primitive(source)=source else{return Err(unsupported());};
        if let Some(points)=axial_cylinder_section(&source.primitive,pnt(origin),vec3(normal))? {
            let polygon=SectionPolygon::ordered_quadrilateral(points);
            return Ok(SectionPlan::Polygon(self.plan_polygon_face_points(polygon.count())?,polygon));
        }
        let (frame,conic,boundary)=conic_section(&source.primitive,pnt(origin),vec3(normal))?;
        self.plan_conic_boundary(frame,conic,boundary,false).map(SectionPlan::Conic)
    }

    fn plan_section_region(&mut self, region: PlanarSectionRegion) -> Result<(PolygonFacePublication, semio_framework_3d_brep_construction::primitives::SectionPolygon), ValueError> {
        let (polygon, holes) = region.into_rings();
        let mut plan = self.plan_polygon_face_points(polygon.count())?;
        if holes.iter().any(Option::is_some) {
            plan.opening = Some(FaceOpening::Polygon(PolygonOpening::new(holes)));
        }
        Ok((plan, polygon))
    }

    /// 🌀️ Borrows a certified exact conic boundary from a live planar face.
    pub fn conic_face_boundary(&self, id:FaceId) -> Option<(&Curve3,EVec3,bool)> {
        complete_conic_boundary(&self.body,id)
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
        if conic.radii().expect("analytic profile").into_iter().any(|radius| !radius.is_finite() || radius <= Tol::DEFAULT.value()) {
            return Err(invalid());
        }
        if xy_only && frame.z.z.abs() < 1.0 - 1e-9 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "planar face boundary must be parallel to the XY plane"));
        }
        if !forward { frame.y = -frame.y; frame.z = -frame.z; }
        self.plan_conic_boundary(frame,conic,ConicBoundary::FULL,false)
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
        let radius = conic.radii().expect("analytic profile").into_iter().fold(0.0_f64, f64::max);
        if evec(frame.origin).into_iter().any(|coordinate| !(coordinate.abs() + radius).is_finite()) {
            return Err(invalid());
        }
        self.plan_conic_boundary(frame,conic,ConicBoundary::FULL,flipped)
    }

    fn plan_conic_boundary(&mut self,frame:Frame3,conic:Conic,boundary:ConicBoundary,flipped:bool)->Result<ConicFacePublication,ValueError>{
        use semio_framework_3d_brep_representation::topology::history::LabelSource;
        let first_label = self.body.labels.next();
        let next = first_label.checked_add(2*boundary.members() as u64+1).ok_or_else(|| ValueError::invariant("conic face label range exhausted"))?;
        self.body.labels = LabelSource::from_next(next);
        self.unpublished_topology += 1;
        Ok(ConicFacePublication::new(std::ptr::from_ref(&*self) as usize,frame,conic,boundary,flipped,first_label,
            analytic_publication::primitive_handle(b"Face:",next-1)))
    }

    /// 📏️ Quotes the next original arena slot or the final live index element.
    pub fn conic_face_demand(&self, plan: &ConicFacePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        plan.check_owner(std::ptr::from_ref(&*self) as usize)?;
        if !plan.topology_ready() { return plan.topology_demand(&self.body); }
        let key = plan.handle();
        Ok(RetirementDemand { capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?, depth: self.live.next_insert_depth_demand(key)?, ..Default::default() })
    }

    /// 🧱️ Advances one funded conic face topology action.
    pub fn step_conic_face_topology(&mut self, plan: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        plan.check_owner(std::ptr::from_ref(&*self) as usize)?;
        plan.step(&mut self.body,effect)
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
        if let Err(error) = plan.check_owner(std::ptr::from_ref(&*self) as usize) { return Err((error, key)); }
        let (previous, receipt) = self.live.insert_reserved(key, Entity::Face(plan.face_id()), grant).map_err(|(error, key, _)| (error, key))?;
        assert!(previous.is_none());
        plan.finish();
        self.unpublished_topology -= 1;
        self.advance_live_revision();
        Ok(receipt)
    }

    /// 🧹️ Quotes one unpublished conic face arena removal.
    pub fn conic_face_close_demand(&self, plan: &ConicFacePublication) -> Result<RetirementDemand, ValueError> {
        plan.check_owner(std::ptr::from_ref(&*self) as usize)?;
        plan.close_demand(&self.body)
    }

    /// ♻️ Releases one original unpublished face arena slot in reverse dependency order.
    pub fn close_conic_face(&mut self, plan: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        plan.check_owner(std::ptr::from_ref(&*self) as usize)?;
        let step=plan.close(&mut self.body,effect)?;
        if plan.terminal_is_empty(){self.unpublished_topology-=1;}
        Ok(step)
    }
}
