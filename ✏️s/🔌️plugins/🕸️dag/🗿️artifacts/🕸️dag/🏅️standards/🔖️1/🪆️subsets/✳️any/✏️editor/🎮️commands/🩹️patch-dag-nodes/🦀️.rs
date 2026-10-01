//! 🔧️ 🔧️ DAG play app commands command — `patch-dag-nodes`: the inspector's name and slider fields.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::mutations::{change_node_name, set_slider, DagSliderField};
use crate::op::DagMutation;
use crate::{DagNodeKind, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-dag-nodes")]
pub struct PatchDagNodes {
    pub node_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🩹️ `name` renames every addressed node whose name differs; `value`/`min`/`max` yield the ABSOLUTE `set-slider` leaf per
/// addressed slider. A held number field carries its press as the dispatch's top-level `gesture`/`commit`, so the
/// framework scrub machine keeps every tick provisional and commits the release as ONE edit; this handler never reads a
/// gesture and the same dispatch serves a one-shot (MCP, keyboard) unchanged.
pub fn handle(payload: &PatchDagNodes, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let nodes = doc.snapshot.nodes();
    let addressed = nodes.iter().filter(|node| payload.node_ids.contains(&node.id));
    let operations: Vec<DagMutation> = match (payload.field.as_str(), DagSliderField::parse(&payload.field), payload.value.trim().parse::<f64>().ok().filter(|value| value.is_finite())) {
        ("name", _, _) => addressed.filter(|node| node.name != payload.value).map(|node| change_node_name(node.id.clone(), payload.value.clone())).collect(),
        (_, Some(field), Some(value)) => addressed
            .filter(|node| match node.kind {
                DagNodeKind::Slider { value: current, min, max, .. } => match field {
                    DagSliderField::Value => current != value,
                    DagSliderField::Min => min != value,
                    DagSliderField::Max => max != value,
                },
                _ => false,
            })
            .map(|node| set_slider(node.id.clone(), field, value))
            .collect(),
        _ => Vec::new(),
    };
    Ok(Emit::mutations(operations))
}
