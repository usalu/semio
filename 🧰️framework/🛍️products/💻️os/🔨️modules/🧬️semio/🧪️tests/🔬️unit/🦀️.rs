use super::*;

/// 🎟️ Keeps every cleanup span inside the original caller wallet and native receipt.
fn retire_original_semio_owner<T:semio_framework_value::retirement::RetireOwned>(value:T,quantum:usize,caller:semio_framework_value::RetainedCloneGrant,total:&mut semio_framework_value::RetainedCloneProgress,control:&mut semio_framework_value::NativeEncodeControl<'_>)->semio_framework_value::RetainedCloneProgress{
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
    let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("original semio retirement owner: {error}"));
    assert!(matches!(owner.step(Default::default()).unwrap(),RetainedCloneStep::Progress(progress)if progress==RetainedCloneProgress::default()));
    let mut performed=RetainedCloneProgress::default();let mut turns=0;
    while !owner.terminal_is_empty(){
        turns+=1;assert!(turns<=caller.maximum_items);
        let remaining=RetainedCloneGrant{maximum_items:caller.maximum_items-total.copied_items,maximum_copy_bytes:caller.maximum_copy_bytes-total.copied_bytes,maximum_capacity_bytes:caller.maximum_capacity_bytes-total.retained_capacity_bytes,maximum_release_bytes:caller.maximum_release_bytes-total.released_bytes,maximum_depth:caller.maximum_depth};
        let copy=owner.next_copy_byte_demand().unwrap();let work=quantum.max(copy).min(remaining.maximum_copy_bytes);let capacity=owner.next_capacity_byte_demand(work).unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=owner.next_depth_demand().unwrap();
        assert!(remaining.maximum_items>0&&copy<=remaining.maximum_copy_bytes&&capacity<=remaining.maximum_capacity_bytes&&release<=remaining.maximum_release_bytes&&depth<=remaining.maximum_depth);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,..remaining};control.checkpoint().unwrap();control.charge(capacity).unwrap();
        let(step,allocated,released)=crate::test_allocation::observe_backing(||owner.step(grant));let progress=step.unwrap().progress();assert_eq!((allocated,released),(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(grant));assert_ne!(progress,RetainedCloneProgress::default(),"funded original cleanup must make progress");*total=total.checked_add(progress).unwrap();performed=performed.checked_add(progress).unwrap();
    }
    performed
}

