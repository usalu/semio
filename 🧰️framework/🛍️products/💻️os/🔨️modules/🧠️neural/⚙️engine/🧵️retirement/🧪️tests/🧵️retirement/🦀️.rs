//! 🧪️ Nested byte accounting, shared ownership, cold decoder errors, and explicit terminal guards.

use super::*;
use std::mem::size_of;

#[test]
fn recursive_dictionary_owns_typed_retirement_under_independent_grants() {
    use semio_framework_value::{retirement::{RetireOwned, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎮️typed-owners/🔣️.json")).unwrap();
    assert!(Dictionary::controlled_retirement_supported());
    assert!(Value::controlled_retirement_supported());
    assert!(Atom::controlled_retirement_supported());
    for row in fixture["cases"].as_array().unwrap() {
        for copy in fixture["grants"].as_array().unwrap() {
            let ((source,pointer),original_born,original_freed)=observe_ownership(||{
                let mut text=String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize);
                text.push_str(row["text"].as_str().unwrap());let pointer=text.as_ptr();
                (Dictionary::new().insert("nested",Value::Dictionary(Dictionary::new().insert("value",Value::Atom(Atom::String(text))))),pointer)
            });
            assert_eq!(source.get("nested").unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr(),pointer);
            assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::json!({"nested":{"value":row["text"]}}));
            let mut owner=ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap();
            let mut copied=0;let mut released=0;let mut born=0;
            for turn in 0..100000 {
                if owner.terminal_is_empty(){break;}
                let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:(copy.as_u64().unwrap() as usize).max(1),maximum_capacity_bytes:owner.next_capacity_byte_demand((copy.as_u64().unwrap() as usize).max(1)).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
                let (inert,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(inert.progress().copied_items,0);assert_eq!((a,r),(0,0));
                if grant.maximum_capacity_bytes!=0 {let (denied,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
                if grant.maximum_release_bytes!=0 && owner.next_copy_byte_demand().unwrap()==0 {let (denied,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
                let (step,a,r)=observe_ownership(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!(progress.retained_capacity_bytes,a);assert_eq!(progress.released_bytes,r);assert!(progress.copied_items<=1);assert!(progress.copied_bytes<=grant.maximum_copy_bytes);assert!(progress.retained_capacity_bytes<=grant.maximum_capacity_bytes);assert!(progress.released_bytes<=grant.maximum_release_bytes);copied+=progress.copied_bytes;released+=progress.released_bytes;born+=progress.retained_capacity_bytes;
                assert!(progress.copied_items!=0 || matches!(step,RetainedCloneStep::Complete(_)),"exact Neural owner grant blocked on turn {turn}");
            }
            assert!(owner.terminal_is_empty());assert_eq!(copied,"nestedvalue".len()+row["text"].as_str().unwrap().len());assert_eq!(released,original_born-original_freed+born);
            eprintln!("[DEBUG] Recursive Neural original-pointer typed owner copied={copied} born={born} physical={released}");
        }
    }
}

#[test]
fn finite_schema_operator_fields_preserve_typed_work_and_physical_custody() {
    use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::RetainedCloneGrant};
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️finite-fields/🔣️.json")).unwrap();
    fn close<T:RetireOwned>(value:T,original:usize,copy:usize,expected:usize) {
        let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut births=0;let mut released=0;let mut copied=0;
        for turn in 0..1000000 {
            if owner.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let (zero,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress().copied_items,0);assert_eq!((a,r),(0,0));
            let (step,a,r)=observe_ownership(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_items<=1);assert!(progress.copied_bytes<=copy);assert!(a<=grant.maximum_capacity_bytes);assert!(r<=grant.maximum_release_bytes);births+=a;released+=r;copied+=progress.copied_bytes;assert!(progress.copied_items!=0,"exact finite Neural field grant blocked on turn {turn}");
        }
        assert!(owner.terminal_is_empty());assert_eq!(copied,expected);assert_eq!(released,original+births);
        eprintln!("[DEBUG] Actual Neural finite fields copy={copy} work={copied} original={original} scaffolds={births} physical={released}");
    }
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let (schema,a,r)=observe_ownership(||serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());assert_eq!(serde_json::to_value(&schema).unwrap(),fixture["schema"]);close(schema,a-r,copy.as_u64().unwrap() as usize,fixture["expected"]["schemaStringBytes"].as_u64().unwrap() as usize);
        let (operator,a,r)=observe_ownership(||serde_json::from_value::<OperatorInfo>(fixture["operator"].clone()).unwrap());assert_eq!(serde_json::to_value(&operator).unwrap(),fixture["operator"]);close(operator,a-r,copy.as_u64().unwrap() as usize,fixture["expected"]["operatorStringBytes"].as_u64().unwrap() as usize);
    }
}

fn original_grant(owner:&ValueRetirement,copy:usize)->RetainedCloneGrant {
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}
}
fn reserve_external(owner:&mut ValueRetirement) {
    for _ in 0..8 {
        let bytes=owner.next_reserve_capacity_byte_demand().unwrap();if bytes==0{return;}
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};
        let (progress,born,freed)=super::super::registry::tests::observe_ownership(||owner.reserve_step(grant).unwrap());
        assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,freed));
    }
    panic!("original retirement queue reservation did not finish");
}
fn admit_external<T:RetireOwned>(owner:&mut ValueRetirement,value:T) {
    reserve_external(owner);
    owner.push_owned(value,RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),maximum_release_bytes:0,maximum_depth:1}).map_err(|(error,_)|error).unwrap();
}
#[test]
fn original_typed_owner_demands_propagate_without_false_fixed_page_progress() {
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📏️owned-demand/🔣️.json")).unwrap();
    let mut source=String::with_capacity(fixture["reservedCapacity"].as_u64().unwrap()as usize);source.push_str(fixture["text"].as_str().unwrap());
    assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&source).unwrap()).unwrap(),source);
    let pointer=source.as_ptr();let mut owner=ValueRetirement::default();
    let birth=semio_framework_value::retirement::owned_retirement_birth_bytes::<String>();
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:birth,maximum_release_bytes:0,maximum_depth:1};
    let (unreserved,born,freed)=observe_ownership(||owner.push_owned(source,grant));assert_eq!((born,freed),(0,0));let (error,original)=unreserved.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(original.as_ptr(),pointer);source=original;
    reserve_external(&mut owner);
    for refusal in fixture["birthRefusals"].as_array().unwrap() {
        let denied=match refusal.as_str().unwrap() {"zeroItems"=>RetainedCloneGrant {maximum_items:0,..grant},"zeroDepth"=>RetainedCloneGrant {maximum_depth:0,..grant},"shortCapacity"=>RetainedCloneGrant {maximum_capacity_bytes:birth-1,..grant},_=>unreachable!()};
        let (refused,born,freed)=observe_ownership(||owner.push_owned(source,denied));assert_eq!((born,freed),(0,0));
        let (error,original)=refused.unwrap_err();assert_ne!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(original.as_ptr(),pointer);source=original;
    }
    let (progress,born,freed)=observe_ownership(||owner.push_owned(source,grant).map_err(|(error,_)|error).unwrap());
    assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,freed));assert_eq!(born,birth);
    let released=close(owner,fixture["logicalPage"].as_u64().unwrap()as usize);
    assert!(released>=fixture["reservedCapacity"].as_u64().unwrap()as usize);
    eprintln!("[DEBUG] Original Neural typed owner full independent grants physical release={released}");
}

