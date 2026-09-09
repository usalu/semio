//! 📤️ Serialize `s.stdio.semio` (v1/audio) into `s.stdio.mp3` (mpeg1-layer3/✳️any) — the OTHER
//! half of this pair's documented, real, unavoidable asymmetry (see the deserializer's own doc
//! comment): `audio` always carries REAL decoded `f32` PCM samples, but producing a valid MP3
//! frame `payload` requires a genuine Huffman/MDCT psychoacoustic ENCODER, which does not exist
//! anywhere in this repository and is explicitly out of scope (the ticket's "zero codec
//! reimplementation" rule -- this bridge's job is Snapshot-to-Snapshot mapping via the format's own
//! existing engine functions, never writing a new codec). There is no honest way to invent
//! compressed frame bytes from raw samples without one, so this direction returns a typed error
//! rather than fabricating silent/zero frame payloads that would not actually decode as real MP3.
//! `audio←mp3` (metadata + opaque payload) therefore has NO general inverse in this bridge --
//! documented here, not silently pretended otherwise.

use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_mp3::Mp3Snapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

pub struct SemioAudioToMp3;

impl ArtifactSerializer for SemioAudioToMp3 {
    type From = SemioAudioSnapshot;
    type Into = Mp3Snapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.channels.iter().all(|c| c.samples.is_empty()) {
            // 📦️ A snapshot with no real sample content (e.g. one THIS bridge's own deserializer
            // just produced from an mp3 source) has nothing an encoder would need to invent --
            // honestly round-trips to an empty-frame container instead of erroring, since no
            // fabrication is required to represent "no audio content".
            return Ok(Mp3Snapshot { schema: "stdio.mp3".into(), id3v2: None, frames: Vec::new(), id3v1: None });
        }
        Err(store::PackError::Schema(
            "audio→mp3 export requires encoding real f32 PCM samples into compressed MPEG Layer III \
             frames (Huffman/MDCT psychoacoustic encoding); no MP3 encoder exists in this repository \
             and implementing one is out of scope for a snapshot-to-snapshot io bridge (zero codec \
             reimplementation) -- this is the honest mirror of mp3→audio's own opaque-payload boundary, \
             not a bug"
                .to_string(),
        ))
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
