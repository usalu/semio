//! 🧱️ Drawing container rebuild: the move capacity, the rebuild move and the container rebuild authority.

use super::*;

const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY: usize = DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY * 4 + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingContainerRebuildMove {
    Empty,
    SourceToReverse,
    PendingToOutput,
    ReverseToOutput,
    ReverseToRemoved,
    OutputToReverse,
    ReverseToSource,
}

pub struct DrawingContainerRebuildAuthority {
    pub source: std::mem::ManuallyDrop<Option<DrawingNativeLayers>>,
    pub reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub pending: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    pub removed: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    pub moves: [DrawingContainerRebuildMove; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
    pub move_count: usize,
    pub rollback_cursor: Option<usize>,
    pub remove_index: Option<usize>,
    pub insert_index: Option<usize>,
    pub original_index: usize,
    pub phase: u8,
    pub terminal: bool,
}

pub struct DrawingContainerRebuildRejected {
    pub source: DrawingNativeLayers,
    pub pending: Option<DrawingLayerNode>,
    pub reverse: Vec<DrawingLayerNode>,
    pub output: Vec<DrawingLayerNode>,
}

impl DrawingContainerRebuildAuthority {
    pub fn new(
        source: DrawingNativeLayers,
        remove_index: Option<usize>,
        insert_index: Option<usize>,
        pending: Option<DrawingLayerNode>,
        reverse: Vec<DrawingLayerNode>,
        output: Vec<DrawingLayerNode>,
        workset: DrawingMutationWorksetPlan,
    ) -> Result<Self, DrawingContainerRebuildRejected> {
        let extra = usize::from(pending.is_some());
        let Some(output_capacity) = source.len().saturating_sub(usize::from(remove_index.is_some())).checked_add(extra) else {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        };
        if output_capacity > DRAWING_MAXIMUM_NESTED_ITEMS
            || source.len().saturating_add(output_capacity) > workset.container_slots
            || source.len() > workset.maximum_container.saturating_add(1)
            || output_capacity > workset.maximum_container.saturating_add(1)
            || source.capacity() < output_capacity
            || reverse.capacity() < source.len().max(output_capacity)
            || output.capacity() < output_capacity
            || !reverse.is_empty()
            || !output.is_empty()
        {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        }
        Ok(Self {
            source: std::mem::ManuallyDrop::new(Some(source)),
            reverse: std::mem::ManuallyDrop::new(Some(reverse)),
            output: std::mem::ManuallyDrop::new(Some(output)),
            pending: std::mem::ManuallyDrop::new(pending),
            removed: std::mem::ManuallyDrop::new(None),
            moves: [DrawingContainerRebuildMove::Empty; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
            move_count: 0,
            rollback_cursor: None,
            remove_index,
            insert_index,
            original_index: 0,
            phase: 0,
            terminal: false,
        })
    }

    pub fn reserve_move(&self) -> Result<(), &'static str> {
        self.moves.get(self.move_count).map(|_| ()).ok_or("drawing-store.container-move-capacity")
    }

    pub fn record_reserved_move(&mut self, value: DrawingContainerRebuildMove) {
        self.moves[self.move_count] = value;
        self.move_count += 1;
    }

