mod streaming_tests {
    use super::*;
    use semio_framework_async::{CancelToken, ScopeOwner, TraceId};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockReader {
        bytes: Vec<u8>,
        cursor: usize,
        page: usize,
        reads: Arc<AtomicUsize>,
        fail_at: Option<usize>,
    }

    impl Read for MockReader {
        fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
            let turn = self.reads.fetch_add(1, Ordering::SeqCst);
            if self.fail_at == Some(turn) {
                return Err(std::io::Error::other("scripted reader failure"));
            }
            if self.cursor == self.bytes.len() {
                return Ok(0);
            }
            let count = self.page.min(output.len()).min(self.bytes.len() - self.cursor);
            output[..count].copy_from_slice(&self.bytes[self.cursor..self.cursor + count]);
            self.cursor += count;
            Ok(count)
        }
    }

    fn mock_reader(bytes: Vec<u8>, page: usize, reads: Arc<AtomicUsize>, fail_at: Option<usize>) -> Arc<Mutex<Option<UreqBodyReader>>> {
        Arc::new(Mutex::new(Some(Box::new(MockReader { bytes, cursor: 0, page, reads, fail_at }))))
    }

    #[test]
    fn https_owned_reader_yields_one_bounded_page_per_pull_and_reaches_terminal() {
        let reads = Arc::new(AtomicUsize::new(0));
        let reader = mock_reader(vec![1, 2, 3, 4, 5, 6, 7], 3, reads.clone(), None);
        assert_eq!(reads.load(Ordering::SeqCst), 0);
        assert_eq!(ureq_stream_read_page(&reader).expect("first page"), Some(vec![1, 2, 3]));
        assert_eq!(reads.load(Ordering::SeqCst), 1);
        assert_eq!(ureq_stream_read_page(&reader).expect("second page"), Some(vec![4, 5, 6]));
        assert_eq!(ureq_stream_read_page(&reader).expect("third page"), Some(vec![7]));
        assert_eq!(ureq_stream_read_page(&reader).expect("eof"), None);
        assert!(reader.lock().expect("reader lock").is_none());
    }

    #[test]
    fn https_owned_reader_reports_partial_failure_without_an_implicit_retry() {
        let reads = Arc::new(AtomicUsize::new(0));
        let reader = mock_reader(vec![1, 2, 3, 4], 2, reads.clone(), Some(1));
        assert_eq!(ureq_stream_read_page(&reader).expect("first page"), Some(vec![1, 2]));
        assert!(matches!(ureq_stream_read_page(&reader), Err(HttpPoolError::Transport(message)) if message == "scripted reader failure"));
        assert_eq!(reads.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn https_owned_reader_honours_cancel_before_pulling_the_next_page() {
        let runtime = Arc::new(TokioHostRuntime::new());
        let scope = runtime.open_scope_now(ScopeOwner::Service("ureq_stream_test"), None);
        let compute = Arc::new(runtime.block_on(ComputePool::new(1)));
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let reads = Arc::new(AtomicUsize::new(0));
        let reader = mock_reader(vec![1, 2, 3], 1, reads.clone(), None);
        let mut body = UreqStreamingHttpBody { reader, compute, runtime: runtime.clone(), scope, ctx: OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel, capability: None } };
        let result = runtime.block_on(body.next_chunk());
        assert!(matches!(result, Err(HttpPoolError::Transport(message)) if message == "ureq HTTP body cancelled"));
        assert_eq!(reads.load(Ordering::SeqCst), 0);
    }
}
