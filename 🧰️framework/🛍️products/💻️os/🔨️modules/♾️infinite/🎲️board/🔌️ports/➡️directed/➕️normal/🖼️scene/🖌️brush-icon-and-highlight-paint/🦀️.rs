//! 🖌️ Brush icon and highlight paint of the board scene: the brush node icon paint, the highlighted node preview and the drag delta. See [`super`].

use super::*;

fn append_brush_node_icon_paint(scene_state: &BoardSceneState, style: &BoardStyleState, scene: &mut Scene, lod: BoardDrawLod, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: &str, world_space: bool) {
    if !matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
        return;
    }
    let preserve_original_style = false;
    let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&style.canvas_theme);
    let Some(paint) = style.get_or_build_icon_paint(icon_kind, icon_fg, icon_bg, preserve_original_style) else {
        return;
    };
    let (bx, by, bw, bh) = paint.bounds();
    let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
    let fit_inset = ui_styling::metrics::icon::FIT_INSET;
    let (sx_half, sy_half) = match shape {
        NodeShape::Circle => {
            let s = scene_state.draw_space_len(radius, world_space) * fit_inset;
            (s, s)
        }
        NodeShape::Rectangle => (scene_state.draw_space_len(width, world_space) * fit_inset * 0.5, scene_state.draw_space_len(height, world_space) * fit_inset * 0.5),
    };
    let center_ds = scene_state.draw_space_point(center, world_space);
    let cx = bx + bw * 0.5;
    let cy = by + bh * 0.5;
    let avail_w = 2.0 * sx_half;
    let avail_h = 2.0 * sy_half;
    let scale = (avail_w / bw).min(avail_h / bh);
    let aff = Affine::IDENTITY.translate((center_ds.x - scale * cx, center_ds.y - scale * cy)) * Affine::IDENTITY.scale(scale);
    match shape {
        NodeShape::Circle => {
            let r_clip = scene_state.draw_space_len(radius, world_space) * clip_inset;
            let disc = Circle::new(center_ds, r_clip);
            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
            match paint.body() {
                CachedIconBody::Vector(icon_scene) => {
                    scene.append(icon_scene, Some(aff));
                }
                CachedIconBody::Raster(img) => {
                    scene.draw_image(img, aff);
                }
            }
            scene.pop_layer();
        }
        NodeShape::Rectangle => {
            let hw = scene_state.draw_space_len(width, world_space) * clip_inset * 0.5;
            let hh = scene_state.draw_space_len(height, world_space) * clip_inset * 0.5;
            let clip_r = Rect::from_points(Point::new(center_ds.x - hw, center_ds.y - hh), Point::new(center_ds.x + hw, center_ds.y + hh));
            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
            match paint.body() {
                CachedIconBody::Vector(icon_scene) => {
                    scene.append(icon_scene, Some(aff));
                }
                CachedIconBody::Raster(img) => {
                    scene.draw_image(img, aff);
                }
            }
            scene.pop_layer();
        }
    }
}

pub fn paint_highlighted_node_preview(scene_state: &BoardSceneState, style_state: &BoardStyleState, scene: &mut Scene, _lod: BoardDrawLod, x: f64, y: f64, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: Option<&str>, world_space: bool) {
    let center = Point::new(x, y);
    let style = BoardElementStyleKind::Highlighted;
    let fill = node_fill_for_style(&style_state.canvas_theme, style);
    let stroke_c = node_stroke_for_style(&style_state.canvas_theme, style);
    let stroke = Stroke::new(ui_styling::strokes::NODE_BODY);
    match shape {
        NodeShape::Circle => {
            let c = scene_state.draw_space_point(center, world_space);
            let r = scene_state.draw_space_len(radius, world_space);
            let circle = Circle::new(c, r);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
            scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &circle);
        }
        NodeShape::Rectangle => {
            let hw = width * 0.5;
            let hh = height * 0.5;
            let p0 = scene_state.draw_space_point(Point::new(x - hw, y - hh), world_space);
            let p1 = scene_state.draw_space_point(Point::new(x + hw, y + hh), world_space);
            let rect = Rect::from_points(p0, p1);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
            scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &rect);
        }
    }
    if let Some(icon) = icon_kind.map(str::trim).filter(|s| !s.is_empty()) {
        append_brush_node_icon_paint(scene_state, style_state, scene, BoardDrawLod::Detail, center, shape, radius, width, height, icon, world_space);
    }
}

/// 🧲️ The common offset a drag of `start_positions` grabbed at `offset` reaches at `world`, snapped to the grid
/// when snapping is on — the one rule the live preview, the release record and the wgpu plan all share.
pub fn drag_delta(scene: &BoardSceneState, style: &BoardStyleState, world: Point, primary_id: &str, offset: Vec2, start_positions: &BTreeMap<String, (f64, f64)>) -> Option<Vec2> {
    let (px0, py0) = start_positions.get(primary_id).copied()?;
    let (nx, ny) = (world.x - offset.x, world.y - offset.y);
    let (tx, ty) = if style.grid_snap_enabled { board_snap_world_pair(scene, style, nx, ny) } else { (nx, ny) };
    Some(Vec2::new(tx - px0, ty - py0))
}
