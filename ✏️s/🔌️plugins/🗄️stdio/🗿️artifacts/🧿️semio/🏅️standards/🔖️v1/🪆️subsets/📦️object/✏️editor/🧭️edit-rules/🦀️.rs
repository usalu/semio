//! 🧭️ The object editor's edit rules: which snapshot pointer raises which ONE concrete object mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, resolve, slot_edit, spread_snake, Slot};
use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use semio_framework_plugin::Fault;
use semio_s_artifact_stdio_contract::editing::{EditRules, SnapshotEditEvent};

/// 📚 Every pointer an object editor edit resolves, one rule per kind that sets through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[ent("/transform/translation", "move-object", &[], "translation"), ent("/transform/rotation", "rotate-object", &[], "rotation"), ent("/transform/scale", "scale-object", &[], "scale")],
    inserts: &[],
    removes: &[],
};

const SLOTS: &[Slot] = &[
    Slot { field: "brep", create: "create-brep", delete: "delete-brep" },
    Slot { field: "mesh", create: "create-mesh", delete: "delete-mesh" },
    Slot { field: "properties", create: "create-properties", delete: "delete-properties" },
];

/// 🪆️ Attaching or detaching a child is its `create-*` or `delete-*` kind; every other edit goes through [`EDIT_RULES`].
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioObjectSnapshot) -> Result<Option<Vec<SemioObjectMutation>>, Fault> {
    match slot_edit::<SemioObjectSnapshot, SemioObjectMutation>(SLOTS, spread_snake, snapshot, event)? {
        Some(mutations) => Ok(Some(mutations)),
        None => resolve::<SemioObjectSnapshot, SemioObjectMutation>(&EDIT_RULES, &[], snapshot, event).map(Some),
    }
}