#[test]
fn retained_text_envelope_preserves_original_body_and_bounds_resume_cancel_and_release() {
    use semio_framework_value::{NativeEncodeControl, ValueRefusalKind};
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️controlled/🔣️.json")).unwrap();
    let body=fixture["textBody"].as_str().unwrap().repeat(fixture["retained"]["textRepeats"].as_u64().unwrap() as usize);
    let id=fixture["id"].as_str().unwrap(); let version=fixture["version"].as_u64().unwrap() as u16;
    let expected=wrap_text(&SemioEnvelope::from_envelope_id(id,Component::Dsl,version).unwrap(),&body);
    let grant=fixture["retained"]["retirementBytes"].as_u64().unwrap() as usize;
    let declared=&fixture["retained"]["callerGrant"];let caller=semio_framework_value::RetainedCloneGrant{maximum_items:declared["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:declared["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:declared["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:declared["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:declared["maximumDepth"].as_u64().unwrap()as usize};
    let depth=fixture["retained"]["maximumRetirementDepth"].as_u64().unwrap()as usize;let ownership=&fixture["retained"]["ownership"];assert_eq!(ownership["copy"],"bounded-original-payload");assert_eq!(ownership["capacity"],"exact-natural-birth");assert_eq!(ownership["release"],"exact-physical-allocation");assert_eq!(ownership["depth"],"exact-original-frontier");assert_eq!(ownership["refusal"],"original-owner-kept");
    for budget in fixture["retained"]["budgets"].as_array().unwrap() {
        let source=body.clone(); let pointer=source.as_ptr();
        let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,source,caller);
        assert_eq!(cursor.source_body().unwrap().as_ptr(),pointer);
        let mut accept=|_|true; let mut control=NativeEncodeControl::new(fixture["retained"]["nativeMaximum"].as_u64().unwrap()as usize,&mut accept);
        assert!(cursor.step(1,grant,&mut control).unwrap().is_none());
        let output=loop { let before=cursor.position(); assert!(cursor.step(0,0,&mut control).unwrap().is_none()); assert_eq!(cursor.position(),before); let before=cursor.progress();let(step,allocated,released)=crate::test_allocation::observe_backing(||cursor.step(budget.as_u64().unwrap() as usize,grant,&mut control));let after=cursor.progress();assert_eq!((allocated,released),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));if let Some(output)=step.unwrap(){break output;} };
        assert_eq!(output,expected); assert!(cursor.source_body().is_none());assert!(cursor.progress().fits(caller));assert_eq!(cursor.remaining_grant().maximum_copy_bytes,caller.maximum_copy_bytes-cursor.progress().copied_bytes);
        assert!(cursor.retirement.is_none());let mut total=cursor.progress();retire_original_semio_owner(cursor,grant,caller,&mut total,&mut control);
        let output_capacity=output.capacity();let progress=retire_original_semio_owner(output,grant,caller,&mut total,&mut control);assert!(progress.released_bytes>=output_capacity,"the exact physical output allocation is fully returned independently of the small copy grant");
    }
    for stop in [0,1,8,64,256,1024] {
        let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body.clone(),caller);
        let live=std::cell::Cell::new(true); let mut accept=|_|live.get(); let mut control=NativeEncodeControl::new(fixture["retained"]["nativeMaximum"].as_u64().unwrap()as usize,&mut accept);
        for _ in 0..stop { assert!(cursor.step(1,grant,&mut control).unwrap().is_none()); }
        let before=cursor.position(); live.set(false); assert_eq!(cursor.step(1,grant,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled); assert_eq!(cursor.position(),before);
        let sources=cursor.source_body().map_or(0,|body|body.len());let mut total=cursor.progress();live.set(true);let progress=retire_original_semio_owner(cursor,grant,caller,&mut total,&mut control);assert!(progress.released_bytes>=sources);
    }
    let final_step=std::cell::Cell::new(0usize);let mut accept=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{final_step.set(event.completed);true};let mut baseline=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body.clone(),caller);let mut native=NativeEncodeControl::new(fixture["retained"]["nativeMaximum"].as_u64().unwrap()as usize,&mut accept);let output=loop{if let Some(output)=baseline.step(1,grant,&mut native).unwrap(){break output}};let cutoff=final_step.get();assert!(cutoff>0);let mut total=baseline.progress();retire_original_semio_owner(baseline,grant,caller,&mut total,&mut native);retire_original_semio_owner(output,grant,caller,&mut total,&mut native);drop(native);
    let live=std::cell::Cell::new(true);let once=std::cell::Cell::new(true);let mut accept=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if once.get()&&event.completed==cutoff{once.set(false);live.set(false);}live.get()};let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body.clone(),caller);let mut native=NativeEncodeControl::new(fixture["retained"]["nativeMaximum"].as_u64().unwrap()as usize,&mut accept);loop{match cursor.step(1,grant,&mut native){Ok(None)=>{},Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::Canceled);break},Ok(Some(_))=>panic!("original final cancellation published owned output")}}assert_eq!(fixture["retained"]["finalPublication"]["completedOnRefusal"],false);assert!(!cursor.complete);assert!(cursor.retirement.as_ref().unwrap().terminal_is_empty());let pointer=cursor.output.as_ref().unwrap().as_ptr();assert_eq!(cursor.output.as_deref().unwrap(),expected);live.set(true);let output=cursor.step(1,grant,&mut native).unwrap().unwrap();assert_eq!(output.as_ptr(),pointer);assert_eq!(output,expected);let mut total=cursor.progress();retire_original_semio_owner(cursor,grant,caller,&mut total,&mut native);retire_original_semio_owner(output,grant,caller,&mut total,&mut native);assert!(total.fits(caller));
    let narrowed=semio_framework_value::RetainedCloneGrant{maximum_items:fixture["retained"]["undergrantItems"].as_u64().unwrap()as usize,..caller};
    let mut underfunded=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body.clone(),narrowed);let original=underfunded.source_body().unwrap().as_ptr();let mut accept=|_|true;let mut native=NativeEncodeControl::new(fixture["retained"]["nativeMaximum"].as_u64().unwrap()as usize,&mut accept);
    loop{match underfunded.step(1,grant,&mut native){Ok(None)=>{},Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::WorkLimit);break},Ok(Some(_))=>panic!("independent 256-item undergrant cannot cover the original payload")}}
    assert_eq!(underfunded.source_body().unwrap().as_ptr(),original);let progress=underfunded.progress();assert_eq!(progress.copied_items,narrowed.maximum_items);let owned=native.owned_bytes();assert_eq!(underfunded.step(1,grant,&mut native).unwrap_err().kind,ValueRefusalKind::WorkLimit);assert_eq!(underfunded.progress(),progress);assert_eq!(native.owned_bytes(),owned);let mut total=progress;retire_original_semio_owner(underfunded,grant,caller,&mut total,&mut native);assert!(total.fits(caller));
    let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body,caller);let pointer=cursor.source_body().unwrap().as_ptr();let mut accept=|_|true; let mut control=NativeEncodeControl::new(fixture["retained"]["nativeMaximum"].as_u64().unwrap()as usize,&mut accept);assert_eq!(control.scoped_maximum(1,|control|cursor.step(1,grant,control)).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(cursor.source_body().unwrap().as_ptr(),pointer);let mut total=cursor.progress();retire_original_semio_owner(cursor,grant,caller,&mut total,&mut control);
    let script="const x=JSON.parse(await Bun.stdin.text());const bytes=new TextEncoder().encode('semio '+x.id+'.dsl v'+x.version+'\\n'+x.textBody.repeat(x.retained.textRepeats));await Bun.write(Bun.stdout,bytes)";
    use std::{io::Write,process::{Command,Stdio}}; let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(fixture.to_string().as_bytes()).unwrap(); let result=child.wait_with_output().unwrap(); assert!(result.status.success()); assert_eq!(result.stdout,expected.as_bytes());
    eprintln!("[DEBUG] Original declared Text body pointer retained; budgets1/8/256 zero/cancel/ownership refusal and3byte payload work; independent5axisPolicy=true narrowed256WorkRefusal=true finalCancelRetainsOutputPointer=true sameOriginalNativeWallet=true exactSystemBirthRelease=true independentTextEncoder=true");
}

