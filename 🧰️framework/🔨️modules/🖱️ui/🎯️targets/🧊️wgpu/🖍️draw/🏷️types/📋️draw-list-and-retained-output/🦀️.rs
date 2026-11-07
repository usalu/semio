//! 📋️ Draw list and retained output: the draw layer and list, the raster keep cursor, the retained output grant and error, the scene pass usage and the retained output claim API. See [`super`].

use super::*;

// #region draw_types
#[derive(Clone, Copy)]
pub(super) struct SharedBorderPaint {
    pub segment: SharedBorderSegment,
    pub layer: usize,
    pub slot: usize,
    pub overlay: bool,
    pub stroke: f32,
    pub color: Rgba,
    pub effect: Option<UiInstance>,
    pub side: u8,
}

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
    pub(super) shared_normal_finalized: bool,
    pub(super) shared_overlay_finalized: bool,
}

pub struct DrawList {
    pub scene_passes: PagedVec<ScenePass3d>,
    pub layers: PagedVec<DrawLayer>,
    pub(super) shared_borders: PagedVec<SharedBorderPaint>,
    pub glass_regions: PagedVec<GlassRegion>,
    pub(super) scissor_stack: Vec<ScissorRect>,
    pub(super) clip_stack: Vec<ClipRegion>,
    pub(super) glass_content_stack: Vec<usize>,
    pub(super) screen_h: f32,
    pub(super) clock_seconds: f32,
    pub(super) retained_output: Option<RetainedOutputGrant>,
    pub(super) prepared_items: usize,
    pub(super) prepared_bytes: usize,
    pub(super) border_finalize: Option<BorderFinalize>,
    pub(super) border_retained_bytes: usize,
    pub(super) border_finalized: bool,
    pub(super) retained_append: Option<Box<[RetainedAppendWork]>>,
    pub(super) append_sequence: u64,
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

impl Clone for DrawList {
    fn clone(&self) -> Self {
        let layers = self.layers.clone();
        let border_finalize = self.border_finalize.as_ref().map(|state| state.rebased_clone(layers.allocated_bytes()));
        Self {
            scene_passes: self.scene_passes.clone(), layers, shared_borders: self.shared_borders.clone(), glass_regions: self.glass_regions.clone(),
            scissor_stack: self.scissor_stack.clone(), clip_stack: self.clip_stack.clone(), glass_content_stack: self.glass_content_stack.clone(),
            screen_h: self.screen_h, clock_seconds: self.clock_seconds, retained_output: self.retained_output,
            prepared_items: self.prepared_items, prepared_bytes: self.prepared_bytes, border_finalize,
            border_retained_bytes: 0, border_finalized: self.border_finalized,
            retained_append: self.retained_append.clone(), append_sequence: self.append_sequence,
            overlay_route: self.overlay_route,
        }
    }
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
            scene_passes: PagedVec::new(),
            layers: PagedVec::new(),
            shared_borders: PagedVec::new(),
            glass_regions: PagedVec::new(),
            scissor_stack: Vec::new(),
            clip_stack: Vec::new(),
            glass_content_stack: Vec::new(),
            screen_h: 720.0,
            clock_seconds: 0.0,
            retained_output: None,
            prepared_items: 0,
            prepared_bytes: 0,
            border_finalize: None,
            border_retained_bytes: 0,
            border_finalized: false,
            retained_append: None,
            append_sequence: 0,
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
    include(pass.shadow_draws.len(), pass.shadow_draws.capacity().checked_mul(size_of::<semio_ui_scene_pass::SceneDraw3d>())?)?;
    include(pass.draws.len(), pass.draws.capacity().checked_mul(size_of::<semio_ui_scene_pass::SceneDraw3d>())?)?;
    include(pass.translucent_draws.len(), pass.translucent_draws.capacity().checked_mul(size_of::<semio_ui_scene_pass::SceneDraw3d>())?)?;
    for draw in pass.shadow_draws.iter().chain(pass.draws.iter()).chain(pass.translucent_draws.iter()) {
        include(draw.mesh_key.len(), draw.mesh_key.capacity())?;
        include(draw.instances.len(), draw.instances.capacity().checked_mul(size_of::<semio_ui_scene_mesh::Instance3d>())?)?;
        for instance in &draw.instances {
            include(instance.id.len(), instance.id.capacity())?;
        }
    }
    include(pass.material_draws.len(), pass.material_draws.capacity().checked_mul(size_of::<semio_ui_scene_pass::SceneMaterialDraw3d>())?)?;
    for draw in &pass.material_draws {
        include(draw.mesh_key.len(), draw.mesh_key.capacity())?;
        let texture_key = match &draw.material {
            semio_ui_scene_pass::SceneMaterialKind3d::Painted {texture_key} => Some(texture_key),
            semio_ui_scene_pass::SceneMaterialKind3d::Authored(material) => material.base_color_texture.as_ref(),
            semio_ui_scene_pass::SceneMaterialKind3d::Standard | semio_ui_scene_pass::SceneMaterialKind3d::Celebration {..} => None,
        };
        if let Some(texture_key) = texture_key {
            include(texture_key.len(), texture_key.capacity())?;
        }
        include(draw.instances.len(), draw.instances.capacity().checked_mul(size_of::<semio_ui_scene_mesh::Instance3d>())?)?;
        for instance in &draw.instances {
            include(instance.id.len(), instance.id.capacity())?;
        }
    }
    include(pass.line_draws.len(), pass.line_draws.capacity().checked_mul(size_of::<semio_ui_scene_pass::LineDraw3d>())?)?;
    for draw in &pass.line_draws {
        include(draw.vertices.len(), draw.vertices.capacity().checked_mul(size_of::<semio_ui_scene_pass::LineVertex3d>())?)?;
    }
    include(pass.textured_draws.len(), pass.textured_draws.capacity().checked_mul(size_of::<semio_ui_scene_pass::TexturedDraw3d>())?)?;
    for draw in &pass.textured_draws {
        include(draw.instances.len(), draw.instances.capacity().checked_mul(size_of::<semio_ui_scene_pass::TexturedInstance3d>())?)?;
        for instance in &draw.instances {
            include(instance.texture_key.len(), instance.texture_key.capacity())?;
        }
    }
    Some((items, bytes))
}

/// ❌️ A retained draw grant could not be admitted or completed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOutputError {
    Allocation,
    Capacity,
    AlreadyActive,
    NoActiveGrant,
    LimitExceeded,
    Finalized,
    AppendActive,
    InvalidAppendToken,
}

