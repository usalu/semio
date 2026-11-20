//! 🧮️ Converts transformed Canvas2d clip quadrilaterals to non-overlapping native logical pixel bands.

pub(super) fn polygon_clip_bands(polygons: &[[[f64; 2]; 4]], bounds: [f64; 4]) -> Vec<[u32; 4]> {
    let [mut left, mut top, mut right, mut bottom] = bounds;
    for polygon in polygons {
        if !polygon.iter().flatten().copied().all(f64::is_finite) {
            return Vec::new();
        }
        let (min_x, max_x, min_y, max_y) = polygon.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
            |(min_x, max_x, min_y, max_y), &[x, y]| (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y)),
        );
        (left, top, right, bottom) = (left.max(min_x), top.max(min_y), right.min(max_x), bottom.min(max_y));
    }
    if right <= left || bottom <= top {
        return Vec::new();
    }
    let pixel_edge = |coordinate: f64| (coordinate - 0.5).ceil().max(0.0) as u32;
    let mut bands: Vec<[u32; 4]> = Vec::new();
    for row in pixel_edge(top)..pixel_edge(bottom) {
        let y = f64::from(row) + 0.5;
        let (mut start, mut end) = (left, right);
        for polygon in polygons {
            let (l, r) = polygon_row(polygon, y);
            (start, end) = (start.max(l), end.min(r));
        }
        let x = pixel_edge(start);
        let width = pixel_edge(end).saturating_sub(x);
        if width == 0 {
            continue;
        }
        if let Some([_, _, _, height]) = bands.last_mut().filter(|[px, py, pw, ph]| *px == x && *pw == width && *py + *ph == row) {
            *height += 1;
        } else {
            bands.push([x, row, width, 1]);
        }
    }
    bands
}

fn polygon_row(polygon: &[[f64; 2]; 4], y: f64) -> (f64, f64) {
    let mut left = f64::INFINITY;
    let mut right = f64::NEG_INFINITY;
    for (&[ax, ay], &[bx, by]) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
        if (ay <= y && y < by) || (by <= y && y < ay) {
            let x = ax + (y - ay) * (bx - ax) / (by - ay);
            left = left.min(x);
            right = right.max(x);
        }
    }
    (left, right)
}


/// 🌿 Intersects compound path rows and convex form clips at native pixel centers.
pub(super) fn path_clip_bands(
    polygons: &[[[f64; 2]; 4]], paths: &[Vec<crate::canvas2d_paint::Trapezoid>], bounds: [f64; 4],
) -> Vec<[u32; 4]> {
    let [left, top, right, bottom] = bounds;
    if !bounds.into_iter().all(f64::is_finite) || right <= left || bottom <= top
        || polygons.iter().flatten().flatten().copied().any(|point| !point.is_finite()) {
        return Vec::new();
    }
    let edge = |coordinate: f64| (coordinate - 0.5).ceil().max(0.0) as u32;
    let mut bands: Vec<[u32; 4]> = Vec::new();
    let mut group_at = 0;
    let (mut allowed, mut row_path, mut merged, mut clipped) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for row in edge(top)..edge(bottom) {
        let y = f64::from(row) + 0.5;
        let mut limits = (left, right);
        for polygon in polygons {
            let (start, end) = polygon_row(polygon, y);
            limits = (limits.0.max(start), limits.1.min(end));
        }
        let (first, last) = (edge(limits.0), edge(limits.1));
        if first >= last { continue; }
        allowed.clear();
        allowed.push((first, last));
        for path in paths {
            row_path.clear();
            for slab in path.iter().filter(|slab| slab.top <= y && y < slab.bottom) {
                let ratio = (y - slab.top) / (slab.bottom - slab.top);
                let start = edge(slab.top_left + ratio * (slab.bottom_left - slab.top_left));
                let end = edge(slab.top_right + ratio * (slab.bottom_right - slab.top_right));
                if start < end { row_path.push((start, end)); }
            }
            row_path.sort_unstable();
            merged.clear();
            for &(left, right) in &row_path {
                if let Some((_, previous)) = merged.last_mut().filter(|(_, end)| left <= *end) {
                    *previous = (*previous).max(right);
                } else {
                    merged.push((left, right));
                }
            }
            clipped.clear();
            intersect_spans(&allowed, &merged, &mut clipped);
            std::mem::swap(&mut allowed, &mut clipped);
            if allowed.is_empty() { break; }
        }
        if allowed.is_empty() { continue; }
        let identical = bands[group_at..].len() == allowed.len()
            && bands[group_at..].iter().zip(&allowed).all(|(&[x, y, width, height], &(left, right))| {
                x == left && width == right - left && y + height == row
            });
        if identical {
            for band in &mut bands[group_at..] { band[3] += 1; }
        } else {
            group_at = bands.len();
            bands.extend(allowed.iter().map(|&(left, right)| [left, row, right - left, 1]));
        }
    }
    bands
}

