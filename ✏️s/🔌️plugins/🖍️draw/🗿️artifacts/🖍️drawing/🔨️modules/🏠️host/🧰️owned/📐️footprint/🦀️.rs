//! 📐️ Reads actual native retained text backing one chunk at a time.

use semio_framework_value::paged::PagedUtf8;
use crate::{DrawingImageAsset, DrawingLayerNode, DrawingSnapshot, FillStyle};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DrawingTextFootprint {
    pub(super) chunks: usize,
    pub(super) backing_bytes: usize,
}

#[derive(Default)]
pub(super) struct DrawingTextFootprintCursor {
    owner: Option<usize>,
    index: usize,
    totals: DrawingTextFootprint,
    phase: u8,
}

impl DrawingTextFootprintCursor {
    /// 🧷️ The caller retains this exact immutable native text owner until completion or close.
    pub(super) fn step<const N: usize>(&mut self, source: &PagedUtf8<N>, maximum_items: usize) -> Result<bool, &'static str> {
        if maximum_items == 0 { return Ok(self.phase == 2); }
        if self.phase == 3 { return Err("drawing-store.text-footprint-closed"); }
        let owner = source as *const _ as usize;
        if self.owner.is_some_and(|held| held != owner) { return Err("drawing-store.text-footprint-owner-changed"); }
        self.owner = Some(owner);
        let chunks = source.retained_chunks();
        match self.phase {
            0 => {
                self.totals.backing_bytes = chunks.allocated_bytes();
                self.phase = 1;
            }
            1 => {
                if let Some(chunk) = chunks.get(self.index) {
                    let bytes = self.totals.backing_bytes.checked_add(chunk.capacity()).ok_or("drawing-store.text-footprint-byte-overflow")?;
                    let index = self.index.checked_add(1).ok_or("drawing-store.text-footprint-index-overflow")?;
                    self.totals.backing_bytes = bytes;
                    self.totals.chunks = index;
                    self.index = index;
                } else { self.phase = 2; }
            }
            2 => {}
            _ => return Err("drawing-store.text-footprint-phase"),
        }
        Ok(self.phase == 2)
    }

    pub(super) fn totals(&self) -> Option<DrawingTextFootprint> { (self.phase == 2).then_some(self.totals) }

    pub(super) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 { return self.terminal_is_empty(); }
        self.owner = None;
        self.index = 0;
        self.totals = DrawingTextFootprint::default();
        self.phase = 3;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.index == 0 && self.totals == DrawingTextFootprint::default() }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DrawingLayerFootprint {
    pub(super) items: usize,
    pub(super) backing_bytes: usize,
}

#[derive(Default)]
pub(super) struct DrawingLayerFootprintCursor {
    owner: Option<usize>,
    field: usize,
    child: usize,
    text: DrawingTextFootprintCursor,
    totals: DrawingLayerFootprint,
    phase: u8,
}

impl DrawingLayerFootprintCursor {
    fn add(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.totals.items = self.totals.items.checked_add(items).ok_or("drawing-store.layer-footprint-item-overflow")?;
        self.totals.backing_bytes = self.totals.backing_bytes.checked_add(bytes).ok_or("drawing-store.layer-footprint-byte-overflow")?;
        Ok(())
    }

