//! 🧭️ The details-pane vocabulary of the stl editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

/// 📚 A triangle's normal and vertices are edited through their own kinds; a whole-triangle set is answered by the editor as both.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/solidName", "set-solid-name", "name"),
        EntityRule::new("/triangles/*/normal", "set-triangle-normal", "normal").selecting(&[Selector::Index("index")]),
        EntityRule::new("/triangles/*/vertices", "set-triangle-vertices", "vertices").selecting(&[Selector::Index("index")]),
    ],
    inserts: &[InsertRule::new("/triangles", "insert-triangle", "triangle").at("index")],
    removes: &[RemoveRule::by_index("/triangles", "remove-triangle", "index")],
};
