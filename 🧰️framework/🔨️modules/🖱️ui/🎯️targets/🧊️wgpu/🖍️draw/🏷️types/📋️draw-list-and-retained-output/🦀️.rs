//! 📋️ Draw list and retained output: the draw layer and list, the raster keep cursor, the retained output grant and error, the scene pass usage and the retained output claim API. See [`super`].

use super::*;

// #region draw_types
#[derive(Clone, Default)]
pub struct DrawLayer {
    pub scissor: Option<ScissorRect>,
    pub clip: Option<ClipRegion>,
    pub foreground_of: Option<usize>,
    pub ui_instances: Vec<UiInstance>,
    pub raster_instances: Vec<(String, UiInstance)>,
    pub vector_vertices: Vec<VectorVertex>,
    pub overlay_ui_instances: Vec<UiInstance>,
    pub overlay_raster_instances: Vec<(String, UiInstance)>,
    pub overlay_vector_vertices: Vec<VectorVertex>,
}

#[derive(Clone)]
pub struct DrawList {
    pub scene_passes: Vec<ScenePass3d>,
    pub layers: Vec<DrawLayer>,
    pub glass_regions: Vec<GlassRegion>,
    pub(super) scissor_stack: Vec<ScissorRect>,
    pub(super) clip_stack: Vec<ClipRegion>,
    pub(super) glass_content_stack: Vec<usize>,
    pub(super) screen_h: f32,
    pub(super) clock_seconds: f32,
    pub(super) retained_output: Option<RetainedOutputGrant>,
    pub(super) prepared_items: usize,
    pub(super) prepared_bytes: usize,
    /// 🪟️ How many nested `begin_overlay_route`/`end_overlay_route` pairs are open. While non-zero
    /// every ordinary `push_*` below routes into the layer's OVERLAY bucket instead of its normal
    /// one — the bucket `ui_overlay_pass` draws after the main and raster passes, so whatever is
    /// painted inside the pair composites above every panel in the frame. This is what lets
    /// `engine::Ui`'s retained ladder paint a `Dialog`/`Popover`'s own surface chrome AND the
    /// document subtree that sits on it into the same top layer, in that order, instead of the
    /// chrome landing on top of its own content (ticket 26/09/17 packet W15a). Nothing else changes
    /// bucket: a caller that never opens a pair paints exactly where it always did.
    pub(super) overlay_route: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RasterKeepCursorV1 {
    phase: u8,
    outer: usize,
    middle: usize,
    inner: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterKeepStepV1<'a> {
    Pending,
    Key(&'a str),
    Complete,
}

#[derive(Clone, Copy)]
pub(super) struct RetainedOutputGrant {
    pub(super) item_limit: usize,
    pub(super) byte_limit: usize,
    pub(super) items: usize,
    pub(super) bytes: usize,
    pub(super) faulted: bool,
}

impl Default for DrawList {
    fn default() -> Self {
        let mut list = Self {
            scene_passes: Vec::new(),
            layers: Vec::new(),
            glass_regions: Vec::new(),
            scissor_stack: Vec::new(),
            clip_stack: Vec::new(),
            glass_content_stack: Vec::new(),
            screen_h: 720.0,
            clock_seconds: 0.0,
            retained_output: None,
            prepared_items: 0,
            prepared_bytes: 0,
            overlay_route: 0,
        };
        list.layers.push(DrawLayer::default());
        list
    }
}

impl DrawList {
    pub fn raster_keep_step<'a>(&'a self, cursor: &mut RasterKeepCursorV1) -> RasterKeepStepV1<'a> {
        match cursor.phase {
            0 => {
                let Some(pass) = self.scene_passes.get(cursor.outer) else {
                    cursor.phase = 1;
                    cursor.outer = 0;
                    cursor.middle = 0;
                    cursor.inner = 0;
                    return RasterKeepStepV1::Pending;
                };
                let Some(draw) = pass.textured_draws.get(cursor.middle) else {
                    cursor.outer += 1;
                    cursor.middle = 0;
                    cursor.inner = 0;
                    return RasterKeepStepV1::Pending;
                };
                let Some(instance) = draw.instances.get(cursor.inner) else {
                    cursor.middle += 1;
                    cursor.inner = 0;
                    return RasterKeepStepV1::Pending;
                };
                cursor.inner += 1;
                RasterKeepStepV1::Key(instance.texture_key.as_str())
            }
            1 => {
                let Some(pass) = self.scene_passes.get(cursor.outer) else {
                    cursor.phase = 2;
                    cursor.outer = 0;
                    cursor.middle = 0;
                    cursor.inner = 0;
                    return RasterKeepStepV1::Pending;
                };
                let Some(draw) = pass.material_draws.get(cursor.middle) else {
                    cursor.outer += 1;
                    cursor.middle = 0;
                    return RasterKeepStepV1::Pending;
                };
                if let Some(key)=draw.material.texture_keys().nth(cursor.inner) {cursor.inner+=1;RasterKeepStepV1::Key(key)} else {cursor.middle+=1;cursor.inner=0;RasterKeepStepV1::Pending}
            }
            2 | 3 => {
                let Some(layer) = self.layers.get(cursor.outer) else {
                    cursor.phase += 1;
                    cursor.outer = 0;
                    cursor.inner = 0;
                    return RasterKeepStepV1::Pending;
                };
                let instances = if cursor.phase == 2 { &layer.raster_instances } else { &layer.overlay_raster_instances };
                let Some((key, _)) = instances.get(cursor.inner) else {
                    cursor.outer += 1;
                    cursor.inner = 0;
                    return RasterKeepStepV1::Pending;
                };
                cursor.inner += 1;
                RasterKeepStepV1::Key(key.as_str())
            }
            _ => RasterKeepStepV1::Complete,
        }
    }
}

