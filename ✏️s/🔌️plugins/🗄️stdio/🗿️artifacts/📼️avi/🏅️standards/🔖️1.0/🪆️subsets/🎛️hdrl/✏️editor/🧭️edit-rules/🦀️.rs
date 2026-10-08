//! 🧭️ The details-pane vocabulary of the avi editor: which snapshot pointer raises which concrete kind.
//! A chunk or unknown-chunk content edit (a remove then an insert at the same position) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

/// 📚 Streams, chunks and unknown chunks are addressed by position.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/mainHeader", "set-main-header", "mainHeader"),
        EntityRule::new("/idx1Present", "set-idx1-present", "idx1Present"),
        EntityRule::new("/hdrlExtra", "set-hdrl-extra", "chunks"),
        EntityRule::new("/streams/*/strh", "set-stream-header", "strh").selecting(&[Selector::Index("streamIndex")]),
        EntityRule::new("/streams/*/strf", "set-stream-format", "strf").selecting(&[Selector::Index("streamIndex")]),
        EntityRule::new("/streams/*/chunks/*/keyframe", "set-chunk-keyframe", "keyframe").selecting(&[Selector::Index("streamIndex"), Selector::Index("index")]),
    ],
    inserts: &[
        InsertRule::new("/streams", "insert-stream", "stream").at("index"),
        InsertRule::new("/streams/*/chunks", "insert-chunk", "chunk").at("index").selecting(&[Selector::Index("streamIndex")]),
        InsertRule::new("/unknownChunks", "add-unknown-chunk", "item").at("index"),
    ],
    removes: &[
        RemoveRule::by_index("/streams", "remove-stream", "index"),
        RemoveRule::by_index("/streams/*/chunks", "remove-chunk", "index").selecting(&[Selector::Index("streamIndex")]),
        RemoveRule::by_index("/unknownChunks", "remove-unknown-chunk", "index"),
    ],
};
