//! 🗃️ Fixed-capacity linear containers — the bounded vector, map and set every reconcile cursor and retained reconciler keys its identities with. They allocate their whole capacity once and refuse the next push instead of growing.

#[cfg(any(test, feature = "testkit"))]
use std::collections::HashSet;
use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};

//#region 🔖️Identity

/// 🔑️ A node's reconciliation identity: which parent it hangs under (`None` only for the root, which
/// has no parent) plus its own sibling `key`. Two [`TreeNode`]s presented on different frames
/// with the same identity are the SAME node as far as reconciliation is concerned, regardless of what
/// position either occupied among its siblings — this is the one invariant every other rule here
/// exists to preserve.
pub type NodeIdentity = (Option<ui_contract::UiNodeId>, ui_contract::UiText);

pub const SURFACE_RECONCILE_FIXED_NODES: usize = ui_contract::UI_DOCUMENT_NODES;
#[cfg(any(test, feature = "testkit"))]
pub const SURFACE_RECONCILE_FIXED_OPS: usize = SURFACE_RECONCILE_FIXED_NODES * 9 + 1;

#[derive(Debug)]
pub struct SurfaceFixedVec<T, const N: usize> {
    pub entries: Box<[Option<T>]>,
    len: usize,
}

impl<T, const N: usize> Default for SurfaceFixedVec<T, N> {
    fn default() -> Self {
        let mut entries = Vec::with_capacity(N);
        entries.resize_with(N, || None);
        Self { entries: entries.into_boxed_slice(), len: 0 }
    }
}

impl<T, const N: usize> SurfaceFixedVec<T, N> {
    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len == N || self.len == self.entries.len() {
            return Err(value);
        }
        self.entries[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.entries[index].take()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| self.entries[index].as_ref()).flatten()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.len).then(|| self.entries[index].as_mut()).flatten()
    }

    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.len.checked_sub(1).and_then(|index| self.entries[index].as_mut())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries[..self.len].iter().filter_map(Option::as_ref)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.entries[..self.len].iter_mut().filter_map(Option::as_mut)
    }

    pub fn take_all(&mut self) -> Self {
        std::mem::replace(self, Self { entries: Box::new([]), len: 0 })
    }
}

impl<T: PartialEq, const N: usize> SurfaceFixedVec<T, N> {
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|candidate| candidate == value)
    }
}

impl<T, const N: usize> std::ops::Index<usize> for SurfaceFixedVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("fixed reconcile index was admitted")
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for SurfaceFixedVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("fixed reconcile index was admitted")
    }
}

#[derive(Debug)]
pub struct SurfaceLinearMap<K, V, const N: usize> {
    pub entries: SurfaceFixedVec<(K, V), N>,
}

impl<K, V, const N: usize> Default for SurfaceLinearMap<K, V, N> {
    fn default() -> Self {
        Self { entries: SurfaceFixedVec::default() }
    }
}

impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    pub fn take_all(&mut self) -> Self {
        Self { entries: self.entries.take_all() }
    }
}

impl<K: Eq, V, const N: usize> SurfaceLinearMap<K, V, N> {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value)
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)> {
        if let Some((_, current)) = self.entries.iter_mut().find(|(candidate, _)| candidate == &key) {
            return Ok(Some(std::mem::replace(current, value)));
        }
        self.entries.try_push((key, value)).map(|()| None)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.entries.iter().position(|(candidate, _)| candidate == key)?;
        let last = self.entries.pop()?;
        if index == self.entries.len() {
            return Some(last.1);
        }
        let removed = std::mem::replace(&mut self.entries[index], last);
        Some(removed.1)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.iter().any(|(candidate, _)| candidate == key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(key, _)| key)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn clear(&mut self) {
        while self.entries.pop().is_some() {}
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Debug)]
pub struct SurfaceLinearSet<T, const N: usize> {
    pub entries: SurfaceFixedVec<T, N>,
}

impl<T, const N: usize> Default for SurfaceLinearSet<T, N> {
    fn default() -> Self {
        Self { entries: SurfaceFixedVec::default() }
    }
}

impl<T: Eq, const N: usize> SurfaceLinearSet<T, N> {
    pub fn try_insert(&mut self, value: T) -> Result<bool, T> {
        if self.entries.contains(&value) {
            return Ok(false);
        }
        self.entries.try_push(value).map(|()| true)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// 🔑️ `node`'s identity under `parent`, as looked up in / inserted into [`SurfaceReconciler::key_index`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "testkit"))]
pub fn identity_of(parent: Option<ui_contract::UiNodeId>, node: &TreeNode) -> NodeIdentity {
    (parent, node.key.clone())
}

/// 🚨️ Panics naming the first duplicate sibling key found in `children`. [`ComponentTree`]'s
/// own constructor already asserts this on every level of a tree built through `TreeNode::with_children`
/// / `ComponentTree::new`, but `ComponentTree::new(root)`'s field is `pub`, so a caller can hand this
/// reconciler a tree that skipped that constructor entirely — this is therefore genuine defense in
/// depth, not a redundant re-check, and it is what makes a duplicate key a loud authoring-bug panic
/// here too rather than one key silently shadowing the other during matching.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "testkit"))]
pub fn assert_unique_child_keys(parent: ui_contract::UiNodeId, children: &ui_contract::BuiltChildren) {
    let mut seen: HashSet<&str> = HashSet::with_capacity(children.len());
    for child in children {
        assert!(seen.insert(child.key.as_str()), "🚫️ duplicate sibling key {:?} under parent {parent:?} — reconciliation keys must be unique among siblings", child.key);
    }
}

//#endregion 🔖️Identity

//#region 🔖️Reconciler

impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    #[cfg(any(test, feature = "testkit"))]
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get(index).map(|(key, value)| (key, value))
    }

    pub fn take_first(&mut self) -> Option<(K, V)> {
        if self.entries.is_empty() {
            return None;
        }
        let last = self.entries.pop()?;
        if self.entries.is_empty() {
            return Some(last);
        }
        Some(std::mem::replace(&mut self.entries[0], last))
    }
}

impl<T, const N: usize> SurfaceLinearSet<T, N> {
    pub fn pop(&mut self) -> Option<T> {
        self.entries.pop()
    }
}

//#endregion 🔖️Reconciler
