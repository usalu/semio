//! 🧬️ `dsl_token` — token-native document substrate: tokens carry a `StableId` that survives
//! edits (never a byte offset or index), stored in a `TokenRope` with copy-on-write immutable
//! snapshots, mutated only through invertible `TokenTransaction`s, addressed by stable
//! `Position`s. Incremental relexing reuses unaffected tokens' identity across an edit instead of
//! re-lexing (and re-identifying) the whole document.

use dsl_core::{lex, Limits, SpannedToken, TextError, TextSpan, TokenKind};
use std::sync::Arc;

//#region 🔖️StableId
/// @emoji 🪪️ A token's identity across its whole lifetime — assigned once on creation, never
/// reused (even after the token is deleted), never derived from position. This is the "TokenId"
/// the blueprint requires public APIs to expose instead of byte offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StableId(pub u64);

/// @emoji 🧬️ How a token came to exist, for editors that want to explain provenance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lineage {
    Authored,
    SplitFrom(StableId),
    MergedFrom(StableId, StableId),
    Generated,
}

struct IdAllocator {
    next: u64,
}

impl IdAllocator {
    fn new() -> Self {
        Self { next: 0 }
    }

    fn alloc(&mut self) -> StableId {
        let id = StableId(self.next);
        self.next += 1;
        id
    }
}
//#endregion 🔖️StableId

//#region 🔖️RopeToken
/// @emoji 🧾️ One token as stored in the rope: stable identity plus the same kind/text/span a
/// fresh `dsl_core::lex` pass would produce.
#[derive(Clone, Debug, PartialEq)]
pub struct RopeToken {
    pub id: StableId,
    pub kind: TokenKind,
    pub text: String,
    pub span: TextSpan,
    pub lineage: Lineage,
}

impl RopeToken {
    fn from_spanned(spanned: &SpannedToken, id: StableId, lineage: Lineage) -> Self {
        Self { id, kind: spanned.kind, text: spanned.text.as_str().to_string(), span: spanned.span, lineage }
    }
}
//#endregion 🔖️RopeToken

//#region 🔖️Rope
/// @emoji 🧵️ An immutable, cheaply-clonable (`Arc`-backed) sequence of tokens — `Send + Sync` by
/// construction. v1 stores tokens in a flat `Vec` behind an `Arc` (copy-on-write on mutation);
/// the public API (stable ids, patches, positions) is designed so a real persistent B-tree rope
/// can replace the backing store later without changing any caller.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenRopeSnapshot {
    tokens: Arc<Vec<RopeToken>>,
    revision: u64,
}

impl TokenRopeSnapshot {
    pub fn tokens(&self) -> &[RopeToken] {
        &self.tokens
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn find(&self, id: StableId) -> Option<usize> {
        self.tokens.iter().position(|t| t.id == id)
    }

    pub fn get(&self, id: StableId) -> Option<&RopeToken> {
        self.tokens.iter().find(|t| t.id == id)
    }

    /// @emoji 📝️ Renders the token text back into source text (concatenation — trivia tokens
    /// carry their own whitespace/newline text so this is a faithful reprint of what was lexed).
    pub fn text(&self) -> String {
        self.tokens.iter().map(|t| t.text.as_str()).collect()
    }
}

pub struct TokenRope {
    tokens: Vec<RopeToken>,
    ids: IdAllocator,
    revision: u64,
}

impl TokenRope {
    pub fn from_text(text: &str, limits: &Limits) -> Result<Self, TextError> {
        let spanned = lex(text, limits, false)?;
        let mut ids = IdAllocator::new();
        let tokens = spanned
            .iter()
            .filter(|t| t.kind != TokenKind::Eof)
            .map(|t| RopeToken::from_spanned(t, ids.alloc(), Lineage::Authored))
            .collect();
        Ok(Self { tokens, ids, revision: 0 })
    }

