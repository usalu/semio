//! 🧭️ The kit editor's edit rules: which snapshot pointer raises which ONE concrete kit mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{complete_by, ent, ins, keyed, named, rem, resolve, slot_edit, spread_snake, Entries, Reshape, Slot, ITEM};
use crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector, SnapshotEditEvent};

const DESIGN: Selector = named("id", "id");

/// 📚 Every pointer a kit editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/types/*/name", "rename-type", &[named("id", "id")], "new_name"),
        ent("/designs/*/pieces", "edit-design", &[DESIGN], "pieces"),
        ent("/designs/*/connections", "edit-design", &[DESIGN], "connections"),
        ent("/representations/*/pin", "change-representation-pin", &[Selector::Index("index")], "pin"),
    ],
    inserts: &[
        ins("/types", "add-type", &[], Some("at"), ITEM),
        ins("/designs", "add-design", &[], Some("at"), ITEM),
        ins("/objects", "create-object", &[], Some("at"), ITEM),
        ins("/models", "create-model", &[], Some("at"), ITEM),
        ins("/representations", "bind-representation", &[], Some("at"), ITEM),
    ],
    removes: &[
        rem("/types", "remove-type", &[], keyed("id", "id")),
        rem("/designs", "remove-design", &[], keyed("id", "id")),
        rem("/objects", "delete-object", &[], keyed("child_id", "childId")),
        rem("/models", "delete-model", &[], keyed("child_id", "childId")),
        rem("/representations", "unbind-representation", &[], RowKey::Index("index")),
    ],
};

const SLOTS: &[Slot] = &[Slot { field: "properties", create: "create-properties", delete: "delete-properties" }];

const RESHAPES: &[(&str, Reshape)] = &[
    ("add-type", spread_snake),
    ("add-design", empty_design),
    ("create-object", spread_snake),
    ("create-model", spread_snake),
    ("bind-representation", spread_snake),
    ("edit-design", design_parts),
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn empty_design(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    spread_snake(tree, entries)?;
    for part in ["pieces", "connections"] {
        if let Some(at) = entries.iter().position(|(key, _)| key == part) {
            if !matches!(entries.remove(at).1, DslValue::Array(rows) if rows.is_empty()) {
                return Err(format!("a design is added empty: insert it, then edit its {part}"));
            }
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn design_parts(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete_by(tree, entries, "designs", "id", &["pieces", "connections"])
}

/// 🧩️ Attaching or detaching the properties child is its `create-*` or `delete-*` kind; every other edit goes through [`EDIT_RULES`] and completes its payload.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioKitSnapshot) -> Result<Option<Vec<SemioKitMutation>>, Fault> {
    match slot_edit::<SemioKitSnapshot, SemioKitMutation>(SLOTS, spread_snake, snapshot, event)? {
        Some(mutations) => Ok(Some(mutations)),
        None => resolve::<SemioKitSnapshot, SemioKitMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some),
    }
}
