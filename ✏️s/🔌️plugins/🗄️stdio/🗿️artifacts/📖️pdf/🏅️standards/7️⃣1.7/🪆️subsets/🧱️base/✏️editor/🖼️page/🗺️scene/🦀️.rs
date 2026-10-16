//! 🗺️ PDF page editor scene: the addressable page objects, the canvas layers and the hit test.

use super::*;

//#region 🗺️Scene
/// 👁 Every addressable object in painting order.
pub fn objects(snapshot: &PdfSnapshot) -> Vec<PageObject> {
    let mut found = Vec::new();
    for (page_index, page) in snapshot.pages.iter().enumerate() {
        found.extend(page_objects(snapshot, page_index, page));
    }
    found
}

/// 🖼️ Host canvas layers for the whole document.
pub fn canvas_layers(snapshot: &PdfSnapshot, selected: Option<&str>) -> String {
    let placed = placement(snapshot);
    let mut layers = Vec::new();
    let mut top = 0.0;
    for (page_index, page) in snapshot.pages.iter().enumerate() {
        let (width, height) = displayed_size(page);
        layers.push(json!({ "id": format!("p{page_index}:paper"), "segments": rect_segments(0.0, top, width, height), "fill": { "color": [1.0, 1.0, 1.0, 1.0] }, "stroke": { "color": [0.75, 0.75, 0.78, 1.0], "width": 1.0 } }));
        top += height + 16.0;
    }
    for item in &placed {
        let color = [item.object.fill[0] as f32, item.object.fill[1] as f32, item.object.fill[2] as f32, 1.0];
        let stroke = [item.object.stroke[0] as f32, item.object.stroke[1] as f32, item.object.stroke[2] as f32, 1.0];
        match item.object.kind {
            ObjectKind::Text => {
                layers.push(json!({
                    "id": item.object.id,
                    "kind": "text",
                    "x": item.canvas_x,
                    "y": item.canvas_y,
                    "width": item.canvas_w.max(1.0),
                    "height": item.canvas_h.max(1.0),
                    "text": { "content": item.object.text, "size": item.object.font_size.max(1.0) },
                    "fill": { "color": color },
                }));
            }
            ObjectKind::Vector => {
                let page = &snapshot.pages[item.object.page];
                let segments = vector_segments(page, item.top, &page.content[item.object.start..=item.object.end.min(page.content.len().saturating_sub(1))]);
                layers.push(json!({ "id": item.object.id, "segments": segments, "fill": { "color": color }, "stroke": { "color": stroke, "width": item.object.line_width.max(0.5) } }));
            }
            ObjectKind::Image | ObjectKind::Form | ObjectKind::Shading | ObjectKind::Annotation => {
                layers.push(json!({ "id": item.object.id, "segments": rect_segments(item.canvas_x, item.canvas_y, item.canvas_w.max(1.0), item.canvas_h.max(1.0)), "fill": { "color": if item.object.kind == ObjectKind::Image { [0.85, 0.88, 0.92, 1.0] } else { color } }, "stroke": { "color": stroke, "width": 1.0 } }));
                if !item.object.text.is_empty() {
                    layers.push(json!({ "id": format!("{}.label", item.object.id), "kind": "text", "x": item.canvas_x + 4.0, "y": item.canvas_y + 4.0, "width": item.canvas_w.max(1.0), "height": 12.0, "text": { "content": item.object.text, "size": 12.0 }, "fill": { "color": [0.1, 0.1, 0.12, 1.0] } }));
                }
            }
        }
        if selected == Some(item.object.id.as_str()) {
            layers.push(json!({ "id": format!("{}.selection", item.object.id), "segments": rect_segments(item.canvas_x, item.canvas_y, item.canvas_w.max(1.0), item.canvas_h.max(1.0)), "stroke": { "color": [0.1, 0.45, 0.95, 1.0], "width": 2.0 } }));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

/// 🎯 Top-most object whose displayed bounds contain the canvas point.
pub fn hit_test(snapshot: &PdfSnapshot, x: f64, y: f64) -> Option<String> {
    placement(snapshot).into_iter().rev().find(|item| x >= item.canvas_x && y >= item.canvas_y && x <= item.canvas_x + item.canvas_w && y <= item.canvas_y + item.canvas_h).map(|item| item.object.id)
}
//#endregion 🗺️Scene
