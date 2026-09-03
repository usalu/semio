//! 🆔 Protocol identifier newtypes and hybrid logical timestamps.

//#region 🔖️Identifiers
// Moved from framework/core/rs/lib.rs 🔖️Identifiers (L5768-5838). Serde-transparent newtypes,
// shapes unchanged from their framework-core originals.

/// @emoji 🆔️ A stable identifier for one operation instance (an `Edit`'s forward/backward op).
/// 🌱️ Serde's derives are kept ALONGSIDE the hand-written `ToValue`/`FromValue` twin below
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02): `🎠️kernel`/`🛂️manifest`
/// (off-limits, owned by another agent) still fan out through this id via their own serde derives,
/// so blind-removing here breaks `cargo check -p semio-framework`. Drop once those consumers move.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MutationId(pub String);

/// @emoji 🧑️ A stable identifier for one collaborating actor.
/// 🌱️ Same reason as `MutationId` above — real `🎠️kernel` consumer.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActorId(pub String);

/// @emoji 📄️ A stable identifier for one document.
/// 🌱️ Same reason as `MutationId` above — real `🎠️kernel` consumer.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArtifactId(pub String);

/// @emoji 🔢️ A monotone document version counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactVersion(pub u64);

/// @emoji 🧬️ A stable identifier for one document/operation schema.
/// 🌱️ Same reason as `MutationId` above — real `🎠️kernel`/`🛂️manifest` consumers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

/// @emoji 🔢️ A schema's version number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaVersion(pub u32);

/// @emoji #⃣ A blake3 content hash over an operation/snapshot payload.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PayloadHash(pub [u8; 32]);

/// 🌉️ Hand-written, not derived — same DAG reason as `HybridLogicalTimestamp` above (this crate
/// sits below `os-kernel`, where the derive macro's generated code is rooted). `#[serde(transparent)]`
/// means the wire shape is the bare inner value, mirrored here directly.
impl crate::value::ToValue for MutationId {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for MutationId {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<String as crate::value::FromValue>::from_value(value)?))
    }
}

impl crate::value::ToValue for ActorId {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for ActorId {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<String as crate::value::FromValue>::from_value(value)?))
    }
}

impl crate::value::ToValue for SchemaId {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for SchemaId {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<String as crate::value::FromValue>::from_value(value)?))
    }
}

impl crate::value::ToValue for ArtifactId {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for ArtifactId {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<String as crate::value::FromValue>::from_value(value)?))
    }
}

impl crate::value::ToValue for ArtifactVersion {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for ArtifactVersion {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<u64 as crate::value::FromValue>::from_value(value)?))
    }
}

impl crate::value::ToValue for SchemaVersion {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for SchemaVersion {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<u32 as crate::value::FromValue>::from_value(value)?))
    }
}

/// 🌉️ `[u8; 32]` has no blanket `ToValue`/`FromValue` (unlike serde, which supports fixed arrays
/// natively) — encoded/decoded element-by-element as a `DslValue::Array`, matching serde's default
/// `[u8; N]` wire shape (a plain JSON array of numbers) byte for byte.
impl crate::value::ToValue for PayloadHash {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::Array(self.0.iter().map(crate::value::ToValue::to_value).collect())
    }
}
impl crate::value::FromValue for PayloadHash {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Array(items) = value else {
            return Err(crate::value::ValueError::new(format!("expected an array for PayloadHash, found {value:?}")));
        };
        if items.len() != 32 {
            return Err(crate::value::ValueError::new(format!("expected exactly 32 bytes for PayloadHash, found {}", items.len())));
        }
        let mut bytes = [0u8; 32];
        for (index, item) in items.into_iter().enumerate() {
            bytes[index] = <u8 as crate::value::FromValue>::from_value(item).map_err(|error| error.under(index))?;
        }
        Ok(Self(bytes))
    }
}
//#endregion 🔖️Identifiers

//#region 🔖️HybridLogicalTimestamp
// Moved from framework/core (L5840-5881). FIX vs the original: cmp_key gains `actor` as a total-
// order tiebreak (the original omitted it, so two ticks with equal physical_ms/logical from
// different actors compared Equal — a real ordering bug). Real `Ord`/`PartialOrd` now derive from
// cmp_key, not from field declaration order.

/// @emoji ⏰️ A hybrid logical clock tick: physical time plus a logical tiebreak plus the
/// originating actor (the third tiebreak — see the module note on the ordering fix above).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HybridLogicalTimestamp {
    pub actor: u64,
    pub physical_ms: u64,
    pub logical: u64,
}

impl HybridLogicalTimestamp {
    pub fn new(actor: u64, physical_ms: u64) -> Self {
        Self { actor, physical_ms, logical: 0 }
    }

    /// @emoji ⏩️ Advances to `physical_ms` if it's newer, else bumps the logical counter.
    pub fn tick(&mut self, physical_ms: u64) {
        if physical_ms > self.physical_ms {
            self.physical_ms = physical_ms;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    /// @emoji 🔀️ Merges in a remote tick: adopts the greater `(physical_ms, logical)`, then bumps.
    pub fn merge(&mut self, other: &Self) {
        if other.physical_ms > self.physical_ms {
            self.physical_ms = other.physical_ms;
            self.logical = other.logical;
        } else if other.physical_ms == self.physical_ms && other.logical > self.logical {
            self.logical = other.logical;
        }
        self.logical = self.logical.saturating_add(1);
    }

    pub fn cmp_key(&self) -> (u64, u64, u64) {
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

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` (`semio-framework-value-derive`)
/// roots its generated code at `::semio_framework_os_kernel::…`, which this crate sits BELOW in
/// the dependency DAG (os-kernel depends on replication, not the reverse) — the path could never
/// resolve here. `crate::value` is the raw first-party value module mounted directly by `#[path]`
/// (see the crate root), so it needs no such dependency edge. Field names match the pre-existing
/// serde wire shape (no `rename_all`, so `physical_ms` stays snake_case, not `physicalMs`).
impl crate::value::ToValue for HybridLogicalTimestamp {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object([
            ("actor".to_string(), crate::value::ToValue::to_value(&self.actor)),
            ("physical_ms".to_string(), crate::value::ToValue::to_value(&self.physical_ms)),
            ("logical".to_string(), crate::value::ToValue::to_value(&self.logical)),
        ])
    }
}
impl crate::value::FromValue for HybridLogicalTimestamp {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for HybridLogicalTimestamp, found {value:?}")));
        };
        let mut actor = None;
        let mut physical_ms = None;
        let mut logical = None;
        for (key, entry) in fields {
            match key.as_str() {
                "actor" => actor = Some(<u64 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "physical_ms" => physical_ms = Some(<u64 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("physical_ms"))?),
                "logical" => logical = Some(<u64 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("logical"))?),
                _ => {}
            }
        }
        Ok(HybridLogicalTimestamp {
            actor: actor.ok_or_else(|| crate::value::ValueError::new("HybridLogicalTimestamp missing actor"))?,
            physical_ms: physical_ms.ok_or_else(|| crate::value::ValueError::new("HybridLogicalTimestamp missing physical_ms"))?,
            logical: logical.ok_or_else(|| crate::value::ValueError::new("HybridLogicalTimestamp missing logical"))?,
        })
    }
}
//#endregion 🔖️HybridLogicalTimestamp
