
use super::*;

fn payload_vec(mut payload: semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
        bytes.extend_from_slice(page);
    }
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    bytes
}

fn run(request: NativeIoRequest) -> Result<NativeIoValue, String> {
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::allocate_operation_id(),
        generation: semio_framework_job::Generation(1),
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig {
            site: "native_io_test",
            stage: semio_framework_job::InteractiveStage::InteractiveStep,
            fuel_per_step: semio_framework_job::INTERACTIVE_LANE_FUEL,
            step_budget_us: semio_framework_job::INTERACTIVE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = semio_framework_job::BatchJobSession::try_new(NativeIoJob::new(request), params).unwrap_or_else(|_| panic!("native I/O test session admission"));
    let result;
    loop {
        if session.step().is_err() {
            panic!("native I/O test session contention");
        }
        let Some(job) = session.checked_out_job_mut() else { panic!("native I/O checked-out job") };
        let terminal_result = job.take_result();
        let Some(mut outcome) = session.take_outcome() else { panic!("native I/O retained outcome") };
        let terminal = outcome.is_terminal();
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            result = terminal_result.unwrap_or_else(|| panic!("native I/O terminal result"));
            break;
        }
        if session.resume().is_err() {
            panic!("native I/O test session resume");
        }
    }
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    result
}

#[test]
fn request_and_completion_are_send() {
    fn assert_send<T: Send>() {}
    assert_send::<NativeIoJob>();
}

#[test]
fn resident_memory_observation_does_not_spawn_a_process() {
    let value = process_resident_bytes();
    assert!(value.is_none() || value.is_some_and(|bytes| bytes > 0));
}

#[test]
fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact() {
    let mut paths = NativePathSet::new();
    for index in 0..NATIVE_IO_PATH_CAPACITY {
        paths.try_push(PathBuf::from(format!("/retained-native-path-{index:04}"))).expect("fixed path capacity");
    }
    let plus_one = PathBuf::from("/retained-native-path-plus-one");
    let plus_one_pointer = plus_one.as_os_str().as_encoded_bytes().as_ptr();
    let returned = paths.try_push(plus_one).expect_err("maximum plus one returns exact path owner");
    assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer);
    drop(returned);
    let mut job = NativeIoJob::new(NativeIoRequest::Modified(paths));
    job.begin_close();
    assert_eq!(job.close_step(0, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    let mut released = 0;
    while !job.terminal_is_empty() {
        if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } = job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            assert!(released_items <= 1);
            released += released_items;
        }
    }
    assert!(released >= NATIVE_IO_PATH_CAPACITY);
}

#[test]
fn chunked_read_write_scan_and_modified_round_trip() {
    let root = std::env::temp_dir().join(format!("semio-native-io-{}", std::process::id()));
    let path = root.join("fixture.wasm");
    let bytes = vec![0xA5; 96 * 1024 + 17];
    std::fs::create_dir_all(&root).expect("test fixture directory");
    std::fs::write(&path, &bytes).expect("test-only filesystem oracle");
    assert!(run(NativeIoRequest::ReadBytes(path.clone())).is_err());
    let NativeIoValue::Page { bytes: first, eof: false } = run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 16 * 1024 }).unwrap() else { panic!("first page") };
    assert_eq!(payload_vec(first), bytes[..16 * 1024]);
    let offset = (bytes.len() - 7) as u64;
    let NativeIoValue::Page { bytes: last, eof: true } = run(NativeIoRequest::ReadPage { path: path.clone(), offset, max_bytes: 16 * 1024 }).unwrap() else { panic!("last page") };
    assert_eq!(payload_vec(last), bytes[bytes.len() - 7..]);
    assert!(run(NativeIoRequest::ReadPage { path: path.clone(), offset: 0, max_bytes: 64 * 1024 + 1 }).is_err());
    let NativeIoValue::Paths(mut paths) = run(NativeIoRequest::ScanDirectory { path: root.clone(), directories_only: false, extension: Some("wasm".into()), first_only: true }).unwrap() else { panic!("scan value") };
    assert_eq!(paths.pop(), Some(path.clone()));
    let mut modified_paths = NativePathSet::new();
    modified_paths.try_push(path).expect("one modified path");
    let NativeIoValue::Modified(mut modified) = run(NativeIoRequest::Modified(modified_paths)).unwrap() else { panic!("modified value") };
    assert_eq!(modified.len(), 1);
    drop(modified.pop());
    std::fs::remove_dir_all(root).unwrap();
}
