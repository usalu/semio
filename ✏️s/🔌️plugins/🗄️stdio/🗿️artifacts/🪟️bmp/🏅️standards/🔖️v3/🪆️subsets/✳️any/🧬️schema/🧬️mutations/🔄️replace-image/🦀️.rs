//! 🔄️ Semantic replacement of one precise owned BMP image.
use crate::{BmpSnapshot,schema::{snapshot::BmpImage,diff::BmpDiff,mutations::BmpMutation}};

#[derive(semio_framework_value::RetireOwned,Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ReplaceImage {#[dsl(block)] pub image:BmpImage}

impl protocol::MutationKind<BmpSnapshot,BmpMutation> for ReplaceImage {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"replace",entity:"image",kind:"replace-image",record:"ReplaceImage"};
    fn diff(&self,_base:&BmpSnapshot)->protocol::MutationOutcome<BmpDiff>{
        match self.image.validate(){Ok(())=>protocol::MutationOutcome::new(BmpDiff{image:Some(self.image.clone()),..BmpDiff::default()}),Err(message)=>protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch,message,["image"])}
    }
    fn inverse(&self,base:&BmpSnapshot)->Result<Vec<BmpMutation>,semio_framework_value::ValueError>{Ok(vec![BmpMutation::ReplaceImage(Self{image:base.image.clone()})])}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Replace native image","Natives Bild ersetzen")}
    fn target(&self)->Vec<String>{vec!["image".into()]}
}
