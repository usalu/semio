//! 🧬️ Transparent PDF 1.4/ANY mutation registry and delegation.

use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;
pub use insert_page::InsertPage;
#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;
pub use remove_page::RemovePage;
#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;
pub use move_page::MovePage;
#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;
pub use resize_page::ResizePage;
#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;
pub use replace_page_text::ReplacePageText;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4")]
pub enum PdfMutation {
    InsertPage(InsertPage),
    RemovePage(RemovePage),
    MovePage(MovePage),
    ResizePage(ResizePage),
    ReplacePageText(ReplacePageText),
}

//#endregion 🔖️Aggregate

//#region 🔖️Net
/// 🧮️ The concrete leaves carrying `base` to `next`: a single moved page as one move, the pages that changed in place as a resize
/// and/or a text replacement each, then the surplus pages removed from the end or inserted at their final positions. Replaying
/// them through the central applier is the proof the net is exact; a change to the schema marker is left out, so the replay then
/// refuses the edit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_mutations(base: &PdfSnapshot, next: &PdfSnapshot) -> Vec<PdfMutation> {
    let (before, after) = (&base.pages, &next.pages);
    let prefix = before.iter().zip(after).take_while(|(left, right)| left == right).count();
    let suffix = before[prefix..].iter().rev().zip(after[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
    let (old, new) = (&before[prefix..before.len() - suffix], &after[prefix..after.len() - suffix]);
    if old.len() == new.len() && old.len() >= 2 {
        let last = old.len() - 1;
        if old[1..] == new[..last] && old[0] == new[last] {
            return vec![PdfMutation::MovePage(MovePage { from: prefix, to: prefix + last })];
        }
        if old[..last] == new[1..] && old[last] == new[0] {
            return vec![PdfMutation::MovePage(MovePage { from: prefix + last, to: prefix })];
        }
    }
    let mut leaves = Vec::new();
    let paired = old.len().min(new.len());
    for offset in 0..paired {
        let (previous, page) = (&old[offset], &new[offset]);
        if previous.width.to_bits() != page.width.to_bits() || previous.height.to_bits() != page.height.to_bits() {
            leaves.push(PdfMutation::ResizePage(ResizePage { index: prefix + offset, width: page.width, height: page.height }));
        }
        if previous.text != page.text {
            leaves.push(PdfMutation::ReplacePageText(ReplacePageText { index: prefix + offset, text: page.text.clone() }));
        }
    }
    for index in (prefix + paired..prefix + old.len()).rev() {
        leaves.push(PdfMutation::RemovePage(RemovePage { index }));
    }
    for offset in paired..new.len() {
        leaves.push(PdfMutation::InsertPage(InsertPage { index: prefix + offset, page: new[offset].clone() }));
    }
    leaves
}
//#endregion 🔖️Net

//#region 🔖️Delegation
/// 🛡️ Applies `outcome` to `snapshot` atomically through the central applier and converts an apply rejection into a fatal outcome.
pub fn apply_outcome(outcome: protocol::MutationOutcome<PdfDiff>, snapshot: &mut PdfSnapshot) -> protocol::MutationOutcome<PdfDiff> {
    let (diff, messages) = outcome.into_parts();
    match protocol::apply_diff(&diff, snapshot) {
        Ok(next) => {
            *snapshot = next;
            protocol::MutationOutcome::new(diff).absorb_messages(messages)
        }
        Err(error) => protocol::MutationOutcome::new(PdfDiff::default()).absorb_messages(messages).absorb_messages([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]),
    }
}

/// ▶️ Applies the authoritative leaf diff.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    apply_outcome(outcome, snapshot)
}

//#endregion 🔖️Delegation

//#region 🔖️Codecs
//#endregion 🔖️Codecs

//#region 🧪️Structure
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Structure