/// 🧭️ The original Neural source handoff retains its pointer without an unadmitted frontier node.
#[test]
fn original_neural_value_frontier_stages_without_hidden_birth() {
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎮️typed-owners/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let (value,source_births,source_frees)=observe_ownership(||{let mut text=String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize);text.push_str(row["text"].as_str().unwrap());Value::Atom(Atom::String(text))});
        assert_eq!(source_frees,0);
        let pointer=value.as_atom().unwrap().as_str().unwrap().as_ptr();
        let (owner,births,frees)=observe_ownership(||ValueRetirement::from_value(value));
        let original_pointer=owner.original_value().unwrap().as_atom().unwrap().as_str().unwrap().as_ptr();
        assert_eq!(original_pointer,pointer);
        retire_value_cold(owner);
        eprintln!("[DEBUG] original Neural staged source={} originalPointer=true hiddenBirth={births} hiddenFree={frees}",source_births);
        assert_eq!((births,frees),(fixture["frontier"]["sourceBirthBytes"].as_u64().unwrap() as usize,fixture["frontier"]["sourceReleaseBytes"].as_u64().unwrap() as usize),"source custody must remain inline before an admitted retirement step");
    }
}

//#region 🔣️FixtureLaws
fn exact_backing(source:Owner,allocated:usize) {
    assert!(allocated>4096);
    let mut owner=ValueRetirement::from_owner(source);let mut refused=false;
    for _ in 0..1000 {
        let grant=original_grant(&owner,1);
        if grant.maximum_release_bytes>=allocated {
            let (step,born,freed)=super::super::registry::tests::observe_ownership(||owner.close_step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());
            assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((born,freed),(0,0));refused=true;break;
        }
        owner.close_step(grant).unwrap();
    }
    assert!(refused,"original backing must expose a whole physical release");
    assert!(close(owner,1)>=allocated);
}
#[test]
fn neural_physical_retirement_charges_byte_buffers_down_and_frees_the_whole_reservation() {
    let mut bytes=Vec::with_capacity(8193);bytes.extend(std::iter::repeat_n(1u8,8000));let capacity=bytes.capacity();
    assert!(close(ValueRetirement::from_owner(Owner::Bytes(bytes)),1)>=capacity);
    let text=String::with_capacity(8193);let capacity=text.capacity();
    assert!(close(ValueRetirement::from_value(Value::Atom(Atom::String(text))),1)>=capacity);
}
#[test]
fn neural_physical_retirement_direct_vector_backings_require_whole_release_grant() {
    let strings=Vec::<String>::with_capacity(8193usize.div_ceil(size_of::<String>()));let capacity=strings.capacity()*size_of::<String>();exact_backing(Owner::Strings(strings),capacity);
    let fields=Vec::<FieldSpec>::with_capacity(8193usize.div_ceil(size_of::<FieldSpec>()));let capacity=fields.capacity()*size_of::<FieldSpec>();exact_backing(Owner::Fields(fields),capacity);
}
#[test]
fn neural_physical_retirement_admits_the_current_nested_ordered_key_demand() {
    use super::super::registry::tests::observe_ownership;
    let (released,born,freed)=observe_ownership(||{let mut key=String::with_capacity(8193);key.push_str("nested-key");close(ValueRetirement::from_dictionary(Dictionary::new().insert(key,Value::null())),1)});
    assert_eq!(born,freed);assert!(released>=8193);eprintln!("[DEBUG] Neural nested key original and frontier physical allocations={freed}");
}
fn close(mut owner:ValueRetirement,copy:usize)->usize {
    use super::super::registry::tests::observe_ownership;
    let mut released=0;
    for turn in 0..1000000 {
        if owner.terminal_is_empty(){return released;}
        let grant=original_grant(&owner,copy.max(1));
        let (zero,born,freed)=observe_ownership(||owner.close_step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((born,freed),(0,0));
        if grant.maximum_capacity_bytes>0 {
            let (short,born,freed)=observe_ownership(||owner.close_step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(short.progress(),RetainedCloneProgress::default());assert_eq!((born,freed),(0,0));
        }
        let (step,born,freed)=observe_ownership(||owner.close_step(grant).unwrap());let progress=step.progress();
        assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,freed));released+=freed;
        assert!(progress.copied_items>0||matches!(step,RetainedCloneStep::Complete(_)),"exact original Neural grant blocked at turn {turn}");
    }
    panic!("original Neural owner did not finish");
}

