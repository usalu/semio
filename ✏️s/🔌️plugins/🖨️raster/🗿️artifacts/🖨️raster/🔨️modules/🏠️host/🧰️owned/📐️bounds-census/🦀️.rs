//! 📐️ raster bounds census: the owner totals, the map key cursor, the snapshot, layer and dsl value bounds authorities, the layer locator and the traversal frame.

use super::*;

impl RasterOwnerTotals {
    pub fn new() -> Self {
        Self { source_items: 0, source_bytes: 0, candidate_items: 0, candidate_bytes: 0 }
    }

    pub fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        self.source_items = self.source_items.checked_add(items).ok_or("raster-store.preflight-source-item-overflow")?;
        self.source_bytes = self.source_bytes.checked_add(bytes).ok_or("raster-store.preflight-source-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(candidate_items).ok_or("raster-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(candidate_bytes).ok_or("raster-store.preflight-candidate-byte-overflow")?;
        if self.source_items > RASTER_MAXIMUM_NESTED_ITEMS || self.candidate_items > RASTER_MAXIMUM_NESTED_ITEMS {
            return Err("raster-store.preflight-item-capacity");
        }
        if self.source_bytes > RASTER_MAXIMUM_NESTED_BYTES || self.candidate_bytes > RASTER_MAXIMUM_NESTED_BYTES {
            return Err("raster-store.preflight-byte-capacity");
        }
        Ok(())
    }

    fn string(&mut self, value: &String) -> Result<(), &'static str> {
        if value.capacity() > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-string-allocation-capacity");
        }
        self.add(1, value.capacity(), 1, value.len())
    }

    fn vector<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let bytes = value.capacity().checked_mul(size_of::<T>()).ok_or("raster-store.preflight-vector-byte-overflow")?;
        if bytes > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-vector-allocation-capacity");
        }
        self.add(1, bytes, 1, bytes)
    }

    fn layer_vector(&mut self, value: &Vec<RasterLayerNode>) -> Result<(), &'static str> {
        let source_bytes = value.capacity().checked_mul(size_of::<RasterLayerNode>()).ok_or("raster-store.preflight-layer-vector-byte-overflow")?;
        let candidate_capacity = value.capacity().checked_add(1).ok_or("raster-store.preflight-layer-vector-capacity-overflow")?;
        let candidate_bytes = candidate_capacity.checked_mul(size_of::<RasterLayerNode>()).ok_or("raster-store.preflight-layer-candidate-byte-overflow")?;
        if source_bytes > RASTER_OWNED_FIELD_BYTES || candidate_bytes > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-layer-vector-allocation-capacity");
        }
        self.add(1, source_bytes, 1, candidate_bytes)
    }

    pub fn map<V>(&mut self, value: &RasterOwnedMap<V>, candidate_extra_entries: usize) -> Result<(), &'static str> {
        let source_pages = value.allocated_page_count();
        let candidate_entries = value.len().checked_add(candidate_extra_entries).ok_or("raster-store.preflight-map-entry-overflow")?;
        if candidate_entries > semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY {
            return Err("raster-store.preflight-map-item-capacity");
        }
        let candidate_pages = candidate_entries.div_ceil(semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_PAGE_CAPACITY);
        let page_bytes = RasterOwnedMap::<V>::conservative_page_credit_bytes();
        self.add(source_pages, source_pages.checked_mul(page_bytes).ok_or("raster-store.preflight-map-source-byte-overflow")?, candidate_pages, candidate_pages.checked_mul(page_bytes).ok_or("raster-store.preflight-map-candidate-byte-overflow")?)
    }

    pub fn observe_candidate_capacity(&mut self, requested: usize, observed: usize, element_bytes: usize) -> Result<(), &'static str> {
        if observed < requested {
            return Err("raster-store.candidate-capacity-underflow");
        }
        let extra = observed.checked_sub(requested).and_then(|value| value.checked_mul(element_bytes)).ok_or("raster-store.candidate-capacity-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(extra).ok_or("raster-store.candidate-byte-overflow")?;
        if self.candidate_bytes > RASTER_MAXIMUM_NESTED_BYTES || extra > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.candidate-observed-capacity");
        }
        Ok(())
    }
}

