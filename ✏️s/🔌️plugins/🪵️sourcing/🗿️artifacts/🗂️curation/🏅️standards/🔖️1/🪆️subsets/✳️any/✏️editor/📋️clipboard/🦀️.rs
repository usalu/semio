//! 📋️ Sourcing curation clipboard: the copied kinds travel as `(objectId, count)` pairs and paste back as curated counts, so a copy followed by a paste doubles the copied kinds and a cut followed by a paste moves them. The curation is a list, not a space, so a paste anchor and position carry no meaning here and are accepted and ignored.

use crate::standards::v1::subsets::any::schema::mutations::{change_curated_item_count, create_curated_item, delete_curated_item, SourcingMutation};
use crate::{CuratedItem, CurationSnapshot};
use semio_plugin::kernel::{ClipboardError, ClipboardFragment, PastePlacement};
use semio_plugin::{MediaClass, MediaForm, MediaType};
use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::domain::{curated_count, curation_decision_for_set, CurationDecision};
use std::collections::BTreeMap;

//#region 🔖️Packet
pub const SOURCING_CLIPBOARD_SCHEMA: &str = "sourcing.clipboard.v1";
pub const SOURCING_CLIPBOARD_MAX_ITEMS: usize = 4096;
pub const SOURCING_CLIPBOARD_MAX_ID_BYTES: usize = 256;

/// 🧺️ The clipboard packet: one pair per copied kind.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourcingClipboard {
    pub schema: String,
    pub items: Vec<CuratedItem>,
}

pub fn media_type() -> MediaType {
    MediaType { class: MediaClass::Kit, form: MediaForm::Kit }
}

fn invalid(message: impl Into<String>) -> ClipboardError {
    ClipboardError::ParseFailed(message.into())
}
//#endregion 🔖️Packet

//#region 🔖️Copy
/// 📋️ The selected kinds in stock order, each with its curated count — a selected kind that is not curated yet stands for one unit.
pub fn copy(document: &CurationSnapshot, selected_ids: &[String]) -> Result<ClipboardFragment, ClipboardError> {
    let items: Vec<CuratedItem> = document
        .stock_extra
        .iter()
        .filter(|kind| selected_ids.contains(&kind.id))
        .map(|kind| CuratedItem { object_id: kind.id.clone(), count: curated_count(document, &kind.id).max(1) })
        .collect();
    if items.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let label = format!("{} kinds / Arten", items.len());
    let text = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&SourcingClipboard { schema: SOURCING_CLIPBOARD_SCHEMA.into(), items }));
    Ok(ClipboardFragment { schema: SOURCING_CLIPBOARD_SCHEMA.into(), media_type: media_type(), dsl_text: text, pack_bytes: None, source_app: crate::editor::sourcing::editor_vocabulary::SOURCING_CONTROLLER_ID.into(), label })
}

/// ✂️ Cutting removes every selected kind from the curation; the fragment [`copy`] authors carries them away.
pub fn cut(document: &CurationSnapshot, selected_ids: &[String]) -> Vec<SourcingMutation> {
    document.curated.iter().filter(|item| selected_ids.contains(&item.object_id)).map(|item| delete_curated_item(item.object_id.clone())).collect()
}
//#endregion 🔖️Copy

//#region 🔖️Paste
/// 🔍️ The packet a fragment carries, refused when it is not a sourcing fragment, is oversized or names an empty identity or count.
pub fn decode(fragment: &ClipboardFragment) -> Result<SourcingClipboard, ClipboardError> {
    if fragment.media_type != media_type() {
        return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));
    }
    if fragment.schema != SOURCING_CLIPBOARD_SCHEMA || fragment.dsl_text.len() > semio_plugin::kernel::CLIPBOARD_TEXT_MAX_BYTES {
        return Err(invalid("Clipboard schema or byte capacity is invalid"));
    }
    let packet: SourcingClipboard = semio_framework_pack_json::from_json_str(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(error.to_string()))?;
    if packet.schema != SOURCING_CLIPBOARD_SCHEMA || packet.items.is_empty() || packet.items.len() > SOURCING_CLIPBOARD_MAX_ITEMS || packet.items.iter().any(|item| item.object_id.is_empty() || item.object_id.len() > SOURCING_CLIPBOARD_MAX_ID_BYTES || item.count == 0) {
        return Err(invalid("Invalid sourcing clipboard packet"));
    }
    Ok(packet)
}

fn mutation_for(decision: CurationDecision) -> Option<SourcingMutation> {
    match decision {
        CurationDecision::NoOp => None,
        CurationDecision::Create(item) => Some(create_curated_item(item)),
        CurationDecision::ChangeCount { object_id, new_count } => Some(change_curated_item_count(object_id, new_count)),
        CurationDecision::Delete { object_id } => Some(delete_curated_item(object_id)),
    }
}

/// 📥️ Adds every pasted pair to the curated count of its kind, bounded by the kind's availability; a kind the document does not stock is skipped. The anchor is accepted and ignored.
pub fn paste(document: &CurationSnapshot, fragment: &ClipboardFragment, _placement: &PastePlacement) -> Result<Vec<SourcingMutation>, ClipboardError> {
    let mut pasted = BTreeMap::<String, u32>::new();
    for item in decode(fragment)?.items {
        let total = pasted.entry(item.object_id).or_default();
        *total = total.saturating_add(item.count);
    }
    Ok(pasted.into_iter().filter_map(|(object_id, count)| mutation_for(curation_decision_for_set(document, &object_id, curated_count(document, &object_id).saturating_add(count)))).collect())
}
//#endregion 🔖️Paste
