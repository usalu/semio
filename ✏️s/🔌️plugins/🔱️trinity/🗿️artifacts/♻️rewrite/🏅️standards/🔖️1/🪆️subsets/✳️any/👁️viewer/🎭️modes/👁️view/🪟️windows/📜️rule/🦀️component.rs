//! 📜️ Trinity Rewrite viewer — the Rule window: a read-only text render of the live
//! `RewriteSnapshot` projection (LHS/RHS pattern JSON plus parameter bindings), built on the
//! framework's `TextWindowKit` (contract §2.6). Pure `RewriteSnapshot -> TextView` read, built from
//! the same artifact-level fields the editor's own windows read — this file itself imports nothing
//! from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright).

use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{UiNode, WindowKindDefinition};
// 🚧️ SDK GAP: `TextWindowKit`/`TextView`/`WindowKit` (contract §2.6) are not yet in
// `semio_framework_plugin`'s curated crate-root re-export list (`🔌️plugin/🦀️component.rs`) — the
// whole `//#region 🔖️WindowKits` region lives inside `pub mod app`, same gap category as
// `InteractionView`. Not fixable here (`🧰️framework/**` is outside this packet's lease); flagged in
// this packet's notes file for W1-A.
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};

//#region 🔖️Definition
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

/// 🧱️ Stitched into the viewer manifest by `crate::viewer::rewrite::create_trinity_rewrite_viewer` —
/// the read-only variant (`TextWindowKit::window_kind`, no `replace-text` action).
pub fn definition() -> WindowKindDefinition {
    TextWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `RewriteSnapshot -> TextView` read: LHS pattern, RHS actions and the live parameter
/// bindings, pretty-printed as one read-only JSON document — no rule-applied After computation (that
/// stays the editor-only `after_fixture_json` helper's job), no rule-layout point positions (pure
/// window-arrangement state, not rule content).
pub fn rule_text(state: &RewriteSnapshot) -> String {
    let lhs: serde_json::Value = serde_json::from_str(&state.lhs_json).unwrap_or_default();
    let rhs: serde_json::Value = serde_json::from_str(&state.rhs_json).unwrap_or_default();
    let document = serde_json::json!({ "lhs": lhs, "rhs": rhs, "parameterBindings": state.parameter_bindings });
    serde_json::to_string_pretty(&document).unwrap_or_default()
}

pub fn render(document: &RewriteSnapshot) -> UiNode {
    TextWindowKit::render(&TextView { text: rule_text(document), language: Some("json".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_shared_text_window_kind() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
    }

    #[semio_framework_async_macros::async_test]
    async fn rule_text_embeds_lhs_and_rhs() {
        let state = RewriteSnapshot { before_fixture_json: "{}".into(), lhs_json: r#"{"whereClause":"a.name = 'b'"}"#.into(), rhs_json: r#"{"parameters":[]}"#.into(), parameter_bindings: Default::default(), rule_layout: Default::default() };
        let text = rule_text(&state);
        assert!(text.contains("whereClause"));
        assert!(text.contains("parameters"));
    }
}
//#endregion 🧪️Tests
