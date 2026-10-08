//! 📥️ Projects admitted MP3 metadata and MPEG header meaning into semantic audio.
//! Compressed MPEG payload is retained by MP3; this bridge does not fabricate PCM samples.

use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
use {semio_framework_plugin::ArtifactDeserializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_mp3::Mp3Snapshot;
use semio_s_artifact_stdio_mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

pub struct SemioAudioFromMp3;

impl ArtifactDeserializer for SemioAudioFromMp3 {
    type From = Mp3Snapshot;
    type Into = SemioAudioSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let (sample_rate, channel_count) = match from.frames.first() {
            Some(frame) => (mpeg_sample_rate(frame.header.mpeg_version_id, frame.header.sample_rate_index), if frame.header.channel_mode == 3 { 1 } else { 2 }),
            None => (0, 0),
        };
        let channels = (0..channel_count).map(|_| SemioAudioChannel { samples: Vec::new() }).collect();

        let mut tags = Vec::new();
        if let Some(id3v2) = &from.id3v2 {
            for frame in &id3v2.frames {
                match &frame.content {
                    Id3Content::Text{values} => for value in values{tags.push(SemioAudioTag{key:frame.id.clone(),value:value.clone()});},
                    Id3Content::UserText{description,values} => for value in values{tags.push(SemioAudioTag{key:format!("{}:{description}",frame.id),value:value.clone()});},
                    Id3Content::Comment{language,description,text}|Id3Content::Lyrics{language,description,text} => tags.push(SemioAudioTag{key:format!("{}:{language}:{description}",frame.id),value:text.clone()}),
                    Id3Content::Url{url} => tags.push(SemioAudioTag{key:frame.id.clone(),value:url.clone()}),
                    Id3Content::UserUrl{description,url} => tags.push(SemioAudioTag{key:format!("{}:{description}",frame.id),value:url.clone()}),
                    Id3Content::Picture{..}|Id3Content::Opaque{..} => {},
                }
            }
        }
        if let Some(tag) = &from.id3v1 {
            for(key,value)in [("title",&tag.title),("artist",&tag.artist),("album",&tag.album),("year",&tag.year),("comment",&tag.comment)]{tags.push(SemioAudioTag{key:format!("id3v1.{key}"),value:value.clone()});}
            if let Some(track)=tag.track{tags.push(SemioAudioTag{key:"id3v1.track".into(),value:track.to_string()});}
            if let Some(genre)=tag.genre{tags.push(SemioAudioTag{key:"id3v1.genre".into(),value:genre.to_string()});}
        }

        Ok(SemioAudioSnapshot { schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(), sample_rate, format: SemioAudioFormat::default(), channels, tags })
    }
}

/// 📐️ MPEG-1/2/2.5 Layer III sample rate table (ISO/IEC 11172-3 Table 3.B.2), keyed by
/// `(mpeg_version_id, sample_rate_index)`. `sample_rate_index == 3` is spec-reserved; falls back
/// honestly to `0` (never a fabricated guess) for both the reserved index and unrecognized version.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mpeg_sample_rate(version_id: u8, index: u8) -> u32 {
    match (version_id, index) {
        (3, 0) => 44_100,
        (3, 1) => 48_000,
        (3, 2) => 32_000, // MPEG1
        (2, 0) => 22_050,
        (2, 1) => 24_000,
        (2, 2) => 16_000, // MPEG2
        (0, 0) => 11_025,
        (0, 1) => 12_000,
        (0, 2) => 8_000, // MPEG2.5
        _ => 0,
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
