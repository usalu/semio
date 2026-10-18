//! 🧪️ Original source loans and opaque producer results share one portable corpus.
use super::*;
use crate::forms_bridge::flow_host_snapshot_to_form_spec;
use semio_framework_artifact_flow_flow::artifact::{FlowHostSnapshot,Widget};
use std::cell::RefCell;
struct InputProducer {loans:RefCell<Vec<usize>>}
impl InputProducer {fn field(&self,id:&str,kind:&str)->(String,String){self.loans.borrow_mut().push(id.as_ptr() as usize);(id.to_owned(),kind.to_owned())}}
impl FlowFormProducer for InputProducer {
 type Field=(String,String);type Form=Vec<Self::Field>;
 fn slider(&self,id:&str,_:&str,_:f64,_:f64,_:f64,_:f64)->Self::Field{self.field(id,"slider")}
 fn note(&self,id:&str,_:&str)->Self::Field{self.field(id,"note")}
 fn image(&self,id:&str,_:&str)->Self::Field{self.field(id,"image")}
 fn variable(&self,id:&str,_:&str,_:&str)->Self::Field{self.field(id,"variable")}
 fn finish(&self,fields:Vec<Self::Field>)->Self::Form{fields}
}
#[test]
fn selected_form_producer_keeps_original_widget_loans(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let widgets=semio_framework_pack_json::from_json_str::<Vec<Widget>>(&corpus["widgets"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let mut snapshot=FlowHostSnapshot::default();snapshot.replace_widgets(widgets);
 let expected=snapshot.widgets.iter().filter_map(|widget|match widget {Widget::InputSlider{id,..}|Widget::InputNote{id,..}|Widget::InputImage{id,..}|Widget::Variable{id,..}=>Some(id.as_ptr()as usize),_=>None}).collect::<Vec<_>>();
 let producer=InputProducer{loans:RefCell::new(Vec::new())};
 let actual=flow_host_snapshot_to_form_spec(&snapshot,&producer);
 assert_eq!(serde_json::to_value(&actual).unwrap(),corpus["selectedInputs"]);
 assert_eq!(*producer.loans.borrow(),expected);
 snapshot.retire_cold();
}
