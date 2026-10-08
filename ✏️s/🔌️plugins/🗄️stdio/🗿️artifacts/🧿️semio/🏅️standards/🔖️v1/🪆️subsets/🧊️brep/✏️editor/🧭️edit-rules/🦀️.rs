//! 🧭️ The B-rep editor's edit rules: which snapshot pointer raises which ONE concrete B-rep mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, keyed, named, rem, resolve, spread_snake, Reshape, ITEM};
use crate::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_framework_plugin::Fault;
use semio_s_artifact_stdio_contract::editing::{EditRules, SnapshotEditEvent};

/// 📚 Every pointer a B-rep editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/vertices/*/point", "move-vertex", &[named("vertex_id", "id")], "new_point"),
        ent("/edges/*/curve", "replace-curve", &[named("edge_id", "id")], "new_curve"),
        ent("/faces/*/surface", "replace-surface", &[named("face_id", "id")], "new_surface"),
    ],
    inserts: &[
        ins("/vertices", "create-vertex", &[], Some("at"), ITEM),
        ins("/edges", "create-edge", &[], Some("at"), ITEM),
        ins("/faces", "create-face", &[], Some("at"), ITEM),
        ins("/shells", "create-shell", &[], Some("at"), ITEM),
        ins("/solids", "create-solid", &[], Some("at"), ITEM),
    ],
    removes: &[
        rem("/vertices", "delete-vertex", &[], keyed("id", "id")),
        rem("/edges", "delete-edge", &[], keyed("id", "id")),
        rem("/faces", "delete-face", &[], keyed("id", "id")),
        rem("/shells", "delete-shell", &[], keyed("id", "id")),
        rem("/solids", "delete-solid", &[], keyed("id", "id")),
    ],
};

const RESHAPES: &[(&str, Reshape)] = &[("create-vertex", spread_snake), ("create-edge", spread_snake), ("create-face", spread_snake), ("create-shell", spread_snake), ("create-solid", spread_snake)];

/// 🧩️ Every edit goes through [`EDIT_RULES`]; a `create-*` kind spreads the inserted topology row into its payload.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioBrepSnapshot) -> Result<Option<Vec<SemioBrepMutation>>, Fault> {
    resolve::<SemioBrepSnapshot, SemioBrepMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some)
}
