//! 🧭️ The details-pane vocabulary of the mp4 editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule, InsertRule, RemoveRule, Selector};

const TRACK: &[Selector] = &[Selector::Index("trackIndex")];
const SAMPLE: &[Selector] = &[Selector::Index("trackIndex"), Selector::Index("index")];

/// 📚 Tracks and samples are addressed by position; a track's width and height travel together in `set-track-dimensions`.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/ftyp", "set-ftyp", "ftyp"),
        EntityRule::new("/movie", "set-movie", "movie"),
        EntityRule::new("/tracks/*/codec", "set-track-codec", "codec").selecting(TRACK),
        EntityRule::new("/tracks/*/width", "set-track-dimensions", "width").selecting(TRACK).carrying(&[Carried { payload: "height", pointer: "/tracks/*/height" }]),
        EntityRule::new("/tracks/*/height", "set-track-dimensions", "height").selecting(TRACK).carrying(&[Carried { payload: "width", pointer: "/tracks/*/width" }]),
        EntityRule::new("/tracks/*/samples/*/sync", "set-sample-sync", "sync").selecting(SAMPLE),
    ],
    inserts: &[InsertRule::new("/tracks", "insert-track", "track").at("index"), InsertRule::new("/tracks/*/samples", "insert-sample", "sample").at("index").selecting(TRACK)],
    removes: &[RemoveRule::by_index("/tracks", "remove-track", "index"), RemoveRule::by_index("/tracks/*/samples", "remove-sample", "index").selecting(TRACK)],
};