fn intersect_spans(left: &[(u32, u32)], right: &[(u32, u32)], out: &mut Vec<(u32, u32)>) {
    let (mut i, mut j) = (0, 0);
    while let (Some(&(a, b)), Some(&(c, d))) = (left.get(i), right.get(j)) {
        let (start, end) = (a.max(c), b.min(d));
        if start < end { out.push((start, end)); }
        if b <= d { i += 1; } else { j += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::polygon_clip_bands;

    #[test]
    fn skewed_clip_admits_only_pixels_inside_pdf_form() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🧪️oriented-form-clip.json")).unwrap();
        let polygon: [[f64; 2]; 4] = serde_json::from_value(fixture["polygon"].clone()).unwrap();
        let bands = polygon_clip_bands(&[polygon], [0.0, 0.0, 100.0, 100.0]);
        let contains = |x, y| bands.iter().any(|&[left, top, width, height]| (left..left + width).contains(&x) && (top..top + height).contains(&y));
        for probe in fixture["probes"].as_array().unwrap() {
            let x = probe["at"][0].as_u64().unwrap() as u32;
            let y = probe["at"][1].as_u64().unwrap() as u32;
            assert_eq!(contains(x, y), probe["rgb"][1] == 0, "probe {x},{y}");
        }
        assert!(bands.windows(2).all(|pair| pair[0][1] + pair[0][3] <= pair[1][1]));
        let mirrored = [polygon[3], polygon[2], polygon[1], polygon[0]];
        assert_eq!(polygon_clip_bands(&[mirrored], [0.0, 0.0, 100.0, 100.0]), bands);
        let inner = [[26.0, 74.0], [37.0, 67.0], [41.0, 59.0], [30.0, 66.0]];
        let nested = polygon_clip_bands(&[polygon, inner], [0.0, 0.0, 100.0, 100.0]);
        assert!(nested.len() < bands.len());
        assert!(nested.iter().all(|&[x, y, width, height]| (y..y + height).all(|row| (x..x + width).all(|column| contains(column, row)))));
    }

    #[test]
    fn native_clip_bands_match_the_pdf_compound_path_and_holes() {
        use crate::canvas2d_paint::{fill_trapezoids, flatten, Affine, FillRule, ScenePathSegment};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🧪️path-clips.json")).unwrap();
        let (width, height) = (fixture["width"].as_f64().unwrap(), fixture["height"].as_f64().unwrap());
        let mut pixel = vec![[255; 3]; (width * height) as usize];
        let fill = |segments: &serde_json::Value, rule| {
            let segments: Vec<ScenePathSegment> = serde_json::from_value(segments.clone()).unwrap();
            fill_trapezoids(&flatten(&segments, &Affine::node(None)), FillRule::of(rule), (0.0, height))
        };
        for layer in fixture["layers"].as_array().unwrap() {
            let mut fills = vec![fill(&layer["segments"], Some("nonzero"))];
            for clip in layer["clipPaths"].as_array().into_iter().flatten() {
                fills.push(fill(&clip["segments"], clip["fillRule"].as_str()));
            }
            let polygons = serde_json::from_value::<Vec<[[f64; 2]; 4]>>(layer["clipPolygons"].clone()).unwrap_or_default();
            let bands = super::path_clip_bands(&polygons, &fills, [0.0, 0.0, width, height]);
            let color = std::array::from_fn(|channel| (layer["fill"]["color"][channel].as_f64().unwrap() * 255.0) as u8);
            for [x, y, w, h] in bands {
                for row in y..y+h {
                    for column in x..x+w { pixel[row as usize * width as usize + column as usize] = color; }
                }
            }
        }
        for probe in fixture["probes"].as_array().unwrap() {
            let x = probe["at"][0].as_u64().unwrap() as usize;
            let y = probe["at"][1].as_u64().unwrap() as usize;
            let expected = serde_json::from_value::<[u8; 3]>(probe["rgb"].clone()).unwrap();
            assert_eq!(pixel[y * width as usize + x], expected, "native clip at {x},{y}");
        }
    }

}
