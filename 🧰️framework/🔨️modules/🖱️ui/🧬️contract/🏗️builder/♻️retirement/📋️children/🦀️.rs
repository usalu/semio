//! 📋️ Exact retained child-page transfer and nonblocking final reservation release.

use super::*;

pub(crate) enum BuiltChildRetirementNext {
    Pending,
    Node(Box<BuiltNode>),
    Complete,
}

impl BuiltChildrenIntoIter {
    pub(crate) fn original_release_demand(&self)->usize{if self.cursor<self.len{0}else{self.backing.as_ref().map_or(0,|backing|std::mem::size_of_val(backing.as_ref()))}}
    pub(crate) fn original_terminal_is_empty(&self)->bool{self.cursor==self.len&&self.backing.is_none()&&self.handback.is_none()}

    pub(crate) fn try_next_or_release(&mut self) -> Result<BuiltChildRetirementNext, &'static str> {
        if self.cursor < self.len {
            let node = self.backing.as_mut().and_then(|backing| backing.get_mut(self.cursor)).and_then(Option::take).ok_or("built child retirement lost its next node")?;
            self.cursor += 1;
            return Ok(BuiltChildRetirementNext::Node(node));
        }
        if let Some(key) = self.handback {
            let mut authority = match BUILT_CHILD_RETIRE_AUTHORITY.try_lock() {
                Ok(authority) => authority,
                Err(std::sync::TryLockError::WouldBlock) => return Ok(BuiltChildRetirementNext::Pending),
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("built child retirement authority is poisoned"),
            };
            let entry = &authority.slots[key.slot];
            if entry.epoch != key.epoch || !entry.reserved || entry.owner.is_some() {
                return Err("built child retirement reservation changed");
            }
            authority.release(key);
            self.handback = None;
        }
        self.backing = None;
        Ok(BuiltChildRetirementNext::Complete)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
