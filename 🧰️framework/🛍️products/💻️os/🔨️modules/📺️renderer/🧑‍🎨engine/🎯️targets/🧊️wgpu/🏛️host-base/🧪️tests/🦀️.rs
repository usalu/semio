//! 🏛️ Unit tests of host-base; the work package moves the tests of its sources here.

use super::*;

#[test]
fn the_host_platform_preferences_are_all_false_until_a_host_publishes_and_replace_as_a_whole() {
    assert_eq!(host_platform_prefs(), semio_framework::ShellPlatformPrefs::default());
    let reported = semio_framework::ShellPlatformPrefs { forced_colors: true, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true };
    set_host_platform_prefs(reported);
    assert_eq!(host_platform_prefs(), reported);
    set_host_platform_prefs(semio_framework::ShellPlatformPrefs::default());
    assert_eq!(host_platform_prefs(), semio_framework::ShellPlatformPrefs::default());
}

#[test]
fn publishing_the_four_answers_reports_a_change_once() {
    set_host_platform_prefs(semio_framework::ShellPlatformPrefs::default());
    assert!(!publish_host_platform_prefs(false, false, false, false));
    assert!(publish_host_platform_prefs(true, true, false, true));
    assert!(!publish_host_platform_prefs(true, true, false, true));
    assert_eq!(host_platform_prefs(), semio_framework::ShellPlatformPrefs { forced_colors: true, prefers_contrast_more: true, prefers_dark: false, prefers_reduced_motion: true });
    set_host_platform_prefs(semio_framework::ShellPlatformPrefs::default());
}
