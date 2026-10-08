//! 🧭️ The details-pane vocabulary of the wav editor: which snapshot pointer raises which concrete kind.
//! Sample-level edits under `/data/value/<i>` and the `/data/kind` retag are computed gestures answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule};

/// 📚 One entity per kind: a field edit replaces its whole entity through the kind that sets it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/fmt", "set-fmt", "fmt"),
        EntityRule::new("/data", "set-data", "data"),
        EntityRule::new("/otherChunks", "set-other-chunks", "chunks"),
        EntityRule::new("/chunkOrder", "set-other-chunks", "chunkOrder").carrying(&[Carried { payload: "chunks", pointer: "/otherChunks" }]),
        EntityRule::new("/fmtPadByte", "set-pad-bytes", "fmtPadByte").carrying(&[Carried { payload: "dataPadByte", pointer: "/dataPadByte" }]),
        EntityRule::new("/dataPadByte", "set-pad-bytes", "dataPadByte").carrying(&[Carried { payload: "fmtPadByte", pointer: "/fmtPadByte" }]),
    ],
    inserts: &[],
    removes: &[],
};
