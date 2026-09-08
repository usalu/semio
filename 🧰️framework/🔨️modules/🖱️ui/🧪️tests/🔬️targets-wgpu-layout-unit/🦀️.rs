
use super::*;

#[test]
fn vertical_layout_distributes_children() {
    let theme = Theme::default();
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let rects = layout_vertical(bounds, 4.0, 8.0, &[20.0, 30.0]);
    assert_eq!(rects.len(), 2);
    assert!(rects[0].h > 20.0);
    assert!(rects[1].y > rects[0].y);
    let _ = theme;
}
