//! 🕰️ The owned WASI clocks answer real, monotonic, range-checked nanoseconds.
use super::*;

#[test]
fn microsecond_owned_wasi_clock_is_real_nanoseconds_with_checked_unsigned_range() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/⏱️budget/🕰️clock.json")).unwrap();
    for law in fixture["wasi"].as_array().unwrap() {
        let nanoseconds = law["nanoseconds"].as_str().unwrap().parse::<u128>().unwrap();
        let duration = std::time::Duration::new((nanoseconds / 1_000_000_000) as u64, (nanoseconds % 1_000_000_000) as u32);
        let result = owned_wasi_nanoseconds(duration);
        assert_eq!(result.is_ok(), law["accepted"].as_bool().unwrap());
        if let Ok(actual) = result {
            assert_eq!(u128::from(actual), nanoseconds);
        }
    }
    let read = || match owned_wasi_monotonic_clock().unwrap() {
        Value::I64(value) => u64::from_ne_bytes(value.to_ne_bytes()),
        _ => panic!("WASI clock primitive"),
    };
    let start = read();
    let mut last = start;
    for _ in 0..10_000 {
        let next = read();
        assert!(next >= last);
        last = next;
        if last > start {
            break;
        }
    }
    assert!(last > start, "owned WASI host must not return a frozen zero clock");
}

/// 🎟️ A supplied original clock bounds the same absolute deadline on every resume.
#[test]
fn original_interpreter_deadline_uses_exact_units_and_original_clock_boundary(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🎟️deadline.json")).unwrap();
 for row in fixture["duration"].as_array().unwrap(){let ms=u32::try_from(row["milliseconds"].as_u64().unwrap()).unwrap();assert_eq!(u64::from(ms)*1_000,row["microseconds"].as_u64().unwrap());}
 for row in fixture["absolute"].as_array().unwrap(){assert_eq!(original_deadline_at(Some(row["now"].as_str().unwrap().parse().unwrap()),row["deadline"].as_str().unwrap().parse().unwrap()).unwrap(),row["expired"].as_bool().unwrap());}
 assert!(matches!(original_deadline_at(None,1_000),Err(TurnFault::Host(PluginHostError::NativeIo(_)))));
 let runtime=OwnedRuntime::new(||Some(1_000));assert_eq!((runtime.original_clock)(),Some(1_000));assert!(!original_deadline_at((runtime.original_clock)(),1_001).unwrap());assert!(original_deadline_at((runtime.original_clock)(),1_000).unwrap());
 static SNAPSHOT_CLOCK:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);
 let clock=||Some(SNAPSHOT_CLOCK.load(std::sync::atomic::Ordering::Acquire));
 let runtime=OwnedRuntime::new(clock);
 let compiled=runtime.compile_component(&super::owned_runtime_tests::owned_channel_guest(semio_framework_os_kernel::CHANNEL_VERSION)).unwrap();
 let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
 let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
 let mut identity=crate::test_native_authority::original(&mut observer,&mut recipient);
 for row in fixture["snapshot"].as_array().unwrap(){
  SNAPSHOT_CLOCK.store(row["now"].as_str().unwrap().parse().unwrap(),std::sync::atomic::Ordering::Release);
  let mut state=runtime.instantiate_actor(&compiled).unwrap();let mut input=Some(Vec::new());state.begin_snapshot(OwnedSnapshotOperation::Export,&mut input).unwrap();assert!(input.is_none());
  let mut receiver=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::new(&mut identity);let mut observed=None;
  let actual=state.resume_snapshot(OwnedSnapshotOperation::Export,64,row["deadline"].as_str().unwrap().parse().unwrap(),|fuel,_|observed=Some(fuel),None,&mut receiver);
  if row["expired"].as_bool().unwrap(){assert!(matches!(actual,Err(TurnFault::DeadlineExceeded)));assert_eq!(observed,Some(0));assert_eq!(state.pending_snapshot().unwrap(),Some((OwnedSnapshotOperation::Export,0)));}else{assert!(matches!(actual,Err(TurnFault::Trapped(_))));}
 }
 crate::test_native_authority::close(&mut identity);
 eprintln!("[DEBUG] original interpreter exactMsToUs=true absoluteBoundary=true missingClockRefused=true noRenewal=true originalSource=true actualSnapshotResume=true deniedSnapshotPendingPreserved=true");
}
