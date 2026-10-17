/// 🔒️ Close registry and queue shutdown: the modulo collision and contention of the fixed close registry and the shutdown release of handles, pages, create fields, surfaces and events.

#[test]
fn fixed_kernel_close_registry_returns_the_exact_modulo_collision_and_reuses_only_after_terminal_generation() {
    let registry = Arc::new(KernelCloseSubmissionRegistry::new());
    let first = close_submission(&registry, 3, 11);
    assert!(first.try_admit());
    assert!(registry.contains(3, 11));
    let collision = close_submission(&registry, 3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32, 12);
    assert!(!collision.try_admit());
    assert_eq!(collision.instance, 3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32);
    assert_eq!(collision.generation, 12);
    assert!(registry.contains(3, 11));
    first.finish(KernelCloseStatus::Complete);
    assert!(!registry.contains(3, 11));
    assert!(collision.try_admit());
    assert!(registry.contains(3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32, 12));
}

#[test]
fn fixed_kernel_close_registry_contention_returns_unadmitted_owner_for_exact_retry() {
    let registry = Arc::new(KernelCloseSubmissionRegistry::new());
    let owner = close_submission(&registry, 8, 21);
    let guard = registry.slots.lock().unwrap();
    assert!(!owner.try_admit());
    assert_eq!(owner.phase.load(std::sync::atomic::Ordering::Acquire), KERNEL_CLOSE_UNADMITTED);
    assert_eq!(owner.instance, 8);
    assert_eq!(owner.generation, 21);
    drop(guard);
    assert!(owner.try_admit());
    assert!(registry.contains(8, 21));
}

#[test]
fn fixed_kernel_request_shutdown_faults_the_retained_close_handle_before_terminal_removal() {
    let queue = KernelRequestQueue::default();
    let owner = match destroy_request(19) {
        KernelRequest::DestroyApp { owner } => owner,
        _ => unreachable!(),
    };
    queue.try_push(KernelRequest::DestroyApp { owner: owner.clone() }, Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
    assert!(queue.begin_shutdown());
    assert_eq!(queue.shutdown_step(0), (true, 1, 0));
    assert_eq!(owner.terminal_status(), Some(KernelCloseStatus::Fault));
}

#[test]
fn fixed_kernel_request_queue_shutdown_releases_one_real_page_per_grant() {
    let queue = KernelRequestQueue::default();
    queue.try_push(command_request(3, 12, 2), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
    assert!(queue.begin_shutdown());
    assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1), (false, 0, 0));
    assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (false, 1, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES));
    assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 4));
}

#[test]
fn fixed_kernel_request_queue_shutdown_releases_create_fields_one_owner_per_grant() {
    let queue = KernelRequestQueue::default();
    queue
        .try_push(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(PathBuf::from("path"), "plugin".to_string(), "app".to_string(), "schema".to_string()) }, Arc::new(ResponseSlot::default()), None)
        .unwrap_or_else(|_| panic!("fixture request queue admission"));
    assert!(queue.begin_shutdown());
    assert_eq!(queue.shutdown_step(3), (false, 0, 0));
    assert_eq!(queue.shutdown_step(4), (false, 1, 4));
    assert_eq!(queue.shutdown_step(6), (false, 1, 6));
    assert_eq!(queue.shutdown_step(3), (false, 1, 3));
    assert_eq!(queue.shutdown_step(6), (true, 1, 6));
}

#[test]
fn fixed_kernel_request_queue_shutdown_releases_surface_and_rejected_events_in_fifo_units() {
    let queue = KernelRequestQueue::default();
    queue
        .try_push(
            KernelRequest::Exchange {
                instance: 1,
                event: QueuedKernelEvent {
                    kind: Some(QueuedKernelEventKind::SurfaceVisible),
                    surface_visible: Some("surface".to_string()),
                    surface_body_key: None,
                    surface_view_state: None,
                    message_endpoint: None,
                    message_payload: None,
                },
            },
            Arc::new(ResponseSlot::default()),
            None,
        )
        .unwrap_or_else(|_| panic!("fixture request queue admission"));
    queue
        .try_push(KernelRequest::CloseRejectedEvents { owner: RejectedKernelEvents { events: std::collections::VecDeque::from([Event::Wake, Event::Wake]) } }, Arc::new(ResponseSlot::default()), None)
        .unwrap_or_else(|_| panic!("fixture request queue admission"));
    assert!(queue.begin_shutdown());
    assert_eq!(queue.shutdown_step(6), (false, 0, 0));
    assert_eq!(queue.shutdown_step(7), (false, 1, 7));
    assert_eq!(queue.shutdown_step(0), (false, 1, 0));
    assert_eq!(queue.shutdown_step(0), (true, 1, 0));
}
