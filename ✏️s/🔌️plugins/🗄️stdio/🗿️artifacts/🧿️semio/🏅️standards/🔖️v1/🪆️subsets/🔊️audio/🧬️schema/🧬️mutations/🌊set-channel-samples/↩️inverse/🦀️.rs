//! ↩️ Inverse for `SetChannelSamples`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetChannelSamples, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    let super::SetChannelSamples { index, .. } = payload;
    Ok(vec![match base.channels.get(*index) {
        Some(c) => SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: *index, samples: c.samples.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
