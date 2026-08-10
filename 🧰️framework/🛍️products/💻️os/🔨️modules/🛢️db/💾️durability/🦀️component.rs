//! 💾 Durability class, frontier sync, and epoch fencing.

use crate::db_ids::{ActorId, DbError, ArtifactId};
use pack::ContentHash;

//#region 🔖️Durability
/// @emoji 💾️ How durably a command's effects are guaranteed to survive a crash before its
/// `CommandReceipt` is returned. Ordered strongest-last: `Memory < Os < Fsync < Quorum(n)`
/// (`Quorum` variants order among themselves by acknowledging-replica count `n`) — group-commit
/// batching in `db_wal` computes `max()` over the durability classes requested by the commands in
/// one batch to decide how hard to push the flush.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DurabilityClass {
    /// @emoji 🧠️ Visible to readers once applied in-process; no persistence guarantee at all.
    #[default]
    Memory,
    /// @emoji 🗂️ Written to the WAL and handed to the OS (`write(2)`), not yet `fsync`ed.
    Os,
    /// @emoji 🔒️ `fsync`ed to local storage before the receipt is returned.
    Fsync,
    /// @emoji 🤝️ Acknowledged `fsync`ed by at least `n` cluster replicas (`db_cluster`).
    Quorum(u8),
}

impl DurabilityClass {
    /// @emoji 🥇️ A total order key: `(tier, quorum_n)`, so `Ord`/`PartialOrd` can be derived from
    /// arithmetic comparison rather than a hand-written match ladder.
    fn rank(&self) -> (u8, u8) {
        match self {
            DurabilityClass::Memory => (0, 0),
            DurabilityClass::Os => (1, 0),
            DurabilityClass::Fsync => (2, 0),
            DurabilityClass::Quorum(n) => (3, *n),
        }
    }
}

impl PartialOrd for DurabilityClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DurabilityClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}
//#endregion 🔖️Durability

//#region 🔖️Frontier
/// @emoji 🧭️ A document's sync-relevant position: how far its WAL/commit sequence has advanced,
/// its commit chain's current tip hash, and the fencing epoch it was produced under. Mirrors the
/// `db` facade's frozen `Frontier{document, head_seq, commit_seq, chain_hash, epoch}` shape
/// exactly (see module doc for the `ArtifactId` conversion rationale).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Frontier {
    pub document: ArtifactId,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

impl Frontier {
    /// @emoji 🌱️ The frontier of a freshly created, empty document.
    pub fn genesis(document: ArtifactId) -> Frontier {
        Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 }
    }

    /// @emoji 🔑️ Reinterprets `chain_hash` as a `pack::ContentHash` — the family hashes
    /// pack-style throughout; this is the bridge for callers that want the typed/`Display`able
    /// form instead of a raw array.
    pub fn chain_hash_typed(&self) -> pack::ContentHash {
        pack::ContentHash(self.chain_hash)
    }

    /// @emoji 🏔️ True iff `self` has observed everything `other` has (same document, `>=` on
    /// every sequence/epoch field) — the law `Consistency::AtLeast(frontier)` query resolution
    /// checks against a document's current frontier.
    pub fn dominates(&self, other: &Frontier) -> Result<bool, DbError> {
        if self.document != other.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", self.document, other.document)));
        }
        Ok(self.head_seq >= other.head_seq && self.commit_seq >= other.commit_seq && self.epoch >= other.epoch)
    }
}

/// @emoji 📐️ The gap between two frontiers of the SAME document — `db_sync`'s unit of "how much
/// missing-command transfer does a replica need".
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FrontierDelta {
    pub document: ArtifactId,
    pub from_head_seq: u64,
    pub to_head_seq: u64,
    pub commands: u64,
}

impl FrontierDelta {
    /// @emoji ➖️ Computes the delta from `from` to `to`. Errors on a document mismatch or on `to`
    /// being behind `from` (a delta only ever moves a replica forward).
    pub fn between(from: &Frontier, to: &Frontier) -> Result<FrontierDelta, DbError> {
        if from.document != to.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", from.document, to.document)));
        }
        if to.head_seq < from.head_seq {
            return Err(DbError::InvalidArgument(format!("to frontier (head_seq {}) is behind from frontier (head_seq {})", to.head_seq, from.head_seq)));
        }
        Ok(FrontierDelta { document: from.document.clone(), from_head_seq: from.head_seq, to_head_seq: to.head_seq, commands: to.head_seq - from.head_seq })
    }

    /// @emoji 🕳️ True iff the two frontiers were already equal (nothing to transfer).
    pub fn is_empty(&self) -> bool {
        self.commands == 0
    }
}

/// @emoji 🎫️ An opaque, serialized `Frontier` a replica hands back on reconnect so `db_sync` can
/// resume exactly where it left off, instead of re-negotiating from scratch. Deliberately
/// text-encoded (not a bincode/serde blob) so it stays diffable in logs and stable across a
/// `Frontier` field-order change — the wire format is this crate's own choice (the contract
/// leaves the exact encoding unspecified), versioned via a leading `v1` tag.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResumeToken(String);

