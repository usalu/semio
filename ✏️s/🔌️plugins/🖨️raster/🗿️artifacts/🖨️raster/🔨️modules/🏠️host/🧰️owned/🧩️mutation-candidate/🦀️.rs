//! 🧩️ raster mutation candidate: the mutation candidate authority with its phase machine and the one-item apply.

use super::*;
use semio_s_artifact_raster_raster_model::RasterTransform;
#[path="📜️demands/🦀️.rs"]mod demands;
#[path="↩️inverse/🦀️.rs"]
mod inverse;

pub(super) struct RasterMutationCandidateAuthority {
    pub(super) value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    pub(super) clone: std::mem::ManuallyDrop<Option<Box<RasterSnapshotCloneAuthority>>>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pending_layer: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    pub(super) retirement: RasterRetirementSlot,
    pub(super) locator: Option<RasterLayerLocator>,
    primary: Option<RasterLayerAddress>,
    secondary: Option<RasterLayerAddress>,
    container: Option<RasterLayerAddress>,
    shift_index: usize,
    shift_target: usize,
    pixel:std::mem::ManuallyDrop<Option<Box<RasterRetainedMutationWork>>>,
    pixel_ready:bool,
    inverse:std::mem::ManuallyDrop<Option<Vec<RasterMutation>>>,
    prepare_inverse:bool,
    inverse_ready:bool,
    pub(super) phase: RasterMutationCandidatePhase,
    pub(super) terminal: bool,
}

