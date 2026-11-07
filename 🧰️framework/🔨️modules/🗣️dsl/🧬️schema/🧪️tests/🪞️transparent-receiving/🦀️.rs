//! 🪞️ A transparent newtype receives exactly like its inner field, equals the plain value, and closes every cancelled frontier.
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,OwnedFieldReceiving,OriginalReceivingPrimary};
use semio_framework_dsl_record::transparent::{TransparentField,TransparentReceiving};
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,retirement::RetireOwned};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[derive(Debug,PartialEq,semio_framework_value::RetireOwned)]
struct Octets(Vec<u8>);
semio_framework_dsl_record::dsl_transparent_receiving!(Octets,Vec<u8>);
#[derive(Debug,PartialEq,semio_framework_value::RetireOwned)]
struct Numbers(Vec<i64>);
semio_framework_dsl_record::dsl_transparent_receiving!(Numbers,Vec<i64>);

#[derive(Debug,PartialEq,semio_framework_value::RetireOwned)]
struct Even(Vec<i64>);
semio_framework_dsl_record::dsl_transparent_receiving!(Even,Vec<i64>,|inner|if inner.iter().all(|n|n%2==0){Ok(())}else{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"odd"))},|inner|Even(inner));

fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn close_value<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut value=ControlledRetirement::retain(value);for _ in 0..100_000{if value.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=value.selected_demand(loan().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=value.step(grant);RetainedWorkOutcome{result,work:value.last_turn_work()}})?;Ok(())}).unwrap();}panic!("paged text fixture retirement did not settle")}
fn close<W:TransparentField>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut TransparentReceiving<W>){for _ in 0..100_000{if cursor.terminal_is_empty(){break}caller.turn(loan(),|cx|{let demand=cursor.close_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let turn=cursor.close_step(grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();}assert!(cursor.terminal_is_empty());for _ in 0..100_000{if decoder.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())}).unwrap();}panic!("paged text fixture decoder did not settle")}
fn step<W:TransparentField>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut TransparentReceiving<W>,input:&mut Option<FieldValue>)->Result<(),OriginalTurnFailure>{caller.turn_with_decoder(loan(),decoder,|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|native|{let turn=cursor.step(input,native,32768,issued);work=turn.work;turn.result});let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None})}})}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096}}
fn run<W:TransparentField+PartialEq+std::fmt::Debug>(source:FieldValue,expected:W,cut:Option<usize>){
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(996),Generation(16),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
    let mut input=Some(source);let mut cursor=TransparentReceiving::<W>::default();let mut turns=0;
    while !cursor.is_ready()&&cut.is_none_or(|cut|turns<cut){step(&mut caller,&mut decoder,&mut cursor,&mut input).unwrap();turns+=1;assert!(turns<100_000);}
    if cut.is_none(){let mut output=None;caller.turn(loan(),|cx|{let turn=cursor.take_into(&mut output,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();let output=output.unwrap();assert_eq!(output,expected);close_value(&mut caller,output);}else{caller.cancel().cancel_now();}
    close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
}
#[test]
fn transparent_newtypes_equal_their_plain_values_and_cancel_at_every_turn(){
    for cut in [None,Some(0),Some(1),Some(3),Some(9)]{
        for bytes in [vec![],vec![7u8],(0..=255u8).collect::<Vec<_>>()]{run(FieldValue::List(bytes.iter().map(|b|FieldValue::UInt(u64::from(*b))).collect()),Octets(bytes.clone()),cut);}
        for numbers in [vec![],vec![-3i64,0,9]]{run(FieldValue::List(numbers.iter().map(|n|FieldValue::Int(*n)).collect()),Numbers(numbers.clone()),cut);}
    }
}
#[test]
fn transparent_newtypes_refuse_foreign_shapes_without_consuming_the_source(){
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(995),Generation(15),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
    let mut input=Some(FieldValue::Bool(true));let mut cursor=TransparentReceiving::<Octets>::default();let mut failed=false;
    for _ in 0..16{if step(&mut caller,&mut decoder,&mut cursor,&mut input).is_err(){failed=true;break}}
    assert!(failed);assert!(input.is_some());
    close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
}
#[test]
fn checked_transparent_wrap_accepts_valid_values_and_refuses_invalid_ones_keeping_retirement_paid(){
    run(FieldValue::List([2i64,4].iter().map(|n|FieldValue::Int(*n)).collect()),Even(vec![2,4]),None);
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(994),Generation(14),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
    let mut input=Some(FieldValue::List([1i64].iter().map(|n|FieldValue::Int(*n)).collect()));let mut cursor=TransparentReceiving::<Even>::default();let mut failed=false;
    for _ in 0..64{if step(&mut caller,&mut decoder,&mut cursor,&mut input).is_err(){failed=true;break}}
    assert!(failed,"an odd value must be refused by the check");
    close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
}
