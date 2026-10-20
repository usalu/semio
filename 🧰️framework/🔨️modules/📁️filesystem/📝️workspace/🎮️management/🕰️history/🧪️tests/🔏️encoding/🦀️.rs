//! 📐️ Actual compact encoding fits the independent UTF8/SQLite extent oracle.
use super::*;
#[test]
fn history_reconciliation_maximum_actual_encoding_fits_reserved_capacity(){
 let mut rows=(0..4096).map(|index|Settlement{id:format!("{}{:016x}","f".repeat(48),index),intent:"f".repeat(64),status:'o',identity:(u64::MAX,u64::MAX),directory:true,fingerprint:"f".repeat(64)}).collect::<Vec<_>>();let bytes=encode_reconciliation(&"f".repeat(64),usize::MAX,&rows).unwrap();let expected_header=format!("source-history-reconciliation-v1\n{}\n{}\n4096\n","f".repeat(64),usize::MAX);assert_eq!(bytes.len(),987136+expected_header.len());assert!(bytes.len()<=1_048_576);let external=serde_json::to_value(std::str::from_utf8(&bytes).unwrap()).unwrap();assert_eq!(external.as_str().unwrap().as_bytes(),bytes);rows[0].id="🦑".into();assert_eq!(encode_reconciliation(&"f".repeat(64),0,&rows).unwrap_err().kind,ErrorKind::Boundary);
}
