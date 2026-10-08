//! 🔄️ Semantic replacement of one precise owned PNG image.
use crate::{PngSnapshot,schema::{snapshot::PngImage,diff::PngDiff,mutations::PngMutation}};

#[derive(semio_framework_value::RetireOwned,Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ReplaceImage {#[dsl(block)] pub image:PngImage}

impl protocol::MutationKind<PngSnapshot,PngMutation> for ReplaceImage {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"replace",entity:"image",kind:"replace-image",record:"ReplaceImage"};
    fn diff(&self,_base:&PngSnapshot)->protocol::MutationOutcome<PngDiff>{
        match self.image.validate(){Ok(())=>protocol::MutationOutcome::new(PngDiff{image:Some(self.image.clone())}),Err(message)=>protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch,message,["image"])}
    }
    fn inverse(&self,base:&PngSnapshot)->Result<Vec<PngMutation>,semio_framework_value::ValueError>{Ok(vec![PngMutation::ReplaceImage(Self{image:base.image.clone()})])}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Replace native image","Natives Bild ersetzen")}
    fn target(&self)->Vec<String>{vec!["image".into()]}
}