impl ResumeToken {
    /// @emoji ✍️ Encodes `frontier` as `v1|<document>|<head_seq>|<commit_seq>|<epoch>|<hex chain_hash>`.
    /// Rejects a document id containing `|` (would make the encoding ambiguous to decode).
    pub fn encode(frontier: &Frontier) -> Result<ResumeToken, DbError> {
        if frontier.document.0.contains('|') {
            return Err(DbError::InvalidArgument("document id must not contain '|' to be resume-token safe".to_string()));
        }
        let mut hex = String::with_capacity(64);
        for byte in frontier.chain_hash {
            use std::fmt::Write;
            let _ = write!(hex, "{byte:02x}");
        }
        Ok(ResumeToken(format!("v1|{}|{}|{}|{}|{}", frontier.document, frontier.head_seq, frontier.commit_seq, frontier.epoch, hex)))
    }

    /// @emoji 📖️ Inverse of `encode`. Rejects an unknown version tag, a wrong field count, or a
    /// malformed hex/decimal field, always returning `DbError` rather than panicking.
    pub fn decode(&self) -> Result<Frontier, DbError> {
        let mut parts = self.0.split('|');
        let malformed = || DbError::Corrupt("malformed resume token".to_string());

        let version = parts.next().ok_or_else(malformed)?;
        if version != "v1" {
            return Err(DbError::Corrupt(format!("unsupported resume token version {version:?}")));
        }
        let document = parts.next().ok_or_else(malformed)?.to_string();
        let head_seq = parts.next().ok_or_else(malformed)?.parse::<u64>().map_err(|_| malformed())?;
        let commit_seq = parts.next().ok_or_else(malformed)?.parse::<u64>().map_err(|_| malformed())?;
        let epoch = parts.next().ok_or_else(malformed)?.parse::<u64>().map_err(|_| malformed())?;
        let hex = parts.next().ok_or_else(malformed)?;
        if parts.next().is_some() {
            return Err(malformed());
        }
        if hex.len() != 64 {
            return Err(malformed());
        }
        let mut chain_hash = [0u8; 32];
        for (i, slot) in chain_hash.iter_mut().enumerate() {
            *slot = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).map_err(|_| malformed())?;
        }
        Ok(Frontier { document: ArtifactId(document), head_seq, commit_seq, chain_hash, epoch })
    }

    /// @emoji 🔍️ Borrows the token's wire form, e.g. for embedding in a `protocol_wire::Hello`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
//#endregion 🔖️Frontier

//#region 🔖️Fencing
/// @emoji 🚧️ The split-brain gate: a monotonic epoch a `CatalogStorage::cas_root` write must
/// present to succeed. A writer that lost leadership (its epoch superseded by a newer one) gets
/// `DbError::Fenced` on its next write instead of silently corrupting the catalog root — the
/// primitive `db_cluster`'s ownership-lease failover builds on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EpochFence {
    pub epoch: u64,
}

impl EpochFence {
    /// @emoji 🌱️ The fence a document's catalog entry starts at before any leadership handoff.
    pub const INITIAL: EpochFence = EpochFence { epoch: 0 };

    /// @emoji ⏭️ The fence a new leader claims after winning an ownership lease.
    pub fn next(self) -> EpochFence {
        EpochFence { epoch: self.epoch + 1 }
    }

