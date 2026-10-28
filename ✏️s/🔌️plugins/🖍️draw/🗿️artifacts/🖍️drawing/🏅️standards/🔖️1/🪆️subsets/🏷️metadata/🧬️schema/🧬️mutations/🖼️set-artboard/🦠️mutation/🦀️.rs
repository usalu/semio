//! 🖼️ Artboard replacement belongs to the Drawing member's actual typed mutation stream.
use semio_s_artifact_draw_drawing_model::{DrawingArtboard,snapshot::DrawingSnapshot};
use semio_s_artifact_draw_drawing_diff::DrawingDiff;
#[derive(Clone,Debug,PartialEq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf,semio_framework_value::CanonicalJsonTree,semio_framework_value::NativeReceive)]
#[canonical_json(owner=semio_framework_pack_json)]#[mutation_leaf(contract=::protocol)]
#[cfg_attr(any(test,feature="testkit"),derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]#[cfg_attr(any(test,feature="testkit"),serde(rename_all="camelCase"))]
#[dsl(keyword="set-artboard")]
pub struct SetArtboard{pub artboard:Option<DrawingArtboard>}
pub fn set_artboard<Op:From<SetArtboard>>(artboard:Option<DrawingArtboard>)->Op{SetArtboard{artboard}.into()}
pub fn valid(artboard:&Option<DrawingArtboard>)->bool{artboard.as_ref().is_none_or(|a|a.width.is_finite()&&a.width>0.0&&a.height.is_finite()&&a.height>0.0)}
pub fn copy(artboard:&Option<DrawingArtboard>)->Option<DrawingArtboard>{artboard.as_ref().map(|a|DrawingArtboard{width:a.width,height:a.height})}
impl<Op> protocol::MutationKind<DrawingSnapshot,Op> for SetArtboard where Op:protocol::Mutation<DrawingSnapshot,Diff=DrawingDiff>+From<SetArtboard>{
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"set",entity:"artboard",kind:"set-artboard",record:"SetArtboard"};
 fn diff(&self,b:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff>{if !valid(&self.artboard){return protocol::MutationOutcome::fatal("mutation.invariant","Artboard dimensions must be finite and positive",["artboard"])}if self.artboard==b.artboard{return protocol::MutationOutcome::empty().warning("mutation.no-op","The Drawing artboard already has these dimensions")}protocol::MutationOutcome::new(DrawingDiff{artboard:Some(copy(&self.artboard)),..Default::default()})}
 fn inverse(&self,b:&DrawingSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{Ok(vec![SetArtboard{artboard:copy(&b.artboard)}.into()])}
 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Set artboard","Zeichenfläche festlegen")}
 fn target(&self)->Vec<String>{vec!["artboard".into()]}
}
