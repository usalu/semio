//! 📬️ A resident stash keeps one reply-shaped child's answer and its pending state across turns.
use crate::{ValueError, retained_clone::{OriginalEffectReply, RetainedCloneStep}};

/// 📬️ The answer slot lent to a child producer; an unrecorded answer keeps the child pending until the next turn recovers it.
pub struct ReplyStash<E> {
    answer: Option<Result<RetainedCloneStep, E>>,
    pending: bool,
}

impl<E> Default for ReplyStash<E> {
    fn default() -> Self { Self { answer: None, pending: false } }
}

impl<E> ReplyStash<E> {
    pub fn pending(&self) -> bool { self.pending }

    /// 🎟️ Lends the slot with `recover` set once an earlier turn left the same child pending; `None` keeps that pending state.
    pub fn run(&mut self, call: impl for<'r> FnOnce(OriginalEffectReply<'r, RetainedCloneStep, E>, bool) -> Result<(), ValueError>) -> Result<Option<Result<RetainedCloneStep, E>>, ValueError> {
        if self.answer.is_none() { call(OriginalEffectReply::borrow(&mut self.answer)?, self.pending)?; }
        match self.answer.take() {
            None => { self.pending = true; Ok(None) }
            Some(answer) => { self.pending = false; Ok(Some(answer)) }
        }
    }
}