impl RasterMutationCandidateAuthority {
    pub(super) fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            layer_clone: std::mem::ManuallyDrop::new(None),
            pending_layer: std::mem::ManuallyDrop::new(None),
            retirement: RasterRetirementSlot::new(),
            locator: None,
            primary: None,
            secondary: None,
            container: None,
            shift_index: 0,
            shift_target: 0,
            pixel:std::mem::ManuallyDrop::new(None),
            pixel_ready:false,
            inverse:std::mem::ManuallyDrop::new(None),
            prepare_inverse:false,
            inverse_ready:false,
            phase: RasterMutationCandidatePhase::Clone,
            terminal: false,
        }
    }

    pub(super) fn target(operation: &RasterMutation) -> Option<&str> {
        match operation {
            RasterMutation::CreateLayer(value) => Some(RasterLayerLocator::node_id(&value.layer)),
            RasterMutation::DeleteLayer(value) => Some(&value.layer_id),
            RasterMutation::ReorderLayers(value) => Some(&value.layer_id),
            RasterMutation::RenameLayer(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerLocked(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerVisible(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerOpacity(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerBlendMode(value) => Some(&value.layer_id),
            RasterMutation::MoveLayer(value) => Some(&value.layer_id),
            RasterMutation::ResizeLayer(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerPixels(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerMask(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerTransform(value)=>Some(&value.layer_id),
            RasterMutation::ChangeLayerAdjustmentParameter(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerAdjustmentKind(value) => Some(&value.layer_id),
            RasterMutation::PaintStroke(value) => Some(&value.layer_id),
            RasterMutation::FillRegion(value) => Some(&value.layer_id),
            RasterMutation::ApplyFilter(value) => Some(&value.layer_id),
            RasterMutation::TransformImage(value) => Some(&value.layer_id),
            RasterMutation::FillSelection(value) => Some(&value.layer_id),
            RasterMutation::WritePixelRegion(value) => Some(&value.layer_id),
            RasterMutation::AddLayerAsset(_) | RasterMutation::RemoveLayerAsset(_) => None,
        }
    }

    pub(super) fn parent(operation: &RasterMutation) -> Option<&str> {
        match operation {
            RasterMutation::CreateLayer(value) => value.parent_id.as_deref(),
            RasterMutation::ReorderLayers(value) => value.parent_id.as_deref(),
            _ => None,
        }
    }

    fn prepare_insert(&mut self, parent: Option<RasterLayerAddress>, index: usize) {
        self.container = parent;
        self.shift_target = index;
        self.phase = RasterMutationCandidatePhase::BeginInsert;
    }

    fn begin_insert(&mut self) -> Result<(), &'static str> {
        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-insert-snapshot")?;
        let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-insert-parent")?;
        if self.shift_target > values.len() || values.len() >= values.capacity() {
            return Err("raster-store.mutation-insert-capacity");
        }
        values.push(self.pending_layer.take().ok_or("raster-store.mutation-insert-owner")?);
        self.shift_index = values.len() - 1;
        self.phase = RasterMutationCandidatePhase::ShiftInsert;
        Ok(())
    }

    fn begin_remove(&mut self, address: RasterLayerAddress) {
        self.container = address.parent();
        self.shift_index = address.index();
        self.phase = RasterMutationCandidatePhase::ShiftRemove;
    }

    fn replace_string(target: &mut String, source: &String,cx:&mut dyn RasterWork) -> Result<Option<RasterDisplaced>, &'static str> {
        let copy=2*size_of::<String>()+source.len();if !admit_clone(cx,copy,source.len(),0){return Ok(None)}
        let replacement = raster_clone_owned_string(source)?;
        let capacity=replacement.capacity();
        let previous = std::mem::replace(target, replacement);
        record_clone(cx,copy,capacity,0)?;
        Ok(Some(RasterDisplaced::String(previous)))
    }

    fn exact_string(value: &str) -> Result<String, &'static str> {
        raster_exact_string_from_parts(&[value.as_bytes()])
    }

    fn pump_retirement(&mut self, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        if self.retirement.is_empty() {
            return Ok(false);
        }
        if cx.should_yield() {
            return Ok(true);
        }
        let turn = self.retirement.close(RetainedCloneGrant{maximum_items:1,..cx.retained_grant()});cx.consume_fuel(turn.work);let step=match turn.result{Ok(step)=>step,Err(error)=>{cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.mutation-retirement-error-receipt")?;return Err("raster-store.mutation-retirement-fault");}};
        cx.consume_retained(step.progress()).map_err(|_| "raster-store.mutation-retirement-receipt")?;
        Ok(true)
    }

    pub(super) fn step(&mut self, current: &RasterSnapshot, operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.pump_retirement(cx)? {
            return Ok(false);
        }
        match self.phase {
            RasterMutationCandidatePhase::Clone => self.step_clone(current, operation, cx),
            RasterMutationCandidatePhase::LocatePrimary => self.step_locate_primary(operation, cx),
            RasterMutationCandidatePhase::LocateSecondary | RasterMutationCandidatePhase::LocateDestination => self.step_locate_secondary(operation, cx),
            RasterMutationCandidatePhase::PrepareLayer => self.step_prepare_layer(operation, cx),
            RasterMutationCandidatePhase::Apply => self.step_apply(current, operation, cx),
            RasterMutationCandidatePhase::ShiftRemove => self.step_shift_remove(operation, cx),
            RasterMutationCandidatePhase::BeginInsert => {
                if !admit_clone(cx,2*size_of::<RasterLayerNode>(),0,0) {
                    return Ok(false);
                }
                self.begin_insert()?;
                record_clone(cx,2*size_of::<RasterLayerNode>(),0,0)?;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftInsert => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index > self.shift_target {
                    if !admit_clone(cx,3*size_of::<RasterLayerNode>(),0,0) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index - 1);
                    record_clone(cx,3*size_of::<RasterLayerNode>(),0,0)?;
                    self.shift_index -= 1;
                    return Ok(false);
                }
                self.phase = RasterMutationCandidatePhase::Drain;
                Ok(false)
            }
            RasterMutationCandidatePhase::Drain => {
                if self.retirement.is_empty() {
                    self.phase = RasterMutationCandidatePhase::Complete;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::Complete => {
                self.terminal = true;
                Ok(true)
            }
            RasterMutationCandidatePhase::Closing => Ok(false),
        }
    }

    /// 🧱️ Clones the candidate snapshot.
    fn step_clone(&mut self, current: &RasterSnapshot, operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        if self.clone.is_none() {
            if !admit_clone(cx,size_of::<RasterSnapshotCloneAuthority>(),size_of::<RasterSnapshotCloneAuthority>(),0) {
                return Ok(false);
            }
            *self.clone = Some(Box::new(RasterSnapshotCloneAuthority::new()));
            record_clone(cx,size_of::<RasterSnapshotCloneAuthority>(),size_of::<RasterSnapshotCloneAuthority>(),0)?;
            return Ok(false);
        }
        let clone = self.clone.as_mut().ok_or("raster-store.mutation-clone")?;
        if !clone.terminal{clone.step(current,cx)?;return Ok(false);}
        if !admit_clone(cx,size_of::<RasterSnapshot>(),0,size_of::<RasterSnapshotCloneAuthority>()) {
            return Ok(false);
        }
        *self.value = clone.take_value();
        drop(self.clone.take());
        record_clone(cx,size_of::<RasterSnapshot>(),0,size_of::<RasterSnapshotCloneAuthority>())?;
        if Self::target(operation).is_some() {
            self.locator = Some(RasterLayerLocator::new());
            self.phase = RasterMutationCandidatePhase::LocatePrimary;
        } else {
            self.phase = RasterMutationCandidatePhase::Apply;
        }
        Ok(false)
    }

    /// 🪵️ Locates the primary layer of the mutation.
    fn step_locate_primary(&mut self, operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        let snapshot = self.value.as_ref().ok_or("raster-store.mutation-locate-snapshot")?;
        let locator = self.locator.as_mut().ok_or("raster-store.mutation-locator")?;
        if !locator.step(snapshot, Self::target(operation).ok_or("raster-store.mutation-target")?, cx)? {
            return Ok(false);
        }
        self.primary = locator.found;
        self.locator = None;
        if matches!(operation, RasterMutation::CreateLayer(_)) {
            if self.primary.is_some() {
                return Err("raster-store.mutation-duplicate-layer");
            }
            if Self::parent(operation).is_some() {
                self.locator = Some(RasterLayerLocator::new());
                self.phase = RasterMutationCandidatePhase::LocateSecondary;
            } else {
                self.phase = RasterMutationCandidatePhase::PrepareLayer;
            }
        } else if self.primary.is_none() {
            return Err("raster-store.mutation-target-missing");
        } else {
            self.phase = RasterMutationCandidatePhase::Apply;
        }
        Ok(false)
    }

    /// 🔩️ Locates the secondary or destination layer of the mutation.
    fn step_locate_secondary(&mut self, operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        let snapshot = self.value.as_ref().ok_or("raster-store.mutation-parent-snapshot")?;
        let locator = self.locator.as_mut().ok_or("raster-store.mutation-parent-locator")?;
        if !locator.step(snapshot, Self::parent(operation).ok_or("raster-store.mutation-parent")?, cx)? {
            return Ok(false);
        }
        self.secondary = locator.found;
        self.locator = None;
        let parent = self.secondary.ok_or("raster-store.mutation-parent-missing")?;
        if !matches!(RasterLayerLocator::node_at(snapshot, parent), Some(RasterLayerNode::Group { .. })) {
            return Err("raster-store.mutation-parent-not-group");
        }
        if self.phase == RasterMutationCandidatePhase::LocateSecondary {
            self.phase = RasterMutationCandidatePhase::PrepareLayer;
        } else {
            let index = match operation {
                RasterMutation::ReorderLayers(value) => value.index,
                _ => return Err("raster-store.mutation-destination-variant"),
            };
            self.prepare_insert(Some(parent), index);
        }
        Ok(false)
    }

    /// ⚙️ Prepares the layer the mutation carries.
    fn step_prepare_layer(&mut self, operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        let RasterMutation::CreateLayer(value) = operation else { return Err("raster-store.mutation-prepare-variant") };
        if self.layer_clone.is_none() {
            if !admit_clone(cx,size_of::<RasterLayerCloneAuthority>(),size_of::<RasterLayerCloneAuthority>(),0) {
                return Ok(false);
            }
            *self.layer_clone = Some(Box::new(RasterLayerCloneAuthority::new(&value.layer)));
            record_clone(cx,size_of::<RasterLayerCloneAuthority>(),size_of::<RasterLayerCloneAuthority>(),0)?;
            return Ok(false);
        }
        let clone = self.layer_clone.as_mut().expect("Raster create layer clone remains retained");
        if !clone.terminal{clone.step(&value.layer,cx)?;return Ok(false);}
        if !admit_clone(cx,size_of::<RasterLayerNode>(),0,size_of::<RasterLayerCloneAuthority>()) {
            return Ok(false);
        }
        *self.pending_layer = clone.take();
        drop(self.layer_clone.take());
        record_clone(cx,size_of::<RasterLayerNode>(),0,size_of::<RasterLayerCloneAuthority>())?;
        self.phase = RasterMutationCandidatePhase::Apply;
        Ok(false)
    }

    /// 🧲️ Applies the mutation to the candidate snapshot.
    fn step_apply(&mut self, current:&RasterSnapshot,operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        if pixel_target(operation).is_some()||matches!(operation,RasterMutation::AddLayerAsset(_)|RasterMutation::RemoveLayerAsset(_)){return self.step_pixel(current,operation,cx);}
        if self.prepare_inverse&&!self.inverse_ready{return self.step_inverse(current,operation,cx);}
        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-apply-snapshot")?;
        match operation {
            RasterMutation::CreateLayer(value) => {
                self.prepare_insert(self.secondary, value.index);
                return Ok(false);
            }
            RasterMutation::DeleteLayer(_) | RasterMutation::ReorderLayers(_) => {
                self.begin_remove(self.primary.ok_or("raster-store.mutation-remove-address")?);
                return Ok(false);
            }
            RasterMutation::RenameLayer(value) => {
                let (RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. }) =
                    RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                let Some(previous)=Self::replace_string(name,&value.new_name,cx)?else{return Ok(false)};self.retirement.put(previous);
            }
            RasterMutation::ChangeLayerLocked(value)=>{
                if !admit_clone(cx,size_of::<bool>(),0,0){return Ok(false);}
                semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_locked::validate_target(value,RasterLayerLocator::node_at(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?).map_err(|code| code.as_str())?;
                let (RasterLayerNode::Pixel {locked,..}|RasterLayerNode::Group {locked,..}|RasterLayerNode::Adjustment {locked,..})=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                *locked=value.locked;
                record_clone(cx,size_of::<bool>(),0,0)?;
            }
            RasterMutation::ChangeLayerVisible(value) => {
                if !admit_clone(cx,size_of::<bool>(),0,0) {
                    return Ok(false);
                }
                let (RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. }) =
                    RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                *visible = value.new_visible;
                record_clone(cx,size_of::<bool>(),0,0)?;
            }
            RasterMutation::ChangeLayerOpacity(value) => {
                if !value.new_opacity.is_finite() || !admit_clone(cx,size_of::<f32>(),0,0) {
                    if !value.new_opacity.is_finite() {
                        return Err("raster-store.mutation-opacity-invalid");
                    }
                    return Ok(false);
                }
                let (RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. }) =
                    RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                *opacity = value.new_opacity;
                record_clone(cx,size_of::<f32>(),0,0)?;
            }
            RasterMutation::ChangeLayerBlendMode(value) => {
                let (RasterLayerNode::Pixel { blend_mode, .. } | RasterLayerNode::Group { blend_mode, .. } | RasterLayerNode::Adjustment { blend_mode, .. }) =
                    RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                let Some(previous)=Self::replace_string(blend_mode,&value.new_blend_mode,cx)?else{return Ok(false)};self.retirement.put(previous);
            }
            RasterMutation::MoveLayer(value) => {
                if !value.new_x.is_finite() || !value.new_y.is_finite() || !admit_clone(cx,2*size_of::<f64>(),0,0) {
                    if !value.new_x.is_finite() || !value.new_y.is_finite() {
                        return Err("raster-store.mutation-transform-invalid");
                    }
                    return Ok(false);
                }
                match RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? {
                    RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } => {
                        transform.x = value.new_x;
                        transform.y = value.new_y;
                    }
                    RasterLayerNode::Adjustment { .. } => return Err("raster-store.mutation-transform-target"),
                }
                record_clone(cx,2*size_of::<f64>(),0,0)?;
            }
            RasterMutation::ResizeLayer(value) => {
                if !admit_clone(cx,2*size_of::<Option<u32>>(),0,0) {
                    return Ok(false);
                }
                let RasterLayerNode::Pixel { width, height, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                    return Err("raster-store.mutation-resize-target");
                };
                *width = Some(value.new_width);
                *height = Some(value.new_height);
                record_clone(cx,2*size_of::<Option<u32>>(),0,0)?;
            }
            RasterMutation::ChangeLayerAdjustmentParameter(value) => {
                semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_adjustment_parameter::validate_target(value,RasterLayerLocator::node_at(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?).map_err(|code| code.as_str())?;
                let RasterLayerNode::Adjustment {params,..}=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {return Err("raster-store.mutation-adjustment-target");};
                if value.value.is_some() && params.page_required_for_insert(&value.parameter) {
                    let page=RasterOwnedMap::<semio_framework_value::DslValue>::conservative_page_credit_bytes();if !admit_clone(cx,page,page,0) {return Ok(false);}
                    params.admit_one_page()?;record_clone(cx,page,page,0)?;return Ok(false);
                }
                let capacity=if value.value.is_some(){value.parameter.len()}else{0};let copy=3*size_of::<(String,semio_framework_value::DslValue)>()+capacity;if !admit_clone(cx,copy,capacity,0) {return Ok(false);}
                let replacement=value.value.map(|v|Ok::<_,&'static str>((raster_clone_owned_string(&value.parameter)?,v.literal()))).transpose()?;
                record_clone(cx,copy,capacity,0)?;if let Some((key,number))=replacement {match params.insert_pre_admitted(key,number){Ok(RasterOwnedMapInsert::Inserted)=>{},Ok(RasterOwnedMapInsert::Replaced(mut owner))=>{self.retirement.put(RasterDisplaced::ValueEntry(owner.take()));},Err(error)=>{self.retirement.put(RasterDisplaced::ValueEntry((error.key,error.value)));return Err(error.reason)}}}else if let Some(mut entry)=params.remove_entry(&value.parameter){self.retirement.put(RasterDisplaced::ValueEntry(entry.take()));}
            }
            RasterMutation::ChangeLayerTransform(value)=>{
                if !admit_clone(cx,size_of::<RasterTransform>(),0,0){return Ok(false);}
                semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_transform::validate_target(value,RasterLayerLocator::node_at(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?).map_err(|code| code.as_str())?;
                let (RasterLayerNode::Pixel {transform,..}|RasterLayerNode::Group {transform,..})=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {return Err("raster-store.mutation-transform-target");};
                *transform=value.transform.clone();
                record_clone(cx,size_of::<RasterTransform>(),0,0)?;
            }
            RasterMutation::ChangeLayerMask(value) => {
                let capacity=value.mask.as_ref().and_then(|mask|mask.image_key.as_ref()).map_or(0,String::len);let copy=2*size_of::<Option<semio_s_artifact_raster_raster_model::RasterLayerMask>>()+capacity;if !admit_clone(cx,copy,capacity,0) { return Ok(false); }
                semio_s_artifact_raster_raster_mutations_pixels::change_layer_mask::validate_target(value,snapshot,RasterLayerLocator::node_at(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?).map_err(|code| code.as_str())?;
                let replacement = value.mask.as_ref().map(|mask| {
                    Ok::<_, &'static str>(semio_s_artifact_raster_raster_model::RasterLayerMask {
                        enabled: mask.enabled, linked: mask.linked, invert: mask.invert,
                        width: mask.width, height: mask.height, transform: mask.transform.clone(),
                        image_key: mask.image_key.as_ref().map(raster_clone_owned_string).transpose()?,
                    })
                }).transpose()?;
                let (RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else { return Err("raster-store.mutation-mask-target"); };
                if let Some(previous) = std::mem::replace(mask, replacement).and_then(|mask| mask.image_key) {
                    self.retirement.put(RasterDisplaced::String(previous));
                }
                record_clone(cx,copy,capacity,0)?;
            }
            RasterMutation::ChangeLayerPixels(value) => {
                let capacity=value.content.image_key.as_ref().map_or(0,String::len);let copy=2*size_of::<semio_s_artifact_raster_raster_model::RasterPixelContent>()+size_of::<RasterTransform>()+capacity;if !admit_clone(cx,copy,capacity,0) { return Ok(false); }
                semio_s_artifact_raster_raster_mutations_pixels::change_layer_pixels::validate_target(value,snapshot,RasterLayerLocator::node_at(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?).map_err(|code| code.as_str())?;
                let replacement = value.content.image_key.as_ref().map(raster_clone_owned_string).transpose()?;
                let RasterLayerNode::Pixel { image_key, width, height, transform, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else { return Err("raster-store.mutation-pixels-target"); };
                if let Some(previous) = std::mem::replace(image_key, replacement) {
                    self.retirement.put(RasterDisplaced::String(previous));
                }
                *width = value.content.width;
                *height = value.content.height;
                if let Some(next) = &value.transform { *transform = next.clone(); }
                record_clone(cx,copy,capacity,0)?;
            }
            RasterMutation::ChangeLayerAdjustmentKind(value) => {
                let RasterLayerNode::Adjustment { adjustment_kind, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                    return Err("raster-store.mutation-adjustment-target");
                };
                let Some(previous)=Self::replace_string(adjustment_kind,&value.new_adjustment_kind,cx)?else{return Ok(false)};self.retirement.put(previous);
            }
            RasterMutation::AddLayerAsset(_) | RasterMutation::RemoveLayerAsset(_)=>return Err("raster-store.asset-publication-dispatch"),
            RasterMutation::PaintStroke(_) | RasterMutation::FillRegion(_) | RasterMutation::ApplyFilter(_) | RasterMutation::TransformImage(_) | RasterMutation::FillSelection(_) | RasterMutation::WritePixelRegion(_) => {
                return Err("raster-store.pixel-publication-dispatch");
            }
        }
        self.phase = RasterMutationCandidatePhase::Drain;
        Ok(false)
    }

    fn step_pixel(&mut self,current:&RasterSnapshot,operation:&RasterMutation,cx:&mut dyn RasterWork)->Result<bool,&'static str>{
        if self.pixel.is_none(){
            let bytes=size_of::<RasterRetainedMutationWork>();let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()};
            if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0||raster_short(cx.retained_grant(),demand){return Ok(false);}cx.consume_fuel(1);*self.pixel=Some(Box::new(RasterRetainedMutationWork::new(operation,self.prepare_inverse)));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()}).map_err(|_|"raster-store.pixel-publication-birth-receipt")?;return Ok(false);
        }
        let pixel=self.pixel.as_mut().ok_or("raster-store.pixel-publication-owner")?;
        if !self.pixel_ready{
            if !pixel.step(current,self.value.as_mut().ok_or("raster-store.pixel-candidate")?,operation,self.primary,&mut self.retirement,cx)?{return Ok(false);}
            let bytes=size_of::<bool>()+if self.prepare_inverse{2*size_of::<Option<Vec<RasterMutation>>>()}else{0};let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false);}cx.consume_fuel(1);
            if self.prepare_inverse{*self.inverse=pixel.take_inverse();}
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).map_err(|_|"raster-store.pixel-inverse-handoff-receipt")?;
            self.pixel_ready=true;return Ok(false);
        }
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false);}
        if pixel.terminal_is_empty(){let bytes=size_of::<RasterRetainedMutationWork>();let demand=semio_framework_value::RetirementDemand{release_bytes:bytes,depth:1,..Default::default()};if raster_short(cx.retained_grant(),demand){return Ok(false);}cx.consume_fuel(1);drop(self.pixel.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}).map_err(|_|"raster-store.pixel-frame-release")?;self.phase=RasterMutationCandidatePhase::Drain;return Ok(false);}
        let demand=pixel.close_demands(cx.retained_grant().maximum_copy_bytes).map_err(|_|"raster-store.pixel-close-demand")?;if cx.retained_grant().maximum_depth<demand.depth||raster_short(cx.retained_grant(),demand){return Ok(false);}let turn=pixel.close_step(RetainedCloneGrant{maximum_items:1,..cx.retained_grant()});cx.consume_fuel(turn.work);let step=match turn.result{Ok(step)=>step,Err(error)=>{cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.pixel-close-error-receipt")?;return Err("raster-store.pixel-close");}};cx.consume_retained(step.progress()).map_err(|_|"raster-store.pixel-close-receipt")?;
        Ok(false)
    }


    /// 🔧️ Shifts the layers behind a removed one.
    fn step_shift_remove(&mut self, operation: &RasterMutation, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
        let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
        if self.shift_index + 1 < values.len() {
            if !admit_clone(cx,3*size_of::<RasterLayerNode>(),0,0) {
                return Ok(false);
            }
            values.swap(self.shift_index, self.shift_index + 1);
            record_clone(cx,3*size_of::<RasterLayerNode>(),0,0)?;
            self.shift_index += 1;
            return Ok(false);
        }
        if !admit_clone(cx,2*size_of::<RasterLayerNode>(),0,0) {
            return Ok(false);
        }
        *self.pending_layer = values.pop();
        record_clone(cx,2*size_of::<RasterLayerNode>(),0,0)?;
        if matches!(operation, RasterMutation::DeleteLayer(_)) {
            let layer = self.pending_layer.take().ok_or("raster-store.mutation-delete-owner")?;
            self.retirement.put(RasterDisplaced::Layer(layer));
            self.phase = RasterMutationCandidatePhase::Drain;
        } else if let RasterMutation::ReorderLayers(value) = operation {
            self.secondary = None;
            if value.parent_id.is_some() {
                self.locator = Some(RasterLayerLocator::new());
                self.phase = RasterMutationCandidatePhase::LocateDestination;
            } else {
                self.prepare_insert(None, value.index);
            }
        } else {
            return Err("raster-store.mutation-remove-variant");
        }
        Ok(false)
    }

    pub(super) fn take(&mut self) -> Option<RasterSnapshot> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if let Some(pixel)=self.pixel.as_ref(){if pixel.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of::<RasterRetainedMutationWork>(),depth:1,..Default::default()});}return pixel.close_demands(body);}
        if let Some(clone) = self.layer_clone.as_ref() {
            if clone.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of::<RasterLayerCloneAuthority>(),depth:1,..Default::default()});}
            return raster_nested(clone.close_demands(body)?);
        }
        if let Some(clone) = self.clone.as_ref() {
            if clone.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of::<RasterSnapshotCloneAuthority>(),depth:1,..Default::default()});}
            return raster_nested(clone.close_demands(body)?);
        }
        if self.pending_layer.is_some() || self.value.is_some() || self.inverse.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        Ok(Default::default())
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        let mut work=0;let result=(||{
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.phase = RasterMutationCandidatePhase::Closing;
        if self.retirement.is_empty() {
            if let Some(pixel)=self.pixel.as_mut(){if pixel.terminal_is_empty(){let bytes=size_of::<RasterRetainedMutationWork>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}drop(self.pixel.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..empty}));}let demand=pixel.close_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth||raster_short(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}let turn=pixel.close_step(grant);work=turn.work;return turn.result;}
            if let Some(clone) = self.layer_clone.as_mut() {
                if clone.terminal_is_empty(){let bytes=size_of::<RasterLayerCloneAuthority>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}drop(self.layer_clone.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..empty}));}
                let demand = raster_nested(clone.close_demands(grant.maximum_copy_bytes)?)?;
                if grant.maximum_depth < demand.depth {
                    return Err(invariant("Raster candidate layer clone exceeds admitted depth"));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let turn=clone.close_step(raster_child_grant(grant));work=turn.work;let step=turn.result?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(clone) = self.clone.as_mut() {
                if clone.terminal_is_empty(){let bytes=size_of::<RasterSnapshotCloneAuthority>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}drop(self.clone.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..empty}));}
                let demand = raster_nested(clone.close_demands(grant.maximum_copy_bytes)?)?;
                if grant.maximum_depth < demand.depth {
                    return Err(invariant("Raster candidate snapshot clone exceeds admitted depth"));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let turn=clone.close_step(raster_child_grant(grant));work=turn.work;let step=turn.result?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(layer) = self.pending_layer.take() {
                self.retirement.put(RasterDisplaced::Layer(layer));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Snapshot(value));
            } else if let Some(inverse)=self.inverse.take(){
                self.retirement.put(RasterDisplaced::Mutations(inverse));
            }
        }
        let turn=self.retirement.close(grant);work=turn.work;let step=turn.result?;
        self.terminal = self.terminal_is_empty();
        Ok(raster_finish(step, self.terminal))
        })();semio_framework_value::RetainedWorkOutcome{result,work}
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.clone.is_none() && self.layer_clone.is_none() && self.pending_layer.is_none() && self.retirement.is_empty() && self.pixel.is_none() && self.inverse.is_none()
    }
}

