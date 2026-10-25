//! 🛫️ Render upload and eviction: the render directive, the prepared render upload and the prepared render eviction. See [`super`].

use super::*;

/// 🧲️ Presentation behavior selected during worker preparation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderDirective {
    FullRedraw,
    DamageOnly,
    PreservePreviousOnFailure,
}

/// 📤️ Typed upload data owned by the prepared transaction.
#[derive(Debug, PartialEq)]
pub enum PreparedRenderUpload {
    #[cfg(any(test, feature = "testkit"))]
    GlyphAtlas {
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    #[cfg(any(test, feature = "testkit"))]
    IconAtlas {
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    GlyphAtlasPages {
        pixels: PreparedAtlasPages,
    },
    IconAtlasPages {
        pixels: PreparedAtlasPages,
    },
    #[cfg(any(test, feature = "testkit"))]
    Raster {
        key: String,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    RasterPages {
        key: String,
        pixels: PreparedRasterPages,
    },
    SceneRaster {
        key: String,
        lease: semio_framework_ui_raster_owner::SceneRasterLease,
    },
    Mesh {
        key: String,
        version: u64,
        lease: Mesh3dLease,
    },
}

/// 🧹️ UI-thread GPU cache invalidation selected during worker preparation.
#[derive(Debug, PartialEq, Eq)]
pub enum PreparedRenderEviction {
    Mesh { key: String },
}

impl PreparedRenderEviction {
    pub fn byte_len(&self) -> Option<usize> {
        match self {
            Self::Mesh { key } => Some(key.len()),
        }
    }
}

impl PreparedRenderUpload {
    pub fn next_close_demands(&self, maximum_copy_bytes: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Self::SceneRaster { key, lease } = self {
            if key.capacity() == 0 && !lease.terminal_is_empty() {
                return Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: lease.next_close_copy_byte_demand()?, maximum_capacity_bytes: lease.next_close_capacity_byte_demand(maximum_copy_bytes)?, maximum_release_bytes: lease.next_close_release_byte_demand()?, maximum_depth: lease.next_close_depth_demand()? });
            }
        }
        let mut demand = prepared_owner_demands(self.terminal_is_empty(), self.next_close_release_byte_demand())?;
        demand.maximum_copy_bytes = self.next_close_copy_byte_demand()?;
        Ok(demand)
    }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } | Self::Raster { pixels, .. } => Ok(usize::from(!pixels.is_empty())),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.next_close_copy_byte_demand(),
            _ => Ok(0),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => pixels.capacity() == 0,
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.terminal_is_empty(),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => pixels.capacity() == 0 && key.capacity() == 0,
            Self::RasterPages { key, pixels } => pixels.terminal_is_empty() && key.capacity() == 0,
            Self::SceneRaster { key, lease } => key.capacity() == 0 && lease.terminal_is_empty(),
            Self::Mesh { key, .. } => key.capacity() == 0,
        }
    }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => if pixels.is_empty() { prepared_vec_bytes(pixels) } else { Ok(0) },
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.next_close_release_byte_demand(),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => if !pixels.is_empty() { Ok(0) } else if pixels.capacity() != 0 { prepared_vec_bytes(pixels) } else { Ok(if key.is_empty() { key.capacity() } else { 0 }) },
            Self::RasterPages { key, pixels } => pixels.next_close_release_byte_demand(key),
            Self::SceneRaster { key, lease } => if key.capacity() != 0 { Ok(if key.is_empty() { key.capacity() } else { 0 }) } else { lease.next_close_release_byte_demand() },
            Self::Mesh { key, .. } => Ok(if key.is_empty() { key.capacity() } else { 0 }),
        }
    }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => return prepared_close_bytes(pixels, grant),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => return pixels.close_original_step(grant),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => { if !pixels.is_empty() || pixels.capacity() != 0 { return prepared_close_bytes(pixels, grant); } else { prepared_close_text(key); } }
            Self::RasterPages { key, pixels } => return pixels.close_with_key_granted(key, grant),
            Self::SceneRaster { key, lease } => {
                if key.capacity() != 0 { prepared_close_text(key); }
                else { return lease.close_step(grant).map(|step|match step { semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => CloseStep::Pending { progress }, semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => CloseStep::Complete { progress } }); }
            }
            Self::Mesh { key, .. } => { prepared_close_text(key); }
        }
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn byte_len(&self) -> Option<usize> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => Some(pixels.len()),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => Some(pixels.byte_len()),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => key.len().checked_add(pixels.len()),
            Self::RasterPages { key, pixels } => key.len().checked_add(pixels.byte_len()),
            Self::SceneRaster { key, .. } => key.len().checked_add(size_of::<semio_framework_ui_raster_owner::SceneRasterLease>()),
            Self::Mesh { key, lease, .. } => {
                let Ok(schema) = lease.schema() else { return Some(key.len()) };
                let bytes = usize::try_from(schema.vertices)
                    .ok()?
                    .checked_mul(24)?
                    .checked_add(usize::try_from(schema.indices).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.face_ids).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.vertex_ids).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.edges).ok()?.checked_mul(24)?)?
                    .checked_add(usize::try_from(schema.edge_ids).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.uvs).ok()?.checked_mul(8)?)?
                    .checked_add(usize::try_from(schema.colors).ok()?.checked_mul(16)?)?;
                key.len().checked_add(bytes)
            }
        }
    }

    pub fn scene_raster_byte_len(&self) -> usize {
        match self {
            Self::SceneRaster { lease, .. } => lease.byte_len(),
            _ => 0,
        }
    }

    /// 📪️ Releases one page, byte, or key scalar from a rejected upload owner.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        self.close_granted(grant)
    }
}
