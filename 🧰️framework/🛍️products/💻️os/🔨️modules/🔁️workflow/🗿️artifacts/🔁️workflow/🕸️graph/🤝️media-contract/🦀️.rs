//! 🤝️ Media contract of the workflow graph: the connect-time negotiated wire contract and its placeholder. See [`super`].

use super::*;

//#region 🔖️MediaContract
/// 🤝️ A connect-time negotiated wire contract between two `WorkflowMediaPort`s — stored on
/// `WorkflowEdge` so later passes (`validate_workflow`, merge reconciliation) can re-check it
/// without re-resolving the artifact registry. `kind_id`/`media_type` describe the *accepted*
/// (target) side — see `semio_framework::media_types_compatible`. Ported down from
/// `framework/product/os/core`'s `workflow` module (`MediaContract`) so the persisted graph carries
/// its own edge contracts; the negotiation logic itself (`negotiate_media_contract`) stays in
/// os-core for now since it needs the artifact-kind registry, which doesn't exist at this layer yet.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct MediaContract {
    pub kind_id: String,
    pub media_type: MediaType,
    pub wire: MediaWireFormat,
    pub conversion: Option<(MediaForm, MediaForm)>,
}

/// 🧪️ Placeholder contract for test/fixture edges built without a real port-negotiation context —
/// schema pinned to `kind_id` itself, `Data`/`Value` media type (mirrors os-core's unregistered-kind
/// registry fallback).
pub async fn placeholder_media_contract(kind_id: &str) -> MediaContract {
    MediaContract { kind_id: kind_id.into(), media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, wire: MediaWireFormat::Document { schema: kind_id.into() }, conversion: None }
}
//#endregion 🔖️MediaContract