impl Drop for RasterMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster mutation candidate reached Drop before exact handoff or retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterMutationCandidatePhase {
    Clone,
    LocatePrimary,
    LocateSecondary,
    PrepareLayer,
    Apply,
    ShiftRemove,
    LocateDestination,
    BeginInsert,
    ShiftInsert,
    Drain,
    Complete,
    Closing,
}

/// 🧮 A private semantic candidate borrows the original caller's physical loan and primitive budget.
pub struct RasterOneItemApply {
    pub(super) candidate: std::mem::ManuallyDrop<Option<RasterMutationCandidateAuthority>>,
    binding:Option<(RasterApplyIdentity,usize,usize)>,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum RasterApplyIdentity{Publication(semio_framework_job::OperationId,semio_framework_job::Generation),Replay(store::os_store::ArtifactReplayPreparationContext,u64,Option<u64>)}

struct RasterGrantedWork<'probe>{grant:RetainedCloneGrant,fuel:u64,spent:RetainedCloneProgress,work:u64,stopped:bool,stop:&'probe mut dyn FnMut()->bool}
impl RasterWork for RasterGrantedWork<'_>{
    fn should_yield(&mut self)->bool{self.stopped|=(self.stop)();self.stopped||self.work>=self.fuel||self.spent.copied_items>=self.grant.maximum_items}
    fn is_cancelled(&self)->bool{self.stopped}
    fn retained_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.spent.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.spent.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.spent.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.spent.released_bytes),maximum_depth:self.grant.maximum_depth}}
    fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{self.spent=self.spent.checked_add(progress)?;if !self.spent.fits(self.grant){return Err(invariant("raster-store.raw-work-receipt-exceeded").with_retained_progress(self.spent));}Ok(())}
    fn consume_fuel(&mut self,work:u64){self.work=self.work.saturating_add(work);}
    fn fuel_remaining(&self)->u64{self.fuel.saturating_sub(self.work)}
}

