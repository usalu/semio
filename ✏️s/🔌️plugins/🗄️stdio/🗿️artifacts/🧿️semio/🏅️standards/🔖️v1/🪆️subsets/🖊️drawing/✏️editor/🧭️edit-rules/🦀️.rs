//! 🧭️ The drawing editor's edit rules: which snapshot pointer raises which ONE concrete drawing mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, keyed, named, rem};
use semio_s_artifact_stdio_contract::editing::{EditRules, Selector};

const STYLE: Selector = named("style_name", "name");

/// 📚 Every pointer a drawing editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/styles/*/fill", "replace-fill", &[STYLE], "new_fill"),
        ent("/styles/*/stroke", "change-stroke-color", &[STYLE], "new_color"),
        ent("/styles/*/strokeWidth", "change-stroke-width", &[STYLE], "new_width"),
    ],
    inserts: &[ins("/layers", "create-layer", &[], Some("index"), "layer")],
    removes: &[rem("/layers", "delete-layer", &[], keyed("id", "id"))],
};
