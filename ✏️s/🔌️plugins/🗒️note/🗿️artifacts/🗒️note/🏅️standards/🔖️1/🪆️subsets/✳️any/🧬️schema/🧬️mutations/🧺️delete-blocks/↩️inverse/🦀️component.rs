//! ↩️ Inverse for `DeleteBlocks`.
use super::mutation::DeleteBlocks;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::schema::mutations::CreateBlock;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteBlocks, base: &NoteSnapshot) -> Vec<NoteMutation> {
    // 🩹 Pre-existing bug fixed here (confirmed via `git log --date=iso`: this file was authored
    // 2026-08-12 15:50:51 by an unrelated wave, unrelated to composition — never touched `content`/
    // `paragraphs`). Every `MutationKind::inverse` caller (`protocol::testkit::assert_mutation_inverse_law`
    // included) reverses the returned Vec before applying each step in turn. Each `CreateBlock` step
    // reinserts at its ORIGINAL absolute index from `base`, which is only valid if the lowest index is
    // inserted FIRST (so it never gets pushed rightward by an insert that hasn't happened yet). Sorting
    // ascending and letting the caller's `.reverse()` flip that to descending-first was exactly backwards
    // — descending here becomes ascending after that reversal, restoring the correct original order.
    let mut entries: Vec<(Option<String>, usize, crate::artifacts::note::NoteBlockNode)> = payload.ids.iter().filter_map(|id| {
        let block = crate::artifacts::note::schema::find_block(&base.blocks, id)?.clone();
        let (parent_id, index) = crate::artifacts::note::schema::find_block_location(&base.blocks, id)?;
        Some((parent_id, index, block))
    }).collect();
    entries.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    entries.into_iter().map(|(parent_id, index, block)| NoteMutation::CreateBlock(CreateBlock { block: Box::new(block), parent_id, index: Some(index) })).collect()
}
//#endregion 🔖️Inverse
