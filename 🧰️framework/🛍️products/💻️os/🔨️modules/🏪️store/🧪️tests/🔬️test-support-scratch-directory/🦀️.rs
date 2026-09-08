mod scratch_directory_tests {
    use super::*;

    #[test]
    fn owned_scratch_directory_is_unique_and_removed_on_drop() {
        let first = tempdir().expect("first scratch directory");
        let second = tempdir().expect("second scratch directory");
        assert_ne!(first.path(), second.path());
        assert!(first.path().is_dir());
        let first_path = first.path().to_path_buf();
        drop(first);
        assert!(!first_path.exists());
    }
}
