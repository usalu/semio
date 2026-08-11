//! 🏗️ SemioImageBuilder — local `ArtifactBuilder` plus typed constructors (`new`/`add_frame`/
//! `set_icc`/`add_metadata`), matching the gif 89a/svg "typed constructors, not raw snapshot
//! literals" precedent (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
//! EVOLUTION D2).

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{SemioImageMutation, apply_semio_image_mutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};

#[derive(Clone, Debug, Default)]
pub struct SemioImageBuilder { snapshot: SemioImageSnapshot }

//#region 🔖️TypedConstructors
impl SemioImageBuilder {
    /// 🏗️ Starts a fresh image at the given pixel dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self { snapshot: SemioImageSnapshot { width, height, ..SemioImageSnapshot::default() } }
    }
    /// 🏗️ Sets the source colorspace.
    pub fn set_colorspace(mut self, colorspace: SemioColorspace) -> Self {
        self.snapshot.colorspace = colorspace;
        self
    }
    /// 🏗️ Sets the bit depth.
    pub fn set_bit_depth(mut self, bit_depth: u8) -> Self {
        self.snapshot.bit_depth = bit_depth;
        self
    }
    /// 🏗️ Appends one frame, in order.
    pub fn add_frame(mut self, frame: SemioImageFrame) -> Self {
        self.snapshot.frames.push(frame);
        self
    }
    /// 🏗️ Sets the embedded ICC profile (`None` clears it).
    pub fn set_icc(mut self, icc: Option<Vec<u8>>) -> Self {
        self.snapshot.icc = icc;
        self
    }
    /// 🏗️ Appends one metadata entry.
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.snapshot.metadata.push(SemioImageMetadataEntry { key: key.into(), value: value.into() });
        self
    }
}
//#endregion 🔖️TypedConstructors

impl ArtifactBuilder for SemioImageBuilder {
    type Snapshot = SemioImageSnapshot;
    type Mutation = SemioImageMutation;
    type Diff = SemioImageDiff;
    fn empty() -> Self { Self { snapshot: SemioImageSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_image_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioImageDiff as protocol::MutationDiff<SemioImageSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioImageBuilder::new(2, 2)
            .set_colorspace(SemioColorspace::Rgba)
            .set_bit_depth(8)
            .add_frame(SemioImageFrame { delay_ms: 0, rgba8: vec![255; 16] })
            .set_icc(Some(vec![1, 2, 3]))
            .add_metadata("Title", "test")
            .build()
            .expect("build");
        assert_eq!(snapshot.width, 2);
        assert_eq!(snapshot.height, 2);
        assert_eq!(snapshot.colorspace, SemioColorspace::Rgba);
        assert_eq!(snapshot.frames.len(), 1);
        assert_eq!(snapshot.icc, Some(vec![1, 2, 3]));
        assert_eq!(snapshot.metadata.len(), 1);
    }
}
//#endregion 🔖️Tests
