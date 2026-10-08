//! 🧭️ The details-pane vocabulary of the bmp editor: which snapshot pointer raises which concrete kind.
//! A single sample edit (the pixel holding it) is a computed gesture answered by the editor itself as one `replace-samples` row; every other image edit replaces the image through its replace kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule};

/// 📚 Any edit inside the image is the image's replace kind.
pub const EDIT_RULES: EditRules = EditRules { entities: &[EntityRule::new("/image", "replace-image", "image")], inserts: &[], removes: &[] };
