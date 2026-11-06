//! 🚪️ IO cause retains refusal identity and authored text source data.
use crate::IoError;
use semio_framework_value::{ValueError,ValueRefusalKind,NativeEncodeControl,RetainedCloneProgress};
use semio_framework_diagnostic::{TextError,TextSpan};
fn receive_io(wire:semio_framework_value::DslValue,maximum:usize,accept:bool)->Result<IoError,semio_framework_value_native_owned::OriginalReceivingPrimary>{
 use semio_framework_value::{NativeReceive,NativeReceiver,RetainedAllocationEffect,RetainedCloneGrant};
 use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,NativeDecodeConstructionRecipient,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkActionIssuer};
 struct Work;impl OriginalWorkRecipient for Work{fn admits<'a>(&mut self,quote:OriginalWorkQuote<'a>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'a>{quote.complete(Ok(observed.checked_add(additional).is_some_and(|n|n<=10000000)))}}
 struct Physical{grant:RetainedCloneGrant,submitted:RetainedCloneProgress}impl NativeDecodeConstructionRecipient for Physical{fn submitted(&self)->RetainedCloneProgress{self.submitted}fn grant(&mut self,_:semio_framework_value::RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.grant)}fn accept(&mut self,_:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.submitted=self.submitted.checked_add(p)?;Ok(())}}
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:4096};let mut wallet=();let mut source=semio_framework_value_testing_admission::capture(wire,&mut wallet,|_|grant,|_,_,_,_,_|{});let mut callback=|_|accept;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(maximum,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut receiver=<IoError as NativeReceive<semio_framework_value::DslValue>>::receiver();let mut failure=None;let mut work=Work;let mut issuer=OriginalWorkActionIssuer::new();let mut work_loan=issuer.loan(&mut work);let mut physical=Physical{grant,submitted:Default::default()};let mut physical_issuer=semio_framework_value_native_owned::native_decoding::NativeConstructionRecipientIssuer::new();let mut physical_loan=physical_issuer.loan(&mut physical);
 native.with_work_recipient(&mut work_loan,|native|{for _ in 0..1000000{if receiver.output_ready(){break}let turn=native.with_construction_recipient(&mut physical_loan,|native|Ok::<_,ValueError>(receiver.advance(source.borrow(),native,grant.maximum_copy_bytes,grant)))?;match turn.result{Ok(step)=>assert!(step.progress().fits(grant)),Err(error)=>{failure=Some(error);break}}}
 let output=if receiver.output_ready(){Some(receiver.take(grant).result.unwrap().unwrap().0)}else{None};receiver.begin_close();for _ in 0..1000000{if receiver.terminal_is_empty(){break}let turn=if receiver.requires_original_birth(){native.with_retirement::<_,ValueError>(grant,|loan|loan.with_original_close(|original|Ok(receiver.close_original_birth_step(original,grant)))).unwrap()}else{let mut actual=None;let mut effect=RetainedAllocationEffect::new(grant,u64::MAX,&mut actual);if receiver.requires_original_control(){native.with_retirement::<_,ValueError>(grant,|loan|loan.with_original_close(|original|Ok(receiver.close_original_step(original,&mut effect)))).unwrap()}else{receiver.close_step(&mut effect)}};turn.result.unwrap();}assert!(receiver.terminal_is_empty());
 for _ in 0..1000000{if source.terminal_is_empty(){break}let mut actual=None;source.close_step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual)).result.unwrap();}assert!(source.terminal_is_empty());while native.has_retirement_owner()||native.has_retirement_failure_storage(){native.close_retirement_recipient(grant).result.unwrap();}
 Ok::<_,ValueError>(match(output,failure){(Some(value),None)=>Ok(value),(None,Some(error))=>Err(error),_=>panic!("native IO receiving did not settle")})}).unwrap()
}
fn close_io(value:IoError){
 use semio_framework_value::{ErasedSnapshotRetirement,RetainedAllocationEffect,RetainedCloneGrant};
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:4096};let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();for _ in 0..1000000{if close.terminal_is_empty(){break}let mut actual=None;close.step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual)).unwrap();}assert!(close.terminal_is_empty());
}
#[test]
fn io_owned_cause_retains_kind_and_text_diagnostics() {
    let kinds=[ValueRefusalKind::InvalidValue,ValueRefusalKind::Canceled,ValueRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed,ValueRefusalKind::WorkLimit,ValueRefusalKind::DepthLimit,ValueRefusalKind::UnsupportedOwner,ValueRefusalKind::InvariantViolated];
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let kind=*kinds.iter().find(|kind|kind.as_str()==row["kind"].as_str().unwrap()).unwrap();
        let message=row["message"].as_str().unwrap().to_owned();
        let mut accept=|_|row["operation"]!="textCanceled";
        let mut control=NativeEncodeControl::new(if row["operation"]=="textOwnership"{0}else{1048576},&mut accept);
        let operation=row["operation"].as_str().unwrap();
        if operation=="textCanceled"||operation=="textOwnership" {
            let refusal=IoError::from_text_error_controlled(TextError::expected(kind,message,TextSpan{line:7,column:11,length:4},"required"),&mut control).unwrap_err();
            assert_eq!(refusal.kind.as_str(),row["expectedKind"].as_str().unwrap());
            continue;
        }
        let mut original=ValueError::new(kind,message.clone());
        if let Some(receipt)=row.get("retainedProgress") {original=original.with_retained_progress(RetainedCloneProgress{copied_items:usize::try_from(receipt["copiedItems"].as_u64().unwrap()).unwrap(),copied_bytes:usize::try_from(receipt["copiedBytes"].as_u64().unwrap()).unwrap(),retained_capacity_bytes:usize::try_from(receipt["retainedCapacityBytes"].as_u64().unwrap()).unwrap(),released_bytes:usize::try_from(receipt["releasedBytes"].as_u64().unwrap()).unwrap()});}
        let retained=original.retained_progress();
        let pointer=original.message.as_ptr();
        let error=if operation=="text" {IoError::from_text_error_controlled(TextError::expected(kind,message.clone(),TextSpan{line:7,column:11,length:4},"required"),&mut control).unwrap()}else{IoError::from_value_error(original)};
        if operation=="value" {assert_eq!(error.cause.message().as_ptr(),pointer);assert_eq!(error.cause.retained_progress(),retained);}
        assert_eq!(error.cause.kind(),kind);
        assert_eq!(error.cause.message(),message);
        if row["operation"]=="text" {
            assert_eq!(error.diagnostics.len(),1);
            assert_eq!(error.diagnostics[0].span,TextSpan{line:7,column:11,length:4});
            assert_eq!(error.diagnostics[0].message,message);
            assert_eq!(error.diagnostics[0].expected.as_ref().unwrap().describe(),"required");
        } else {assert!(error.diagnostics.is_empty());}
    }
    let copy_cases:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⚠️refusal/🧵️stress/🔣️.json")).unwrap();
    for row in copy_cases.as_array().unwrap() {
        let message=row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let threshold=row["cancelAfterCompleted"].as_u64().unwrap() as usize;
        let mut observed=Vec::new();
        let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{observed.push(event);threshold==0||event.total<=1||event.completed<threshold};
        let mut control=NativeEncodeControl::new(row["maximumBytes"].as_u64().unwrap() as usize,&mut progress);
        let result=IoError::from_text_error_controlled(TextError::expected(ValueRefusalKind::InvariantViolated,message.clone(),TextSpan{line:7,column:11,length:4},"required"),&mut control);
        if row["expectedKind"]=="success" {let output=result.unwrap();assert_eq!(output.cause.kind(),ValueRefusalKind::InvariantViolated);assert_eq!(output.cause.message(),message);assert_eq!(output.diagnostics[0].message,message);assert_eq!(output.diagnostics[0].span,TextSpan{line:7,column:11,length:4});}
        else {assert_eq!(result.unwrap_err().kind.as_str(),row["expectedKind"].as_str().unwrap());}
        drop(control);
        if threshold!=0 {assert!(observed.iter().any(|event|event.total>1&&event.completed>=threshold));}
    }
}

