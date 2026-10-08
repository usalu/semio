//! 🧭️ The video editor's edit rules: which snapshot pointer raises which ONE concrete video mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{complete_in, ent, ins, rem, resolve, Entries, Reshape, INDEX};
use crate::standards::v1::subsets::video::schema::mutations::SemioVideoMutation;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector, SnapshotEditEvent};

const STREAM: Selector = Selector::Index("streamIndex");
const AT: RowKey = RowKey::Index("index");

/// 📚 Every pointer a video editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/streams/*/kind", "set-stream-meta", &[INDEX], "kind"),
        ent("/streams/*/codec", "set-stream-meta", &[INDEX], "codec"),
        ent("/streams/*/width", "set-stream-meta", &[INDEX], "width"),
        ent("/streams/*/height", "set-stream-meta", &[INDEX], "height"),
        ent("/streams/*/rate", "set-stream-meta", &[INDEX], "rate"),
        ent("/streams/*/samples/*/data", "set-sample-data", &[STREAM, INDEX], "data"),
        ent("/streams/*/samples/*/pts", "set-sample-flags", &[STREAM, INDEX], "pts"),
        ent("/streams/*/samples/*/key", "set-sample-flags", &[STREAM, INDEX], "key"),
    ],
    inserts: &[ins("/streams", "insert-stream", &[], Some("index"), "stream"), ins("/streams/*/samples", "insert-sample", &[STREAM], Some("index"), "sample")],
    removes: &[rem("/streams", "remove-stream", &[], AT), rem("/streams/*/samples", "remove-sample", &[STREAM], AT)],
};

const RESHAPES: &[(&str, Reshape)] = &[("set-stream-meta", stream_meta), ("set-sample-flags", sample_flags)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_meta(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete_in(tree, entries, &[("streams", "index")], &["kind", "codec", "width", "height", "rate"])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_flags(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete_in(tree, entries, &[("streams", "streamIndex"), ("samples", "index")], &["pts", "key"])
}

/// 🧩️ Every edit goes through [`EDIT_RULES`]; a stream or sample setter carries the members it keeps.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioVideoSnapshot) -> Result<Option<Vec<SemioVideoMutation>>, Fault> {
    resolve::<SemioVideoSnapshot, SemioVideoMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some)
}