/// 📬️ The private post-state and its already prepared semantic inverse transfer together.
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterPreparedMutation {pub snapshot:RasterSnapshot,pub inverse:Option<Vec<RasterMutation>>}

impl RasterOneItemApply {
    /// 👁️ Visual candidates use the same semantic apply without allocating a history inverse.
    pub fn for_preview()->Self{Self{candidate:std::mem::ManuallyDrop::new(Some(RasterMutationCandidateAuthority::new())),binding:None}}
    pub fn new() -> Self {
        let mut candidate=RasterMutationCandidateAuthority::new();candidate.prepare_inverse=true;
        Self { candidate: std::mem::ManuallyDrop::new(Some(candidate)), binding:None }
    }

    /// ▶️ Spends up to `fuel` units under the supplied retained grant; the first value is the post
    /// snapshot once the candidate has produced it, and the second is the exact physical receipt.
    pub fn advance(&mut self,base:&RasterSnapshot,operation:&RasterMutation,operation_id:semio_framework_job::OperationId,generation:semio_framework_job::Generation,fuel:u64,grant:RetainedCloneGrant)->Result<(Option<RasterPreparedMutation>,RetainedCloneProgress),ValueError>{
        self.advance_controlled(base,operation,operation_id,generation,fuel,grant,||false).result
    }