fn fixture_value(value: &serde_json::Value) -> semio_framework_value::DslValue {
    use semio_framework_value::{DslValue,Number};
    match value {
        serde_json::Value::Null=>DslValue::Null,
        serde_json::Value::Bool(value)=>DslValue::Bool(*value),
        serde_json::Value::Number(value)=>DslValue::Number(value.as_u64().map(Number::UInt).or_else(||value.as_i64().map(Number::Int)).unwrap()),
        serde_json::Value::String(value)=>DslValue::String(value.clone()),
        serde_json::Value::Array(values)=>DslValue::Array(values.iter().map(fixture_value).collect()),
        serde_json::Value::Object(fields)=>DslValue::Object(fields.iter().map(|(key,value)|(key.clone(),fixture_value(value))).collect()),
    }
}
#[test]
fn io_cause_controlled_wire_retains_all_authorities_and_source_data() {
    crate::register_io_schema_exports().unwrap();
    let published=semio_framework_schema_registry::resolve_schema_export("framework.io","schema",semio_framework_schema_registry::SchemaFormat::JsonSchema).unwrap();
    let schema:serde_json::Value=serde_json::from_str(published).unwrap();
    let canonical=&schema["$defs"]["IoError"];
    assert_eq!(canonical["required"],serde_json::json!(["cause","diagnostics"]));
    assert_eq!(canonical["additionalProperties"],false);
    assert_eq!(canonical["properties"]["cause"]["$ref"],"#/$defs/NativeSnapshotFailure");
    assert_eq!(schema["$defs"]["NativeSnapshotFailure"]["required"].as_array().unwrap().len(),8);
    assert_eq!(canonical["properties"]["diagnostics"]["items"]["$ref"],"https://semio.tech/schema/diagnostic/controlled#/$defs/Diagnostic");
    assert!(canonical["properties"].get("message").is_none());
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let Some(expected)=row.get("expectedWire")else{continue};
        let expected=fixture_value(expected);
        let decoded=receive_io(expected,1048576,true).unwrap();
        assert_eq!(decoded.cause.kind().as_str(),row["kind"].as_str().unwrap());
        assert_eq!(decoded.cause.message(),row["message"].as_str().unwrap());
        let mut accept=|_|true;
        let encoded=decoded.to_value_controlled(&mut NativeEncodeControl::new(1048576,&mut accept)).unwrap();
        let encoded_wire=fixture_output(&encoded);
        let roundtrip=receive_io(encoded,1048576,true).unwrap();
        assert_eq!(roundtrip,decoded);
        assert_eq!(encoded_wire,row["expectedWire"]);
        close_io(roundtrip);close_io(decoded);
    }
}
#[test]
fn io_cause_wire_refuses_closed_members_and_hostile_controls() {
    use semio_framework_value::DslValue;
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⚠️refusal/🔁️wire/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let input=DslValue::Object(row["members"].as_array().unwrap().iter().map(|member|(member[0].as_str().unwrap().to_owned(),fixture_value(&member[1]))).collect());
        let accept=row["accept"].as_bool().unwrap();
        let maximum=usize::try_from(row["maximumBytes"].as_u64().unwrap()).unwrap();
        let refusal=if row["operation"]=="encode" {
            let mut progress=|_|accept;
            IoError::from_value_error(ValueError::new(ValueRefusalKind::WorkLimit,"canceled spoofed prose")).to_value_controlled(&mut NativeEncodeControl::new(maximum,&mut progress)).unwrap_err()
        } else {
            let failure=receive_io(input,maximum,accept).unwrap_err();
            failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).or(failure.work_refusal).or(failure.receipt_refusal).expect("actual IO refusal")
        };
        assert_eq!(refusal.kind.as_str(),row["expectedKind"].as_str().unwrap());
    }
}

fn fixture_output(value: &semio_framework_value::DslValue) -> serde_json::Value {
    use semio_framework_value::{DslValue,Number};
    match value {
        DslValue::Null=>serde_json::Value::Null,
        DslValue::Bool(value)=>serde_json::Value::Bool(*value),
        DslValue::Number(Number::UInt(value))=>serde_json::Value::Number((*value).into()),
        DslValue::Number(Number::Int(value))=>serde_json::Value::Number((*value).into()),
        DslValue::Number(Number::Float(value))=>serde_json::Value::Number(serde_json::Number::from_f64(*value).unwrap()),
        DslValue::String(value)=>serde_json::Value::String(value.clone()),
        DslValue::Array(values)=>serde_json::Value::Array(values.iter().map(fixture_output).collect()),
        DslValue::Object(fields)=>serde_json::Value::Object(fields.iter().map(|(key,value)|(key.clone(),fixture_output(value))).collect()),
        _=>panic!("closed IO fixture primitive"),
    }
}
