//! 🎞️ Protocol CRDT merge strategies: real per-`MergeStrategyKind` conflict resolution for
//! concurrent `protocol_command::OperationDiff` pairs, replacing `store::merge_concurrent_diffs`
//! (`vcs/rs/lib.rs`'s `🔖️MergeStrategy` region), which today collapses all five declared
//! strategies to a blind `absorb()` regardless of what the operation actually declared. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §
//! `protocol_crdt`.
//!
//! # Design note (how five strategies come out of one generic signature)
//! `merge_concurrent_diffs` is generic over `D: protocol_command::OperationDiff<P>` and only ever
//! sees `D::apply`/`D::absorb` plus the two sides' `OperationMeta` — it never inspects `D`'s
//! concrete fields (ops stay schema-opaque in this family, same rule as `protocol_command`). Given
//! that surface, exactly two primitive combinators are expressible:
//! - **winner-take-all**: return one side's diff entirely, discarding the other (`Lww`,
//!   `ContentAddressedBlob`'s non-equal-hash fallback) — arbitrated by
//!   `HybridLogicalTimestamp::Ord` (`protocol_core::HybridLogicalTimestamp`), which is
//!   actor-tiebroken so a strict winner always exists.
//! - **chronological compose**: order the two sides by `OperationMeta.timestamp` and call
//!   `earlier.absorb(later)` — every real `absorb` impl in this codebase (see
//!   `store::DocumentVcsEnvelopeDiff::absorb` and friends) is "per-field overwrite iff the other
//!   side set that field", i.e. later-in-time already wins per-field when absorbed in order. This
//!   single combinator implements `OrderedSequence`/`TextSequence`'s compose behavior *and*
//!   `TombstonedGraphSet`'s "tombstone outranks add only if its timestamp is greater, else the add
//!   resurrects it" law for free: feed the earlier diff first, `absorb` the later one — whichever
//!   side is later always ends up as the surviving field value.
//!
//! `ContentAddressedBlob` additionally short-circuits on `OperationMeta.payload_hash` equality
//! before falling back to winner-take-all, per the contract.

//#region 🔖️Merge
/// @emoji 🧩️ Replaces `store::merge_concurrent_diffs` (`vcs/rs/lib.rs` L680), which collapsed every
/// `MergeStrategyKind` to plain `absorb()`. Dispatches to a real per-strategy combinator instead.
pub fn merge_concurrent_diffs<P, D: protocol_command::OperationDiff<P>>(
    strategy: protocol_core::MergeStrategyKind,
    existing: D,
    incoming: D,
    existing_meta: &protocol_command::OperationMeta,
    incoming_meta: &protocol_command::OperationMeta,
) -> D {
    match strategy {
        protocol_core::MergeStrategyKind::LwwRegister => lww_merge(existing, incoming, existing_meta, incoming_meta),
        protocol_core::MergeStrategyKind::OrderedSequence => ordered_sequence_merge(existing, incoming, existing_meta, incoming_meta),
        protocol_core::MergeStrategyKind::TextSequence => text_sequence_merge(existing, incoming, existing_meta, incoming_meta),
        protocol_core::MergeStrategyKind::TombstonedGraphSet => tombstoned_graph_set_merge(existing, incoming, existing_meta, incoming_meta),
        protocol_core::MergeStrategyKind::ContentAddressedBlob => content_addressed_blob_merge(existing, incoming, existing_meta, incoming_meta),
    }
}