pub(super) fn prepared_scene_pass_usage(pass: &ScenePass3d) -> Option<(usize, usize)> {
    let mut items = 1usize;
    let mut bytes = size_of::<ScenePass3d>();
    let mut include = |next_items: usize, next_bytes: usize| {
        items = items.checked_add(next_items)?;
        bytes = bytes.checked_add(next_bytes)?;
        Some(())
    };
    include(pass.shadow_draws.len(), pass.shadow_draws.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::SceneDraw3d>())?)?;
    include(pass.draws.len(), pass.draws.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::SceneDraw3d>())?)?;
    include(pass.translucent_draws.len(), pass.translucent_draws.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::SceneDraw3d>())?)?;
    for draw in pass.shadow_draws.iter().chain(pass.draws.iter()).chain(pass.translucent_draws.iter()) {
        include(draw.mesh_key.len(), draw.mesh_key.capacity())?;
        include(draw.instances.len(), draw.instances.capacity().checked_mul(size_of::<semio_framework_ui_scene_mesh::Instance3d>())?)?;
        for instance in &draw.instances {
            include(instance.id.len(), instance.id.capacity())?;
        }
    }
    include(pass.material_draws.len(), pass.material_draws.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::SceneMaterialDraw3d>())?)?;
    for draw in &pass.material_draws {
        include(draw.mesh_key.len(), draw.mesh_key.capacity())?;
        let texture_key = match &draw.material {
            semio_framework_ui_scene_pass::SceneMaterialKind3d::Painted {texture_key} => Some(texture_key),
            semio_framework_ui_scene_pass::SceneMaterialKind3d::Authored(material) => material.base_color_texture.as_ref(),
            semio_framework_ui_scene_pass::SceneMaterialKind3d::Standard | semio_framework_ui_scene_pass::SceneMaterialKind3d::Celebration {..} => None,
        };
        if let Some(texture_key) = texture_key {
            include(texture_key.len(), texture_key.capacity())?;
        }
        include(draw.instances.len(), draw.instances.capacity().checked_mul(size_of::<semio_framework_ui_scene_mesh::Instance3d>())?)?;
        for instance in &draw.instances {
            include(instance.id.len(), instance.id.capacity())?;
        }
    }
    include(pass.line_draws.len(), pass.line_draws.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::LineDraw3d>())?)?;
    for draw in &pass.line_draws {
        include(draw.vertices.len(), draw.vertices.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::LineVertex3d>())?)?;
    }
    include(pass.textured_draws.len(), pass.textured_draws.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::TexturedDraw3d>())?)?;
    for draw in &pass.textured_draws {
        include(draw.instances.len(), draw.instances.capacity().checked_mul(size_of::<semio_framework_ui_scene_pass::TexturedInstance3d>())?)?;
        for instance in &draw.instances {
            include(instance.texture_key.len(), instance.texture_key.capacity())?;
        }
    }
    Some((items, bytes))
}

