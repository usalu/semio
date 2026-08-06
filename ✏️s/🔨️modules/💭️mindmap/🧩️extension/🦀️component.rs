//! 🧠️ Mindmap graph extension: topics and relationships on a property graph.
//!
//! 🪢️ This is genuinely cross-cutting infrastructure, not app-specific document data — it's a generic
//! graph-extension trait/type-alias pattern kept here (outside the `app/wires` constitutional split)
//! because `s/plugin/puzzle/2d/rs` also depends on it directly (`pub use semio_s_mindmap as
//! mindmap;`). The mindmap-wires document/operation entities that used to live alongside this trait
//! moved into `s/plugin/reasoning/app/wires/{rs,op}` — see that split's `dsl`/`engine` crates for the
//! `MindmapWiresDocument`/`MindmapWiresOperation` types this trait no longer bundles.

pub use infinite_canvas as canvas;
pub use canvas::board::{EdgeId, GraphExtension, NodeId};

// #region 🔖️MindmapExtension
/// 🧠️ Mindmap semantics over a property graph canvas.
pub trait MindmapExtension: GraphExtension {
    fn topic_label(&self, node_id: NodeId) -> Option<&str>;
}

/// 🧩️ Topic is a graph node; relationship is a graph edge.
pub type TopicId = NodeId;
pub type RelationshipId = EdgeId;

/// 🧭️ Default mindmap extension stub (projection mirror — OS graph packs own topic authority).
#[derive(Clone, Debug, Default)]
pub struct DefaultMindmapExtension {
    pub topics: std::collections::BTreeMap<TopicId, String>,
}

impl canvas::CanvasExtension for DefaultMindmapExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/default"
    }
}

impl GraphExtension for DefaultMindmapExtension {}

impl MindmapExtension for DefaultMindmapExtension {
    fn topic_label(&self, node_id: TopicId) -> Option<&str> {
        self.topics.get(&node_id).map(String::as_str)
    }
}
// #endregion 🔖️MindmapExtension
