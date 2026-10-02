use semio_framework_os_kernel::{native_decoding::*,os_semio::*};

#[test]
fn sqlite_snapshot_native_envelope_borrows_admitted_payload() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️controlled/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🚦️controlled/🔣️.json")).unwrap();
    let id=fixture["id"].as_str().unwrap();let version=fixture["version"].as_u64().unwrap() as u16;
    let envelope=SemioEnvelope::from_envelope_id(id,Component::Pack,version).unwrap();let body=vec![127;fixture["bodyBytes"].as_u64().unwrap() as usize];let bytes=wrap_binary(&envelope,&body);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(0,&mut accepted);
    let borrowed=unwrap_binary_controlled(&bytes,id,Component::Pack,version,&mut control).unwrap();assert_eq!(borrowed,body);assert_eq!(borrowed.as_ptr(),bytes[bytes.len()-body.len()..].as_ptr());assert_eq!(control.owned_bytes(),0);
    for (identity,component,v) in [("foreign.owner",Component::Pack,version),(id,Component::Spr,version),(id,Component::Pack,version+1)]{assert!(unwrap_binary_controlled(&bytes,identity,component,v,&mut control).is_err());}
    let mut malformed=bytes.clone();malformed[8..12].copy_from_slice(&u32::MAX.to_le_bytes());assert!(unwrap_binary_controlled(&malformed,id,Component::Pack,version,&mut control).is_err());
    let text_body=fixture["textBody"].as_str().unwrap();let text=wrap_text(&SemioEnvelope::from_envelope_id(id,Component::Dsl,version).unwrap(),text_body);
    assert_eq!(split_text_preamble_controlled(&text,id,Component::Dsl,version,&mut control).unwrap(),text_body);assert_eq!(control.owned_bytes(),0);
    assert!(split_text_preamble_controlled(&text,id,Component::Op,version,&mut control).is_err());
    let mut canceled=|_:NativeDecodeProgress|false;let mut control=NativeDecodeControl::new(0,&mut canceled);assert!(unwrap_binary_controlled(&bytes,id,Component::Pack,version,&mut control).is_err());assert!(split_text_preamble_controlled(&text,id,Component::Dsl,version,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
    let script="import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const b=Buffer.from(x.bytes,'base64'),n=b.readUInt32LE(8),token=b.subarray(12,12+n).toString('utf8');if(token!==x.fixture.id+'.pack v'+x.fixture.version)throw Error('identity');await Bun.write(Bun.stdout,String(b.subarray(12+n).length));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture,"bytes":protocol::bytes::encode_base64(&bytes)}).to_string().as_bytes()).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));assert_eq!(String::from_utf8(result.stdout).unwrap(),body.len().to_string());
}
