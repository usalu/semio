//! 🧬️ raster snapshot clone: the snapshot, dsl value and layer clone authorities that copy a snapshot incrementally.

use super::*;

pub(super) struct RasterSnapshotCloneAuthority {
    pub(super) value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    pub(super) retirement: RasterRetirementSlot,
    pub(super) layer: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pub(super) pending_asset: std::mem::ManuallyDrop<Option<(String, RasterAssetChild)>>,
    pub(super) bounds: RasterSnapshotBoundsAuthority,
    pub(super) asset_key: RasterMapKeyCursor,
    pub(super) phase: u8,
    pub(super) index: usize,
    pub(super) asset_field: u8,
    pub(super) terminal: bool,
}

impl RasterSnapshotCloneAuthority {
    pub(super) fn new() -> Self {
        let value = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() };
        Self {
            value: std::mem::ManuallyDrop::new(Some(value)),
            retirement: RasterRetirementSlot::new(),
            layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            bounds: RasterSnapshotBoundsAuthority::new(),
            asset_key: RasterMapKeyCursor::new(),
            phase: 0,
            index: 0,
            asset_field: 0,
            terminal: false,
        }
    }

    pub(super) fn step(&mut self, source: &RasterSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.phase == 0 {
            if self.bounds.step(source, cx)? {
                self.phase = 1;
            }
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("raster-store.initializer-clone-target")?;
        match self.phase {
            1 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.schema = raster_clone_owned_string(&source.schema)?;
            }
            2 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.id = raster_clone_owned_string(&source.id)?;
            }
            3 => {
                if let Some(title) = source.title.as_ref() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.title = Some(raster_clone_owned_string(title)?);
                } else {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                }
            }
            4 => {
                if self.index == 0 && target.layers.capacity() == 0 {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.layers.try_reserve_exact(source.layers.capacity().saturating_add(1)).map_err(|_| "raster-store.initializer-layer-admission")?;
                    self.bounds.totals.observe_candidate_capacity(source.layers.capacity().saturating_add(1), target.layers.capacity(), size_of::<RasterLayerNode>())?;
                    return Ok(false);
                }
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("raster-store.initializer-layer-source")?, cx)? {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.layers.push(layer.take().ok_or("raster-store.initializer-layer-handoff")?);
                        drop(self.layer.take());
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if let Some(layer) = source.layers.get(self.index) {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.layer = Some(Box::new(RasterLayerCloneAuthority::new(layer)));
                    return Ok(false);
                }
                self.index = 0;
            }
            5 => {
                if self.pending_asset.is_none() {
                    let Some((key, _)) = self.asset_key.next(&source.assets) else {
                        self.phase = 6;
                        return Ok(false);
                    };
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let key = raster_clone_owned_string(key)?;
                    let child =
                        store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                    *self.pending_asset = Some((key, child));
                    self.asset_field = 0;
                    return Ok(false);
                }
                if self.asset_field >= 5 {
                    let (key, _) = self.pending_asset.as_ref().expect("Raster pending asset remains retained");
                    if target.assets.page_required_for_insert(key) {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.assets.admit_one_page()?;
                        return Ok(false);
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let (key, mut child) = self.pending_asset.take().expect("Raster pending asset handoff remains exact");
                    // 🪆 A clone rebuilds every handle string into exact-capacity storage, so the
                    // source child's materialization has to be carried across explicitly — a cloned
                    // snapshot whose assets lost their pixels renders an empty composite.
                    if let Some(source_child) = source.assets.get(&key) {
                        semio_s_artifact_raster_raster_model::adopt_raster_asset_owner(source_child, &mut child);
                    }
                    match target.assets.insert_pre_admitted(key, child) {
                        Ok(RasterOwnedMapInsert::Inserted) => {}
                        Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                            self.retirement.put(RasterDisplaced::AssetEntry(previous.take()));
                            return Err("raster-store.initializer-duplicate-asset");
                        }
                        Err(rejected) => {
                            self.retirement.put(RasterDisplaced::AssetEntry((rejected.key, rejected.value)));
                            return Err(rejected.reason);
                        }
                    }
                    self.asset_key.advance("")?;
                    self.asset_field = 0;
                    return Ok(false);
                }
                let (key, pending) = self.pending_asset.as_mut().expect("Raster pending asset remains exact");
                let source_child = source.assets.get(key).ok_or("raster-store.initializer-asset-source")?;
                let source_value = match self.asset_field {
                    0 => &source_child.child_id,
                    1 => &source_child.target.artifact_id,
                    2 => &source_child.target.dialect.artifact_kind,
                    3 => &source_child.target.dialect.standard,
                    4 => &source_child.target.dialect.subset,
                    _ => unreachable!("Raster asset field cursor is exact"),
                };
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                let target_value = match self.asset_field {
                    0 => &mut pending.child_id,
                    1 => &mut pending.target.artifact_id,
                    2 => &mut pending.target.dialect.artifact_kind,
                    3 => &mut pending.target.dialect.standard,
                    4 => &mut pending.target.dialect.subset,
                    _ => unreachable!("Raster asset field cursor is exact"),
                };
                *target_value = raster_clone_owned_string(source_value)?;
                self.asset_field += 1;
                return Ok(false);
            }
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        }
        self.phase += 1;
        Ok(false)
    }

    pub(super) fn take_value(&mut self) -> Option<RasterSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(layer) = self.layer.as_ref() {
            return raster_nested(layer.close_demands(body)?);
        }
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.pending_asset.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
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
        if let Some(layer) = self.layer.as_mut() {
            let demand = raster_nested(layer.close_demands(grant.maximum_copy_bytes)?)?;
            if grant.maximum_depth < demand.depth {
                return Err(invariant("Raster snapshot clone layer exceeds admitted depth"));
            }
            if raster_short(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let step = layer.close_step(raster_child_grant(grant))?;
            if layer.terminal_is_empty() {
                drop(self.layer.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_empty() {
            if let Some(entry) = self.pending_asset.take() {
                self.retirement.put(RasterDisplaced::AssetEntry(entry));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Snapshot(value));
            }
        }
        let step = self.retirement.close(grant)?;
        Ok(raster_finish(step, self.terminal_is_empty()))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_empty() && self.layer.is_none() && self.pending_asset.is_none()
    }
}

impl Drop for RasterSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

pub(super) fn raster_exact_string_from_parts(parts: &[&[u8]]) -> Result<String, &'static str> {
    let length = parts.iter().try_fold(0usize, |length, part| length.checked_add(part.len())).ok_or("raster-store.clone-string-length-overflow")?;
    if length > RASTER_OWNED_FIELD_BYTES {
        return Err("raster-store.clone-string-capacity");
    }
    let mut bytes = Box::<[u8]>::new_uninit_slice(length);
    let mut offset = 0;
    for part in parts {
        for byte in *part {
            bytes[offset].write(*byte);
            offset += 1;
        }
    }
    let bytes = unsafe { bytes.assume_init() };
    String::from_utf8(Vec::from(bytes)).map_err(|_| "raster-store.clone-string-utf8")
}

