//! 🧮️ Net of one snapshot edit as video domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `video` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_ordered, NetStep};
use crate::standards::v1::subsets::video::schema::mutations::{insert_sample, insert_stream, remove_sample, remove_stream, set_sample_data, set_sample_flags, set_stream_meta, SemioVideoMutation};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStream};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioVideoSnapshot, next: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
    let mut out = Vec::new();
    for step in net_ordered(&base.streams, &next.streams) {
        match step {
            NetStep::Modify { index, item } => net_stream(index, &base.streams[index], item, &mut out),
            NetStep::Remove { index } => out.push(SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index })),
            NetStep::Insert { index, item } => out.push(SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, stream: item.clone() })),
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_stream(stream_index: usize, before: &SemioVideoStream, after: &SemioVideoStream, out: &mut Vec<SemioVideoMutation>) {
    if (before.kind, &before.codec, before.width, before.height, &before.rate) != (after.kind, &after.codec, after.width, after.height, &after.rate) {
        out.push(SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: stream_index, kind: after.kind, codec: after.codec.clone(), width: after.width, height: after.height, rate: after.rate.clone() }));
    }
    for step in net_ordered(&before.samples, &after.samples) {
        match step {
            NetStep::Modify { index, item } => {
                let base_sample = &before.samples[index];
                if base_sample.data != item.data {
                    out.push(SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, data: item.data.clone() }));
                }
                if (base_sample.pts, base_sample.key) != (item.pts, item.key) {
                    out.push(SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, pts: item.pts, key: item.key }));
                }
            }
            NetStep::Remove { index } => out.push(SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index })),
            NetStep::Insert { index, item } => out.push(SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, sample: item.clone() })),
        }
    }
}
