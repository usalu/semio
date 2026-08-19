// #region layout
//! 🧮️ Flex stack layout for widget trees.

use crate::wgpu::geometry::Rect;
use crate::wgpu::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub async fn gap_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        Some("tight") => 4.0,
        Some("loose") => 12.0,
        Some("none") | Some("0") => 0.0,
        _ => theme.gap_standard,
    }
}

pub async fn padding_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        Some("none") | Some("0") => 0.0,
        Some("tight") => 6.0,
        Some("loose") => 16.0,
        _ => theme.padding_standard,
    }
}

pub async fn layout_vertical(bounds: Rect, gap: f32, padding: f32, child_heights: &[f32]) -> Vec<Rect> {
    let inner = bounds.inset(padding);
    let total_gap = gap * (child_heights.len().saturating_sub(1) as f32);
    let total_children: f32 = child_heights.iter().sum();
    let mut y = inner.y;
    let mut rects = Vec::with_capacity(child_heights.len());
    let available = (inner.h - total_gap - total_children).max(0.0);
    let extra_per_child = if child_heights.is_empty() { 0.0 } else { available / child_heights.len() as f32 };
    for &height in child_heights {
        let h = height + extra_per_child;
        rects.push(Rect::new(inner.x, y, inner.w, h));
        y += h + gap;
    }
    rects
}

pub async fn layout_horizontal(bounds: Rect, gap: f32, padding: f32, child_widths: &[f32]) -> Vec<Rect> {
    let inner = bounds.inset(padding);
    let total_gap = gap * (child_widths.len().saturating_sub(1) as f32);
    let total_children: f32 = child_widths.iter().sum();
    let mut x = inner.x;
    let mut rects = Vec::with_capacity(child_widths.len());
    let available = (inner.w - total_gap - total_children).max(0.0);
    let extra_per_child = if child_widths.is_empty() { 0.0 } else { available / child_widths.len() as f32 };
    for &width in child_widths {
        let w = width + extra_per_child;
        rects.push(Rect::new(x, inner.y, w, inner.h));
        x += w + gap;
    }
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn vertical_layout_distributes_children() {
        let theme = Theme::default();
        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rects = layout_vertical(bounds, 4.0, 8.0, &[20.0, 30.0]);
        assert_eq!(rects.len(), 2);
        assert!(rects[0].h > 20.0);
        assert!(rects[1].y > rects[0].y);
        let _ = theme;
    }
}
// #endregion layout