fn payload_bytes(value:&serde_json::Value)->usize {
    match value {serde_json::Value::String(value)=>value.len(),serde_json::Value::Object(values)=>values.iter().map(|(key,value)|key.len()+payload_bytes(value)).sum(),serde_json::Value::Array(values)=>values.iter().map(payload_bytes).sum(),_=>0}
}

#[test]
fn nested_fixture_preserves_payload_and_closes_all_actual_allocations() {
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️value-retirement.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text=row["expandedText"]["text"].as_str().unwrap().repeat(row["expandedText"]["repetitions"].as_u64().unwrap()as usize);
        let json=row["json"].as_str().unwrap().replace("$text",&text);let oracle:serde_json::Value=serde_json::from_str(&json).unwrap();
        assert_eq!(payload_bytes(&oracle),row["expectedBytes"].as_u64().unwrap()as usize);
        for page in [1,64,4096] {
            let (_,born,freed)=observe_ownership(|| {let dictionary:Dictionary=serde_json::from_str(&json).unwrap();assert_eq!(serde_json::to_value(&dictionary).unwrap(),oracle);close(dictionary.into_retirement(),page)});
            assert_eq!(born,freed,"original and frontier allocations must all close at page {page}");
            println!("[DEBUG] Neural fixture {} page={page} payload oracle preserved, allocator closed={freed}",row["id"]);
        }
    }
}

