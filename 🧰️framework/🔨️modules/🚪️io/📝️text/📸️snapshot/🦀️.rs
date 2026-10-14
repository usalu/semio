//! 📝️ Product-neutral authored snapshot text contract.
use semio_framework_diagnostic::TextError;
/// 📜️ Handcrafted textual representation of a document snapshot, implemented once per
/// technology next to its `Snapshot` type. LAW: `P::parse_dsl(&snapshot.print_dsl())` recovers
/// an equal snapshot — canonical `print_dsl` output is always a `parse_dsl` fixpoint; hand-written
/// text may normalize (whitespace, ordering) before reaching that fixpoint.
pub trait ArtifactDsl: Sized {
    /// 🏷️ Snapshot suffix used by fixture paths and codecs.
    const EXTENSION: &'static str;
    fn parse_dsl(text: &str) -> Result<Self, TextError>;
    fn print_dsl(&self) -> String;
    /// 🪪️ Dotted `plugin.artifact` identity for `.semio` preambles and on-disk names.
    fn envelope_id() -> &'static str {
        Self::EXTENSION
    }
}

