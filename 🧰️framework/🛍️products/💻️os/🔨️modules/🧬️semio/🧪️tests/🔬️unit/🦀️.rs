use super::*;

/// 🎟️ Funds each original owner demand independently and retains ownership across denied grants.
fn retire_original_semio_owner<T:semio_framework_value::retirement::RetireOwned>(value:T,copy_bytes:usize,maximum_depth:usize)->semio_framework_value::retained_clone::RetainedCloneProgress{
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
    let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("original semio retirement owner: {error}"));
    assert!(matches!(owner.step(Default::default()).unwrap(),RetainedCloneStep::Progress(progress)if progress==RetainedCloneProgress::default()));
    assert_eq!(owner.step(RetainedCloneGrant{maximum_items:1,..Default::default()}).unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert!(owner.original().is_some());
    assert!(matches!(owner.step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy_bytes,maximum_depth,..Default::default()}).unwrap(),RetainedCloneStep::Progress(progress)if progress==RetainedCloneProgress::default()));
    assert!(owner.original().is_some());
    let mut total=RetainedCloneProgress::default();let mut turns=0;
    while !owner.terminal_is_empty(){turns+=1;assert!(turns<100000);let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let capacity=owner.next_capacity_byte_demand(if copy==0{release}else{copy_bytes}).unwrap();let depth=owner.next_depth_demand().unwrap();assert!(depth<=maximum_depth);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy_bytes,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};let progress=match owner.step(grant).unwrap(){RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert!(progress.fits(grant));total.copied_items+=progress.copied_items;total.copied_bytes+=progress.copied_bytes;total.retained_capacity_bytes+=progress.retained_capacity_bytes;total.released_bytes+=progress.released_bytes;}
    total
}

#[test]
fn retained_text_envelope_preserves_original_body_and_bounds_resume_cancel_and_release() {
    use semio_framework_value::{NativeEncodeControl, ValueRefusalKind};
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️controlled/🔣️.json")).unwrap();
    let body=fixture["textBody"].as_str().unwrap().repeat(fixture["retained"]["textRepeats"].as_u64().unwrap() as usize);
    let id=fixture["id"].as_str().unwrap(); let version=fixture["version"].as_u64().unwrap() as u16;
    let expected=wrap_text(&SemioEnvelope::from_envelope_id(id,Component::Dsl,version).unwrap(),&body);
    let grant=fixture["retained"]["retirementBytes"].as_u64().unwrap() as usize;
    let depth=fixture["retained"]["maximumRetirementDepth"].as_u64().unwrap()as usize;let ownership=&fixture["retained"]["ownership"];assert_eq!(ownership["copy"],"bounded-original-payload");assert_eq!(ownership["capacity"],"exact-natural-birth");assert_eq!(ownership["release"],"exact-physical-allocation");assert_eq!(ownership["depth"],"exact-original-frontier");assert_eq!(ownership["refusal"],"original-owner-kept");
    for budget in fixture["retained"]["budgets"].as_array().unwrap() {
        let source=body.clone(); let pointer=source.as_ptr();
        let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,source);
        assert_eq!(cursor.source_body().unwrap().as_ptr(),pointer);
        let mut accept=|_|true; let mut control=NativeEncodeControl::new(1_000_000,&mut accept);
        assert!(cursor.step(1,grant,&mut control).unwrap().is_none());
        let output=loop { let before=cursor.position(); assert!(cursor.step(0,0,&mut control).unwrap().is_none()); assert_eq!(cursor.position(),before); if let Some(output)=cursor.step(budget.as_u64().unwrap() as usize,grant,&mut control).unwrap() { break output; } };
        assert_eq!(output,expected); assert!(cursor.source_body().is_none());
        assert!(cursor.retirement.is_none());retire_original_semio_owner(cursor,grant,depth);
        let output_capacity=output.capacity();let progress=retire_original_semio_owner(output,grant,depth);assert!(progress.released_bytes>=output_capacity,"the exact physical output allocation is fully returned independently of the small copy grant");
    }
    for stop in [0,1,8,64,256,1024] {
        let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body.clone());
        let live=std::cell::Cell::new(true); let mut accept=|_|live.get(); let mut control=NativeEncodeControl::new(1_000_000,&mut accept);
        for _ in 0..stop { assert!(cursor.step(1,grant,&mut control).unwrap().is_none()); }
        let before=cursor.position(); live.set(false); assert_eq!(cursor.step(1,grant,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled); assert_eq!(cursor.position(),before);
        let sources=cursor.source_body().map_or(0,|body|body.len());let progress=retire_original_semio_owner(cursor,grant,depth);assert!(progress.released_bytes>=sources);
    }
    let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body);let pointer=cursor.source_body().unwrap().as_ptr();let mut accept=|_|true; let mut control=NativeEncodeControl::new(1,&mut accept); assert_eq!(cursor.step(1,grant,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(cursor.source_body().unwrap().as_ptr(),pointer);retire_original_semio_owner(cursor,grant,depth);
    let script="const x=JSON.parse(await Bun.stdin.text());const bytes=new TextEncoder().encode('semio '+x.id+'.dsl v'+x.version+'\\n'+x.textBody.repeat(x.retained.textRepeats));await Bun.write(Bun.stdout,bytes)";
    use std::{io::Write,process::{Command,Stdio}}; let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(fixture.to_string().as_bytes()).unwrap(); let result=child.wait_with_output().unwrap(); assert!(result.status.success()); assert_eq!(result.stdout,expected.as_bytes());
    eprintln!("[DEBUG] Original declared Text body pointer retained; budgets1/8/256 zero/cancel/ownership refusal and3byte payload work; exact separate capacity/full physical release/depth; independent TextEncoder bytes identical");
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
