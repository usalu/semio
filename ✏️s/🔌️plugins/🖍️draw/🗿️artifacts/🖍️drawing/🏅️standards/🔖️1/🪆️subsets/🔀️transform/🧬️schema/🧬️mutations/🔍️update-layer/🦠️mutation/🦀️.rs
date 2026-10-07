//! 🔧 Drawing mutation — `UpdateLayerTraceParams`: sets a trace layer's `params` facet (threshold +
//! simplify epsilon, always validated/persisted together — the `update` verb's cohesive-facet case).
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::{DrawingSnapshot, DrawingTraceParams};

//#region 🔖️Mutation
/// 🔧 `update-layer-trace-params` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-layer-trace-params")]
pub struct UpdateLayerTraceParams {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(block)]
    pub params: DrawingTraceParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_layer_trace_params(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, params: DrawingTraceParams) -> DrawingMutation {
    DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id, params })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdateLayerTraceParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer-trace-params", record: "UpdatedLayerTraceParams" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update layer \"{}\" trace params", self.layer_id), &format!("Nachzeichnungsparameter von Ebene \"{}\" aktualisieren", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