/// @emoji 🕰️ Shared "later absorbs into earlier" combinator: orders `existing`/`incoming` by
/// `OperationMeta.timestamp` (ties — not expected post the `HybridLogicalTimestamp` actor-tiebreak
/// fix, see `protocol_core::HybridLogicalTimestamp::cmp_key` — break toward `existing`) and calls
/// `earlier.absorb(later)`, so per-field overwrites land in chronological order regardless of which
/// side the caller happened to pass as `existing` vs `incoming`.
fn chronological_compose<P, D: protocol_command::OperationDiff<P>>(existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D {
    let (mut earlier, later) = if incoming_meta.timestamp < existing_meta.timestamp { (incoming, existing) } else { (existing, incoming) };
    earlier.absorb(later);
    earlier
}
//#endregion 🔖️Merge

//#region 🔖️Lww
/// @emoji 🏆️ HLC-arbitrated register: the diff whose `OperationMeta.timestamp` is greater (via
/// `HybridLogicalTimestamp::Ord`, actor-tiebroken) wins outright — the loser is discarded whole,
/// not merged field-by-field, matching classical LWW-register semantics.
fn lww_merge<P, D: protocol_command::OperationDiff<P>>(existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D {
    if incoming_meta.timestamp > existing_meta.timestamp {
        incoming
    } else {
        existing
    }
}
//#endregion 🔖️Lww

//#region 🔖️OrderedSequence
/// @emoji 📍️ Dense order key for stable-anchor sequence positions (fractional-index style):
/// comparable via `Ord` so two replicas that independently generate an anchor between the same
/// neighbors converge on the same relative order. Byte-lexicographic `Ord` (derived) is the dense
/// order relation this crate assumes fractional-index generators already produce.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorId(pub Vec<u8>);

/// @emoji 🧵️ Stable-anchor sequence merge: concurrent inserts are ordered deterministically by
/// `(timestamp, actor)` on both replicas via `chronological_compose` — see the module-level design
/// note for why chronological absorb order is sufficient without inspecting anchor keys directly.
fn ordered_sequence_merge<P, D: protocol_command::OperationDiff<P>>(existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D {
    chronological_compose(existing, incoming, existing_meta, incoming_meta)
}
//#endregion 🔖️OrderedSequence

//#region 🔖️TextSequence
/// @emoji ✂️ Character/grapheme-range merge over two concurrent text diffs: non-overlapping ranges
/// compose (delegated to `D::absorb`, which a text technology implements as per-range overwrite);
/// overlapping ranges fall back to Lww on the overlapping span only, which is exactly what
/// `chronological_compose`'s later-overwrites-earlier absorb order already gives for whichever
/// sub-range both sides touched.
fn text_sequence_merge<P, D: protocol_command::OperationDiff<P>>(existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D {
    chronological_compose(existing, incoming, existing_meta, incoming_meta)
}
//#endregion 🔖️TextSequence

//#region 🔖️TombstonedGraphSet
/// @emoji 🪦️ Node/edge add-wins-by-default set: a concurrent remove leaves a tombstone that
/// outranks a concurrent add of the same id only if the tombstone's timestamp is greater, else the
/// add resurrects it. `chronological_compose` implements this exactly: absorbing the later op into
/// the earlier one means whichever op is chronologically later always ends up as the surviving
/// per-field state — a later remove overwrites an earlier add (tombstone outranks), and a later add
/// overwrites an earlier remove (resurrection).
fn tombstoned_graph_set_merge<P, D: protocol_command::OperationDiff<P>>(existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D {
    chronological_compose(existing, incoming, existing_meta, incoming_meta)
}
//#endregion 🔖️TombstonedGraphSet

//#region 🔖️ContentAddressedBlob
/// @emoji 🗄️ Two concurrent blob-extent writes: equal `OperationMeta.payload_hash` short-circuits
/// to "not a conflict at all" (either side's diff already represents the same content, so
/// `existing` is returned as-is); unequal hashes fall back to `lww_merge` by timestamp.
fn content_addressed_blob_merge<P, D: protocol_command::OperationDiff<P>>(existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D {
    let hashes_equal = matches!((&existing_meta.payload_hash, &incoming_meta.payload_hash), (Some(a), Some(b)) if a == b);
    if hashes_equal {
        existing
    } else {
        lww_merge(existing, incoming, existing_meta, incoming_meta)
    }
}
//#endregion 🔖️ContentAddressedBlob

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol_command::OperationDiff;

    //#region 🧸️Fixtures
    // `RegisterDiff`: single-value-register shaped diff (P = (i64, i64), two overwrite-able
    // fields) used to demonstrate the LwwRegister/ContentAddressedBlob "discard the loser whole"
    // behavior versus the compose strategies' "merge per field" behavior on the exact same inputs.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct RegisterDiff {
        field_a: Option<i64>,
        field_b: Option<i64>,
    }
    impl OperationDiff<(i64, i64)> for RegisterDiff {
        fn apply(&self, base: &(i64, i64)) -> (i64, i64) {
            (self.field_a.unwrap_or(base.0), self.field_b.unwrap_or(base.1))
        }
        fn absorb(&mut self, other: Self) {
            if other.field_a.is_some() {
                self.field_a = other.field_a;
            }
            if other.field_b.is_some() {
                self.field_b = other.field_b;
            }
        }
    }

    // `GraphDiff`: mutually-exclusive add/remove op pair (P = bool, "is the node present") used to
    // prove the TombstonedGraphSet law directly: whichever op is chronologically later always wins.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct GraphDiff {
        add: Option<bool>,
        remove: Option<bool>,
    }
    impl OperationDiff<bool> for GraphDiff {
        fn apply(&self, base: &bool) -> bool {
            if self.remove.is_some() {
                false
            } else if self.add.is_some() {
                true
            } else {
                *base
            }
        }
        fn absorb(&mut self, other: Self) {
            if other.add.is_some() {
                self.add = other.add;
                self.remove = None;
            }
            if other.remove.is_some() {
                self.remove = other.remove;
                self.add = None;
            }
        }
    }

    fn meta_at(actor: u64, physical_ms: u64) -> protocol_command::OperationMeta {
        protocol_command::OperationMeta {
            operation_id: None,
            dependencies: Vec::new(),
            base_version: 0,
            author_id: None,
            timestamp: protocol_core::HybridLogicalTimestamp::new(actor, physical_ms),
            undo_policy: protocol_core::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
        }
    }

    fn meta_with_hash(actor: u64, physical_ms: u64, hash: Option<[u8; 32]>) -> protocol_command::OperationMeta {
        protocol_command::OperationMeta { payload_hash: hash.map(protocol_core::PayloadHash), ..meta_at(actor, physical_ms) }
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Lww
    #[test]
    fn lww_register_discards_loser_whole_not_per_field() {
        let existing = RegisterDiff { field_a: Some(1), field_b: Some(100) };
        let incoming = RegisterDiff { field_a: Some(2), field_b: None };
        let existing_meta = meta_at(1, 10);
        let incoming_meta = meta_at(2, 20);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::LwwRegister, existing, incoming.clone(), &existing_meta, &incoming_meta);
        assert_eq!(merged, incoming, "later diff must win outright, including its unset field_b");
    }

    #[test]
    fn lww_register_follows_hlc_ord_strictly() {
        let existing = RegisterDiff { field_a: Some(1), field_b: None };
        let incoming = RegisterDiff { field_a: Some(2), field_b: None };
        // Same physical_ms/logical, different actor: HybridLogicalTimestamp::Ord's actor tiebreak
        // must decide, not an accidental Equal (the bug the moved HLC fixes).
        let existing_meta = meta_at(9, 100);
        let incoming_meta = meta_at(1, 100);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::LwwRegister, existing.clone(), incoming, &existing_meta, &incoming_meta);
        assert_eq!(merged, existing, "actor 9 must outrank actor 1 at equal physical_ms/logical");
    }

    #[test]
    fn lww_register_is_commutative_and_idempotent() {
        let a = RegisterDiff { field_a: Some(1), field_b: None };
        let b = RegisterDiff { field_a: Some(2), field_b: Some(9) };
        let ma = meta_at(1, 10);
        let mb = meta_at(2, 20);
        let forward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::LwwRegister, a.clone(), b.clone(), &ma, &mb);
        let backward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::LwwRegister, b, a.clone(), &mb, &ma);
        assert_eq!(forward, backward);
        let idempotent = merge_concurrent_diffs(protocol_core::MergeStrategyKind::LwwRegister, a.clone(), a.clone(), &ma, &ma);
        assert_eq!(idempotent, a);
    }
    //#endregion 🔖️Lww

    //#region 🔖️OrderedSequence
    #[test]
    fn ordered_sequence_composes_both_sides_per_field() {
        let existing = RegisterDiff { field_a: Some(1), field_b: Some(100) };
        let incoming = RegisterDiff { field_a: Some(2), field_b: None };
        let existing_meta = meta_at(1, 10);
        let incoming_meta = meta_at(2, 20);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::OrderedSequence, existing, incoming, &existing_meta, &incoming_meta);
        assert_eq!(merged, RegisterDiff { field_a: Some(2), field_b: Some(100) }, "later field_a wins but earlier field_b survives, unlike Lww");
    }

    #[test]
    fn ordered_sequence_is_commutative_and_idempotent() {
        let a = RegisterDiff { field_a: Some(1), field_b: None };
        let b = RegisterDiff { field_a: None, field_b: Some(9) };
        let ma = meta_at(1, 10);
        let mb = meta_at(2, 20);
        let forward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::OrderedSequence, a.clone(), b.clone(), &ma, &mb);
        let backward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::OrderedSequence, b, a.clone(), &mb, &ma);
        assert_eq!(forward, backward);
        let idempotent = merge_concurrent_diffs(protocol_core::MergeStrategyKind::OrderedSequence, a.clone(), a.clone(), &ma, &ma);
        assert_eq!(idempotent, a);
    }

    #[test]
    fn anchor_id_orders_lexicographically() {
        let low = AnchorId(vec![0x01]);
        let mid = AnchorId(vec![0x02]);
        let high = AnchorId(vec![0x02, 0x00]);
        assert!(low < mid);
        assert!(mid < high);
    }
    //#endregion 🔖️OrderedSequence

    //#region 🔖️TextSequence
    #[test]
    fn text_sequence_composes_non_overlapping_fields_and_lww_on_overlap() {
        // Non-overlapping: field_b only ever set by `existing` -> survives untouched.
        let existing = RegisterDiff { field_a: Some(1), field_b: Some(7) };
        let incoming = RegisterDiff { field_a: Some(2), field_b: None };
        let existing_meta = meta_at(1, 10);
        let incoming_meta = meta_at(2, 20);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TextSequence, existing, incoming, &existing_meta, &incoming_meta);
        assert_eq!(merged, RegisterDiff { field_a: Some(2), field_b: Some(7) });
    }

    #[test]
    fn text_sequence_is_commutative_and_idempotent() {
        let a = RegisterDiff { field_a: Some(3), field_b: None };
        let b = RegisterDiff { field_a: None, field_b: Some(4) };
        let ma = meta_at(1, 5);
        let mb = meta_at(2, 6);
        let forward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TextSequence, a.clone(), b.clone(), &ma, &mb);
        let backward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TextSequence, b, a.clone(), &mb, &ma);
        assert_eq!(forward, backward);
        let idempotent = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TextSequence, a.clone(), a.clone(), &ma, &ma);
        assert_eq!(idempotent, a);
    }
    //#endregion 🔖️TextSequence

    //#region 🔖️TombstonedGraphSet
    #[test]
    fn later_tombstone_outranks_earlier_add() {
        let add = GraphDiff { add: Some(true), remove: None };
        let remove = GraphDiff { add: None, remove: Some(true) };
        let add_meta = meta_at(1, 10);
        let remove_meta = meta_at(2, 20);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TombstonedGraphSet, add, remove, &add_meta, &remove_meta);
        assert!(!merged.apply(&true), "a tombstone later than the add must win");
    }

    #[test]
    fn later_add_resurrects_earlier_tombstone() {
        let remove = GraphDiff { add: None, remove: Some(true) };
        let add = GraphDiff { add: Some(true), remove: None };
        let remove_meta = meta_at(1, 10);
        let add_meta = meta_at(2, 20);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TombstonedGraphSet, remove, add, &remove_meta, &add_meta);
        assert!(merged.apply(&false), "an add later than the tombstone must resurrect the node");
    }

    #[test]
    fn tombstoned_graph_set_is_commutative_and_idempotent() {
        let add = GraphDiff { add: Some(true), remove: None };
        let remove = GraphDiff { add: None, remove: Some(true) };
        let ma = meta_at(1, 10);
        let mb = meta_at(2, 20);
        let forward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TombstonedGraphSet, add.clone(), remove.clone(), &ma, &mb);
        let backward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TombstonedGraphSet, remove, add.clone(), &mb, &ma);
        assert_eq!(forward, backward);
        let idempotent = merge_concurrent_diffs(protocol_core::MergeStrategyKind::TombstonedGraphSet, add.clone(), add.clone(), &ma, &ma);
        assert_eq!(idempotent, add);
    }
    //#endregion 🔖️TombstonedGraphSet

    //#region 🔖️ContentAddressedBlob
    #[test]
    fn equal_hashes_short_circuit_without_conflict() {
        let existing = RegisterDiff { field_a: Some(5), field_b: Some(6) };
        let incoming = existing.clone();
        let existing_meta = meta_with_hash(1, 10, Some([7u8; 32]));
        let incoming_meta = meta_with_hash(2, 20, Some([7u8; 32]));
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::ContentAddressedBlob, existing.clone(), incoming, &existing_meta, &incoming_meta);
        assert_eq!(merged, existing);
    }

    #[test]
    fn unequal_hashes_fall_back_to_lww() {
        let existing = RegisterDiff { field_a: Some(1), field_b: None };
        let incoming = RegisterDiff { field_a: Some(2), field_b: None };
        let existing_meta = meta_with_hash(1, 10, Some([1u8; 32]));
        let incoming_meta = meta_with_hash(2, 20, Some([2u8; 32]));
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::ContentAddressedBlob, existing, incoming.clone(), &existing_meta, &incoming_meta);
        assert_eq!(merged, incoming, "unequal hashes must fall back to timestamp-arbitrated Lww");
    }

    #[test]
    fn missing_hashes_are_never_treated_as_equal() {
        let existing = RegisterDiff { field_a: Some(1), field_b: None };
        let incoming = RegisterDiff { field_a: Some(2), field_b: None };
        let existing_meta = meta_at(1, 10);
        let incoming_meta = meta_at(2, 20);
        let merged = merge_concurrent_diffs(protocol_core::MergeStrategyKind::ContentAddressedBlob, existing, incoming.clone(), &existing_meta, &incoming_meta);
        assert_eq!(merged, incoming, "two None hashes must not short-circuit; falls back to Lww");
    }

    #[test]
    fn content_addressed_blob_is_commutative_and_idempotent() {
        let a = RegisterDiff { field_a: Some(1), field_b: None };
        let b = RegisterDiff { field_a: Some(2), field_b: None };
        let ma = meta_with_hash(1, 10, Some([1u8; 32]));
        let mb = meta_with_hash(2, 20, Some([2u8; 32]));
        let forward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::ContentAddressedBlob, a.clone(), b.clone(), &ma, &mb);
        let backward = merge_concurrent_diffs(protocol_core::MergeStrategyKind::ContentAddressedBlob, b, a.clone(), &mb, &ma);
        assert_eq!(forward, backward);
        let idempotent = merge_concurrent_diffs(protocol_core::MergeStrategyKind::ContentAddressedBlob, a.clone(), a.clone(), &ma, &ma);
        assert_eq!(idempotent, a);
    }
    //#endregion 🔖️ContentAddressedBlob
}
//#endregion 🧪️Tests
