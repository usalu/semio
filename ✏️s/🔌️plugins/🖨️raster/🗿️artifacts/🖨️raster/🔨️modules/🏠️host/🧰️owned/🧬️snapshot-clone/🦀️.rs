//! 🧬️ raster snapshot clone: the snapshot, dsl value and layer clone authorities that copy a snapshot incrementally.

use super::*;

#[path="📜️demands/🦀️.rs"]mod demands;

pub(super) fn admit_clone(cx:&mut dyn RasterWork,copy:usize,capacity:usize,release:usize)->bool{
 let grant=cx.retained_grant();let d=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,release_bytes:release,depth:1,..Default::default()};if grant.maximum_items==0||grant.maximum_depth<1||raster_short(grant,d){return false}raster_reserve_unit(cx)
}
pub(super) fn record_clone(cx:&mut dyn RasterWork,copy:usize,capacity:usize,release:usize)->Result<(),&'static str>{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:release}).map_err(|_|"raster-store.clone-original-receipt")}

pub(super) fn empty_vec<T>(count:usize)->Vec<T>{let storage=Box::<[T]>::new_uninit_slice(count);let capacity=storage.len();let pointer=Box::into_raw(storage).cast::<std::mem::MaybeUninit<T>>().cast::<T>();unsafe{Vec::from_raw_parts(pointer,0,capacity)}}

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

    pub(super) fn step(&mut self, source: &RasterSnapshot, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        if self.phase == 0 {
            if self.bounds.step(source, cx)? {
                self.phase = 1;
            }
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("raster-store.initializer-clone-target")?;
        match self.phase {
            1 => {
                if !admit_clone(cx,size_of::<String>()+source.schema.len(),source.schema.len(),0) {
                    return Ok(false);
                }
                target.schema = raster_clone_owned_string(&source.schema)?;
                record_clone(cx,size_of::<String>()+source.schema.len(),target.schema.capacity(),0)?;
            }
            2 => {
                if !admit_clone(cx,size_of::<String>()+source.id.len(),source.id.len(),0) {
                    return Ok(false);
                }
                target.id = raster_clone_owned_string(&source.id)?;
                record_clone(cx,size_of::<String>()+source.id.len(),target.id.capacity(),0)?;
            }
            3 => {
                if let Some(title) = source.title.as_ref() {
                    if !admit_clone(cx,size_of::<Option<String>>()+title.len(),title.len(),0) {
                        return Ok(false);
                    }
                    target.title = Some(raster_clone_owned_string(title)?);
                    record_clone(cx,size_of::<Option<String>>()+title.len(),target.title.as_ref().map_or(0,String::capacity),0)?;
                } else {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                }
            }
            4 => {
                if self.index == 0 && target.layers.capacity() == 0 {
                    let count=source.layers.len().saturating_add(1);let capacity=count*size_of::<RasterLayerNode>();
                    if !admit_clone(cx,size_of::<Vec<RasterLayerNode>>(),capacity,0) {
                        return Ok(false);
                    }
                    target.layers=empty_vec(count);
                    record_clone(cx,size_of::<Vec<RasterLayerNode>>(),target.layers.capacity()*size_of::<RasterLayerNode>(),0)?;
                    self.bounds.totals.observe_candidate_capacity(count, target.layers.capacity(), size_of::<RasterLayerNode>())?;
                    return Ok(false);
                }
                if let Some(layer) = self.layer.as_mut() {
                    if !layer.terminal{layer.step(source.layers.get(self.index).ok_or("raster-store.initializer-layer-source")?,cx)?;return Ok(false);}
                    {
                        if !admit_clone(cx,size_of::<RasterLayerNode>(),0,size_of::<RasterLayerCloneAuthority>()) {
                            return Ok(false);
                        }
                        target.layers.push(layer.take().ok_or("raster-store.initializer-layer-handoff")?);
                        drop(self.layer.take());
                        record_clone(cx,size_of::<RasterLayerNode>(),0,size_of::<RasterLayerCloneAuthority>())?;
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if let Some(layer) = source.layers.get(self.index) {
                    if !admit_clone(cx,size_of::<RasterLayerCloneAuthority>(),size_of::<RasterLayerCloneAuthority>(),0) {
                        return Ok(false);
                    }
                    *self.layer = Some(Box::new(RasterLayerCloneAuthority::new(layer)));
                    record_clone(cx,size_of::<RasterLayerCloneAuthority>(),size_of::<RasterLayerCloneAuthority>(),0)?;
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
                    let copy=size_of::<(String,RasterAssetChild)>()+key.len();
                    if !admit_clone(cx,copy,key.len(),0) {
                        return Ok(false);
                    }
                    let key = raster_clone_owned_string(key)?;
                    let child =
                        store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                    *self.pending_asset = Some((key, child));
                    record_clone(cx,copy,self.pending_asset.as_ref().unwrap().0.capacity(),0)?;
                    self.asset_field = 0;
                    return Ok(false);
                }
                if self.asset_field >= 5 {
                    let (key, _) = self.pending_asset.as_ref().expect("Raster pending asset remains retained");
                    if target.assets.page_required_for_insert(key) {
                        let page=RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes();
                        if !admit_clone(cx,page,page,0) {
                            return Ok(false);
                        }
                        target.assets.admit_one_page()?;
                        record_clone(cx,page,page,0)?;
                        return Ok(false);
                    }
                    let copy=2*size_of::<(String,RasterAssetChild)>();
                    if !admit_clone(cx,copy,0,0) {
                        return Ok(false);
                    }
                    let (key, mut child) = self.pending_asset.take().expect("Raster pending asset handoff remains exact");
                    if let Some(source_child) = source.assets.get(&key) {
                        semio_s_artifact_raster_raster_model::adopt_raster_asset_owner(source_child, &mut child);
                    }
                    record_clone(cx,copy,0,0)?;
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
                if !admit_clone(cx,size_of::<String>()+source_value.len(),source_value.len(),0) {
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
                record_clone(cx,size_of::<String>()+source_value.len(),target_value.capacity(),0)?;
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
            if layer.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of::<RasterLayerCloneAuthority>(),depth:1,..Default::default()});}
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

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        let mut work=0;let result=(||{
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(layer) = self.layer.as_mut() {
            if layer.terminal_is_empty(){let release=size_of::<RasterLayerCloneAuthority>();if grant.maximum_depth<1||grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(empty));}drop(self.layer.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()}));}
            let demand = raster_nested(layer.close_demands(grant.maximum_copy_bytes)?)?;
            if grant.maximum_depth < demand.depth {
                return Err(invariant("Raster snapshot clone layer exceeds admitted depth"));
            }
            if raster_short(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let turn=layer.close_step(raster_child_grant(grant));work=turn.work;let step=turn.result?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_empty() {
            if let Some(entry) = self.pending_asset.take() {
                self.retirement.put(RasterDisplaced::AssetEntry(entry));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Snapshot(value));
            }
        }
        let turn=self.retirement.close(grant);work=turn.work;let step=turn.result?;
        Ok(raster_finish(step, self.terminal_is_empty()))
        })();semio_framework_value::RetainedWorkOutcome{result,work}
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
    fn skeleton_capacity(source:&semio_framework_value::DslValue)->usize{match source{semio_framework_value::DslValue::Array(values)=>values.len()*size_of::<semio_framework_value::DslValue>(),semio_framework_value::DslValue::Object(values)=>values.len()*size_of::<(String,semio_framework_value::DslValue)>(),_=>0}}
    fn skeleton(source: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
        match source {
            semio_framework_value::DslValue::Null => semio_framework_value::DslValue::Null,
            semio_framework_value::DslValue::Bool(value) => semio_framework_value::DslValue::Bool(*value),
            semio_framework_value::DslValue::Number(value) => semio_framework_value::DslValue::Number(*value),
            semio_framework_value::DslValue::String(_) => semio_framework_value::DslValue::String(String::new()),
            semio_framework_value::DslValue::Bytes(_) => semio_framework_value::DslValue::Bytes(Vec::new()),
            semio_framework_value::DslValue::Array(values) => semio_framework_value::DslValue::Array(empty_vec(values.len())),
            semio_framework_value::DslValue::Object(values) => semio_framework_value::DslValue::Object(empty_vec(values.len())),
        }
    }

    pub(super) fn new(_source: &semio_framework_value::DslValue) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
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
            (semio_framework_value::DslValue::Array(source), semio_framework_value::DslValue::Array(target)) => totals.observe_candidate_capacity(source.len(), target.capacity(), size_of::<semio_framework_value::DslValue>()),
            (semio_framework_value::DslValue::Object(source), semio_framework_value::DslValue::Object(target)) => totals.observe_candidate_capacity(source.len(), target.capacity(), size_of::<(String, semio_framework_value::DslValue)>()),
            _ => Ok(()),
        }
    }

    pub(super) fn step(&mut self, source_root: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.value.is_none(){let capacity=Self::skeleton_capacity(source_root);if !admit_clone(cx,size_of::<semio_framework_value::DslValue>(),capacity,0){return Ok(false)}*self.value=Some(Self::skeleton(source_root));record_clone(cx,size_of::<semio_framework_value::DslValue>(),capacity,0)?;return Ok(false)}
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
                    if !admit_clone(cx,size_of::<String>()+source.len(),source.len(),0) {
                        return Ok(false);
                    }
                    *target = raster_clone_owned_string(source)?;
                    record_clone(cx,size_of::<String>()+source.len(),target.capacity(),0)?;
                }
                (semio_framework_value::DslValue::Bytes(source), semio_framework_value::DslValue::Bytes(target)) => {
                    if source.len() > RASTER_MAXIMUM_NESTED_BYTES {
                        return Err("raster-store.clone-value-bytes-capacity");
                    }
                    if target.capacity()==0&&!source.is_empty(){if !admit_clone(cx,size_of::<Vec<u8>>(),source.len(),0){return Ok(false)}*target=empty_vec(source.len());record_clone(cx,size_of::<Vec<u8>>(),target.capacity(),0)?;return Ok(false)}
                    if target.len()<source.len(){let count=(source.len()-target.len()).min(cx.retained_grant().maximum_copy_bytes.saturating_sub(size_of::<Vec<u8>>())).min(32768);if count==0||!admit_clone(cx,size_of::<Vec<u8>>()+count,0,0){return Ok(false)}let start=target.len();target.extend_from_slice(&source[start..start+count]);record_clone(cx,size_of::<Vec<u8>>()+count,0,0)?;return Ok(false)}
                    if !raster_reserve_unit(cx){return Ok(false)}
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
                let capacity=Self::skeleton_capacity(&source[frame.child]);
                if !admit_clone(cx,size_of::<semio_framework_value::DslValue>(),capacity,0) {
                    return Ok(false);
                }
                target.push(Self::skeleton(&source[frame.child]));
                record_clone(cx,size_of::<semio_framework_value::DslValue>(),capacity,0)?;
                Self::observe_container_capacity(&source[frame.child], target.last().ok_or("raster-store.clone-value-child-target")?, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            (semio_framework_value::DslValue::Object(source), semio_framework_value::DslValue::Object(target)) if frame.child < source.len() => {
                if frame.phase == 1 {
                    let key=&source[frame.child].0;
                    if !admit_clone(cx,size_of::<String>()+key.len(),key.len(),0) {
                        return Ok(false);
                    }
                    *self.pending_key = Some(raster_clone_owned_string(&source[frame.child].0)?);
                    record_clone(cx,size_of::<String>()+key.len(),self.pending_key.as_ref().map_or(0,String::capacity),0)?;
                    self.frames[self.depth].phase = 2;
                    return Ok(false);
                }
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                let capacity=Self::skeleton_capacity(&source[frame.child].1);
                if !admit_clone(cx,size_of::<(String,semio_framework_value::DslValue)>(),capacity,0) {
                    return Ok(false);
                }
                target.push((self.pending_key.take().ok_or("raster-store.clone-value-key")?, Self::skeleton(&source[frame.child].1)));
                record_clone(cx,size_of::<(String,semio_framework_value::DslValue)>(),capacity,0)?;
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

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        let mut work=0;let result=(||{
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
        let turn=self.retirement.close(grant);work=turn.work;let step=turn.result?;
        Ok(raster_finish(step, self.terminal_is_empty()))
        })();semio_framework_value::RetainedWorkOutcome{result,work}
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
    fn skeleton_capacity(source:&RasterLayerNode)->usize{match source{RasterLayerNode::Group{children,..}=>children.len().saturating_add(1)*size_of::<RasterLayerNode>(),_=>0}}
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
                children: empty_vec(children.len().saturating_add(1)),
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

    pub(super) fn step(&mut self, source_root: &RasterLayerNode, cx: &mut dyn RasterWork) -> Result<bool, &'static str> {
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
            let capacity=Self::skeleton_capacity(source_root);
            if !admit_clone(cx,size_of::<RasterLayerNode>(),capacity,0) {
                return Ok(false);
            }
            *self.value = Some(Self::skeleton(source_root));
            record_clone(cx,size_of::<RasterLayerNode>(),capacity,0)?;
            return Ok(false);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source_root, self.value.as_ref().ok_or("raster-store.clone-layer-observed-target")?) {
                self.totals.observe_candidate_capacity(source.len().saturating_add(1), target.capacity(), size_of::<RasterLayerNode>())?;
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
                if !admit_clone(cx,size_of::<String>()+source.len(),source.len(),0) {
                    return Ok(false);
                }
                **target = raster_clone_owned_string(source)?;
                record_clone(cx,size_of::<String>()+source.len(),target.capacity(),0)?;
                self.frames[self.depth].phase += 1;
            }
            3 => {
                match (source, target) {
                    (RasterLayerNode::Pixel { image_key: source, .. }, RasterLayerNode::Pixel { image_key: target, .. }) => {
                        if let Some(source) = source {
                            if !admit_clone(cx,size_of::<Option<String>>()+source.len(),source.len(),0) {
                                return Ok(false);
                            }
                            *target = Some(raster_clone_owned_string(source)?);
                            record_clone(cx,size_of::<Option<String>>()+source.len(),target.as_ref().map_or(0,String::capacity),0)?;
                        } else if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                    }
                    (RasterLayerNode::Adjustment { adjustment_kind: source, .. }, RasterLayerNode::Adjustment { adjustment_kind: target, .. }) => {
                        if !admit_clone(cx,size_of::<String>()+source.len(),source.len(),0) {
                            return Ok(false);
                        }
                        *target = raster_clone_owned_string(source)?;
                        record_clone(cx,size_of::<String>()+source.len(),target.capacity(),0)?;
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
                        if !authority.terminal { authority.step(source_value, &mut self.totals, cx)?; return Ok(false); }
                        {
                            let (source_key, _) = self.parameter_key.next(source).ok_or("raster-store.clone-parameter-advance")?;
                            let pending_key = self.pending_parameter_key.as_ref().ok_or("raster-store.clone-parameter-key")?;
                            if target.page_required_for_insert(pending_key) {
                                let page=RasterOwnedMap::<semio_framework_value::DslValue>::conservative_page_credit_bytes();
                                if !admit_clone(cx,page,page,0) {
                                    return Ok(false);
                                }
                                target.admit_one_page()?;
                                record_clone(cx,page,page,0)?;
                                return Ok(false);
                            }
                            let copy=2*size_of::<(String,semio_framework_value::DslValue)>();let release=size_of::<RasterDslValueCloneAuthority>();
                            if !admit_clone(cx,copy,0,release) {
                                return Ok(false);
                            }
                            let key = self.pending_parameter_key.take().ok_or("raster-store.clone-parameter-key")?;
                            let value = authority.take().ok_or("raster-store.clone-parameter-value")?;
                            drop(self.parameter_value.take());
                            record_clone(cx,copy,0,release)?;
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
                            if !admit_clone(cx,size_of::<String>()+key.len(),key.len(),0) {
                                return Ok(false);
                            }
                            *self.pending_parameter_key = Some(raster_clone_owned_string(key)?);
                            record_clone(cx,size_of::<String>()+key.len(),self.pending_parameter_key.as_ref().map_or(0,String::capacity),0)?;
                            return Ok(false);
                        }
                        let capacity=size_of::<RasterDslValueCloneAuthority>();
                        if !admit_clone(cx,capacity,capacity,0) {
                            return Ok(false);
                        }
                        *self.parameter_value = Some(Box::new(RasterDslValueCloneAuthority::new(value)));
                        record_clone(cx,capacity,capacity,0)?;
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
                        let capacity=Self::skeleton_capacity(&source[frame.child]);
                        if !admit_clone(cx,2*size_of::<RasterLayerNode>(),capacity,0) {
                            return Ok(false);
                        }
                        target.push(Self::skeleton(&source[frame.child]));
                        record_clone(cx,2*size_of::<RasterLayerNode>(),capacity,0)?;
                        if let (RasterLayerNode::Group { children: source_child, .. }, RasterLayerNode::Group { children: target_child, .. }) = (&source[frame.child], target.last().ok_or("raster-store.clone-layer-child-target")?) {
                            self.totals.observe_candidate_capacity(source_child.len().saturating_add(1), target_child.capacity(), size_of::<RasterLayerNode>())?;
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
                let pair=match (source,target) {
                    (RasterLayerNode::Pixel {mask:source,..},RasterLayerNode::Pixel {mask:target,..})|(RasterLayerNode::Group {mask:source,..},RasterLayerNode::Group {mask:target,..})=>source.as_ref().zip(target.as_mut()),
                    _=>None,
                };
                let key=pair.as_ref().and_then(|(source,_)|source.image_key.as_ref());if key.is_none(){if !raster_reserve_unit(cx){return Ok(false)}self.frames[self.depth].phase=7;return Ok(false)}let bytes=key.map_or(0,String::len);
                if !admit_clone(cx,size_of::<Option<String>>()+bytes,bytes,0){return Ok(false);}
                if let Some((source,target))=pair {
                    if let Some(key)=&source.image_key {target.image_key=Some(raster_clone_owned_string(key)?);}
                }
                record_clone(cx,size_of::<Option<String>>()+bytes,bytes,0)?;
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
            if authority.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of::<RasterDslValueCloneAuthority>(),depth:1,..Default::default()});}
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

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        let mut work=0;let result=(||{
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(authority) = self.parameter_value.as_mut() {
            if authority.terminal_is_empty(){let release=size_of::<RasterDslValueCloneAuthority>();if grant.maximum_depth<1||grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(empty));}drop(self.parameter_value.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()}));}
            let demand = raster_nested(authority.close_demands(grant.maximum_copy_bytes)?)?;
            if grant.maximum_depth < demand.depth {
                return Err(invariant("Raster layer clone parameter exceeds admitted depth"));
            }
            if raster_short(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let turn=authority.close_step(raster_child_grant(grant));work=turn.work;let step=turn.result?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_empty() {
            if let Some(key) = self.pending_parameter_key.take() {
                self.retirement.put(RasterDisplaced::String(key));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Layer(value));
            }
        }
        let turn=self.retirement.close(grant);work=turn.work;let step=turn.result?;
        Ok(raster_finish(step, self.terminal_is_empty()))
        })();semio_framework_value::RetainedWorkOutcome{result,work}
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

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
