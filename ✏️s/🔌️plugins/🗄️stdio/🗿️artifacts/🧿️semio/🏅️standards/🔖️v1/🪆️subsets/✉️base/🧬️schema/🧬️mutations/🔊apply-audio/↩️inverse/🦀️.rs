//! ↩️ Inverse for `ApplyAudio`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::SemioAudioMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_audio;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyAudio, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_audio::ApplyAudio>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Audio(b) => <SemioAudioMutation as ColdMutation<SemioAudioSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyAudio { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
