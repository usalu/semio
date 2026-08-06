//! 🧠️ Mindmap graph extension: topics and relationships on a property graph.
//!
//! 🪢️ This is genuinely cross-cutting infrastructure, not app-specific document data — it's a generic
//! graph-extension trait/type-alias pattern kept here (outside the `app/wires` constitutional split)
//! because `s/plugin/puzzle/2d/rs` also depends on it directly (`pub use reasoning_mindmap as
//! mindmap;`). The mindmap-wires document/operation entities that used to live alongside this trait
//! moved into `s/plugin/reasoning/app/wires/{rs,op}` — see that split's `dsl`/`engine` crates for the
//! `MindmapWiresDocument`/`MindmapWiresOperation` types this trait no longer bundles.

pub use infinite_board_normal_directed as graph;
pub use infinite_canvas as canvas;

// #region 🔖️MindmapExtension
/// 🧠️ Mindmap semantics over a property graph canvas.
pub trait MindmapExtension: graph::GraphExtension {
    fn topic_label(&self, node_id: graph::NodeId) -> Option<&str>;
}

/// 🧩️ Topic is a graph node; relationship is a graph edge.
pub type TopicId = graph::NodeId;
pub type RelationshipId = graph::EdgeId;

/// 🧭️ Default mindmap extension stub.
#[derive(Clone, Debug, Default)]
pub struct DefaultMindmapExtension {
    pub topics: std::collections::BTreeMap<TopicId, String>,
}

impl canvas::CanvasExtension for DefaultMindmapExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/default"
    }
}

impl graph::GraphExtension for DefaultMindmapExtension {}

impl MindmapExtension for DefaultMindmapExtension {
    fn topic_label(&self, node_id: TopicId) -> Option<&str> {
        self.topics.get(&node_id).map(String::as_str)
    }
}
// #endregion 🔖️MindmapExtension
