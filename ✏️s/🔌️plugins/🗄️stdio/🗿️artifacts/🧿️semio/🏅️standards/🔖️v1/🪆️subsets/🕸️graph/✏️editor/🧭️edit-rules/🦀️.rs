//! 🧭️ The graph editor's edit rules: which snapshot pointer raises which ONE concrete graph mutation kind. An edit no kind expresses is refused, never approximated.

use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::complete_by;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::ent;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::ins;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::keyed;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::named;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::rem;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::resolve;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::spread_snake;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::Entries;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::Reshape;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::ITEM;
use crate::schema::mutations::SemioGraphMutation;
use crate::schema::snapshot::SemioGraphSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector, SnapshotEditEvent};

const NODE: Selector = named("id", "id");
const PORT_NODE: Selector = named("node_id", "id");
const PROPERTY_NODE: Selector = named("node_id", "id");
const PROPERTY_EDGE: Selector = named("edge_id", "id");
const PROPERTY: Selector = named("key", "key");

/// 📚 Every pointer a graph editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/nodes/*/id", "rename-node", &[NODE], "new_id"),
        ent("/nodes/*/kind", "change-node-kind", &[NODE], "new_kind"),
        ent("/nodes/*/label", "change-node-label", &[NODE], "new_label"),
        ent("/nodes/*/position", "move-node", &[NODE], "new_position"),
        ent("/nodes/*/width", "resize-node", &[NODE], "width"),
        ent("/nodes/*/height", "resize-node", &[NODE], "height"),
        ent("/nodes/*/properties/*/value", "set-node-property", &[PROPERTY_NODE, PROPERTY], "value"),
        ent("/edges/*/properties/*/value", "set-edge-property", &[PROPERTY_EDGE, PROPERTY], "value"),
    ],
    inserts: &[
        ins("/nodes", "create-node", &[], Some("at"), ITEM),
        ins("/edges", "create-edge", &[], Some("at"), ITEM),
        ins("/nodes/*/ports", "add-node-port", &[PORT_NODE], Some("index"), "port"),
        ins("/nodes/*/properties", "add-node-property", &[PROPERTY_NODE], Some("index"), "property"),
        ins("/edges/*/properties", "add-edge-property", &[PROPERTY_EDGE], Some("index"), "property"),
    ],
    removes: &[
        rem("/nodes", "delete-node", &[], keyed("id", "id")),
        rem("/edges", "delete-edge", &[], keyed("id", "id")),
        rem("/nodes/*/ports", "remove-node-port", &[PORT_NODE], RowKey::Index("index")),
        rem("/nodes/*/properties", "remove-node-property", &[PROPERTY_NODE], keyed("key", "key")),
        rem("/edges/*/properties", "remove-edge-property", &[PROPERTY_EDGE], keyed("key", "key")),
    ],
};

const RESHAPES: &[(&str, Reshape)] = &[("create-node", spread_snake), ("create-edge", spread_snake), ("resize-node", resize_node)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resize_node(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete_by(tree, entries, "nodes", "id", &["width", "height"])
}

/// 🧩️ Every edit goes through [`EDIT_RULES`]; `create-*` spreads the inserted row and `resize-node` carries the size it keeps.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioGraphSnapshot) -> Result<Option<Vec<SemioGraphMutation>>, Fault> {
    resolve::<SemioGraphSnapshot, SemioGraphMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some)
}
