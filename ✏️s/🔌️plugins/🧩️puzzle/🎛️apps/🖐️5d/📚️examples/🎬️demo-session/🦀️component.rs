//! 📚️ App demo-session example.

use semio_framework_os_kernel::plugin::ExampleSource;
use semio_framework::LocalizedLabel;

pub const ID: &str = "demo-session";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Demo Session", "Demo-Sitzung") }
pub const ICON: &str = "play";
pub const CMD_TEXT: &str = include_str!("🖼️assets/🎮️demo.cmd.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), CMD_TEXT, ICON) }