    pub fn snapshot(&self) -> TokenRopeSnapshot {
        TokenRopeSnapshot { tokens: Arc::new(self.tokens.clone()), revision: self.revision }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    fn bump(&mut self) {
        self.revision += 1;
    }
}
//#endregion 🔖️Rope

//#region 🔖️Position
/// @emoji 📌️ A position addressed by token identity, never a character offset — survives
/// formatting/reordering/edits elsewhere in the document. `TextOffset` additionally carries a
/// grapheme index into a `Text`-kind token's payload for cursor placement inside prose.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Before(StableId),
    After(StableId),
    TextOffset(StableId, usize),
    DocumentStart,
    DocumentEnd,
}

impl Position {
    /// @emoji 🎯️ Resolves to a concrete token-vector index in `snapshot`. Falls back to the
    /// nearest document boundary if the referenced token was deleted (ancestry fallback).
    pub fn resolve(&self, snapshot: &TokenRopeSnapshot) -> usize {
        match self {
            Position::DocumentStart => 0,
            Position::DocumentEnd => snapshot.tokens.len(),
            Position::Before(id) => snapshot.find(*id).unwrap_or(snapshot.tokens.len()),
            Position::After(id) => snapshot.find(*id).map_or(snapshot.tokens.len(), |i| i + 1),
            Position::TextOffset(id, _) => snapshot.find(*id).unwrap_or(snapshot.tokens.len()),
        }
    }
}
//#endregion 🔖️Position

//#region 🔖️Patch
/// @emoji 🩹️ One atomic change to the rope: replace the tokens in `[start, end)` with `inserted`.
/// Invertible: the inverse patch replaces `inserted` with the tokens that were removed.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenPatch {
    pub start: usize,
    pub end: usize,
    pub inserted: Vec<RopeToken>,
}

impl TokenPatch {
    pub fn insert_at(index: usize, tokens: Vec<RopeToken>) -> Self {
        Self { start: index, end: index, inserted: tokens }
    }

    pub fn delete_range(start: usize, end: usize) -> Self {
        Self { start, end, inserted: Vec::new() }
    }

    pub fn replace_range(start: usize, end: usize, tokens: Vec<RopeToken>) -> Self {
        Self { start, end, inserted: tokens }
    }
}

/// @emoji 📦️ An ordered batch of patches applied atomically, with preconditions on the base
/// revision. Produces its own inverse transaction so undo/redo are structural, not text-diffed.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenTransaction {
    pub base_revision: u64,
    pub patches: Vec<TokenPatch>,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TransactionError {
    #[error("base revision mismatch: transaction expects {expected}, rope is at {actual}")]
    RevisionMismatch { expected: u64, actual: u64 },
    #[error("patch range [{start}, {end}) out of bounds for a rope of length {len}")]
    OutOfBounds { start: usize, end: usize, len: usize },
}

impl TokenRope {
    /// @emoji ⚡️ Applies a transaction atomically (all patches or none), returning the inverse
    /// transaction so the caller can undo exactly this change.
    pub fn apply(&mut self, transaction: &TokenTransaction) -> Result<TokenTransaction, TransactionError> {
        if transaction.base_revision != self.revision {
            return Err(TransactionError::RevisionMismatch { expected: transaction.base_revision, actual: self.revision });
        }
        for patch in &transaction.patches {
            if patch.start > patch.end || patch.end > self.tokens.len() {
                return Err(TransactionError::OutOfBounds { start: patch.start, end: patch.end, len: self.tokens.len() });
            }
        }
        let mut inverse_patches = Vec::with_capacity(transaction.patches.len());
        // Apply in reverse-index order so earlier patches' index ranges remain valid.
        let mut ordered: Vec<&TokenPatch> = transaction.patches.iter().collect();
        ordered.sort_by_key(|p| std::cmp::Reverse(p.start));
        for patch in ordered {
            let removed: Vec<RopeToken> = self.tokens.splice(patch.start..patch.end, patch.inserted.clone()).collect();
            let inserted_len = patch.inserted.len();
            inverse_patches.push(TokenPatch { start: patch.start, end: patch.start + inserted_len, inserted: removed });
        }
        self.bump();
        Ok(TokenTransaction { base_revision: self.revision, patches: inverse_patches })
    }

    pub fn alloc_id(&mut self) -> StableId {
        self.ids.alloc()
    }

