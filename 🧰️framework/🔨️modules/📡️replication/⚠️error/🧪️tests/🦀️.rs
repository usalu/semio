//! 🧪️ The canonical nominal protocol error matches portable independent error messages.
use crate::ProtocolError;
use semio_framework_pack_error::{PackError,PackRefusal};
use semio_framework_test_fixtures::fixture_text;
use std::error::Error;
#[test]
fn nominal_protocol_error_display_matches_portable_sqlite_and_serde_contract(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let first=row["first"].as_u64().unwrap();let second=row["second"].as_u64().unwrap();let text=row["text"].as_str().unwrap();
  let error=match row["kind"].as_str().unwrap(){
   "chain"=>ProtocolError::ChainMismatch{commit_seq:first},"tail"=>ProtocolError::TornTail(first),"critical"=>ProtocolError::UnknownCriticalRecord(first as u8),"dictionary"=>ProtocolError::DictMiss(first as u32),"order"=>ProtocolError::DictOutOfOrder{expected:first as u32,actual:second as u32},"verifier"=>ProtocolError::VerifierRequired,"signature"=>ProtocolError::SignatureInvalid{commit_seq:first},"frame"=>ProtocolError::FrameFraming(first),"limit"=>ProtocolError::LimitExceeded("frames"),"malformed"=>ProtocolError::Malformed{what:"wire",offset:first,detail:text.to_owned()},"io"=>ProtocolError::Io(text.to_owned()),_=>panic!("unknown neutral error"),
  };
  let actual=error.to_string();assert_eq!(actual,row["message"].as_str().unwrap());assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&actual).unwrap()).unwrap(),actual);assert!(error.source().is_none());
 }
}
#[test]
fn pack_conversion_exposes_the_original_pack_refusal_as_system_source(){
 let refusal=PackRefusal::BadMagic;let error:ProtocolError=refusal.into();assert!(matches!(error,ProtocolError::Pack(PackError::Refusal(PackRefusal::BadMagic))));assert!(error.source().is_some());
 let original=PackError::Refusal(PackRefusal::Truncated(17));let expected=original.to_string();let error:ProtocolError=original.into();assert_eq!(error.to_string(),expected);assert_eq!(error.source().unwrap().to_string(),expected);
}
