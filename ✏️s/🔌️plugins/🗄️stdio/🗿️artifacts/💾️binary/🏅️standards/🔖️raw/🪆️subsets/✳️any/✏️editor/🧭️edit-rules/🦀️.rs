//! 🧭️ The details-pane vocabulary of the binary editor: every pointer edit of `/bytes` is answered by the editor as the byte splice it
//! means (a byte set, insert, remove or move; the whole list set replaces the buffer), so the table itself is empty.

use semio_s_artifact_stdio_contract::editing::EditRules;

/// 📚 No fixed pointer shape: see the editor's `snapshot_edit_special`.
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };
