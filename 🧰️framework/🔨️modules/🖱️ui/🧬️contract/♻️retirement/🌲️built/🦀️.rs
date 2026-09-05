//! 🌲️ Exact typed built-tree retirement across ordinary and rejected retained child pages.

use super::*;
use crate::builder::{BuiltChildRetirementNext, BuiltChildrenIntoIter, BuiltNode, UI_BUILT_CHILD_RETIRE_SLOTS};
use std::mem::ManuallyDrop;

const _: () = {
    assert!(<UiText as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<crate::Component as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<crate::LayoutSpec as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<crate::StyleSpec as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<crate::Activity as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<bool as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<crate::AccessibilitySpec as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<crate::UiNodeBindings as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
    assert!(<Option<crate::MenuRef> as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH);
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
mod tests {
    use super::*;
    use crate::{BuiltChildren, Component, TextProps};

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap() }

    fn text_node(key: &str, value: &str) -> BuiltNode {
        BuiltNode::try_new(key, Component::Text(TextProps { value: crate::Label::try_from(value).unwrap(), emphasize: None, data_attributes: Default::default() })).unwrap()
    }

    fn close(owner: &mut BuiltTreeRetirement, grant: usize) -> usize {
        assert_eq!(owner.close_step(0, grant).unwrap(), UiValueRetirementStep::default());
        assert_eq!(owner.close_step(1, 0).unwrap(), UiValueRetirementStep::default());
        let mut bytes = 0;
        for _ in 0..2_000_000 {
            let step = owner.close_step(1, grant).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= grant);
            bytes += step.released_bytes;
            if step.complete { assert!(owner.terminal_is_empty()); return bytes; }
            if !step.progressed { std::thread::yield_now(); }
        }
        panic!("built tree retirement did not finish");
    }

    #[test]
    fn built_tree_retirement_closes_all_typed_fields_and_preserves_foreign_values() {
        let fixture = fixture();
        let components: serde_json::Value = serde_json::from_str(include_str!("../🌳️typed/🧩️components.json")).unwrap();
        let foreign: UiValue = serde_json::from_value(serde_json::json!({"foreign": ["Grüße"]})).unwrap();
        let foreign_handles = super::super::tests::descendants(&foreign);
        for grant in fixture["grants"].as_array().unwrap() {
            for row in components["cases"].as_array().unwrap() {
                let mut node = BuiltNode::try_new("K", serde_json::from_value(row["component"].clone()).unwrap()).unwrap();
                node.bindings.try_push(serde_json::from_value(fixture["binding"].clone()).unwrap()).unwrap();
                node.menu = Some(serde_json::from_value(fixture["menu"].clone()).unwrap());
                node.children.try_push(text_node("C", "child")).unwrap();
                node.rejected_children.try_push(text_node("R", "rejected")).unwrap();
                let mut owner = BuiltTreeRetirement::new(node);
                assert_eq!(close(&mut owner, grant.as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize + fixture["extraPayloadBytes"].as_u64().unwrap() as usize);
                assert_eq!(serde_json::to_value(&foreign).unwrap(), serde_json::json!({"foreign": ["Grüße"]}));
                with_ui_value_arena(|arena| assert!(foreign_handles.iter().all(|handle| arena.collection(*handle).is_some())));
            }
        }
        let mut foreign = UiValueRetirement::new(foreign);
        for _ in 0..10_000 { if foreign.close_step(1, 4096).unwrap().complete { break; } }
        assert!(foreign.terminal_is_empty());
        let mut node = text_node("root", "V");
        node.children.try_push(text_node("ordinary", "V")).unwrap();
        node.rejected_children.try_push(text_node("rejected", "V")).unwrap();
        let mut owner = BuiltTreeRetirement::new(node);
        let mut visited = Vec::new();
        for _ in 0..10_000 {
            if owner.owned.field == 0 { if let Some(node) = owner.owned.node.as_ref() { visited.push(node.key.as_str().to_owned()); } }
            if owner.close_step(1, 4096).unwrap().complete { break; }
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(visited, ["root", "ordinary", "rejected"]);
        eprintln!("[DEBUG] built-tree retirement components=18 grants=3 ordinary+rejected=2 extraBytes=30 foreign=preserved");
    }

    #[test]
    fn built_tree_retirement_closes_full_page_chain_beyond_observer_depth() {
        let fixture = fixture();
        let pages = fixture["chain"]["pages"].as_u64().unwrap() as usize;
        assert_eq!(pages, UI_BUILT_CHILD_RETIRE_SLOTS);
        let mut node = text_node("K", "V");
        for index in 0..pages {
            let mut parent = text_node("K", "V");
            let children: &mut BuiltChildren = if index % 2 == 0 { &mut parent.children } else { &mut parent.rejected_children };
            children.try_push(node).unwrap();
            node = parent;
        }
        let mut owner = BuiltTreeRetirement::new(node);
        assert_eq!(close(&mut owner, 1), fixture["chain"]["nodes"].as_u64().unwrap() as usize * 2);
        assert_eq!(owner.close_step(0, 0).unwrap(), UiValueRetirementStep { complete: true, ..Default::default() });
        eprintln!("[DEBUG] built-tree retirement fullPages={pages} nodes={} observerDepth=64 exactTerminal=true", pages + 1);
    }
}
