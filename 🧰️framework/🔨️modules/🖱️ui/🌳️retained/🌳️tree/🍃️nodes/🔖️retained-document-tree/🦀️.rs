//! 🔖️ Retained document tree: the document tree faults, the page rejection and the `UiDocumentTree` with its node table, lease and surface. See [`super`].

use super::*;

// #region tree
//#region 🔖️RetainedDocument
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentTreeFault {
    Generation,
    Revision,
    PageOrder,
    DuplicateNode,
    NodeCapacity,
    Count,
    MissingRoot,
    MissingChild,
    MultipleParents,
    Cycle,
    Credits,
}

#[derive(Debug)]
pub struct UiDocumentPageRejection {
    pub fault: UiDocumentTreeFault,
    pub generation: u64,
    pub revision: UiRevision,
    pub index: usize,
    pub record: UiNodeRecord,
}

#[derive(Debug)]
pub struct UiDocumentTree {
    pub generation: u64,
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: UiNodeId,
    pub layout_epoch: u64,
    pub node_count: usize,
    pub nodes: UiNodeTable,
}

impl UiDocumentTree {
    pub fn new(header: UiDocumentLeaseHeader) -> Result<Self, UiDocumentTreeFault> {
        if header.generation == 0 || header.node_count > UI_DOCUMENT_NODES {
            return Err(UiDocumentTreeFault::Generation);
        }
        Ok(Self { generation: header.generation, surface: header.surface, revision: header.revision, root: header.root, layout_epoch: header.layout_epoch, node_count: header.node_count, nodes: UiNodeTable::default() })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn surface(&self) -> &SurfaceId {
        &self.surface
    }

    pub fn revision(&self) -> UiRevision {
        self.revision
    }

    pub fn layout_epoch(&self) -> u64 {
        self.layout_epoch
    }

    pub fn root_id(&self) -> UiNodeId {
        self.root
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// 🪙 Starts an independently credited copy of this admitted document. Records are copied by
    /// [`UiDocumentTree::credited_record_at`] one bounded unit at a time; this header alone owns no
    /// component, binding, or menu payload credits.
    pub fn credited_baseline(&self) -> Self {
        Self { generation: self.generation, surface: self.surface.clone(), revision: self.revision, root: self.root, layout_epoch: self.layout_epoch, node_count: self.node_count, nodes: UiNodeTable::default() }
    }

    /// 💳️ Copies one record through the contract's explicit credit-accounting seam. A record is
    /// never duplicated with `Clone`: component values, action bindings, and menu references must
    /// each acquire their own bounded owner or the whole candidate copy is refused.
    pub fn credited_record_at(&self, index: usize) -> Result<UiNodeRecord, UiDocumentTreeFault> {
        self.nodes.get_index(index).ok_or(UiDocumentTreeFault::Count)?.credited_clone().ok_or(UiDocumentTreeFault::Credits)
    }

    pub fn record(&self, id: UiNodeId) -> Option<&UiNodeRecord> {
        self.nodes.get(&id)
    }

    pub fn parent_of(&self, id: UiNodeId) -> Option<UiNodeId> {
        self.nodes.values().find(|record| record.children.iter().any(|child| *child == id)).map(|record| record.id)
    }

    #[expect(clippy::result_large_err, reason = "Fixed-capacity node admission returns the exact rejected node or page without allocating on refusal.")]
    pub fn try_upsert_record(&mut self, record: UiNodeRecord) -> Result<Option<UiNodeRecord>, UiNodeRecord> {
        self.nodes.try_insert(record)
    }

    pub fn try_reparent(&mut self, child: UiNodeId, parent: UiNodeId) -> Result<(), UiDocumentTreeFault> {
        if child == self.root || self.record(child).is_none() || self.record(parent).is_none() {
            return Err(UiDocumentTreeFault::MissingChild);
        }
        let previous = self.parent_of(child);
        if previous == Some(parent) {
            return Ok(());
        }
        let mut ancestor = Some(parent);
        for _ in 0..UI_DOCUMENT_NODES {
            let Some(current) = ancestor else { break };
            if current == child {
                return Err(UiDocumentTreeFault::Cycle);
            }
            ancestor = self.parent_of(current);
        }
        if ancestor.is_some() {
            return Err(UiDocumentTreeFault::Cycle);
        }
        self.nodes.get_mut(&parent).ok_or(UiDocumentTreeFault::MissingChild)?.children.try_push(child).map_err(|_| UiDocumentTreeFault::NodeCapacity)?;
        if let Some(previous) = previous {
            let children = &mut self.nodes.get_mut(&previous).ok_or(UiDocumentTreeFault::MissingChild)?.children;
            let index = children.iter().position(|id| *id == child).ok_or(UiDocumentTreeFault::MissingChild)?;
            children.swap_remove(index);
            for cursor in index..children.len().saturating_sub(1) {
                let next = *children.get(cursor + 1).ok_or(UiDocumentTreeFault::MissingChild)?;
                let current = std::mem::replace(children.get_mut(cursor).ok_or(UiDocumentTreeFault::MissingChild)?, next);
                *children.get_mut(cursor + 1).ok_or(UiDocumentTreeFault::MissingChild)? = current;
            }
        }
        Ok(())
    }

    pub fn remove_record(&mut self, id: UiNodeId) -> Option<UiNodeRecord> {
        self.nodes.remove(&id)
    }

    pub fn close_step(&mut self) -> bool {
        let next = self.nodes.keys().next().copied();
        if let Some(id) = next {
            drop(self.nodes.remove(&id));
            return false;
        }
        true
    }

    pub fn validate_header(&self) -> Result<(), UiDocumentTreeFault> {
        if self.nodes.len() != self.node_count {
            return Err(UiDocumentTreeFault::Count);
        }
        if self.nodes.get(&self.root).is_none() {
            return Err(UiDocumentTreeFault::MissingRoot);
        }
        Ok(())
    }

    pub fn validate_record(&self, index: usize) -> Result<(), UiDocumentTreeFault> {
        let record = self.nodes.get_index(index).ok_or(UiDocumentTreeFault::Count)?;
        for child in &record.children {
            if self.nodes.get(child).is_none() {
                return Err(UiDocumentTreeFault::MissingChild);
            }
            let parents = self.nodes.values().filter(|candidate| candidate.children.iter().any(|candidate_child| candidate_child == child)).count();
            if parents != 1 || *child == self.root {
                return Err(UiDocumentTreeFault::MultipleParents);
            }
        }
        Ok(())
    }
}
//#endregion 🔖️RetainedDocument
// #endregion tree
