//! 🚪️ IO stdio.pdf (1.7/🧱️base): the native text, binary and SQLite representations, the mutation bridge, the foreign
//! artifact custody port, and the vocabulary every PDF codec layer shares — the typed engine error and the PDF text
//! string codec (§7.9.2.2). The byte reader is the `import` child (sniff → cross-reference → decrypt → retained graph
//! → typed lanes); the byte writer is the `export` child (typed lanes reconciled onto the retained graph → bytes),
//! together with the typed builders every consumer starts from.
//!
//! Laws (proven in `📤️export/🧪️tests`): `lift(lower(t)) == t` on the typed lanes; `decode(encode(s)) == s`;
//! a write re-states only what a moved lane owns, so every other retained object, dictionary entry
//! and trailer entry — direct COS edits included — survives it untouched; a decoded document is
//! its own fixed point.

pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text_string;

pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
pub mod binary {
    pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::binary::*;
    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::io::binary::diff;
    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::binary::inferences;
}
pub mod text {
    pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::text::*;
    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text::inferences;
}
pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::io::sqlite;
/// 🌉️ Where a builder or a test meets the central applier: a mutation's diff is applied atomically to the working snapshot, and a
/// rejection becomes a fatal outcome. The schema layer only builds diffs; applying them is an io/store concern.
pub mod mutation_bridge {
    use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::PdfSnapshot};

    /// 🛡️ Applies `outcome` to `snapshot` atomically through the central applier and converts an apply rejection into a fatal outcome.
    pub fn apply_outcome(outcome: protocol::MutationOutcome<PdfDiff>, snapshot: &mut PdfSnapshot) -> protocol::MutationOutcome<PdfDiff> {
        let (diff, messages) = outcome.into_parts();
        match protocol::apply_diff(&diff, snapshot) {
            Ok(next) => {
                *snapshot = next;
                protocol::MutationOutcome::new(diff).absorb_messages(messages)
            }
            Err(error) => protocol::MutationOutcome::new(PdfDiff::default()).absorb_messages(messages).absorb_messages([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]),
        }
    }

    /// ▶️ Applies one mutation through its leaf-owned diff.
    pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
        use protocol::Mutation;
        let outcome = mutation.diff(snapshot);
        apply_outcome(outcome, snapshot)
    }
}

pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_pdf_mutation;
    use crate::standards::v1_7::subsets::base::schema::mutations::{InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.pdf.1.7` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PdfBuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl PdfBuilderConstruction {
        /// ➕ Typed construction: appends a page (the analyzer→builder round-trip acceptance test's
        /// primary entry point -- requirement #8's `InsertPage`, exposed ergonomically).
        pub fn add_page(self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            let (next, _diff) = self.mutate(PdfMutation::InsertPage(InsertPage { index, page }));
            next
        }
        pub fn set_info(self, info: PdfInfo) -> Self {
            let (next, _diff) = self.mutate(PdfMutation::SetInfo(SetInfo { info }));
            next
        }
    }

    impl ArtifactBuilder for PdfBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;
        fn empty() -> Self {
            Self { snapshot: PdfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