#[test]
fn immutable_dictionary_aliases_release_without_copying_or_retiring_payloads() {
    use super::super::registry::tests::observe_ownership;
    let (_,born,freed)=observe_ownership(|| {
        let dictionary=Dictionary::new().insert("label",Value::Atom(Atom::String("🌊".repeat(4096))));let alias=dictionary.clone();
        let pointer=dictionary.get("label").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr();
        assert_eq!(alias.get("label").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr(),pointer);
        close(alias.into_retirement(),1);
        assert_eq!(dictionary.get("label").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr(),pointer);
        assert_eq!(serde_json::to_value(&dictionary).unwrap()["label"].as_str().unwrap(),"🌊".repeat(4096));
        close(dictionary.into_retirement(),1)
    });assert_eq!(born,freed);
}

#[test]
fn cold_builder_closes_nested_replacements_and_decoder_partial_errors() {
    let mut builder = ColdDictionaryBuilder::new();
    builder.insert("value".into(), Value::Dictionary(Dictionary::new().insert("old", Value::Atom(Atom::String("discarded".into())))));
    builder.insert("value".into(), Value::Dictionary(Dictionary::new().insert("new", Value::Atom(Atom::String("kept".into())))));
    assert_eq!(serde_json::to_string(builder.dictionary()).unwrap(), r#"{"value":{"new":"kept"}}"#);
    drop(builder);
    assert!(serde_json::from_str::<Dictionary>(r#"{"ok":{"long":"value"},"bad":[]}"#).is_err());
    drop(ColdValueOwner::new(Value::Dictionary(Dictionary::new().insert("owned", Value::null()))));
}

#[test]
fn strict_dictionary_and_retirement_drop_guards_reject_live_final_owners() {
    for retirement in [false, true] {
        let guarded = std::panic::catch_unwind(|| {
            let dictionary = Dictionary::new().insert("nested", Value::Dictionary(Dictionary::new().insert("label", Value::Atom(Atom::String("guarded".into())))));
            if retirement { drop(dictionary.into_retirement()); } else { drop(dictionary); }
        });
        assert!(guarded.is_err());
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn partial_nested_retirement_transfers_workers_without_allocation_loss() {
    use super::super::registry::tests::observe_ownership;
    let ((owner,initial_charge),born,freed)=observe_ownership(|| {
        let dictionary=Dictionary::new().insert("label",Value::Atom(Atom::String("高".repeat(6000))));
        assert_eq!(serde_json::to_value(&dictionary).unwrap()["label"].as_str().unwrap().len()+"label".len(),18005);
        let mut owner=dictionary.into_retirement();let mut charge=0;
        for _ in 0..23 {let grant=original_grant(&owner,1);let step=owner.close_step(grant).unwrap();assert!(step.progress().fits(grant));charge+=step.progress().released_bytes;if matches!(step,RetainedCloneStep::Complete(_)){break;}}
        (owner,charge)
    });
    let (remainder,worker_born,worker_freed)=std::thread::spawn(move ||observe_ownership(||close(owner,64))).join().unwrap();
    assert_eq!(born+worker_born,freed+worker_freed);assert!(initial_charge+remainder>=18005);
    println!("[DEBUG] Neural original worker ownership closed allocations={}",freed+worker_freed);
}
//#endregion 🔣️FixtureLaws

//#region 🧠️CacheRetirement
fn cache_grant(owner:&super::super::NeuralCacheRetirement,copy:usize)->RetainedCloneGrant {
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}
}
#[test]
fn cache_retirement_preserves_shared_roots_and_drains_replaced_nested_values() {
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🗃️cache-retirement/🔣️.json")).unwrap();
    let seeds:Vec<_>=fixture["operations"].as_array().unwrap().iter().filter(|value|value["op"]=="seed").collect();
    let payload:usize=seeds.iter().map(|seed|seed["field"].as_str().unwrap().len()+seed["text"].as_str().unwrap().len()*seed["repeat"].as_u64().unwrap()as usize).sum();assert_eq!(payload,fixture["expected"]["finalReleasedBytes"].as_u64().unwrap()as usize);
    for copy in [1,64,4096] {
        let (_,born,freed)=observe_ownership(|| {
            let cache=Arc::new(super::super::NeuralCache::new());
            for seed in &seeds {cache.seed(fixture["key"].as_u64().unwrap(),Dictionary::new().insert(seed["field"].as_str().unwrap(),Value::Atom(Atom::String(seed["text"].as_str().unwrap().repeat(seed["repeat"].as_u64().unwrap()as usize))))).unwrap();}
            let mut shared=super::super::NeuralCacheRetirement::new(Arc::clone(&cache));
            for _ in 0..100 {if shared.terminal_is_empty(){break;}let grant=cache_grant(&shared,copy);let (step,a,r)=observe_ownership(||shared.close_step(grant).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));assert_eq!(r,0);}
            assert!(shared.terminal_is_empty());assert_eq!(cache.len(),1);
            let mut owner=super::super::NeuralCacheRetirement::new(cache);
            for _ in 0..1000000 {
                if owner.terminal_is_empty(){break;}let grant=cache_grant(&owner,copy);
                let (zero,a,r)=observe_ownership(||owner.close_step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((a,r),(0,0));
                let (step,a,r)=observe_ownership(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));
            }
            assert!(owner.terminal_is_empty());
        });assert_eq!(born,freed);eprintln!("[DEBUG] Original Neural cache copy={copy} alias/replacement/final allocations closed={freed}");
    }
}
#[test]
fn cache_live_final_drop_is_guarded_instead_of_recursively_destroying_dictionaries() {
    assert!(std::panic::catch_unwind(||{let cache=super::super::NeuralCache::new();cache.seed(1,Dictionary::new().insert("label",Value::Atom(Atom::String("guarded".into())))).unwrap();drop(cache);}).is_err());
}
//#endregion 🧠️CacheRetirement

//#region 📸️EvaluationRetirement
#[test]
fn evaluation_snapshot_and_channel_owners_preserve_payload_and_close_all_allocations() {
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️evaluation-owners/🔣️.json")).unwrap();
    let node=fixture["node"]["text"].as_str().unwrap().repeat(fixture["node"]["repeat"].as_u64().unwrap()as usize);
    let payload=fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap()as usize);
    assert_eq!(node.len()+2*payload.len()+"seednodelabel".len(),fixture["expectedBytes"].as_u64().unwrap()as usize);
    for page in [1,64,4096] {
        let (_,born,freed)=observe_ownership(|| {
            let snapshot=TreeSnapshot {tree:Arc::new(super::super::super::Tree {neurons:vec![super::super::super::Neuron{id:node.clone(),kind:"input".into(),params:Dictionary::new(),tree:None}],synapses:vec![super::super::super::Synapse{id:"seed".into(),from:String::new(),to:payload.clone(),from_port:String::new(),to_port:String::new()}]}),seeds:Arc::new(HistoryFoldIndex::new())};
            let channels=EvalChannels {outputs:HistoryFoldIndex::from([("node".into(),Dictionary::new().insert("label",Value::Atom(Atom::String(payload.clone()))))]),inputs:HistoryFoldIndex::new()};
            assert_eq!(serde_json::to_value(channels.outputs.get("node").unwrap()).unwrap(),serde_json::json!({"label":payload}));
            close(ValueRetirement::from_snapshot(snapshot),page);close(ValueRetirement::from_channels(channels),page)
        });assert_eq!(born,freed);println!("[DEBUG] Neural evaluation page={page} UTF8 payload preserved, allocator closed={freed}");
    }
}
//#endregion 📸️EvaluationRetirement

#[test]
fn body_dependent_birth_uses_the_same_borrowed_body_and_finite_progress() {
    use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep as TypedStep};
    struct BodyOwner {overhead:usize}
    struct BodyCursor {overhead:usize,storage:Option<Vec<u8>>,finished:bool}
    impl RetireOwned for BodyOwner {
        fn retirement(self)->Box<dyn RetirementCursor> {Box::new(BodyCursor {overhead:self.overhead,storage:None,finished:false})}
        fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<BodyCursor>())}
        fn controlled_retirement_supported()->bool {true}
    }
    impl RetirementCursor for BodyCursor {
        fn close_step(&mut self,grant:RetainedCloneGrant)->TypedStep {
            if grant.maximum_items==0 {return TypedStep::BudgetExhausted;}
            if self.finished {return TypedStep::Complete;}
            if self.storage.is_none() {
                let bytes=self.overhead+grant.maximum_copy_bytes;
                if grant.maximum_copy_bytes==0 || bytes>grant.maximum_capacity_bytes {return TypedStep::BudgetExhausted;}
                let storage=Vec::with_capacity(bytes);assert_eq!(storage.capacity(),bytes);self.storage=Some(storage);
                TypedStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:1,retained_capacity_bytes:bytes,released_bytes:0})
            } else {
                let bytes=self.storage.as_ref().unwrap().capacity();
                if bytes>grant.maximum_release_bytes {return TypedStep::BudgetExhausted;}
                self.storage=None;self.finished=true;TypedStep::Bytes(bytes)
            }
        }
        fn terminal_is_empty(&self)->bool {self.finished&&self.storage.is_none()}
        fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(usize::from(self.storage.is_none()&&!self.finished))}
        fn next_birth_bytes(&self,body:usize)->Option<usize> {Some(if self.storage.is_none()&&!self.finished {self.overhead+body}else {0})}
        fn next_close_byte_demand(&self)->Option<usize> {Some(self.storage.as_ref().map_or(0,Vec::capacity))}
        fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📏️owned-demand/🔣️.json")).unwrap();let law=&fixture["bodyDependentBirth"];
    let mut owner=ValueRetirement::default();
    let birth=semio_framework_value::retirement::owned_retirement_birth_bytes::<BodyOwner>();
    admit_external(&mut owner,BodyOwner {overhead:law["overhead"].as_u64().unwrap()as usize});
    let mut turns=0;let mut released=0;
    while !owner.terminal_is_empty() {turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let grant=original_grant(&owner,law["logicalPage"].as_u64().unwrap()as usize);let (step,born,freed)=super::super::registry::tests::observe_ownership(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));released+=freed;}
    assert!(released>birth);println!("[DEBUG] Neural body-dependent allocation closed with exact admission, turns={turns}, released={released}");
}

#[test]
fn retained_entry_storage_preserves_original_vector_allocation_and_order() {
    use super::super::registry::tests::observe_ownership;
    let mut entries=Vec::with_capacity(8193);entries.push((String::from("first"),protocol::value::DslValue::String(String::from("雪"))));entries.push((String::from("second"),protocol::value::DslValue::Null));
    let pointer=entries.as_ptr();let capacity=entries.capacity();
    let (mut entries,born,freed)=observe_ownership(||VecDeque::from(entries));
    assert_eq!((born,freed),(0,0));assert_eq!(entries.as_slices().0.as_ptr(),pointer);assert_eq!(entries.capacity(),capacity);
    let first=entries.pop_front().unwrap();let second=entries.pop_front().unwrap();assert_eq!(first.0,"first");assert_eq!(second.0,"second");
    close(ValueRetirement::from_owner(Owner::Input(protocol::value::DslValue::Object(vec![first,second]))),3);
    let mut owner=ValueRetirement::default();admit_external(&mut owner,entries);let released=close(owner,3);
    assert!(released>=capacity*size_of::<(String,protocol::value::DslValue)>());eprintln!("[DEBUG] Neural original entry storage kept its backing and front order, physical release={released}");
}
