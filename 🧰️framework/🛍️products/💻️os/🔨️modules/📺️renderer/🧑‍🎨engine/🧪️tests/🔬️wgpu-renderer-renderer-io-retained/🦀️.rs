use super::*;

struct RendererIoPollWake {
    count: std::sync::atomic::AtomicUsize,
}

impl std::task::Wake for RendererIoPollWake {
    fn wake(self: Arc<Self>) {
        self.count.fetch_add(1, Ordering::AcqRel);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.count.fetch_add(1, Ordering::AcqRel);
    }
}

/// 🪢 A first poll contending with the exact mounted-session owner retains a runnable future.
#[test]
fn a_contended_renderer_io_poll_wakes_itself_until_the_exact_session_can_register() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).expect("native asset contract");
    let contention = &contract["rendererIoContention"];
    assert_eq!(contention["registrationState"].as_str(), Some("checkedOut"));
    assert_eq!(contention["exactGenerationLive"].as_bool(), Some(true));
    assert_eq!(contention["poll"].as_str(), Some("pending"));
    let mut handle = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ProcessResidentBytes).expect("mounted native I/O handle");
    let slot = &RENDERER_IO_SLOTS[handle.slot];
    assert_eq!(slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire), Ok(RENDERER_IO_LIVE));
    let wake = Arc::new(RendererIoPollWake { count: std::sync::atomic::AtomicUsize::new(0) });
    let waker = std::task::Waker::from(wake.clone());
    let mut context = std::task::Context::from_waker(&waker);
    assert!(std::pin::Pin::new(&mut handle).poll(&mut context).is_pending());
    assert_eq!(wake.count.load(Ordering::Acquire), contention["wakeCount"].as_u64().expect("wake count") as usize, "contended registration preserves a runnable owner");
    slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
    drop(handle);
    for _ in 0..16 {
        let _ = pump_renderer_io_sessions(1);
    }
    assert_eq!(RENDERER_IO_SLOTS.iter().filter(|slot| slot.state.load(Ordering::Acquire) == RENDERER_IO_LIVE).count(), contention["terminalOwners"].as_u64().expect("terminal owner count") as usize);
}

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
