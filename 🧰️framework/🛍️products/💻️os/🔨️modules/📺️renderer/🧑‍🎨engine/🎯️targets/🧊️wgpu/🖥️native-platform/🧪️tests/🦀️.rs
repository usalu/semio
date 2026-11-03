//! 🖥️ Unit tests of native-platform; the work package moves the tests of its sources here.

#[cfg(not(target_arch = "wasm32"))]
mod platform_prefs {
    use crate::platform_prefs::*;

    const HC_ON: &str = "\r\nHKEY_CURRENT_USER\\Control Panel\\Accessibility\\HighContrast\r\n    Flags    REG_SZ    127\r\n";
    const HC_OFF: &str = "\r\nHKEY_CURRENT_USER\\Control Panel\\Accessibility\\HighContrast\r\n    Flags    REG_SZ    126\r\n";
    const DARK: &str = "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\r\n    AppsUseLightTheme    REG_DWORD    0x0\r\n";
    const LIGHT: &str = "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\r\n    AppsUseLightTheme    REG_DWORD    0x1\r\n";
    const NO_ANIMATION: &str = "HKEY_CURRENT_USER\\Control Panel\\Desktop\\WindowMetrics\r\n    MinAnimate    REG_SZ    0\r\n";
    const ANIMATION: &str = "HKEY_CURRENT_USER\\Control Panel\\Desktop\\WindowMetrics\r\n    MinAnimate    REG_SZ    1\r\n";

    #[test]
    fn windows_reads_high_contrast_theme_and_animation_from_the_registry_text() {
        assert_eq!(from_windows(HC_ON, LIGHT, ANIMATION), NativePlatformPrefs { forced_colors: true, prefers_contrast_more: true, prefers_dark: false, prefers_reduced_motion: false });
        assert_eq!(from_windows(HC_OFF, DARK, NO_ANIMATION), NativePlatformPrefs { forced_colors: false, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true });
        assert_eq!(from_windows("", "", ""), NativePlatformPrefs::default(), "missing keys leave every field false");
        assert_eq!(reg_value("    DisplayVersion    REG_SZ    x", "Flags"), None);
    }

    #[test]
    fn macos_reads_the_defaults_flags_and_the_missing_interface_style() {
        assert_eq!(from_macos("1\n", "0\n", "Dark\n"), NativePlatformPrefs { forced_colors: false, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true });
        assert_eq!(from_macos("0\n", "1\n", ""), NativePlatformPrefs { forced_colors: false, prefers_contrast_more: true, prefers_dark: false, prefers_reduced_motion: false });
        assert_eq!(from_macos("", "", ""), NativePlatformPrefs::default());
    }

    #[test]
    fn linux_reads_gsettings_and_the_gtk_theme() {
        assert_eq!(from_linux("true\n", "'prefer-dark'\n", "false\n", ""), NativePlatformPrefs { forced_colors: true, prefers_contrast_more: true, prefers_dark: true, prefers_reduced_motion: true });
        assert_eq!(from_linux("false\n", "'default'\n", "true\n", ""), NativePlatformPrefs::default());
        assert!(from_linux("", "", "", "HighContrast").forced_colors, "a HighContrast GTK theme counts without gsettings");
        assert!(from_linux("", "", "", "Adwaita:dark").prefers_dark);
        assert_eq!(from_linux("", "", "", ""), NativePlatformPrefs::default(), "no tool and no variable leaves every field false");
        assert_eq!(gsettings_flag("maybe"), None);
    }

    #[test]
    fn detect_never_panics_where_the_tools_are_missing() {
        let _ = detect();
    }
}
