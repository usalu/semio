//! 🧭️ The details-pane vocabulary of the idml editor: a part's bytes are replaced as a whole, addressed by the part path.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, Selector};

const PART: &[Selector] = &[Selector::Field { payload: "path", field: "path" }];

/// 📚 Parts are addressed by their unique package path.
pub const EDIT_RULES: EditRules = EditRules { entities: &[EntityRule::new("/parts/*/bytes", "replace-part", "bytes").selecting(PART)], inserts: &[], removes: &[] };