    fn list<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        self.add(value.capacity().checked_add(1).ok_or("drawing-store.layer-footprint-item-overflow")?, value.allocated_bytes())
    }

    fn lists(&mut self, source: &DrawingLayerNode) -> Result<(), &'static str> {
        self.add(1, 0)?;
        let base = crate::schema::layer_base(source);
        if let Some(FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. }) = base.attributes.fill.as_ref() { self.list(stops)?; }
        if let Some(dash) = base.attributes.stroke.as_ref().and_then(|stroke| stroke.dash.as_ref()) { self.list(dash)?; }
        match source {
            DrawingLayerNode::Shape(value) => { if let Some(polygon) = value.polygon.as_ref() { self.list(&polygon.points)?; } }
            DrawingLayerNode::Path(value) => self.list(&value.segments)?,
            DrawingLayerNode::Group(value) => self.list(&value.children)?,
            DrawingLayerNode::Boolean(value) => self.list(&value.children)?,
            _ => {}
        }
        Ok(())
    }

    fn field(source: &DrawingLayerNode, index: usize) -> Option<&PagedUtf8<{usize::MAX}>> {
        let base = crate::schema::layer_base(source);
        match index {
            0 => Some(&base.id),
            1 => Some(&base.name),
            2 => Some(&base.blend_mode),
            3 => match source {
                DrawingLayerNode::Shape(value) => Some(&value.shape_kind),
                DrawingLayerNode::Text(value) => Some(&value.content),
                DrawingLayerNode::Image(value) => Some(&value.image_key),
                DrawingLayerNode::Boolean(value) => Some(&value.operation),
                DrawingLayerNode::Trace(value) => Some(&value.source_key),
                _ => None,
            },
            _ => None,
        }
    }

    fn text(&mut self, value: &PagedUtf8<{usize::MAX}>) -> Result<bool, &'static str> {
        if !self.text.step(value, 1)? { return Ok(false); }
        let totals = self.text.totals().ok_or("drawing-store.layer-footprint-text-incomplete")?;
        self.add(totals.chunks.checked_add(1).ok_or("drawing-store.layer-footprint-item-overflow")?, totals.backing_bytes)?;
        self.text.close_step(1);
        self.text = DrawingTextFootprintCursor::default();
        Ok(true)
    }

    /// 🧷️ Reads only direct backing; the caller retains this immutable native layer until close.
    pub(super) fn step(&mut self, source: &DrawingLayerNode, maximum_items: usize) -> Result<bool, &'static str> {
        if maximum_items == 0 { return Ok(self.phase == 3); }
        if self.phase == 4 { return Err("drawing-store.layer-footprint-closed"); }
        let owner = source as *const _ as usize;
        if self.owner.is_some_and(|held| held != owner) { return Err("drawing-store.layer-footprint-owner-changed"); }
        self.owner = Some(owner);
        match self.phase {
            0 => { self.lists(source)?; self.phase = 1; }
            1 => {
                if self.field < 4 {
                    let complete = match Self::field(source, self.field) { Some(value) => self.text(value)?, None => true };
                    if complete { self.field += 1; }
                } else { self.phase = 2; }
            }
            2 => {
                if let DrawingLayerNode::Boolean(value) = source {
                    if let Some(child) = value.children.get(self.child) {
                        if self.text(child)? { self.child += 1; }
                    } else { self.phase = 3; }
                } else { self.phase = 3; }
            }
            3 => {}
            _ => return Err("drawing-store.layer-footprint-phase"),
        }
        Ok(self.phase == 3)
    }

    pub(super) fn totals(&self) -> Option<DrawingLayerFootprint> { (self.phase == 3).then_some(self.totals) }

    pub(super) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 { return self.terminal_is_empty(); }
        self.text.close_step(1);
        self.owner = None;
        self.field = 0;
        self.child = 0;
        self.totals = DrawingLayerFootprint::default();
        self.phase = 4;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.field == 0 && self.child == 0 && self.text.terminal_is_empty() && self.totals == DrawingLayerFootprint::default() }
}

#[derive(Clone, Copy)]
pub(super) enum DrawingRecordFootprintSource<'a> {
    Snapshot(&'a DrawingSnapshot),
    Asset(&'a PagedUtf8<{usize::MAX}>, &'a DrawingImageAsset),
}

impl<'a> DrawingRecordFootprintSource<'a> {
    fn identity(self) -> (u8, usize, usize) {
        match self {
            Self::Snapshot(value) => (0, value as *const _ as usize, 0),
            Self::Asset(key, value) => (1, key as *const _ as usize, value as *const _ as usize),
        }
    }