    /// 🎟️ Raw workers spend only the supplied primitive budget and physical loan; the caller retains all source authority.
    pub fn advance_controlled(&mut self,base:&RasterSnapshot,operation:&RasterMutation,operation_id:semio_framework_job::OperationId,generation:semio_framework_job::Generation,fuel:u64,grant:RetainedCloneGrant,mut should_stop:impl FnMut()->bool)->semio_framework_value::RetainedWorkOutcome<(Option<RasterPreparedMutation>,RetainedCloneProgress)>{
        self.advance_original(base,operation,RasterApplyIdentity::Publication(operation_id,generation),fuel,grant,&mut should_stop)
    }

    /// 📽️ Replay borrows its actual decoder/retirement recipient and keeps the genuine replay context.
    pub fn advance_replay_controlled(&mut self,base:&RasterSnapshot,operation:&RasterMutation,context:store::os_store::ArtifactReplayPreparationContext,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->semio_framework_value::RetainedWorkOutcome<(Option<RasterPreparedMutation>,RetainedCloneProgress)>{
        let mut budget=256;loop{match control.prepare_work(budget){Ok(true)=>break,Ok(false)if budget>1=>budget=(budget/2).max(1),Ok(false)=>return semio_framework_value::RetainedWorkOutcome{result:Ok((None,Default::default())),work:0},Err(error)=>return semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}}}
        let identity=RasterApplyIdentity::Replay(context,control.ledger_identity(),control.retirement_recipient_identity());let mut stopped=None;let turn=self.advance_original(base,operation,identity,budget,grant,&mut||match control.checkpoint(){Ok(())=>false,Err(error)=>{stopped=Some(error);true}});let receipt=turn.result.as_ref().map(|(_,progress)|*progress).unwrap_or_else(|error|error.retained_progress());let result=match control.commit_work(turn.work){Err(error)=>Err(error.with_retained_progress(receipt)),Ok(())=>if let Some(error)=stopped{Err(error.with_retained_progress(receipt))}else{turn.result}};semio_framework_value::RetainedWorkOutcome{result,work:turn.work}
    }

