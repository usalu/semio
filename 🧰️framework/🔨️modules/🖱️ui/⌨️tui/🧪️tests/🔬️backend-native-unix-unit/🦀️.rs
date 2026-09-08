mod tests {
    use super::*;

    #[test]
    fn raw_mode_restoration_retries_after_setup_cleanup_fails() {
        let mut raw_mode_entered = true;
        assert!(!retry_mode_restoration(&mut raw_mode_entered, || false));
        assert!(raw_mode_entered);
        assert!(retry_mode_restoration(&mut raw_mode_entered, || true));
        assert!(!raw_mode_entered);
    }

    #[test]
    fn failed_ansi_teardown_remains_owned_until_a_later_success() {
        let mut ansi_setup_owned = true;
        assert!(!release_owned_terminal_cleanup(&mut ansi_setup_owned, false));
        assert!(ansi_setup_owned);
        assert!(release_owned_terminal_cleanup(&mut ansi_setup_owned, true));
        assert!(!ansi_setup_owned);
    }

    #[test]
    fn pending_terminal_cleanup_rejects_reentry() {
        assert!(terminal_entry_is_available(false, false, true));
        assert!(!terminal_entry_is_available(true, false, true));
        assert!(!terminal_entry_is_available(false, true, true));
        assert!(!terminal_entry_is_available(false, false, false));
    }
}
