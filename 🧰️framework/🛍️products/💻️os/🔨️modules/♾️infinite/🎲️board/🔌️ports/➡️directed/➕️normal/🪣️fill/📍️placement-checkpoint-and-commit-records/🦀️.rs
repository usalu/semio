//! 📍️ Placement, checkpoint and commit records of the board fill: the placement and checkpoint impls and drops, the fill result and the commit handle, shape, placement and candidate records. See [`super`].

use super::*;

impl BoardFillPlacement {
    pub fn handle_count(&self) -> usize {
        self.handles.len
    }

    pub fn handle(&self, index: usize) -> Option<(&str, &str, f64, Option<f64>)> {
        let handle = self.handles.get(index)?;
        Some((handle.id.as_str(), handle.template.handle_kind.as_str(), handle.template.angle, handle.template.radius))
    }

    pub fn fixed_handle_id(&self, index: usize) -> Option<&BoardFillText> {
        self.handles.get(index).map(|handle| &handle.id)
    }

    pub fn fixed_handle_kind(&self, index: usize) -> Option<&BoardFillText> {
        self.handles.get(index).map(|handle| &handle.template.handle_kind)
    }

    pub fn fixed_handle_angle(&self, index: usize) -> Option<f64> {
        self.handles.get(index).map(|handle| handle.template.angle)
    }

    pub fn fixed_handle_radius(&self, index: usize) -> Option<Option<f64>> {
        self.handles.get(index).map(|handle| handle.template.radius)
    }

    pub fn next_close_release_byte_demand(&self)->usize {self.handles.next_release_bytes()}

    pub fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::RetainedCloneProgress;
        if self.terminal_is_empty(){return InteractiveJobCloseStep::Complete{progress:Default::default()};}
        if grant.maximum_items==0||grant.maximum_depth==0{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
        if self.handles.pop().is_some(){return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if let Some(bytes)=self.handles.retained_page_bytes(){
            if bytes>grant.maximum_release_bytes{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
            let Some(released_bytes)=self.handles.retire_empty_page()else{return InteractiveJobCloseStep::Pending{progress:Default::default()};};
            return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}};
        }
        self.handles.retire_descriptor(grant).unwrap_or(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handles.terminal_is_empty()
    }
}

impl Drop for BoardFillPlacement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Board fill placement must reach exact terminal-empty before Drop");
    }
}

/// 💾 Exact typed checkpoint root moved out of the job until the mounted caller adopts it.
pub struct BoardFillCheckpoint {
        original_source: Option<semio_framework_action_bus::OriginalToolSourceBinding>,
pub operation: semio_framework_job::Operation,
    pub naming: &'static BoardNaming,
    pub state: Option<BoardFillJobState>,
}

impl BoardFillCheckpoint {
    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub fn take_pending_placement(&mut self) -> Option<BoardFillPlacement> {
        self.state.as_mut()?.pending_placement.take()
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn put_pending_placement(&mut self, placement: BoardFillPlacement) -> Result<(), BoardFillPlacement> {
        let Some(state) = self.state.as_mut() else { return Err(placement) };
        if state.pending_placement.is_some() {
            return Err(placement);
        }
        state.pending_placement = Some(placement);
        Ok(())
    }

    pub fn pending_placement(&self) -> Option<&BoardFillPlacement> {
        self.state.as_ref()?.pending_placement.as_ref()
    }

    pub fn accepted_count(&self) -> u32 {
        self.state.as_ref().map_or(0, |state| state.accepted_count)
    }

    pub fn into_closing_job(mut self) -> BoardFillJob {
        BoardFillJob {  original_source: self.original_source, operation: self.operation, naming: self.naming, state: self.state.take(), checkpoint: None, preview: None, commit_encoder: None, commit_writer: None, lent: None, commit_output: None, offer: Cell::new(None), admitted: Cell::new(false), fault: None, closing: true }
    }
}

impl Drop for BoardFillCheckpoint {
    fn drop(&mut self) {
        assert!(self.state.is_none() || std::thread::panicking(), "Board fill checkpoint must transfer or close its exact state before Drop");
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillResult {
    pub accepted_count: u32,
    pub stalled: bool,
    pub search_count: u64,
}

/// 🧩️ Fixed handle carried by the exact terminal fill candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCommitHandle {
    pub id: BoardFillText,
    pub handle_kind: BoardFillText,
    pub angle: f64,
    pub radius: Option<f64>,
}

/// 🔷️ Fixed shape carried by the exact terminal fill candidate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCommitShape {
    Circle,
    Rectangle,
}

/// 📦️ Full bounded typed placement carried by the exact terminal candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCommitPlacement {
    pub node_id: BoardFillText,
    pub edge_id: BoardFillText,
    pub edge_kind: BoardFillText,
    pub node_kind: BoardFillText,
    pub source_handle_id: BoardFillText,
    pub target_handle_id: BoardFillText,
    pub target_handle_index: usize,
    pub x: f64,
    pub y: f64,
    pub shape: BoardFillCommitShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon_kind: Option<BoardFillText>,
    pub handles: [Option<BoardFillCommitHandle>; BOARD_FILL_KIND_HANDLE_CAPACITY],
    pub handle_count: usize,
}

/// 🎯️ Typed terminal projection decoded while the retained payload remains authoritative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCommitCandidate {
    pub result: BoardFillResult,
    pub placement: Option<BoardFillCommitPlacement>,
}
