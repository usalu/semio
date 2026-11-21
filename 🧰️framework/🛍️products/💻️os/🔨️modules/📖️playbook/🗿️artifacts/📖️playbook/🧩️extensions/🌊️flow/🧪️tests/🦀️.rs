//! 🧪️ Actual child construction matches the portable independent JSON document oracle.
use super::*;
fn normalized(value:&mut serde_json::Value){match value {serde_json::Value::Number(number)=>*number=serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap(),serde_json::Value::Array(values)=>for value in values{normalized(value)},serde_json::Value::Object(values)=>for value in values.values_mut(){normalized(value)},_=>{}}}
fn corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../../🌊️flow/📝️forms/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()
}

fn project(corpus: &serde_json::Value) -> PlaybookSpec {
 let producer=PlaybookFlowFormProducer;
 let fields=corpus["widgets"].as_array().unwrap().iter().filter_map(|widget|{
  let id=widget["id"].as_str().unwrap();
  Some(match widget["kind"].as_str().unwrap(){
   "inputSlider"=>producer.slider(id,widget["label"].as_str().unwrap(),widget["value"].as_f64().unwrap(),widget["min"].as_f64().unwrap(),widget["max"].as_f64().unwrap(),widget["step"].as_f64().unwrap()),
   "inputNote"=>producer.note(id,widget["text"].as_str().unwrap()),
   "inputImage"=>producer.image(id,widget["src"].as_str().unwrap()),
   "variable"=>return producer.variable(id,widget["name"].as_str().unwrap(),widget["schema"].as_str().unwrap()),
   _=>return None,
  })
 }).collect();
 producer.finish(fields)
}

#[test]
fn actual_playbook_factory_matches_portable_form_corpus() {
 let corpus = corpus();
 let actual = project(&corpus);
 let mut independent=serde_json::to_value(&actual).unwrap();let mut expected=corpus["playbook"].clone();normalized(&mut independent);normalized(&mut expected);assert_eq!(independent,expected);
 let mut native:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&actual))).unwrap();normalized(&mut native);assert_eq!(native,independent);
}

#[test]
fn generation_commands_preserve_typed_answers_and_untouched_axes() {
    use crate::{apply_generation_mutation, generation_operations, selected_generation, GenerationPlayState};
    use semio_framework_value::{FromValue, ToValue};

    let corpus = corpus();
    let spec = project(&corpus);
    let mut state = GenerationPlayState::default();
    for operation in generation_operations("addGeneration", None, &state, &spec).unwrap() {
        apply_generation_mutation(&mut state, &operation);
    }
    for edit in corpus["answerEdits"].as_array().unwrap() {
        let args = DslValue::from(edit.clone());
        let operations = generation_operations("updateGenerationValues", Some(&args), &state, &spec);
        assert_eq!(operations.is_some(), edit["accepted"].as_bool().unwrap(), "{edit}");
        if let Some(operations) = operations {
            for operation in operations {
                apply_generation_mutation(&mut state, &operation);
            }
        }
    }
    let generation = selected_generation(&state).unwrap();
    let mut actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&generation.values.to_value())).unwrap();
    let mut oracle = corpus["expectedAnswers"].clone();
    normalized(&mut oracle);
    normalized(&mut actual);
    assert_eq!(actual, oracle);
    GenerationPlayState::retire_decoded(state);
}