pub struct RasterMapKeyCursor {
    pub index: usize,
}

impl RasterMapKeyCursor {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> Option<(&'a String, &'a T)> {
        values.entry_at(self.index)
    }

    pub fn advance(&mut self, _key: &str) -> Result<(), &'static str> {
        self.index = self.index.checked_add(1).ok_or("raster-store.map-index-overflow")?;
        Ok(())
    }
}

impl RasterDslValueBoundsAuthority {
    pub fn new(layer_depth: usize) -> Self {
        Self { layer_depth, depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], terminal: false }
    }

    pub fn value_at<'a>(root: &'a semio_framework_value::DslValue, path: &[usize]) -> Option<&'a semio_framework_value::DslValue> {
        let mut value = root;
        for index in path {
            value = match value {
                semio_framework_value::DslValue::Array(values) => values.get(*index)?,
                semio_framework_value::DslValue::Object(values) => &values.get(*index)?.1,
                _ => return None,
            };
        }
        Some(value)
    }

    pub fn step(&mut self, root: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let _required_frames = raster_combined_depth_requirement(self.layer_depth, self.depth + 1)?;
        let value = Self::value_at(root, &self.path[..self.depth]).ok_or("raster-store.preflight-value-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            totals.add(1, size_of::<semio_framework_value::DslValue>(), 1, size_of::<semio_framework_value::DslValue>())?;
            match value {
                semio_framework_value::DslValue::String(value) => totals.string(value)?,
                semio_framework_value::DslValue::Bytes(value) => totals.vector(value)?,
                semio_framework_value::DslValue::Array(values) => totals.vector(values)?,
                semio_framework_value::DslValue::Object(values) => totals.vector(values)?,
                _ => {}
            }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        let child = frame.child;
        match value {
            semio_framework_value::DslValue::Object(values) if frame.phase == 1 && child < values.len() => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.string(&values[child].0)?;
                self.frames[self.depth].phase = 2;
                return Ok(false);
            }
            _ if match value {
                semio_framework_value::DslValue::Array(values) => child < values.len(),
                semio_framework_value::DslValue::Object(values) => frame.phase == 2 && child < values.len(),
                _ => false,
            } =>
            {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH || !raster_reserve_unit(cx) {
                    if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                        return Err("raster-store.preflight-value-depth");
                    }
                    return Ok(false);
                }
                self.path[self.depth] = child;
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
}

impl RasterLayerBoundsAuthority {
    pub fn new() -> Self {
        Self { depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], parameter_key: RasterMapKeyCursor::new(), parameter_value: None, terminal: false }
    }

    pub fn layer_at<'a>(root: &'a RasterLayerNode, path: &[usize]) -> Option<&'a RasterLayerNode> {
        let mut value = root;
        for index in path {
            let RasterLayerNode::Group { children, .. } = value else { return None };
            value = children.get(*index)?;
        }
        Some(value)
    }

    pub fn strings(layer: &RasterLayerNode) -> [&String; 3] {
        match layer {
            RasterLayerNode::Pixel { id, name, blend_mode, .. } | RasterLayerNode::Group { id, name, blend_mode, .. } | RasterLayerNode::Adjustment { id, name, blend_mode, .. } => [id, name, blend_mode],
        }
    }

    pub fn step(&mut self, root: &RasterLayerNode, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("raster-store.preflight-layer-path")?;
        let frame = self.frames[self.depth];
        match frame.phase {
            0 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.add(1, size_of::<RasterLayerNode>(), 1, size_of::<RasterLayerNode>())?;
                self.frames[self.depth].phase = 1;
            }
            1..=3 => {
                let value = Self::strings(layer)[(frame.phase - 1) as usize];
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.string(value)?;
                self.frames[self.depth].phase += 1;
            }
            4 => {
                let value = match layer {
                    RasterLayerNode::Pixel { image_key, .. } => image_key.as_ref(),
                    RasterLayerNode::Adjustment { adjustment_kind, .. } => Some(adjustment_kind),
                    RasterLayerNode::Group { .. } => None,
                };
                if let Some(value) = value {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    totals.string(value)?;
                } else if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.frames[self.depth].phase = 5;
            }
            5 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                match layer {
                    RasterLayerNode::Group { children, .. } => totals.layer_vector(children)?,
                    RasterLayerNode::Adjustment { params, .. } => totals.map(params, 0)?,
                    RasterLayerNode::Pixel { .. } => {}
                }
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if let RasterLayerNode::Adjustment { params, .. } = layer {
                    if let Some(value) = self.parameter_value.as_mut() {
                        let (_, source) = self.parameter_key.next(params).ok_or("raster-store.preflight-parameter-source")?;
                        if value.step(source, totals, cx)? {
                            let (key, _) = self.parameter_key.next(params).ok_or("raster-store.preflight-parameter-key")?;
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            self.parameter_value = None;
                            self.parameter_key.advance(key)?;
                        }
                        return Ok(false);
                    }
                    if let Some((key, _)) = self.parameter_key.next(params) {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        totals.string(key)?;
                        self.parameter_value = Some(RasterDslValueBoundsAuthority::new(self.depth + 1));
                        return Ok(false);
                    }
                    self.parameter_key = RasterMapKeyCursor::new();
                }
                self.frames[self.depth].phase = 7;
            }
            7 => {
                if let RasterLayerNode::Group { children, .. } = layer {
                    if frame.child < children.len() {
                        if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH || !raster_reserve_unit(cx) {
                            if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                                return Err("raster-store.preflight-layer-depth");
                            }
                            return Ok(false);
                        }
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 8;
            }
            8 => {
                if !raster_reserve_unit(cx) {return Ok(false);}
                if let RasterLayerNode::Pixel {mask:Some(mask),..}|RasterLayerNode::Group {mask:Some(mask),..}=layer {
                    if let Some(key)=&mask.image_key {totals.string(key)?;}
                }
                self.frames[self.depth].phase=9;
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
}

pub struct RasterSnapshotBoundsAuthority {
    pub totals: RasterOwnerTotals,
    pub layer: Option<RasterLayerBoundsAuthority>,
    pub asset_key: RasterMapKeyCursor,
    pub phase: u8,
    pub index: usize,
    pub asset_field: u8,
    pub terminal: bool,
}

impl RasterSnapshotBoundsAuthority {
    pub fn new() -> Self {
        Self { totals: RasterOwnerTotals::new(), layer: None, asset_key: RasterMapKeyCursor::new(), phase: 0, index: 0, asset_field: 0, terminal: false }
    }

    pub fn step(&mut self, source: &RasterSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match self.phase {
            0 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.add(1, size_of::<RasterSnapshot>(), 1, size_of::<RasterSnapshot>())?;
                self.phase = 1;
            }
            1 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(&source.schema)?;
                self.phase = 2;
            }
            2 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(&source.id)?;
                self.phase = 3;
            }
            3 => {
                if let Some(title) = &source.title {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    self.totals.string(title)?;
                } else if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.phase = 4;
            }
            4 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.layer_vector(&source.layers)?;
                self.phase = 5;
            }
            5 => {
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("raster-store.preflight-root-layer")?, &mut self.totals, cx)? {
                        self.layer = None;
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if source.layers.get(self.index).is_some() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    self.layer = Some(RasterLayerBoundsAuthority::new());
                    return Ok(false);
                }
                self.phase = 6;
                self.index = 0;
            }
            6 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.map(&source.assets, usize::from(source.assets.len()<semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY))?;
                self.phase = 7;
            }
            7 => {
                let Some((key, child)) = self.asset_key.next(&source.assets) else {
                    self.terminal = true;
                    return Ok(true);
                };
                let value = match self.asset_field {
                    0 => key,
                    1 => &child.child_id,
                    2 => &child.target.artifact_id,
                    3 => &child.target.dialect.artifact_kind,
                    4 => &child.target.dialect.standard,
                    5 => &child.target.dialect.subset,
                    _ => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        self.asset_key.advance(key)?;
                        self.asset_field = 0;
                        return Ok(false);
                    }
                };
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(value)?;
                self.asset_field += 1;
            }
            _ => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterLayerAddress {
    pub length: usize,
    pub indices: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
}

