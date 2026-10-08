//! 🧭️ The presentation editor's edit rules: which snapshot pointer raises which ONE concrete presentation mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, keyed, named, rem, INDEX};
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector};

const SLIDE: Selector = Selector::Index("slide_index");
const SHAPE: Selector = Selector::Index("shape_index");

/// 📚 Every pointer a presentation editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/layouts/*/masterId", "set-layout-master", &[named("id", "id")], "master_id"),
        ent("/slides/*/layoutId", "set-slide-layout", &[INDEX], "layout_id"),
        ent("/slides/*/notes", "set-slide-notes", &[INDEX], "notes"),
        ent("/slides/*/shapes/*/frame", "set-shape-frame", &[SLIDE, SHAPE], "frame"),
        ent("/slides/*/shapes/*/blocks", "set-text-box-blocks", &[SLIDE, SHAPE], "blocks"),
    ],
    inserts: &[
        ins("/masters", "insert-master", &[], Some("at"), "master"),
        ins("/layouts", "insert-layout", &[], Some("at"), "layout"),
        ins("/slides", "insert-slide", &[], Some("index"), "slide"),
        ins("/slides/*/shapes", "insert-shape", &[SLIDE], Some("shape_index"), "shape"),
    ],
    removes: &[
        rem("/masters", "remove-master", &[], keyed("id", "id")),
        rem("/layouts", "remove-layout", &[], keyed("id", "id")),
        rem("/slides", "remove-slide", &[], RowKey::Index("index")),
        rem("/slides/*/shapes", "remove-shape", &[SLIDE], RowKey::Index("shape_index")),
    ],
};
