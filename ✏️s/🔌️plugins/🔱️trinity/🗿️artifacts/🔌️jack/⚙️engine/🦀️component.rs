//! ⚙️ `trinity.graph` artifact — headless compute over the projection (constitutional: engine).
//!
//! 📌️ The jack query-language compute itself (`run_jack_query` and friends) lives in the plugin's
//! `🫀️core` cross-artifact kernel — used by both the `jack` app's UI and the `rewrite` app's
//! `apply_rule` — not here. This file holds the one document-level pure helper the old bundle crate's
//! `⚙️engine` module also held.

use crate::artifacts::jack::{empty_trinity_graph_fixture, GraphFixture};

/// 📦️ An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> GraphFixture {
    empty_trinity_graph_fixture()
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_jack_document_has_no_nodes_or_edges() {
        let fixture = empty_jack_document();
        assert!(fixture.nodes.is_empty());
        assert!(fixture.edges.is_empty());
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.document",
        extension: Some("trinity"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::core::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::core::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::core::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::core::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::core::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::core::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::core::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::core::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::core::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::core::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("jack.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::core::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::core::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::core::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::core::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.spr"),
    });
}
