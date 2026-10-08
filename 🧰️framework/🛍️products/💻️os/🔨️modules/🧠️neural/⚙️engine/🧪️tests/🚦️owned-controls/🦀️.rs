use crate::{Atom,ColdOwner,Dictionary};
use semio_framework_value::{DslValue,FromValue,ToValue,NativeDecodeControl,NativeEncodeControl,native_decoding::NativeDecodeProgress,native_encoding::NativeEncodeProgress};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🚦️owned-controls.json")).unwrap()}

fn input_grant(owner:&crate::retirement::RetainedDictionaryInput,copy:usize,closing:bool)->crate::RetainedCloneGrant {
    crate::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,
        maximum_capacity_bytes:if closing{owner.next_close_capacity_byte_demand(copy).unwrap()}else{owner.next_capacity_byte_demand(copy).unwrap()},
        maximum_release_bytes:if closing{owner.next_close_release_byte_demand().unwrap()}else{owner.next_release_byte_demand().unwrap()},
        maximum_depth:if closing{owner.next_close_depth_demand().unwrap()}else{owner.next_depth_demand().unwrap()}}
}
fn close_input(owner:&mut crate::retirement::RetainedDictionaryInput,copy:usize) {
    for _ in 0..100000 {
        if owner.terminal_is_empty(){return;}
        let grant=input_grant(owner,copy,true);
        let (step,born,freed)=crate::registry::tests::observe_ownership(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));
    }
    panic!("original input cancellation retained an allocation");
}
#[test]
fn retained_dictionary_binding_moves_typed_input_under_existing_owner_grants() {
    use crate::retirement::RetainedDictionaryInput;
    use crate::registry::tests::observe_ownership;
    let fixture=fixture();let law=&fixture["retainedBinding"];let key=law["keyUnit"].as_str().unwrap().repeat(law["keyRepeats"].as_u64().unwrap()as usize);let label=law["textUnit"].as_str().unwrap().repeat(law["textRepeats"].as_u64().unwrap()as usize);let expected=serde_json::json!({"nested":{"first":label.clone(),"second":-3.25},key.clone():label.clone(),format!("{key}2"):label});let text=expected.to_string();let oracle=ColdOwner::new(serde_json::from_str::<Dictionary>(&text).unwrap());
    let input=||semio_framework_pack_json::from_json_str::<DslValue>(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let original=input();let (mut owner,born,freed)=observe_ownership(||RetainedDictionaryInput::new(original));assert_eq!((born,freed),(law["physical"]["sourceBirthBytes"].as_u64().unwrap()as usize,law["physical"]["sourceReleaseBytes"].as_u64().unwrap()as usize));let mut phases=std::collections::BTreeSet::new();let mut turns=0;
    let actual=loop {
        let before=owner.progress();phases.insert(before.2);let grant=input_grant(&owner,law["grantBytes"].as_u64().unwrap()as usize,false);
        let (zero,born,freed)=observe_ownership(||owner.step(crate::RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert!(zero.dictionary.is_none());assert_eq!(zero.progress,Default::default());assert_eq!((born,freed),(0,0));assert_eq!(owner.progress(),before);
        if grant.maximum_capacity_bytes>0 {
            let (short,born,freed)=observe_ownership(||owner.step(crate::RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}));assert_eq!((born,freed),(0,0));if let Ok(step)=short{assert_eq!(step.progress,Default::default());assert!(step.dictionary.is_none());}
        }
        turns+=1;assert!(turns<100000);
        let (step,born,freed)=observe_ownership(||owner.step(grant).unwrap());assert!(step.progress.fits(grant));assert_eq!((step.progress.retained_capacity_bytes,step.progress.released_bytes),(born,freed));
        if let Some(output)=step.dictionary {break ColdOwner::new(output);}
    };
    assert_eq!(&*actual,&*oracle);assert!(owner.terminal_is_empty());for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()));}
    for cutoff in law["cancelUnits"].as_array().unwrap() {
        let mut owner=RetainedDictionaryInput::new(input());
        for _ in 0..cutoff.as_u64().unwrap(){let grant=input_grant(&owner,3,false);let (step,born,freed)=observe_ownership(||owner.step(grant).unwrap());assert!(step.dictionary.is_none());assert_eq!((step.progress.retained_capacity_bytes,step.progress.released_bytes),(born,freed));}
        let before=owner.progress();owner.cancel();let grant=input_grant(&owner,3,true);let (zero,born,freed)=observe_ownership(||owner.close_step(crate::RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!((born,freed),(0,0));assert_eq!(owner.progress(),before);close_input(&mut owner,3);
    }
    let refused=fixture["invalidDictionaryText"].as_str().unwrap();assert!(serde_json::from_str::<Dictionary>(refused).is_err());let mut owner=RetainedDictionaryInput::new(semio_framework_pack_json::from_json_str(refused,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    for turn in 0..100000 {let grant=input_grant(&owner,3,false);match owner.step(grant){Ok(step)=>{if let Some(value)=step.dictionary{drop(ColdOwner::new(value));panic!("malformed typed input admitted");}},Err(_)=>break}assert!(turn<99999);}
    owner.cancel();close_input(&mut owner,3);eprintln!("[DEBUG] Original input shared headers/frame/update work and cancellation physical receipts, turns={turns}, phases={phases:?}");
}

#[test]
fn owned_dictionary_controls_preserve_the_independent_typed_domain(){
 let f=fixture();let text=f["dictionaryText"].as_str().unwrap();let borrowed:DslValue=semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let expected=ColdOwner::new(serde_json::from_str::<Dictionary>(text).unwrap());let ordinary=ColdOwner::new(semio_framework_pack_json::from_json_str::<Dictionary>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());let mut accepted=|_|true;let mut control=NativeDecodeControl::new(2_000_000,&mut accepted);let actual=ColdOwner::new(Dictionary::from_value_controlled(&borrowed,&mut control).unwrap());assert_eq!(&*actual,&*expected);assert_eq!(&*actual,&*ordinary);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(2_000_000,&mut accepted);let encoded=actual.to_value_controlled(&mut control).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&encoded)).unwrap(),serde_json::from_str::<serde_json::Value>(text).unwrap());let keys:Vec<_>=actual.keys().map(String::as_str).collect();assert_eq!(keys,["a","z"]);
}

