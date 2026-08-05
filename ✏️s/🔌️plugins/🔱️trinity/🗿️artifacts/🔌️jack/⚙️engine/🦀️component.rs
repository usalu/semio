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
