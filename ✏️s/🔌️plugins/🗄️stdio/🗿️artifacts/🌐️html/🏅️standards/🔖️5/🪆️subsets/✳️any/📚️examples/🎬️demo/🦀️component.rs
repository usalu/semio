//! 📚️ Example demo for stdio.html — a small literal, well-formed HTML5 document (HTML is a
//! text-based format, like svg/md; unlike gif's binary demo, no hex encoding is needed —
//! `HtmlSnapshot::parse_dsl` accepts this text directly).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
