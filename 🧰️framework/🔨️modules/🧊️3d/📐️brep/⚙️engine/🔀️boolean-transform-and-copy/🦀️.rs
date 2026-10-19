//! 🔀️ Boolean, transform and copy: the synchronous fuse, cut, boolean and boolean job steps, the intersection, the transforms and the shape copy. See [`super`].

use super::*;

// #region 🔖️SyncApi
impl Brep {
    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Unite, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Cut, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }

    /// 🔀️ The one-shot set operation, over the SAME road [`Brep::boolean_job_sync`] takes — so a
    /// caller that cannot afford a job and a caller that drives one cannot diverge.
    pub fn boolean_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, op: BooleanOp) -> Result<GeometryHandle, BrepError> {
        match self.boolean_job_sync(a, b, op)? {
            BrepBooleanAdmission::Answered(handle) => Ok(handle),
            BrepBooleanAdmission::Job(mut job)=>{let solid=job.job.run_to_completion(&mut self.body,&mut job.rec).map_err(|error|map_err(&error))?;Ok(self.register_solid(solid))},
        }
    }

    /// ⏱️ A resumable, budgetable boolean of `a` and `b` — the interactive twin of
    /// [`Brep::cut_sync`]/[`Brep::fuse_sync`]/[`Brep::intersect_sync`], and the exact analogue of
    /// [`Brep::tessellate_job_sync`]. The microsecond-cheap fast paths are taken here and answer
    /// immediately; everything else comes back as a job the caller drives with
    /// [`Brep::step_boolean_job_sync`] so no single call outruns an interactive step ceiling
    /// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
    pub fn boolean_job_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, op: BooleanOp) -> Result<BrepBooleanAdmission, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        match boolean_job(&mut self.body, sa, sb, op, 1e-6, &mut rec).map_err(|error| map_err(&error))? {
            BooleanAdmission::Answered(solid) => Ok(BrepBooleanAdmission::Answered(self.register_solid(solid))),
            BooleanAdmission::Job(job) => Ok(BrepBooleanAdmission::Job(BrepBooleanJob { job, rec })),
        }
    }

    /// ⏱️ Advances `job` by at most `budget` units against THIS kernel's topology. The job borrows
    /// nothing, so a host may retain it across turns and re-present the kernel on every step — the
    /// same contract [`Brep::tessellation_body`] states for tessellation.
    pub fn step_boolean_job_sync(&mut self,job:&mut BrepBooleanJob,cx:&mut semio_framework_job::StepContext<'_>)->Result<BrepBooleanStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        match job.job.step(&mut self.body,&mut job.rec,cx)?{
            BooleanStep::Working(progress)=>Ok(BrepBooleanStep::Working(progress)),
            BooleanStep::Cancelled(progress)=>Ok(BrepBooleanStep::Cancelled(progress)),
            BooleanStep::Done(_)=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original boolean result registration has no funded producer yet").into()),
        }
    }
    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Intersect, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    /// 🔁 Dispatches an exact affine transform to whichever geometry kind `shape` resolves to: a
    /// solid (topology deep copy, [`transform_solid`]), a face ([`transform_face`]), a wire
    /// ([`transform_wire`]), or a bare curve/surface ([`Curve3`]/[`Surface::transformed`] directly
    /// — cheap, since neither lives in `self.body`). Vertex/Edge/🐚️Shell/Compound handles are out of
    /// this ticket's scope (audit §6.2 only covers solids/faces/wires/curves/surfaces).
    fn transform_shape_sync(&mut self, shape: &GeometryHandle, map: &Affine3) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        if let Ok(id) = self.solid_id(shape) {
            let out = transform_solid(&mut self.body, id, map, &mut rec).map_err(|error| map_err(&error))?;
            return Ok(self.register_solid(out));
        }
        if let Ok(id) = self.face_id(shape) {
            let out = transform_face(&mut self.body, id, map, &mut rec).map_err(|error| map_err(&error))?;
            return Ok(self.register_face(out));
        }
        if let Ok(wire) = self.wire_ref(shape) {
            let wire = wire.clone();
            let out = transform_wire(&mut self.body, &wire, map, &mut rec);
            return Ok(self.register_wire(out));
        }
        if let Ok(curve) = self.curve_ref(shape) {
            let out = curve.transformed(map);
            return Ok(self.register_curve(out));
        }
        if let Ok(surface) = self.surface_ref(shape) {
            let out = surface.transformed(map);
            return Ok(self.register_surface(out));
        }
        Err(BrepError::InvalidInput(format!("{} is not a solid/face/wire/curve/surface", shape.as_str())))
    }
    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        require_finite_vector("translate offset", offset)?;
        self.transform_shape_sync(shape, &Affine3::translation(vec3(offset)))
    }
    /// 🔁 Rotates about the WORLD origin, not `shape`'s bounding-box center — the trait's `rotate`
    /// carries no explicit origin. Use [`Self::rotate_about_sync`] for rotation about an arbitrary
    /// point.
    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        require_direction("rotate axis", axis)?;
        require_finite_scalar("rotate angle", angle)?;
        self.transform_shape_sync(shape, &Affine3::rotation_axis_angle(vec3(axis), angle))
    }
    pub fn rotate_about_sync(&mut self, shape: &GeometryHandle, origin: EVec3, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        require_finite_vector("rotate origin", origin)?;
        require_direction("rotate axis", axis)?;
        require_finite_scalar("rotate angle", angle)?;
        self.transform_shape_sync(shape, &Affine3::rotation_about(pnt(origin), vec3(axis), angle))
    }
    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        require_scale_factor("scale factor", factor)?;
        require_finite_vector("scale center", center)?;
        self.transform_shape_sync(shape, &Affine3::scaling(pnt(center), NativeVec3::new(factor, factor, factor)))
    }
    /// 📐️ Applies a finite, invertible axis scale through the kernel affine dispatcher.
    pub fn scale_axes_sync(&mut self, shape: &GeometryHandle, factors: EVec3, center: EVec3) -> Result<GeometryHandle, BrepError> {
        if factors.iter().any(|value| !value.is_finite() || *value == 0.0) || center.iter().any(|value| !value.is_finite()) { return Err(BrepError::InvalidInput("Scale factors must be finite and nonzero; the center must be finite".into())); }
        self.transform_shape_sync(shape, &Affine3::scaling(pnt(center), vec3(factors)))
    }
    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        require_finite_vector("mirror origin", origin)?;
        require_direction("mirror normal", normal)?;
        self.transform_shape_sync(shape, &Affine3::mirror(pnt(origin), vec3(normal)))
    }

    /// 🔁 A solid copies via [`copy_solid`] (its dedicated identity-transform entry point); every
    /// other supported kind copies via [`Self::transform_shape_sync`] under [`Affine3::IDENTITY`].
    pub fn copy_shape_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        if let Ok(id) = self.solid_id(shape) {
            let mut rec = OpRecorder::new();
            let out = copy_solid(&mut self.body, id, &mut rec).map_err(|error| map_err(&error))?;
            return Ok(self.register_solid(out));
        }
        self.transform_shape_sync(shape, &Affine3::IDENTITY)
    }
    /// 🔒️ Resolves unique selected edges only within the chosen solid's topology.
    pub(super) fn scoped_solid_edges(&self, solid: SolidId, edges: &[GeometryHandle]) -> Result<Vec<EdgeId>, BrepError> {
        if edges.is_empty() { return Err(BrepError::InvalidInput("Select at least one edge".into())); }
        let scope = all_edges(&self.body, solid);
        let mut selected = std::collections::BTreeSet::new();
        for handle in edges {
            let edge = self.edge_id(handle)?;
            if scope.binary_search(&edge).is_err() { return Err(BrepError::InvalidInput("Edge is not part of the selected solid".into())); }
            selected.insert(edge);
        }
        Ok(selected.into_iter().collect())
    }
}
// #endregion 🔖️SyncApi
