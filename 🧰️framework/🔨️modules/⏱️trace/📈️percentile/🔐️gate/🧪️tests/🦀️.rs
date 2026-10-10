//! 🧪️ A fresh original static gate admits one exclusive native borrow without physical lock backing.
use super::*;
#[test]
fn original_static_telemetry_gate_preserves_exclusive_custody_and_zero_native_birth(){
 let original=StaticTryGate::new(17u64);let mut guard=original.try_lock().unwrap();assert_eq!(*guard,17);assert!(original.try_lock().is_none());*guard=19;drop(guard);let guard=original.try_lock().unwrap();assert_eq!(*guard,19);assert!(original.try_lock().is_none());drop(guard);assert!(original.try_lock().is_some());eprintln!("[DEBUG] actual original static telemetry gate nativeInlineBacking exclusiveAcquire deniedSameOwner releaseAcquire metadataCopy0");
}
