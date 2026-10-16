//! 🧬️ Canonical action field order for typed ownership operations.
#[macro_export]
macro_rules! ui_action_field_catalog {
    ($visitor:ident) => {
        $visitor!(ActionId { 0 => scope: UiText, 1 => name: UiText, 2 => version: u16 });
        $visitor!(ActionBinding { 0 => trigger: Trigger, 1 => action: ActionId, 2 => args: Option<UiValue>, 3 => capability: Option<UiText> });
        $visitor!(MenuRef { 0 => id: UiText, 1 => args: Option<UiValue> });
        $visitor!(UiIntent { 0 => surface: SurfaceId, 1 => revision: UiRevision, 2 => node: UiNodeId, 3 => node_key: UiText, 4 => trigger: Trigger, 5 => action: ActionId, 6 => args: Option<UiValue>, 7 => input: Option<UiValue>, 8 => seq: u64 });
    };
}
