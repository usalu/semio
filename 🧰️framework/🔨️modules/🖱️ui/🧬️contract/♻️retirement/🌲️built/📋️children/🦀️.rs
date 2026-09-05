//! 📋️ Exact retained child-page transfer and nonblocking final reservation release.

use super::*;

pub(crate) enum BuiltChildRetirementNext {
    Pending,
    Node(Box<BuiltNode>),
    Complete,
}

impl BuiltChildrenIntoIter {
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
            if entry.epoch != key.epoch || !entry.reserved || entry.owner.is_some() { return Err("built child retirement reservation changed"); }
            authority.release(key);
            self.handback = None;
        }
        self.backing = None;
        Ok(BuiltChildRetirementNext::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_child_retirement_contention_retains_exact_page() {
        let node = BuiltNode::try_new("child", crate::Component::Separator(crate::SeparatorProps {})).unwrap();
        let mut children = BuiltChildren::default();
        children.try_push(node).unwrap();
        let mut iterator = children.into_iter();
        let key = iterator.handback.unwrap();
        let BuiltChildRetirementNext::Node(node) = iterator.try_next_or_release().unwrap() else { panic!("retained child must transfer") };
        let mut owner = crate::BuiltTreeRetirement::new(*node);
        for _ in 0..1_000 { if owner.close_step(1, 4096).unwrap().complete { break; } }
        assert!(owner.terminal_is_empty());
        let guard = BUILT_CHILD_RETIRE_AUTHORITY.lock().unwrap();
        assert!(matches!(iterator.try_next_or_release().unwrap(), BuiltChildRetirementNext::Pending));
        assert_eq!(iterator.handback, Some(key));
        assert!(iterator.backing.is_some());
        drop(guard);
        assert!(matches!(iterator.try_next_or_release().unwrap(), BuiltChildRetirementNext::Complete));
        assert!(iterator.handback.is_none() && iterator.backing.is_none());
        eprintln!("[DEBUG] built-child exact page: contention preserved reservation, retry released locally");
    }

    #[test]
    fn built_tree_retirement_preserves_foreign_queued_page_at_full_capacity() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap();
        let contract = &fixture["foreignPage"];
        let node = || BuiltNode::try_new("node", crate::Component::Separator(crate::SeparatorProps {})).unwrap();
        let mut foreign = BuiltChildren::default();
        foreign.try_push(node()).unwrap();
        let key = foreign.handback.unwrap();
        drop(foreign);
        let mut root = node();
        let owned_pages = contract["ownedPages"].as_u64().unwrap() as usize;
        assert_eq!(owned_pages + contract["queuedPages"].as_u64().unwrap() as usize, UI_BUILT_CHILD_RETIRE_SLOTS);
        for _ in 0..owned_pages {
            let mut parent = node();
            parent.rejected_children.try_push(root).unwrap();
            root = parent;
        }
        let mut owner = crate::BuiltTreeRetirement::new(root);
        for _ in 0..100_000 { if owner.close_step(1, 4096).unwrap().complete { break; } }
        assert!(owner.terminal_is_empty());
        let mut foreign = {
            let mut authority = BUILT_CHILD_RETIRE_AUTHORITY.lock().unwrap();
            let entry = &mut authority.slots[key.slot];
            assert_eq!(entry.epoch, key.epoch);
            assert!(entry.reserved);
            let foreign = entry.owner.take().expect("foreign queued page must remain owned");
            assert_eq!(foreign.cursor, contract["cursor"].as_u64().unwrap() as usize);
            assert_eq!(foreign.len, contract["nodes"].as_u64().unwrap() as usize);
            BuiltChildrenIntoIter { backing: Some(foreign.backing), len: foreign.len, cursor: foreign.cursor, handback: Some(key) }
        };
        let BuiltChildRetirementNext::Node(node) = foreign.try_next_or_release().unwrap() else { panic!("foreign child must remain available") };
        let mut owner = crate::BuiltTreeRetirement::new(*node);
        for _ in 0..1_000 { if owner.close_step(1, 4096).unwrap().complete { break; } }
        assert!(owner.terminal_is_empty());
        assert!(matches!(foreign.try_next_or_release().unwrap(), BuiltChildRetirementNext::Complete));
        eprintln!("[DEBUG] built-tree exact pages:383 fixture+1 foreign queued; foreign preserved and separately retired");
    }
}
