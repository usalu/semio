//! 💾 Durability class, frontier sync, and epoch fencing.

use crate::db_ids::{ArtifactId, DbError};

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
    // 🚫️async: E1 pure accessor consumed by `impl Ord` (itself E1) — see R9
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
    // 🚫️async: E1 pure constructor consumed synchronously at every call site but one — see R9
    pub fn genesis(document: ArtifactId) -> Frontier {
        Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 }
    }

    /// @emoji 🔑️ Reinterprets `chain_hash` as a `pack::ContentHash` — the family hashes
    /// pack-style throughout; this is the bridge for callers that want the typed/`Display`able
    /// form instead of a raw array.
    pub async fn chain_hash_typed(&self) -> pack::ContentHash {
        pack::ContentHash(self.chain_hash)
    }

    /// @emoji 🏔️ True iff `self` has observed everything `other` has (same document, `>=` on
    /// every sequence/epoch field) — the law `Consistency::AtLeast(frontier)` query resolution
    /// checks against a document's current frontier.
    // 🚫️async: E1 pure accessor consumed by a sync Iterator::filter — see R9
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
    pub async fn between(from: &Frontier, to: &Frontier) -> Result<FrontierDelta, DbError> {
        if from.document != to.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", from.document, to.document)));
        }
        if to.head_seq < from.head_seq {
            return Err(DbError::InvalidArgument(format!("to frontier (head_seq {}) is behind from frontier (head_seq {})", to.head_seq, from.head_seq)));
        }
        Ok(FrontierDelta { document: from.document.clone(), from_head_seq: from.head_seq, to_head_seq: to.head_seq, commands: to.head_seq - from.head_seq })
    }

    /// @emoji 🕳️ True iff the two frontiers were already equal (nothing to transfer).
    pub async fn is_empty(&self) -> bool {
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

    /// @emoji 🔍️ Borrows the token's wire form for embedding in `protocol_wire::SocketHelloV1`.
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
    // 🚫️async: E1 pure accessor consumed synchronously throughout `db_storage` — see R9
    pub fn next(self) -> EpochFence {
        EpochFence { epoch: self.epoch + 1 }
    }

    /// @emoji ✅️ Compare-and-swap gate: succeeds only if `self` (the epoch presented by the
    /// writer) exactly matches `current` (the epoch stamped on the stored root). Any mismatch —
    /// stale writer OR a writer somehow ahead of the stored root — is fenced, since the latter
    /// indicates the caller read a root written concurrently under a different epoch.
    // 🚫️async: E1 pure accessor consumed synchronously throughout `db_storage` — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
