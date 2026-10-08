//! 🧭️ The image editor's edit rules: which snapshot pointer raises which ONE concrete image mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{complete, ent, ins, keyed, list_move, named, positions, rem, resolve, spread_snake, Entries, Reshape, INDEX, ITEM};
use crate::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, SnapshotEditEvent};

/// 📚 Every pointer an image editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/width", "set-dimensions", &[], "width"),
        ent("/height", "set-dimensions", &[], "height"),
        ent("/colorspace", "set-colorspace", &[], "colorspace"),
        ent("/bitDepth", "set-bit-depth", &[], "bit_depth"),
        ent("/icc", "set-icc", &[], "icc"),
        ent("/frames/*/delayMs", "set-frame-delay", &[INDEX], "delay_ms"),
        ent("/frames/*/rgba8", "set-frame-pixels", &[INDEX], "rgba8"),
        ent("/metadata/*/value", "set-metadata-entry", &[named("key", "key")], "value"),
    ],
    inserts: &[ins("/frames", "insert-frame", &[], Some("index"), "frame"), ins("/metadata", "set-metadata-entry", &[], Some("at"), ITEM)],
    removes: &[rem("/frames", "remove-frame", &[], RowKey::Index("index")), rem("/metadata", "remove-metadata-entry", &[], keyed("key", "key"))],
};

const RESHAPES: &[(&str, Reshape)] = &[("set-dimensions", dimensions), ("set-metadata-entry", spread_snake)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimensions(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    complete(entries, tree, &["width", "height"])
}

/// 🔀️ Moving a frame is `move-frame`; every other edit goes through [`EDIT_RULES`] and completes its payload.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioImageSnapshot) -> Result<Option<Vec<SemioImageMutation>>, Fault> {
    match list_move(event, &["frames"])? {
        Some((from, to)) => positions(snapshot, event, "move-frame", &[("from", from), ("to", to)]).map(Some),
        None => resolve::<SemioImageSnapshot, SemioImageMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some),
    }
}
