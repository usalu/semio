//! 🌲️ Exact typed built-tree retirement across ordinary and rejected retained child pages.

use super::*;
use crate::builder::{BuiltChildRetirementNext, BuiltChildrenIntoIter, BuiltNode, UI_BUILT_CHILD_RETIRE_SLOTS};
use std::mem::ManuallyDrop;

const _: () = {
    let depths = [<UiText as UiTypedRetire>::DEPTH, <crate::Component as UiTypedRetire>::DEPTH, <crate::LayoutSpec as UiTypedRetire>::DEPTH, <crate::StyleSpec as UiTypedRetire>::DEPTH, <crate::Activity as UiTypedRetire>::DEPTH, <bool as UiTypedRetire>::DEPTH, <crate::AccessibilitySpec as UiTypedRetire>::DEPTH, <crate::UiNodeBindings as UiTypedRetire>::DEPTH, <Option<MenuRef> as UiTypedRetire>::DEPTH];
    let mut index = 0;
    while index < depths.len() {
        assert!(depths[index] <= typed::UI_TYPED_RETIREMENT_DEPTH);
        index += 1;
    }
};

struct BuiltTreeOwned {
    node: Option<Box<BuiltNode>>,
    pages: Box<[Option<BuiltChildrenIntoIter>]>,
    page_count: usize,
    field: u8,
    cursor: UiTypedRetirementCursor,
}

/// 🌿️ Retains the whole tree until exact closure; early Drop is a contract violation, not cancellation.
pub struct BuiltTreeRetirement {
    owned: ManuallyDrop<BuiltTreeOwned>,
}

impl BuiltTreeRetirement {
    /// 🎟️ Moves the root and allocates one fixed page stack; no component or child is cloned or traversed.
    pub fn new(root: BuiltNode) -> Self {
        let mut pages = Vec::with_capacity(UI_BUILT_CHILD_RETIRE_SLOTS);
        pages.resize_with(UI_BUILT_CHILD_RETIRE_SLOTS, || None);
        Self { owned: ManuallyDrop::new(BuiltTreeOwned { node: Some(Box::new(root)), pages: pages.into_boxed_slice(), page_count: 0, field: 0, cursor: UiTypedRetirementCursor::empty() }) }
    }

    /// 🪶️ Advances one typed leaf, child transfer, or exact page release without draining global queues.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
        let owned = &mut *self.owned;
        if let Some(node) = owned.node.as_mut() {
            let BuiltNode { key, component, layout, style, activity, disabled, accessibility, bindings, menu, children, rejected_children } = node.as_mut();
            if owned.field < 9 {
                let mut step = match owned.field {
                    0 => owned.cursor.advance(key, 1, maximum_bytes)?,
                    1 => owned.cursor.advance(component, 1, maximum_bytes)?,
                    2 => owned.cursor.advance(layout, 1, maximum_bytes)?,
                    3 => owned.cursor.advance(style, 1, maximum_bytes)?,
                    4 => owned.cursor.advance(activity, 1, maximum_bytes)?,
                    5 => owned.cursor.advance(disabled, 1, maximum_bytes)?,
                    6 => owned.cursor.advance(accessibility, 1, maximum_bytes)?,
                    7 => owned.cursor.advance(bindings, 1, maximum_bytes)?,
                    8 => owned.cursor.advance(menu, 1, maximum_bytes)?,
                    _ => unreachable!(),
                };
                if step.complete { owned.cursor = UiTypedRetirementCursor::empty(); owned.field += 1; }
                step.complete = false;
                return Ok(step);
            }
            if owned.field < 11 {
                let children = if owned.field == 9 { rejected_children } else { children };
                if children.capacity() != 0 {
                    if owned.page_count == owned.pages.len() { return Err("built tree retirement exceeds admitted child pages"); }
                    owned.pages[owned.page_count] = Some(std::mem::take(children).into_iter());
                    owned.page_count += 1;
                }
                owned.field += 1;
                return Ok(UiValueRetirementStep { progressed: true, ..Default::default() });
            }
            owned.node.take();
            owned.field = 0;
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
        }
        if owned.page_count != 0 {
            let index = owned.page_count - 1;
            let iterator = owned.pages[index].as_mut().ok_or("built tree retirement lost its retained page")?;
            return match iterator.try_next_or_release()? {
                BuiltChildRetirementNext::Pending => Ok(UiValueRetirementStep::default()),
                BuiltChildRetirementNext::Node(node) => { owned.node = Some(node); owned.cursor = UiTypedRetirementCursor::empty(); Ok(UiValueRetirementStep { progressed: true, ..Default::default() }) }
                BuiltChildRetirementNext::Complete => { owned.pages[index].take(); owned.page_count -= 1; Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() }) }
            };
        }
        owned.pages = Box::default();
        Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: 1, ..Default::default() })
    }

    /// 🧺️ Requires the root, every child reservation, typed descendant, and traversal allocation to be gone.
    pub fn terminal_is_empty(&self) -> bool {
        self.owned.node.is_none() && self.owned.page_count == 0 && self.owned.pages.is_empty()
    }
}

impl Drop for BuiltTreeRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && !std::thread::panicking() { panic!("built tree retirement requires exact terminal closure"); }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