impl std::fmt::Display for RetainedOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Allocation => "retained output allocation failed",
            Self::Capacity => "retained output capacity overflowed",
            Self::AlreadyActive => "a retained output grant is already active",
            Self::NoActiveGrant => "no retained output grant is active",
            Self::LimitExceeded => "retained output exceeded its grant",
            Self::Finalized => "shared border frame is finalized; clear or create a new frame",
            Self::AppendActive => "retained append is already active",
            Self::InvalidAppendToken => "retained append token is stale",
        })
    }
}

impl std::error::Error for RetainedOutputError {}

impl DrawList {
    /// 🪣 Creates an allocation-free transfer slot for a later exact draw admission.
    pub fn empty() -> Self {
        Self {
            scene_passes: PagedVec::new(),
            layers: PagedVec::new(),
            shared_borders: PagedVec::new(),
            glass_regions: PagedVec::new(),
            scissor_stack: Vec::new(),
            clip_stack: Vec::new(),
            glass_content_stack: Vec::new(),
            screen_h: 0.0,
            clock_seconds: 0.0,
            retained_output: None,
            prepared_items: 0,
            prepared_bytes: 0,
            border_finalize: None,
            border_retained_bytes: 0,
            border_finalized: false,
            retained_append: None,
            append_sequence: 0,
            overlay_route: 0,
        }
    }

    /// ➕️ Publishes one balanced retained candidate after its producer reaches the accepted frame
    /// boundary. Indices are rebased onto the caller's existing frame and the caller's active clip
    /// and glass context continues across the appended layers.
    pub fn append_retained_candidate(&mut self, candidate: &mut Self) -> Result<(), RetainedOutputError> {
        let (token, _) = self.begin_retained_append(candidate);
        let token = token?;
        loop {
            match self.step_retained_append(token).0 {
                Ok(RetainedAppendStep::Pending) => {},
                Ok(RetainedAppendStep::Complete) => return Ok(()),
                Err(error) => {
                    loop {
                        match self.cancel_retained_append_step(token).0 {
                            Ok(RetainedAppendStep::Pending) => {},
                            Ok(RetainedAppendStep::Complete) => return Err(error),
                            Err(_) => return Err(error),
                        }
                    }
                }
            }
        }
    }

