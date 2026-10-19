//! 🍃️ The semio audio mutation leaves: every payload with its diff and inverse, generic over the aggregate that wraps them.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        use semio_s_artifact_stdio_semio_audio_model::schema::diff::SemioAudioChannelDiff;
        use semio_s_artifact_stdio_semio_audio_model::schema::diff::SemioAudioDiff;
        use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioChannel;
        use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioFormat;
        use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
        use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioTag;
        use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
        use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
        use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;

        #[path = "../../../🎚️set-sample-rate/🦀️.rs"]
        pub mod set_sample_rate;
        #[path = "../../../💽set-format/🦀️.rs"]
        pub mod set_format;
        #[path = "../../../🎙️insert-channel/🦀️.rs"]
        pub mod insert_channel;
        #[path = "../../../🔇remove-channel/🦀️.rs"]
        pub mod remove_channel;
        #[path = "../../../🌊set-channel-samples/🦀️.rs"]
        pub mod set_channel_samples;
        #[path = "../../../🏷️insert-tag/🦀️.rs"]
        pub mod insert_tag;
        #[path = "../../../✂️remove-tag/🦀️.rs"]
        pub mod remove_tag;
        #[path = "../../../💬set-tag-value/🦀️.rs"]
        pub mod set_tag_value;

        /// 🔗️ The aggregate-facing contract of every audio leaf: an operation enum that diffs an audio snapshot into an audio diff and wraps each leaf.
        pub trait SemioAudioOp: protocol::Mutation<SemioAudioSnapshot, Diff = SemioAudioDiff> + From<set_sample_rate::SetSampleRate> + From<set_format::SetFormat> + From<insert_channel::InsertChannel> + From<remove_channel::RemoveChannel> + From<set_channel_samples::SetChannelSamples> + From<insert_tag::InsertTag> + From<remove_tag::RemoveTag> + From<set_tag_value::SetTagValue> {}

        impl<T> SemioAudioOp for T where T: protocol::Mutation<SemioAudioSnapshot, Diff = SemioAudioDiff> + From<set_sample_rate::SetSampleRate> + From<set_format::SetFormat> + From<insert_channel::InsertChannel> + From<remove_channel::RemoveChannel> + From<set_channel_samples::SetChannelSamples> + From<insert_tag::InsertTag> + From<remove_tag::RemoveTag> + From<set_tag_value::SetTagValue> {}
    }
}
