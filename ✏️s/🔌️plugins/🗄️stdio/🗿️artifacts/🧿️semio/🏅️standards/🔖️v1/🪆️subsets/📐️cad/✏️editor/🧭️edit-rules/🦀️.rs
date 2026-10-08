//! 🧭️ The CAD editor's edit rules: which snapshot pointer raises which ONE concrete CAD mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, keyed, named, rem};
use semio_s_artifact_stdio_contract::editing::{EditRules, Selector};

const LAYER: Selector = named("name", "name");
const BLOCK: Selector = named("name", "name");
const OWNER: Selector = named("blockName", "name");
const HANDLE: Selector = named("handle", "handle");

/// 📚 Every pointer a CAD editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/layers/*/colorIndex", "set-layer", &[LAYER], "colorIndex"),
        ent("/layers/*/lineType", "set-layer", &[LAYER], "lineType"),
        ent("/layers/*/visible", "set-layer", &[LAYER], "visible"),
        ent("/blocks/*/basePoint", "set-block-base-point", &[BLOCK], "basePoint"),
        ent("/blocks/*/entities/*/layer", "set-block-entity-layer", &[OWNER, HANDLE], "layer"),
        ent("/blocks/*/entities/*/entity", "set-block-entity-geometry", &[OWNER, HANDLE], "entity"),
        ent("/entities/*/layer", "set-entity-layer", &[HANDLE], "layer"),
        ent("/entities/*/entity", "set-entity-geometry", &[HANDLE], "entity"),
    ],
    inserts: &[
        ins("/layers", "add-layer", &[], Some("at"), "layer"),
        ins("/blocks", "add-block", &[], Some("at"), "block"),
        ins("/entities", "add-entity", &[], Some("at"), "entity"),
        ins("/blocks/*/entities", "add-block-entity", &[OWNER], Some("at"), "entity"),
    ],
    removes: &[
        rem("/layers", "remove-layer", &[], keyed("name", "name")),
        rem("/blocks", "remove-block", &[], keyed("name", "name")),
        rem("/entities", "remove-entity", &[], keyed("handle", "handle")),
        rem("/blocks/*/entities", "remove-block-entity", &[OWNER], keyed("handle", "handle")),
    ],
};
