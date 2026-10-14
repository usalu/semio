//! 🧪️ Same original source funds value rows, refused donors and every partial physical close.
use super::*;
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};

fn clock()->Option<u64>{Some(1)}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn caller(law:&serde_json::Value)->OriginalJobFixtureCaller{
    let input=&law["caller"];let stock=input["stock"].as_array().unwrap();
    OriginalJobFixtureCaller::admit(OperationId(input["operation"].as_u64().unwrap()),Generation(input["generation"].as_u64().unwrap()),RetainedCloneGrant{maximum_items:stock[0].as_u64().unwrap()as usize,maximum_copy_bytes:stock[1].as_u64().unwrap()as usize,maximum_capacity_bytes:stock[2].as_u64().unwrap()as usize,maximum_release_bytes:stock[3].as_u64().unwrap()as usize,maximum_depth:stock[4].as_u64().unwrap()as usize},clock,input["deadlineUs"].as_u64().unwrap(),input["fuel"].as_u64().unwrap())
}
fn turn(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>)->(bool,usize,usize){
    let(result,born,freed)=crate::test_allocation::observe_backing(||caller.turn(grant,|cx|Ok(run(cx))).unwrap().unwrap());
    let receipt=caller.receipt();assert!(receipt.fits(grant));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));(result,born,freed)
}

#[semio_framework_async_macros::async_test]
async fn original_inferred_values_same_source_donor_denials_partial_cancel_and_full_paid_close(){
    let law=fixture();let extent=law["nodes"].as_u64().unwrap()as usize;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:256,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
    for stop in [0usize,1,4,9,usize::MAX]{
        let mut caller=caller(&law);let mut values=InferenceValues::<i64>::new();let(mut turns,mut births,mut releases)=(0usize,0usize,0usize);
        loop{
            if turns==stop{caller.cancel().cancel().await;break}
            let result=if values.source.is_none(){turn(&mut caller,grant,|cx|values.admit_source_step(cx))}else{turn(&mut caller,grant,|cx|values.initialize_step(extent,cx))};
            births+=result.1;releases+=result.2;turns+=1;assert!(turns<64);
            if result.0&&values.rows_ready(){break}
        }
        if stop==usize::MAX{
            for row in law["outputs"].as_array().unwrap(){
                let source=row["source"].as_u64().unwrap()as usize;let value=row["value"].as_i64().unwrap();let mut donor=Some(value);
                for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:size_of::<Option<i64>>()-1,..grant},RetainedCloneGrant{maximum_depth:1,..grant}]{
                    assert_eq!(turn(&mut caller,denied,|cx|values.publish_step(source,&mut donor,cx)),(false,0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(donor,Some(value));assert_eq!(values.get(source),None);
                }
                assert!(turn(&mut caller,grant,|cx|values.publish_step(source,&mut donor,cx)).0);assert_eq!(donor,None);assert_eq!(values.get(source),Some(&value));
            }
            assert_eq!(values.iter().collect::<Vec<_>>(),vec![(0,&3),(2,&11)]);
        }
        if values.source.is_some(){assert!(!caller.turn(grant,|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());}
        loop{
            let denied=RetainedCloneGrant{maximum_items:0,..grant};assert_eq!(turn(&mut caller,denied,|cx|values.close_step(cx)),(false,0,0));assert_eq!(caller.receipt(),Default::default());
            let(result,born,freed)=turn(&mut caller,grant,|cx|values.close_step(cx));births+=born;releases+=freed;turns+=1;assert!(turns<1024);
            if !values.terminal_is_empty()&&values.source.is_some(){assert!(!caller.turn(grant,|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());}
            if result{break}
        }
        assert!(values.terminal_is_empty());assert!(caller.turn(grant,|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());assert_eq!(births,releases);caller.retire(grant);
        let(_,born,freed)=crate::test_allocation::observe_backing(||drop(values));assert_eq!((born,freed),(0,0));
        eprintln!("[DEBUG] original inference values op969/gen17 stop{stop} turns{turns} capacity{births}=release{releases}; refused exact output donor preserved; typed payload before source return; terminal Drop0");
    }
}
