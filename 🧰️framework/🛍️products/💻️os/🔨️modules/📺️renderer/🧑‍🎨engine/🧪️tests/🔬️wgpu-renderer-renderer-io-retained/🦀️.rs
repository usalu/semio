
use super::*;

#[test]
fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact() {
    let mut handles = Vec::with_capacity(RENDERER_IO_SESSION_SLOTS);
    for index in 0..RENDERER_IO_SESSION_SLOTS {
        let path = std::path::PathBuf::from(format!("/semio-retained-native-io-{index:04}"));
        let request_pointer = path.as_os_str().as_encoded_bytes().as_ptr();
        let handle = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(path)).expect("logical maximum plus one has a mounted rejection owner");
        if index + 1 == RENDERER_IO_SESSION_SLOTS {
            let plus_one_request_pointer = request_pointer;
            let returned_pointer =
                renderer_io_with_node(handle.slot, handle.generation, |node| node.rejected.as_ref().and_then(|rejected| rejected.job().retained_request_backing_identity()).expect("maximum plus one retains the exact rejected request"))
                    .expect("maximum plus one mounted generation");
            assert_eq!(returned_pointer, plus_one_request_pointer);
        }
        handles.push(handle);
    }
    assert_eq!(pump_renderer_io_sessions(0), 0);
    let first_slot = handles[0].slot;
    let first_generation = handles[0].generation;
    drop(handles);
    assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity");
    for _ in 0..RENDERER_IO_SESSION_SLOTS * 16 {
        if RENDERER_IO_SLOTS.iter().all(|slot| slot.state.load(Ordering::Acquire) != RENDERER_IO_LIVE) {
            break;
        }
        assert!(pump_renderer_io_sessions(1) <= 1);
    }
    assert!(RENDERER_IO_SLOTS.iter().all(|slot| slot.state.load(Ordering::Acquire) != RENDERER_IO_LIVE));
    assert!(!renderer_io_generation_live(first_slot, first_generation));
    let replacement = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ProcessResidentBytes).expect("closed registry slot is reusable with a new generation");
    assert_eq!(replacement.slot, first_slot);
    assert!(replacement.generation > first_generation);
    drop(replacement);
    for _ in 0..16 {
        let _ = pump_renderer_io_sessions(1);
    }
}
