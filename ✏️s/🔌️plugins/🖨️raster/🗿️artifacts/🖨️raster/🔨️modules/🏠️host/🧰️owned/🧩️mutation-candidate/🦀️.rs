//! 🧩️ raster mutation candidate: the mutation candidate authority with its phase machine and the one-item apply.

use super::*;

pub(super) struct RasterMutationCandidateAuthority {
    pub(super) value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    pub(super) clone: std::mem::ManuallyDrop<Option<Box<RasterSnapshotCloneAuthority>>>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pending_layer: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    pub(super) pending_asset: std::mem::ManuallyDrop<Option<(String, RasterAssetChild)>>,
    pub(super) retirement: RasterRetirementSlot,
    /// 🌉️ The canonical child this apply minted from the payload's real bytes, held between the
    /// mint step and the handoff that moves its materialization onto the pending handle.
    asset_mint: Option<RasterAssetChild>,
    pub(super) asset_field: u8,
    pub(super) locator: Option<RasterLayerLocator>,
    primary: Option<RasterLayerAddress>,
    secondary: Option<RasterLayerAddress>,
    container: Option<RasterLayerAddress>,
    shift_index: usize,
    shift_target: usize,
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
            pending_asset: std::mem::ManuallyDrop::new(None),
            retirement: RasterRetirementSlot::new(),
            asset_mint: None,
            asset_field: 0,
            locator: None,
            primary: None,
            secondary: None,
            container: None,
            shift_index: 0,
            shift_target: 0,
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

