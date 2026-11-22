//! 🧊️ Admits funded solid body transactions and commits their original live handles.

use super::*;
use super::certified_boolean::OrthogonalBoundary;
use semio_framework_3d_brep_construction::primitives::BoxGeometry;
use semio_framework_3d_brep_solid_topology::{certify_bore_stock, published_bore_stock, spherical_axial_span, split_spherical_support};
use semio_framework_3d_brep_representation::topology::history::LabelSource;
use semio_framework_3d_brep_solid_transaction::SolidBoundary;
pub use semio_framework_3d_brep_solid_transaction::{SolidExchangeBoundary,SolidPublication};
pub(super) use semio_framework_3d_brep_solid_transaction::{store_demand, store_insert, topology_slot_depth, remove_topology_slot};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand};
use analytic_publication::primitive_handle;

#[path = "🧹️rounded-box-proof/🦀️.rs"]
mod rounded_box_proof;

impl BrepRegistry {
    /// 📏️ Admits a finite analytic primitive and an independent range of stable topology labels.
    pub fn plan_solid_primitive(&mut self, primitive: SolidPrimitive) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Analytic(primitive))
    }

    pub fn plan_orthogonal_solid(&mut self, boundary: OrthogonalBoundary) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Orthogonal(boundary))
    }

    /// 📥️ Admits an exact boundary certified by a retained exchange cursor.
    pub fn plan_exchange_boundary(&mut self, boundary: SolidExchangeBoundary) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(boundary.into())
    }

    /// 🪚️ Stages a solid finish from a certified box and any selected members.
    pub fn plan_box_finish(&mut self, shape: &str, selected: &[&str], finish: BoxFinish) -> Result<SolidPublication, ValueError> {
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, finish.source_requirement());
        let Entity::Solid(solid,Some(proof))=self.original_query_entity(shape)? else{return Err(unsupported());};
        let record=self.body.solids.get(*solid).ok_or_else(unsupported)?;
        let count = match finish { BoxFinish::ChamferAll(_) | BoxFinish::FilletAll(_) => 0..=0, BoxFinish::Fillet(_) => 1..=4, _ => 1..=1 };
        if !count.contains(&selected.len()) { return Err(unsupported()); }
        if let SolidProvenance::PlanarBoundary(Some(source))=proof {
            if !matches!(finish,BoxFinish::Fillet(_)|BoxFinish::Chamfer(_)|BoxFinish::ChamferAll(_)){return Err(unsupported());}
            let stock=source.rectangular().ok_or_else(unsupported)?;
            let edge_count=BoxGeometry::EDGES.len() as u64;
            let first=record.label.0.checked_sub(edge_count+BoxGeometry::FACES.len() as u64+1).ok_or_else(unsupported)?;
            let mut edges=[0;4];
            for (slot,handle) in edges.iter_mut().zip(selected){
                let Entity::Edge(id)=self.original_query_entity(handle)? else{return Err(unsupported());};
                let label=self.body.edges.get(*id).ok_or_else(unsupported)?.label.0;
                *slot=label.checked_sub(first).filter(|&index|index<edge_count).ok_or_else(unsupported)? as usize;
            }
            let boundary=match finish{
                BoxFinish::Fillet(radius)=>stock.fillet(&edges[..selected.len()],radius),
                BoxFinish::Chamfer(distance)=>stock.chamfer(edges[0],distance),
                BoxFinish::ChamferAll(distance)=>stock.chamfer_all(distance),
                _=>unreachable!(),
            }.ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,finish.geometry_requirement()))?;
            return self.plan_solid_boundary(boundary);
        }
        let SolidProvenance::Primitive(proof)=proof else{return Err(unsupported());};
        let SolidPrimitive::Box { origin,width,depth,height } = &proof.primitive else { return Err(unsupported()); };
        let primitive = SolidPrimitive::Box { origin:*origin,width:*width,depth:*depth,height:*height };
        let mut ordinals=[0;4];
        for (slot,handle) in ordinals.iter_mut().zip(selected) {
            let (label, start, count) = match (finish,self.original_query_entity(handle)?) {
                (BoxFinish::Chamfer(_) | BoxFinish::Fillet(_),Entity::Edge(id)) => (self.body.edges.get(*id).ok_or_else(unsupported)?.label,primitive.vertex_count(),primitive.edge_count()),
                (BoxFinish::Draft { .. },Entity::Face(id)) => (self.body.faces.get(*id).ok_or_else(unsupported)?.label,primitive.vertex_count()+primitive.edge_count(),primitive.ring_sizes().len()),
                _ => return Err(unsupported()),
            };
            let first=proof.preview_first_label.ok_or_else(unsupported)?+start as u64;
            *slot=label.0.checked_sub(first).filter(|&i|i<count as u64).ok_or_else(unsupported)? as usize;
        }
        let boundary=finish.boundary(primitive,&ordinals[..selected.len()]).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,finish.geometry_requirement()))?;
        self.plan_solid_boundary(boundary)
    }

    pub const DEFEATURE_FACE_COUNT: usize = 20;

    /// 🧹️ Certifies the selected feature and restores the six stock planes.
    pub fn defeature_stock(&self, shape: &str, selected: &[&str]) -> Result<SolidPrimitive, ValueError> {
        self.restored_box(shape, selected, false)
    }

    /// 📥️ Re-identifies the complete boundary when shape exchange discarded live provenance.
    pub fn defeature_imported_stock(&self, shape: &str, selected: &[&str]) -> Result<SolidPrimitive, ValueError> {
        self.restored_box(shape, selected, true)
    }

    fn restored_box(&self, shape: &str, selected: &[&str], check_imported: bool) -> Result<SolidPrimitive, ValueError> {
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "defeature requires every bore wall and floor or all twenty distinct fillet faces");
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "defeature requires certified rectangular stock with a complete fillet or bore feature");
        if !matches!(selected.len(), 1 | 2 | Self::DEFEATURE_FACE_COUNT) { return Err(invalid()); }
        let Entity::Solid(id, provenance) = self.original_query_entity(shape)? else { return Err(unsupported()); };
        let solid = self.body.solids.get(*id).filter(|solid| solid.inners.is_empty()).ok_or_else(unsupported)?;
        let shell = self.body.shells.get(solid.outer).ok_or_else(unsupported)?;
        if selected.len() <= 2 {
            let face = |handle: &str| match self.original_query_entity(handle)? { Entity::Face(id) => Ok(*id), _ => Err(invalid()) };
            let verify = |faces: &[_]| match provenance {
                Some(SolidProvenance::Bore(source)) => published_bore_stock(&self.body, shell, *source, faces),
                None if check_imported => certify_bore_stock(&self.body, shell, faces),
                _ => None,
            }.ok_or_else(unsupported);
            return match selected {
                [a] => verify(&[face(a)?]),
                [a, b] => verify(&[face(a)?, face(b)?]),
                _ => Err(invalid()),
            };
        }
        let source = match provenance {
            Some(SolidProvenance::RoundedBox(source, _)) => *source,
            None if check_imported => rounded_box_proof::certify(&self.body, shell).ok_or_else(unsupported)?,
            _ => return Err(unsupported()),
        };
        if shell.faces.len() != source.face_count() { return Err(unsupported()); }
        let stock_faces = source.face_count() - Self::DEFEATURE_FACE_COUNT;
        let mut seen = [false; Self::DEFEATURE_FACE_COUNT];
        for handle in selected {
            let Entity::Face(face_id) = self.original_query_entity(handle)? else { return Err(invalid()); };
            let index = shell.faces[stock_faces..].iter().position(|id| id == face_id).ok_or_else(invalid)?;
            if seen[index] { return Err(invalid()); }
            seen[index] = true;
        }
        let origin: [f64; 3] = std::array::from_fn(|axis| source.face_frame(2 * axis).origin.to_array()[axis]);
        let [width, depth, height] = std::array::from_fn(|axis| source.face_frame(2 * axis + 1).origin.to_array()[axis] - origin[axis]);
        Ok(SolidPrimitive::Box { origin: Pnt3::from_array(origin), width, depth, height })
    }

    /// 🧹️ Quotes the original solid, shell and selected face paths.
    pub fn defeature_depth(&self, shape: &str, selected: &[&str]) -> Result<usize, ValueError> {
        if !matches!(selected.len(), 1 | 2 | Self::DEFEATURE_FACE_COUNT) { return Err(ValueError::invariant("defeature requires a complete bore feature or twenty fillet faces")); }
        let mut depth = self.selected_member_depth(shape, selected)?;
        if let Entity::Solid(id, _) = self.original_query_entity(shape)? {
            if let Some(solid) = self.body.solids.get(*id) {
                depth = depth.max(self.body.shells.next_remove_depth_demand(solid.outer)?);
            }
        }
        Ok(depth)
    }

    /// 🎟️ Quotes both original index paths and the selected native member record.
    pub fn box_finish_depth(&self, shape: &str, selected: &[&str]) -> Result<usize, ValueError> {
        if selected.len() > 4 { return Err(ValueError::invariant("box finish member count exceeds the fixed certificate")); }
        self.selected_member_depth(shape, selected)
    }

    fn selected_member_depth(&self, shape: &str, selected: &[&str]) -> Result<usize, ValueError> {
        let mut depth = self.certified_solid_depth(shape)?;
        for handle in selected{
            let member=match self.original_query_entity(handle)? {
                Entity::Edge(id)=>self.body.edges.next_remove_depth_demand(*id)?,
                Entity::Face(id)=>self.body.faces.next_remove_depth_demand(*id)?,
                _=>0,
            };
            depth=depth.max(self.tessellation_source_depth_demand(handle)?).max(member);
        }
        Ok(depth)
    }

    /// 📦️ Admits the verified closed convex boundary for funded native publication.
    pub fn finish_point_hull(&mut self, draft: &mut HullDraft) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Hull(draft.finish()?))
    }

    pub(super) fn plan_solid_boundary(&mut self, boundary: SolidBoundary) -> Result<SolidPublication, ValueError> {
        if let SolidBoundary::Analytic(primitive) = &boundary {
        primitive.validate().map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        }
        let owner = std::ptr::from_ref(&*self) as usize;
        let item = SolidPublication::begin(owner, boundary, &mut self.body.labels, |label| primitive_handle(b"Solid:", label))?;
        self.unpublished_topology += 1;
        Ok(item)
    }

    /// 〰️ Opens an independent spline representation of a live certified cuboid.
    pub fn plan_nurbs_cuboid(&mut self, source: &str) -> Result<SolidPublication, ValueError> {
        let primitive = self.live_solid_primitive(source).ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "NURBS conversion requires a certified cuboid"))?;
        self.plan_primitive_encoding(primitive, true)
    }

    /// 📥️ Plans a spline cuboid certified by an original exchange cursor.
    pub fn plan_nurbs_cuboid_primitive(&mut self, primitive: SolidPrimitive) -> Result<SolidPublication, ValueError> { self.plan_primitive_encoding(primitive, true) }
    fn plan_primitive_encoding(&mut self, primitive: SolidPrimitive, nurbs: bool) -> Result<SolidPublication, ValueError> {
        if !nurbs { return self.plan_solid_primitive(primitive); }
        let owner = std::ptr::from_ref(&*self) as usize;
        let plan = SolidPublication::begin_nurbs_cuboid(owner, primitive, &mut self.body.labels, |label| primitive_handle(b"Solid:", label))?;
        self.unpublished_topology += 1;
        Ok(plan)
    }

    /// 🧭️ Plans an exact affine image when the result still fits a certified primitive recipe.
    pub fn plan_certified_solid_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<Option<SolidPublication>,ValueError> {
        let Some(Entity::Solid(id, Some(proof))) = self.live.get(source) else { return Ok(None); };
        if !self.body.solids.contains(*id) { return Ok(None); }
        let Some((shape,nurbs))=proof.affine_image(&transform.map()?,matches!(transform,AnalyticTransform::Copy)) else { return Ok(None); };
        if let (SolidBoundary::Analytic(primitive),true)=(&shape,nurbs){return self.plan_primitive_encoding(*primitive,true).map(Some);}
        self.plan_solid_boundary(shape).map(Some)
    }

    /// ⤴️ Plans the exact axial extrusion of one live complete circular curve, wire, or face.
    pub fn plan_circular_extrusion(&mut self, handle: &str, extrusion: EVec3) -> Result<Option<SolidPublication>, ValueError> {
        let curve = match self.original_query_entity(handle)? {
            Entity::Curve(curve, _) => Some(curve),
            Entity::Edge(id) => self.full_turn_curve(*id),
            Entity::Wire(wire, _) if wire.closed && wire.members.len() == 1 => self.full_turn_curve(wire.members[0].0),
            Entity::Face(face) => self.conic_face_boundary(*face).map(|(curve, _, _)| curve),
            _ => None,
        };
        let Some(Curve3::Circle { frame, radius }) = curve else { return Ok(None); };
        let mut frame = *frame;
        let radius = *radius;
        let length = extrusion[0].hypot(extrusion[1]).hypot(extrusion[2]);
        let invalid_vector = || ValueError::literal(ValueRefusalKind::InvalidValue, "circular extrusion requires a finite nonzero vector parallel to the profile normal");
        if !length.is_finite() || length <= Tol::DEFAULT.value() { return Err(invalid_vector()); }
        let direction = vec3(extrusion.map(|component| component / length));
        if frame.z.cross(direction).norm() > 1e-9 { return Err(invalid_vector()); }
        let height = vec3(extrusion).dot(frame.z);
        if height < 0.0 { frame.y = -frame.y; frame.z = -frame.z; }
        self.plan_solid_primitive(SolidPrimitive::Cylinder { frame, radius, height: height.abs() }).map(Some)
    }

    pub fn full_turn_curve(&self, id: EdgeId) -> Option<&Curve3> {
        let edge = self.body.edges.get(id)?;
        let turn = std::f64::consts::TAU;
        let sweep = (edge.range.1 - edge.range.0).abs();
        if !sweep.is_finite() || (sweep - turn).abs() > 32.0 * turn * f64::EPSILON { return None; }
        self.body.curves3.get(edge.curve)
    }

    /// 🎟️ Quotes one native topology turn or the final live-index insertion.
    pub fn solid_primitive_demand(&self, item: &SolidPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        item.check_owner(std::ptr::from_ref(&*self) as usize)?;
        if !item.topology_ready() { return item.topology_demand(&self.body); }
        let key = item.handle();
        Ok(RetirementDemand {
            capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?,
            depth: self.live.next_insert_depth_demand(key)?,
            ..Default::default()
        })
    }

    /// 🧱️ Advances one charged arena reservation or topology action.
    pub fn step_solid_primitive_topology(&mut self, item: &mut SolidPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        item.check_owner(std::ptr::from_ref(&*self) as usize)?;
        item.advance(&mut self.body, effect)
    }

    /// 🗄️ Reserves only the next original backing allocation for the completed solid.
    pub fn reserve_solid_primitive(&mut self, item: &SolidPublication, grant: Grant) -> Result<Progress, ValueError> {
        self.solid_primitive_demand(item, grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(item.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    /// 🫴️ Transfers the completed solid into its admitted live slot.
    pub fn publish_solid_primitive(&mut self, item: &mut SolidPublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if !item.topology_ready() || key != item.handle() || self.live.contains_key(&key) {
            return Err((ValueError::invariant("invalid solid primitive publication"), key));
        }
        let (id, provenance) = match item.product(std::ptr::from_ref(&*self) as usize) {
            Ok(product) => product,
            Err(error) => return Err((error, key)),
        };
        let (previous, receipt) = self.live.insert_reserved(key, Entity::Solid(id, provenance), grant)
            .map_err(|(error, key, _)| (error, key))?;
        assert!(previous.is_none());
        item.mark_published();
        self.unpublished_topology -= 1;
        self.advance_live_revision();
        Ok(receipt)
    }

    /// 🪓️ Admits both analytic halves without changing the live index.
    pub fn plan_plane_split(&mut self, source: &str, origin: EVec3, normal: EVec3) -> Result<[SolidPublication; 2], ValueError> {
        let (_,solid)=self.preview_provenance(source).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Split requires a certified analytic or box-derived solid"))?;
        let [positive,negative]=match solid {
            SolidProvenance::Primitive(PrimitiveProvenance{primitive:SolidPrimitive::Sphere{frame,radius},..})=>
                split_spherical_support(*frame,*radius,None,origin,normal)?.map(SolidBoundary::SphericalCaps),
            SolidProvenance::SphericalCaps(shape,_) if shape.is_disk(1)=>
                split_spherical_support(shape.frame(0),shape.radius(0),spherical_axial_span(*shape),origin,normal)?.map(SolidBoundary::SphericalCaps),
            _=>solid.split_by_plane(origin,normal)?,
        };
        let owner = std::ptr::from_ref(&*self) as usize;
        let mut labels = semio_framework_3d_brep_representation::topology::history::LabelSource::from_next(self.body.labels.next());
        let mut admit = |shape| SolidPublication::begin(owner, shape, &mut labels, |label| primitive_handle(b"Solid:", label));
        let pair = [admit(positive)?, admit(negative)?];
        self.body.labels = labels;
        self.unpublished_topology += 2;
        Ok(pair)
    }

    /// 🤝️ Places both complete primitive roots in one admitted live-index step.
    pub fn publish_solid_pair(&mut self, pair: &mut [SolidPublication; 2], keys: &mut [String; 2], grant: Grant) -> Result<Progress, ValueError> {
        if !pair.iter().zip(keys.iter()).all(|(solid, key)| solid.is_open() && solid.handle() == key.as_str()) { return Err(ValueError::invariant("split publication keys changed")); }
        let product = |solid: &SolidPublication| solid.product(std::ptr::from_ref(&*self) as usize).map(|(id, provenance)| Entity::Solid(id, provenance));
        let [positive, negative] = [product(&pair[0])?, product(&pair[1])?];
        let [positive_key, negative_key] = std::mem::take(keys);
        let receipt = self.live.insert_vacant_pair_reserved([(positive_key, positive), (negative_key, negative)], grant)
            .map_err(|(error, [(positive_key, _), (negative_key, _)])| { *keys = [positive_key, negative_key]; error })?;
        pair.iter_mut().for_each(SolidPublication::mark_published);
        self.unpublished_topology -= 2;
        self.advance_live_revision();
        Ok(receipt)
    }

    /// 🧹️ Quotes one unpublished body retirement or a published journal release.
    pub fn solid_primitive_close_demand(&self, item: &SolidPublication) -> Result<RetirementDemand, ValueError> {
        if !item.owns(std::ptr::from_ref(self) as usize) { return Err(ValueError::invariant("original solid retirement source changed")); }
        item.close_demand(&self.body)
    }

    /// ♻️ Rolls back an abandoned body transaction and returns its native topology claim.
    pub fn close_solid_primitive(&mut self, item: &mut SolidPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        if !item.owns(std::ptr::from_ref(&*self) as usize) { return Err(ValueError::invariant("original solid retirement source changed")); }
        let was_open = item.is_open();
        let result = item.close(&mut self.body, effect)?;
        if was_open && !item.is_open() { self.unpublished_topology -= 1; }
        Ok(result)
    }
}