pub(super) fn raster_clone_owned_string(source: &String) -> Result<String, &'static str> {
    if source.capacity() > RASTER_OWNED_FIELD_BYTES {
        return Err("raster-store.clone-string-capacity");
    }
    raster_exact_string_from_parts(&[source.as_bytes()])
}

impl RasterDslValueCloneAuthority {
    fn skeleton(source: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
        match source {
            semio_framework_value::DslValue::Null => semio_framework_value::DslValue::Null,
            semio_framework_value::DslValue::Bool(value) => semio_framework_value::DslValue::Bool(*value),
            semio_framework_value::DslValue::Number(value) => semio_framework_value::DslValue::Number(*value),
            semio_framework_value::DslValue::String(_) => semio_framework_value::DslValue::String(String::new()),
            semio_framework_value::DslValue::Bytes(_) => semio_framework_value::DslValue::Bytes(Vec::new()),
            semio_framework_value::DslValue::Array(values) => semio_framework_value::DslValue::Array(Vec::with_capacity(values.capacity())),
            semio_framework_value::DslValue::Object(values) => semio_framework_value::DslValue::Object(Vec::with_capacity(values.capacity())),
        }
    }

    pub(super) fn new(source: &semio_framework_value::DslValue) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(Self::skeleton(source))),
            retirement: RasterRetirementSlot::new(),
            depth: 0,
            path: [0; RASTER_MAXIMUM_NESTED_DEPTH],
            frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH],
            pending_key: std::mem::ManuallyDrop::new(None),
            root_capacity_observed: false,
            terminal: false,
        }
    }

    fn target_at_mut<'a>(root: &'a mut semio_framework_value::DslValue, path: &[usize]) -> Option<&'a mut semio_framework_value::DslValue> {
        let Some((head, tail)) = path.split_first() else { return Some(root) };
        match root {
            semio_framework_value::DslValue::Array(values) => Self::target_at_mut(values.get_mut(*head)?, tail),
            semio_framework_value::DslValue::Object(values) => Self::target_at_mut(&mut values.get_mut(*head)?.1, tail),
            _ => None,
        }
    }

    fn observe_container_capacity(source: &semio_framework_value::DslValue, target: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals) -> Result<(), &'static str> {
        match (source, target) {
            (semio_framework_value::DslValue::Array(source), semio_framework_value::DslValue::Array(target)) => totals.observe_candidate_capacity(source.capacity(), target.capacity(), size_of::<semio_framework_value::DslValue>()),
            (semio_framework_value::DslValue::Object(source), semio_framework_value::DslValue::Object(target)) => totals.observe_candidate_capacity(source.capacity(), target.capacity(), size_of::<(String, semio_framework_value::DslValue)>()),
            _ => Ok(()),
        }
    }

    pub(super) fn step(&mut self, source_root: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            Self::observe_container_capacity(source_root, self.value.as_ref().ok_or("raster-store.clone-value-observed-target")?, totals)?;
            self.root_capacity_observed = true;
            return Ok(false);
        }
        let source = RasterDslValueBoundsAuthority::value_at(source_root, &self.path[..self.depth]).ok_or("raster-store.clone-value-source")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("raster-store.clone-value-target")?, &self.path[..self.depth]).ok_or("raster-store.clone-value-target-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            match (source, target) {
                (semio_framework_value::DslValue::String(source), semio_framework_value::DslValue::String(target)) => {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *target = raster_clone_owned_string(source)?;
                }
                (semio_framework_value::DslValue::Bytes(source), semio_framework_value::DslValue::Bytes(target)) => {
                    if source.len() > RASTER_MAXIMUM_NESTED_BYTES {
                        return Err("raster-store.clone-value-bytes-capacity");
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.extend_from_slice(source);
                }
                _ if !raster_reserve_unit(cx) => return Ok(false),
                _ => {}
            }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        match (source, target) {
            (semio_framework_value::DslValue::Array(source), semio_framework_value::DslValue::Array(target)) if frame.child < source.len() => {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.push(Self::skeleton(&source[frame.child]));
                Self::observe_container_capacity(&source[frame.child], target.last().ok_or("raster-store.clone-value-child-target")?, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            (semio_framework_value::DslValue::Object(source), semio_framework_value::DslValue::Object(target)) if frame.child < source.len() => {
                if frame.phase == 1 {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.pending_key = Some(raster_clone_owned_string(&source[frame.child].0)?);
                    self.frames[self.depth].phase = 2;
                    return Ok(false);
                }
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.push((self.pending_key.take().ok_or("raster-store.clone-value-key")?, Self::skeleton(&source[frame.child].1)));
                Self::observe_container_capacity(&source[frame.child].1, &target.last().ok_or("raster-store.clone-value-object-target")?.1, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.frames[self.depth].phase = 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            _ => {}
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.terminal = true;
        } else {
            self.depth -= 1;
        }
        Ok(self.terminal)
    }

    pub(super) fn take(&mut self) -> Option<semio_framework_value::DslValue> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.pending_key.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        Ok(Default::default())
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if self.retirement.is_empty() {
            if let Some(key) = self.pending_key.take() {
                self.retirement.put(RasterDisplaced::String(key));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Value(value));
            }
        }
        let step = self.retirement.close(grant)?;
        Ok(raster_finish(step, self.terminal_is_empty()))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_empty() && self.pending_key.is_none()
    }
}

impl Drop for RasterDslValueCloneAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster value clone reached Drop before exact handoff or retirement");
    }
}