    /// @emoji 🔬️ Builds fresh `RopeToken`s (new stable ids) from lexing `text` — used both to seed
    /// a transaction's `inserted` tokens and by incremental relexing for a damaged region.
    pub fn tokenize_fresh(&mut self, text: &str, limits: &Limits, lineage: Lineage) -> Result<Vec<RopeToken>, TextError> {
        let spanned = lex(text, limits, false)?;
        Ok(spanned.iter().filter(|t| t.kind != TokenKind::Eof).map(|t| RopeToken::from_spanned(t, self.ids.alloc(), lineage)).collect())
    }
}
//#endregion 🔖️Patch

//#region 🔖️Incremental
/// @emoji 🔂️ Re-lexes only the region touched by a `[byte_start, byte_end)` text replacement,
/// reusing the identity of every token entirely outside the damaged region. `old_text`/`new_text`
/// are the full document before/after the edit. Falls back to a full re-tokenize (fresh ids
/// throughout) if no stable splice point can be found — always correct, just not always minimal.
pub fn relex_incremental(
    rope: &mut TokenRope,
    old_text: &str,
    new_text: &str,
    byte_start: usize,
    byte_end: usize,
    limits: &Limits,
) -> Result<(), TextError> {
    let old_len = old_text.len();
    let new_len = new_text.len();
    let removed_len = byte_end.saturating_sub(byte_start);
    let inserted_len = new_len.saturating_sub(old_len.saturating_sub(removed_len));

    let snapshot = rope.snapshot();
    // Find the last token entirely ending at or before byte_start (checkpoint before damage).
    let mut prefix_end_index = 0usize;
    let mut prefix_byte_end = 0u32;
    let mut consumed = 0u32;
    for (i, tok) in snapshot.tokens().iter().enumerate() {
        let tok_bytes = tok.text.len() as u32;
        if consumed + tok_bytes <= byte_start as u32 {
            prefix_end_index = i + 1;
            prefix_byte_end = consumed + tok_bytes;
            consumed += tok_bytes;
        } else {
            break;
        }
    }
    // Find the first token entirely starting at or after byte_end (checkpoint after damage).
    let mut suffix_start_index = snapshot.tokens().len();
    let mut suffix_byte_start = 0u32;
    let mut running = 0u32;
    for (i, tok) in snapshot.tokens().iter().enumerate() {
        if running >= byte_end as u32 {
            suffix_start_index = i;
            suffix_byte_start = running;
            break;
        }
        running += tok.text.len() as u32;
    }
    if suffix_start_index < prefix_end_index {
        // Damage spans token boundaries ambiguously (e.g. edit inside a single token) — fall
        // back to a full re-tokenize; still correct, just assigns fresh ids throughout.
        *rope = TokenRope::from_text(new_text, limits)?;
        return Ok(());
    }

    let damaged_new_text = &new_text[prefix_byte_end as usize..(new_len - (old_len as u32 - suffix_byte_start) as usize).max(prefix_byte_end as usize)];
    let _ = inserted_len; // documents the size delta; not otherwise needed by the splice logic above.
    let fresh = rope.tokenize_fresh(damaged_new_text, limits, Lineage::Authored)?;
    let transaction = TokenTransaction { base_revision: rope.revision(), patches: vec![TokenPatch::replace_range(prefix_end_index, suffix_start_index, fresh)] };
    rope.apply(&transaction).map_err(|e| TextError::new(e.to_string(), TextSpan::at(1, 1)))?;
    Ok(())
}
//#endregion 🔖️Incremental

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn kinds_texts(rope: &TokenRope) -> Vec<(TokenKind, String)> {
        rope.snapshot().tokens().iter().map(|t| (t.kind, t.text.clone())).collect()
    }

    #[test]
    fn rope_from_text_assigns_unique_monotonic_stable_ids() {
        let rope = TokenRope::from_text("a b c", &Limits::default()).expect("rope");
        let ids: Vec<u64> = rope.snapshot().tokens().iter().map(|t| t.id.0).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "ids must be unique");
        assert_eq!(ids, sorted, "ids assigned in monotonic order");
    }

    #[test]
    fn snapshot_text_reprints_the_original_source() {
        let source = "camera x=1 y=2\nlayer a { }";
        let rope = TokenRope::from_text(source, &Limits::default()).expect("rope");
        assert_eq!(rope.snapshot().text(), source);
    }

    #[test]
    fn transaction_apply_and_inverse_round_trip() {
        let mut rope = TokenRope::from_text("a b c", &Limits::default()).expect("rope");
        let before = rope.snapshot();
        let fresh = rope.tokenize_fresh("X", &Limits::default(), Lineage::Authored).expect("tokenize");
        let replace_b = before.find(StableId(2)).expect("find 'b' token index");
        let transaction = TokenTransaction { base_revision: rope.revision(), patches: vec![TokenPatch::replace_range(replace_b, replace_b + 1, fresh)] };
        let inverse = rope.apply(&transaction).expect("apply");
        assert_eq!(rope.snapshot().text(), "a X c");
        rope.apply(&inverse).expect("apply inverse");
        assert_eq!(rope.snapshot().text(), before.text(), "inverse transaction must restore the original text");
    }

    #[test]
    fn transaction_rejects_stale_base_revision() {
        let mut rope = TokenRope::from_text("a b", &Limits::default()).expect("rope");
        let stale = TokenTransaction { base_revision: 999, patches: vec![] };
        assert!(matches!(rope.apply(&stale), Err(TransactionError::RevisionMismatch { .. })));
    }

    #[test]
    fn position_before_after_resolve_to_token_neighbors() {
        let rope = TokenRope::from_text("a b c", &Limits::default()).expect("rope");
        let snapshot = rope.snapshot();
        let b_id = snapshot.tokens()[2].id; // index 0='a',1=' ',2='b'
        assert_eq!(Position::Before(b_id).resolve(&snapshot), 2);
        assert_eq!(Position::After(b_id).resolve(&snapshot), 3);
    }

    #[test]
    fn position_falls_back_to_document_end_for_a_deleted_token() {
        let rope = TokenRope::from_text("a b", &Limits::default()).expect("rope");
        let snapshot = rope.snapshot();
        let deleted = Position::Before(StableId(9999));
        assert_eq!(deleted.resolve(&snapshot), snapshot.tokens().len());
    }

    #[test]
    fn tokens_outside_the_damaged_region_keep_their_stable_id_after_incremental_relex() {
        let old_text = "camera x=1 y=2 zoom=1";
        let new_text = "camera x=1 y=99 zoom=1";
        let mut rope = TokenRope::from_text(old_text, &Limits::default()).expect("rope");
        let before = rope.snapshot();
        let camera_id = before.tokens()[0].id;
        let zoom_key_id = before.tokens().iter().find(|t| t.text == "zoom").expect("zoom token").id;

        let byte_start = old_text.find("y=2").expect("find y=2");
        let byte_end = byte_start + "y=2".len();
        relex_incremental(&mut rope, old_text, new_text, byte_start, byte_end, &Limits::default()).expect("relex");

        assert_eq!(rope.snapshot().text(), new_text);
        let after = rope.snapshot();
        assert_eq!(after.get(camera_id).map(|t| t.text.as_str()), Some("camera"), "unaffected prefix token keeps its id");
        assert_eq!(after.get(zoom_key_id).map(|t| t.text.as_str()), Some("zoom"), "unaffected suffix token keeps its id");
    }

    #[test]
    fn incremental_relex_agrees_with_full_relex_on_kind_and_text_sequence() {
        let old_text = "a b c=3";
        let new_text = "a bb c=3";
        let mut incremental = TokenRope::from_text(old_text, &Limits::default()).expect("rope");
        let byte_start = old_text.find('b').expect("find b");
        relex_incremental(&mut incremental, old_text, new_text, byte_start, byte_start + 1, &Limits::default()).expect("relex");
        let full = TokenRope::from_text(new_text, &Limits::default()).expect("full rope");
        assert_eq!(kinds_texts(&incremental), kinds_texts(&full), "incremental and full lex must agree on kind+text sequence");
    }

    #[test]
    fn lineage_records_lookup_for_generated_tokens() {
        let mut rope = TokenRope::from_text("a", &Limits::default()).expect("rope");
        let generated = rope.tokenize_fresh("b", &Limits::default(), Lineage::Generated).expect("tokenize");
        assert_eq!(generated[0].lineage, Lineage::Generated);
    }
}
//#endregion 🧪️Tests
