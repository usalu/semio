//! 📐️ Drawing bounds census: the traversal frame, the snapshot owner totals, the fixed owner census, the asset bounds cursor and the snapshot bounds authority.

use super::*;

#[derive(Clone, Copy)]
pub struct DrawingTraversalFrame {
    pub phase: u8,
    pub child: usize,
    pub string: usize,
}

impl DrawingTraversalFrame {
    pub const EMPTY: Self = Self { phase: 0, child: 0, string: 0 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingSnapshotOwnerTotals {
    pub source_items: usize,
    pub source_bytes: usize,
    pub candidate_items: usize,
    pub candidate_bytes: usize,
    pub maximum_container: usize,
}

#[derive(Clone, Copy)]
struct DrawingOwnerCreditSlot {
    pub source_items: u32,
    pub source_bytes: u32,
    pub derived_items: u32,
    pub derived_bytes: u32,
}

impl DrawingOwnerCreditSlot {
    pub const EMPTY: Self = Self { source_items: 0, source_bytes: 0, derived_items: 0, derived_bytes: 0 };
}

pub struct DrawingFixedOwnerCensus {
    pub slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS],
    pub length: usize,
}

impl DrawingFixedOwnerCensus {
    pub fn new() -> Self {
        Self { slots: [DrawingOwnerCreditSlot::EMPTY; DRAWING_MAXIMUM_NESTED_ITEMS], length: 0 }
    }

    pub fn admit(&mut self, source_items: usize, source_bytes: usize, derived_items: usize, derived_bytes: usize) -> Result<DrawingOwnerCreditSlot, &'static str> {
        let target = self.slots.get_mut(self.length).ok_or("drawing-store.owner-census-slot-capacity")?;
        *target = DrawingOwnerCreditSlot {
            source_items: source_items.try_into().map_err(|_| "drawing-store.owner-census-item-width")?,
            source_bytes: source_bytes.try_into().map_err(|_| "drawing-store.owner-census-byte-width")?,
            derived_items: derived_items.try_into().map_err(|_| "drawing-store.owner-census-item-width")?,
            derived_bytes: derived_bytes.try_into().map_err(|_| "drawing-store.owner-census-byte-width")?,
        };
        self.length += 1;
        Ok(*target)
    }
}

pub struct DrawingAssetBoundsCursor {
    pub index: usize,
    pub owner: Option<usize>,
}

impl DrawingAssetBoundsCursor {
    pub fn new() -> Self { Self { index: 0, owner: None } }

    pub fn next<'a>(&mut self, assets: &'a semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>) -> Result<Option<(&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>, &'a DrawingImageAsset)>, &'static str> {
        let owner = assets as *const _ as usize;
        if self.owner.is_some_and(|expected| expected != owner) { return Err("drawing-store.preflight-asset-owner-changed"); }
        self.owner = Some(owner);
        Ok(assets.entry_at(self.index))
    }

    pub fn advance(&mut self) -> Result<(), &'static str> {
        self.index = self.index.checked_add(1).ok_or("drawing-store.preflight-asset-index-overflow")?;
        Ok(())
    }
}

pub struct DrawingSnapshotBoundsAuthority {
    pub root: usize,
    pub asset_cursor: DrawingAssetBoundsCursor,
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub items: usize,
    pub bytes: usize,
    pub candidate_items: usize,
    pub candidate_bytes: usize,
    pub maximum_container: usize,
    pub owner_census: DrawingFixedOwnerCensus,
    pub direct: native_text_footprint::DrawingLayerFootprintCursor,
    pub record: native_text_footprint::DrawingRecordFootprintCursor,
    pub root_complete: bool,
    pub layers_complete: bool,
    pub terminal: bool,
}

