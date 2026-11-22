//! 🧪 Group alpha marks exactly the layers painted inside the group, with glass and nested scissors included.

use super::*;
use semio_ui_theme::geometry::Rect;

fn solid(draw: &mut DrawList) {
    draw.push_solid([0.0, 0.0, 4.0, 4.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
}

#[test]
fn group_layers_span_from_begin_to_end_across_scissors_and_glass() {
    let mut draw = DrawList::default();
    solid(&mut draw);
    let mark = draw.begin_group_alpha(Rect::new(2.0, 2.0, 20.0, 10.0), 0.5);
    solid(&mut draw);
    draw.push_scissor(Rect::new(0.0, 0.0, 8.0, 8.0));
    solid(&mut draw);
    draw.pop_scissor();
    let region = draw.push_glass([0.0, 0.0, 8.0, 8.0], 2.0, GlassStyle { tint: Rgba::new(1.0, 1.0, 1.0, 0.5), alpha: 0.5, blur_px: 0.0, saturate: 1.0 });
    draw.end_group_alpha(mark);
    solid(&mut draw);
    let groups: Vec<Option<GroupAlpha>> = draw.layers.iter().map(|layer| layer.group).collect();
    assert_eq!(groups[0], None);
    let begin = groups[1].expect("begin layer");
    assert!(begin.begin && begin.alpha == 0.5 && begin.bounds == ScissorRect { x: 2, y: 2, w: 20, h: 10 });
    assert!(groups[2..groups.len() - 1].iter().all(|group| *group == Some(begin.continued())));
    assert_eq!(*groups.last().unwrap(), None);
    assert!(draw.glass_regions[region].layer_index < draw.layers.len() - 1);
    assert_eq!(draw.layers[draw.glass_regions[region].layer_index].group, Some(begin.continued()));
}

#[test]
fn full_alpha_nested_and_unmatched_groups_are_no_ops() {
    let mut draw = DrawList::default();
    let none = draw.begin_group_alpha(Rect::new(0.0, 0.0, 4.0, 4.0), 1.0);
    draw.end_group_alpha(none);
    assert_eq!(draw.layers.len(), 1);
    let outer = draw.begin_group_alpha(Rect::new(0.0, 0.0, 4.0, 4.0), 0.5);
    let inner = draw.begin_group_alpha(Rect::new(0.0, 0.0, 2.0, 2.0), 0.25);
    draw.end_group_alpha(inner);
    assert!(draw.layers.last().unwrap().group.is_some());
    draw.end_group_alpha(outer);
    assert!(draw.layers.last().unwrap().group.is_none());
}
