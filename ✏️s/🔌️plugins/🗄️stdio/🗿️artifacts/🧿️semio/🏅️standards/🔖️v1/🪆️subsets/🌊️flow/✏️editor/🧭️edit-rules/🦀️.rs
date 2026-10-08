//! 🧭️ The flow editor's edit rules: which snapshot pointer raises which ONE concrete flow mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{complete_by, ent, ins, keyed, named, rem, resolve, spread_item, Entries, Reshape, ITEM};
use crate::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{EditRules, Selector, SnapshotEditEvent};

const ID: Selector = named("id", "id");
const PARAM: Selector = named("key", "key");

/// 📚 Every pointer a flow editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/nodes/*/kind", "set-node-kind", &[ID], "kind"),
        ent("/nodes/*/label", "set-node-label", &[ID], "label"),
        ent("/nodes/*/position", "set-node-position", &[ID], "position"),
        ent("/nodes/*/params/*/value", "set-node-param", &[ID, PARAM], "value"),
        ent("/edges/*/kind", "set-edge-kind", &[ID], "kind"),
        ent("/edges/*/from", "set-edge-endpoints", &[ID], "from"),
        ent("/edges/*/to", "set-edge-endpoints", &[ID], "to"),
    ],
    inserts: &[ins("/nodes", "insert-node", &[], Some("at"), "node"), ins("/edges", "insert-edge", &[], Some("at"), "edge"), ins("/nodes/*/params", "set-node-param", &[ID], Some("at"), ITEM)],
    removes: &[rem("/nodes", "remove-node", &[], keyed("id", "id")), rem("/edges", "remove-edge", &[], keyed("id", "id")), rem("/nodes/*/params", "remove-node-param", &[ID], keyed("key", "key"))],
};

const RESHAPES: &[(&str, Reshape)] = &[("set-node-param", spread_item), ("set-edge-endpoints", endpoints)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn endpoints(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete_by(tree, entries, "edges", "id", &["from", "to"])
}

/// 🧩️ Every edit goes through [`EDIT_RULES`]; an inserted parameter spreads into its payload and a moved endpoint carries the one it keeps.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioFlowSnapshot) -> Result<Option<Vec<SemioFlowMutation>>, Fault> {
    resolve::<SemioFlowSnapshot, SemioFlowMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some)
}
