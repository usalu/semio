//! 🛍️ The host-side merge of an externally supplied catalogue payload with the static palette sections.

use super::*;

pub fn merge_catalogue_sections(host_json: &str) -> Result<Vec<CatalogueSection>, FlowCoreError> {
    let mut sections: Vec<CatalogueSection> = if host_json.trim().is_empty() { vec![] } else { semio_framework_pack_json::from_json_str(host_json, semio_framework_pack_json::JsonMemberPolicy::Reject)? };
    sections.extend(static_catalogue_sections());
    Ok(sections)
}
