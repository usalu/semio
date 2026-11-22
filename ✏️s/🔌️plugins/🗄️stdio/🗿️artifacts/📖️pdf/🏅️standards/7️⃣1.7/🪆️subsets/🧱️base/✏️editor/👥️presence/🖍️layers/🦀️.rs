//! 🖍️ Collaborator overlay layers: the scene layers a page window draws above the page for the peers on it. Away peers keep their selection frames, dimmed; only live peers draw a pointer, viewport and caret.
//! [Canvas scene layers](https://www.w3.org/TR/css-transforms-1/)

use crate::{PeerMark, PeerOverlay, PeerStatus};
use serde_json::{json, Value};

//#region 🖍️Layers
fn color(mark: &PeerMark, alpha: f64) -> [f64; 4] {
    let [red, green, blue] = mark.rgb();
    let alpha = if mark.status == PeerStatus::Away { alpha * 0.4 } else { alpha };
    [f64::from(red) / 255.0, f64::from(green) / 255.0, f64::from(blue) / 255.0, alpha]
}

fn rect(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([{ "kind": "move", "to": [x, y] }, { "kind": "line", "to": [x + width, y] }, { "kind": "line", "to": [x + width, y + height] }, { "kind": "line", "to": [x, y + height] }, { "kind": "close" }])
}

/// 🖍️ The layers of every peer mark on the overlay's page. `page` is the page's `[x, y, width, height]` in canvas units and `object` resolves an object id to its canvas rectangle; marks whose object is unknown are skipped.
pub fn peer_layers(overlay: &PeerOverlay, page: [f64; 4], object: &dyn Fn(&str) -> Option<[f64; 4]>) -> Vec<Value> {
    let [px, py, pw, ph] = page;
    let at = |x: u32, y: u32| (px + pw * f64::from(x) / 1000.0, py + ph * f64::from(y) / 1000.0);
    let mut layers = Vec::new();
    for selection in &overlay.selections {
        if let Some([x, y, w, h]) = object(&selection.object) {
            layers.push(json!({ "id": format!("peer.{}.selection.{}", selection.mark.actor, selection.object), "segments": rect(x, y, w.max(1.0), h.max(1.0)), "stroke": { "color": color(&selection.mark, 1.0), "width": 3.0 } }));
        }
    }
    for viewport in &overlay.viewports {
        let (left, top) = at(viewport.left, viewport.top);
        let (right, bottom) = at(viewport.right, viewport.bottom);
        layers.push(json!({ "id": format!("peer.{}.viewport", viewport.mark.actor), "segments": rect(left, top, right - left, bottom - top), "stroke": { "color": color(&viewport.mark, 0.9), "width": 2.0 } }));
    }
    for caret in &overlay.carets {
        if let Some([x, y, _, h]) = object(&caret.object) {
            layers.push(json!({ "id": format!("peer.{}.caret", caret.mark.actor), "segments": rect(x, y, 2.0, h.max(1.0)), "fill": { "color": color(&caret.mark, 1.0) } }));
        }
    }
    for pointer in &overlay.pointers {
        let (x, y) = at(pointer.x, pointer.y);
        let segments = json!([{ "kind": "move", "to": [x, y] }, { "kind": "line", "to": [x + 9.0, y + 22.0] }, { "kind": "line", "to": [x + 13.0, y + 14.0] }, { "kind": "line", "to": [x + 22.0, y + 9.0] }, { "kind": "close" }]);
        layers.push(json!({ "id": format!("peer.{}.pointer", pointer.mark.actor), "segments": segments, "fill": { "color": color(&pointer.mark, 1.0) }, "stroke": { "color": [1.0, 1.0, 1.0, 1.0], "width": 1.5 } }));
    }
    layers
}
//#endregion 🖍️Layers
