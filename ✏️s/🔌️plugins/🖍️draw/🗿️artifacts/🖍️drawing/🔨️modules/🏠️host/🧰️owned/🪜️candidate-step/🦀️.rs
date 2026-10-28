//! 🪜️ Drawing candidate step: the `step` of the mutation candidate authority that applies one drawing mutation to the candidate snapshot in bounded turns.

use super::*;

//#region 🔖️RetainedStoreInitialization
impl DrawingMutationCandidateAuthority {
    pub fn step(&mut self, mut access: DrawingSourceAccess<'_, '_>, mutation_ref: RetainedCloneRef<'_, DrawingMutation>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let mutation = mutation_ref.get();
        if (cx.operation() != self.operation || cx.generation() != self.generation) && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return self.fail("drawing-store.mutation-candidate-stale-authority");
        }
        if cx.is_cancelled() && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            self.fault = Some("drawing-store.mutation-candidate-cancelled");
            self.phase = DrawingMutationCandidatePhase::Retire;
            return Err("drawing-store.mutation-candidate-cancelled");
        }
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.validate(access.read())?;
        }
        if let DrawingMutation::SetArtboard(value)=mutation{if self.phase==DrawingMutationCandidatePhase::PreflightSource{
            let copy=2*size_of::<Option<semio_s_artifact_draw_drawing_model::DrawingArtboard>>()+2*size_of::<DrawingMutationOverlayPatch>()+size_of::<DrawingMutationCandidatePhase>();
            let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}).map_err(|_|"drawing-store.artboard-original-receipt")?;cx.consume_fuel(copy.max(1)as u64);
            if !semio_s_artifact_draw_drawing_mutations::artboard::valid(&value.artboard){return self.fail("drawing-store.artboard-invalid")}
            let source=access.write()?;source.artboard=semio_s_artifact_draw_drawing_mutations::artboard::copy(&value.artboard);let mut overlay=DrawingMutationOverlayPatch::bind(source);overlay.commit(source)?;self.overlay=Some(overlay);self.phase=DrawingMutationCandidatePhase::Complete;return Ok(false);
        }}
        match self.phase {
            DrawingMutationCandidatePhase::PreflightSource => {
                let source = access.read();
                let bounds = self.preflight_source.as_mut().ok_or("drawing-store.mutation-source-preflight-missing")?;
                if bounds.step(source, cx)? {
                    self.phase = DrawingMutationCandidatePhase::PreflightMutation;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::PreflightMutation => self.step_preflight_mutation(cx, mutation),
            DrawingMutationCandidatePhase::LocateCloneSource => {
                let source = access.read();
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-clone-locator-missing")?;
                if !locator.step(source, drawing_mutation_target_owner(mutation).ok_or("drawing-store.mutation-target-missing")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if self.primary.is_none() {
                    return Err("drawing-store.mutation-target-missing");
                }
                self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                Ok(false)
            }
            DrawingMutationCandidatePhase::BindOverlay => {
                let source = access.read();
                self.clone_work = None;
                self.overlay = Some(DrawingMutationOverlayPatch::bind(source));
                if matches!(mutation, DrawingMutation::ImportImageAsset(_) | DrawingMutation::RemoveImageAsset(_) | DrawingMutation::DuplicateLayer(_) | DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_)) {
                    self.phase = DrawingMutationCandidatePhase::Apply;
                } else {
                    self.locator = Some(DrawingLayerLocator::new());
                    self.phase = DrawingMutationCandidatePhase::LocatePrimary;
                }
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocatePrimary => self.step_locate_primary(&mut access, cx, mutation),
            DrawingMutationCandidatePhase::LocateSecondary => {
                let source = access.read();
                let target = Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?;
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-parent-locator")?;
                if !locator.step(source, target, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::Apply;
                Ok(false)
            }
            DrawingMutationCandidatePhase::PrepareOwnedValue => self.step_prepare_owned_value(&mut access, mutation_ref, cx, mutation),
            DrawingMutationCandidatePhase::PlanOwnedValue => {
                let pending = self.pending_layer.as_ref().ok_or("drawing-store.mutation-clone-work-owner-missing")?;
                let work = self.clone_work.as_mut().ok_or("drawing-store.mutation-clone-work-missing")?;
                if !work.step(pending, cx)? {
                    return Ok(false);
                }
                let totals = work.totals().ok_or("drawing-store.mutation-clone-work-false-terminal")?;
                self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                self.preflight_digest = None;
                self.phase = DrawingMutationCandidatePhase::BindOverlay;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Apply => self.step_apply(&mut access, cx, mutation),
            DrawingMutationCandidatePhase::RebuildSource => self.step_rebuild_source(&mut access, cx, mutation),
            DrawingMutationCandidatePhase::LocateDestination => {
                let source = access.write()?;
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-destination-locator")?;
                if !locator.step(source, Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildDestination => self.step_rebuild_destination(&mut access, cx, mutation),
            DrawingMutationCandidatePhase::Complete => {
                let source = access.write()?;
                if self.pump_retirement(cx)? {
                    return Ok(false);
                }
                if let Some(value) = self.pending_layer.take() {
                    *self.retire_pending = Some(DrawingRetirementOwner::Layer(value));
                    return Ok(false);
                }
                if self.return_arena_owner()? != Some(true) {
                    return Ok(false);
                }
                if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                }
                self.terminal = true;
                Ok(true)
            }
            DrawingMutationCandidatePhase::Retire | DrawingMutationCandidatePhase::Fault => Err(self.fault.unwrap_or("drawing-store.mutation-candidate-fault")),
        }
    }

    /// 🧱️ Digests the mutation and admits its arena capacities.
    fn step_preflight_mutation(&mut self, cx: &mut semio_framework_job::StepContext<'_>, mutation: &DrawingMutation) -> Result<bool, &'static str> {
        let digest = self.preflight_mutation.as_mut().ok_or("drawing-store.mutation-preflight-missing")?;
        if !digest.step(mutation, self.preflight_digest.as_mut().ok_or("drawing-store.mutation-preflight-digest")?, cx)? {
            return Ok(false);
        }
        let source_credit = self.preflight_source.as_ref().and_then(DrawingSnapshotBoundsAuthority::totals).ok_or("drawing-store.mutation-source-preflight-incomplete")?;
        let mutation_credit = digest.totals().ok_or("drawing-store.mutation-preflight-incomplete")?;
        let reverse_slots = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?.capacity();
        let output_slots = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?.capacity();
        let overlay_pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
        let overlay_slots = overlay_pages.capacity();
        let required_overlay_slots = usize::from(matches!(
            mutation,
            DrawingMutation::DuplicateLayer(_)
        ));
        if required_overlay_slots > overlay_slots {
            return Err("drawing-store.mutation-overlay-slot-capacity");
        }
        let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("drawing-store.mutation-duplicate-id-arena-missing")?;
        let (arena_items, arena_bytes) = DrawingMutationArenaOwner::retained_totals(
            self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?,
            self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?,
            overlay_pages,
            duplicate_id,
        )?;
        let container_slots = reverse_slots.checked_add(output_slots).ok_or("drawing-store.mutation-container-credit-overflow")?;
        self.workset = Some(DrawingMutationWorksetPlan::admit(source_credit, mutation_credit, arena_items, arena_bytes, container_slots, size_of::<Self>())?);
        drop(self.preflight_mutation.take());
        self.preflight_source = None;
        self.preflight_digest = None;
        if matches!(mutation, DrawingMutation::DuplicateLayer(_)) {
            self.locator = Some(DrawingLayerLocator::new());
            self.phase = DrawingMutationCandidatePhase::LocateCloneSource;
        } else if matches!(mutation, DrawingMutation::ImportImageAsset(_) | DrawingMutation::RemoveImageAsset(_) | DrawingMutation::CreateLayer(_) | DrawingMutation::ReplaceLayerFill(_) | DrawingMutation::ReplaceLayerStroke(_) | DrawingMutation::UpdatePathGeometry(_) | DrawingMutation::UpdateText(_) | DrawingMutation::UpdateImage(_) | DrawingMutation::SetLayerBlendMode(_) | DrawingMutation::RenameLayer(_) | DrawingMutation::SetLayerBooleanOperation(_)) {
            self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
        } else {
            self.phase = DrawingMutationCandidatePhase::BindOverlay;
        }
        Ok(false)
    }

    /// 🪵️ Locates the primary layer node of the mutation.
    fn step_locate_primary(
        &mut self,
        access: &mut DrawingSourceAccess<'_, '_>,
        cx: &mut semio_framework_job::StepContext<'_>,
        mutation: &DrawingMutation,
    ) -> Result<bool, &'static str> {
        let source = access.read();
        let locator = self.locator.as_mut().ok_or("drawing-store.mutation-locator-missing")?;
        if !locator.step(source, drawing_mutation_target_owner(mutation).ok_or("drawing-store.mutation-target-missing")?, cx)? {
            return Ok(false);
        }
        self.primary = locator.found();
        self.locator = None;
        if matches!(mutation, DrawingMutation::CreateLayer(_)) {
            if self.primary.is_some() {
                return Err("drawing-store.mutation-duplicate-layer");
            }
            if Self::parent(mutation).is_some() {
                self.locator = Some(DrawingLayerLocator::new());
                self.phase = DrawingMutationCandidatePhase::LocateSecondary;
            } else {
                self.phase = DrawingMutationCandidatePhase::Apply;
            }
        } else if self.primary.is_none() {
            return Err("drawing-store.mutation-target-missing");
        } else {
            self.phase = DrawingMutationCandidatePhase::Apply;
        }
        Ok(false)
    }

    /// 🔩️ Prepares the owned value the mutation carries.
    fn step_prepare_owned_value(
        &mut self,
        access: &mut DrawingSourceAccess<'_, '_>,
        mutation_ref: RetainedCloneRef<'_, DrawingMutation>,
        cx: &mut semio_framework_job::StepContext<'_>,
        mutation: &DrawingMutation,
    ) -> Result<bool, &'static str> {
        let source = access.read();
        match mutation {
            DrawingMutation::RemoveImageAsset(value)=>{
                if value.asset_id.is_empty()||value.asset_id.len()>4096||source.assets.get(&value.asset_id).is_none(){return Err("drawing-store.asset-missing");}
                let locator=self.locator.get_or_insert_with(DrawingLayerLocator::asset_reference);
                if !locator.step(source,&value.asset_id,cx)?{return Ok(false);}
                if locator.found().is_some(){return Err("drawing-store.asset-still-referenced");}
                self.locator=None;
            }
            DrawingMutation::ImportImageAsset(value)=>{
                if value.asset_id.is_empty() || value.asset_id.len()>4096 || value.asset.width==0 || value.asset.height==0 || (value.asset.width as usize).checked_mul(value.asset.height as usize)!=Some(value.asset.samples.len()) || source.assets.get(&value.asset_id).is_some(){return Err("drawing-store.asset-invalid");}
                if !self.prepare_native_text(mutation_ref.project(1, drawing_mutation_asset_id), cx)?{return Ok(false);}
                if self.asset_clone.is_none(){self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(value.asset.samples.len()+1,value.asset.samples.allocated_bytes().checked_add(size_of::<DrawingImageAsset>()).ok_or("drawing-store.asset-size-overflow")?)?;}
                let clone=self.asset_clone.get_or_insert_with(DrawingNativeCloneAuthority::new);
                if !clone.step(mutation_ref.project(2, drawing_mutation_asset), cx)?{return Ok(false);}
            }
            DrawingMutation::CreateLayer(_value) => {
                if self.layer_clone.is_none() {
                    *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new()));
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                let clone = self.layer_clone.as_mut().expect("Drawing create layer clone remains retained");
                if !clone.step(mutation_ref.project(3, drawing_mutation_layer), cx)? {
                    return Ok(false);
                }
                *self.pending_layer = clone.take();
                drop(self.layer_clone.take());
                self.clone_work = Some(DrawingLayerCloneWorkAuthority::new());
            }
            DrawingMutation::DuplicateLayer(value) => {
                let duplicate_source = DrawingLayerLocator::node_at(source, self.primary.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-duplicate-source")?;
                if self.pending_layer.is_none() {
                    if self.layer_clone.is_none() {
                        *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new()));
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    let clone = self.layer_clone.as_mut().expect("Drawing duplicate layer clone remains retained");
                    let base = access.base().ok_or("drawing-store.duplicate-source-custody-missing")?;
                    let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                    if !clone.step(base.project(4, move |snapshot| DrawingLayerLocator::node_at(snapshot, address).expect("sealed Drawing duplicate source remains addressable")), cx)? {
                        return Ok(false);
                    }
                    *self.pending_layer = clone.take();
                    drop(self.layer_clone.take());
                    let name_owner = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?.pop().ok_or("drawing-store.duplicate-name-owner-missing")?;
                    self.duplicate_rewrite = Some(DrawingDuplicateRewriteAuthority::new(self.duplicate_id_owner.take().ok_or("drawing-store.duplicate-id-owner-missing")?, name_owner));
                    return Ok(false);
                }
                if !self.duplicate_rewrite.as_mut().ok_or("drawing-store.duplicate-rewrite-missing")?.step(self.pending_layer.as_mut().ok_or("drawing-store.duplicate-owner-missing")?, duplicate_source, source, &value.identities, cx)? {
                    return Ok(false);
                }
                let pages = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                if pages.len() >= pages.capacity() {
                    return Err("drawing-store.duplicate-name-owner-return-saturated");
                }
                let (id_owner, name_owner) = self.duplicate_rewrite.as_mut().and_then(DrawingDuplicateRewriteAuthority::take_owners).ok_or("drawing-store.duplicate-owner-false-terminal")?;
                *self.duplicate_id_owner = Some(id_owner);
                pages.push(name_owner);
                drop(self.duplicate_rewrite.take());
                self.clone_work = Some(DrawingLayerCloneWorkAuthority::new());
            }
            DrawingMutation::UpdateImage(value)=>{
                if ![value.width,value.height].iter().all(|number|number.is_finite()&&*number>0.0){return Err("drawing-store.image-dimension-invalid");}
                if !self.prepare_native_text(mutation_ref.project(5, drawing_mutation_image_key), cx)?{return Ok(false);}
            }
            DrawingMutation::UpdateText(value) => {
                if !value.size.is_finite() || value.size <= 0.0 { return Err("drawing-store.text-size-invalid"); }
                if !self.prepare_native_text(mutation_ref.project(6, drawing_mutation_content), cx)? { return Ok(false); }
            }
            DrawingMutation::SetLayerBlendMode(_value) => { if !self.prepare_native_text(mutation_ref.project(7, drawing_mutation_blend_mode), cx)? { return Ok(false); } }
            DrawingMutation::RenameLayer(_value) => { if !self.prepare_native_text(mutation_ref.project(8, drawing_mutation_new_name), cx)? { return Ok(false); } }
            DrawingMutation::SetLayerBooleanOperation(_value) => { if !self.prepare_native_text(mutation_ref.project(9, drawing_mutation_boolean_operation), cx)? { return Ok(false); } }
            DrawingMutation::UpdatePathGeometry(_value) => {
                if self.segments_clone.is_none() {
                    *self.segments_clone = Some(DrawingSegmentsCloneAuthority::new());
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                let clone = self.segments_clone.as_mut().ok_or("drawing-store.path-clone-missing")?;
                if !clone.step(mutation_ref.project(10, drawing_mutation_segments), cx)? { return Ok(false); }
                let (items, bytes) = DrawingLayerCloneWorkAuthority::vector(clone.value.as_ref().ok_or("drawing-store.path-clone-value")?)?;
                self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(items, bytes)?;
            }
            DrawingMutation::ReplaceLayerFill(value) => {
                if value.fill.is_some() {
                    if self.fill_clone.is_none() {
                        *self.fill_clone = Some(DrawingFillCloneAuthority::new());
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    if !self.fill_clone.as_mut().expect("Drawing fill clone remains retained").step(mutation_ref.project(11, drawing_mutation_fill), cx)? {
                        return Ok(false);
                    }
                    let cloned = self.fill_clone.as_ref().and_then(|clone| clone.value.as_ref()).ok_or("drawing-store.fill-clone-work-missing")?;
                    let totals = drawing_fill_clone_work_totals(cloned)?;
                    self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                }
            }
            DrawingMutation::ReplaceLayerStroke(value) => {
                if value.stroke.is_some() {
                    if self.stroke_clone.is_none() {
                        *self.stroke_clone = Some(DrawingStrokeCloneAuthority::new());
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    if !self.stroke_clone.as_mut().expect("Drawing stroke clone remains retained").step(mutation_ref.project(12, drawing_mutation_stroke), cx)? {
                        return Ok(false);
                    }
                    let cloned = self.stroke_clone.as_ref().and_then(|clone| clone.value.as_ref()).ok_or("drawing-store.stroke-clone-work-missing")?;
                    let totals = drawing_stroke_clone_work_totals(cloned)?;
                    self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                }
            }
            _ => {}
        }
        self.phase = if self.clone_work.is_some() { DrawingMutationCandidatePhase::PlanOwnedValue } else { DrawingMutationCandidatePhase::BindOverlay };
        Ok(false)
    }

    /// ⚙️ Applies the mutation to the candidate snapshot.
    fn step_apply(
        &mut self,
        access: &mut DrawingSourceAccess<'_, '_>,
        cx: &mut semio_framework_job::StepContext<'_>,
        mutation: &DrawingMutation,
    ) -> Result<bool, &'static str> {
        let source = access.write()?;
        match mutation {
            DrawingMutation::SetArtboard(_)=>return Err("drawing-store.artboard-requires-original-scalar-stage"),
            DrawingMutation::ImportImageAsset(_)=>{
                let key=self.text_clone.as_mut().ok_or("drawing-store.asset-key-clone")?.take().ok_or("drawing-store.asset-key-incomplete")?;
                let asset=self.asset_clone.as_mut().ok_or("drawing-store.asset-clone")?.take().ok_or("drawing-store.asset-incomplete")?;
                source.assets.insert(key,asset);drop(self.text_clone.take());
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;self.phase=DrawingMutationCandidatePhase::Complete;cx.consume_fuel(1);return Ok(false);
            }
            DrawingMutation::RemoveImageAsset(value)=>{
                let asset=source.assets.remove(&value.asset_id).ok_or("drawing-store.asset-missing")?;
                *self.retire_pending = Some(DrawingRetirementOwner::Asset(asset));
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;self.phase=DrawingMutationCandidatePhase::Complete;cx.consume_fuel(1);return Ok(false);
            }
            DrawingMutation::CreateLayer(value) => {
                let parent = self.secondary;
                let index = match value.index {
                    Some(index) => index,
                    None => DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len()),
                };
                self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                return Ok(false);
            }
            DrawingMutation::DuplicateLayer(_) => {
                let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                self.start_rebuild(source, address.parent(), None, Some(address.index() + 1), DrawingContainerRebuildRole::Destination)?;
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                return Ok(false);
            }
            DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                self.start_rebuild(source, address.parent(), Some(address.index()), None, DrawingContainerRebuildRole::Source)?;
                self.source_undo = Some(DrawingContainerSourceUndo { parent: address.parent(), index: address.index() });
                self.phase = DrawingMutationCandidatePhase::RebuildSource;
                return Ok(false);
            }
            DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_) => {
                self.apply_selection_transform(source, mutation)?;
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.phase = DrawingMutationCandidatePhase::Complete;
                cx.consume_fuel(1);
                return Ok(false);
            }
            _ => {}
        }
        let address = self.primary;
        match mutation {
            DrawingMutation::SetLayerVisible(value) => {
                semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).visible = value.visible;
            }
            DrawingMutation::SetLayerLocked(value) => {
                semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).locked = value.locked;
            }
            DrawingMutation::SetGroupIsolation(value) => {
                let DrawingLayerNode::Group(group)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {return Err("drawing-store.mutation-isolation-target-not-group");};
                group.isolation=value.isolation;
            }
            DrawingMutation::SetLayerFillRule(value) => {
                semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill_rule=value.fill_rule;
            }
            DrawingMutation::SetLayerOpacity(value) if value.opacity.is_finite() => {
                semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).opacity = value.opacity;
            }
            DrawingMutation::SetLayerOpacity(_) => return Err("drawing-store.mutation-opacity-invalid"),
            DrawingMutation::SetLayerBlendMode(value) => {
                if !semio_s_artifact_draw_drawing_model::DRAWING_BLEND_MODES.iter().any(|allowed| value.blend_mode.eq_str(allowed)) { return Err("drawing-store.mutation-blend-mode-invalid"); }
                self.adopt_native_text(&mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).blend_mode)?;
            }
            DrawingMutation::SetShapeCoordinate(value)=>{
                let DrawingLayerNode::Shape(shape)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else{return Err("drawing-store.shape-target-kind");};
                semio_s_artifact_draw_drawing_model::shape_geometry::set_shape_coordinate(shape,value.field,value.index,value.value)?;
            }
            DrawingMutation::UpdateImage(value)=>{
                if ![value.width,value.height].iter().all(|number|number.is_finite()&&*number>0.0){return Err("drawing-store.image-dimension-invalid");}
                let DrawingLayerNode::Image(image)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else{return Err("drawing-store.image-target-kind");};
                self.adopt_native_text(&mut image.image_key)?;image.width=value.width;image.height=value.height;
            }
            DrawingMutation::UpdateText(value) => {
                if !value.size.is_finite() || value.size <= 0.0 { return Err("drawing-store.text-size-invalid"); }
                let DrawingLayerNode::Text(text) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else { return Err("drawing-store.text-target-kind"); };
                self.adopt_native_text(&mut text.content)?;
                text.size = value.size;
            }
            DrawingMutation::RenameLayer(_) => {
                self.adopt_native_text(&mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).name)?;
            }
            DrawingMutation::UpdateLayerTransform(value)
                if [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation, value.transform.shear].iter().all(|field| field.is_finite()) =>
            {
                semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).transform =
                    semio_s_artifact_draw_drawing_model::DrawingTransform { x: value.transform.x, y: value.transform.y, scale_x: value.transform.scale_x, scale_y: value.transform.scale_y, rotation: value.transform.rotation, shear: value.transform.shear };
            }
            DrawingMutation::UpdateLayerTransform(_) => return Err("drawing-store.mutation-transform-invalid"),
            DrawingMutation::UpdatePathGeometry(_) => {
                let DrawingLayerNode::Path(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else { return Err("drawing-store.path-target-kind"); };
                let replacement = self.segments_clone.as_mut().ok_or("drawing-store.path-clone-missing")?.take().ok_or("drawing-store.path-false-terminal")?;
                let old = std::mem::replace(&mut target.segments, replacement);
                *self.retire_pending = Some(DrawingRetirementOwner::Segments(old));
                drop(self.segments_clone.take());
            }
            DrawingMutation::ReplaceLayerFill(value) => {
                let replacement = match value.fill.as_ref() {
                    Some(_) => Some(self.fill_clone.as_mut().ok_or("drawing-store.fill-clone-missing")?.take().ok_or("drawing-store.fill-false-terminal")?),
                    None => None,
                };
                let old = std::mem::replace(
                    &mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill,
                    replacement,
                );
                if let Some(old) = old {
                    *self.retire_pending = Some(DrawingRetirementOwner::Fill(old));
                }
                if let Some(mut clone) = self.fill_clone.take() {
                    clone.terminal = true;
                    drop(clone);
                }
            }
            DrawingMutation::ReplaceLayerStroke(value) => {
                let replacement = match value.stroke.as_ref() {
                    Some(_) => Some(self.stroke_clone.as_mut().ok_or("drawing-store.stroke-clone-missing")?.take().ok_or("drawing-store.stroke-false-terminal")?),
                    None => None,
                };
                let old = std::mem::replace(
                    &mut semio_s_artifact_draw_drawing_model::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.stroke,
                    replacement,
                );
                if let Some(old) = old {
                    *self.retire_pending = Some(DrawingRetirementOwner::Stroke(old));
                }
                if let Some(mut clone) = self.stroke_clone.take() {
                    clone.terminal = true;
                    drop(clone);
                }
            }
            DrawingMutation::SetLayerBooleanOperation(_value) => {
                let DrawingLayerNode::Boolean(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                    return Err("drawing-store.mutation-boolean-target");
                };
                self.adopt_native_text(&mut target.operation)?;
            }
            DrawingMutation::UpdateLayerTraceParams(value) if value.params.threshold.is_finite() && value.params.simplify_epsilon.is_finite() => {
                let DrawingLayerNode::Trace(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                    return Err("drawing-store.mutation-trace-target");
                };
                target.params = semio_s_artifact_draw_drawing_model::DrawingTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon };
            }
            DrawingMutation::UpdateLayerTraceParams(_) => return Err("drawing-store.mutation-trace-invalid"),
            DrawingMutation::CreateLayer(_) | DrawingMutation::DuplicateLayer(_) | DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                unreachable!("structural Drawing mutations start retained rebuild before scalar mutation")
            }
            DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_) => {
                unreachable!("selection transforms apply their resolved patches before scalar mutation")
            }
            DrawingMutation::ImportImageAsset(_) | DrawingMutation::RemoveImageAsset(_) => {
                unreachable!("image asset mutations commit before scalar mutation")
            }
        }
        self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
        self.phase = DrawingMutationCandidatePhase::Complete;
        cx.consume_fuel(1);
        Ok(false)
    }

    /// 🧲️ Rebuilds the source container index.
    fn step_rebuild_source(
        &mut self,
        access: &mut DrawingSourceAccess<'_, '_>,
        cx: &mut semio_framework_job::StepContext<'_>,
        mutation: &DrawingMutation,
    ) -> Result<bool, &'static str> {
        let source = access.write()?;
        if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
            return Ok(false);
        }
        let parent = self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent();
        let removed = self.finish_rebuild(source, parent)?.ok_or("drawing-store.mutation-removal-missing")?;
        *self.pending_layer = Some(removed);
        if matches!(mutation, DrawingMutation::DeleteLayer(_)) {
            self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
            self.source_undo = None;
            self.phase = DrawingMutationCandidatePhase::Complete;
        } else {
            self.secondary = None;
            if Self::parent(mutation).is_some() {
                self.locator = Some(DrawingLayerLocator::new());
                self.phase = DrawingMutationCandidatePhase::LocateDestination;
            } else {
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
            }
        }
        Ok(false)
    }

    /// 🪛️ Rebuilds the destination container index.
    fn step_rebuild_destination(
        &mut self,
        access: &mut DrawingSourceAccess<'_, '_>,
        cx: &mut semio_framework_job::StepContext<'_>,
        mutation: &DrawingMutation,
    ) -> Result<bool, &'static str> {
        let source = access.write()?;
        if self.rebuild.is_none() {
            let parent = match mutation {
                DrawingMutation::CreateLayer(_) => self.secondary,
                DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                DrawingMutation::ReorderLayer(_) => self.secondary,
                _ => return Err("drawing-store.mutation-destination-variant"),
            };
            let index = match mutation {
                DrawingMutation::CreateLayer(value) => value.index.unwrap_or_else(|| DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len())),
                DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.index() + 1,
                DrawingMutation::ReorderLayer(value) => value.index,
                _ => 0,
            };
            self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
        }
        if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
            return Ok(false);
        }
        let parent = match mutation {
            DrawingMutation::CreateLayer(_) | DrawingMutation::ReorderLayer(_) => self.secondary,
            DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
            _ => None,
        };
        if self.finish_rebuild(source, parent)?.is_some() {
            return Err("drawing-store.mutation-unexpected-removal");
        }
        self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
        self.source_undo = None;
        self.phase = DrawingMutationCandidatePhase::Complete;
        Ok(false)
    }
}
//#endregion 🔖️RetainedStoreInitialization
