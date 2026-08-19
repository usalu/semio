//! 🆔 Protocol identifier newtypes and hybrid logical timestamps.

//#region 🔖️Identifiers
// Moved from framework/core/rs/lib.rs 🔖️Identifiers (L5768-5838). Serde-transparent newtypes,
// shapes unchanged from their framework-core originals.

/// @emoji 🆔️ A stable identifier for one operation instance (an `Edit`'s forward/backward op).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MutationId(pub String);

/// @emoji 🧑️ A stable identifier for one collaborating actor.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActorId(pub String);

/// @emoji 📄️ A stable identifier for one document.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArtifactId(pub String);

/// @emoji 🔢️ A monotone document version counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArtifactVersion(pub u64);

/// @emoji 🧬️ A stable identifier for one document/operation schema.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

/// @emoji 🔢️ A schema's version number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub u32);

/// @emoji #⃣ A blake3 content hash over an operation/snapshot payload.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PayloadHash(pub [u8; 32]);
//#endregion 🔖️Identifiers

//#region 🔖️HybridLogicalTimestamp
// Moved from framework/core (L5840-5881). FIX vs the original: cmp_key gains `actor` as a total-
// order tiebreak (the original omitted it, so two ticks with equal physical_ms/logical from
// different actors compared Equal — a real ordering bug). Real `Ord`/`PartialOrd` now derive from
// cmp_key, not from field declaration order.

/// @emoji ⏰️ A hybrid logical clock tick: physical time plus a logical tiebreak plus the
/// originating actor (the third tiebreak — see the module note on the ordering fix above).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HybridLogicalTimestamp {
    pub actor: u64,
    pub physical_ms: u64,
    pub logical: u64,
}

impl HybridLogicalTimestamp {
    pub async fn new(actor: u64, physical_ms: u64) -> Self {
        Self { actor, physical_ms, logical: 0 }
    }

    /// @emoji ⏩️ Advances to `physical_ms` if it's newer, else bumps the logical counter.
    pub async fn tick(&mut self, physical_ms: u64) {
        if physical_ms > self.physical_ms {
            self.physical_ms = physical_ms;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    /// @emoji 🔀️ Merges in a remote tick: adopts the greater `(physical_ms, logical)`, then bumps.
    pub async fn merge(&mut self, other: &Self) {
        if other.physical_ms > self.physical_ms {
            self.physical_ms = other.physical_ms;
            self.logical = other.logical;
        } else if other.physical_ms == self.physical_ms && other.logical > self.logical {
            self.logical = other.logical;
        }
        self.logical = self.logical.saturating_add(1);
    }

    pub async fn cmp_key(&self) -> (u64, u64, u64) {
        (self.physical_ms, self.logical, self.actor)
    }
}

impl Ord for HybridLogicalTimestamp {
    // 🚫️async: E1 impl of externally-declared `Ord` trait
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.physical_ms, self.logical, self.actor).cmp(&(other.physical_ms, other.logical, other.actor))
    }
}

impl PartialOrd for HybridLogicalTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//#endregion 🔖️HybridLogicalTimestamp