impl DrawingSnapshotBoundsAuthority {
    pub fn new() -> Self {
        Self {
            root: 0,
            asset_cursor: DrawingAssetBoundsCursor::new(),
            depth: 0,
            path: [0; DRAWING_MAXIMUM_LAYER_DEPTH],
            frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH],
            items: 0,
            bytes: 0,
            candidate_items: 0,
            candidate_bytes: 0,
            maximum_container: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            direct: native_text_footprint::DrawingLayerFootprintCursor::default(),
            record: native_text_footprint::DrawingRecordFootprintCursor::default(),
            root_complete: false,
            layers_complete: false,
            terminal: false,
        }
    }

    pub fn layer_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut value = root;
        for index in path {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    pub fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        let credit = self.owner_census.admit(items, bytes, candidate_items, candidate_bytes)?;
        self.items = self.items.checked_add(credit.source_items as usize).ok_or("drawing-store.preflight-item-overflow")?;
        self.bytes = self.bytes.checked_add(credit.source_bytes as usize).ok_or("drawing-store.preflight-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(credit.derived_items as usize).ok_or("drawing-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(credit.derived_bytes as usize).ok_or("drawing-store.preflight-candidate-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS || self.candidate_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.preflight-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES || self.candidate_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.preflight-byte-capacity");
        }
        Ok(())
    }

    pub fn step(&mut self, source: &DrawingSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.layers_complete {
            self.maximum_container = self.maximum_container.max(source.layers.len());
            let Some(root) = source.layers.get(self.root) else {
                self.layers_complete = true;
                cx.consume_fuel(1);
                return Ok(false);
            };
            let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.preflight-path")?;
            let frame = self.frames[self.depth];
            if frame.phase == 0 {
                let complete = self.direct.step(layer, 1)?;
                cx.consume_fuel(1);
                if !complete { return Ok(false); }
                let totals = self.direct.totals().ok_or("drawing-store.preflight-layer-incomplete")?;
                self.add(totals.items, totals.backing_bytes, 0, 0)?;
                self.direct.close_step(1);
                self.direct = native_text_footprint::DrawingLayerFootprintCursor::default();
                self.frames[self.depth].phase = 1;
                return Ok(false);
            }
            if let DrawingLayerNode::Group(value) = layer {
                self.maximum_container = self.maximum_container.max(value.children.len());
                if frame.child < value.children.len() {
                    if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                        return Err("drawing-store.preflight-depth-capacity");
                    }
                    self.path[self.depth] = frame.child;
                    self.frames[self.depth].child += 1;
                    self.depth += 1;
                    self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
            }
            if self.depth == 0 {
                self.root += 1;
                self.frames[0] = DrawingTraversalFrame::EMPTY;
            } else {
                self.depth -= 1;
            }
            cx.consume_fuel(1);
            return Ok(false);
        }
        if !self.root_complete {
            let complete = self.record.step(native_text_footprint::DrawingRecordFootprintSource::Snapshot(source), 1)?;
            cx.consume_fuel(1);
            if !complete { return Ok(false); }
            let totals = self.record.totals().ok_or("drawing-store.preflight-root-incomplete")?;
            self.add(totals.items, totals.backing_bytes.checked_add(size_of::<DrawingSnapshot>()).ok_or("drawing-store.preflight-byte-overflow")?, 0, 0)?;
            self.record.close_step(1);
            self.record = native_text_footprint::DrawingRecordFootprintCursor::default();
            self.root_complete = true;
            return Ok(false);
        }
        let Some((key, value)) = self.asset_cursor.next(&source.assets)? else {
            self.terminal = true;
            return Ok(true);
        };
        let complete = self.record.step(native_text_footprint::DrawingRecordFootprintSource::Asset(key, value), 1)?;
        cx.consume_fuel(1);
        if !complete { return Ok(false); }
        let totals = self.record.totals().ok_or("drawing-store.preflight-asset-incomplete")?;
        self.add(totals.items, totals.backing_bytes, 0, 0)?;
        self.record.close_step(1);
        self.record = native_text_footprint::DrawingRecordFootprintCursor::default();
        self.asset_cursor.advance()?;
        Ok(false)
    }

    pub fn totals(&self) -> Option<DrawingSnapshotOwnerTotals> {
        self.terminal.then_some(DrawingSnapshotOwnerTotals { source_items: self.items, source_bytes: self.bytes, candidate_items: self.candidate_items, candidate_bytes: self.candidate_bytes, maximum_container: self.maximum_container })
    }
}
