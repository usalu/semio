use super::*;
#[test]
fn retained_pack_required_chunked_matches_actual_custody_and_independent_crc(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧱️flags/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let flags=row["requiredFlags"].as_u64().unwrap()as u32;let header=Header{version_major:FORMAT_VERSION_MAJOR,version_minor:FORMAT_VERSION_MINOR,required_flags:flags,optional_flags:OPTIONAL_CANONICAL}.write_bytes_sync();let expected:Vec<u8>=row["headerCrcHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();assert_eq!(&header[20..24],expected);let result=validate_chunk_custody(flags,row["chunkCount"].as_u64().unwrap(),ByteRange{offset:if row["tableBytes"]==0{0}else{32},len:row["tableBytes"].as_u64().unwrap()});assert_eq!(result.is_ok(),row["valid"].as_bool().unwrap());if let Err(reason)=result{assert!(matches!(reason,PackRefusal::RetainedMalformed{what:"required_chunked",..}));}}
}
