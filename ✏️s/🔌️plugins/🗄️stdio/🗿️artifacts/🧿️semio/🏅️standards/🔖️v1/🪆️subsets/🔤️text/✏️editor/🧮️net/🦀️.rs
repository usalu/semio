//! 🧮️ Net of one snapshot edit as text domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `text` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_ordered, NetStep};
use crate::standards::v1::subsets::text::schema::mutations::{add_mark, change_run_language, edit_run, insert_run, remove_mark, remove_run, SemioTextMutation};
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioTextSnapshot, next: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    let mut out = Vec::new();
    for step in net_ordered(&base.runs, &next.runs) {
        match step {
            NetStep::Modify { index, item } => net_run(index, &base.runs[index], item, &mut out),
            NetStep::Remove { index } => out.push(SemioTextMutation::RemoveRun(remove_run::RemoveRun { index })),
            NetStep::Insert { index, item } => out.push(SemioTextMutation::InsertRun(insert_run::InsertRun { index, run: item.clone() })),
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_run(run_index: usize, before: &SemioTextRun, after: &SemioTextRun, out: &mut Vec<SemioTextMutation>) {
    if before.content != after.content {
        out.push(SemioTextMutation::EditRun(edit_run::EditRun { index: run_index, new_content: after.content.clone() }));
    }
    if before.language != after.language {
        out.push(SemioTextMutation::ChangeRunLanguage(change_run_language::ChangeRunLanguage { index: run_index, new_language: after.language.clone() }));
    }
    for step in net_ordered(&before.marks, &after.marks) {
        match step {
            NetStep::Modify { index, item } => {
                out.push(SemioTextMutation::RemoveMark(remove_mark::RemoveMark { run_index, index }));
                out.push(SemioTextMutation::AddMark(add_mark::AddMark { run_index, index, mark: item.clone() }));
            }
            NetStep::Remove { index } => out.push(SemioTextMutation::RemoveMark(remove_mark::RemoveMark { run_index, index })),
            NetStep::Insert { index, item } => out.push(SemioTextMutation::AddMark(add_mark::AddMark { run_index, index, mark: item.clone() })),
        }
    }
}
