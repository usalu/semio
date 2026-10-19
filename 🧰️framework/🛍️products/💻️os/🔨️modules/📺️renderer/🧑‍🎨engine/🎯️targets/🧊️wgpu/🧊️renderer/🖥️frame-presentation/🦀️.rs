//! 🖥️ Frame presentation: frame preparation, presenter state and the presenter implementation.
use super::*;

#[path = "🎞️present-state-and-engine-closers/🦀️.rs"]
mod present_state_and_engine_closers;
pub use present_state_and_engine_closers::{
    note_present_stall_signature, AppPresentInputWait, AppPresentPhase, AppPresentProgress,
    AppPresentStallWatch, AppPresentStep, AppPresenter, APP_PRESENT_STALL_STEPS,
};
use present_state_and_engine_closers::{
    AppPresentCursor, AppPresentedRetirement, AppSurfaceResizeCursor, AppSurfaceResizePhase,
    RasterCandidateRetirement,
};

#[path = "🧯️retirement-and-present-steps/🦀️.rs"]
mod retirement_and_present_steps;

#[path = "🎬️present-step-inner/🦀️.rs"]
mod present_step_inner_child;
