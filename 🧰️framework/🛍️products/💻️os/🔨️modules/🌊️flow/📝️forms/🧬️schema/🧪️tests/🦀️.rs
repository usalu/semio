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
 fn variable(&self,id:&str,_:&str,_:&str)->Option<Self::Field>{Some(self.field(id,"variable"))}
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

#[test]
fn generation_seeds_keep_registered_schemas_and_original_port_names() {
    use crate::forms_bridge::add_generation_variable_seeds;
    use semio_framework_artifact_flow_flow::neural::ColdRetire;
    use semio_framework_value::ToValue;

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let widgets = semio_framework_pack_json::from_json_str::<Vec<Widget>>(&corpus["widgets"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let answers = DslValue::from(corpus["expectedAnswers"].clone());
    let mut snapshot = FlowHostSnapshot::default();
    snapshot.replace_widgets(widgets);
    let mut seeds = std::collections::HashMap::new();
    add_generation_variable_seeds(&snapshot, |id| answers.get(id), &mut seeds);
    let expected = corpus["expectedSeeds"].as_object().unwrap();
    assert_eq!(seeds.len(), expected.len());
    for (id, expected) in expected {
        let port = expected["port"].as_str().unwrap();
        let dictionary = seeds[id].get(port).and_then(|value| value.as_dictionary()).unwrap();
        assert_eq!(dictionary.schema(), expected["schema"].as_str());
        for (key, expected) in expected["fields"].as_object().unwrap() {
            let actual = dictionary.get(key).unwrap().to_value();
            if let Some(number) = expected.as_f64() {
                assert_eq!(actual.as_f64(), Some(number));
            } else {
                assert_eq!(actual, DslValue::from(expected.clone()));
            }
        }
    }
    seeds.retire_cold();

    snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec {
        id: "supply-width".into(), from: "amount".into(), to: "width".into(), from_port: "number".into(), to_port: "width".into(),
    });
    let mut seeds = std::collections::HashMap::new();
    add_generation_variable_seeds(&snapshot, |id| answers.get(id), &mut seeds);
    assert!(!seeds.contains_key("width"));
    assert_eq!(seeds.len() + 1, expected.len());
    let producer = InputProducer { loans: RefCell::new(Vec::new()) };
    let form = flow_host_snapshot_to_form_spec(&snapshot, &producer);
    assert!(!form.iter().any(|(id, _)| id == "width"));
    seeds.retire_cold();
    snapshot.retire_cold();
}
