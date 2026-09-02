//! 📜️ Trinity Rewriting viewer — the Rule window: a read-only text render of the live
//! `RewritingSnapshot` projection (LHS/RHS pattern JSON plus parameter bindings), built on the
//! framework's `TextWindowKit` (contract §2.6). Pure `RewritingSnapshot -> TextView` read, built from
//! the same artifact-level fields the editor's own windows read — this file itself imports nothing
//! from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright).

use crate::artifacts::rewriting::RewritingSnapshot;
use semio_framework_plugin::WindowKindDefinition;
// 🚧️ SDK GAP: `TextWindowKit`/`TextView`/`WindowKit` (contract §2.6) are not yet in
// `semio_framework_plugin`'s curated crate-root re-export list (`🔌️plugin/🦀️.rs`) — the
// whole `//#region 🔖️WindowKits` region lives inside `pub mod app`, same gap category as
// `InteractionView`. Not fixable here (`🧰️framework/**` is outside this packet's lease); flagged in
// this packet's notes file for W1-A.
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};

//#region 🔖️Definition
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

/// 🧱️ Stitched into the viewer manifest by `crate::viewer::rewriting::create_trinity_rewriting_viewer` —
/// the read-only variant (`TextWindowKit::window_kind`, no `replace-text` action).
pub fn definition() -> WindowKindDefinition {
    TextWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `RewritingSnapshot -> TextView` read: LHS pattern, RHS actions and the live parameter
/// bindings, pretty-printed as one read-only JSON document — no rule-applied After computation (that
/// stays the editor-only `after_fixture_json` helper's job), no rule-layout point positions (pure
/// window-arrangement state, not rule content).
pub fn rule_text(state: &RewritingSnapshot) -> String {
    let lhs: pack::JsonValue = pack::parse_json(&state.lhs_json).unwrap_or_default();
    let rhs: pack::JsonValue = pack::parse_json(&state.rhs_json).unwrap_or_default();
    let document = pack::json!({ "lhs": lhs, "rhs": rhs, "parameterBindings": state.parameter_bindings });
    pack::json_to_string_pretty(&document)
}

pub fn render(document: &RewritingSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
        let state = RewritingSnapshot { before_fixture_json: "{}".into(), lhs_json: r#"{"whereClause":"a.name = 'b'"}"#.into(), rhs_json: r#"{"parameters":[]}"#.into(), parameter_bindings: Default::default(), rule_layout: Default::default() };
        let text = rule_text(&state);
        assert!(text.contains("whereClause"));
        assert!(text.contains("parameters"));
    }
}
//#endregion 🧪️Tests
