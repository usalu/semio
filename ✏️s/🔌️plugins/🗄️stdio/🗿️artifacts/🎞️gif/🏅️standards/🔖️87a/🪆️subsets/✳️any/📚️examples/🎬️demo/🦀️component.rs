//! 📚️ Example demo for stdio.gif.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON).await
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
    /// parsed through the real `ArtifactDsl` codec — not a hand-built stub). `PRIMARY_TEXT`'s own
    /// `semio stdio.gif.dsl v1` envelope is the 87a standard's (`envelope_id()`), so this parses
    /// as `standards::v87a`'s `GifSnapshot` — 89a's own laws are covered separately by the
    /// `💃️dancing` example (a real 89a fixture).
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        use crate::artifacts::gif::standards::v87a::subsets::any::schema::inferences::GifInference;
        use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
        use protocol::Inference;
        let snapshot = <GifSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("demo fixture must parse");
        assert_eq!(GifInference::infer(&snapshot), GifInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        use crate::artifacts::gif::standards::v87a::subsets::any::schema::inferences::GifInference;
        use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
        use protocol::Inference;
        assert_eq!(GifInference::infer(&GifSnapshot::default()), GifInference::default());
    }
}
