//! 🧪️ Guided payload construction borrows an existing arena and preserves inline exact keys.
use semio_framework_ui_contract::{UiMapBuilder,UiListBuilder,UiText,UiValue,UiValueRetirement};
use semio_framework_plugin_decl_views::staged_form::{StagedDslPayload,StagedPayloadCursor};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneStep};
#[test]
fn native_staged_payload_map_limit_refuses_before_collection_birth(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-cases.json")).unwrap();
    let maximum=fixture["payloadMapMembers"].as_u64().unwrap()as usize;let refused=fixture["payloadMapRefusedMembers"].as_u64().unwrap()as usize;
    assert_eq!(maximum,semio_framework_plugin_decl_views::staged_form::STAGED_PAYLOAD_MAP_MEMBERS);
    let source=DslValue::Object((0..refused).map(|index|(index.to_string(),DslValue::Null)).collect());let mut cursor=StagedPayloadCursor::default();let mut output=None;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:32768,maximum_depth:64};
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&StagedDslPayload{source:&source},&mut output,grant));
    assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(output.is_none());assert!(cursor.terminal_is_empty());
}
fn close(value:UiValue){let mut value=UiValueRetirement::new(value);for _ in 0..16384{if value.terminal_is_empty(){break}value.close_step(1,32768).unwrap();}assert!(value.terminal_is_empty());}
fn close_control(cursor:&mut semio_framework_plugin_decl_views::staged_form::StagedScalarCursor,grant:RetainedCloneGrant){for _ in 0..16384{let step=cursor.close(grant).unwrap();if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());return}}panic!("scalar owner did not retire");}
#[test]
fn native_body_split_caps_match_shared_schema_and_typed_page_classification(){
    use semio_framework_plugin_decl_views::staged_form::*;
    use semio_framework::action_staging::*;
    let limits:StagedFormBodyLimits=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-limits.json")).unwrap();assert_eq!(limits,STAGED_FORM_BODY_LIMITS);assert_eq!(limits.content+limits.pages,limits.controls);
    struct Source{status:StagedFormStatus,page:StagedFormPageBinding,content:usize,pages:usize,fake:bool}
    impl StagedBodySource for Source{fn identity(&self)->usize{self as*const _ as usize}fn title(&self)->&str{"Page"}fn status(&self)->&StagedFormStatus{&self.status}fn controls(&self)->usize{self.content+self.pages}fn control(&self,index:usize)->Result<StagedBodyControl<'_>,semio_framework_value::ValueError>{Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:index as u64,controller:"ifc",label:"Field",description:None,disabled:false,kind:if index>=self.content||self.fake{StagedScalarKind::Button{page:true}}else{StagedScalarKind::ReadOnly{value:"x"}}},required:false,error:None,binding:if index>=self.content{Some(StagedBodyBinding::Page(&self.page))}else{None}})}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-composition-cases.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){let token=StagedFormToken{action:"applyIfcTemplate".into(),generation:"1".into(),revision:"r1".into(),window:"w1".into()};let source=Source{status:StagedFormStatus{token:token.clone(),ready:false,diagnostics:Vec::new(),diagnostic_offset:"0".into(),diagnostic_total:"0".into()},page:StagedFormPageBinding{token,page:StagedFormPage{field_offset:"16".into(),diagnostic_offset:"0".into(),choice:None}},content:case["content"].as_u64().unwrap()as usize,pages:case["pages"].as_u64().unwrap()as usize,fake:false};assert_eq!(staged_body_composition(&source).is_ok(),case["allowed"].as_bool().unwrap());let forged=Source{fake:true,..source};assert_eq!(staged_body_composition(&forged).is_ok(),case["allowed"].as_bool().unwrap());}
}
#[test]
fn native_typed_staged_bindings_match_closed_neutral_wire_without_intermediate_clones(){
    use semio_framework_plugin_decl_views::staged_form::*;
    use semio_framework::action_staging::{StagedFormBinding,StagedFormPageBinding};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/binding-cases.json")).unwrap();
    let pages:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/page-cases.json")).unwrap();
    let mut warm=UiMapBuilder::try_new().unwrap();warm.try_insert("warm".into(),UiValue::Null).unwrap();close(UiValue::Map(warm.finish()));
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:32768,maximum_depth:64};
    for row in fixture["cases"].as_array().unwrap(){
        let expected=serde_json::json!({"token":pages["token"],"layout":if row["edit"]["source"]=="text"{"preserve"}else{"replace"},"edits":[row["edit"]]});let binding:StagedFormBinding=serde_json::from_value(expected.clone()).unwrap();
        let source=StagedBindingPayload::Action(&binding);let mut cursor=StagedPayloadCursor::default();let mut output=None;for _ in 0..1024{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut output,grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if matches!(result.unwrap(),RetainedCloneStep::Complete(_)){break}}let output=output.unwrap();assert_eq!(serde_json::to_value(&output).unwrap(),expected);close(output);assert!(cursor.terminal_is_empty());
    }
    for page in pages["valid"].as_array().unwrap(){let expected=serde_json::json!({"token":pages["token"],"page":page});let binding:StagedFormPageBinding=serde_json::from_value(expected.clone()).unwrap();let source=StagedBindingPayload::Page(&binding);let mut cursor=StagedPayloadCursor::default();let mut output=None;for _ in 0..1024{if matches!(cursor.step(&source,&mut output,grant).unwrap(),RetainedCloneStep::Complete(_)){break}}let output=output.unwrap();assert_eq!(serde_json::to_value(&output).unwrap(),expected);close(output);assert!(cursor.terminal_is_empty());}
}
#[test]
fn native_typed_choice_key_capacity_is_resource_refusal_with_retained_partial_owner(){
    use semio_framework_plugin_decl_views::staged_form::*;
    use semio_framework::action_staging::StagedFormBinding;
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-cases.json")).unwrap();assert_eq!(cases["oversizedChoiceKey"]["length"],"uiTextCapacity+1");let refusal=cases["oversizedChoiceKey"]["refusal"].as_str().unwrap();
    let token=serde_json::json!({"action":"applyIfcTemplate","generation":"1","revision":"r1","window":"w1"});let key="k".repeat(UiText::default().capacity()+1);
    let expected=serde_json::json!({"token":token,"layout":"replace","edits":[{"source":"choice","kind":"set","path":["choice"],"options":[{"key":key,"value":"exact source"}]}]});let binding:StagedFormBinding=serde_json::from_value(expected).unwrap();
    let mut warm=UiMapBuilder::try_new().unwrap();warm.try_insert("warm".into(),UiValue::Null).unwrap();close(UiValue::Map(warm.finish()));
    let source=StagedBindingPayload::Action(&binding);let mut cursor=StagedPayloadCursor::default();let mut output=None;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:32768,maximum_depth:64};let mut refused=false;
    for _ in 0..1024{match cursor.step(&source,&mut output,grant){Err(error)=>{assert!(error.to_string().contains(refusal));refused=true;break},Ok(step)=>assert!(!matches!(step,RetainedCloneStep::Complete(_)))}}
    assert!(refused&&output.is_none());for _ in 0..16384{let step=cursor.close(grant).unwrap();if matches!(step,RetainedCloneStep::Complete(_)){break}}assert!(cursor.terminal_is_empty());
}
#[test]
fn native_staged_body_actual_original_receiving_keeps_resident_source_and_paid_cancelled_trees(){
    use semio_framework_plugin_decl_views::staged_form::*;
    use semio_framework_ui_contract::{Component,ContainerRole,OriginalBuiltTreeRetirement};
    use semio_framework::action_staging::{StagedFormStatus,StagedFormToken};
    struct Source{status:StagedFormStatus,binding:semio_framework::action_staging::StagedFormBinding}
    impl StagedBodySource for Source{
        fn identity(&self)->usize{self as*const _ as usize}fn title(&self)->&str{"Vorlage"}fn status(&self)->&StagedFormStatus{&self.status}fn controls(&self)->usize{1}
        fn control(&self,index:usize)->Result<StagedBodyControl<'_>,semio_framework_value::ValueError>{assert_eq!(index,0);Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:7,controller:"ifc",label:"Wert",description:Some("Exakter Wert"),disabled:false,kind:StagedScalarKind::Text{value:"-"}},required:true,error:Some("Ungültiger Wert"),binding:Some(StagedBodyBinding::Action(&self.binding))})}
    }
    semio_framework_ui_contract::built_node_pages_are_terminal_empty();let mut warm=UiMapBuilder::try_new().unwrap();warm.try_insert("warm".into(),UiValue::Null).unwrap();close(UiValue::Map(warm.finish()));
    for cut in [0,1,2,8,32,64,128,256]{
        let token=StagedFormToken{action:"applyIfcTemplate".into(),generation:"7".into(),revision:"a".repeat(64),window:"real-window".into()};
        let binding=serde_json::json!({"token":token,"layout":"preserve","edits":[{"source":"text","path":["entries","0","value"]}]});
        let source=Source{status:StagedFormStatus{token,ready:false,diagnostics:Vec::new(),diagnostic_offset:"0".into(),diagnostic_total:"0".into()},binding:serde_json::from_value(binding.clone()).unwrap()};
        let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();let mut cursor=StagedBodyCursor::default();let mut output=None;
        for _ in 0..cut{
            let grant=RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};let before=cx.retained_progress();
            let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_receiving(|native|cursor.step(&source,native,&mut output,grant)));
            let step=result.unwrap();cx.consume_receiving_retained(grant,step.progress()).unwrap();cx.consume_fuel(step.progress().copied_items as u64);let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));assert_eq!(cx.original_recipient_identity(),recipient);if matches!(step,RetainedCloneStep::Complete(_)){break}
        }
        if let Some(node)=output{
            let Component::Container(props)=&node.component else{panic!("form missing")};assert_eq!(props.role,ContainerRole::Form);assert_eq!(node.children.len(),2);
            let mut children=node.children.iter();let Component::Extension(status)=&children.next().unwrap().component else{panic!("status missing")};assert_eq!(status.extension.as_ref(),"framework.stagedFormStatus");assert_eq!(serde_json::to_value(&status.props).unwrap(),serde_json::to_value(&source.status).unwrap());
            let field=children.next().unwrap();let Component::Container(props)=&field.component else{panic!("field missing")};assert_eq!(props.required,Some(true));assert_eq!(props.error.as_ref().unwrap().as_ref(),"Ungültiger Wert");let input=field.children.iter().next().unwrap();assert_eq!(serde_json::to_value(input.bindings.get(0).unwrap().args.as_ref().unwrap()).unwrap(),binding);
            let mut owner=OriginalBuiltTreeRetirement::new(node);for _ in 0..16384{if owner.terminal_is_empty(){break}let demand=owner.next_demand(32768).unwrap();semio_framework_job::close_original_retained_step(&mut cx,demand,|grant|owner.step(grant)).unwrap();}assert!(owner.terminal_is_empty());
        }
        cx.original_cancel_token().cancel_now();
        for _ in 0..16384{if cursor.terminal_is_empty(){break}let demand=cursor.close_demand(32768).unwrap();let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::close_original_retained_step(&mut cx,demand,|grant|cursor.close(grant)));result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));}
        assert!(cursor.terminal_is_empty());drop(cursor);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
    }
}
#[test]
fn native_staged_status_payload_carries_only_exact_bounded_source_metadata(){
    use semio_framework_plugin_decl_views::staged_form::*;
    use semio_framework::action_staging::{StagedFormStatus,StagedFormToken};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-cases.json")).unwrap();
    let mut warm=UiMapBuilder::try_new().unwrap();warm.try_insert("warm".into(),UiValue::Null).unwrap();close(UiValue::Map(warm.finish()));
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:32768,maximum_depth:64};
    for row in fixture["statuses"].as_array().unwrap(){
        let status=StagedFormStatus{token:StagedFormToken{action:"applyIfcTemplate".into(),generation:"18446744073709551615".into(),revision:"0123456789abcdef".repeat(4),window:"real-window".into()},ready:row["ready"].as_bool().unwrap(),diagnostics:Vec::new(),diagnostic_offset:"0".into(),diagnostic_total:"0".into()};
        let source=StagedStatusPayload{status:&status};let mut cursor=StagedPayloadCursor::default();let mut output=None;for _ in 0..256{if matches!(cursor.step(&source,&mut output,grant).unwrap(),RetainedCloneStep::Complete(_)){break}}
        let output=output.unwrap();assert_eq!(serde_json::to_value(&output).unwrap(),serde_json::to_value(&status).unwrap());close(output);assert!(cursor.terminal_is_empty());
    }
}
#[test]
fn native_staged_scalar_controls_keep_exact_resident_bindings_and_cancelled_prefixes(){
    use semio_framework_plugin_decl_views::staged_form::*;
    use semio_framework_ui_contract::{Component,Trigger,OriginalBuiltTreeRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-cases.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:512};
    for row in fixture["cases"].as_array().unwrap(){
        let value=row["value"].as_str().unwrap_or("");let choices=[StagedScalarChoice{key:value,label:"Exact value"}];
        let kind=match row["control"].as_str().unwrap(){"input"=>StagedScalarKind::Text{value},"select"=>StagedScalarKind::Choice{value,choices:&choices},"button"=>StagedScalarKind::Button{page:true},_=>unreachable!()};
        let spec=StagedScalarControl{owner:1,address:17,controller:"ifc",label:if row["locale"]=="de"{"Wert"}else{"Value"},description:Some("Exact source control"),disabled:false,kind};
        for cut in [0,1,2,3,4,5,6,8,16]{
            let mut cursor=StagedScalarCursor::default();let mut args=Some(UiValue::Text(UiText::try_from_str("owned binding").unwrap()));let mut output=None;
            for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{assert_eq!(cursor.step(&spec,&mut args,&mut output,denied).unwrap().progress(),Default::default());assert!(args.is_some());}
            for _ in 0..cut{if matches!(cursor.step(&spec,&mut args,&mut output,grant).unwrap(),RetainedCloneStep::Complete(_)){break}}
            if let Some(node)=output{
                assert_eq!(node.key.as_ref(),"stage.control.17");assert_eq!(node.accessibility.label.as_ref().unwrap().0.as_ref(),spec.label);
                let binding=node.bindings.get(0).unwrap();assert_eq!(binding.action.name.as_ref(),row["event"].as_str().unwrap());assert_eq!(binding.action.scope.as_ref(),"ifc");assert_eq!(serde_json::to_value(binding.args.as_ref().unwrap()).unwrap(),"owned binding");assert!(args.is_none());
                match &node.component{Component::Input(props)=>{assert_eq!(props.value.as_ref(),value);assert_eq!(binding.trigger,Trigger::Change);},Component::Select(props)=>{assert_eq!(props.value.as_ref(),value);assert_eq!(props.items.get(0).unwrap().value.as_ref(),value);assert_eq!(binding.trigger,Trigger::Change);},Component::Button(_)=>assert_eq!(binding.trigger,Trigger::Activate),_=>unreachable!()}
                let mut owner=OriginalBuiltTreeRetirement::new(node);for _ in 0..16384{if owner.terminal_is_empty(){break}owner.step(grant).unwrap();}assert!(owner.terminal_is_empty());
            }
            close_control(&mut cursor,grant);if let Some(args)=args{close(args);}
        }
    }
}
#[test]
fn native_staged_payload_cursor_preserves_nested_source_and_closes_cancelled_prefixes(){
    let row=serde_json::json!({"token":{"generation":"9007199254740993"},"options":[{"key":"x","value":{"bits":"8000000000000000","id":"18446744073709551615"}}]});let source=DslValue::from(&row);let input=StagedDslPayload{source:&source};
    let mut initial=UiMapBuilder::try_new().unwrap();initial.try_insert("warm".into(),UiValue::Null).unwrap();close(UiValue::Map(initial.finish()));
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:32768,maximum_depth:64};
    for cut in [0,1,2,4,8,16,32,64,128]{
        let mut cursor=StagedPayloadCursor::default();let mut output=None;
        assert_eq!(cursor.step(&input,&mut output,RetainedCloneGrant{maximum_copy_bytes:0,..grant}).unwrap().progress(),Default::default());
        for _ in 0..cut{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&input,&mut output,grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if matches!(result.unwrap(),RetainedCloneStep::Complete(_)){break}}
        if let Some(output)=output{assert_eq!(serde_json::to_value(&output).unwrap(),row);close(output);}
        for turn in 0..16384{if cursor.terminal_is_empty(){break}let demand=cursor.close_demand().unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close(grant));let progress=result.unwrap().progress();if progress.copied_items!=0{assert_eq!(progress.copied_bytes,demand.copy_bytes);}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(turn+1<16384);}
        assert!(cursor.terminal_is_empty());
    }
}
#[test]
fn native_staged_collection_sources_keep_exact_payloads_and_charge_no_cold_arena_birth(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/body-cases.json")).unwrap();
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(UiMapBuilder::try_new_initialized);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    if let Ok(Some(builder))=result{close(UiValue::Map(builder.finish_initialized()));}
    let mut initial=UiMapBuilder::try_new().unwrap();initial.try_insert("warm".into(),UiValue::Null).unwrap();close(UiValue::Map(initial.finish()));
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(UiMapBuilder::try_new_initialized);let mut builder=result.unwrap().unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for(key,value)in fixture["collection"].as_object().unwrap(){
        let mut donor=Some((UiText::try_from_str(key).unwrap(),UiValue::Text(UiText::try_from_str(value.as_str().unwrap()).unwrap())));
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||builder.insert_initialized(&mut donor));assert!(result.unwrap());assert!(donor.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    let value=UiValue::Map(builder.finish_initialized());assert_eq!(serde_json::to_value(&value).unwrap(),fixture["collection"]);close(value);
    let mut list=UiListBuilder::try_new_initialized().unwrap().unwrap();let mut donor=Some(UiValue::Text(UiText::try_from_str("18446744073709551615").unwrap()));
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||list.push_initialized(&mut donor));assert!(result.unwrap());assert!(donor.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    close(UiValue::List(list.finish_initialized()));
}
