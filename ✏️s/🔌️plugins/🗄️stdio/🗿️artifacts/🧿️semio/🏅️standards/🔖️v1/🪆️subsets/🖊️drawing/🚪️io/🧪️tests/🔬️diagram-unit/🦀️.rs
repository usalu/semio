use super::*;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn board() -> SemioDiagram {
    SemioDiagram {
        nodes: vec![
            SemioDiagramNode { id: "a".into(), x: 0.0, y: 0.0, shape: SemioDiagramShape::Circle { radius: 10.0 }, label: Some("A".into()) },
            SemioDiagramNode { id: "b".into(), x: 100.0, y: 0.0, shape: SemioDiagramShape::Rectangle { width: 40.0, height: 20.0 }, label: None },
        ],
        links: vec![SemioDiagramLink { from: "a".into(), to: "b".into(), label: None }, SemioDiagramLink { from: "a".into(), to: "missing".into(), label: None }],
        frames: vec![SemioDiagramFrame { x: -20.0, y: -40.0, width: 160.0, height: 80.0, label: Some("goal".into()) }],
    }
}

#[test]
fn links_run_boundary_to_boundary_and_skip_unknown_ends() {
    let drawing = diagram_drawing(&board());
    let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("group root") };
    let links: Vec<&Vec<PathSegment>> = children.iter().filter_map(|n| if let DrawNode::Path { segments, style: Some(s) } = n { (s == "diagram-link").then_some(segments) } else { None }).collect();
    assert_eq!(links.len(), 1);
    let [PathSegment::MoveTo { to: start }, PathSegment::LineTo { to: end }] = links[0].as_slice() else { panic!("a straight link") };
    assert_eq!((start.x - 32.0 + (-20.0), end.x - 32.0 + (-20.0)), (10.0, 80.0), "circle edge at x=10, rectangle edge at x=80");
}

#[test]
fn the_canvas_holds_every_frame_and_node_with_padding() {
    let drawing = diagram_drawing(&board());
    assert_eq!((drawing.canvas.width, drawing.canvas.height), (160.0 + 64.0, 80.0 + 64.0));
}

#[test]
fn an_empty_diagram_is_a_small_blank_canvas() {
    let drawing = diagram_drawing(&SemioDiagram::default());
    assert_eq!((drawing.canvas.width, drawing.canvas.height), (64.0, 64.0));
}
