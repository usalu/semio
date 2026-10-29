//! 🧪️ Genuine native host configuration sources match the neutral policy fixture and retain all physical owners.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::RetainedCloneGrant;
static RAW:Declaration<String>=Declaration{maximum_input_bytes:usize::MAX,maximum_output_bytes:usize::MAX,projection:Projection::RawText,mutation:|text|text};
static PROCESS:Declaration<String>=Declaration{maximum_input_bytes:262144,maximum_output_bytes:24576,projection:Projection::Contributions(Contributions{maximum_depth:32,maximum_items:4096,maximum_string_bytes:16384,topic:"process.machines",app_id:"process3d-play",payload_fields:&["appId","moduleId","label","iconId","machinesJson"],deduplicate_modules:false,excluded_module_ids:&[],maximum_modules:usize::MAX}),mutation:|text|text};
static SOURCING:Declaration<String>=Declaration{maximum_input_bytes:262144,maximum_output_bytes:2048,projection:Projection::Contributions(Contributions{maximum_depth:32,maximum_items:4096,maximum_string_bytes:4096,topic:"sourcing.module",app_id:"sourcing-curation",payload_fields:&["appId","moduleId","label","typologyJson","kindsJson"],deduplicate_modules:true,excluded_module_ids:&["beams","windows","slabs","reuse"],maximum_modules:8}),mutation:|text|text};
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:16<<20,maximum_release_bytes:16<<20,maximum_depth:4096}}
fn clock()->Option<u64>{Some(1)}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(937),Generation(41),RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},clock,u64::MAX,1<<24)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));result.unwrap()
}
fn close(mut source:Option<Box<OriginalHostConfiguration<String>>>,mut output:Option<String>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
    let g=grant();for step in 0..1000000{
        if source.is_none(){break}
        turn(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{
            if source.as_ref().unwrap().terminal_is_empty(){let d=OriginalHostConfiguration::<String>::release_demands();close_original_retained_step(cx,d,|_|{drop(source.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;}else{source.as_mut().unwrap().close_step(cx)?;}Ok(())
        }));assert!(step<999999,"native host owner close stalled");
    }
    if let Some(value)=output.as_ref(){let d=RetirementDemand{copy_bytes:size_of::<Option<String>>(),release_bytes:value.capacity(),depth:1,..Default::default()};turn(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|_|{drop(output.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(())}));}
    while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})));let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));result.unwrap();}caller.retire(g);
}
#[test]
fn original_host_configuration_matches_all_neutral_policies_with_actual_native_custody(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let declaration=match case["policy"].as_str().unwrap(){"raw"=>&RAW,"process"=>&PROCESS,"sourcing"=>&SOURCING,_=>unreachable!()};
        let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut args=Some(DslValue::object([("json".to_owned(),DslValue::String(case["input"].as_str().unwrap().to_owned()))]));let mut source=None;
        turn(&mut caller,&mut decoder,grant(),|cx|{assert!(OriginalHostConfiguration::admit(declaration,&mut args,cx,&mut source)?);Ok(())});assert!(args.is_none());
        let mut foreign=OriginalJobDecodeOwner::empty();turn(&mut caller,&mut foreign,grant(),|cx|{assert!(source.as_mut().unwrap().advance(cx).is_err());Ok(())});assert!(foreign.terminal_is_empty());
        for step in 0..1000000{if source.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..grant()},|cx|{assert!(!source.as_mut().unwrap().advance(cx)?);Ok(())});assert_eq!(caller.receipt(),RetainedCloneProgress::default());turn(&mut caller,&mut decoder,grant(),|cx|source.as_mut().unwrap().advance(cx));assert!(step<999999,"native host source stalled at {}",case["name"]);}
        let mut output=None;turn(&mut caller,&mut decoder,grant(),|cx|{assert!(source.as_mut().unwrap().take_into(&mut output,cx)?);Ok(())});assert_eq!(output.as_deref(),case["expected"].as_str(),"{}",case["name"]);println!("[DEBUG] [host-configuration] {}",serde_json::json!({"name":case["name"],"output":output.as_ref().unwrap()}));close(source,output,&mut caller,&mut decoder);
    }
}
#[test]
fn original_host_configuration_cancellation_keeps_all_accepted_source_owners_until_paid_release(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let text=fixture["cases"].as_array().unwrap().iter().find(|case|case["name"]=="process-addressed-host-order").unwrap()["input"].as_str().unwrap();
    for cut in [0,1,3,7,31,127,511]{
        let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut args=Some(DslValue::object([("json".to_owned(),DslValue::String(text.to_owned()))]));let mut source=None;turn(&mut caller,&mut decoder,grant(),|cx|{OriginalHostConfiguration::admit(&PROCESS,&mut args,cx,&mut source)?;Ok(())});
        for _ in 0..cut{if source.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,grant(),|cx|source.as_mut().unwrap().advance(cx));}caller.cancel().cancel_now();close(source,None,&mut caller,&mut decoder);
    }
}
