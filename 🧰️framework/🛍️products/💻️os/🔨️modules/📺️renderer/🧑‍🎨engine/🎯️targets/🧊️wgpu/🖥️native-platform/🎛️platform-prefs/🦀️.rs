//! 🎛️ Native platform preferences: forced colours (high contrast), more contrast, dark scheme and reduced motion as the operating system reports them, read on Windows, macOS and Linux through the system tools each ships (no library, no unsafe code). The parsers are pure and run on every platform; only the process spawn depends on the one the host runs on.
//!
//! 🔗️ Web twin: `resolveWgpuHostPlatformPrefs` (`🧭️boot-descriptor/🟦️.ts`); consumer: `semio_framework::ShellPlatformPrefs` through `host-base`.

use std::process::{Command, Stdio};

/// 🎛️ What the operating system reports; each field is `false` when the platform has no such setting or the tool answered nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NativePlatformPrefs {
    pub forced_colors: bool,
    pub prefers_contrast_more: bool,
    pub prefers_dark: bool,
    pub prefers_reduced_motion: bool,
}

//#region 🔖️Parsers
/// 🪟️ The value of `name` in the output of `reg query`, REG_DWORD (`0x1`) or REG_SZ (`126`) alike.
pub fn reg_value(output: &str, name: &str) -> Option<u32> {
    let line = output.lines().find(|line| line.split_whitespace().next() == Some(name))?;
    let raw = line.split_whitespace().last()?;
    raw.strip_prefix("0x").map_or_else(|| raw.parse().ok(), |hex| u32::from_str_radix(hex, 16).ok())
}

/// 🪟️ Windows: Flags bit 0 of `HKCU\Control Panel\Accessibility\HighContrast` is high contrast on (the same bit `SPI_GETHIGHCONTRAST` returns as `HCF_HIGHCONTRASTON`); `AppsUseLightTheme` 0 is dark; `MinAnimate` 0 under `WindowMetrics` is "animation effects" off.
pub fn from_windows(high_contrast: &str, personalize: &str, window_metrics: &str) -> NativePlatformPrefs {
    let contrast = reg_value(high_contrast, "Flags").is_some_and(|flags| flags & 1 == 1);
    NativePlatformPrefs { forced_colors: contrast, prefers_contrast_more: contrast, prefers_dark: reg_value(personalize, "AppsUseLightTheme") == Some(0), prefers_reduced_motion: reg_value(window_metrics, "MinAnimate") == Some(0) }
}

/// 🍎️ Whether `defaults read` answered a truthy value.
pub fn defaults_flag(output: &str) -> bool {
    matches!(output.trim(), "1" | "true" | "YES" | "yes")
}

/// 🍎️ macOS: `reduceMotion` and `increaseContrast` of `com.apple.universalaccess`, and `AppleInterfaceStyle` which reads `Dark` in dark mode and is missing in light mode. macOS has no forced-colours mode.
pub fn from_macos(reduce_motion: &str, increase_contrast: &str, interface_style: &str) -> NativePlatformPrefs {
    NativePlatformPrefs { forced_colors: false, prefers_contrast_more: defaults_flag(increase_contrast), prefers_dark: interface_style.trim().eq_ignore_ascii_case("dark"), prefers_reduced_motion: defaults_flag(reduce_motion) }
}

/// 🐧️ The boolean of a `gsettings get` answer.
pub fn gsettings_flag(output: &str) -> Option<bool> {
    match output.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// 🐧️ The text of a `gsettings get` answer without its quotes.
pub fn gsettings_text(output: &str) -> &str {
    output.trim().trim_matches('\'')
}

/// 🐧️ Linux (GNOME settings, which GTK and most portals follow): `a11y.interface high-contrast`, `interface color-scheme` (`prefer-dark`) and `interface enable-animations` (false is reduced motion); the `GTK_THEME` environment variable naming a HighContrast theme counts as high contrast, one ending in `:dark` or `-dark` as dark.
pub fn from_linux(high_contrast: &str, color_scheme: &str, enable_animations: &str, gtk_theme: &str) -> NativePlatformPrefs {
    let contrast = gsettings_flag(high_contrast).unwrap_or(false) || gtk_theme.to_ascii_lowercase().contains("highcontrast");
    let theme = gtk_theme.to_ascii_lowercase();
    NativePlatformPrefs {
        forced_colors: contrast,
        prefers_contrast_more: contrast,
        prefers_dark: gsettings_text(color_scheme) == "prefer-dark" || theme.ends_with(":dark") || theme.ends_with("-dark"),
        prefers_reduced_motion: gsettings_flag(enable_animations) == Some(false),
    }
}
//#endregion 🔖️Parsers

//#region 🔖️Detection
fn run(program: &str, args: &[&str]) -> String {
    let mut command = Command::new(program);
    command.args(args).stdin(Stdio::null()).stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    command.output().map(|output| String::from_utf8_lossy(&output.stdout).into_owned()).unwrap_or_default()
}

fn reg(key: &str, value: &str) -> String {
    run("reg", &["query", key, "/v", value])
}

fn defaults(args: &[&str]) -> String {
    run("defaults", args)
}

fn gsettings(schema: &str, key: &str) -> String {
    run("gsettings", &["get", schema, key])
}

/// 🎛️ Reads the preferences of the platform this process runs on; every tool that is missing or answers nothing leaves its field `false`. Spawns up to three short system tools, so call it at window creation and when the window regains focus, never per frame.
pub fn detect() -> NativePlatformPrefs {
    if cfg!(target_os = "windows") {
        from_windows(&reg("HKCU\\Control Panel\\Accessibility\\HighContrast", "Flags"), &reg("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "AppsUseLightTheme"), &reg("HKCU\\Control Panel\\Desktop\\WindowMetrics", "MinAnimate"))
    } else if cfg!(target_os = "macos") {
        from_macos(&defaults(&["read", "com.apple.universalaccess", "reduceMotion"]), &defaults(&["read", "com.apple.universalaccess", "increaseContrast"]), &defaults(&["read", "-g", "AppleInterfaceStyle"]))
    } else {
        from_linux(&gsettings("org.gnome.desktop.a11y.interface", "high-contrast"), &gsettings("org.gnome.desktop.interface", "color-scheme"), &gsettings("org.gnome.desktop.interface", "enable-animations"), &std::env::var("GTK_THEME").unwrap_or_default())
    }
}
//#endregion 🔖️Detection
