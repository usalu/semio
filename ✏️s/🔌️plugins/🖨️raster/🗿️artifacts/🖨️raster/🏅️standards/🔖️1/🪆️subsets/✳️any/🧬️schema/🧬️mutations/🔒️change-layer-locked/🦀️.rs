//! 🔒️ Guarded, undoable changes to persisted layer protection.
use crate::{RasterLayerPatch,RasterMutation,RasterSnapshot};
use crate::diff::{diff_patch_layer,RasterDiff};
use crate::standards::v1::subsets::any::schema::{find_layer,layer_locked};
#[derive(Clone,Debug,PartialEq,dsl::ToValue,dsl::FromValue,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase")]
pub struct ChangeLayerLocked {pub layer_id:String,pub expected:bool,pub locked:bool}
pub fn validate(payload:&ChangeLayerLocked,base:&RasterSnapshot)->Result<(),&'static str> {
    let layer=find_layer(&base.layers,&payload.layer_id).ok_or("mutation.target-missing")?;
    if layer_locked(layer)!=payload.expected {return Err("mutation.target-mismatch");}Ok(())
}
impl protocol::MutationKind<RasterSnapshot,RasterMutation> for ChangeLayerLocked {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"change",entity:"layer-lock",kind:"change-layer-locked",record:"ChangedLayerLocked"};
    fn diff(&self,base:&RasterSnapshot)->protocol::MutationOutcome<RasterDiff> {
        if let Err(code)=validate(self,base){return protocol::MutationOutcome::refuse(code,"Layer protection changed before this edit.",[self.layer_id.clone()]);}
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id,RasterLayerPatch {locked:Some(self.locked),..Default::default()}))
    }
    fn inverse(&self,base:&RasterSnapshot)->Vec<RasterMutation> {
        let Some(layer)=find_layer(&base.layers,&self.layer_id) else {return Vec::new();};
        vec![RasterMutation::ChangeLayerLocked(Self {layer_id:self.layer_id.clone(),expected:self.locked,locked:layer_locked(layer)})]
    }
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Change layer protection","Ebenenschutz ändern")}
    fn target(&self)->Vec<String>{vec![self.layer_id.clone()]}
}