    fn field(self, index: usize) -> Option<&'a PagedUtf8<{usize::MAX}>> {
        match self {
            Self::Snapshot(value) => match index { 0 => Some(&value.schema), 1 => Some(&value.id), 2 => value.title.as_ref(), _ => None },
            Self::Asset(key, _) => if index==0 {Some(key)} else {None},
        }
    }
}

#[derive(Default)]
pub(super) struct DrawingRecordFootprintCursor {
    owner: Option<(u8, usize, usize)>,
    field: usize,
    text: DrawingTextFootprintCursor,
    totals: DrawingLayerFootprint,
    phase: u8,
}

impl DrawingRecordFootprintCursor {
    fn add(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.totals.items = self.totals.items.checked_add(items).ok_or("drawing-store.record-footprint-item-overflow")?;
        self.totals.backing_bytes = self.totals.backing_bytes.checked_add(bytes).ok_or("drawing-store.record-footprint-byte-overflow")?;
        Ok(())
    }

    /// 🧷️ The caller retains the exact immutable snapshot or key/asset pair until close.
    pub(super) fn step(&mut self, source: DrawingRecordFootprintSource<'_>, maximum_items: usize) -> Result<bool, &'static str> {
        if maximum_items == 0 { return Ok(self.phase == 2); }
        if self.phase == 3 { return Err("drawing-store.record-footprint-closed"); }
        let owner = source.identity();
        if self.owner.is_some_and(|held| held != owner) { return Err("drawing-store.record-footprint-owner-changed"); }
        self.owner = Some(owner);
        match self.phase {
            0 => {
                self.add(1, 0)?;
                if let DrawingRecordFootprintSource::Snapshot(value) = source {
                    self.add(value.layers.capacity().checked_add(1).ok_or("drawing-store.record-footprint-item-overflow")?, value.layers.allocated_bytes())?;
                    self.add(value.assets.retained_entries().capacity().checked_add(1).ok_or("drawing-store.record-footprint-item-overflow")?, value.assets.retained_entries().allocated_bytes())?;
                }
                if let DrawingRecordFootprintSource::Asset(_,value)=source{self.add(value.samples.capacity().checked_add(1).ok_or("drawing-store.record-footprint-item-overflow")?,value.samples.allocated_bytes())?;}
                self.phase = 1;
            }
            1 => {
                if self.field < 3 {
                    let complete = match source.field(self.field) {
                        Some(value) => {
                            if self.text.step(value, 1)? {
                                let totals = self.text.totals().ok_or("drawing-store.record-footprint-text-incomplete")?;
                                self.add(totals.chunks.checked_add(1).ok_or("drawing-store.record-footprint-item-overflow")?, totals.backing_bytes)?;
                                self.text.close_step(1);
                                self.text = DrawingTextFootprintCursor::default();
                                true
                            } else { false }
                        }
                        None => true,
                    };
                    if complete { self.field += 1; }
                } else { self.phase = 2; }
            }
            2 => {}
            _ => return Err("drawing-store.record-footprint-phase"),
        }
        Ok(self.phase == 2)
    }

    pub(super) fn totals(&self) -> Option<DrawingLayerFootprint> { (self.phase == 2).then_some(self.totals) }

    pub(super) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 { return self.terminal_is_empty(); }
        self.text.close_step(1);
        self.owner = None;
        self.field = 0;
        self.totals = DrawingLayerFootprint::default();
        self.phase = 3;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.field == 0 && self.text.terminal_is_empty() && self.totals == DrawingLayerFootprint::default() }
}

