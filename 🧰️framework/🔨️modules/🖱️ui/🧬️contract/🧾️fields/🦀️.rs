//! 🧬️ Canonical document field order for typed ownership operations.
#[macro_export]
macro_rules! ui_document_field_catalog {
    ($visitor:ident) => {
        $visitor!(UiNodeRecord { 0 => id: UiNodeId, 1 => key: UiText, 2 => component: Component, 3 => layout: LayoutSpec, 4 => style: StyleSpec, 5 => activity: Activity, 6 => disabled: bool, 7 => transition: Option<TransitionHint>, 8 => accessibility: AccessibilitySpec, 9 => bindings: UiNodeBindings, 10 => menu: Option<MenuRef>, 11 => children: UiNodeChildren });
        $visitor!(UiSnapshot { 0 => surface: SurfaceId, 1 => revision: UiRevision, 2 => root: UiNodeId, 3 => nodes: UiSnapshotNodes, 4 => layout_epoch: u64 });
    };
}
