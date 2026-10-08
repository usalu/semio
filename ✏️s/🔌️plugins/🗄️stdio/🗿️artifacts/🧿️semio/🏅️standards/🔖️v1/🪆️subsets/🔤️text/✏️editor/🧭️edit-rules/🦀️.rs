//! 🧭️ The text editor's edit rules: which snapshot pointer raises which ONE concrete text mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{ent, ins, list_move, positions, rem, INDEX};
use crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use semio_framework_plugin::Fault;
use semio_s_artifact_stdio_contract::editing::{EditRules, RowKey, Selector, SnapshotEditEvent};

const RUN: Selector = Selector::Index("run_index");

/// 📚 Every pointer a text editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[ent("/runs/*/content", "edit-run", &[INDEX], "new_text"), ent("/runs/*/language", "change-run-language", &[INDEX], "new_language")],
    inserts: &[ins("/runs", "insert-run", &[], Some("index"), "run"), ins("/runs/*/marks", "add-mark", &[RUN], Some("index"), "mark")],
    removes: &[rem("/runs", "remove-run", &[], RowKey::Index("index")), rem("/runs/*/marks", "remove-mark", &[RUN], RowKey::Index("index"))],
};

/// 🔀️ Moving a run is `reorder-runs`; every other edit goes through [`EDIT_RULES`].
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioTextSnapshot) -> Result<Option<Vec<SemioTextMutation>>, Fault> {
    match list_move(event, &["runs"])? {
        Some((from, to)) => positions(snapshot, event, "reorder-runs", &[("from", from), ("to", to)]).map(Some),
        None => Ok(None),
    }
}
