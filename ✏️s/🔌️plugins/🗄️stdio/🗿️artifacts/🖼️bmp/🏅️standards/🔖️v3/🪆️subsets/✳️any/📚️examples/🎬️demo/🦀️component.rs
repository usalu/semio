//! 📚️ Example demo for stdio.bmp.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Demo", "Demo") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }

    /// 🧪️ Ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's
    /// inference laws, exercised against this example's own real fixture (`PRIMARY_TEXT`,
    /// parsed through the real `ArtifactDsl` codec — not a hand-built stub).
    #[test]
    fn inference_determinism_law() {
        use crate::artifacts::bmp::standards::v_v3::subsets::any::schema::inferences::BmpInference;
        use crate::artifacts::bmp::BmpSnapshot;
        use protocol::Inference;
        let snapshot = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("demo fixture must parse");
        assert_eq!(BmpInference::infer(&snapshot), BmpInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        use crate::artifacts::bmp::standards::v_v3::subsets::any::schema::inferences::BmpInference;
        use crate::artifacts::bmp::BmpSnapshot;
        use protocol::Inference;
        assert_eq!(BmpInference::infer(&BmpSnapshot::default()), BmpInference::default());
    }
}
