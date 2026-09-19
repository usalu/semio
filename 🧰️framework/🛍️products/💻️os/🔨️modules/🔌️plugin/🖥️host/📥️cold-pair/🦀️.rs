use super::*;

#[cfg(test)]
#[path = "🧪️tests/📥️cold-pair/🦀️.rs"]
mod tests;
use semio_framework::kernel::{ColdDocumentPairApplied, ColdDocumentPairCursor, ColdDocumentPairFrontier, ColdDocumentPairPage, ColdPairIngressStatus};

fn invalid(reason: &str) -> TurnFault {
    TurnFault::Trapped(reason.to_owned())
}

fn hash(bytes: Vec<u8>) -> Result<[u8; 32], TurnFault> {
    bytes.try_into().map_err(|_| invalid("cold-pair hash must contain exactly 32 bytes"))
}

fn frontier(value: wit_reactor::ColdDocumentPairFrontier) -> Result<ColdDocumentPairFrontier, TurnFault> {
    if value.document_id.is_empty() || value.document_id.len() > 512 || value.head_edit_id.is_empty() || value.head_edit_id.len() > 512 || value.last_commit_seq > value.head_edit_ordinal {
        return Err(invalid("cold-pair frontier is invalid"));
    }
    let chain_sha256 = hash(value.chain_sha256)?;
    if chain_sha256 == [0; 32] {
        return Err(invalid("cold-pair frontier has no verified chain"));
    }
    Ok(ColdDocumentPairFrontier { document_id: value.document_id, head_edit_ordinal: value.head_edit_ordinal, head_edit_id: value.head_edit_id, last_commit_seq: value.last_commit_seq, chain_sha256 })
}

fn cursor(value: wit_reactor::ColdDocumentPairCursor) -> Result<ColdDocumentPairCursor, TurnFault> {
    let lifetime = wit_lifetime_to_kernel(value.lifetime);
    if !lifetime.is_valid() || value.transfer_generation == 0 || value.page_count == 0 || value.page_count > semio_framework::kernel::COLD_PAIR_MAXIMUM_PAGES || value.page_index >= value.page_count {
        return Err(invalid("cold-pair cursor is invalid"));
    }
    Ok(ColdDocumentPairCursor { lifetime, transfer_generation: value.transfer_generation, page_index: value.page_index, page_count: value.page_count })
}

pub(super) fn validate_cold_page(page: &ColdDocumentPairPage) -> Result<(), TurnFault> {
    let expected = page.header.page_length(page.page_index).map_err(invalid)?;
    if page.bytes.len() != expected || page.header.baseline_frontier.chain_sha256 == [0; 32] {
        return Err(invalid("cold-pair page length or verified frontier is invalid"));
    }
    Ok(())
}

pub(super) fn kernel_cold_page_to_wit(page: &ColdDocumentPairPage) -> Result<wit_reactor::ColdDocumentPairPage, TurnFault> {
    validate_cold_page(page)?;
    let header = &page.header;
    let baseline = &header.baseline_frontier;
    Ok(wit_reactor::ColdDocumentPairPage {
        header: wit_reactor::ColdDocumentPairHeader {
            lifetime: kernel_lifetime_to_wit(header.lifetime),
            transfer_generation: header.transfer_generation,
            descriptor_sha256: header.descriptor_sha256.to_vec(),
            baseline_frontier: wit_reactor::ColdDocumentPairFrontier {
                document_id: baseline.document_id.clone(),
                head_edit_ordinal: baseline.head_edit_ordinal,
                head_edit_id: baseline.head_edit_id.clone(),
                last_commit_seq: baseline.last_commit_seq,
                chain_sha256: baseline.chain_sha256.to_vec(),
            },
            pack_sha256: header.pack_sha256.to_vec(),
            spr_sha256: header.spr_sha256.to_vec(),
            aggregate_sha256: header.aggregate_sha256.to_vec(),
            pack_length: header.pack_length,
            spr_length: header.spr_length,
            page_count: header.page_count,
        },
        page_index: page.page_index,
        bytes: page.bytes.clone(),
    })
}

pub(super) fn wit_cold_ingress_to_kernel(value: wit_reactor::ColdPairIngressStatus) -> Result<ColdPairIngressStatus, TurnFault> {
    use wit_reactor::ColdPairIngressStatus as W;
    let status = match value {
        W::Idle => ColdPairIngressStatus::Idle,
        W::PageAccepted(value) => ColdPairIngressStatus::PageAccepted(cursor(value)?),
        W::Backpressure(value) => ColdPairIngressStatus::Backpressure(cursor(value)?),
        W::Loading(value) => ColdPairIngressStatus::Loading(cursor(value)?),
        W::Fault(value) => {
            if value.fault.len() > 4096 {
                return Err(invalid("cold-pair fault exceeds its fixed bound"));
            }
            ColdPairIngressStatus::Fault { cursor: cursor(value.cursor)?, fault: value.fault }
        }
        W::Applied(value) => {
            let lifetime = wit_lifetime_to_kernel(value.lifetime);
            let aggregate_sha256 = hash(value.aggregate_sha256)?;
            if !lifetime.is_valid() || value.transfer_generation == 0 || aggregate_sha256 == [0; 32] {
                return Err(invalid("cold-pair Applied authority is invalid"));
            }
            ColdPairIngressStatus::Applied(ColdDocumentPairApplied { lifetime, transfer_generation: value.transfer_generation, baseline_frontier: frontier(value.baseline_frontier)?, aggregate_sha256 })
        }
    };
    status.validate().map_err(invalid)?;
    Ok(status)
}
