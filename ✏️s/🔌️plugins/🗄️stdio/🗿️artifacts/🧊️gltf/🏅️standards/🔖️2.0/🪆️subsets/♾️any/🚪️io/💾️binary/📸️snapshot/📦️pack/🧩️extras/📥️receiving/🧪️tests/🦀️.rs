//! 🧪️ GLTF extras rebuild one node per funded turn and equal the serde_json tree they came from, including every cancellation.
use super::*;
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,OwnedFieldReceiving,OriginalReceivingPrimary};
use semio_framework_dsl_record::{DslField,FieldValue};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,retirement::{RetireOwned,controlled::ControlledRetirement}};

fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096}}
fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn close_value<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut value=ControlledRetirement::retain(value);for _ in 0..1_000_000{if value.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=value.selected_demand(loan().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=value.step(grant);RetainedWorkOutcome{result,work:value.last_turn_work()}})?;Ok(())}).unwrap();}panic!("retirement did not settle")}
fn close(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut GltfJsonReceiving){for _ in 0..1_000_000{if cursor.terminal_is_empty(){break}caller.turn(loan(),|cx|{let demand=cursor.close_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let turn=cursor.close_step(grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();}assert!(cursor.terminal_is_empty());for _ in 0..1_000_000{if decoder.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())}).unwrap();}panic!("decoder did not settle")}
fn step(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut GltfJsonReceiving,input:&mut Option<FieldValue>)->Result<(),OriginalTurnFailure>{caller.turn_with_decoder(loan(),decoder,|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|native|{let turn=cursor.step(input,native,32768,issued);work=turn.work;turn.result});let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None})}})}

fn gltf(value:&serde_json::Value)->GltfJson{match value{serde_json::Value::Null=>GltfJson::Null,serde_json::Value::Bool(flag)=>GltfJson::Bool(*flag),serde_json::Value::Number(number)=>GltfJson::Number(number.as_f64().unwrap()),serde_json::Value::String(text)=>GltfJson::String(text.clone()),serde_json::Value::Array(items)=>GltfJson::Array(items.iter().map(gltf).collect()),serde_json::Value::Object(members)=>GltfJson::Object(members.iter().map(|(name,value)|(name.clone(),gltf(value))).collect())}}
fn back(value:&GltfJson)->serde_json::Value{match value{GltfJson::Null=>serde_json::Value::Null,GltfJson::Bool(flag)=>serde_json::Value::Bool(*flag),GltfJson::Number(number)=>serde_json::json!(*number),GltfJson::String(text)=>serde_json::Value::String(text.clone()),GltfJson::Array(items)=>serde_json::Value::Array(items.iter().map(back).collect()),GltfJson::Object(members)=>serde_json::Value::Object(members.iter().map(|(name,value)|(name.clone(),back(value))).collect())}}
fn samples()->Vec<serde_json::Value>{
    let mut deep=serde_json::json!(0.5);for _ in 0..40{deep=serde_json::json!([deep])}
    vec![serde_json::json!(null),serde_json::json!(true),serde_json::json!(1.5),serde_json::json!("s"),serde_json::json!([]),serde_json::json!({}),
        serde_json::json!([1.0,[2.0,[3.0]],{"a":null}]),serde_json::json!({"k":{"x":[true,false],"y":"z"},"n":-0.25}),deep,
        serde_json::Value::Array((0..300).map(|n|serde_json::json!(f64::from(n))).collect())]
}
#[test]
fn gltf_extras_rebuild_per_node_equal_serde_json_and_cancel_at_every_cut(){
    for sample in samples(){
        let expected=gltf(&sample);assert_eq!(back(&expected),sample,"oracle round trip of the sample itself");
        let nodes=Json::from(&expected).nodes.len();
        for cut in [None,Some(0),Some(1),Some(4),Some(nodes+3)]{
            let mut caller=OriginalJobFixtureCaller::admit(OperationId(993),Generation(13),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
            let mut input=Some(Json::from(&expected).to_value());let mut cursor=GltfJson::receiving_cursor();let mut turns=0;
            while !cursor.is_ready()&&cut.is_none_or(|cut|turns<cut){step(&mut caller,&mut decoder,&mut cursor,&mut input).unwrap();turns+=1;assert!(turns<1_000_000);}
            if cut.is_none(){
                assert!(turns>=nodes,"one funded turn per node: {turns} turns for {nodes} nodes");
                let mut output=None;caller.turn(loan(),|cx|{let turn=cursor.take_into(&mut output,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();
                let output=output.unwrap();assert_eq!(output,expected);assert_eq!(back(&output),sample);close_value(&mut caller,output);
            }else{caller.cancel().cancel_now();}
            close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
        }
    }
}
#[test]
fn gltf_extras_refuse_a_topology_that_differs_and_keep_every_child_owned(){
    let expected=gltf(&serde_json::json!([1.0,2.0]));let mut value=Json::from(&expected).to_value();
    if let FieldValue::Record(record)=&mut value{if let Some(FieldValue::List(nodes))=record.fields.get_mut(&0){if let Some(FieldValue::Record(root))=nodes.first_mut(){if let Some(FieldValue::List(items))=root.fields.get_mut(&4){if let Some(first)=items.first_mut(){*first=FieldValue::UInt(0)}}}}}
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(992),Generation(12),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
    let mut input=Some(value);let mut cursor=GltfJson::receiving_cursor();let mut failed=false;
    for _ in 0..1000{if step(&mut caller,&mut decoder,&mut cursor,&mut input).is_err(){failed=true;break}if cursor.is_ready(){break}}
    assert!(failed,"a child key that does not point forward must be refused");
    close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
}