pub(super) struct RasterLayerCloneAuthority {
    pub(super) value: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    pub(super) retirement: RasterRetirementSlot,
    pub(super) bounds: RasterLayerBoundsAuthority,
    pub(super) totals: RasterOwnerTotals,
    admitted: bool,
    root_capacity_observed: bool,
    pub(super) depth: usize,
    pub(super) path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    pub(super) frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pub(super) parameter_key: RasterMapKeyCursor,
    pending_parameter_key: std::mem::ManuallyDrop<Option<String>>,
    pub(super) parameter_value: std::mem::ManuallyDrop<Option<Box<RasterDslValueCloneAuthority>>>,
    pub(super) terminal: bool,
}

impl RasterLayerCloneAuthority {
    pub(super) fn mask(source: &Option<semio_s_artifact_raster_raster_model::RasterLayerMask>) -> Option<semio_s_artifact_raster_raster_model::RasterLayerMask> {
        source.as_ref().map(|value| semio_s_artifact_raster_raster_model::RasterLayerMask { enabled: value.enabled, linked: value.linked, invert: value.invert, width: value.width, height: value.height, image_key:None, transform:value.transform.clone() })
    }

    fn skeleton(source: &RasterLayerNode) -> RasterLayerNode {
        match source {
            RasterLayerNode::Pixel { visible, locked, opacity, transform, mask, width, height, .. } => RasterLayerNode::Pixel {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                locked: *locked,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: transform.clone(),
                mask: Self::mask(mask),
                width: *width,
                height: *height,
                image_key: None,
            },
            RasterLayerNode::Group { visible, locked, opacity, transform, mask, children, .. } => RasterLayerNode::Group {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                locked: *locked,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: transform.clone(),
                mask: Self::mask(mask),
                children: Vec::with_capacity(children.capacity().saturating_add(1)),
            },
            RasterLayerNode::Adjustment { visible, locked, opacity, transform, .. } => RasterLayerNode::Adjustment {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                locked: *locked,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: transform.clone(),
                adjustment_kind: String::new(),
                params: RasterOwnedMap::new(),
            },
        }
    }

