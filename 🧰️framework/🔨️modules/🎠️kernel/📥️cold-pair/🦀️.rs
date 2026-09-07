//#region 🧊️ColdDocumentPairIngress
pub const COLD_PAIR_PAGE_MAXIMUM_BYTES: usize = 64 * 1024;
pub const COLD_PAIR_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;
pub const COLD_PAIR_MAXIMUM_PAGES: u32 = 64;
const COLD_PAIR_ID_MAXIMUM_BYTES: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairFrontier {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_sha256: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairHeader {
    pub lifetime: super::ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub descriptor_sha256: [u8; 32],
    pub baseline_frontier: ColdDocumentPairFrontier,
    pub pack_sha256: [u8; 32],
    pub spr_sha256: [u8; 32],
    pub aggregate_sha256: [u8; 32],
    pub pack_length: u64,
    pub spr_length: u64,
    pub page_count: u32,
}

impl ColdDocumentPairHeader {
    pub fn validate(&self) -> Result<(), &'static str> {
        let total = self.pack_length.checked_add(self.spr_length).ok_or("cold-pair.length-overflow")?;
        let expected_pages = total.checked_add(COLD_PAIR_PAGE_MAXIMUM_BYTES as u64 - 1).ok_or("cold-pair.page-count-overflow")? / COLD_PAIR_PAGE_MAXIMUM_BYTES as u64;
        if !self.lifetime.is_valid() || self.transfer_generation == 0 {
            return Err("cold-pair.authority");
        }
        if self.descriptor_sha256 == [0; 32] || self.pack_sha256 == [0; 32] || self.spr_sha256 == [0; 32] || self.aggregate_sha256 == [0; 32] {
            return Err("cold-pair.hash");
        }
        if self.baseline_frontier.document_id.is_empty()
            || self.baseline_frontier.document_id.as_bytes().len() > COLD_PAIR_ID_MAXIMUM_BYTES
            || self.baseline_frontier.head_edit_id.is_empty()
            || self.baseline_frontier.head_edit_id.as_bytes().len() > COLD_PAIR_ID_MAXIMUM_BYTES
            || self.baseline_frontier.last_commit_seq > self.baseline_frontier.head_edit_ordinal
        {
            return Err("cold-pair.frontier");
        }
        if self.pack_length == 0 || self.spr_length == 0 || total > COLD_PAIR_MAXIMUM_BYTES as u64 {
            return Err("cold-pair.length");
        }
        if self.page_count == 0 || self.page_count > COLD_PAIR_MAXIMUM_PAGES || u64::from(self.page_count) != expected_pages {
            return Err("cold-pair.page-count");
        }
        Ok(())
    }

    pub fn total_length(&self) -> Result<usize, &'static str> {
        self.validate()?;
        usize::try_from(self.pack_length.checked_add(self.spr_length).ok_or("cold-pair.length-overflow")?).map_err(|_| "cold-pair.length")
    }

    pub fn page_length(&self, page_index: u32) -> Result<usize, &'static str> {
        let total = self.total_length()?;
        if page_index >= self.page_count {
            return Err("cold-pair.page-index");
        }
        let offset = usize::try_from(page_index).map_err(|_| "cold-pair.page-index")?.checked_mul(COLD_PAIR_PAGE_MAXIMUM_BYTES).ok_or("cold-pair.page-index")?;
        Ok((total - offset).min(COLD_PAIR_PAGE_MAXIMUM_BYTES))
    }

    pub fn cursor(&self, page_index: u32) -> ColdDocumentPairCursor {
        ColdDocumentPairCursor { lifetime: self.lifetime, transfer_generation: self.transfer_generation, page_index, page_count: self.page_count }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairCursor {
    pub lifetime: super::ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub page_index: u32,
    pub page_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairPage {
    pub header: ColdDocumentPairHeader,
    pub page_index: u32,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairApplied {
    pub lifetime: super::ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub baseline_frontier: ColdDocumentPairFrontier,
    pub aggregate_sha256: [u8; 32],
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
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
//#endregion 🧊️ColdDocumentPairIngress