    /// 🎟️ Pre-admits fixed candidate backing before a retained paint child transfers output.
    pub fn try_reserve_retained_items(&mut self, items: usize) -> Result<(), RetainedOutputError> {
        if self.layers.is_empty() {
            self.layers.try_push(DrawLayer::default()).map_err(|_| RetainedOutputError::Allocation)?;
        }
        let border_limit = self.shared_borders.len().checked_add(items).ok_or(RetainedOutputError::Capacity)?;
        while self.shared_borders.capacity() < border_limit {
            let bytes = self.shared_borders.next_capacity_allocation_bytes(border_limit).map_err(|_| RetainedOutputError::Capacity)?.ok_or(RetainedOutputError::Capacity)?;
            self.shared_borders.reserve_capacity_one(border_limit, bytes).map_err(|_| RetainedOutputError::Allocation)?;
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
        if self.retained_append.is_some() || self.border_finalize.is_some() { return Err(RetainedOutputError::AppendActive); }
        if self.retained_output.is_some() {
            return Err(RetainedOutputError::AlreadyActive);
        }
        self.try_reserve_retained_items(item_limit)?;
        let layer_limit = self.layers.len().checked_add(item_limit).ok_or(RetainedOutputError::Capacity)?;
        while self.layers.capacity() < layer_limit {
            let bytes = self.layers.next_capacity_allocation_bytes(layer_limit).map_err(|_| RetainedOutputError::Capacity)?.ok_or(RetainedOutputError::Capacity)?;
            self.layers.reserve_capacity_one(layer_limit, bytes).map_err(|_| RetainedOutputError::Allocation)?;
        }
        let glass_limit = self.glass_regions.len().checked_add(item_limit).ok_or(RetainedOutputError::Capacity)?;
        while self.glass_regions.capacity() < glass_limit {
            let bytes = self.glass_regions.next_capacity_allocation_bytes(glass_limit).map_err(|_| RetainedOutputError::Capacity)?.ok_or(RetainedOutputError::Capacity)?;
            self.glass_regions.reserve_capacity_one(glass_limit, bytes).map_err(|_| RetainedOutputError::Allocation)?;
        }
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
        fn page<T>(value:&PagedVec<T>)->Result<usize,semio_framework_value::ValueError>{value.next_release_allocation_bytes().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"draw page retirement failed"))}
        if let Some(token) = self.active_retained_append_token() { return self.next_retained_append_cancel_demand(token).map(|grant| grant.maximum_release_bytes).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained append retirement preflight failed")); }
        if let Some(cursor) = &self.border_finalize { return cursor.next_release_bytes().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"shared border retirement failed")); }
        if let Some(pass)=self.scene_passes.last(){
            if let Some(draw)=pass.textured_draws.last(){if let Some(instance)=draw.instances.last(){return Ok(text(&instance.texture_key))}return if draw.instances.capacity()!=0{bytes(&draw.instances)}else{Ok(0)}}
            if let Some(draw)=pass.material_draws.last(){
                if let Some(instance)=draw.instances.last(){return Ok(text(&instance.id))}if draw.instances.capacity()!=0{return bytes(&draw.instances)}
                use semio_ui_scene_pass::SceneMaterialKind3d;
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
        if !self.shared_borders.is_empty(){return Ok(0)}
        if !self.shared_borders.terminal_is_empty(){return page(&self.shared_borders)}
        if let Some(layer)=self.layers.last(){
            if let Some(clip)=layer.clip.as_ref(){return if !clip.scissors.is_empty(){Ok(0)}else{page(&clip.scissors)}}
            if let Some((key,_))=layer.raster_instances.last(){return Ok(text(key))}if let Some((key,_))=layer.overlay_raster_instances.last(){return Ok(text(key))}
            if !layer.overlay_vector_vertices.is_empty()||!layer.overlay_ui_instances.is_empty()||!layer.vector_vertices.is_empty()||!layer.ui_instances.is_empty(){return Ok(0)}
            if layer.overlay_vector_vertices.capacity()!=0{return bytes(&layer.overlay_vector_vertices)}if layer.overlay_ui_instances.capacity()!=0{return bytes(&layer.overlay_ui_instances)}if layer.overlay_raster_instances.capacity()!=0{return bytes(&layer.overlay_raster_instances)}if layer.vector_vertices.capacity()!=0{return bytes(&layer.vector_vertices)}if layer.ui_instances.capacity()!=0{return bytes(&layer.ui_instances)}if layer.raster_instances.capacity()!=0{return bytes(&layer.raster_instances)}return Ok(0)
        }
        if !self.glass_regions.is_empty()||!self.scissor_stack.is_empty()||!self.glass_content_stack.is_empty(){return Ok(0)}
        if let Some(clip)=self.clip_stack.last(){return if !clip.scissors.is_empty(){Ok(0)}else{page(&clip.scissors)}}
        if !self.scene_passes.terminal_is_empty(){return page(&self.scene_passes)}if !self.layers.terminal_is_empty(){return page(&self.layers)}if !self.glass_regions.terminal_is_empty(){return page(&self.glass_regions)}if self.scissor_stack.capacity()!=0{return bytes(&self.scissor_stack)}if self.clip_stack.capacity()!=0{return bytes(&self.clip_stack)}if self.glass_content_stack.capacity()!=0{return bytes(&self.glass_content_stack)}Ok(0)
    }
    /// 🪜 Quotes the traversal depth needed by the next retained retirement step.
    pub fn next_retirement_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        match self.active_retained_append_token() {
            Some(token) => self.next_retained_append_cancel_demand(token).map(|demand| demand.maximum_copy_bytes).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "append cancellation copy preflight failed")),
            None => Ok(0),
        }
    }

    pub fn next_retirement_capacity_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        match self.active_retained_append_token() {
            Some(token) => self.next_retained_append_cancel_demand(token).map(|demand| demand.maximum_capacity_bytes).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "append cancellation capacity preflight failed")),
            None => Ok(0),
        }
    }

    pub fn next_retirement_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(token) = self.active_retained_append_token() { return self.next_retained_append_cancel_demand(token).map(|demand| demand.maximum_depth).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "append cancellation depth preflight failed")); }
        fn depth<T>(values: &PagedVec<T>) -> Result<usize, semio_framework_value::ValueError> {
            let demand = if !values.is_empty() { values.next_pop_depth_demand() }
            else if !values.terminal_is_empty() { values.next_release_depth_demand() }
            else { return Ok(0) };
            demand.map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"draw retirement depth overflow"))
        }
        if let Some(cursor) = &self.border_finalize { return cursor.next_retirement_depth().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"border retirement depth overflow")); }
        let mut result = 1usize.max(depth(&self.scene_passes)?).max(depth(&self.shared_borders)?).max(depth(&self.layers)?).max(depth(&self.glass_regions)?);
        if let Some(clip) = self.layers.last().and_then(|layer| layer.clip.as_ref()) { result = result.max(depth(&clip.scissors)?.saturating_add(1)); }
        if let Some(clip) = self.clip_stack.last() { result = result.max(depth(&clip.scissors)?.saturating_add(1)); }
        Ok(result)
    }

    pub fn retire_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();if self.retirement_is_empty(){return Ok(Step::Complete{progress:empty})}if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        let bytes=self.next_retirement_release_byte_demand()?;
        if bytes>grant.maximum_release_bytes||self.next_retirement_depth_demand()?>grant.maximum_depth||self.next_retirement_copy_byte_demand()?>grant.maximum_copy_bytes||self.next_retirement_capacity_byte_demand()?>grant.maximum_capacity_bytes{return Ok(Step::Pending{progress:empty})}
        if let Some(token)=self.active_retained_append_token(){
            let (result, progress)=self.cancel_retained_append_step(token);
            result.map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"append cancellation failed").with_retained_progress(progress))?;
            return Ok(if self.retirement_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}})
        }
        let complete=self.retire_step();let progress=RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:0,released_bytes:bytes};
        Ok(if complete{Step::Complete{progress}}else{Step::Pending{progress}})
    }

}
// #endregion draw_types
