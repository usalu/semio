//! 🏗️ SemioDocumentBuilder — real `ArtifactBuilder` round-tripping the full block-tree snapshot,
//! plus typed fluent constructors (`with_style`/`with_image`/`with_block`) for building a
//! document up mutation-free before the real mutation vocabulary takes over.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::SemioDocumentDiff;
use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{SemioDocumentMutation, apply_semio_document_mutation};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};

#[derive(Clone, Debug, Default)]
pub struct SemioDocumentBuilder { snapshot: SemioDocumentSnapshot }

impl SemioDocumentBuilder {
    /// 🎨️ Fluent: appends a named style.
    pub fn with_style(mut self, style: DocStyle) -> Self {
        self.snapshot.styles.push(style);
        self
    }
    /// 🖼️ Fluent: appends a named image.
    pub fn with_image(mut self, image: DocImage) -> Self {
        self.snapshot.images.push(image);
        self
    }
    /// 🧱️ Fluent: appends a top-level block.
    pub fn with_block(mut self, block: DocBlock) -> Self {
        self.snapshot.blocks.push(block);
        self
    }
}

impl ArtifactBuilder for SemioDocumentBuilder {
    type Snapshot = SemioDocumentSnapshot;
    type Mutation = SemioDocumentMutation;
    type Diff = SemioDocumentDiff;
    fn empty() -> Self { Self { snapshot: SemioDocumentSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_document_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioDocumentDiff as protocol::MutationDiff<SemioDocumentSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fluent_builder_round_trips_through_text_and_binary() {
        let built = SemioDocumentBuilder::empty()
            .with_style(DocStyle { id: "Normal".into(), name: "Normal".into(), based_on: None })
            .with_image(DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] })
            .with_block(DocBlock::paragraph("hello"))
            .build()
            .expect("build");
        assert_eq!(built.styles.len(), 1);
        assert_eq!(built.images.len(), 1);
        assert_eq!(built.blocks.len(), 1);

        let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&built);
        let from_text = SemioDocumentBuilder::from_text(&text).expect("from_text").build().expect("build");
        assert_eq!(from_text, built);

        let bytes = <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(&built);
        let from_binary = SemioDocumentBuilder::from_binary(&bytes).expect("from_binary").build().expect("build");
        assert_eq!(from_binary, built);
    }

    #[test]
    fn mutate_then_absorb_round_trips() {
        let (builder, diff) = SemioDocumentBuilder::empty().mutate(SemioDocumentMutation::InsertStyle { style: DocStyle { id: "s".into(), name: "S".into(), based_on: None } });
        let rebuilt = SemioDocumentBuilder::empty().absorb(diff);
        assert_eq!(builder.build().unwrap(), rebuilt.build().unwrap());
    }
}
//#endregion 🔖️Tests
