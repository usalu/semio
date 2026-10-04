use super::*;

#[test]
fn retained_text_envelope_preserves_original_body_and_bounds_resume_cancel_and_release() {
    use semio_framework_value::{NativeEncodeControl, SnapshotRetirementStep, ValueRefusalKind};
    use semio_framework_value::retirement::owned_retirement;
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️controlled/🔣️.json")).unwrap();
    let body=fixture["textBody"].as_str().unwrap().repeat(fixture["retained"]["textRepeats"].as_u64().unwrap() as usize);
    let id=fixture["id"].as_str().unwrap(); let version=fixture["version"].as_u64().unwrap() as u16;
    let expected=wrap_text(&SemioEnvelope::from_envelope_id(id,Component::Dsl,version).unwrap(),&body);
    let grant=fixture["retained"]["retirementBytes"].as_u64().unwrap() as usize;
    for budget in fixture["retained"]["budgets"].as_array().unwrap() {
        let source=body.clone(); let pointer=source.as_ptr();
        let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,source);
        assert_eq!(cursor.source_body().unwrap().as_ptr(),pointer);
        let mut accept=|_|true; let mut control=NativeEncodeControl::new(1_000_000,&mut accept);
        assert!(cursor.step(1,grant,&mut control).unwrap().is_none());
        let output=loop { let before=cursor.position(); assert!(cursor.step(0,0,&mut control).unwrap().is_none()); assert_eq!(cursor.position(),before); if let Some(output)=cursor.step(budget.as_u64().unwrap() as usize,grant,&mut control).unwrap() { break output; } };
        assert_eq!(output,expected); assert!(cursor.source_body().is_none());
        let mut close=owned_retirement(cursor); while !close.terminal_is_empty() { if let SnapshotRetirementStep::Pending {released_bytes,..}=close.close_step(1,grant).unwrap() { assert!(released_bytes<=grant); } }
        let mut close=owned_retirement(output); while !close.terminal_is_empty() { close.close_step(1,grant).unwrap(); }
    }
    for stop in [0,1,8,64,256,1024] {
        let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body.clone());
        let live=std::cell::Cell::new(true); let mut accept=|_|live.get(); let mut control=NativeEncodeControl::new(1_000_000,&mut accept);
        for _ in 0..stop { assert!(cursor.step(1,grant,&mut control).unwrap().is_none()); }
        let before=cursor.position(); live.set(false); assert_eq!(cursor.step(1,grant,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled); assert_eq!(cursor.position(),before);
        let mut close=owned_retirement(cursor); assert!(matches!(close.close_step(0,0).unwrap(),SnapshotRetirementStep::Pending {released_items:0,released_bytes:0})); while !close.terminal_is_empty() { if let SnapshotRetirementStep::Pending {released_bytes,..}=close.close_step(1,grant).unwrap() { assert!(released_bytes<=grant); } }
    }
    let mut cursor=RetainedTextEnvelope::new(id.into(),Component::Dsl,version,body); let mut accept=|_|true; let mut control=NativeEncodeControl::new(1,&mut accept); assert_eq!(cursor.step(1,grant,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit); let mut close=owned_retirement(cursor); while !close.terminal_is_empty() { close.close_step(1,grant).unwrap(); }
    let script="const x=JSON.parse(await Bun.stdin.text());const bytes=new TextEncoder().encode('semio '+x.id+'.dsl v'+x.version+'\\n'+x.textBody.repeat(x.retained.textRepeats));await Bun.write(Bun.stdout,bytes)";
    use std::{io::Write,process::{Command,Stdio}}; let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(fixture.to_string().as_bytes()).unwrap(); let result=child.wait_with_output().unwrap(); assert!(result.status.success()); assert_eq!(result.stdout,expected.as_bytes());
    eprintln!("[DEBUG] Original declared Text body pointer retained; budgets1/8/256 zero/cancel/ownership refusal and3byte release; independent TextEncoder bytes identical");
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
