//! 🔒️ Canonical private receipt corruption preserves the original closed accounting boundary.
use super::*;
use std::sync::atomic::{AtomicUsize,Ordering};
#[test]
fn original_forwarded_receipt_refuses_corrupt_private_accounting_without_consuming_original_ports(){
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let ceiling=neutral["observer"]["ceiling"].as_u64().unwrap()as usize;
 let denied_calls=AtomicUsize::new(0);let mut denied_port=|_:NativeEncodeAllocation|{denied_calls.fetch_add(1,Ordering::Relaxed);Ok(())};let mut invalid=crate::native_encoding::NativeForwardedEncodeContinuation::new(ceiling,&mut denied_port);invalid.receipt.as_mut().unwrap().owned_bytes=ceiling+1;let mut denied_observer=|_|true;assert_eq!(invalid.encode(&mut denied_observer,|control|control.charge(1)).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(invalid.receipt.as_ref().unwrap().owned_bytes,ceiling+1);let(error,invalid)=match NativeForwardedEncodeControl::resume(invalid,&mut denied_observer){Err(refused)=>refused,Ok(_)=>panic!("invalid original forwarded receipt must refuse before consumption")};assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(invalid.receipt.as_ref().unwrap().owned_bytes,ceiling+1);assert_eq!(invalid.maximum_bytes(),ceiling);assert_eq!(denied_calls.load(Ordering::Relaxed),0);drop(invalid);println!("[DEBUG] Forwarded original resume refusal returnsSamePorts=true invalidLedgerPreserved=true originalPortCalls=0");
}
