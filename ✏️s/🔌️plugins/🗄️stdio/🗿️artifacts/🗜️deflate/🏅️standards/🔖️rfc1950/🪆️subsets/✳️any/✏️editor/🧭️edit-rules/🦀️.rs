//! 🧭️ The details-pane vocabulary of the deflate editor: which snapshot pointer raises which concrete kind. The three stream-header
//! fields travel together in `set-compression-params`.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule};

/// 📚 A header field raises the params kind carrying the two it leaves unchanged; the preset dictionary and the payload have their own kinds.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/compressionMethod", "set-compression-params", "method").carrying(&[Carried { payload: "window_bits", pointer: "/windowBits" }, Carried { payload: "level_hint", pointer: "/compressionLevelHint" }]),
        EntityRule::new("/windowBits", "set-compression-params", "window_bits").carrying(&[Carried { payload: "method", pointer: "/compressionMethod" }, Carried { payload: "level_hint", pointer: "/compressionLevelHint" }]),
        EntityRule::new("/compressionLevelHint", "set-compression-params", "level_hint").carrying(&[Carried { payload: "method", pointer: "/compressionMethod" }, Carried { payload: "window_bits", pointer: "/windowBits" }]),
        EntityRule::new("/dictId", "set-preset-dictionary", "dict_id"),
        EntityRule::new("/payload", "set-payload", "payload"),
    ],
    inserts: &[],
    removes: &[],
};
