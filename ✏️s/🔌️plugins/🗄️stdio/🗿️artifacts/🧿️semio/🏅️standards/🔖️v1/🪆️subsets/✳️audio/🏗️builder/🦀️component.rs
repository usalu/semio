//! 🏗️ SemioAudioBuilder — `ArtifactBuilder` plus typed constructors (`new`/`add_channel`/
//! `add_tag`), matching gif/svg's "typed constructors, not raw snapshot literals" precedent. This
//! is what an analyzer→builder round-trip test drives: it reconstructs a document using ONLY
//! these typed methods, never `from_snapshot`/`SetSnapshot` directly.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::diff::SemioAudioDiff;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{SemioAudioMutation, apply_semio_audio_mutation};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct SemioAudioBuilder { snapshot: SemioAudioSnapshot }

//#region 🔖️TypedConstructors
impl SemioAudioBuilder {
    /// 🏗️ Starts a fresh document at the given sample rate/format.
    pub fn new(sample_rate: u32, format: SemioAudioFormat) -> Self {
        Self { snapshot: SemioAudioSnapshot { sample_rate, format, ..SemioAudioSnapshot::default() } }
    }
    /// 🏗️ Appends one channel's decoded samples, in channel order.
    pub fn add_channel(mut self, channel: SemioAudioChannel) -> Self {
        self.snapshot.channels.push(channel);
        self
    }
    /// 🏗️ Appends one metadata key/value pair.
    pub fn add_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.snapshot.tags.push(SemioAudioTag { key: key.into(), value: value.into() });
        self
    }
    /// 🏗️ Sets the sample rate.
    pub fn set_sample_rate(mut self, sample_rate: u32) -> Self {
        self.snapshot.sample_rate = sample_rate;
        self
    }
    /// 🏗️ Sets the original-encoding sample format.
    pub fn set_format(mut self, format: SemioAudioFormat) -> Self {
        self.snapshot.format = format;
        self
    }
}
//#endregion 🔖️TypedConstructors

impl ArtifactBuilder for SemioAudioBuilder {
    type Snapshot = SemioAudioSnapshot;
    type Mutation = SemioAudioMutation;
    type Diff = SemioAudioDiff;
    fn empty() -> Self { Self { snapshot: SemioAudioSnapshot::default() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_semio_audio_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SemioAudioDiff as protocol::MutationDiff<SemioAudioSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
}
//#endregion 🔖️Builder

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_constructors_build_the_expected_snapshot() {
        let snapshot = SemioAudioBuilder::new(44_100, SemioAudioFormat::Pcm16)
            .add_channel(SemioAudioChannel { samples: vec![0.0, 0.5] })
            .add_channel(SemioAudioChannel { samples: vec![0.0, -0.5] })
            .add_tag("title", "test")
            .build()
            .expect("build");
        assert_eq!(snapshot.sample_rate, 44_100);
        assert_eq!(snapshot.channels.len(), 2);
        assert_eq!(snapshot.tags[0].key, "title");
    }

    #[test]
    fn mutate_then_absorb_round_trips_through_the_builder() {
        let builder = SemioAudioBuilder::new(48_000, SemioAudioFormat::Float32);
        let (builder, diff) = builder.mutate(SemioAudioMutation::InsertChannel { index: 0, channel: SemioAudioChannel { samples: vec![1.0, 2.0] } });
        let snapshot_after_mutate = builder.clone().build().expect("build");
        let rebuilt = SemioAudioBuilder::empty().absorb(SemioAudioDiff::default()).mutate(SemioAudioMutation::SetSampleRate { sample_rate: 48_000 }).0
            .mutate(SemioAudioMutation::SetFormat { format: SemioAudioFormat::Float32 }).0;
        let rebuilt = rebuilt.absorb(diff);
        assert_eq!(rebuilt.build().expect("build"), snapshot_after_mutate);
    }

    #[test]
    fn from_binary_and_from_text_round_trip_through_the_builder() {
        let snapshot = SemioAudioBuilder::new(22_050, SemioAudioFormat::Pcm24).add_channel(SemioAudioChannel { samples: vec![0.1] }).build().expect("build");
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let via_binary = SemioAudioBuilder::from_binary(&bytes).expect("from_binary").build().expect("build");
        assert_eq!(via_binary, snapshot);

        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let via_text = SemioAudioBuilder::from_text(&text).expect("from_text").build().expect("build");
        assert_eq!(via_text, snapshot);
    }
}
//#endregion 🔖️Tests
