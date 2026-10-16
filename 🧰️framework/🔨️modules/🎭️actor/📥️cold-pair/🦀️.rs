//#region 🧰️ColdPairIngressStatus
use crate::instance_lifetime::ActorInstanceLifetime;
use semio_framework_value_derive::{FromValue, ToValue};

pub const COLD_PAIR_MAXIMUM_PAGES: u32 = 64;
pub const COLD_PAIR_FAULT_MAXIMUM_BYTES: usize = 4 * 1024;
const COLD_PAIR_ID_MAXIMUM_BYTES: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ColdDocumentPairFrontier {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairCursor {
    pub lifetime: ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub page_index: u32,
    pub page_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairApplied {
    pub lifetime: ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub baseline_frontier: ColdDocumentPairFrontier,
    pub aggregate_sha256: [u8; 32],
}

#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ColdPairIngressStatus {
    #[default]
    Idle,
    PageAccepted(ColdDocumentPairCursor),
    Backpressure(ColdDocumentPairCursor),
    Loading(ColdDocumentPairCursor),
    Applied(ColdDocumentPairApplied),
    Fault {
        cursor: ColdDocumentPairCursor,
        fault: Vec<u8>,
    },
}

impl ColdDocumentPairFrontier {
    /// 🌱️ Exact empty history — the frontier a freshly created document's FIRST published
    /// checkpoint carries. It never invents an edit identifier or a chain hash, so demanding a
    /// nonempty head here refused every new document's own canonical pair.
    pub fn is_genesis(&self) -> bool {
        self.head_edit_ordinal == 0 && self.head_edit_id.is_empty() && self.last_commit_seq == 0 && self.chain_sha256 == [0; 32]
    }

    /// 🌿️ Normal edited history: positive counters and a real authenticated head.
    pub fn is_edited(&self) -> bool {
        self.head_edit_ordinal > 0 && !self.head_edit_id.is_empty() && self.head_edit_id.len() <= COLD_PAIR_ID_MAXIMUM_BYTES && self.last_commit_seq > 0 && self.chain_sha256 != [0; 32]
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.document_id.is_empty() || self.document_id.len() > COLD_PAIR_ID_MAXIMUM_BYTES || self.last_commit_seq > self.head_edit_ordinal || !(self.is_genesis() || self.is_edited()) {
            return Err("cold-pair.frontier");
        }
        Ok(())
    }

    async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_str(out, &self.document_id).await;
        semio_framework_actor_pack::write_u64(out, self.head_edit_ordinal).await;
        semio_framework_actor_pack::write_str(out, &self.head_edit_id).await;
        semio_framework_actor_pack::write_u64(out, self.last_commit_seq).await;
        semio_framework_actor_pack::write_hash32(out, &self.chain_sha256).await;
    }

    async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let value = Self {
            document_id: semio_framework_actor_pack::read_str(bytes, pos, "ColdDocumentPairFrontier::document_id").await?,
            head_edit_ordinal: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairFrontier::head_edit_ordinal").await?,
            head_edit_id: semio_framework_actor_pack::read_str(bytes, pos, "ColdDocumentPairFrontier::head_edit_id").await?,
            last_commit_seq: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairFrontier::last_commit_seq").await?,
            chain_sha256: semio_framework_actor_pack::read_hash32(bytes, pos, "ColdDocumentPairFrontier::chain_sha256").await?,
        };
        value.validate().map_err(semio_framework_actor_pack::PackError::InvalidColdPair)?;
        Ok(value)
    }
}

impl ColdDocumentPairCursor {
    pub fn validate(self) -> Result<(), &'static str> {
        if !self.lifetime.is_valid() || self.transfer_generation == 0 || self.page_count == 0 || self.page_count > COLD_PAIR_MAXIMUM_PAGES || self.page_index >= self.page_count {
            return Err("cold-pair.cursor");
        }
        Ok(())
    }

    async fn pack_encode(self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u64(out, self.lifetime.activation_generation).await;
        semio_framework_actor_pack::write_u32(out, self.lifetime.instance_id).await;
        semio_framework_actor_pack::write_u64(out, self.lifetime.guest_lifetime).await;
        semio_framework_actor_pack::write_u64(out, self.transfer_generation).await;
        semio_framework_actor_pack::write_u32(out, self.page_index).await;
        semio_framework_actor_pack::write_u32(out, self.page_count).await;
    }

    async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let value = Self {
            lifetime: ActorInstanceLifetime {
                activation_generation: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairCursor::activation_generation").await?,
                instance_id: semio_framework_actor_pack::read_u32(bytes, pos, "ColdDocumentPairCursor::instance_id").await?,
                guest_lifetime: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairCursor::guest_lifetime").await?,
            },
            transfer_generation: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairCursor::transfer_generation").await?,
            page_index: semio_framework_actor_pack::read_u32(bytes, pos, "ColdDocumentPairCursor::page_index").await?,
            page_count: semio_framework_actor_pack::read_u32(bytes, pos, "ColdDocumentPairCursor::page_count").await?,
        };
        value.validate().map_err(semio_framework_actor_pack::PackError::InvalidColdPair)?;
        Ok(value)
    }
}

