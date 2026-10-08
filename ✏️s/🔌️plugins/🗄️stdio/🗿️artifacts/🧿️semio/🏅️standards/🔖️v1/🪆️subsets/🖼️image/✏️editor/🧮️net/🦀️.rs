//! 🧮️ Net of one snapshot edit as image domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `image` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_keyed, net_ordered, NetStep};
use crate::standards::v1::subsets::image::schema::mutations::{insert_frame, remove_frame, remove_metadata_entry, set_bit_depth, set_colorspace, set_dimensions, set_frame_delay, set_frame_pixels, set_icc, set_metadata_entry, SemioImageMutation};
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioImageSnapshot, next: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
    let mut out = Vec::new();
    if (base.width, base.height) != (next.width, next.height) {
        out.push(SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: next.width, height: next.height }));
    }
    if base.colorspace != next.colorspace {
        out.push(SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: next.colorspace }));
    }
    if base.bit_depth != next.bit_depth {
        out.push(SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: next.bit_depth }));
    }
    if base.icc != next.icc {
        out.push(SemioImageMutation::SetIcc(set_icc::SetIcc { icc: next.icc.clone() }));
    }
    for step in net_ordered(&base.frames, &next.frames) {
        match step {
            NetStep::Modify { index, item } => {
                let frame = &base.frames[index];
                if frame.delay_ms != item.delay_ms {
                    out.push(SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_ms: item.delay_ms }));
                }
                if frame.rgba8 != item.rgba8 {
                    out.push(SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8: item.rgba8.clone() }));
                }
            }
            NetStep::Remove { index } => out.push(SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index })),
            NetStep::Insert { index, item } => out.push(SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, frame: item.clone() })),
        }
    }
    let metadata = net_keyed(&base.metadata, &next.metadata, |entry| entry.key.clone());
    out.extend(metadata.removed.iter().map(|entry| SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: entry.key.clone() })));
    out.extend(metadata.modified.iter().map(|(_, entry)| SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: entry.key.clone(), value: entry.value.clone(), at: None })));
    out.extend(metadata.added.iter().map(|entry| SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: entry.key.clone(), value: entry.value.clone(), at: None })));
    out
}
