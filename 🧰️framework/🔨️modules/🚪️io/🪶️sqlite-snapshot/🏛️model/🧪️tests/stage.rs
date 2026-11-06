//! 🧪️ Portable stage laws preserve the original result and independent refusal.
use super::*;
use semio_framework_value::RetainedWorkOutcome;

#[test]
fn native_stage_retains_owned_result_and_independent_settlement() {
    struct Loan;
    impl SqliteSnapshotWorkRecipient for Loan{fn admits(&mut self,_:u64,_:u64)->std::result::Result<bool,ValueError>{Ok(true)}fn committed(&mut self,_:u64)->std::result::Result<(),ValueError>{Ok(())}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("stage.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() { for native in [false,true] {
        let mut limits=SqliteDatabaseLimits::default();
        limits.max_allocation_bytes=0;
        let body=std::cell::Cell::new(false);
        let mut progress=|_|row["body"].as_bool().unwrap();
        let mut control=SqliteSnapshotControl::new(&mut progress,limits);
        let mut loan=Loan;let mut failure=None;control.install_work_recipient(&mut loan,&mut failure).unwrap();
        let operation=row["operation"].as_str().unwrap();
        let mut address=std::ptr::null();
        let mut produce=|| {
            body.set(true);
            let value=operation.to_owned();address=value.as_ptr();
            let result=if operation=="output"{Ok(value)}else{Err((value,"earlier-refusal".to_owned()))};
            (RetainedWorkOutcome{result,work:7},1)
        };
        let(result,work)=if native {let outcome=control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|_,_|{let mut callback=|_|true;let mut child=semio_framework_value::NativeEncodeControl::new(0,&mut callback);let mut child_loan=Loan;let before=child.actual_work();let(turn,bytes)=child.with_work_recipient(&mut child_loan,|child|{assert!(child.prepare_work(7)?);let output=produce();child.commit_work(7)?;Ok::<_,ValueError>(output)}).unwrap();let after=child.actual_work();(turn,bytes,SqliteStageWorkReceipt{declared:7,before,after,actual:after.checked_sub(before)})});(outcome.result,outcome.work)} else {let outcome=control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|_,_,_,work|{assert!(work.admits(0,7).unwrap());let output=produce();work.committed(7).unwrap();output});(outcome.result,Some(outcome.work))};
        assert_eq!(body.get(),row["body"].as_bool().unwrap());
        match result {
            Err(SqliteStageFailure::NotStarted(_))=>assert!(!body.get()),
            Err(SqliteStageFailure::Settlement{operation:Ok(value),refusal,receipt})=>{assert_eq!(value.as_ptr(),address);assert_eq!(value,operation);assert_eq!(refusal.kind,if native {ValueRefusalKind::OwnershipLimit}else{ValueRefusalKind::InvariantViolated});assert_eq!(work,Some(7));let receipt=receipt.unwrap();assert_eq!((receipt.declared,receipt.actual,receipt.before,receipt.after,receipt.remaining),(1,Some(0),0,0,0))},
            Err(SqliteStageFailure::Settlement{operation:Err((value,prior)),refusal,receipt})=>{assert_eq!(value.as_ptr(),address);assert_eq!(value,operation);assert_eq!(prior,"earlier-refusal");assert_eq!(refusal.kind,if native {ValueRefusalKind::OwnershipLimit}else{ValueRefusalKind::InvariantViolated});assert_eq!(work,Some(7));let receipt=receipt.unwrap();assert_eq!((receipt.declared,receipt.actual,receipt.before,receipt.after,receipt.remaining),(1,Some(0),0,0,0))},
            _=>panic!("stage case did not retain its owners"),
        }
    }}
}

