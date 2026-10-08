//! 🧭️ The details-pane vocabulary of the las editor: which snapshot pointer raises which concrete kind.
//! Header groups (version, date, scale and offset, bounds, …) and VLR identity edits are computed gestures answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

/// 📚 Points and VLR data are addressed by position.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/points/*", "set-point", "point").selecting(&[Selector::Index("index")]), EntityRule::new("/vlrs/*/data", "set-vlr-data", "data").selecting(&[Selector::Index("index")])],
    inserts: &[InsertRule::new("/points", "insert-point", "point").at("index"), InsertRule::new("/vlrs", "insert-vlr", "vlr").at("index")],
    removes: &[RemoveRule::by_index("/points", "remove-point", "index"), RemoveRule::by_index("/vlrs", "remove-vlr", "index")],
};
