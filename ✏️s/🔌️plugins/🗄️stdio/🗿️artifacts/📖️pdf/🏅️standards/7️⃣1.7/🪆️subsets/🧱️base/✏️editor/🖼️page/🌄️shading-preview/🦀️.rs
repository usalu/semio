//! 🌄️ Projects supported PDF shadings onto the editor canvas.

use super::*;

pub(super) fn shading_layer(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed) -> Option<Value> {
    let shading = snapshot.shadings.iter().find(|shading| shading.id == item.object.text)?;
    let PdfShadingKind::Axial { coords: [x1, y1, x2, y2], domain, function, extend } = &shading.kind else { return None };
    let PdfFunction::Exponential { domain: function_domain, range, c0, c1, n } = function else { return None };
    let components = match shading.color_space {
        PdfColorSpace::DeviceGray => 1,
        PdfColorSpace::DeviceRgb => 3,
        _ => return None,
    };
    if c0.len() != components || c1.len() != components || function_domain.len() != 2 || function_domain[0] > function_domain[1] {
        return None;
    }
    let squared_length = (x2 - x1).powi(2) + (y2 - y1).powi(2);
    if !squared_length.is_finite() || squared_length <= 0.0 {
        return None;
    }
    let (box_coords, to_local) = match shading.bbox {
        Some(rect) => (rect, PDF_IDENTITY_MATRIX),
        None => (page.media_box, invert(item.object.ctm)?),
    };
    let [left, bottom, right, top] = box_coords;
    let mut polygon = vec![apply(to_local, left, bottom), apply(to_local, right, bottom), apply(to_local, right, top), apply(to_local, left, top)];
    let projection = |(x, y): (f64, f64)| ((x - x1) * (x2 - x1) + (y - y1) * (y2 - y1)) / squared_length;
    if !extend[0] {
        polygon = clip_polygon(&polygon, &projection);
    }
    if !extend[1] {
        polygon = clip_polygon(&polygon, |point| 1.0 - projection(point));
    }
    let mut segments = Vec::new();
    for (index, point) in polygon.into_iter().enumerate() {
        segments.push(json!({ "kind": if index == 0 { "move" } else { "line" }, "to": point }));
    }
    if !segments.is_empty() {
        segments.push(json!({ "kind": "close" }));
    }
    let [start, end] = domain.unwrap_or([0.0, 1.0]);
    let steps = if *n == 1.0 && range.is_none() && start >= function_domain[0] && end <= function_domain[1] { 1 } else { 32 };
    let mut stops = Vec::with_capacity(steps + 1);
    for index in 0..=steps {
        let offset = index as f64 / steps as f64;
        let input = (start + offset * (end - start)).max(function_domain[0]).min(function_domain[1]);
        let weight = input.powf(*n);
        if !weight.is_finite() {
            return None;
        }
        let mut color = [0.0; 4];
        for (channel, output) in color[..3].iter_mut().enumerate() {
            let channel = channel.min(components - 1);
            let mut value = c0[channel] + weight * (c1[channel] - c0[channel]);
            if let Some(limits) = range.as_ref().and_then(|r| r.get(2 * channel..2 * channel + 2)) {
                value = value.max(limits[0]).min(limits[1]);
            }
            *output = value.clamp(0.0, 1.0);
        }
        color[3] = 1.0;
        stops.push(json!({ "offset": offset, "color": color }));
    }
    let display = |x, y| {
        let (x, y) = apply(item.object.ctm, x, y);
        display_point(page, item.top, x, y)
    };
    let (x, y) = display(0.0, 0.0);
    let (a, b) = display(1.0, 0.0);
    let (c, d) = display(0.0, 1.0);
    Some(json!({
        "id": item.object.id,
        "segments": segments,
        "transform": [a - x, b - y, c - x, d - y, x, y],
        "fill": { "kind": "linearGradient", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "stops": stops },
    }))
}

fn clip_polygon(polygon: &[(f64, f64)], signed_distance: impl Fn((f64, f64)) -> f64) -> Vec<(f64, f64)> {
    let Some(&last) = polygon.last() else { return Vec::new() };
    let mut clipped = Vec::new();
    let mut previous = last;
    let mut previous_distance = signed_distance(previous);
    for &point in polygon {
        let distance = signed_distance(point);
        if (distance < 0.0) != (previous_distance < 0.0) {
            let ratio = previous_distance / (previous_distance - distance);
            clipped.push((previous.0 + ratio * (point.0 - previous.0), previous.1 + ratio * (point.1 - previous.1)));
        }
        if distance >= 0.0 {
            clipped.push(point);
        }
        previous = point;
        previous_distance = distance;
    }
    clipped
}
