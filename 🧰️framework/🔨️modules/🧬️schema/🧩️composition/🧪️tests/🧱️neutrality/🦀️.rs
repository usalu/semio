use semio_framework_schema_composition::{ChildFieldRefs,ChildRefFields,ChildRefVisitor};
use serde_json::{Value,json};
enum Node { Child(Value),Optional(Option<Nested>),List(Vec<Node>) }
struct Nested(Box<Node>);
impl ChildFieldRefs for Nested {
 const MANY:bool=false;
 fn visit_child_field<'a,V:ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{self.0.visit_child_field(slot,visitor)}
}
impl Node {
 fn from_input(value:&Value)->Self{match value["tag"].as_str().unwrap(){"child"=>Self::Child(value["identity"].clone()),"none"=>Self::Optional(None),"some"=>Self::Optional(Some(Nested(Box::new(Self::from_input(&value["value"]))))),"list"=>Self::List(value["values"].as_array().unwrap().iter().map(Self::from_input).collect()),_=>panic!("closed projection tag")}}
}
impl ChildFieldRefs for Node {
 const MANY:bool=false;
 fn visit_child_field<'a,V:ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{match self{Self::Child(identity)=>{visitor.step()?;visitor.child(slot,ChildRefFields{child_id:identity["id"].as_str().unwrap(),artifact_id:identity["artifact"].as_str().unwrap(),artifact_kind:identity["kind"].as_str().unwrap(),standard:identity["standard"].as_str().unwrap(),subset:identity["subset"].as_str().unwrap()})},Self::Optional(value)=>value.visit_child_field(slot,visitor),Self::List(values)=>values.visit_child_field(slot,visitor)}}
}
struct Visitor{maximum:usize,steps:usize,rows:Vec<Value>}
impl<'a> ChildRefVisitor<'a> for Visitor{
 type Error=();
 fn step(&mut self)->Result<(),()>{if self.steps>=self.maximum{return Err(())}self.steps+=1;Ok(())}
 fn child(&mut self,slot:&'static str,fields:ChildRefFields<'a>)->Result<(),()>{self.rows.push(json!([slot,fields.child_id,fields.artifact_id,fields.artifact_kind,fields.standard,fields.subset]));Ok(())}
}
#[test]
fn portable_child_projection_vectors_preserve_exact_admission_and_identity(){
 let fixture:Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧱️neutrality/🔣️.json")).unwrap();
 assert_eq!(fixture["composition"].as_array().unwrap().len(),8);
 for row in fixture["composition"].as_array().unwrap(){let mut visitor=Visitor{maximum:row["maximumSteps"].as_u64().unwrap() as usize,steps:0,rows:vec![]};let result=row["fields"].as_array().unwrap().iter().try_for_each(|field|{let slot:&'static str=Box::leak(field["slot"].as_str().unwrap().to_owned().into_boxed_str());Node::from_input(&field["value"]).visit_child_field(slot,&mut visitor)});assert_eq!(json!({"accepted":result.is_ok(),"steps":visitor.steps,"rows":visitor.rows}),row["expected"],"{}",row["id"]);}
 assert!(!<Option<Node> as ChildFieldRefs>::MANY);assert!(<Vec<Node> as ChildFieldRefs>::MANY);assert!(<Option<Vec<Node>> as ChildFieldRefs>::MANY);
}
