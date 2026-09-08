mod tests {
    use super::*;

    /// 📜️ `from_next`/`next` are the pair `Body::from_seed`/`Body::to_seed` use to carry the label
    /// high-water-mark forward across a rebuild instead of restarting at 0 (see `crate::standards::v1::subsets::brep::schema::snapshot::topology`).
    #[semio_framework_async_macros::async_test]
    async fn from_next_seeds_the_counter_and_next_reports_it_without_advancing() {
        let mut source = LabelSource::from_next(42);
        assert_eq!(source.next(), 42);
        assert_eq!(source.next(), 42, "next() must be a pure read, not itself advance the counter");
        assert_eq!(source.next_label(), PersistentLabel(42));
        assert_eq!(source.next(), 43);
    }

    #[semio_framework_async_macros::async_test]
    async fn label_source_never_repeats() {
        let mut source = LabelSource::new();
        let a = source.next_label();
        let b = source.next_label();
        assert_ne!(a, b);
        assert_eq!(a.0, 0);
        assert_eq!(b.0, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn recorder_generated_then_deleted_cancels_out() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(5);
        rec.record_generated(label);
        rec.record_deleted(label);
        let delta = rec.into_delta();
        assert!(delta.generated.is_empty());
        assert_eq!(delta.deleted, vec![label]);
    }

    #[semio_framework_async_macros::async_test]
    async fn recorder_generated_entity_is_not_also_reported_modified() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(1);
        rec.record_generated(label);
        rec.record_modified(label);
        let delta = rec.into_delta();
        assert_eq!(delta.generated, vec![label]);
        assert!(delta.modified.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn recorder_deduplicates_repeated_reports() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(2);
        rec.record_modified(label);
        rec.record_modified(label);
        let delta = rec.into_delta();
        assert_eq!(delta.modified.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn op_delta_merge_concatenates_all_three_lists() {
        let mut a = OpDelta { generated: vec![PersistentLabel(1)], modified: vec![PersistentLabel(2)], deleted: vec![] };
        let b = OpDelta { generated: vec![], modified: vec![], deleted: vec![PersistentLabel(3)] };
        a.merge(b);
        assert_eq!(a.generated, vec![PersistentLabel(1)]);
        assert_eq!(a.modified, vec![PersistentLabel(2)]);
        assert_eq!(a.deleted, vec![PersistentLabel(3)]);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_delta_reports_is_empty() {
        assert!(OpDelta::default().is_empty());
        assert!(!OpDelta { generated: vec![PersistentLabel(0)], ..Default::default() }.is_empty());
    }
}
