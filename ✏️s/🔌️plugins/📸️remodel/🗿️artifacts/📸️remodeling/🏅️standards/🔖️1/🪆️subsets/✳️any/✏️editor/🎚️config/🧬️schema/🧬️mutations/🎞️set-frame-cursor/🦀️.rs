//! 🎞️ Set Frame Cursor in the remodeling config channel.

use super::{RemodelingConfig, RemodelingConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-frame-cursor")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFrameCursor {
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

impl protocol::MutationKind<RemodelingConfig, RemodelingConfigMutation> for SetFrameCursor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-cursor", kind: "set-frame-cursor", record: "SetFrameCursor" };
    fn diff(&self, base: &RemodelingConfig) -> protocol::MutationOutcome<RemodelingConfig> {
        let mut next = base.clone();
        if self.stream_id.is_some() { next.frame_cursor.stream_id = self.stream_id.clone(); }
        next.frame_cursor.frame_index = self.frame_index;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RemodelingConfig) -> Vec<RemodelingConfigMutation> { vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Frame Cursor".into() }
    fn target(&self) -> Vec<String> { vec!["frame-cursor".into()] }
}
