//! 📝️ The whole-value PagedUtf8 receiver equals std UTF-8, charges original work, and refuses over-capacity text.
use semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner;
use semio_framework_dsl_record::{DslReceivingField,FieldValue};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,RetainedWorkOutcome,ValueError,ValueRefusalKind,paged::PagedUtf8,retirement::{RetireOwned,controlled::ControlledRetirement}};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096}}
fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let g=grant();let mut value=ControlledRetirement::retain(value);for _ in 0..100_000{if value.terminal_is_empty(){return}caller.turn(g,|cx|cx.with_original_retirement(|cx|{let demand=value.selected_demand(g.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=value.step(grant);RetainedWorkOutcome{result,work:value.last_turn_work()}})?;Ok(())})).unwrap();}panic!("retirement did not settle")}
fn receive<const N:usize>(text:&str)->(Result<(),ValueRefusalKind>,Option<String>,usize){
    let g=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(902),Generation(24),g,||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
    let mut captured=None;let mut copied=0;
    caller.turn_with_decoder(g,&mut decoder,|cx|cx.with_original_work_loan(1_000_000,u64::MAX,|cx|{
        let(turn,progress)=cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,g);let turn=PagedUtf8::<N>::receive_value(&FieldValue::Text(text.to_owned()),&mut owner);Ok((turn,owner.progress()))})?;
        copied=progress.copied_bytes;let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"lost issued grant"))?;cx.consume_receiving_retained(issued,progress)?;cx.consume_fuel(turn.work);
        captured=Some(turn.result);Ok(())
    })).unwrap();
    let outcome=captured.take().unwrap();
    let result=match outcome{Ok(value)=>{let bytes:Vec<u8>=value.bytes().collect();let string=String::from_utf8(bytes).unwrap();retire(&mut caller,value);(Ok(()),Some(string))},Err(error)=>{let kind=error.kind();retire(&mut caller,error);(Err(kind),None)}};
    for _ in 0..100_000{if decoder.terminal_is_empty(){break}caller.turn(g,|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(g.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}
    assert!(decoder.terminal_is_empty());caller.retire(g);
    (result.0,result.1,copied)
}
#[test]
fn whole_value_paged_text_equals_std_utf8_and_charges_its_bytes(){
    for text in [String::new(),"x".into(),"a\u{e4}\u{1f600}\u{4e2d}".repeat(700),"y".repeat(1025)]{
        let(result,received,copied)=receive::<{1<<20}>(&text);
        assert!(result.is_ok(),"received: {} bytes",text.len());assert_eq!(received.as_deref(),Some(text.as_str()));assert!(copied>=text.len(),"copy charged: {copied} of {}",text.len());
    }
}
#[test]
fn whole_value_paged_text_refuses_over_capacity_text(){
    let(result,received,_)=receive::<3>("abcd");
    assert!(result.is_err());assert!(received.is_none());
}
