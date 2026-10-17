//! 🧬️ Drawing snapshot clone: the native clone authority, the clone work totals, the layer clone work authority and the initial snapshot clone grant.

use super::*;

/// ♻️ Rebuilt source, removed layer, reverse arena and forward arena.
pub type DrawingRebuiltLayerOwners = (DrawingNativeLayers, Option<DrawingLayerNode>, Vec<DrawingLayerNode>, Vec<DrawingLayerNode>);

pub struct DrawingNativeCloneAuthority<T: RetainedClone> {
    pub cursor: T::Cursor,
    pub value: std::mem::ManuallyDrop<Option<T>>,
    pub retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>>,
    pub phase: u8,
    pub terminal: bool,
}

impl<T: RetainedClone> DrawingNativeCloneAuthority<T> {
    pub fn new() -> Self {
        Self {
            cursor: T::retained_clone_cursor(),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            terminal: false,
        }
    }

    /// 🧬️ Clones from a sealed original source ref; every turn is funded by the job's retained wallet.
    pub fn step(&mut self, source: RetainedCloneRef<'_, T>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal { return Ok(true); }
        let grant = cx.retained_grant();
        match self.phase {
            0 => {
                let step = self.cursor.advance(source, grant).map_err(|_| "drawing-store.layer-native-clone")?;
                cx.consume_retained(step.progress()).map_err(|_| "drawing-store.layer-native-clone-grant")?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 1; }
            }
            1 => {
                let bytes = size_of::<T>();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes { return Ok(false); }
                *self.value = self.cursor.take();
                if self.value.is_none() { return Err("drawing-store.layer-clone-false-terminal"); }
                self.cursor.begin_close();
                self.phase = 2;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }).map_err(|_| "drawing-store.layer-clone-take-grant")?;
                cx.consume_fuel(bytes.max(1) as u64);
            }
            2 => {
                let step = self.cursor.close_step(grant).map_err(|_| "drawing-store.layer-native-clone-close")?;
                cx.consume_retained(step.progress()).map_err(|_| "drawing-store.layer-native-clone-close-grant")?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) && self.cursor.terminal_is_empty() { self.terminal = true; }
            }
            _ => return Err("drawing-store.layer-clone-phase"),
        }
        Ok(self.terminal)
    }

    pub fn take(&mut self) -> Option<T> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.cursor.terminal_is_empty() && self.value.is_none() && self.retirement.is_none()
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::ValueError>{
        if !self.cursor.terminal_is_empty(){self.cursor.begin_close();return self.cursor.close_step(grant);}
        if self.value.is_some(){
            if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<T>(){return Ok(RetainedCloneStep::Progress(Default::default()));}
            let value=self.value.take().unwrap();match semio_framework_value::retirement::controlled::ControlledRetirement::new(value){Ok(owner)=>*self.retirement=Some(owner),Err((error,value))=>{*self.value=Some(value);return Err(error);}}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}));
        }
        if let Some(owner)=self.retirement.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.retirement.take();}return Ok(step);}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn close_demands(&self,copy:usize)->Result<RetainedCloneGrant,semio_framework_value::ValueError>{
        if !self.cursor.terminal_is_empty(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.cursor.next_close_copy_byte_demand()?,maximum_capacity_bytes:self.cursor.next_close_capacity_byte_demand(copy)?,maximum_release_bytes:self.cursor.next_close_release_byte_demand()?,maximum_depth:self.cursor.next_close_depth_demand()?});}
        if self.value.is_some(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1});}
        if let Some(owner)=self.retirement.as_ref(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand()?,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy)?,maximum_release_bytes:owner.next_release_byte_demand()?,maximum_depth:owner.next_depth_demand()?});}
        Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0})
    }
}

impl<T: RetainedClone> Drop for DrawingNativeCloneAuthority<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing layer clone reached Drop before exact handoff or cursor retirement");
    }
}

pub type DrawingLayerCloneAuthority = DrawingNativeCloneAuthority<DrawingLayerNode>;

pub type DrawingFillCloneAuthority = DrawingNativeCloneAuthority<FillStyle>;

pub type DrawingStrokeCloneAuthority = DrawingNativeCloneAuthority<StrokeStyle>;

pub type DrawingNativeLayers = semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>;

pub type DrawingNativeText = semio_framework_value::paged::PagedUtf8<{usize::MAX}>;

