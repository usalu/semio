//! 📚️ Example demo for stdio.png.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
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

    /// 🧪️ Ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's
    /// inference laws, exercised against this example's own real fixture (`PRIMARY_TEXT`,
    /// parsed through the real `ArtifactDsl` codec — not a hand-built stub).
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        use crate::artifacts::png::standards::v1_2::subsets::any::schema::inferences::PngInference;
        use crate::artifacts::png::PngSnapshot;
        use protocol::Inference;
        let snapshot = <PngSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).await.expect("demo fixture must parse");
        assert_eq!(PngInference::infer(&snapshot), PngInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        use crate::artifacts::png::standards::v1_2::subsets::any::schema::inferences::PngInference;
        use crate::artifacts::png::PngSnapshot;
        use protocol::Inference;
        assert_eq!(PngInference::infer(&PngSnapshot::default()), PngInference::default());
    }
}
