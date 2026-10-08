//! 🧭️ The details-pane vocabulary of the png editor: which snapshot pointer raises which concrete kind.
//! A single sample edit (the pixel holding it) is a computed gesture answered by the editor itself; every other image metadata edit replaces the image through its replace kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule};

/// 📚 The gamma has its own setter; any other edit inside the image is the image's replace kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/image/gamma", "set-gamma", "gama"), EntityRule::new("/image", "replace-image", "image")],
    inserts: &[],
    removes: &[],
};