    pub(super) fn new(_source: &RasterLayerNode) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
            retirement: RasterRetirementSlot::new(),
            bounds: RasterLayerBoundsAuthority::new(),
            totals: RasterOwnerTotals::new(),
            admitted: false,
            root_capacity_observed: false,
            depth: 0,
            path: [0; RASTER_MAXIMUM_NESTED_DEPTH],
            frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH],
            parameter_key: RasterMapKeyCursor::new(),
            pending_parameter_key: std::mem::ManuallyDrop::new(None),
            parameter_value: std::mem::ManuallyDrop::new(None),
            terminal: false,
        }
    }

    fn target_at_mut<'a>(root: &'a mut RasterLayerNode, path: &[usize]) -> Option<&'a mut RasterLayerNode> {
        let Some((head, tail)) = path.split_first() else { return Some(root) };
        let RasterLayerNode::Group { children, .. } = root else { return None };
        Self::target_at_mut(children.get_mut(*head)?, tail)
    }

    pub(super) fn strings<'a>(source: &'a RasterLayerNode, target: &'a mut RasterLayerNode) -> [(&'a String, &'a mut String); 3] {
        match (source, target) {
            (RasterLayerNode::Pixel { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Pixel { id: target_id, name: target_name, blend_mode: target_blend, .. })
            | (RasterLayerNode::Group { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Group { id: target_id, name: target_name, blend_mode: target_blend, .. })
            | (RasterLayerNode::Adjustment { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Adjustment { id: target_id, name: target_name, blend_mode: target_blend, .. }) => {
                [(source_id, target_id), (source_name, target_name), (source_blend, target_blend)]
            }
            _ => unreachable!("Raster layer clone source and target variants remain exact"),
        }
    }

    pub(super) fn step(&mut self, source_root: &RasterLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.admitted {
            if self.bounds.step(source_root, &mut self.totals, cx)? {
                self.admitted = true;
            }
            return Ok(false);
        }
        if self.value.is_none() {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            *self.value = Some(Self::skeleton(source_root));
            return Ok(false);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source_root, self.value.as_ref().ok_or("raster-store.clone-layer-observed-target")?) {
                self.totals.observe_candidate_capacity(source.capacity().saturating_add(1), target.capacity(), size_of::<RasterLayerNode>())?;
            }
            self.root_capacity_observed = true;
            return Ok(false);
        }
        let source = RasterLayerBoundsAuthority::layer_at(source_root, &self.path[..self.depth]).ok_or("raster-store.clone-layer-source")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("raster-store.clone-layer-target")?, &self.path[..self.depth]).ok_or("raster-store.clone-layer-target-path")?;
        let frame = self.frames[self.depth];
        match frame.phase {
            0..=2 => {
                let (source, target) = &mut Self::strings(source, target)[frame.phase as usize];
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                **target = raster_clone_owned_string(source)?;
                self.frames[self.depth].phase += 1;
            }
            3 => {
                match (source, target) {
                    (RasterLayerNode::Pixel { image_key: source, .. }, RasterLayerNode::Pixel { image_key: target, .. }) => {
                        if let Some(source) = source {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            *target = Some(raster_clone_owned_string(source)?);
                        } else if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                    }
                    (RasterLayerNode::Adjustment { adjustment_kind: source, .. }, RasterLayerNode::Adjustment { adjustment_kind: target, .. }) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *target = raster_clone_owned_string(source)?;
                    }
                    _ if !raster_reserve_unit(cx) => return Ok(false),
                    _ => {}
                }
                self.frames[self.depth].phase = 4;
            }
            4 => {
                if let (RasterLayerNode::Adjustment { params: source, .. }, RasterLayerNode::Adjustment { params: target, .. }) = (source, target) {
                    if let Some(authority) = self.parameter_value.as_mut() {
                        let (_, source_value) = self.parameter_key.next(source).ok_or("raster-store.clone-parameter-source")?;
                        if authority.step(source_value, &mut self.totals, cx)? {
                            let (source_key, _) = self.parameter_key.next(source).ok_or("raster-store.clone-parameter-advance")?;
                            let pending_key = self.pending_parameter_key.as_ref().ok_or("raster-store.clone-parameter-key")?;
                            if target.page_required_for_insert(pending_key) {
                                if !raster_reserve_unit(cx) {
                                    return Ok(false);
                                }
                                target.admit_one_page()?;
                                return Ok(false);
                            }
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            let key = self.pending_parameter_key.take().ok_or("raster-store.clone-parameter-key")?;
                            let value = authority.take().ok_or("raster-store.clone-parameter-value")?;
                            drop(self.parameter_value.take());
                            match target.insert_pre_admitted(key, value) {
                                Ok(RasterOwnedMapInsert::Inserted) => {}
                                Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                                    self.retirement.put(RasterDisplaced::ValueEntry(previous.take()));
                                    return Err("raster-store.clone-duplicate-parameter");
                                }
                                Err(rejected) => {
                                    self.retirement.put(RasterDisplaced::ValueEntry((rejected.key, rejected.value)));
                                    return Err(rejected.reason);
                                }
                            }
                            self.parameter_key.advance(source_key)?;
                        }
                        return Ok(false);
                    }
                    if let Some((key, value)) = self.parameter_key.next(source) {
                        if self.pending_parameter_key.is_none() {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            *self.pending_parameter_key = Some(raster_clone_owned_string(key)?);
                            return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *self.parameter_value = Some(Box::new(RasterDslValueCloneAuthority::new(value)));
                        return Ok(false);
                    }
                    self.parameter_key = RasterMapKeyCursor::new();
                }
                self.frames[self.depth].phase = 5;
            }
            5 => {
                if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source, target) {
                    if frame.child < source.len() {
                        if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                            return Err("raster-store.clone-layer-depth");
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.push(Self::skeleton(&source[frame.child]));
                        if let (RasterLayerNode::Group { children: source_child, .. }, RasterLayerNode::Group { children: target_child, .. }) = (&source[frame.child], target.last().ok_or("raster-store.clone-layer-child-target")?) {
                            self.totals.observe_candidate_capacity(source_child.capacity().saturating_add(1), target_child.capacity(), size_of::<RasterLayerNode>())?;
                        }
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if !raster_reserve_unit(cx) {return Ok(false);}
                let pair=match (source,target) {
                    (RasterLayerNode::Pixel {mask:source,..},RasterLayerNode::Pixel {mask:target,..})|(RasterLayerNode::Group {mask:source,..},RasterLayerNode::Group {mask:target,..})=>source.as_ref().zip(target.as_mut()),
                    _=>None,
                };
                if let Some((source,target))=pair {
                    if let Some(key)=&source.image_key {target.image_key=Some(raster_clone_owned_string(key)?);}
                }
                self.frames[self.depth].phase=7;
            }
            _ => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    pub(super) fn take(&mut self) -> Option<RasterLayerNode> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(authority) = self.parameter_value.as_ref() {
            return raster_nested(authority.close_demands(body)?);
        }
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.pending_parameter_key.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
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
        if let Some(authority) = self.parameter_value.as_mut() {
            let demand = raster_nested(authority.close_demands(grant.maximum_copy_bytes)?)?;
            if grant.maximum_depth < demand.depth {
                return Err(invariant("Raster layer clone parameter exceeds admitted depth"));
            }
            if raster_short(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let step = authority.close_step(raster_child_grant(grant))?;
            if authority.terminal_is_empty() {
                drop(self.parameter_value.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_empty() {
            if let Some(key) = self.pending_parameter_key.take() {
                self.retirement.put(RasterDisplaced::String(key));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Layer(value));
            }
        }
        let step = self.retirement.close(grant)?;
        Ok(raster_finish(step, self.terminal_is_empty()))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_empty() && self.pending_parameter_key.is_none() && self.parameter_value.is_none()
    }
}

impl Drop for RasterLayerCloneAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster layer clone reached Drop before exact handoff or retirement");
    }
}

struct RasterDslValueCloneAuthority {
    pub(super) value: std::mem::ManuallyDrop<Option<semio_framework_value::DslValue>>,
    pub(super) retirement: RasterRetirementSlot,
    pub(super) depth: usize,
    pub(super) path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    pub(super) frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pub(super) pending_key: std::mem::ManuallyDrop<Option<String>>,
    root_capacity_observed: bool,
    pub(super) terminal: bool,
}
