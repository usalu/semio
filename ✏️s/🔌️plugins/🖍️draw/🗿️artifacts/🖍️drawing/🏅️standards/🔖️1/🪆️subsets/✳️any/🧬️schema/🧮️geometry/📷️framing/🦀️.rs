//! 📷️ Visible world bounds shared by Drawing surfaces.
use crate::PathSegment;

/// 📷️ Frames visible transformed artwork, including an explicitly authored artboard.
pub fn drawing_scene_bounds(artboard: Option<&crate::DrawingArtboard>, nodes: &[crate::schema::DrawingSceneNode]) -> [f64;4] {
    let mut bounds = artboard.filter(|board| board.width > 0.0 && board.height > 0.0).map(|board| [0.0,0.0,board.width,board.height]);
    for node in nodes.iter().filter(|node| node.visible && node.opacity > 0.0) {
        let rectangle = node.image.as_ref().map(|image| (0.0,0.0,image.width,image.height)).or_else(|| node.text.as_ref().map(|text| { let [width,height] = semio_s_2d::text::drawing_text_fallback_extent(&text.content,text.size); (0.0,0.0,width,height) }));
        let shape;
        let segments = if let Some((x,y,w,h)) = rectangle {
            shape = vec![PathSegment::Move { to: [x,y] },PathSegment::Line { to: [x+w,y] },PathSegment::Line { to: [x+w,y+h] },PathSegment::Line { to: [x,y+h] },PathSegment::Close];
            &shape
        } else { &node.segments };
        let Some((x,y,w,h)) = crate::schema::path_segments_bounds_with_matrix(segments,node.transform) else { continue };
        let radius = node.stroke.as_ref().map_or(0.0,|stroke| stroke.width*0.5);
        let dx = radius*node.transform[0].hypot(node.transform[2]);
        let dy = radius*node.transform[1].hypot(node.transform[3]);
        let next = [x-dx,y-dy,x+w+dx,y+h+dy];
        bounds = Some(bounds.map_or(next,|old| [old[0].min(next[0]),old[1].min(next[1]),old[2].max(next[2]),old[3].max(next[3])]));
    }
    bounds.unwrap_or([0.0,0.0,1024.0,1024.0])
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
