//! 🪪️ Native member capability witnesses preserve immutable original types independently of the cold reference facet.
use super::*;
use super::tests::DemoSnapshot;
use super::fixture_mutations::demo::DemoMutation;

struct OriginalProjection(DemoSnapshot);
impl ToValue for OriginalProjection { fn to_value(&self)->semio_framework_value::DslValue { self.0.to_value() } }
impl ArtifactPack for OriginalProjection {
 fn encode_pack_with(&self,options:&PackEncodeOptions)->Result<Vec<u8>,PackError>{self.0.encode_pack_with(options)}
 fn decode_pack_with(bytes:&[u8],options:&PackDecodeOptions)->Result<Self,PackError>{DemoSnapshot::decode_pack_with(bytes,options).map(Self)}
}
impl semio_framework_schema_composition::ArtifactCompositionFields for OriginalProjection {
 fn visit_child_refs<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,_:&mut V)->Result<(),V::Error>{Ok(())}
}
#[derive(Default,PartialEq,semio_framework_value_derive::ToValue)]
struct NativeDelta { value:Option<i32> }
impl crate::os_spr::DiffAlgebra<OriginalProjection> for NativeDelta { fn is_empty(&self)->bool { self.value.is_none() } }
impl crate::os_spr::MutationDiff<OriginalProjection> for NativeDelta {}
struct OriginalOperation(DemoMutation);
impl ToValue for OriginalOperation { fn to_value(&self)->semio_framework_value::DslValue {self.0.to_value()} }
impl FromValue for OriginalOperation {fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{DemoMutation::from_value(value).map(Self)}}
impl OpBinary for OriginalOperation {
 fn encode_op(&self)->Result<Vec<u8>,crate::os_spr::ProtocolError>{self.0.encode_op()}
 fn decode_op(bytes:&[u8])->Result<Self,crate::os_spr::ProtocolError>{DemoMutation::decode_op(bytes).map(Self)}
}
impl OpText for OriginalOperation {
 fn parse_op(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{DemoMutation::parse_op(line).map(Self)}
 fn print_op(&self)->String{self.0.print_op()}
}
impl crate::os_spr::Mutation<OriginalProjection> for OriginalOperation {
 type Diff=NativeDelta;
 const DESCRIPTORS:&'static [crate::os_spr::MutationLeafDescriptor]=<DemoMutation as crate::os_spr::Mutation<DemoSnapshot>>::DESCRIPTORS;
 fn descriptor(&self)->&'static crate::os_spr::MutationLeafDescriptor{<DemoMutation as crate::os_spr::Mutation<DemoSnapshot>>::descriptor(&self.0)}
}
fn assert_native_member<P,M>()
where P:ToValue+ArtifactPack+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,M:ToValue+FromValue+crate::os_spr::Mutation<P>+OpBinary+OpText+Send+'static {
 fn member<T:SpaceMember>(){}
 member::<ArtifactStore<P,M>>();
}
/// 🧊️ The actual live member implementation accepts concrete types with neither ordinary Clone nor cold operation arithmetic.
#[test]
fn native_member_facet_admits_non_clone_non_reference_types(){
 assert_native_member::<OriginalProjection,OriginalOperation>();
 let original=OriginalProjection(DemoSnapshot{n:Some(7)});
 assert_eq!(serde_json::Value::from(original.to_value()),serde_json::json!({"n":7}));
}
/// 📖️ Portable explicit selection never removes native capabilities or manufactures reference capabilities.
#[test]
fn original_member_profiles_match_explicit_authority_selection(){
 let law:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){assert!(row["native"].as_bool().unwrap());let admitted=["snapshotClone","mutationClone","coldMutation","referenceSelected"].into_iter().all(|key|row[key].as_bool().unwrap());assert_eq!(admitted,row["reference"].as_bool().unwrap());}
}