/// 🧾️ A retained draw grant could not be admitted or completed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOutputError {
    Allocation,
    Capacity,
    AlreadyActive,
    NoActiveGrant,
    LimitExceeded,
}

impl std::fmt::Display for RetainedOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Allocation => "retained output allocation failed",
            Self::Capacity => "retained output capacity overflowed",
            Self::AlreadyActive => "a retained output grant is already active",
            Self::NoActiveGrant => "no retained output grant is active",
            Self::LimitExceeded => "retained output exceeded its grant",
        })
    }
}

impl std::error::Error for RetainedOutputError {}

impl DrawList {
    /// 🪣 Creates an allocation-free transfer slot for a later exact draw admission.
    pub fn empty() -> Self {
        Self {
            scene_passes: Vec::new(),
            layers: Vec::new(),
            glass_regions: Vec::new(),
            scissor_stack: Vec::new(),
            clip_stack: Vec::new(),
            glass_content_stack: Vec::new(),
            screen_h: 0.0,
            clock_seconds: 0.0,
            retained_output: None,
            prepared_items: 0,
            prepared_bytes: 0,
            overlay_route: 0,
        }
    }

    /// 🧾️ Publishes one balanced retained candidate after its producer reaches the accepted frame
    /// boundary. Indices are rebased onto the caller's existing frame and the caller's active clip
    /// and glass context continues across the appended layers.
    pub fn append_retained_candidate(&mut self, candidate: &mut Self) -> Result<(), RetainedOutputError> {
        if !candidate.scissor_stack.is_empty() || !candidate.clip_stack.is_empty() || !candidate.glass_content_stack.is_empty() || candidate.retained_output.is_some() || candidate.overlay_route != 0 {
            return Err(RetainedOutputError::Capacity);
        }
        if !self.claim_retained_output(candidate.prepared_items, candidate.prepared_bytes) {
            return Err(RetainedOutputError::LimitExceeded);
        }
        let layer_offset = self.layers.len();
        let glass_offset = self.glass_regions.len();
        let inherited_scissor = self.scissor_stack.last().copied();
        let inherited_clip = self.clip_stack.last().cloned();
        let inherited_foreground = self.glass_content_stack.last().copied();
        for pass in &mut candidate.scene_passes {
            pass.layer_index = pass.layer_index.checked_add(layer_offset).ok_or(RetainedOutputError::Capacity)?;
        }
        for region in &mut candidate.glass_regions {
            region.layer_index = region.layer_index.checked_add(layer_offset).ok_or(RetainedOutputError::Capacity)?;
        }
        for layer in &mut candidate.layers {
            layer.scissor = match (layer.scissor, inherited_scissor) {
                (Some(local), Some(parent)) => Some(local.intersect(&parent)),
                (local, parent) => local.or(parent),
            };
            layer.clip = match (&layer.clip, &inherited_clip) {
                (Some(local), Some(parent)) => Some(local.intersect(parent)),
                (Some(local), None) => Some(local.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            };
            layer.foreground_of = match layer.foreground_of {
                Some(region) => Some(region.checked_add(glass_offset).ok_or(RetainedOutputError::Capacity)?),
                None => inherited_foreground,
            };
        }
        self.scene_passes.append(&mut candidate.scene_passes);
        self.layers.append(&mut candidate.layers);
        self.glass_regions.append(&mut candidate.glass_regions);
        candidate.prepared_items = 0;
        candidate.prepared_bytes = 0;
        Ok(())
    }

    /// 🎟️ Pre-admits fixed candidate backing before a retained paint child transfers output.
    pub fn try_reserve_retained_items(&mut self, items: usize) -> Result<(), RetainedOutputError> {
        if self.layers.is_empty() {
            self.layers.try_reserve_exact(1).map_err(|_| RetainedOutputError::Allocation)?;
            self.layers.push(DrawLayer::default());
        }
        let vertices = items.checked_mul(6).ok_or(RetainedOutputError::Capacity)?;
        let layer = self.layers.last_mut().ok_or(RetainedOutputError::Capacity)?;
        layer.ui_instances.try_reserve_exact(items).map_err(|_| RetainedOutputError::Allocation)?;
        layer.overlay_ui_instances.try_reserve_exact(items).map_err(|_| RetainedOutputError::Allocation)?;
        layer.overlay_raster_instances.try_reserve_exact(items).map_err(|_| RetainedOutputError::Allocation)?;
        layer.vector_vertices.try_reserve_exact(vertices).map_err(|_| RetainedOutputError::Allocation)?;
        layer.overlay_vector_vertices.try_reserve_exact(vertices).map_err(|_| RetainedOutputError::Allocation)?;
        layer.raster_instances.try_reserve_exact(items).map_err(|_| RetainedOutputError::Allocation)?;
        Ok(())
    }

    /// 🎫️ Starts one exact retained output grant and pre-admits all fixed container backing.
    pub fn begin_retained_output(&mut self, item_limit: usize, byte_limit: usize) -> Result<(), RetainedOutputError> {
        if self.retained_output.is_some() {
            return Err(RetainedOutputError::AlreadyActive);
        }
        self.try_reserve_retained_items(item_limit)?;
        self.layers.try_reserve_exact(item_limit).map_err(|_| RetainedOutputError::Allocation)?;
        self.glass_regions.try_reserve_exact(item_limit).map_err(|_| RetainedOutputError::Allocation)?;
        self.scissor_stack.try_reserve_exact(item_limit).map_err(|_| RetainedOutputError::Allocation)?;
        self.glass_content_stack.try_reserve_exact(item_limit).map_err(|_| RetainedOutputError::Allocation)?;
        self.retained_output = Some(RetainedOutputGrant { item_limit, byte_limit, items: 0, bytes: 0, faulted: false });
        Ok(())
    }

    /// 🧾️ Closes the exact retained output grant and reports any attempted overflow.
    pub fn finish_retained_output(&mut self) -> Result<(usize, usize), RetainedOutputError> {
        let Some(grant) = self.retained_output.take() else { return Err(RetainedOutputError::NoActiveGrant) };
        if grant.faulted {
            return Err(RetainedOutputError::LimitExceeded);
        }
        Ok((grant.items, grant.bytes))
    }

    pub(super) fn claim_retained_output(&mut self, items: usize, bytes: usize) -> bool {
        let Some(prepared_items) = self.prepared_items.checked_add(items) else {
            if let Some(grant) = self.retained_output.as_mut() {
                grant.faulted = true;
            }
            return false;
        };
        let Some(prepared_bytes) = self.prepared_bytes.checked_add(bytes) else {
            if let Some(grant) = self.retained_output.as_mut() {
                grant.faulted = true;
            }
            return false;
        };
        if let Some(grant) = self.retained_output.as_mut() {
            let Some(next_items) = grant.items.checked_add(items) else {
                grant.faulted = true;
                return false;
            };
            let Some(next_bytes) = grant.bytes.checked_add(bytes) else {
                grant.faulted = true;
                return false;
            };
            if next_items > grant.item_limit || next_bytes > grant.byte_limit {
                grant.faulted = true;
                return false;
            }
            grant.items = next_items;
            grant.bytes = next_bytes;
        }
        self.prepared_items = prepared_items;
        self.prepared_bytes = prepared_bytes;
        true
    }

    /// 📊 Returns the cumulative pre-transfer draw claim maintained at every producer push.
    pub fn prepared_output_usage(&self) -> (usize, usize) {
        (self.prepared_items, self.prepared_bytes)
    }

    /// ♻️ One bounded retirement slice of a retained draw list. `pub`, not `pub`: the wgpu
    /// renderer owns the previous frame's `DrawList` across a frame boundary and drives this ladder
    /// itself — the same contract `prepared.rs`'s `retire_step` already exposes.
    pub fn next_retirement_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        fn bytes<T>(value:&Vec<T>)->Result<usize,semio_framework_value::ValueError>{value.capacity().checked_mul(size_of::<T>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"draw backing extent overflow"))}
        fn text(value:&String)->usize{if value.is_empty(){value.capacity()}else{0}}
        if let Some(pass)=self.scene_passes.last(){
            if let Some(draw)=pass.textured_draws.last(){if let Some(instance)=draw.instances.last(){return Ok(text(&instance.texture_key))}return if draw.instances.capacity()!=0{bytes(&draw.instances)}else{Ok(0)}}
            if let Some(draw)=pass.material_draws.last(){
                if let Some(instance)=draw.instances.last(){return Ok(text(&instance.id))}if draw.instances.capacity()!=0{return bytes(&draw.instances)}
                use semio_framework_ui_scene_pass::SceneMaterialKind3d;
                let key=match &draw.material{SceneMaterialKind3d::Painted{texture_key}=>Some(texture_key),SceneMaterialKind3d::Authored(material)=>[&material.base_color_texture,&material.metallic_roughness_texture,&material.normal_texture,&material.occlusion_texture,&material.emissive_texture].into_iter().flatten().find(|key|!key.is_empty()||key.capacity()!=0),_=>None};
                if let Some(key)=key{if !key.is_empty()||key.capacity()!=0{return Ok(text(key))}}
                return Ok(text(&draw.mesh_key))
            }
            if let Some(draw)=pass.translucent_draws.last(){if let Some(instance)=draw.instances.last(){return Ok(text(&instance.id))}if draw.instances.capacity()!=0{return bytes(&draw.instances)}return Ok(text(&draw.mesh_key))}
            if let Some(draw)=pass.line_draws.last(){return if !draw.vertices.is_empty(){Ok(0)}else{bytes(&draw.vertices)}}
            if let Some(draw)=pass.draws.last(){if let Some(instance)=draw.instances.last(){return Ok(text(&instance.id))}if draw.instances.capacity()!=0{return bytes(&draw.instances)}return Ok(text(&draw.mesh_key))}
            if let Some(draw)=pass.shadow_draws.last(){if let Some(instance)=draw.instances.last(){return Ok(text(&instance.id))}if draw.instances.capacity()!=0{return bytes(&draw.instances)}return Ok(text(&draw.mesh_key))}
            if pass.textured_draws.capacity()!=0{return bytes(&pass.textured_draws)}if pass.material_draws.capacity()!=0{return bytes(&pass.material_draws)}if pass.translucent_draws.capacity()!=0{return bytes(&pass.translucent_draws)}if pass.line_draws.capacity()!=0{return bytes(&pass.line_draws)}if pass.draws.capacity()!=0{return bytes(&pass.draws)}if pass.shadow_draws.capacity()!=0{return bytes(&pass.shadow_draws)}return Ok(0)
        }
        if let Some(layer)=self.layers.last(){
            if let Some(clip)=layer.clip.as_ref(){return if !clip.scissors.is_empty(){Ok(0)}else{bytes(&clip.scissors)}}
            if let Some((key,_))=layer.raster_instances.last(){return Ok(text(key))}if let Some((key,_))=layer.overlay_raster_instances.last(){return Ok(text(key))}
            if !layer.overlay_vector_vertices.is_empty()||!layer.overlay_ui_instances.is_empty()||!layer.vector_vertices.is_empty()||!layer.ui_instances.is_empty(){return Ok(0)}
            if layer.overlay_vector_vertices.capacity()!=0{return bytes(&layer.overlay_vector_vertices)}if layer.overlay_ui_instances.capacity()!=0{return bytes(&layer.overlay_ui_instances)}if layer.overlay_raster_instances.capacity()!=0{return bytes(&layer.overlay_raster_instances)}if layer.vector_vertices.capacity()!=0{return bytes(&layer.vector_vertices)}if layer.ui_instances.capacity()!=0{return bytes(&layer.ui_instances)}if layer.raster_instances.capacity()!=0{return bytes(&layer.raster_instances)}return Ok(0)
        }
        if !self.glass_regions.is_empty()||!self.scissor_stack.is_empty()||!self.glass_content_stack.is_empty(){return Ok(0)}
        if let Some(clip)=self.clip_stack.last(){return if !clip.scissors.is_empty(){Ok(0)}else{bytes(&clip.scissors)}}
        if self.scene_passes.capacity()!=0{return bytes(&self.scene_passes)}if self.layers.capacity()!=0{return bytes(&self.layers)}if self.glass_regions.capacity()!=0{return bytes(&self.glass_regions)}if self.scissor_stack.capacity()!=0{return bytes(&self.scissor_stack)}if self.clip_stack.capacity()!=0{return bytes(&self.clip_stack)}if self.glass_content_stack.capacity()!=0{return bytes(&self.glass_content_stack)}Ok(0)
    }
    pub fn retire_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();if self.retirement_is_empty(){return Ok(Step::Complete{progress:empty})}if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        let bytes=self.next_retirement_release_byte_demand()?;
        if bytes>grant.maximum_release_bytes{return Ok(Step::Pending{progress:empty})}if grant.maximum_depth==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original draw close requires depth"))}
        let complete=self.retire_step();let progress=RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:0,released_bytes:bytes};
        Ok(if complete{Step::Complete{progress}}else{Step::Pending{progress}})
    }

}
// #endregion draw_types