impl ColdDocumentPairApplied {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.lifetime.is_valid() || self.transfer_generation == 0 || self.aggregate_sha256 == [0; 32] {
            return Err("cold-pair.applied");
        }
        self.baseline_frontier.validate()
    }

    async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u64(out, self.lifetime.activation_generation).await;
        semio_framework_actor_pack::write_u32(out, self.lifetime.instance_id).await;
        semio_framework_actor_pack::write_u64(out, self.lifetime.guest_lifetime).await;
        semio_framework_actor_pack::write_u64(out, self.transfer_generation).await;
        self.baseline_frontier.pack_encode(out).await;
        semio_framework_actor_pack::write_hash32(out, &self.aggregate_sha256).await;
    }

    async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let value = Self {
            lifetime: ActorInstanceLifetime {
                activation_generation: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairApplied::activation_generation").await?,
                instance_id: semio_framework_actor_pack::read_u32(bytes, pos, "ColdDocumentPairApplied::instance_id").await?,
                guest_lifetime: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairApplied::guest_lifetime").await?,
            },
            transfer_generation: semio_framework_actor_pack::read_u64(bytes, pos, "ColdDocumentPairApplied::transfer_generation").await?,
            baseline_frontier: ColdDocumentPairFrontier::pack_decode(bytes, pos).await?,
            aggregate_sha256: semio_framework_actor_pack::read_hash32(bytes, pos, "ColdDocumentPairApplied::aggregate_sha256").await?,
        };
        value.validate().map_err(semio_framework_actor_pack::PackError::InvalidColdPair)?;
        Ok(value)
    }
}

impl ColdPairIngressStatus {
    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            Self::Idle => Ok(()),
            Self::PageAccepted(cursor) | Self::Backpressure(cursor) | Self::Loading(cursor) => cursor.validate(),
            Self::Applied(receipt) => receipt.validate(),
            Self::Fault { cursor, fault } => {
                cursor.validate()?;
                if fault.is_empty() || fault.len() > COLD_PAIR_FAULT_MAXIMUM_BYTES {
                    return Err("cold-pair.fault");
                }
                Ok(())
            }
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor_pack::PackError> {
        self.validate().map_err(semio_framework_actor_pack::PackError::InvalidColdPair)?;
        match self {
            Self::Idle => semio_framework_actor_pack::write_u8(out, 0).await,
            Self::PageAccepted(cursor) => {
                semio_framework_actor_pack::write_u8(out, 1).await;
                cursor.pack_encode(out).await;
            }
            Self::Backpressure(cursor) => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                cursor.pack_encode(out).await;
            }
            Self::Loading(cursor) => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                cursor.pack_encode(out).await;
            }
            Self::Applied(receipt) => {
                semio_framework_actor_pack::write_u8(out, 4).await;
                receipt.pack_encode(out).await;
            }
            Self::Fault { cursor, fault } => {
                semio_framework_actor_pack::write_u8(out, 5).await;
                cursor.pack_encode(out).await;
                semio_framework_actor_pack::write_bytes(out, fault).await;
            }
        }
        Ok(())
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "ColdPairIngressStatus").await?;
        let value = match tag {
            0 => Self::Idle,
            1 => Self::PageAccepted(ColdDocumentPairCursor::pack_decode(bytes, pos).await?),
            2 => Self::Backpressure(ColdDocumentPairCursor::pack_decode(bytes, pos).await?),
            3 => Self::Loading(ColdDocumentPairCursor::pack_decode(bytes, pos).await?),
            4 => Self::Applied(ColdDocumentPairApplied::pack_decode(bytes, pos).await?),
            5 => Self::Fault { cursor: ColdDocumentPairCursor::pack_decode(bytes, pos).await?, fault: semio_framework_actor_pack::read_bytes(bytes, pos, "ColdPairIngressStatus::fault").await? },
            other => return Err(semio_framework_actor_pack::PackError::InvalidTag { what: "ColdPairIngressStatus", tag: other, offset: *pos }),
        };
        value.validate().map_err(semio_framework_actor_pack::PackError::InvalidColdPair)?;
        Ok(value)
    }
}
//#endregion 🧰️ColdPairIngressStatus
