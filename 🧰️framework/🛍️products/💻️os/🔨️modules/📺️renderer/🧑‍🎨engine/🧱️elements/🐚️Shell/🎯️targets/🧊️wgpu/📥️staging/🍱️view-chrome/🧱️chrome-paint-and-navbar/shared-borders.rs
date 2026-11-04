//! 🧩 Semantic chrome frame composition; never merges across positive gaps or overlay layers.

use semio_framework_os_renderer_wgpu_dock::shared_borders::{resolve_border_segments, BorderPoint, BorderSegment, BorderState};

pub struct ChromeBorderFrame {
    pub owner: String,
    pub rect: [f32; 4],
    pub state: BorderState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChromeBorderPaint {
    pub rect: [f32; 4],
    pub state: BorderState,
}

/// 📐 Returns each owner's resolved inward strokes, including split parent/control edges.
pub fn chrome_border_plan(frames: &[ChromeBorderFrame], stroke: f32) -> Result<std::collections::HashMap<String, Vec<ChromeBorderPaint>>, &'static str> {
    let mut segments = Vec::new();
    let mut inward = std::collections::HashMap::new();
    for frame in frames {
        let [x, y, width, height] = frame.rect;
        for (side, from, to, inset) in [
            ("top", (x, y), (x + width, y), 0.0),
            ("bottom", (x, y + height), (x + width, y + height), -stroke),
            ("left", (x, y), (x, y + height), 0.0),
            ("right", (x + width, y), (x + width, y + height), -stroke),
        ] {
            let id = format!("{}:{side}", frame.owner);
            inward.insert(id.clone(), inset);
            segments.push(BorderSegment { id, owner: frame.owner.clone(), from: BorderPoint { x: from.0 as f64, y: from.1 as f64 }, to: BorderPoint { x: to.0 as f64, y: to.1 as f64 }, state: frame.state });
        }
    }
    let mut output: std::collections::HashMap<String, Vec<ChromeBorderPaint>> = frames.iter().map(|frame| (frame.owner.clone(), Vec::new())).collect();
    for resolved in resolve_border_segments(&segments)? {
        let edge = resolved.segment;
        let inset = inward[&edge.id];
        let horizontal = edge.from.y == edge.to.y;
        let rect = if horizontal {
            [edge.from.x as f32, edge.from.y as f32 + inset, (edge.to.x - edge.from.x) as f32, stroke]
        } else {
            [edge.from.x as f32 + inset, edge.from.y as f32, stroke, (edge.to.y - edge.from.y) as f32]
        };
        output.entry(edge.owner).or_default().push(ChromeBorderPaint { rect, state: edge.state });
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sibling_hover_beats_active_in_both_orders_without_erasing_detached_frames() {
        let mut frames = vec![
            ChromeBorderFrame { owner: "active".into(), rect: [0.0, 0.0, 40.0, 20.0], state: BorderState::Active },
            ChromeBorderFrame { owner: "hover".into(), rect: [40.0, 0.0, 40.0, 20.0], state: BorderState::Hover },
            ChromeBorderFrame { owner: "detached".into(), rect: [100.0, 0.0, 40.0, 20.0], state: BorderState::Normal },
        ];
        let plan = chrome_border_plan(&frames, 1.0).unwrap();
        assert_eq!(plan["active"].len(), 3);
        assert_eq!(plan["hover"].len(), 4);
        assert_eq!(plan["detached"].len(), 4);
        assert!(plan["hover"].iter().any(|paint| paint.rect == [40.0, 0.0, 1.0, 20.0] && paint.state == BorderState::Hover));
        frames.reverse();
        assert_eq!(plan, chrome_border_plan(&frames, 1.0).unwrap());
    }

    #[test]
    fn child_hover_only_owns_coincident_parent_interval() {
        let frames = vec![
            ChromeBorderFrame { owner: "parent".into(), rect: [0.0, 0.0, 100.0, 80.0], state: BorderState::Active },
            ChromeBorderFrame { owner: "child".into(), rect: [20.0, 0.0, 30.0, 20.0], state: BorderState::Hover },
        ];
        let plan = chrome_border_plan(&frames, 1.0).unwrap();
        assert!(plan["parent"].iter().any(|paint| paint.rect == [0.0, 0.0, 20.0, 1.0]));
        assert!(plan["parent"].iter().any(|paint| paint.rect == [50.0, 0.0, 50.0, 1.0]));
        assert!(plan["child"].iter().any(|paint| paint.rect == [20.0, 0.0, 30.0, 1.0]));
    }
}