impl RasterLayerAddress {
    pub fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    pub fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

pub struct RasterLayerLocator {
    pub root: usize,
    pub depth: usize,
    pub path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    pub frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pub found: Option<RasterLayerAddress>,
    pub terminal: bool,
}

impl RasterLayerLocator {
    pub fn new() -> Self {
        Self { root: 0, depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], found: None, terminal: false }
    }

    pub fn node_id(node: &RasterLayerNode) -> &String {
        match node {
            RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
        }
    }

    pub fn node_at(snapshot: &RasterSnapshot, address: RasterLayerAddress) -> Option<&RasterLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let RasterLayerNode::Group { children, .. } = value else { return None };
            value = children.get(*index)?;
        }
        Some(value)
    }

    pub fn node_at_mut(snapshot: &mut RasterSnapshot, address: RasterLayerAddress) -> Option<&mut RasterLayerNode> {
        fn descend<'a>(value: &'a mut RasterLayerNode, path: &[usize]) -> Option<&'a mut RasterLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let RasterLayerNode::Group { children, .. } = value else { return None };
            descend(children.get_mut(*head)?, tail)
        }
        descend(snapshot.layers.get_mut(address.indices[0])?, &address.indices[1..address.length])
    }

    pub fn container_mut(snapshot: &mut RasterSnapshot, parent: Option<RasterLayerAddress>) -> Option<&mut Vec<RasterLayerNode>> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                RasterLayerNode::Group { children, .. } => Some(children),
                _ => None,
            },
        }
    }

    pub fn step(&mut self, snapshot: &RasterSnapshot, target: &str, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = RasterLayerBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("raster-store.locator-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            self.frames[self.depth].phase = 1;
            if Self::node_id(node) == target {
                let mut indices = [0; RASTER_MAXIMUM_NESTED_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(RasterLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        if let RasterLayerNode::Group { children, .. } = node {
            if frame.child < children.len() {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.locator-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = RasterTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        Ok(false)
    }
}

#[derive(Clone, Copy)]
pub struct RasterTraversalFrame {
    pub phase: u8,
    pub child: usize,
}

impl RasterTraversalFrame {
    pub const EMPTY: Self = Self { phase: 0, child: 0 };
}

pub struct RasterOwnerTotals {
    source_items: usize,
    pub source_bytes: usize,
    candidate_items: usize,
    pub candidate_bytes: usize,
}

pub struct RasterDslValueBoundsAuthority {
    layer_depth: usize,
    pub depth: usize,
    pub path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    pub frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pub terminal: bool,
}

pub struct RasterLayerBoundsAuthority {
    pub depth: usize,
    pub path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    pub frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pub parameter_key: RasterMapKeyCursor,
    pub parameter_value: Option<RasterDslValueBoundsAuthority>,
    pub terminal: bool,
}

pub fn raster_frozen_now_us() -> Option<u64> {
    Some(0)
}

pub fn raster_combined_depth_requirement(layer_depth: usize, value_depth: usize) -> Result<usize, &'static str> {
    let required = layer_depth.checked_add(value_depth.checked_mul(2).ok_or("raster-store.preflight-combined-depth-overflow")?).and_then(|value| value.checked_add(16)).ok_or("raster-store.preflight-combined-depth-overflow")?;
    if required > RASTER_COMBINED_DEPTH_CAPACITY {
        return Err("raster-store.preflight-combined-depth");
    }
    Ok(required)
}
