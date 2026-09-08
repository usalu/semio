//! 🚫️ Exact ownership of both rejected peer identity bytes and its typed presence value.

use super::*;

//#region 🚫️AdmissionOwner
pub struct PresencePeerAdmissionRejected<P> {
    pub reason: &'static str,
    actor: std::mem::ManuallyDrop<String>,
    presence: std::mem::ManuallyDrop<Option<P>>,
    factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
}

impl<P> PresencePeerAdmissionRejected<P> {
    pub(super) fn new(reason: &'static str, actor: String, presence: P, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { reason, actor: std::mem::ManuallyDrop::new(actor), presence: std::mem::ManuallyDrop::new(Some(presence)), factory: std::mem::ManuallyDrop::new(Some(factory)) }
    }

    pub fn actor(&self) -> &str {
        self.actor.as_str()
    }

    pub fn presence(&self) -> &P {
        self.presence.as_ref().expect("rejected admission retains its exact presence")
    }

    pub fn into_retirement(mut self) -> Box<dyn ErasedSnapshotRetirement>
    where
        P: Send + Sync + 'static,
    {
        let actor = std::mem::take(&mut *self.actor);
        let presence = self.presence.take();
        let factory = self.factory.take().expect("rejected admission retains its minting publication factory");
        Box::new(PresencePeerRejectionRetirement { actor: std::mem::ManuallyDrop::new(Some(actor.into_bytes())), presence: std::mem::ManuallyDrop::new(presence), active: std::mem::ManuallyDrop::new(None), factory })
    }
}

impl<P> Drop for PresencePeerAdmissionRejected<P> {
    fn drop(&mut self) {
        let terminal = self.actor.is_empty() && self.actor.capacity() == 0 && self.presence.is_none() && self.factory.is_none();
        if !std::thread::panicking() {
            assert!(terminal, "rejected peer admission requires exact actor and presence owner transfer");
        }
    }
}
//#endregion 🚫️AdmissionOwner

//#region 🧹️RetainedRejection
pub(super) struct PresencePeerRejectionRetirement<P> {
    actor: std::mem::ManuallyDrop<Option<Vec<u8>>>,
    presence: std::mem::ManuallyDrop<Option<P>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory: Arc<dyn SnapshotRetirementFactory<P>>,
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresencePeerRejectionRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(actor) = self.actor.as_mut() {
            if actor.is_empty() {
                drop(self.actor.take());
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let released_bytes = actor.len().min(maximum_bytes);
            actor.truncate(actor.len() - released_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("rejected peer presence reported Complete without an exact empty witness".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("rejected peer presence exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(presence) = self.presence.take() {
            *self.active = Some(self.factory.retire(Arc::new(presence)));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.actor.is_none() && self.presence.is_none() && self.active.is_none()
    }
}

impl<P> Drop for PresencePeerRejectionRetirement<P> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.actor.is_none() && self.presence.is_none() && self.active.is_none(), "rejected peer cursor requires its exact empty terminal witness");
        }
    }
}
//#endregion 🧹️RetainedRejection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
