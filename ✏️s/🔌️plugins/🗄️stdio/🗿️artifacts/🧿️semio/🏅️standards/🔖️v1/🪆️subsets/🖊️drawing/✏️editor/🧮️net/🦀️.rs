//! 🧮️ Net of one snapshot edit as drawing domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `drawing` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::net_keyed;
use crate::standards::v1::subsets::drawing::schema::mutations::{change_stroke_color, change_stroke_width, create_layer, delete_layer, replace_fill, SemioDrawingMutation};
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioDrawingSnapshot, next: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    let styles = net_keyed(&base.styles, &next.styles, |style| style.name.clone());
    let layers = net_keyed(&base.layers, &next.layers, |layer| layer.id.clone());
    let mut out = Vec::new();
    out.extend(layers.removed.iter().map(|layer| SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: layer.id.clone() })));
    for (before, after) in &styles.modified {
        if before.fill != after.fill {
            out.push(SemioDrawingMutation::ReplaceFill(replace_fill::ReplaceFill { style_name: after.name.clone(), new_fill: after.fill }));
        }
        if before.stroke != after.stroke {
            out.push(SemioDrawingMutation::ChangeStrokeColor(change_stroke_color::ChangeStrokeColor { style_name: after.name.clone(), new_color: after.stroke }));
        }
        if before.stroke_width != after.stroke_width {
            out.push(SemioDrawingMutation::ChangeStrokeWidth(change_stroke_width::ChangeStrokeWidth { style_name: after.name.clone(), new_width: after.stroke_width }));
        }
    }
    for layer in &layers.added {
        let index = next.layers.iter().position(|other| other.id == layer.id).unwrap_or(next.layers.len());
        out.push(SemioDrawingMutation::CreateLayer(create_layer::CreateLayer { index, layer: (*layer).clone() }));
    }
    out
}
