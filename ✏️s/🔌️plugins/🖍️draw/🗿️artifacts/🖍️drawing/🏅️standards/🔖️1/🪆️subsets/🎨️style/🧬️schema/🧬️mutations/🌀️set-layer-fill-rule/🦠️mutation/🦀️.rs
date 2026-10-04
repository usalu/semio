//! 🌀️ Sets a layer's authored compound-path fill rule.
use crate::{DrawingSnapshot,FillRule};
use crate::mutations::DrawingMutation;
use crate::diff::DrawingDiff;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[dsl(keyword="set-layer-fill-rule")]
pub struct SetLayerFillRule {pub layer_id:String,pub fill_rule:FillRule}
pub fn set_layer_fill_rule(layer_id:String,fill_rule:FillRule)->DrawingMutation {DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id,fill_rule})}
impl protocol::MutationKind<DrawingSnapshot,DrawingMutation> for SetLayerFillRule {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"set",entity:"layer",kind:"set-layer-fill-rule",record:"SetLayerFillRule"};
    fn diff(&self,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {super::diff::diff(self,base)}
    fn inverse(&self,base:&DrawingSnapshot)-> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({super::inverse::inverse(self,base)?
    })
}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" fill rule to {}",self.layer_id,self.fill_rule.as_str()),&format!("Füllregel von Ebene \"{}\" auf {} setzen",self.layer_id,self.fill_rule.as_str()))}
    fn target(&self)->Vec<String> {vec![self.layer_id.clone()]}
}