#[test]
fn owned_integer_atoms_refuse_unsigned_overflow_without_reinterpretation(){
 let f=fixture();
 for text in f["acceptedIntegerTexts"].as_array().unwrap(){let text=text.as_str().unwrap();let expected=serde_json::from_str::<i64>(text).unwrap();let value:DslValue=semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(Atom::from_value(value.clone()).unwrap(),Atom::Integer(expected));let mut accepted=|_|true;let mut control=NativeDecodeControl::new(64,&mut accepted);assert_eq!(Atom::from_value_controlled(&value,&mut control).unwrap(),Atom::Integer(expected));}
 for text in f["refusedIntegerTexts"].as_array().unwrap(){let text=text.as_str().unwrap();assert!(serde_json::from_str::<i64>(text).is_err());let value:DslValue=semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert!(Atom::from_value(value.clone()).is_err(),"{text}");let mut accepted=|_|true;let mut control=NativeDecodeControl::new(64,&mut accepted);assert!(Atom::from_value_controlled(&value,&mut control).is_err(),"{text}");}
}

#[test]
fn owned_dictionary_limits_and_cancellation_retire_partial_frontiers(){
 let f=fixture();let value:DslValue=semio_framework_pack_json::from_json_str(f["dictionaryText"].as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut refused=0;
 for maximum in f["allocationCeilings"].as_array().unwrap(){let maximum=maximum.as_u64().unwrap() as usize;let mut accepted=|_|true;let mut control=NativeDecodeControl::new(maximum,&mut accepted);match Dictionary::from_value_controlled(&value,&mut control){Ok(value)=>value.retire_decoded(),Err(_)=>refused+=1}assert!(control.owned_bytes()<=maximum);}
 assert!(refused>0);let before=refused;
 for cutoff in f["callbackCutoffs"].as_array().unwrap(){let cutoff=cutoff.as_u64().unwrap() as usize;let mut calls=0;let mut progress=|_|{calls+=1;calls<=cutoff};let mut control=NativeDecodeControl::new(2_000_000,&mut progress);match Dictionary::from_value_controlled(&value,&mut control){Ok(value)=>value.retire_decoded(),Err(_)=>refused+=1}}
 assert!(refused>before);let malformed=f["invalidDictionaryText"].as_str().unwrap();assert!(serde_json::from_str::<Dictionary>(malformed).is_err());let value:DslValue=semio_framework_pack_json::from_json_str(malformed, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(2_000_000,&mut accepted);assert!(Dictionary::from_value_controlled(&value,&mut control).is_err());
}

#[test]
fn owned_dictionary_controls_cancel_inside_key_and_value_text_copies(){
 let f=fixture();let text=f["textUnit"].as_str().unwrap().repeat(f["textRepeats"].as_u64().unwrap() as usize);
 for key in[false,true]{let value=DslValue::Object(vec![(if key{text.clone()}else{"owned".into()},DslValue::String(if key{"value".into()}else{text.clone()}))]);let mut stopped=false;let mut progress=|event:NativeDecodeProgress|{if event.total==text.len()&&event.completed>0&&event.completed<event.total{stopped=true;false}else{true}};let mut control=NativeDecodeControl::new(2_000_000,&mut progress);assert!(Dictionary::from_value_controlled(&value,&mut control).is_err());assert!(stopped);let mut accepted=|_|true;let mut control=NativeDecodeControl::new(2_000_000,&mut accepted);let source=ColdOwner::new(Dictionary::from_value_controlled(&value,&mut control).unwrap());let mut stopped=false;let mut progress=|event:NativeEncodeProgress|{if event.total==text.len()&&event.completed>0&&event.completed<event.total{stopped=true;false}else{true}};let mut control=NativeEncodeControl::new(2_000_000,&mut progress);assert!(source.to_value_controlled(&mut control).is_err());assert!(stopped);}
}

#[test]
fn owned_empty_dictionary_default_restores_the_enclosing_workload(){
 let mut accepted=|_|true;let mut control=NativeDecodeControl::new(0,&mut accepted);control.begin_stage(4).unwrap();control.step().unwrap();let value=ColdOwner::new(Dictionary::default_value_controlled(&mut control).unwrap());assert!(value.is_empty());assert_eq!(control.owned_bytes(),0);control.advance(3).unwrap();assert!(control.step().is_err());
}


#[test]
fn owned_neural_constructor_failures_retire_completed_dictionary_frontiers() {
    #[derive(serde::Deserialize)]
    struct OracleNeuron {
        #[serde(rename = "id")] _id: String,
        #[serde(rename = "kind")] _kind: String,
        #[serde(default, rename = "params")] _params: std::collections::HashMap<String, serde_json::Value>,
        #[serde(default, rename = "tree")] _tree: Option<OracleTree>,
    }
    #[derive(serde::Deserialize)]
    struct OracleSynapse {
        #[serde(rename = "id")] _id: String,
        #[serde(rename = "from")] _from: String,
        #[serde(rename = "to")] _to: String,
    }
    #[derive(serde::Deserialize)]
    struct OracleTree {
        #[serde(default, rename = "neurons")] _neurons: Vec<OracleNeuron>,
        #[serde(default, rename = "synapses")] _synapses: Vec<OracleSynapse>,
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛬️construction.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let payload = case["payload"].clone();
        if case["oracle"] == "integer" {
            let mut value = &payload;
            for segment in case["integerPath"].as_array().unwrap() {
                let segment = segment.as_str().unwrap();
                value = if value.is_array() { &value[segment.parse::<usize>().unwrap()] } else { &value[segment] };
            }
            assert!(serde_json::from_value::<i64>(value.clone()).is_err());
        } else if case["owner"] == "neuron" {
            assert!(serde_json::from_value::<OracleNeuron>(payload.clone()).is_err());
        } else {
            assert!(serde_json::from_value::<OracleTree>(payload.clone()).is_err());
        }
        let input: DslValue = semio_framework_pack_json::from_json_str(&serde_json::to_string(&payload).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let refused = std::panic::catch_unwind(|| {
            if case["owner"] == "neuron" {
                match crate::Neuron::from_value(input) { Ok(value) => { crate::ColdRetire::retire_cold(value); false }, Err(_) => true }
            } else {
                match crate::Tree::from_value(input) { Ok(value) => { crate::ColdRetire::retire_cold(value); false }, Err(_) => true }
            }
        });
        assert!(refused.is_ok_and(|refused| refused), "{}", case["id"]);
    }
}


#[test]
fn owned_neural_controlled_dictionary_keeps_refusal_categories_at_partial_boundaries() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛬️construction.json")).unwrap();
    let kinds = &corpus["controlledRefusals"];
    let input = DslValue::Object(vec![("owned".into(), DslValue::String("literal".into()))]);
    let mut cancel = |_| false;
    let mut control = NativeDecodeControl::new(1_000_000, &mut cancel);
    let error = Dictionary::from_value_controlled(&input, &mut control).unwrap_err().under("params");
    assert_eq!(error.kind.as_str(), kinds["canceled"].as_str().unwrap());
    let mut allow = |_| true;
    let mut control = NativeDecodeControl::new(0, &mut allow);
    let error = Dictionary::from_value_controlled(&input, &mut control).unwrap_err();
    assert_eq!(error.kind.as_str(), kinds["allocation"].as_str().unwrap());
    assert!(serde_json::from_str::<Dictionary>("{\"owned\":[]}").is_err());
    let mut allow = |_| true;
    let mut control = NativeDecodeControl::new(1_000_000, &mut allow);
    let invalid = DslValue::Object(vec![("owned".into(), DslValue::Array(Vec::new()))]);
    let error = Dictionary::from_value_controlled(&invalid, &mut control).unwrap_err();
    assert_eq!(error.kind.as_str(), kinds["malformed"].as_str().unwrap());
    let mut builder = crate::ColdDictionaryBuilder::new();
    builder.insert("first".into(), crate::Value::Atom(Atom::Integer(7)));
    let nested = Dictionary::new().insert("nested", crate::Value::Atom(Atom::String("owned frontier".into())));
    let mut calls = 0;
    let mut cancel_inside = |_| { calls += 1; calls == 1 };
    let mut control = NativeDecodeControl::new(1_000_000, &mut cancel_inside);
    control.begin_stage(0).unwrap();
    let error = builder.insert_controlled("second".into(), crate::Value::Dictionary(nested), &mut control).unwrap_err().under("params");
    assert_eq!(error.kind.as_str(), kinds["canceled"].as_str().unwrap());
    assert!(control.owned_bytes() > 0);
    let retained = ColdOwner::new(builder.finish());
    assert_eq!(retained.len(), 1);
    assert_eq!(retained.get("first"), Some(&crate::Value::Atom(Atom::Integer(7))));
    eprintln!("[DEBUG] Controlled neural refusal categories survive canceled partial dictionary insertion");
}
