//! 🧭️ The model editor's edit rules: which snapshot pointer raises which ONE concrete model mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, keyed, named, rem};
use semio_s_artifact_stdio_contract::editing::{EditRules, Selector};

const ID: Selector = named("id", "id");

/// 📚 Every pointer a model editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/spatial/*/kind", "set-spatial-node", &[ID], "kind"),
        ent("/spatial/*/name", "set-spatial-node", &[ID], "name"),
        ent("/spatial/*/parentId", "set-spatial-node", &[ID], "parentId"),
        ent("/spatial/*/placement", "set-spatial-node", &[ID], "placement"),
        ent("/elements/*/class", "set-element", &[ID], "class"),
        ent("/elements/*/placement", "set-element", &[ID], "placement"),
        ent("/elements/*/geometry", "set-element", &[ID], "geometry"),
        ent("/elements/*/spatialId", "set-element", &[ID], "spatialId"),
        ent("/elements/*/psets", "set-element", &[ID], "psets"),
        ent("/relations/*/kind", "set-relation", &[ID], "kind"),
        ent("/relations/*/from", "set-relation", &[ID], "from"),
        ent("/relations/*/to", "set-relation", &[ID], "to"),
    ],
    inserts: &[
        ins("/spatial", "insert-spatial-node", &[], Some("at"), "node"),
        ins("/elements", "insert-element", &[], Some("at"), "element"),
        ins("/relations", "insert-relation", &[], Some("at"), "relation"),
    ],
    removes: &[rem("/spatial", "remove-spatial-node", &[], keyed("id", "id")), rem("/elements", "remove-element", &[], keyed("id", "id")), rem("/relations", "remove-relation", &[], keyed("id", "id"))],
};