pub type DrawingNativeSegments = semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>;

pub type DrawingTextCloneAuthority = DrawingNativeCloneAuthority<DrawingNativeText>;

pub type DrawingSegmentsCloneAuthority = DrawingNativeCloneAuthority<DrawingNativeSegments>;

impl DrawingNativeCloneAuthority<DrawingLayerNode> {
    pub fn target_at_mut<'a>(root: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
        if let Some((head, tail)) = path.split_first() {
            let DrawingLayerNode::Group(group) = root else { return None };
            return Self::target_at_mut(group.children.get_mut(*head)?, tail);
        }
        Some(root)
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingCloneWorkTotals {
    pub items: usize,
    pub bytes: usize,
}

pub struct DrawingLayerCloneWorkAuthority {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub items: usize,
    pub bytes: usize,
    pub direct: native_text_footprint::DrawingLayerFootprintCursor,
    pub terminal: bool,
}

impl DrawingLayerCloneWorkAuthority {
    pub fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], items: 1, bytes: size_of::<DrawingLayerNode>(), direct: native_text_footprint::DrawingLayerFootprintCursor::default(), terminal: false }
    }

    pub fn add(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("drawing-store.mutation-clone-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("drawing-store.mutation-clone-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-clone-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-clone-byte-capacity");
        }
        Ok(())
    }

    pub fn vector<T, const N: usize>(value: &semio_framework_value::list::PagedList<T, N>) -> Result<(usize, usize), &'static str> {
        Ok((1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-clone-item-overflow")?, value.allocated_bytes()))
    }

    pub fn step(&mut self, root: &DrawingLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let layer = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.mutation-clone-work-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            let complete = self.direct.step(layer, 1)?;
            cx.consume_fuel(1);
            if !complete { return Ok(false); }
            let totals = self.direct.totals().ok_or("drawing-store.mutation-clone-work-incomplete")?;
            self.add(totals.items, totals.backing_bytes)?;
            self.direct.close_step(1);
            self.direct = native_text_footprint::DrawingLayerFootprintCursor::default();
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        if let DrawingLayerNode::Group(value) = layer {
            if frame.child < value.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                    return Err("drawing-store.mutation-clone-work-depth-capacity");
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
            self.terminal = true;
            Ok(true)
        } else {
            self.depth -= 1;
            cx.consume_fuel(1);
            Ok(false)
        }
    }

    pub fn totals(&self) -> Option<DrawingCloneWorkTotals> {
        self.terminal.then_some(DrawingCloneWorkTotals { items: self.items, bytes: self.bytes })
    }
}

pub fn drawing_fill_clone_work_totals(value: &FillStyle) -> Result<DrawingCloneWorkTotals, &'static str> {
    let (items, bytes) = match value {
        FillStyle::Solid { .. } => (0, 0),
        FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => DrawingLayerCloneWorkAuthority::vector(stops)?,
    };
    Ok(DrawingCloneWorkTotals { items, bytes })
}

pub fn drawing_stroke_clone_work_totals(value: &StrokeStyle) -> Result<DrawingCloneWorkTotals, &'static str> {
    let mut items = 0usize;
    let mut bytes = 0usize;
    if let Some(dash) = value.dash.as_ref() {
        let (dash_items, dash_bytes) = DrawingLayerCloneWorkAuthority::vector(dash)?;
        items = items.checked_add(dash_items).ok_or("drawing-store.mutation-clone-item-overflow")?;
        bytes = bytes.checked_add(dash_bytes).ok_or("drawing-store.mutation-clone-byte-overflow")?;
    }
    Ok(DrawingCloneWorkTotals { items, bytes })
}

pub fn clone_drawing_string(source: &str) -> Result<String, &'static str> {
    if source.len() > DRAWING_OWNED_FIELD_BYTES {
        return Err("drawing-store.initializer-field-too-large");
    }
    let mut value = String::new();
    value.try_reserve_exact(source.len()).map_err(|_| "drawing-store.initializer-string-admission")?;
    value.push_str(source);
    Ok(value)
}

pub fn initial_snapshot_clone_grant(turn: usize) -> RetainedCloneGrant {
    match turn % 3 {
        0 => RetainedCloneGrant::one_capacity_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
        1 => RetainedCloneGrant::one_payload_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
        _ => RetainedCloneGrant::one_release_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
    }
}