#[test]
fn semio_envelope_identity_matches_independent_neutral_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️envelope-identity/🔣️.json")).expect("neutral envelope vectors");
    let expected = &vectors["expected"];
    for case in vectors["cases"].as_array().unwrap() {
        let item = &case["envelope"];
        let envelope = SemioEnvelope {
            plugin: item["plugin"].as_str().unwrap().into(),
            artifact: item["artifact"].as_str().unwrap().into(),
            component: Component::parse(item["component"].as_str().unwrap()).unwrap(),
            version: item["version"].as_u64().unwrap() as u16,
        };
        assert_eq!(envelope.matches_identity(expected["id"].as_str().unwrap(), Component::parse(expected["component"].as_str().unwrap()).unwrap(), expected["version"].as_u64().unwrap() as u16), case["matches"].as_bool().unwrap(), "{}", case["name"]);
    }
}

#[test]
fn text_preamble_round_trip() {
    let env = SemioEnvelope { plugin: "gis".into(), artifact: "gismap".into(), component: Component::Dsl, version: 1 };
    let wrapped = wrap_text(&env, "positions [id:TEXT] { }");
    let (parsed, body) = split_text_preamble(&wrapped).unwrap();
    assert_eq!(parsed, env);
    assert!(body.starts_with("positions"));
}

#[test]
fn binary_header_round_trip() {
    let env = SemioEnvelope { plugin: "gis".into(), artifact: "gismap".into(), component: Component::Pack, version: 1 };
    let inner = b"payload-bytes";
    let wrapped = wrap_binary(&env, inner);
    let (parsed, payload) = unwrap_binary(&wrapped).unwrap();
    assert_eq!(parsed, env);
    assert_eq!(payload, inner);
}

#[test]
fn sniff_text_and_binary() {
    let dsl_env = SemioEnvelope::from_envelope_id("gis.gismap", Component::Dsl, 1).unwrap();
    let text = wrap_text(&dsl_env, "schema=gis.map id=x");
    assert_eq!(sniff(text.as_bytes()).unwrap().component, Component::Dsl);
    let bin = wrap_binary(&SemioEnvelope::from_envelope_id("gis.gismap", Component::Pack, 1).unwrap(), b"x");
    assert_eq!(sniff(&bin).unwrap().component, Component::Pack);
}

#[path = "../⚠️emission/🦀️.rs"]
mod emission;

#[test]
fn original_controlled_binary_refusal_keeps_borrowed_wire_and_zero_system_allocation() {
    use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️controlled/🔣️.json")).unwrap();
    let id=fixture["id"].as_str().unwrap();let version=fixture["version"].as_u64().unwrap()as u16;
    let wire=wrap_binary(&SemioEnvelope::from_envelope_id(id,Component::Pack,version).unwrap(),fixture["textBody"].as_str().unwrap().as_bytes());
    for row in fixture["binaryRefusals"].as_array().unwrap(){
        let mut bytes=wire.clone();let mut declared=id;match row["case"].as_str().unwrap(){"prefix"=>bytes[0]=0,"truncated"=>bytes[8..12].copy_from_slice(&u32::MAX.to_le_bytes()),"identity"=>declared="foreign.owner",_=>panic!("unknown original case")};
        let pointer=bytes.as_ptr();let mut accept=|_|true;let mut control=NativeDecodeControl::new(0,&mut accept);
        let(result,allocated,released)=crate::test_allocation::observe_backing(||unwrap_binary_controlled(&bytes,declared,Component::Pack,version,&mut control));
        let error:ValueError=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(error.to_string(),row["message"].as_str().unwrap());assert_eq!((allocated,released),(0,0));assert_eq!(error.retained_progress(),Default::default());assert_eq!(bytes.as_ptr(),pointer);assert_eq!(control.owned_bytes(),0);
    }
    let mut canceled=|_|false;let mut control=NativeDecodeControl::new(0,&mut canceled);let(result,allocated,released)=crate::test_allocation::observe_backing(||unwrap_binary_controlled(&wire,id,Component::Pack,version,&mut control));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!((allocated,released),(0,0));
    eprintln!("[DEBUG] Original controlled binary malformed and canceled refusals retain caller wire with zero System backing allocation");
}
