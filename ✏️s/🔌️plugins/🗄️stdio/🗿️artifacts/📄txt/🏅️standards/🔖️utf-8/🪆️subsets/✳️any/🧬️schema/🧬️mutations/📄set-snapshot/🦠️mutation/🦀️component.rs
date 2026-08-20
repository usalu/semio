use crate::artifacts::txt::schema::mutations::{apply_txt_mutation, TxtMutation};
use crate::artifacts::txt::TxtSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut TxtSnapshot, mutation: &TxtMutation) {
    apply_txt_mutation(projection, mutation);
}