    fn replace_string(target: &mut String, source: &String) -> Result<RasterDisplaced, &'static str> {
        let replacement = raster_clone_owned_string(source)?;
        let previous = std::mem::replace(target, replacement);
        Ok(RasterDisplaced::String(previous))
    }

    fn exact_string(value: &str) -> Result<String, &'static str> {
        raster_exact_string_from_parts(&[value.as_bytes()])
    }

    fn pump_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.retirement.is_empty() {
            return Ok(false);
        }
        if cx.should_yield() {
            return Ok(true);
        }
        let step = self.retirement.close(cx.retained_grant()).map_err(|_| "raster-store.mutation-retirement-fault")?;
        cx.consume_retained(step.progress()).map_err(|_| "raster-store.mutation-retirement-receipt")?;
        cx.consume_fuel(1);
        Ok(true)
    }

    pub(super) fn step(&mut self, current: &RasterSnapshot, operation: &RasterMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.pump_retirement(cx)? {
            return Ok(false);
        }
        match self.phase {
            RasterMutationCandidatePhase::Clone => {
                if self.clone.is_none() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.clone = Some(Box::new(RasterSnapshotCloneAuthority::new()));
                    return Ok(false);
                }
                let clone = self.clone.as_mut().ok_or("raster-store.mutation-clone")?;
                if !clone.step(current, cx)? {
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.value = clone.take_value();
                drop(self.clone.take());
                if Self::target(operation).is_some() {
                    self.locator = Some(RasterLayerLocator::new());
                    self.phase = RasterMutationCandidatePhase::LocatePrimary;
                } else {
                    self.phase = RasterMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::LocatePrimary => {
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
            RasterMutationCandidatePhase::LocateSecondary | RasterMutationCandidatePhase::LocateDestination => {
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
            RasterMutationCandidatePhase::PrepareLayer => {
                let RasterMutation::CreateLayer(value) = operation else { return Err("raster-store.mutation-prepare-variant") };
                if self.layer_clone.is_none() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.layer_clone = Some(Box::new(RasterLayerCloneAuthority::new(&value.layer)));
                    return Ok(false);
                }
                let clone = self.layer_clone.as_mut().expect("Raster create layer clone remains retained");
                if !clone.step(&value.layer, cx)? {
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.pending_layer = clone.take();
                drop(self.layer_clone.take());
                self.phase = RasterMutationCandidatePhase::Apply;
                Ok(false)
            }
            RasterMutationCandidatePhase::Apply => {
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
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        self.retirement.put(Self::replace_string(name, &value.new_name)?);
                    }
                    RasterMutation::ChangeLayerLocked(value)=>{
                        if !raster_reserve_unit(cx){return Ok(false);}
                        semio_s_artifact_raster_raster_mutations_layers::change_layer_locked::validate(value,snapshot).map_err(|code| code.as_str())?;
                        let (RasterLayerNode::Pixel {locked,..}|RasterLayerNode::Group {locked,..}|RasterLayerNode::Adjustment {locked,..})=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *locked=value.locked;
                    }
                    RasterMutation::ChangeLayerVisible(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *visible = value.new_visible;
                    }
                    RasterMutation::ChangeLayerOpacity(value) => {
                        if !value.new_opacity.is_finite() || !raster_reserve_unit(cx) {
                            if !value.new_opacity.is_finite() {
                                return Err("raster-store.mutation-opacity-invalid");
                            }
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *opacity = value.new_opacity;
                    }
                    RasterMutation::ChangeLayerBlendMode(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { blend_mode, .. } | RasterLayerNode::Group { blend_mode, .. } | RasterLayerNode::Adjustment { blend_mode, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        self.retirement.put(Self::replace_string(blend_mode, &value.new_blend_mode)?);
                    }
                    RasterMutation::MoveLayer(value) => {
                        if !value.new_x.is_finite() || !value.new_y.is_finite() || !raster_reserve_unit(cx) {
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
                    }
                    RasterMutation::ResizeLayer(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let RasterLayerNode::Pixel { width, height, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                            return Err("raster-store.mutation-resize-target");
                        };
                        *width = Some(value.new_width);
                        *height = Some(value.new_height);
                    }
                    RasterMutation::ChangeLayerAdjustmentParameter(value) => {
                        semio_s_artifact_raster_raster_mutations_layers::change_layer_adjustment_parameter::validate(value,snapshot).map_err(|code| code.as_str())?;
                        let RasterLayerNode::Adjustment {params,..}=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {return Err("raster-store.mutation-adjustment-target");};
                        if value.value.is_some() && params.page_required_for_insert(&value.parameter) {
                            if !raster_reserve_unit(cx) {return Ok(false);}
                            params.admit_one_page()?;return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {return Ok(false);}
                        let replacement=value.value.map(|v|Ok::<_,&'static str>((raster_clone_owned_string(&value.parameter)?,v.literal()))).transpose()?;
                        if let Some(mut entry)=params.remove_entry(&value.parameter) {
                            let (key,old)=entry.take();
                            self.retirement.put(RasterDisplaced::ValueEntry((key,old)));
                        }
                        if let Some((key,number))=replacement {params.insert_pre_admitted(key,number).map_err(|error|error.reason)?;}
                    }
                    RasterMutation::ChangeLayerTransform(value)=>{
                        if !raster_reserve_unit(cx){return Ok(false);}
                        semio_s_artifact_raster_raster_mutations_layers::change_layer_transform::validate(value,snapshot).map_err(|code| code.as_str())?;
                        let (RasterLayerNode::Pixel {transform,..}|RasterLayerNode::Group {transform,..})=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {return Err("raster-store.mutation-transform-target");};
                        *transform=value.transform.clone();
                    }
                    RasterMutation::ChangeLayerMask(value) => {
                        if !raster_reserve_unit(cx) { return Ok(false); }
                        semio_s_artifact_raster_raster_mutations_pixels::change_layer_mask::validate(value, snapshot).map_err(|code| code.as_str())?;
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
                    }
                    RasterMutation::ChangeLayerPixels(value) => {
                        if !raster_reserve_unit(cx) { return Ok(false); }
                        semio_s_artifact_raster_raster_mutations_pixels::change_layer_pixels::validate(value, snapshot).map_err(|code| code.as_str())?;
                        let replacement = value.content.image_key.as_ref().map(raster_clone_owned_string).transpose()?;
                        let RasterLayerNode::Pixel { image_key, width, height, transform, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else { return Err("raster-store.mutation-pixels-target"); };
                        if let Some(previous) = std::mem::replace(image_key, replacement) {
                            self.retirement.put(RasterDisplaced::String(previous));
                        }
                        *width = value.content.width;
                        *height = value.content.height;
                        if let Some(next) = &value.transform { *transform = next.clone(); }
                    }
                    RasterMutation::ChangeLayerAdjustmentKind(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let RasterLayerNode::Adjustment { adjustment_kind, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                            return Err("raster-store.mutation-adjustment-target");
                        };
                        self.retirement.put(Self::replace_string(adjustment_kind, &value.new_adjustment_kind)?);
                    }
                    RasterMutation::AddLayerAsset(value) => {
                        if value.asset.schema.capacity() > RASTER_OWNED_FIELD_BYTES || value.asset.frames.iter().map(|frame| frame.rgba8.len()).sum::<usize>() > RASTER_MAXIMUM_NESTED_BYTES || value.asset.icc.as_ref().is_some_and(|icc| icc.len() > RASTER_MAXIMUM_NESTED_BYTES) || value.asset.metadata.iter().any(|entry| entry.key.capacity().max(entry.value.capacity()) > RASTER_OWNED_FIELD_BYTES) {
                            return Err("raster-store.mutation-asset-capacity");
                        }
                        self.asset_field = 0;
                        self.phase = RasterMutationCandidatePhase::PrepareAsset;
                        return Ok(false);
                    }
                    RasterMutation::RemoveLayerAsset(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let mut removed = snapshot.assets.remove_entry(&value.asset_id).ok_or("raster-store.mutation-asset-missing")?;
                        let (key, child) = removed.take();
                        self.retirement.put(RasterDisplaced::AssetEntry((key, child)));
                    }
                    RasterMutation::PaintStroke(_) | RasterMutation::FillRegion(_) | RasterMutation::ApplyFilter(_) | RasterMutation::TransformImage(_) | RasterMutation::FillSelection(_) | RasterMutation::WritePixelRegion(_) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let outcome = Mutation::diff(operation, &*snapshot);
                        let refused = outcome.messages().iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
                        let (diff, _) = outcome.into_parts();
                        let painted = if refused { Err("raster-store.mutation-paint-refused") } else { protocol::apply_diff(&diff, snapshot).map_err(|_| "raster-store.mutation-paint-apply") };
                        protocol::MutationDiff::retire_cold(diff);
                        let previous = std::mem::replace(snapshot, painted?);
                        self.retirement.put(RasterDisplaced::Snapshot(previous));
                    }
                }
                self.phase = RasterMutationCandidatePhase::Drain;
                Ok(false)
            }
            RasterMutationCandidatePhase::PrepareAsset => {
                let RasterMutation::AddLayerAsset(value) = operation else { return Err("raster-store.mutation-asset-variant") };
                match self.asset_field {
                    0 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        // 🌉️ The SAME funnel the native apply uses (`🔺️diff/📝️text`, `io`'s import
                        // bridges): the payload's real bytes are decoded into the composed child's own
                        // content, which both mints the CANONICAL content-addressed child id (a raw
                        // `(mime, data)` digest minted here would disagree with every other route and
                        // break `add-layer-asset`'s inverse) and produces the materialization this
                        // handle must retain — without it the asset pool holds pixel-less handles and
                        // the composite surface renders nothing.
                        self.asset_mint = Some(semio_s_artifact_raster_raster_model::mint_raster_image_child(&value.asset_id, &value.asset));
                    }
                    1 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let key = raster_clone_owned_string(&value.asset_id)?;
                        let child =
                            store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                        *self.pending_asset = Some((key, child));
                    }
                    2 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let child_id = raster_clone_owned_string(&self.asset_mint.as_ref().ok_or("raster-store.mutation-asset-mint")?.child_id)?;
                        self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1.child_id = child_id;
                    }
                    3 => {
                        let length = value.asset_id.len().checked_add(6).ok_or("raster-store.mutation-asset-id-overflow")?;
                        if length > RASTER_OWNED_FIELD_BYTES || !raster_reserve_unit(cx) {
                            if length > RASTER_OWNED_FIELD_BYTES {
                                return Err("raster-store.mutation-asset-id-capacity");
                            }
                            return Ok(false);
                        }
                        let artifact_id = raster_exact_string_from_parts(&[value.asset_id.as_bytes(), b"-image"])?;
                        self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1.target.artifact_id = artifact_id;
                    }
                    4..=6 => {
                        let literal = match self.asset_field {
                            4 => "s.stdio.semio",
                            5 => "v1",
                            _ => "image",
                        };
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let child = &mut self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1;
                        let target = match self.asset_field {
                            4 => &mut child.target.dialect.artifact_kind,
                            5 => &mut child.target.dialect.standard,
                            _ => &mut child.target.dialect.subset,
                        };
                        *target = Self::exact_string(literal)?;
                    }
                    _ => {
                        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-asset-snapshot")?;
                        let pending_key = &self.pending_asset.as_ref().ok_or("raster-store.mutation-asset-owner")?.0;
                        if snapshot.assets.page_required_for_insert(pending_key) {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            snapshot.assets.admit_one_page()?;
                            return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (key, mut child) = self.pending_asset.take().ok_or("raster-store.mutation-asset-owner")?;
                        semio_s_artifact_raster_raster_model::adopt_raster_asset_owner(&self.asset_mint.take().ok_or("raster-store.mutation-asset-mint")?, &mut child);
                        if let Some(slot) = snapshot.assets.get_mut(&value.asset_id) {
                            let previous = std::mem::replace(slot, child);
                            self.retirement.put(RasterDisplaced::AssetEntry((key, previous)));
                        } else {
                            match snapshot.assets.insert_pre_admitted(key, child) {
                                Ok(RasterOwnedMapInsert::Inserted) => {}
                                Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                                    self.retirement.put(RasterDisplaced::AssetEntry(previous.take()));
                                    return Err("raster-store.mutation-duplicate-asset");
                                }
                                Err(rejected) => {
                                    self.retirement.put(RasterDisplaced::AssetEntry((rejected.key, rejected.value)));
                                    return Err(rejected.reason);
                                }
                            }
                        }
                        self.phase = RasterMutationCandidatePhase::Drain;
                        return Ok(false);
                    }
                }
                self.asset_field += 1;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftRemove => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index + 1 < values.len() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index + 1);
                    self.shift_index += 1;
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.pending_layer = values.pop();
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
            RasterMutationCandidatePhase::BeginInsert => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.begin_insert()?;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftInsert => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index > self.shift_target {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index - 1);
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

    pub(super) fn take(&mut self) -> Option<RasterSnapshot> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if let Some(clone) = self.layer_clone.as_ref() {
            return raster_nested(clone.close_demands(body)?);
        }
        if let Some(clone) = self.clone.as_ref() {
            return raster_nested(clone.close_demands(body)?);
        }
        if self.pending_layer.is_some() || self.pending_asset.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        if self.asset_mint.is_some() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterAssetChild>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.phase = RasterMutationCandidatePhase::Closing;
        if self.retirement.is_empty() {
            if let Some(clone) = self.layer_clone.as_mut() {
                let demand = raster_nested(clone.close_demands(grant.maximum_copy_bytes)?)?;
                if grant.maximum_depth < demand.depth {
                    return Err(invariant("Raster candidate layer clone exceeds admitted depth"));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let step = clone.close_step(raster_child_grant(grant))?;
                if clone.terminal_is_empty() {
                    drop(self.layer_clone.take());
                }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(clone) = self.clone.as_mut() {
                let demand = raster_nested(clone.close_demands(grant.maximum_copy_bytes)?)?;
                if grant.maximum_depth < demand.depth {
                    return Err(invariant("Raster candidate snapshot clone exceeds admitted depth"));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let step = clone.close_step(raster_child_grant(grant))?;
                if clone.terminal_is_empty() {
                    drop(self.clone.take());
                }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if self.asset_mint.is_some() {
                if grant.maximum_copy_bytes < size_of::<Option<RasterAssetChild>>() {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                drop(self.asset_mint.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<RasterAssetChild>>(), ..empty }));
            }
            if let Some(layer) = self.pending_layer.take() {
                self.retirement.put(RasterDisplaced::Layer(layer));
            } else if let Some(entry) = self.pending_asset.take() {
                self.retirement.put(RasterDisplaced::AssetEntry(entry));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Snapshot(value));
            }
        }
        let step = self.retirement.close(grant)?;
        self.terminal = self.terminal_is_empty();
        Ok(raster_finish(step, self.terminal))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.clone.is_none() && self.layer_clone.is_none() && self.pending_layer.is_none() && self.pending_asset.is_none() && self.retirement.is_empty() && self.asset_mint.is_none()
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
    PrepareAsset,
    Drain,
    Complete,
    Closing,
}

/// 🧮 The interactive document lane's clone-free apply of ONE mutation over the live base — the
/// same `RasterMutationCandidateAuthority` the archive-load initializer replays edits with, driven
/// under a locally minted `StepContext` whose fuel is the store grant. `RasterStorePreparation`
/// (`✏️editor/🦀️.rs`) used to go through `Mutation::diff` + `RasterDiff::apply`, which `Clone`s the
/// base and every carried layer and refuses a populated asset map outright — so the moment the demo's
/// brighten adjustment (populated `params`) reached the store, the batch faulted and the framework's
/// close path dropped the un-begun `create-layer` into `RasterOwnedMap`'s Drop guard (react boots of
/// ticket 26/09/05/RASTER-PLUGIN-END-TO-END, 2026-09-16).
pub struct RasterOneItemApply {
    pub(super) candidate: std::mem::ManuallyDrop<Option<RasterMutationCandidateAuthority>>,
    pub(super) preview_sequence: u64,
}

impl RasterOneItemApply {
    pub fn new() -> Self {
        Self { candidate: std::mem::ManuallyDrop::new(Some(RasterMutationCandidateAuthority::new())), preview_sequence: 0 }
    }

    /// ▶️ Spends up to `fuel` units under the supplied retained grant; the first value is the post
    /// snapshot once the candidate has produced it, and the second is the exact physical receipt.
    pub fn advance(&mut self, base: &RasterSnapshot, operation: &RasterMutation, operation_id: semio_framework_job::OperationId, generation: semio_framework_job::Generation, fuel: u64, retained: RetainedCloneGrant) -> Result<(Option<RasterSnapshot>, RetainedCloneProgress), ValueError> {
        let candidate = self.candidate.as_mut().ok_or_else(|| invariant("raster-store.one-item-apply-candidate-absent"))?;
        let budget = semio_framework_job::StepBudget::new(fuel.max(1), u64::MAX, retained);
        let mut progress = RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(operation_id, generation, budget, semio_framework_job::CancelToken::root_now(), raster_frozen_now_us, &mut self.preview_sequence, &mut progress);
        let mut spins = fuel.saturating_mul(4).saturating_add(16);
        loop {
            match candidate.step(base, operation, &mut cx) {
                Ok(true) => {
                    let post = candidate.take();
                    drop(self.candidate.take());
                    let receipt = cx.retained_progress();
                    return Ok((post, receipt));
                }
                Ok(false) if cx.should_yield() || spins == 0 => return Ok((None, cx.retained_progress())),
                Ok(false) => spins -= 1,
                Err(code) => return Err(invariant(code).with_retained_progress(cx.retained_progress())),
            }
        }
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        self.candidate.as_ref().map_or(Ok(Default::default()), |candidate| candidate.close_demands(body))
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(candidate) = self.candidate.as_mut() else { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())) };
        let step = candidate.close_step(grant)?;
        if candidate.terminal_is_empty() {
            drop(self.candidate.take());
        }
        Ok(step)
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
