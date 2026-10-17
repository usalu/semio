//! 🧾️ Drawing mutation digest: the mutation digest authority, the workset plan and the mutation target owner.

use super::*;

pub struct DrawingMutationDigestAuthority {
    pub layer: Option<DrawingLayerDigestAuthority>,
    pub fill: Option<DrawingFillDigestAuthority>,
    pub stroke: Option<DrawingStrokeDigestAuthority>,
    pub segment: Option<DrawingPathSegmentDigestAuthority>,
    pub segment_index: usize,
    pub credit: DrawingSemanticDigestCredit,
    pub phase: u8,
    pub terminal: bool,
}

impl DrawingMutationDigestAuthority {
    pub fn new() -> Self {
        Self { layer: None, fill: None, stroke: None, segment: None, segment_index: 0, credit: DrawingSemanticDigestCredit::default(), phase: 0, terminal: false }
    }

    pub fn variant(mutation: &DrawingMutation) -> u8 {
        match mutation {
            DrawingMutation::ImportImageAsset(_) => 25,
            DrawingMutation::RemoveImageAsset(_) => 26,
            DrawingMutation::SetLayerVisible(_) => 1,
            DrawingMutation::SetLayerLocked(_) => 2,
            DrawingMutation::SetLayerOpacity(_) => 3,
            DrawingMutation::SetLayerBlendMode(_) => 4,
            DrawingMutation::RenameLayer(_) => 5,
            DrawingMutation::UpdateLayerTransform(_) => 6,
            DrawingMutation::ReplaceLayerFill(_) => 7,
            DrawingMutation::ReplaceLayerStroke(_) => 8,
            DrawingMutation::SetLayerBooleanOperation(_) => 9,
            DrawingMutation::UpdateLayerTraceParams(_) => 10,
            DrawingMutation::CreateLayer(_) => 11,
            DrawingMutation::DuplicateLayer(_) => 12,
            DrawingMutation::DeleteLayer(_) => 13,
            DrawingMutation::ReorderLayer(_) => 14,
            DrawingMutation::UpdatePathGeometry(_) => 15,
            DrawingMutation::UpdateText(_) => 16,
            DrawingMutation::UpdateImage(_) => 23,
            DrawingMutation::SetShapeCoordinate(_) => 24,
            DrawingMutation::SetLayerFillRule(_) => 17,
            DrawingMutation::SetGroupIsolation(_) => 18,
            DrawingMutation::DragLayers(_) => 19,
            DrawingMutation::RotateLayers(_) => 20,
            DrawingMutation::ScaleLayers(_) => 21,
            DrawingMutation::DragPathPoints(_) => 22,
        }
    }

    pub fn finish(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        self.credit.seal(digest, cx)?;
        self.terminal = true;
        Ok(true)
    }

    /// 🧮️ The scalar parameters of a selection transform leaf in declaration order, and how many there are.
    pub fn selection_scalars(mutation: &DrawingMutation) -> ([f64; 4], usize) {
        match mutation {
            DrawingMutation::DragLayers(value) => ([value.dx, value.dy, 0.0, 0.0], 2),
            DrawingMutation::RotateLayers(value) => ([value.pivot_x, value.pivot_y, value.angle, 0.0], 3),
            DrawingMutation::ScaleLayers(value) => ([value.pivot_x, value.pivot_y, value.scale_x, value.scale_y], 4),
            DrawingMutation::DragPathPoints(value) => ([value.dx, value.dy, 0.0, 0.0], 2),
            _ => ([0.0; 4], 0),
        }
    }

