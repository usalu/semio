use super::serve_on_worker_stack;

const PROBED_BYTES: usize = 12 * 1024 * 1024;

#[inline(never)]
fn descend(remaining: usize) -> usize {
    let page = std::hint::black_box([1u8; 64 * 1024]);
    if remaining <= page.len() { page[0] as usize } else { descend(remaining - page.len()) + page[page.len() - 1] as usize }
}

#[test]
fn serving_thread_holds_more_than_any_main_stack() {
    let (sender, receiver) = std::sync::mpsc::channel();
    serve_on_worker_stack(move || sender.send(descend(PROBED_BYTES)).unwrap());
    assert_eq!(receiver.recv().unwrap(), PROBED_BYTES / (64 * 1024));
}