    fn binding_demands()->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(RasterApplyIdentity,usize,usize)>>(),depth:1,..Default::default()}}

    fn advance_original(&mut self,base:&RasterSnapshot,operation:&RasterMutation,identity:RasterApplyIdentity,fuel:u64,grant:RetainedCloneGrant,should_stop:&mut dyn FnMut()->bool)->semio_framework_value::RetainedWorkOutcome<(Option<RasterPreparedMutation>,RetainedCloneProgress)>{
        let mut work=RasterGrantedWork{grant,fuel,spent:Default::default(),work:0,stopped:false,stop:should_stop};
        let result=(||{
            if work.should_yield()||grant.maximum_depth==0{return Ok((None,Default::default()));}
            let binding=(identity,base as *const RasterSnapshot as usize,operation as *const RasterMutation as usize);
            if self.binding.is_some_and(|original|original!=binding){return Err(invariant("raster-store.one-item-original-source-changed"));}
            if self.binding.is_none(){let bytes=Self::binding_demands().copy_bytes;if grant.maximum_copy_bytes<bytes{return Ok((None,Default::default()));}work.consume_fuel(1);self.binding=Some(binding);work.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok((None,work.spent));}
            let candidate=self.candidate.as_mut().ok_or_else(||invariant("raster-store.one-item-apply-candidate-absent"))?;
            let mut spins=fuel.saturating_mul(4).saturating_add(16);
            loop{
                let before_work=work.work;let before_spent=work.spent;
                match candidate.step(base,operation,&mut work){
                    Ok(true)=>{let post=candidate.take();let inverse=candidate.inverse.take();drop(self.candidate.take());return Ok((post.map(|snapshot|RasterPreparedMutation{snapshot,inverse}),work.spent));},
                    Ok(false)if work.work==before_work&&work.spent==before_spent||work.should_yield()||spins==0=>return Ok((None,work.spent)),
                    Ok(false)=>spins-=1,
                    Err(code)=>return Err(invariant(code)),
                }
            }
        })().map_err(|error:ValueError|error.with_retained_progress(work.spent));
        semio_framework_value::RetainedWorkOutcome{result,work:work.work}
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        self.candidate.as_ref().map_or(Ok(Default::default()), |candidate| candidate.close_demands(body))
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        let Some(candidate) = self.candidate.as_mut() else { return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),work:0} };
        let turn = candidate.close_step(grant);
        if candidate.terminal_is_empty() {
            drop(self.candidate.take());
        }
        turn
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.candidate.is_none()
    }
}

impl Drop for RasterOneItemApply {
    fn drop(&mut self) {
        assert!((self.candidate.is_none()) || std::thread::panicking(), "Raster one-item apply reached Drop before its candidate was closed");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.candidate) };
    }
}