    pub fn step(&mut self, mutation: &DrawingMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            self.credit.observe(digest, 1, &[Self::variant(mutation)], cx)?;
            self.phase = 1;
            return Ok(false);
        }
        if self.phase == 1 {
            match mutation {
                DrawingMutation::CreateLayer(value) => {
                    self.credit.add_source_owner(1, size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.add_derived_owner(1, size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.observe(digest, 2, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 2 } else { 3 };
                }
                DrawingMutation::DragLayers(semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers { targets, .. }) => {
                    self.credit.source_vec(targets)?;
                    self.credit.scalar_usize(digest, 2, targets.len(), cx)?;
                    self.phase = 2;
                }
                DrawingMutation::DragPathPoints(value) => {
                    self.credit.source_vec(&value.targets)?;
                    self.credit.scalar_usize(digest, 2, value.targets.len(), cx)?;
                    self.phase = 2;
                }
                _ => {
                    let target = drawing_mutation_target_owner(mutation).ok_or("drawing-store.mutation-target-owner")?;
                    if !self.credit.observe_owned_string(digest, 2, target, false, cx)? { return Ok(false); }
                    self.phase = 2;
                }
            }
            return Ok(false);
        }
        match mutation {
            DrawingMutation::DragLayers(semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers { targets, .. }) => {
                if let Some(target) = targets.get(self.segment_index) {
                    if !self.credit.observe_owned_string(digest, 3, target, false, cx)? { return Ok(false); }
                    self.segment_index += 1;
                    return Ok(false);
                }
                let (scalars, count) = Self::selection_scalars(mutation);
                let offset = usize::from(self.phase - 2);
                if offset < count {
                    self.credit.scalar_f64(digest, 4 + offset as u16, scalars[offset], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::DragPathPoints(value) => {
                if let Some(target) = value.targets.get(self.segment_index) {
                    if !self.credit.observe_owned_string(digest, 3, &target.layer_id, false, cx)? { return Ok(false); }
                    let point = match target.point {
                        semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Anchor => 0u64,
                        semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Control1 => 1,
                        semio_s_artifact_draw_drawing_geometry::editing::PathPoint::Control2 => 2,
                    };
                    self.credit.scalar_usize(digest, 4, (target.index as u64).checked_mul(3).and_then(|index| index.checked_add(point)).ok_or("drawing-store.mutation-point-index-overflow")? as usize, cx)?;
                    self.segment_index += 1;
                    return Ok(false);
                }
                let (scalars, count) = Self::selection_scalars(mutation);
                let offset = usize::from(self.phase - 2);
                if offset < count {
                    self.credit.scalar_f64(digest, 5 + offset as u16, scalars[offset], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdatePathGeometry(value) => match self.phase {
                2 => {
                    self.credit.source_vec(&value.segments)?;
                    self.credit.derived_vec(&value.segments)?;
                    self.credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 3;
                    Ok(false)
                }
                _ => {
                    let Some(segment) = value.segments.get(self.segment_index) else { return self.finish(digest, cx) };
                    if !semio_s_artifact_draw_drawing_model::valid_path_segment(segment) { return Err("drawing-store.path-invalid-segment"); }
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, &mut self.credit, cx)? { self.segment = None; self.segment_index += 1; }
                    Ok(false)
                }
            },
            DrawingMutation::SetLayerVisible(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.visible)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerLocked(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.locked)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerFillRule(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest,3,&[u8::from(value.fill_rule==semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero)],cx)?;
                    self.phase=3;Ok(false)
                } else {self.finish(digest,cx)}
            }
            DrawingMutation::SetGroupIsolation(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest,3,&[u8::from(value.isolation)],cx)?;
                    self.phase=3;Ok(false)
                } else {self.finish(digest,cx)}
            }
            DrawingMutation::SetLayerOpacity(value) => {
                if self.phase == 2 {
                    self.credit.scalar_f64(digest, 3, value.opacity, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBlendMode(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.blend_mode, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetShapeCoordinate(value)=>match self.phase {
                2=>{self.credit.observe(digest,3,value.field.as_str().as_bytes(),cx)?;self.phase=3;Ok(false)}
                3=>{self.credit.observe(digest,4,&[u8::from(value.index.is_some())],cx)?;self.phase=4;Ok(false)}
                4=>{if let Some(index)=value.index{self.credit.scalar_usize(digest,5,index,cx)?;}self.phase=5;Ok(false)}
                5=>{self.credit.scalar_f64(digest,6,value.value,cx)?;self.phase=6;Ok(false)}
                _=>self.finish(digest,cx),
            },
            DrawingMutation::UpdateImage(value) => match self.phase {
                2=>{if !self.credit.observe_owned_string(digest,3,&value.image_key,true,cx)?{return Ok(false);}self.phase=3;Ok(false)}
                3=>{self.credit.scalar_f64(digest,4,value.width,cx)?;self.phase=4;Ok(false)}
                4=>{self.credit.scalar_f64(digest,5,value.height,cx)?;self.phase=5;Ok(false)}
                _=>self.finish(digest,cx),
            },
            DrawingMutation::UpdateText(value) => match self.phase {
                2 => { if !self.credit.observe_owned_string(digest, 3, &value.content, true, cx)? { return Ok(false); } self.phase = 3; Ok(false) }
                3 => { self.credit.scalar_f64(digest, 4, value.size, cx)?; self.phase = 4; Ok(false) }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::RenameLayer(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.new_name, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTransform(value) => {
                if self.phase <= 7 {
                    let fields = [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation, value.transform.shear];
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerFill(value) => {
                if self.phase == 2 {
                    let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                    if fill.step(value.fill.as_ref(), digest, &mut self.credit, cx)? {
                        self.fill = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerStroke(value) => {
                if self.phase == 2 {
                    let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                    if stroke.step(value.stroke.as_ref(), digest, &mut self.credit, cx)? {
                        self.stroke = None;
        self.segment = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBooleanOperation(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.boolean_operation, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTraceParams(value) => {
                if self.phase <= 3 {
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), [value.params.threshold, value.params.simplify_epsilon][(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::CreateLayer(value) => match self.phase {
                2 => {
                    if !self.credit.observe_owned_string(digest, 3, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                }
                3 => {
                    self.credit.observe(digest, 4, &[u8::from(value.index.is_some())], cx)?;
                    self.phase = if value.index.is_some() { 4 } else { 5 };
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index.ok_or("drawing-store.digest-index-missing")?, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                5 => {
                    let layer = self.layer.get_or_insert_with(DrawingLayerDigestAuthority::new);
                    if layer.step(&value.layer, digest, &mut self.credit, cx)? {
                        self.layer = None;
                        self.phase = 6;
                    }
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::DuplicateLayer(value)=>{
                if self.phase==2 {self.credit.source_vec(&value.identities)?;self.credit.scalar_usize(digest,3,value.identities.len(),cx)?;self.phase=3;return Ok(false);}
                if let Some(assignment)=value.identities.get(self.segment_index){
                    let field=if self.phase==3{&assignment.source}else{&assignment.target};
                    if !self.credit.observe_owned_string(digest,if self.phase==3{4}else{5},field,false,cx)?{return Ok(false);}
                    if self.phase==3{self.phase=4;}else{self.phase=3;self.segment_index+=1;}return Ok(false);
                }self.finish(digest,cx)
            }
            DrawingMutation::RemoveImageAsset(_) | DrawingMutation::DeleteLayer(_) => self.finish(digest, cx),
            DrawingMutation::ImportImageAsset(value)=>match self.phase {
                2=>{self.credit.scalar_usize(digest,3,value.asset.width as usize,cx)?;self.phase=3;Ok(false)}
                3=>{self.credit.scalar_usize(digest,4,value.asset.height as usize,cx)?;self.phase=4;Ok(false)}
                4=>{self.credit.source_vec(&value.asset.samples)?;self.credit.derived_vec(&value.asset.samples)?;self.credit.scalar_usize(digest,5,value.asset.samples.len(),cx)?;self.phase=5;Ok(false)}
                _=>{if let Some(sample)=value.asset.samples.get(self.segment_index){self.credit.observe(digest,6,sample,cx)?;self.segment_index+=1;Ok(false)}else{self.finish(digest,cx)}}
            },
            DrawingMutation::ReorderLayer(value) => match self.phase {
                2 => {
                    self.credit.observe(digest, 3, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 3 } else { 4 };
                    Ok(false)
                }
                3 => {
                    if !self.credit.observe_owned_string(digest, 4, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)? { return Ok(false); }
                    self.phase = 4;
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
        }
    }

    pub fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.terminal.then(|| self.credit.totals()).flatten()
    }

    pub fn close_step(&mut self) -> RetainedCloneStep {
        if let Some(layer) = self.layer.as_mut() {
            layer.close();
        }
        self.layer = None;
        self.fill = None;
        self.stroke = None;
        self.segment = None;
        self.terminal = true;
        RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.layer.is_none() && self.fill.is_none() && self.stroke.is_none() && self.segment.is_none()
    }
}

impl Drop for DrawingMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation digest reached Drop before exact terminal close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingMutationWorksetPlan {
    pub source_items: usize,
    pub source_bytes: usize,
    pub mutation_items: usize,
    pub mutation_bytes: usize,
    pub arena_items: usize,
    pub arena_bytes: usize,
    pub authority_items: usize,
    pub authority_bytes: usize,
    pub clone_items: usize,
    pub clone_bytes: usize,
    pub maximum_container: usize,
    pub container_slots: usize,
}

impl DrawingMutationWorksetPlan {
    pub fn checked_total(values: &[usize], fault: &'static str) -> Result<usize, &'static str> {
        values.iter().try_fold(0usize, |total, value| total.checked_add(*value).ok_or(fault))
    }

    pub fn admit(
        source: DrawingSnapshotOwnerTotals,
        mutation: DrawingSemanticDigestTotals,
        arena_items: usize,
        arena_bytes: usize,
        container_slots: usize,
        authority_bytes: usize,
    ) -> Result<Self, &'static str> {
        if source.source_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.preflight-item-capacity");
        }
        if source.source_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.preflight-byte-capacity");
        }
        if mutation.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if mutation.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        if mutation.maximum_field_bytes > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        Ok(Self {
            source_items: source.source_items,
            source_bytes: source.source_bytes,
            mutation_items: mutation.semantic_items,
            mutation_bytes: mutation.maximum_field_bytes,
            arena_items,
            arena_bytes,
            authority_items: 1,
            authority_bytes,
            clone_items: 0,
            clone_bytes: 0,
            maximum_container: source.maximum_container,
            container_slots,
        })
    }

    pub fn admit_clone(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        if items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-clone-item-capacity");
        }
        if bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-clone-byte-capacity");
        }
        self.clone_items = items;
        self.clone_bytes = bytes;
        self.workset_items()?;
        self.workset_bytes()?;
        Ok(())
    }

    pub fn workset_items(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.arena_items, self.authority_items, self.clone_items], "drawing-store.mutation-workset-item-overflow")
    }

    pub fn workset_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.arena_bytes, self.authority_bytes, self.clone_bytes], "drawing-store.mutation-workset-byte-overflow")
    }

    pub fn simultaneous_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.source_bytes, self.mutation_bytes, self.workset_bytes()?], "drawing-store.mutation-simultaneous-byte-overflow")
    }

    pub fn architectural_maximum_bytes(&self) -> Result<usize, &'static str> {
        DRAWING_MAXIMUM_NESTED_BYTES
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(DRAWING_OWNED_FIELD_BYTES))
            .and_then(|bytes| bytes.checked_add(self.arena_bytes))
            .and_then(|bytes| bytes.checked_add(self.authority_bytes))
            .ok_or("drawing-store.mutation-architectural-byte-overflow")
    }
}

/// 🎯️ The owner text a mutation addresses first: its layer or asset id, or the first target of a selection transform.
pub fn drawing_mutation_target_owner(mutation: &DrawingMutation) -> Option<&DrawingNativeText> {
    match mutation {
        DrawingMutation::ImportImageAsset(value)=>Some(&value.asset_id),
        DrawingMutation::RemoveImageAsset(value)=>Some(&value.asset_id),
        DrawingMutation::SetLayerVisible(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerLocked(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerOpacity(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerFillRule(value) => Some(&value.layer_id),
        DrawingMutation::SetGroupIsolation(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerBlendMode(value) => Some(&value.layer_id),
        DrawingMutation::RenameLayer(value) => Some(&value.layer_id),
        DrawingMutation::UpdateLayerTransform(value) => Some(&value.layer_id),
        DrawingMutation::ReplaceLayerFill(value) => Some(&value.layer_id),
        DrawingMutation::ReplaceLayerStroke(value) => Some(&value.layer_id),
        DrawingMutation::SetLayerBooleanOperation(value) => Some(&value.layer_id),
        DrawingMutation::UpdateLayerTraceParams(value) => Some(&value.layer_id),
        DrawingMutation::CreateLayer(value) => Some(semio_s_artifact_draw_drawing_model::layer_id(&value.layer)),
        DrawingMutation::DuplicateLayer(value) => Some(&value.layer_id),
        DrawingMutation::DeleteLayer(value) => Some(&value.layer_id),
        DrawingMutation::ReorderLayer(value) => Some(&value.layer_id),
        DrawingMutation::UpdatePathGeometry(value) => Some(&value.layer_id),
        DrawingMutation::UpdateText(value) => Some(&value.layer_id),
        DrawingMutation::UpdateImage(value) => Some(&value.layer_id),
        DrawingMutation::SetShapeCoordinate(value) => Some(&value.layer_id),
        DrawingMutation::DragLayers(semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers { targets, .. }) => targets.first(),
        DrawingMutation::DragPathPoints(value) => value.targets.first().map(|target| &target.layer_id),
    }
}
