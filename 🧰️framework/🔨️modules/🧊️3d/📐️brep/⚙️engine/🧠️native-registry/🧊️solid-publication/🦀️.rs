//! 🧊️ Admits funded solid body transactions and commits their original live handles.

use super::*;
use super::certified_boolean::{BoreBoundary, OrthogonalBoundary};
use semio_framework_3d_brep_representation::topology::history::LabelSource;
use semio_framework_3d_brep_solid_transaction::SolidBoundary;
pub use semio_framework_3d_brep_solid_transaction::SolidPublication;
pub(super) use semio_framework_3d_brep_solid_transaction::{store_demand, store_insert, topology_slot_depth, remove_topology_slot};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand};
use analytic_publication::primitive_handle;

impl BrepRegistry {
    /// 📏️ Admits a finite analytic primitive and an independent range of stable topology labels.
    pub fn plan_solid_primitive(&mut self, primitive: SolidPrimitive) -> Result<SolidPublication, ValueError> {
        primitive.validate().map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        self.plan_solid_boundary(SolidBoundary::Analytic(primitive))
    }

    pub(super) fn plan_bore_solid(&mut self, boundary: BoreBoundary) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Bore(boundary))
    }

    pub fn plan_orthogonal_solid(&mut self, boundary: OrthogonalBoundary) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Orthogonal(boundary))
    }

    pub(super) fn plan_prism_boundary(&mut self, boundary: prism_boundary::PrismBoundary) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Prism(boundary))
    }

    /// 🪚️ Stages a new exact prism from one certified original box member.
    pub fn plan_box_finish(&mut self, shape: &str, selected: &str, finish: BoxFinish) -> Result<SolidPublication, ValueError> {
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, finish.source_requirement());
        let Entity::Solid(solid, Some(SolidProvenance::Primitive(proof))) = self.original_query_entity(shape)? else { return Err(unsupported()); };
        let SolidPrimitive::Box { origin,width,depth,height } = &proof.primitive else { return Err(unsupported()); };
        if !self.body.solids.contains(*solid) { return Err(unsupported()); }
        let primitive = SolidPrimitive::Box { origin:*origin,width:*width,depth:*depth,height:*height };
        let (label, start, count) = match (finish,self.original_query_entity(selected)?) {
            (BoxFinish::Chamfer(_),Entity::Edge(id)) => (self.body.edges.get(*id).ok_or_else(unsupported)?.label,primitive.vertex_count(),primitive.edge_count()),
            (BoxFinish::Draft { .. },Entity::Face(id)) => (self.body.faces.get(*id).ok_or_else(unsupported)?.label,primitive.vertex_count()+primitive.edge_count(),primitive.ring_sizes().len()),
            _ => return Err(unsupported()),
        };
        let first = proof.preview_first_label.ok_or_else(unsupported)? + start as u64;
        let ordinal = label.0.checked_sub(first).filter(|&i|i < count as u64).ok_or_else(unsupported)?;
        let boundary = finish.boundary(primitive,ordinal as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue,finish.geometry_requirement()))?;
        self.plan_prism_boundary(boundary)
    }

    /// 🎟️ Quotes both original index paths and the selected native member record.
    pub fn box_finish_depth(&self, shape: &str, selected: &str) -> Result<usize, ValueError> {
        let depth = self.certified_solid_depth(shape)?.max(self.tessellation_source_depth_demand(selected)?);
        let member = match self.original_query_entity(selected)? {
            Entity::Edge(id) => self.body.edges.next_remove_depth_demand(*id)?,
            Entity::Face(id) => self.body.faces.next_remove_depth_demand(*id)?,
            _ => 0,
        };
        Ok(depth.max(member))
    }

    /// 📦️ Admits the verified closed convex boundary for funded native publication.
    pub fn finish_point_hull(&mut self, draft: &mut HullDraft) -> Result<SolidPublication, ValueError> {
        self.plan_solid_boundary(SolidBoundary::Hull(draft.finish()?))
    }

    fn plan_solid_boundary(&mut self, boundary: SolidBoundary) -> Result<SolidPublication, ValueError> {
        let owner = std::ptr::from_ref(&*self) as usize;
        let item = SolidPublication::begin(owner, boundary, &mut self.body.labels, |label| primitive_handle(b"Solid:", label))?;
        self.unpublished_topology += 1;
        Ok(item)
    }

    /// 🧭️ Plans an exact affine image when the result still fits a certified primitive recipe.
    pub fn plan_certified_solid_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<Option<SolidPublication>,ValueError> {
        let Some(primitive) = self.live_solid_primitive(source) else { return Ok(None); };
        if matches!(transform,AnalyticTransform::Copy) { return self.plan_solid_primitive(primitive).map(Some); }
        let map = transform.map()?;
        primitive.affinely_transformed(&map).map(|shape| self.plan_solid_primitive(shape)).transpose()
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

    /// 🪓️ Admits both box halves without changing the live index.
    pub fn plan_box_split(&mut self, source: &str, origin: EVec3, normal: EVec3) -> Result<[SolidPublication; 2], ValueError> {
        let solid = self.live_solid_primitive(source).ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Split requires a certified box"))?;
        let [positive, negative] = solid.split_by_axis_plane(origin, normal).map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        let owner = std::ptr::from_ref(&*self) as usize;
        let mut labels = semio_framework_3d_brep_representation::topology::history::LabelSource::from_next(self.body.labels.next());
        let mut admit = |shape| SolidPublication::begin(owner, SolidBoundary::Analytic(shape), &mut labels, |label| primitive_handle(b"Solid:", label));
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