    /// @emoji ✅️ Compare-and-swap gate: succeeds only if `self` (the epoch presented by the
    /// writer) exactly matches `current` (the epoch stamped on the stored root). Any mismatch —
    /// stale writer OR a writer somehow ahead of the stored root — is fenced, since the latter
    /// indicates the caller read a root written concurrently under a different epoch.
    pub fn check(self, current: EpochFence) -> Result<(), DbError> {
        if self.epoch == current.epoch {
            Ok(())
        } else {
            Err(DbError::Fenced { expected: current.epoch, actual: self.epoch })
        }
    }
}
//#endregion 🔖️Fencing

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Durability
    #[test]
    fn durability_class_orders_memory_below_os_below_fsync_below_quorum() {
        assert!(DurabilityClass::Memory < DurabilityClass::Os);
        assert!(DurabilityClass::Os < DurabilityClass::Fsync);
        assert!(DurabilityClass::Fsync < DurabilityClass::Quorum(1));
        assert!(DurabilityClass::Quorum(1) < DurabilityClass::Quorum(3));
        assert_eq!(DurabilityClass::default(), DurabilityClass::Memory);

        let mut classes = vec![DurabilityClass::Quorum(2), DurabilityClass::Memory, DurabilityClass::Fsync, DurabilityClass::Os];
        classes.sort();
        assert_eq!(classes, vec![DurabilityClass::Memory, DurabilityClass::Os, DurabilityClass::Fsync, DurabilityClass::Quorum(2)]);
    }

    #[test]
    fn durability_class_batch_max_picks_strongest_requested() {
        let requested = [DurabilityClass::Os, DurabilityClass::Memory, DurabilityClass::Fsync];
        let strongest = requested.into_iter().max().unwrap();
        assert_eq!(strongest, DurabilityClass::Fsync);
    }
    //#endregion 🔖️Durability

    //#region 🔖️Frontier
    fn sample_frontier(document: &str, head_seq: u64, commit_seq: u64, epoch: u64) -> Frontier {
        let mut chain_hash = [0u8; 32];
        chain_hash[0] = head_seq as u8;
        Frontier { document: document.into(), head_seq, commit_seq, chain_hash, epoch }
    }

    #[test]
    fn frontier_genesis_is_all_zero() {
        let frontier = Frontier::genesis("doc-1".into());
        assert_eq!(frontier.head_seq, 0);
        assert_eq!(frontier.commit_seq, 0);
        assert_eq!(frontier.epoch, 0);
        assert_eq!(frontier.chain_hash, [0u8; 32]);
    }

    #[test]
    fn frontier_chain_hash_typed_bridges_to_pack_core_content_hash() {
        let frontier = sample_frontier("doc-1", 5, 5, 0);
        let typed = frontier.chain_hash_typed();
        assert_eq!(typed.0, frontier.chain_hash);
    }

    #[test]
    fn frontier_dominates_requires_same_document_and_all_fields_at_least() {
        let earlier = sample_frontier("doc-1", 3, 3, 0);
        let later = sample_frontier("doc-1", 5, 5, 0);
        assert!(later.dominates(&earlier).unwrap());
        assert!(!earlier.dominates(&later).unwrap());
        assert!(later.dominates(&later).unwrap());

        let other_document = sample_frontier("doc-2", 5, 5, 0);
        assert!(matches!(later.dominates(&other_document), Err(DbError::InvalidArgument(_))));
    }

    #[test]
    fn frontier_delta_between_computes_gap_and_rejects_backwards_or_mismatched() {
        let from = sample_frontier("doc-1", 3, 3, 0);
        let to = sample_frontier("doc-1", 8, 8, 0);
        let delta = FrontierDelta::between(&from, &to).unwrap();
        assert_eq!(delta.from_head_seq, 3);
        assert_eq!(delta.to_head_seq, 8);
        assert_eq!(delta.commands, 5);
        assert!(!delta.is_empty());

        let same = FrontierDelta::between(&from, &from).unwrap();
        assert!(same.is_empty());

        assert!(FrontierDelta::between(&to, &from).is_err());

        let other_document = sample_frontier("doc-2", 8, 8, 0);
        assert!(FrontierDelta::between(&from, &other_document).is_err());
    }

    #[test]
    fn resume_token_round_trips_through_encode_decode() {
        let frontier = sample_frontier("doc-1", 42, 41, 7);
        let token = ResumeToken::encode(&frontier).unwrap();
        let decoded = token.decode().unwrap();
        assert_eq!(decoded, frontier);
        assert!(token.as_str().starts_with("v1|doc-1|42|41|7|"));
    }

    #[test]
    fn resume_token_encode_rejects_pipe_in_document_id() {
        let frontier = sample_frontier("doc|1", 1, 1, 0);
        assert!(ResumeToken::encode(&frontier).is_err());
    }

    #[test]
    fn resume_token_decode_rejects_malformed_input_without_panicking() {
        assert!(matches!(ResumeToken("garbage".to_string()).decode(), Err(DbError::Corrupt(_))));
        assert!(matches!(ResumeToken("v2|doc|1|1|1|00".to_string()).decode(), Err(DbError::Corrupt(_))));
        assert!(matches!(ResumeToken("v1|doc|notanumber|1|1|00".to_string()).decode(), Err(DbError::Corrupt(_))));
        let short_hash = format!("v1|doc-1|1|1|1|{}", "ab".repeat(10));
        assert!(matches!(ResumeToken(short_hash).decode(), Err(DbError::Corrupt(_))));
    }
    //#endregion 🔖️Frontier

    //#region 🔖️Fencing
    #[test]
    fn epoch_fence_check_accepts_matching_epoch_and_rejects_stale_or_ahead() {
        let current = EpochFence::INITIAL.next().next();
        assert!(current.check(current).is_ok());

        let stale = EpochFence::INITIAL;
        assert_eq!(stale.check(current), Err(DbError::Fenced { expected: current.epoch, actual: stale.epoch }));

        let ahead = current.next();
        assert_eq!(ahead.check(current), Err(DbError::Fenced { expected: current.epoch, actual: ahead.epoch }));
    }

    #[test]
    fn epoch_fence_next_is_monotonic() {
        let mut fence = EpochFence::INITIAL;
        for expected in 1..=5u64 {
            fence = fence.next();
            assert_eq!(fence.epoch, expected);
        }
    }
    //#endregion 🔖️Fencing
}