#[derive(Default)]
pub(super) struct DrawingTextDigestCursor {
    footprint: DrawingTextFootprintCursor,
    tag: Option<u16>,
    chunk: usize,
    offset: usize,
    phase: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DrawingTextDigestStep {
    pub(super) complete: bool,
    pub(super) observed_bytes: usize,
}

impl DrawingTextDigestCursor {
    /// 🧷️ Retains no source copy; the caller keeps the exact native field immutable until close.
    pub(super) fn step<const N: usize>(&mut self, source: &PagedUtf8<N>, tag: u16, maximum_items: usize, maximum_bytes: usize, mut observe: impl FnMut(&[u8])) -> Result<DrawingTextDigestStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(DrawingTextDigestStep { complete: self.phase == 3, observed_bytes: 0 }); }
        if self.phase == 4 { return Err("drawing-store.text-digest-closed"); }
        if self.tag.is_some_and(|held| held != tag) || self.footprint.owner.is_some_and(|held| held != source as *const _ as usize) {
            return Err("drawing-store.text-digest-owner-changed");
        }
        self.tag = Some(tag);
        let mut observed_bytes = 0;
        match self.phase {
            0 => { if self.footprint.step(source, 1)? { self.phase = 1; } }
            1 => {
                let mut header = [0u8; 11];
                header[0] = 0xd8;
                header[1..3].copy_from_slice(&tag.to_be_bytes());
                header[3..].copy_from_slice(&(source.len() as u64).to_be_bytes());
                observed_bytes = maximum_bytes.min(header.len() - self.offset);
                observe(&header[self.offset..self.offset + observed_bytes]);
                self.offset += observed_bytes;
                if self.offset == header.len() { self.offset = 0; self.phase = 2; }
            }
            2 => {
                if let Some(chunk) = source.retained_chunks().get(self.chunk) {
                    observed_bytes = maximum_bytes.min(chunk.len() - self.offset);
                    observe(&chunk.as_bytes()[self.offset..self.offset + observed_bytes]);
                    self.offset += observed_bytes;
                    if self.offset == chunk.len() { self.offset = 0; self.chunk += 1; }
                } else { self.phase = 3; }
            }
            3 => {}
            _ => return Err("drawing-store.text-digest-phase"),
        }
        Ok(DrawingTextDigestStep { complete: self.phase == 3, observed_bytes })
    }

    pub(super) fn totals(&self) -> Option<DrawingTextFootprint> { (self.phase == 3).then(|| self.footprint.totals()).flatten() }

    pub(super) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 { return self.terminal_is_empty(); }
        self.footprint.close_step(1);
        self.tag = None;
        self.chunk = 0;
        self.offset = 0;
        self.phase = 4;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.footprint.terminal_is_empty() && self.tag.is_none() && self.chunk == 0 && self.offset == 0 }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DrawingTextEqualityStep {
    pub(super) complete: bool,
    pub(super) compared_bytes: usize,
}

#[derive(Default)]
pub(super) struct DrawingTextEqualityCursor {
    owners: Option<(usize, usize)>,
    chunks: [usize; 2],
    offsets: [usize; 2],
    remaining: usize,
    result: Option<bool>,
    closed: bool,
}

