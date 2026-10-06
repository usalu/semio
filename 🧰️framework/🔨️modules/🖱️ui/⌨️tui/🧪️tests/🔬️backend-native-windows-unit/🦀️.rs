mod tests {
    use super::*;

    #[test]
    fn mode_rollback_retries_after_the_first_restore_failure() {
        let mut modes = ConsoleModeOwnership { stdin_changed: false, stdout_changed: true, ..Default::default() };
        assert!(!modes.restore_with(|_, _| false, std::ptr::null_mut(), 1, std::ptr::null_mut(), 2));
        assert!(modes.stdout_changed);
        assert!(modes.restore_with(|_, _| true, std::ptr::null_mut(), 1, std::ptr::null_mut(), 2));
        assert!(modes.is_empty());
    }

    #[test]
    fn mode_rollback_cleans_both_modes_after_a_setup_write_failure() {
        let mut modes = ConsoleModeOwnership { stdin_changed: true, stdout_changed: true, ..Default::default() };
        assert!(modes.restore_with(|_, _| true, std::ptr::null_mut(), 1, std::ptr::null_mut(), 2));
        assert!(modes.is_empty());
    }

    #[test]
    fn code_page_restoration_retries_only_the_remaining_owned_page() {
        let mut modes = ConsoleModeOwnership { input_cp_changed: true, output_cp_changed: true, ..Default::default() };
        modes.restore_code_pages_with(|page| { assert_eq!(page, 437); false }, |page| { assert_eq!(page, 850); true }, 437, 850);
        assert!(modes.input_cp_changed); assert!(!modes.output_cp_changed);
        modes.restore_code_pages_with(|page| { assert_eq!(page, 437); true }, |_| panic!("already restored"), 437, 850);
        assert!(modes.is_empty());
    }

    #[test]
    fn ansi_teardown_failure_remains_owned_for_a_later_windows_cleanup() {
        let mut ansi_setup_owned = true;
        assert!(!release_owned_terminal_cleanup(&mut ansi_setup_owned, false));
        assert!(ansi_setup_owned);
        assert!(release_owned_terminal_cleanup(&mut ansi_setup_owned, true));
        assert!(!ansi_setup_owned);
    }
}
