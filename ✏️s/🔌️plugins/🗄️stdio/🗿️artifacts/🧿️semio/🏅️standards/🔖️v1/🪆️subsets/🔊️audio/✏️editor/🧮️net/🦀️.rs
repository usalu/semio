//! 🧮️ Net of one snapshot edit as audio domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `audio` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::audio::schema::mutations::{insert_channel, insert_tag, remove_channel, remove_tag, set_channel_samples, set_format, set_sample_rate, set_tag_value, SemioAudioMutation};
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::base::schema::triples::{net_ordered, NetStep};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioAudioSnapshot, next: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
    let mut out = Vec::new();
    if base.sample_rate != next.sample_rate {
        out.push(SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: next.sample_rate }));
    }
    if base.format != next.format {
        out.push(SemioAudioMutation::SetFormat(set_format::SetFormat { format: next.format.clone() }));
    }
    for step in net_ordered(&base.channels, &next.channels) {
        match step {
            NetStep::Modify { index, item } => out.push(SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index, samples: item.samples.clone() })),
            NetStep::Remove { index } => out.push(SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index })),
            NetStep::Insert { index, item } => out.push(SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index, channel: item.clone() })),
        }
    }
    for step in net_ordered(&base.tags, &next.tags) {
        match step {
            NetStep::Modify { index, item } if base.tags[index].key == item.key => out.push(SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index, value: item.value.clone() })),
            NetStep::Modify { index, item } => {
                out.push(SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index }));
                out.push(SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, tag: item.clone() }));
            }
            NetStep::Remove { index } => out.push(SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index })),
            NetStep::Insert { index, item } => out.push(SemioAudioMutation::InsertTag(insert_tag::InsertTag { index, tag: item.clone() })),
        }
    }
    out
}