impl DrawingTextEqualityCursor {
    /// 🧷️ Both exact immutable native fields remain retained by the caller until completion or close.
    pub(super) fn step<const L: usize, const R: usize>(&mut self, left: &PagedUtf8<L>, right: &PagedUtf8<R>, maximum_items: usize, maximum_bytes: usize) -> Result<DrawingTextEqualityStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(DrawingTextEqualityStep::default()); }
        if self.closed { return Err("drawing-store.text-equality-closed"); }
        let owners = (left as *const _ as usize, right as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.text-equality-owner-changed"); }
        if self.owners.is_none() {
            self.owners = Some(owners);
            self.remaining = left.len();
            if left.len() != right.len() { self.result = Some(false); }
            else if left.is_empty() { self.result = Some(true); }
            return Ok(DrawingTextEqualityStep { complete: self.result.is_some(), compared_bytes: 0 });
        }
        if self.result.is_some() { return Ok(DrawingTextEqualityStep { complete: true, compared_bytes: 0 }); }
        let left = left.retained_chunks().get(self.chunks[0]).ok_or("drawing-store.text-equality-left-chunk")?.as_bytes();
        let right = right.retained_chunks().get(self.chunks[1]).ok_or("drawing-store.text-equality-right-chunk")?.as_bytes();
        if self.offsets[0] == left.len() { self.chunks[0] += 1; self.offsets[0] = 0; return Ok(DrawingTextEqualityStep::default()); }
        if self.offsets[1] == right.len() { self.chunks[1] += 1; self.offsets[1] = 0; return Ok(DrawingTextEqualityStep::default()); }
        let bytes = (maximum_bytes / 2).min(left.len() - self.offsets[0]).min(right.len() - self.offsets[1]).min(self.remaining);
        if bytes == 0 { return Ok(DrawingTextEqualityStep::default()); }
        if left[self.offsets[0]..self.offsets[0] + bytes] != right[self.offsets[1]..self.offsets[1] + bytes] { self.result = Some(false); }
        self.offsets[0] += bytes;
        self.offsets[1] += bytes;
        self.remaining -= bytes;
        if self.remaining == 0 && self.result.is_none() { self.result = Some(true); }
        Ok(DrawingTextEqualityStep { complete: self.result.is_some(), compared_bytes: bytes * 2 })
    }

    pub(super) fn result(&self) -> Option<bool> { self.result }

    pub(super) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 { return self.terminal_is_empty(); }
        self.owners = None;
        self.chunks = [0; 2];
        self.offsets = [0; 2];
        self.remaining = 0;
        self.result = None;
        self.closed = true;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.owners.is_none() && self.chunks == [0; 2] && self.offsets == [0; 2] && self.remaining == 0 && self.result.is_none() }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DrawingDuplicateIdentityStep {
    pub(super) complete: bool,
    pub(super) observed_bytes: usize,
}

#[derive(Default)]
pub(super) struct DrawingDuplicateIdentityCursor {owner:Option<usize>,chunk:usize,offset:usize,complete:bool,closed:bool}
impl DrawingDuplicateIdentityCursor {
    /// 🧷️ Reads an exact immutable admitted target field without hashing or spelling it.
    pub(super) fn step<const N:usize>(&mut self,target:&PagedUtf8<N>,maximum_items:usize,maximum_bytes:usize)->Result<DrawingDuplicateIdentityStep,&'static str>{
      if maximum_items==0||maximum_bytes==0{return Ok(Default::default());}if self.closed{return Err("drawing-store.duplicate-identity-closed");}
      let owner=target as *const _ as usize;if self.owner.is_some_and(|held|held!=owner){return Err("drawing-store.duplicate-identity-owner-changed");}self.owner=Some(owner);
      if self.complete{return Ok(DrawingDuplicateIdentityStep{complete:true,observed_bytes:0});}
      let Some(chunk)=target.retained_chunks().get(self.chunk)else{self.complete=true;return Ok(DrawingDuplicateIdentityStep{complete:true,observed_bytes:0});};
      let end=chunk.len().min(self.offset.saturating_add(maximum_bytes));let observed_bytes=end-self.offset;self.offset=end;if self.offset==chunk.len(){self.chunk+=1;self.offset=0;}Ok(DrawingDuplicateIdentityStep{complete:false,observed_bytes})
    }
    pub(super) fn identity<'a,const N:usize>(&self,target:&'a PagedUtf8<N>)->Option<&'a PagedUtf8<N>>{(self.complete&&self.owner==Some(target as *const _ as usize)).then_some(target)}
    pub(super) fn close_step(&mut self,maximum_items:usize)->bool{if maximum_items==0{return self.terminal_is_empty();}self.owner=None;self.chunk=0;self.offset=0;self.complete=false;self.closed=true;true}
    pub(super) fn terminal_is_empty(&self)->bool{self.owner.is_none()&&self.chunk==0&&self.offset==0&&!self.complete}
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
