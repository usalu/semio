//! 🧮️ Net of one snapshot edit as presentation domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `presentation` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_keyed, net_ordered, NetStep};
use crate::standards::v1::subsets::presentation::schema::mutations::{insert_layout, insert_master, insert_shape, insert_slide, remove_layout, remove_master, remove_shape, remove_slide, set_layout_master, set_shape_frame, set_slide_layout, set_slide_notes, set_text_box_blocks, SemioPresentationMutation};
use crate::standards::v1::subsets::presentation::schema::snapshot::{Slide, SemioPresentationSnapshot, SlideShape};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioPresentationSnapshot, next: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
    let masters = net_keyed(&base.masters, &next.masters, |master| master.id.clone());
    let layouts = net_keyed(&base.layouts, &next.layouts, |layout| layout.id.clone());
    let mut out = Vec::new();
    out.extend(masters.added.iter().map(|master| SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: (*master).clone(), at: None })));
    out.extend(layouts.added.iter().map(|layout| SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: (*layout).clone(), at: None })));
    out.extend(layouts.modified.iter().filter(|(before, after)| before.master_id != after.master_id).map(|(_, after)| SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: after.id.clone(), master_id: after.master_id.clone() })));
    for step in net_ordered(&base.slides, &next.slides) {
        match step {
            NetStep::Modify { index, item } => net_slide(index, &base.slides[index], item, &mut out),
            NetStep::Remove { index } => out.push(SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index })),
            NetStep::Insert { index, item } => out.push(SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index, slide: item.clone() })),
        }
    }
    out.extend(layouts.removed.iter().map(|layout| SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: layout.id.clone() })));
    out.extend(masters.removed.iter().map(|master| SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: master.id.clone() })));
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_slide(slide_index: usize, before: &Slide, after: &Slide, out: &mut Vec<SemioPresentationMutation>) {
    if before.layout_id != after.layout_id {
        out.push(SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: slide_index, layout_id: after.layout_id.clone() }));
    }
    if before.notes != after.notes {
        out.push(SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: slide_index, notes: after.notes.clone() }));
    }
    for step in net_ordered(&before.shapes, &after.shapes) {
        match step {
            NetStep::Modify { index, item } => net_shape(slide_index, index, &before.shapes[index], item, out),
            NetStep::Remove { index } => out.push(SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index: index })),
            NetStep::Insert { index, item } => out.push(SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index: index, shape: item.clone() })),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_shape(slide_index: usize, shape_index: usize, before: &SlideShape, after: &SlideShape, out: &mut Vec<SemioPresentationMutation>) {
    match (before, after) {
        (SlideShape::TextBox { frame: before_frame, blocks: before_blocks, .. }, SlideShape::TextBox { frame, blocks, .. }) => {
            if before_frame != frame {
                out.push(SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index, shape_index, frame: frame.clone() }));
            }
            if before_blocks != blocks {
                out.push(SemioPresentationMutation::SetTextBoxBlocks(set_text_box_blocks::SetTextBoxBlocks { slide_index, shape_index, blocks: blocks.clone() }));
            }
        }
        _ => {
            out.push(SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }));
            out.push(SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, shape: after.clone() }));
        }
    }
}