#[test]
fn stage_retains_occupied_native_settlement_and_new_refusal() {
    use semio_framework_io_control::NativeSnapshotFailure;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("stage.json")).unwrap();
    assert!(fixture["cases"].as_array().unwrap().iter().any(|row|row["name"]=="occupied-failure-and-settlement"));
    let primary=ValueError::new(ValueRefusalKind::InvalidValue,String::from("original body"));
    let first=ValueError::new(ValueRefusalKind::Canceled,String::from("original settlement"));
    let primary_ptr=primary.message.as_ptr();
    let first_ptr=first.message.as_ptr();
    let failure=NativeSnapshotFailure::from(primary).with_settlement_refusal(first).unwrap();
    let mut progress=|_|true;
    let mut limits=SqliteDatabaseLimits::default();
    limits.max_allocation_bytes=0;
    let mut control=SqliteSnapshotControl::new(&mut progress,limits);
    let turn=control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|_,_|(RetainedWorkOutcome::<(),_>{result:Err(failure),work:0},1,SqliteStageWorkReceipt{declared:0,before:0,after:0,actual:Some(0)}));
    match turn.result {
        Err(SqliteStageFailure::Settlement{operation:Err(failure),refusal,receipt})=>{
            assert_eq!(failure.message().as_ptr(),primary_ptr);
            assert_eq!(failure.settlement_refusal().unwrap().message.as_ptr(),first_ptr);
            assert_eq!(refusal.kind,ValueRefusalKind::OwnershipLimit);let receipt=receipt.unwrap();assert_eq!((receipt.declared,receipt.actual,receipt.before,receipt.after,receipt.remaining),(1,Some(0),0,0,0));
        }
        _=>panic!("both settlement refusals must remain owned"),
    }
}

#[test]
fn stage_reports_actual_bounded_native_work() {
    use semio_framework_value::native_encoding::NativeEncodeWorkRecipient;
    struct Loan { remaining:u64, committed:u64 }
    impl NativeEncodeWorkRecipient for Loan {
        fn admits(&mut self,_:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(additional<=self.remaining)}
        fn committed(&mut self,actual:u64)->std::result::Result<(),ValueError>{self.remaining-=actual;self.committed+=actual;Ok(())}
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("stage.json")).unwrap();
    let expected=&fixture["boundedWork"];
    let mut loan=Loan{remaining:expected["grant"].as_u64().unwrap(),committed:0};
    let mut progress=|_|true;
    let mut limits=SqliteDatabaseLimits::default();limits.max_allocation_bytes=0;
    let mut control=SqliteSnapshotControl::new(&mut progress,limits);
    let turn=control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|_,_|{
        let mut callback=|_|true;
        let mut native=semio_framework_value::NativeEncodeControl::new(0,&mut callback);
        let before=native.actual_work();
        let result=native.with_work_recipient(&mut loan,|native|{
            let mut output=[0u8;2];
            for byte in &mut output { assert!(native.prepare_work(1)?); *byte=1; native.commit_work(1)?; }
            assert_eq!(output,[1,1]);assert!(!native.prepare_work(1)?);
            Ok::<_,ValueError>(())
        });
        let after=native.actual_work();let actual=after.checked_sub(before);(RetainedWorkOutcome{result,work:loan.committed},native.owned_bytes(),SqliteStageWorkReceipt{declared:loan.committed,before,after,actual})
    });
    assert_eq!(turn.work,Some(expected["committed"].as_u64().unwrap()));
    assert_eq!(Some(loan.committed),turn.work);assert_eq!(loan.remaining,0);
    assert!(turn.result.is_ok());
}

#[test]
fn native_stage_retains_counter_regression_and_original_output(){
 let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()});let original=String::from("original output");let pointer=original.as_ptr();let turn=control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|_,_|(RetainedWorkOutcome::<_,ValueError>{result:Ok(original),work:1},1,SqliteStageWorkReceipt{declared:1,before:5,after:4,actual:None}));assert_eq!(turn.work,None);match turn.result{Err(SqliteStageFailure::WorkSettlement{operation:Ok(output),failure:None,work,allocation:Some((refusal,receipt))})=>{assert_eq!(output.as_ptr(),pointer);assert_eq!((work.declared,work.before,work.after,work.actual),(1,5,4,None));assert_eq!(refusal.kind,ValueRefusalKind::OwnershipLimit);assert_eq!((receipt.declared,receipt.actual),(1,Some(0)));},_=>panic!("original output, raw work regression and allocation refusal must survive")}
}
