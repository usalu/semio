//! 🧭️ The details-pane vocabulary of the dxf editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, ItemField, RemoveRule, Selector};

/// 📚 Name-keyed rows (header variables, layers, styles, linetypes) are addressed by their name, position-keyed rows (blocks, entities) by their index.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/headerVars/*", "set-header-var", "headerVar").selecting(&[Selector::Field { payload: "name", field: "name" }]),
        EntityRule::new("/tables/layers/*", "set-layer", "layer").selecting(&[Selector::Field { payload: "name", field: "name" }]),
        EntityRule::new("/tables/styles/*", "set-style", "style").selecting(&[Selector::Field { payload: "name", field: "name" }]),
        EntityRule::new("/tables/linetypes/*", "set-linetype", "linetype").selecting(&[Selector::Field { payload: "name", field: "name" }]),
        EntityRule::new("/entities/*", "set-entity", "entity").selecting(&[Selector::Index("index")]),
        EntityRule::new("/blocks/*", "set-block", "block").selecting(&[Selector::Index("index")]),
        EntityRule::new("/otherTables", "set-other-tables", "otherTables"),
    ],
    inserts: &[
        InsertRule::new("/headerVars", "set-header-var", "headerVar").at("index").keyed(&[ItemField { payload: "name", field: "name" }]),
        InsertRule::new("/tables/layers", "insert-layer", "layer").at("index"),
        InsertRule::new("/tables/styles", "insert-style", "style").at("index"),
        InsertRule::new("/tables/linetypes", "insert-linetype", "linetype").at("index"),
        InsertRule::new("/entities", "insert-entity", "entity").at("index"),
        InsertRule::new("/blocks", "insert-block", "block").at("index"),
    ],
    removes: &[
        RemoveRule::by_key("/headerVars", "remove-header-var", "name", "name"),
        RemoveRule::by_key("/tables/layers", "remove-layer", "name", "name"),
        RemoveRule::by_key("/tables/styles", "remove-style", "name", "name"),
        RemoveRule::by_key("/tables/linetypes", "remove-linetype", "name", "name"),
        RemoveRule::by_index("/entities", "remove-entity", "index"),
        RemoveRule::by_index("/blocks", "remove-block", "index"),
    ],
};
