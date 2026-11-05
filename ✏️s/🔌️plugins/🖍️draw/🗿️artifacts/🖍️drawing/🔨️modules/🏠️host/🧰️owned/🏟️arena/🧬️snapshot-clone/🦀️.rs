//! 🧬️ Drawing snapshot clone: the native clone authority, the clone work totals, the layer clone work authority and the initial snapshot clone grant.

use super::*;

/// ♻️ Rebuilt source, removed layer, reverse arena and forward arena.
pub type DrawingRebuiltLayerOwners = (DrawingNativeLayers, Option<DrawingLayerNode>, Vec<DrawingLayerNode>, Vec<DrawingLayerNode>);

pub struct DrawingNativeCloneAuthority<T: RetainedClone> {
    pub cursor: T::Cursor,
    pub value: std::mem::ManuallyDrop<Option<T>>,
    pub retirement: semio_framework_value::retained_clone::RetainedCloneClose,
    pub phase: u8,
    pub terminal: bool,
}

impl<T: RetainedClone> DrawingNativeCloneAuthority<T> {
    pub fn new() -> Self {
        Self {
            cursor: T::retained_clone_cursor(),
            value: std::mem::ManuallyDrop::new(None),
            retirement: Default::default(),
            phase: 0,
            terminal: false,
        }
    }

    /// 🧬️ Advances the sealed source under its actual next physical quote and preserves child work before failure.
    pub fn step(&mut self,source:RetainedCloneRef<'_,T>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,&'static str>{
        if self.terminal{return Ok(true);}
        if cx.should_yield(){return Ok(false);}
        let grant=cx.retained_grant();
        match self.phase{
            0=>{
                let demand=self.cursor.advance_demands(source,grant.maximum_copy_bytes).map_err(|_|"drawing-store.layer-clone-demand")?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(false);}
                let turn=self.cursor.advance(source,grant.maximum_copy_bytes,grant);cx.consume_fuel(turn.work);
                let step=turn.result.map_err(|_|"drawing-store.layer-native-clone")?;
                cx.consume_retained(step.progress()).map_err(|_|"drawing-store.layer-native-clone-grant")?;
                if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1;}
            }
            1=>{
                let demand=self.cursor.take_demand().map_err(|_|"drawing-store.layer-clone-take-demand")?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(false);}
                let turn=self.cursor.take(grant);cx.consume_fuel(turn.work);
                let Some((value,progress))=turn.result.map_err(|_|"drawing-store.layer-clone-take")? else{return Ok(false);};
                *self.value=Some(value);self.cursor.begin_close();self.phase=2;
                cx.consume_retained(progress).map_err(|_|"drawing-store.layer-clone-take-grant")?;
            }
            2=>{
                let turn=self.cursor.close_selected(grant.maximum_copy_bytes,grant);cx.consume_fuel(turn.work);
                let step=turn.result.map_err(|_|"drawing-store.layer-native-clone-close")?;
                cx.consume_retained(step.progress()).map_err(|_|"drawing-store.layer-native-clone-close-grant")?;
                if matches!(step,RetainedCloneStep::Complete(_))&&self.cursor.terminal_is_empty(){self.terminal=true;}
            }
            _=>return Err("drawing-store.layer-clone-phase")
        }
        Ok(self.terminal)
    }

    pub fn take(&mut self) -> Option<T> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.cursor.terminal_is_empty() && self.value.is_none() && self.retirement.is_empty()
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
        if !self.cursor.terminal_is_empty(){self.cursor.begin_close();return self.cursor.close_selected(grant.maximum_copy_bytes,grant);}
        if self.value.is_some(){
            let turn=self.retirement.begin_granted(&mut self.value,grant);
            return semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default())))};
        }
        self.retirement.step_granted(grant)
    }
    pub fn close_demands(&self,copy:usize)->Result<RetainedCloneGrant,semio_framework_value::ValueError>{
        let demand=if !self.cursor.terminal_is_empty(){self.cursor.close_demands(copy)?}
            else{self.retirement.next_owner_demands::<T>(self.value.is_some(),&None,copy)?};
        Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth})
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

pub type DrawingNativeLayers = semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>;

pub type DrawingNativeText = semio_framework_value::paged::PagedUtf8<{usize::MAX}>;

pub type DrawingNativeSegments = semio_framework_value_persistent::list::PagedList<PathSegment, {usize::MAX}>;

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

    pub fn vector<T, const N: usize>(value: &semio_framework_value_persistent::list::PagedList<T, N>) -> Result<(usize, usize), &'static str> {
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
