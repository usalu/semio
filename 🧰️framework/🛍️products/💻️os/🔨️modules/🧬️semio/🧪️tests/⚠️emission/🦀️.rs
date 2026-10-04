//! 🧬️ Controlled envelope emission retains owned refusal kinds and exact framing.
use semio_framework_os_kernel::os_semio::{declared_envelope_prefix_len,wrap_binary_controlled,wrap_text_controlled,Component};
use semio_framework_value::{NativeEncodeControl,ValueError};
#[test]
fn controlled_envelope_emission_retains_typed_refusal_and_exact_wire() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️emission/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let id=row["id"].as_str().unwrap();
        let version=u16::try_from(row["version"].as_u64().unwrap()).unwrap();
        let component=match row["component"].as_str().unwrap() {"dsl"=>Component::Dsl,"pack"=>Component::Pack,_=>panic!("closed fixture component")};
        let operation=row["operation"].as_str().unwrap();
        let mut progress=|_|row["accept"].as_bool().unwrap();
        let mut control=NativeEncodeControl::new(usize::try_from(row["maximumBytes"].as_u64().unwrap()).unwrap(),&mut progress);
        let result:Result<Vec<u8>,ValueError>=match operation {
            "prefix"=>declared_envelope_prefix_len(id,component,version).map(|count|count.to_string().into_bytes()),
            "binary"=>wrap_binary_controlled(id,component,version,row["body"].as_str().unwrap().as_bytes(),&mut control),
            "text"=>wrap_text_controlled(id,component,version,row["body"].as_str().unwrap(),&mut control).map(String::into_bytes),
            _=>panic!("closed fixture operation"),
        };
        if let Some(kind)=row["expected"].get("kind") {
            let error=result.expect_err("fixture refusal");
            assert_eq!(error.kind.as_str(),kind.as_str().unwrap(),"{}",row["name"]);
            assert_eq!(error.message,row["expected"]["message"].as_str().unwrap(),"{}",row["name"]);
        } else {
            let bytes=result.unwrap();
            let hex=bytes.iter().map(|byte|format!("{byte:02x}")).collect::<String>();
            assert_eq!(hex,row["expected"]["hex"].as_str().unwrap(),"{}",row["name"]);
            assert_eq!(declared_envelope_prefix_len(id,component,version).unwrap(),usize::try_from(row["expected"]["prefixBytes"].as_u64().unwrap()).unwrap());
            assert_eq!(control.owned_bytes(),bytes.len());
        }
    }
}
