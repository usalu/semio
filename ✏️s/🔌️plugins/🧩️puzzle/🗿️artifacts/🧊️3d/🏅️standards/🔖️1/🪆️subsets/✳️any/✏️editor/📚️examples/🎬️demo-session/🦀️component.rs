//! 📚️ App demo-session example for puzzle 3d.

use std::sync::LazyLock;

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

/// 🏷️ Stable example id.
pub const ID: &str = "demo-session";

/// 🗣️ Localized picker label.
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo Session", "Demo-Sitzung")
}

/// 🖼️ Icon id.
pub const ICON: &str = "play";

/// 🎮️ Command-script fixture text.
pub const CMD_TEXT: &str = include_str!("🖼️assets/🎮️demo.cmd.semio");

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {
    ExampleSource::new(ID, label(), CMD_TEXT, ICON)
});
