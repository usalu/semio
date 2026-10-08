//! 🧮️ Net of one snapshot edit as document domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `document` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_keyed, net_ordered, NetStep};
use crate::standards::v1::subsets::document::schema::mutations::{insert_block, insert_image, insert_style, remove_block, remove_image, remove_style, set_block_content, set_image_bytes, set_style_based_on, set_style_name, DocBlockPath, SemioDocumentMutation};
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioDocumentSnapshot, next: &SemioDocumentSnapshot) -> Vec<SemioDocumentMutation> {
    let styles = net_keyed(&base.styles, &next.styles, |style| style.id.clone());
    let images = net_keyed(&base.images, &next.images, |image| image.id.clone());
    let mut out = Vec::new();
    out.extend(styles.added.iter().map(|style| SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: (*style).clone(), at: None })));
    for (before, after) in &styles.modified {
        if before.name != after.name {
            out.push(SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: after.id.clone(), name: after.name.clone() }));
        }
        if before.based_on != after.based_on {
            out.push(SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: after.id.clone(), based_on: after.based_on.clone() }));
        }
    }
    out.extend(images.added.iter().map(|image| SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: (*image).clone(), at: None })));
    out.extend(images.modified.iter().map(|(_, image)| SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: image.id.clone(), mime: image.mime.clone(), bytes: image.bytes.clone() })));
    for step in net_ordered(&base.blocks, &next.blocks) {
        match step {
            NetStep::Modify { index, item } => out.push(SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: DocBlockPath::top(index), block: item.clone() })),
            NetStep::Remove { index } => out.push(SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(index) })),
            NetStep::Insert { index, item } => out.push(SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(index), block: item.clone() })),
        }
    }
    out.extend(images.removed.iter().map(|image| SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: image.id.clone() })));
    out.extend(styles.removed.iter().map(|style| SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: style.id.clone() })));
    out
}
