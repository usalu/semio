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

mod unix_wait_tests {
    use super::*;

    #[test]
    fn a_written_wake_pipe_makes_the_wait_return_at_once_and_drains_clean() {
        let idle = WakePipe::new().unwrap();
        let wake = WakePipe::new().unwrap();
        let started = Instant::now();
        assert_eq!(wait_readable(idle.read, wake.read, Some(Duration::from_millis(30))).unwrap(), (false, false));
        assert!(started.elapsed() >= Duration::from_millis(25));
        write_byte(wake.write);
        write_byte(wake.write);
        assert_eq!(wait_readable(idle.read, wake.read, Some(Duration::from_secs(10))).unwrap(), (false, true));
        wake.drain();
        assert_eq!(wait_readable(idle.read, wake.read, Some(Duration::ZERO)).unwrap(), (false, false));
    }

    #[test]
    fn a_resize_signal_sets_the_flag_and_wakes_the_pipe_then_handlers_are_put_back() {
        let wake = WakePipe::new().unwrap();
        let idle = WakePipe::new().unwrap();
        SIGNAL_PIPE.store(wake.write, Ordering::Release);
        PENDING_RESIZE.store(false, Ordering::Release);
        let guard = SignalGuard::install().unwrap();
        unsafe {
            libc::raise(libc::SIGWINCH);
        }
        assert_eq!(wait_readable(idle.read, wake.read, Some(Duration::from_secs(5))).unwrap(), (false, true));
        assert!(PENDING_RESIZE.swap(false, Ordering::AcqRel));
        drop(guard);
        SIGNAL_PIPE.store(-1, Ordering::Release);
    }

    #[test]
    fn restore_does_nothing_until_armed_and_only_once_after() {
        assert!(!restore_now());
        assert!(!disarm_restore());
    }
}
