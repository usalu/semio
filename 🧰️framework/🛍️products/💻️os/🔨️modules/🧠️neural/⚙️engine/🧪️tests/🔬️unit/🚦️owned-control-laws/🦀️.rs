/// 🚦️ Owned control laws: the compact pending extension and the dictionary writer source retirement and original text laws.

#[test]
fn compact_pending_extension_uses_the_existing_progress_authority_before_serialization() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️owned-controls.json")).unwrap();let law=&fixture["compactPending"];let label=law["textUnit"].as_str().unwrap().repeat(law["textRepeats"].as_u64().unwrap()as usize);let mut tree=wave_tree();let left=tree.neurons.iter_mut().find(|neuron|neuron.id=="left").unwrap();left.params=std::mem::take(&mut left.params).insert("label",Value::Atom(Atom::String(label.clone())));let registry=wave_registry();let evaluator=Evaluator::new(&registry);let cache=NeuralCache::new();
    let mut dispatch=|kind:&str,input:&Dictionary|->Result<Dictionary,EvalError>{let Some(operator_id)=kind.strip_prefix("plugin.")else{return registry.dispatch(kind,input)};Err(EvalError::PendingExtension {extension_id:"geometry".into(),operator_id:operator_id.into(),node_hash:node_hash(kind,input)})};
    let initial=evaluator.evaluate_channels_budgeted(&tree,&HashMap::new(),&HistoryFoldIndex::new(),&mut dispatch,&cache,&HashSet::new(),None,EvalStepBudget::UNBOUNDED,&|_|true).unwrap();let left=initial.pending_extensions.iter().find(|pending|pending.neuron_id=="left").unwrap();let previous_hash=left.node_hash;let oracle:serde_json::Value=serde_json::from_str(&left.input_json).unwrap();assert_eq!(oracle["label"].as_str(),Some(label.as_str()));initial.retire_cold();
    let calls=std::cell::Cell::new(0);let resumed=evaluator.evaluate_channels_budgeted(&tree,&HashMap::new(),&HistoryFoldIndex::new(),&mut dispatch,&cache,&HashSet::new(),None,EvalStepBudget::UNBOUNDED,&|hash|{calls.set(calls.get()+1);hash!=previous_hash}).unwrap();let left=resumed.pending_extensions.iter().find(|pending|pending.neuron_id=="left").unwrap();assert_eq!(left.node_hash,previous_hash);assert!(left.input_json.is_empty());assert!(resumed.pending_extensions.iter().find(|pending|pending.neuron_id=="right").unwrap().input_json.len()>0);assert_eq!(calls.get(),2);resumed.retire_cold();
    let left=tree.neurons.iter_mut().find(|neuron|neuron.id=="left").unwrap();left.params=std::mem::take(&mut left.params).insert("label",Value::Atom(Atom::String(format!("{label} changed"))));let changed=evaluator.evaluate_channels_budgeted(&tree,&HashMap::new(),&HistoryFoldIndex::new(),&mut dispatch,&cache,&HashSet::new(),None,EvalStepBudget::UNBOUNDED,&|hash|hash!=previous_hash).unwrap();let left=changed.pending_extensions.iter().find(|pending|pending.neuron_id=="left").unwrap();assert_ne!(left.node_hash,previous_hash);assert!(!left.input_json.is_empty());changed.retire_cold();cache.retire_cold();tree.retire_cold();registry.retire_cold();
}

#[test]
fn dictionary_writer_source_retirement_uses_the_existing_exact_domain_authority() {
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️owned-controls.json")).unwrap();let text=fixture["dictionaryText"].as_str().unwrap();let law=&fixture["writerSourceRetirement"];let units=law["maximumUnits"].as_u64().unwrap()as usize;let bytes=law["grantBytes"].as_u64().unwrap()as usize;
    let oracle:serde_json::Value=serde_json::from_str(text).unwrap();let source:Dictionary=semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&source)).unwrap(),oracle);
    let mut retained=ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap();let mut released=0;let mut turns=0;
    while !retained.terminal_is_empty() {
        turns+=1;assert!(turns<10000);
        let grant=RetainedCloneGrant {maximum_items:units,maximum_copy_bytes:bytes,maximum_capacity_bytes:retained.next_capacity_byte_demand(bytes).unwrap(),maximum_release_bytes:retained.next_release_byte_demand().unwrap(),maximum_depth:retained.next_depth_demand().unwrap()};
        assert_eq!(retained.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap().progress().copied_items,0);
        let progress=retained.step(grant).unwrap().progress();assert!(progress.copied_items<=units);assert!(progress.copied_bytes<=bytes);assert!(progress.released_bytes<=grant.maximum_release_bytes);released+=progress.released_bytes;
    }
    assert!(released>0);
}

fn original_writer_grant(writer:&semio_framework_pack_json::JsonWriteCursor<Dictionary>,items:usize)->RetainedCloneGrant {let demand=writer.normal_step_demands().unwrap();RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:3.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

fn close_original_writer(writer:semio_framework_pack_json::JsonWriteCursor<Dictionary>) {
    let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(writer).map_err(|(error,_)|error).unwrap();
    for _ in 0..250000 {
        if owner.terminal_is_empty(){return;}
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:owner.next_capacity_byte_demand(3).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let (step,born,freed)=semio_framework_os_kernel_neural_engine_model::testkit::observe_ownership(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));
    }
    panic!("original writer allocations did not close");
}

#[test]
fn dictionary_writer_source_preserves_original_text_and_independent_json_bytes() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️owned-controls.json")).unwrap();let text=fixture["compactPending"]["textUnit"].as_str().unwrap().repeat(fixture["compactPending"]["textRepeats"].as_u64().unwrap()as usize);let oracle=serde_json::json!({"a":null,"b":true,"c":-42,"d":1.25,"z":{"text":text}});let input=serde_json::to_string(&oracle).unwrap();
    let source:Dictionary=semio_framework_pack_json::from_json_str(&input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let pointer=source.get("z").unwrap().as_dictionary().unwrap().get("text").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr();let mut writer=semio_framework_pack_json::JsonWriteCursor::new(source);let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(4*1024*1024,&mut admitted);let mut turns=0;
    let output=loop{let before=writer.progress();assert!(writer.step(0,&mut control,original_writer_grant(&writer,0)).unwrap().is_none());assert_eq!(before,writer.progress());assert!(writer.take_source().is_none());turns+=1;assert!(turns<250000);if let Some(output)=writer.step(1,&mut control,original_writer_grant(&writer,1)).unwrap(){break output;}};assert_eq!(output,input);assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(),oracle);let original=writer.take_source().unwrap();assert_eq!(original.get("z").unwrap().as_dictionary().unwrap().get("text").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr(),pointer);assert!(writer.take_source().is_none());crate::retirement::retire_value_cold(ValueRetirement::from_dictionary(original));close_original_writer(writer);
    for cutoff in [0,1,64,4096] {let source:Dictionary=semio_framework_pack_json::from_json_str(&input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut writer=semio_framework_pack_json::JsonWriteCursor::new(source);let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(4*1024*1024,&mut admitted);for _ in 0..cutoff{assert!(writer.step(1,&mut control,original_writer_grant(&writer,1)).unwrap().is_none());}close_original_writer(writer);}
}
