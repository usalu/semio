//! 🌐️ Canonical UI language axes and label ownership below product hosts.

pub use serde::{Deserialize, Serialize};
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};

#[path = "../🎚️axes/🤖️generated/🦀️.rs"]
mod axes;
pub use axes::{Locale, Terminology};

#[path = "🧾️value/🦀️.rs"]
mod value;
#[path = "🏷️label/🦀️.rs"]
mod label;
pub use label::{AppLabels, Label, LabelText, LocalizedLabel};

#[path = "🧩️labels/🦀️.rs"]
mod labels;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
