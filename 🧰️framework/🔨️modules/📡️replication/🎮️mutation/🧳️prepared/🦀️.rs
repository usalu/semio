//! 🧳️ Neutral prepared operation owners retain publication, inverse, diagnostics and independent refusals.
use super::{MutationMessage,MutationApplyError};
use semio_framework_value::ValueError;
use std::sync::Arc;
/// 🧳️ Semantic owners produced against the immutable base; inverse and apply refusals are independent.
pub struct ArtifactReplayPrepared<P, M> {
    pub next: Option<Arc<P>>,
    pub inverse: Result<semio_framework_value::list::PagedList<M, {usize::MAX}>, ValueError>,
    pub messages: Vec<MutationMessage>,
    pub apply_refusal: Option<MutationApplyError>,
    pub input_refusal: Option<String>,
    pub foreign_steps: bool,
}

#[path="♻️retirement/🦀️.rs"]
pub mod retirement;