    pub fn advance(&mut self) -> Result<(bool, u64), &'static str> {
        if self.terminal {
            return Ok((true, 0));
        }
        if self.rollback_cursor.is_some() {
            return Err("drawing-store.container-advance-after-rollback");
        }
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-owner-missing");
        }
        if self.phase == 0 {
            if !self.source.as_ref().expect("validated Drawing source remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.source.as_mut().expect("validated Drawing source remains retained").pop().expect("nonempty Drawing source yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::SourceToReverse);
                return Ok((false, 1));
            }
            self.phase = 1;
            return Ok((false, 0));
        }
        if self.phase == 1 {
            if self.pending.is_some() && self.insert_index.is_some_and(|index| index.min(self.reverse.as_ref().map_or(0, Vec::len) + self.original_index) == self.output.as_ref().map_or(0, Vec::len)) {
                self.reserve_move()?;
                self.output.as_mut().expect("validated Drawing output remains retained").push(self.pending.take().expect("Drawing insertion owner remains retained"));
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
                if self.remove_index == Some(self.original_index) && self.removed.is_some() {
                    return Err("drawing-store.container-duplicate-removal");
                }
                self.reserve_move()?;
                let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
                if self.remove_index == Some(self.original_index) {
                    *self.removed = Some(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToRemoved);
                } else {
                    self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToOutput);
                }
                self.original_index += 1;
                return Ok((false, 1));
            }
            if self.pending.is_some() {
                self.reserve_move()?;
                let value = self.pending.take().expect("validated Drawing pending owner remains retained");
                self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            self.phase = 2;
            return Ok((false, 0));
        }
        if self.phase == 2 {
            if !self.output.as_ref().expect("validated Drawing output remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.output.as_mut().expect("validated Drawing output remains retained").pop().expect("nonempty Drawing output yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::OutputToReverse);
                return Ok((false, 1));
            }
            self.phase = 3;
            return Ok((false, 0));
        }
        if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
            self.reserve_move()?;
            let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
            self.source.as_mut().expect("validated Drawing source remains retained").push_reserved(value).unwrap_or_else(|_| panic!("admitted native Drawing source slot remains reserved"));
            self.record_reserved_move(DrawingContainerRebuildMove::ReverseToSource);
            return Ok((false, 1));
        }
        self.terminal = true;
        Ok((true, 0))
    }

    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let (complete, fuel) = self.advance()?;
        if fuel > 0 {
            cx.consume_fuel(fuel);
        }
        Ok(complete)
    }

    pub fn close_forward_step(&mut self) -> Result<bool, &'static str> {
        self.advance().map(|(complete, _)| complete)
    }

    pub fn take(&mut self) -> Option<DrawingRebuiltLayerOwners> {
        self.terminal.then(|| {
            (
                self.source.take().expect("Drawing rebuilt source container remains retained"),
                self.removed.take(),
                self.reverse.take().expect("Drawing emptied reverse arena remains retained"),
                self.output.take().expect("Drawing emptied output arena remains retained"),
            )
        })
    }

    pub fn rollback_step(&mut self) -> Result<bool, &'static str> {
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-rollback-owner-missing");
        }
        let cursor = *self.rollback_cursor.get_or_insert(self.move_count);
        if cursor == 0 {
            self.phase = 0;
            self.original_index = 0;
            return Ok(true);
        }
        let index = cursor - 1;
        match self.moves[index] {
            DrawingContainerRebuildMove::SourceToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.source.as_mut().ok_or("drawing-store.container-source")?.push_reserved(value).unwrap_or_else(|_| panic!("native Drawing rollback source slot remains reserved"));
            }
            DrawingContainerRebuildMove::PendingToOutput => {
                if self.pending.is_some() {
                    return Err("drawing-store.container-rollback-pending");
                }
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                *self.pending = Some(value);
            }
            DrawingContainerRebuildMove::ReverseToOutput => {
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToRemoved => {
                let value = self.removed.take().ok_or("drawing-store.container-rollback-removed")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::OutputToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.output.as_mut().ok_or("drawing-store.container-output")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToSource => {
                let value = self.source.as_mut().ok_or("drawing-store.container-source")?.pop().ok_or("drawing-store.container-rollback-source")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::Empty => return Err("drawing-store.container-rollback-empty-move"),
        }
        self.moves[index] = DrawingContainerRebuildMove::Empty;
        self.rollback_cursor = Some(index);
        Ok(false)
    }

    pub fn rollback_complete(&self) -> bool {
        self.rollback_cursor == Some(0)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn recorded_move_in_phase(&self, phase: u8) -> bool {
        let Some(index) = self.move_count.checked_sub(1) else { return false };
        self.phase == phase
            && match phase {
                0 => self.moves[index] == DrawingContainerRebuildMove::SourceToReverse,
                1 => matches!(self.moves[index], DrawingContainerRebuildMove::PendingToOutput | DrawingContainerRebuildMove::ReverseToOutput | DrawingContainerRebuildMove::ReverseToRemoved),
                2 => self.moves[index] == DrawingContainerRebuildMove::OutputToReverse,
                3 => self.moves[index] == DrawingContainerRebuildMove::ReverseToSource,
                _ => false,
            }
    }

    pub fn finish_handoff(&mut self) -> Result<(), &'static str> {
        if !(self.terminal || self.rollback_complete()) || self.source.is_some() || self.reverse.is_some() || self.output.is_some() || self.pending.is_some() || self.removed.is_some() {
            return Err("drawing-store.container-rollback-handoff-incomplete");
        }
        self.terminal = true;
        Ok(())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.source.is_none() && self.reverse.is_none() && self.output.is_none() && self.pending.is_none() && self.removed.is_none()
    }
}

impl Drop for DrawingContainerRebuildAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing container rebuild reached Drop before exact handoff or cursor retirement");
    }
}
