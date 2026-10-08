//! 🧭️ The audio editor's edit rules: which snapshot pointer raises which ONE concrete audio mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, rem, INDEX};
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey};

/// 📚 Every pointer an audio editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/sampleRate", "set-sample-rate", &[], "sampleRate"),
        ent("/format", "set-format", &[], "format"),
        ent("/channels/*/samples", "set-channel-samples", &[INDEX], "samples"),
        ent("/tags/*/value", "set-tag-value", &[INDEX], "value"),
    ],
    inserts: &[ins("/channels", "insert-channel", &[], Some("index"), "channel"), ins("/tags", "insert-tag", &[], Some("index"), "tag")],
    removes: &[rem("/channels", "remove-channel", &[], RowKey::Index("index")), rem("/tags", "remove-tag", &[], RowKey::Index("index"))],
};
