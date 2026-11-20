//! 📤️ Shares original certified tessellation and bounded mesh-text writing for exchange.

use super::*;
use semio_framework_3d_brep_engine::{MeshTextExport, MeshTextFormat, PrimitivePreviewJob};
use semio_framework_value::RetirementDemand;
use semio_framework_value::retained_clone::retained_clone_grant_admits;

type Progress = RetainedCloneProgress;

#[derive(semio_framework_value::RetireOwned)]
enum Preview { Primitive(PrimitivePreviewJob), Planar(TessellationJob) }

/// 🧾️ Holds an original solid preview and the bounded text writer under one retained owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct NativeMeshExport {
    kernel: usize,
    source: usize,
    revision: u64,
    format: MeshTextFormat,
    preview: Preview,
    mesh: Option<semio_framework::MeshData>,
    writer: Option<MeshTextExport>,
}

impl NativeMeshExport {
    pub fn channel(&self) -> &'static str { self.format.channel() }
    pub fn phase(&self) -> &'static str {
        if self.writer.is_some() { "mesh-export-encoding" } else { "mesh-export-tessellation" }
    }
    pub fn ready(&self) -> bool { self.writer.as_ref().is_some_and(MeshTextExport::ready) }
    pub fn take_output(&mut self) -> Option<String> { self.writer.as_mut()?.take_output() }

    fn check(&self, kernel: &Brep, handle: &str) -> Result<(), ValueError> {
        if self.kernel != std::ptr::from_ref(kernel) as usize || self.source != handle.as_ptr() as usize || handle.len() != 64
            || self.revision != kernel.preview_validation_revision() || !kernel.original_validation_is_certified(handle)? {
            return Err(ValueError::invariant("original mesh export source changed"));
        }
        Ok(())
    }

    fn demand(&self, kernel: &Brep, handle: &str) -> Result<RetirementDemand, ValueError> {
        self.check(kernel, handle)?;
        let mut quote = if let Some(writer) = &self.writer { writer.demand() }
        else if self.mesh.is_some() { RetirementDemand { depth: 1, ..Default::default() } }
        else { match &self.preview {
            Preview::Primitive(p) => kernel.primitive_preview_demand(handle, p),
            Preview::Planar(p) => kernel.analytic_preview_demand(handle, p)?
                .ok_or_else(|| ValueError::invariant("original mesh export lost its planar tessellation")),
        }? };
        quote.depth = quote.depth.max(kernel.certified_solid_depth(handle)?);
        Ok(quote)
    }

    fn step(&mut self, kernel: &Brep, handle: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let quote = self.demand(kernel, handle)?;
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, quote) { return Ok(Progress::default()); }
        if let Some(writer) = &mut self.writer { return writer.step(effect); }
        if self.mesh.is_some() {
            if !effect.performed(1)? { return Ok(Progress::default()); }
            let receipt = Progress { copied_items: 1, ..Default::default() };
            self.writer = Some(MeshTextExport::new(&mut self.mesh, self.format).map_err(|error| error.with_retained_progress(receipt))?);
            return Ok(receipt);
        }
        let (mesh, receipt) = match &mut self.preview {
            Preview::Primitive(p) => kernel.step_primitive_preview(handle, p, effect)?,
            Preview::Planar(p) => {
                if !effect.performed(1)? { return Ok(Progress::default()); }
                kernel.step_analytic_tessellation(handle, p, grant)?
            }
        };
        if let Some(mesh) = mesh { self.mesh = Some(mesh); }
        Ok(receipt)
    }
}

impl Session {
    /// 🎟️ Admits export of one live analytic primitive or certified planar solid.
    pub fn plan_mesh_export(&self, handle: &str, tolerance: f64, format: MeshTextFormat) -> Result<NativeMeshExport, ValueError> {
        if !tolerance.is_finite() || tolerance <= 0.0 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "mesh export requires positive finite deflection"));
        }
        self.with_original_geometry_query(|kernel| {
            let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained mesh export requires a certified analytic or planar solid");
            if !kernel.original_validation_is_certified(handle)? { return Err(unsupported()); }
            let preview = if let Some(job) = kernel.admit_primitive_preview(handle, tolerance)? {
                Preview::Primitive(job)
            } else {
                let (job, needs_validation) = kernel.admit_tessellation_job(handle, tolerance)?;
                if needs_validation || !job.is_planar_preview() { return Err(unsupported()); }
                Preview::Planar(job)
            };
            Ok(NativeMeshExport { kernel: std::ptr::from_ref(kernel) as usize, source: handle.as_ptr() as usize, revision: kernel.preview_validation_revision(),
                format, preview, mesh: None, writer: None })
        })
    }

    pub fn mesh_export_demand(&self, handle: &str, export: &NativeMeshExport) -> Result<RetirementDemand, ValueError> {
        self.with_original_geometry_query(|kernel| export.demand(kernel, handle))
    }

    pub fn step_mesh_export(&self, handle: &str, export: &mut NativeMeshExport, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.with_original_geometry_query(|kernel| export.step(kernel, handle, effect))
    }
}
