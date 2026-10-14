//! 📤️ Original pinned native geometry and label bytes lend into bounded caller-owned ABI buffers.

use super::*;
use ui_scene::math::{Mesh3dLease, Mesh3dSchema, MESH3D_PAGE_BYTES};

pub const UI_NATIVE_MESH_CHUNK_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiNativeMeshDescriptor {
    pub identity: Mesh3dResourceIdentity,
    pub schema: Mesh3dSchema,
    pub geometry_bytes: usize,
    pub component_counts: [usize; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiNativeMeshChannel {
    Geometry,
    ComponentLabel { domain: usize, group: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiNativeMeshChunk {
    pub bytes: usize,
    pub total_bytes: usize,
    pub complete: bool,
}

impl UiNativeResourceLoans {
    fn original_transport_lease(&self, resource: usize, cx: &semio_framework_job::StepContext<'_>) -> Result<(Mesh3dLease, Mesh3dResourceIdentity), UiNativeResourceFailure> {
        if !self.capture_complete() { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Incomplete)); }
        let original = self.row(resource).ok_or(UiNativeResourceFailure::Mesh(Mesh3dFault::Stale))?;
        let identity = original.loan.identity();
        if identity.operation != cx.operation().0 || identity.generation != cx.generation().0 { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Stale)); }
        Ok((original.loan.lease().ok_or(UiNativeResourceFailure::Mesh(Mesh3dFault::Closing))?, identity))
    }

    fn finish_transport<T>(&mut self, result: Result<T, UiNativeResourceFailure>, copied_bytes: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<T>, semio_framework_job::OriginalTurnFailure> {
        if matches!(&result, Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Busy))) { return Ok(None); }
        let progress = RetainedCloneProgress { copied_items: usize::from(result.is_ok()), copied_bytes, ..Default::default() };
        cx.consume_fuel(1);
        let receipt_refusal = cx.consume_retained(progress).err();
        match (result, receipt_refusal) {
            (Ok(output), None) => Ok(Some(output)),
            (Err(primary), receipt_refusal) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary, receipt_refusal }); Ok(None) },
            (Ok(_), Some(primary)) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Receipt(primary)); Ok(None) },
        }
    }

    pub fn describe_mesh_on_original(&mut self, resource: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<UiNativeMeshDescriptor>, semio_framework_job::OriginalTurnFailure> {
        if self.pending_failure.is_some() { return admit_native_resource_failure(&mut self.pending_failure, cx).map(|_| None); }
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let result = (|| {
            let (lease, identity) = self.original_transport_lease(resource, cx)?;
            Ok(UiNativeMeshDescriptor { identity, schema: lease.schema().map_err(UiNativeResourceFailure::Mesh)?, geometry_bytes: lease.original_geometry_bytes().map_err(UiNativeResourceFailure::Mesh)?, component_counts: lease.original_component_counts().map_err(UiNativeResourceFailure::Mesh)? })
        })();
        self.finish_transport(result, 0, cx)
    }

    pub fn read_mesh_chunk_on_original(&mut self, resource: usize, descriptor: UiNativeMeshDescriptor, channel: UiNativeMeshChannel, offset: usize, output: &mut [u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<UiNativeMeshChunk>, semio_framework_job::OriginalTurnFailure> {
        if self.pending_failure.is_some() { return admit_native_resource_failure(&mut self.pending_failure, cx).map(|_| None); }
        let grant = cx.retained_grant();
        let limit = output.len().min(grant.maximum_copy_bytes).min(UI_NATIVE_MESH_CHUNK_BYTES);
        if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0 || limit == 0 { return Ok(None); }
        let result = (|| {
            let (lease, identity) = self.original_transport_lease(resource, cx)?;
            if identity != descriptor.identity || lease.schema().map_err(UiNativeResourceFailure::Mesh)? != descriptor.schema || lease.original_geometry_bytes().map_err(UiNativeResourceFailure::Mesh)? != descriptor.geometry_bytes || lease.original_component_counts().map_err(UiNativeResourceFailure::Mesh)? != descriptor.component_counts { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Stale)); }
            match channel {
                UiNativeMeshChannel::Geometry => {
                    if offset > descriptor.geometry_bytes { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Schema)); }
                    if offset == descriptor.geometry_bytes { return Ok(UiNativeMeshChunk { bytes: 0, total_bytes: offset, complete: true }); }
                    lease.with_original_page(offset / MESH3D_PAGE_BYTES, |page| {
                        let start = offset % MESH3D_PAGE_BYTES;
                        let bytes = limit.min(page.len() - start);
                        output[..bytes].copy_from_slice(&page[start..start + bytes]);
                        UiNativeMeshChunk { bytes, total_bytes: descriptor.geometry_bytes, complete: offset + bytes == descriptor.geometry_bytes }
                    }).map_err(UiNativeResourceFailure::Mesh)
                },
                UiNativeMeshChannel::ComponentLabel { domain, group } => {
                    if descriptor.component_counts.get(domain).is_none_or(|count| group >= *count) { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Schema)); }
                    lease.with_original_component_label(domain, group, |label| {
                        if offset > label.len() { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Schema)); }
                        let bytes = limit.min(label.len() - offset);
                        output[..bytes].copy_from_slice(&label.as_bytes()[offset..offset + bytes]);
                        Ok(UiNativeMeshChunk { bytes, total_bytes: label.len(), complete: offset + bytes == label.len() })
                    }).map_err(UiNativeResourceFailure::Mesh)?
                },
            }
        })();
        let copied = result.as_ref().map_or(0, |chunk| chunk.bytes);
        self.finish_transport(result, copied, cx)
    }
}
