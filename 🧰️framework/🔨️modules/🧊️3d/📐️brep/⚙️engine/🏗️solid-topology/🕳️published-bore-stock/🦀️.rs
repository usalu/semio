//! 🕳️ Verifies the complete member selection of a published bore feature.

use super::*;
use semio_framework_3d_brep_representation::contract::BorePreviewSource;

/// 🧱️ Recovers the stock of an owned through or blind bore from its stable wall and floor identities.
pub fn published_bore_stock(body: &Body, shell: &Shell, source: BorePreviewSource, selected: &[FaceId]) -> Option<SolidPrimitive> {
    let bottom_open = source.is_open(false);
    let top_open = source.is_open(true);
    if !(bottom_open || top_open) || shell.faces.len() != source.face_count() { return None; }
    let blind = bottom_open != top_open;
    if selected.len() != 1 + usize::from(blind) { return None; }
    let features = [BorePreviewSource::BORE_WALL_FACE, source.cap_face(bottom_open)];
    for &index in &features[..selected.len()] {
        let id = *shell.faces.get(index)?;
        if !selected.contains(&id) || body.faces.get(id)?.label.0 != source.face_label(index) { return None; }
    }
    let [width, depth, height] = source.dimensions;
    Some(SolidPrimitive::Box { origin: Pnt3::from_array(source.origin), width, depth, height })
}
