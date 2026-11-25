//! 🌈️ Canvas shading, image and annotation paint against MuPDF probe colors from the neutral fixture.

use super::*;

const CURVE_STEPS: usize = 24;

fn case_pdf(case: &Value) -> String {
    let objects = case["objects"].as_array().unwrap();
    let [width, height] = [0, 1].map(|axis| case["page"][axis].as_f64().unwrap());
    let refs = (0..objects.len()).map(|index| format!("{} 0 R", index + 5)).collect::<Vec<_>>().join(" ");
    let annots = if case["kind"] == "annotation" { format!("/Annots [{refs}]") } else { String::new() };
    let content = case["content"].as_str().unwrap();
    let mut all = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] /Contents 4 0 R /Resources << {} >> {annots} >>", case["resources"].as_str().unwrap()),
        format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len()),
    ];
    for entry in objects {
        let dict = entry["dict"].as_str().unwrap();
        all.push(match entry["stream"].as_str() {
            Some(stream) => format!("{}/Length {} >>\nstream\n{stream}\nendstream", &dict[..dict.len() - 2], stream.len()),
            None => dict.to_string(),
        });
    }
    pdf_objects_bytes(&all)
}

fn contours(segments: &Value, transform: [f64; 6]) -> Vec<Vec<(f64, f64)>> {
    let map = |point: &Value| {
        let (x, y) = (point[0].as_f64().unwrap(), point[1].as_f64().unwrap());
        (transform[0] * x + transform[2] * y + transform[4], transform[1] * x + transform[3] * y + transform[5])
    };
    let mut found: Vec<Vec<(f64, f64)>> = Vec::new();
    for segment in segments.as_array().into_iter().flatten() {
        match segment["kind"].as_str().unwrap() {
            "move" => found.push(vec![map(&segment["to"])]),
            "line" => found.last_mut().unwrap().push(map(&segment["to"])),
            "cubic" => {
                let contour = found.last_mut().unwrap();
                let start = *contour.last().unwrap();
                let (a, b, c) = (map(&segment["ctrl1"]), map(&segment["ctrl2"]), map(&segment["to"]));
                for step in 1..=CURVE_STEPS {
                    let t = step as f64 / CURVE_STEPS as f64;
                    let u = 1.0 - t;
                    let blend = |p: f64, q: f64, r: f64, s: f64| u * u * u * p + 3.0 * u * u * t * q + 3.0 * u * t * t * r + t * t * t * s;
                    contour.push((blend(start.0, a.0, b.0, c.0), blend(start.1, a.1, b.1, c.1)));
                }
            }
            _ => {}
        }
    }
    found
}

fn covered(contours: &[Vec<(f64, f64)>], (x, y): (f64, f64)) -> bool {
    let mut inside = false;
    for contour in contours {
        for (index, &(x1, y1)) in contour.iter().enumerate() {
            let (x2, y2) = contour[(index + 1) % contour.len()];
            if (y1 > y) != (y2 > y) && x < x1 + (y - y1) * (x2 - x1) / (y2 - y1) { inside = !inside; }
        }
    }
    inside
}

fn near(contours: &[Vec<(f64, f64)>], (x, y): (f64, f64), half: f64, closed: bool) -> bool {
    contours.iter().any(|contour| contour.iter().copied().zip(contour.iter().copied().skip(1).chain(closed.then(|| contour[0]))).any(|(from, to)| {
        let ((x1, y1), (x2, y2)) = (from, to);
        let length = (x2 - x1).powi(2) + (y2 - y1).powi(2);
        let t = if length == 0.0 { 0.0 } else { (((x - x1) * (x2 - x1) + (y - y1) * (y2 - y1)) / length).clamp(0.0, 1.0) };
        ((x - x1 - t * (x2 - x1)).powi(2) + (y - y1 - t * (y2 - y1)).powi(2)).sqrt() <= half
    }))
}

fn inverse(matrix: [f64; 6]) -> [f64; 6] {
    let determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    let (a, b, c, d) = (matrix[3] / determinant, -matrix[1] / determinant, -matrix[2] / determinant, matrix[0] / determinant);
    [a, b, c, d, -(a * matrix[4] + c * matrix[5]), -(b * matrix[4] + d * matrix[5])]
}

fn gradient(fill: &Value, (x, y): (f64, f64)) -> [f64; 4] {
    let [x1, y1, x2, y2] = ["x1", "y1", "x2", "y2"].map(|key| fill[key].as_f64().unwrap());
    let t = (((x - x1) * (x2 - x1) + (y - y1) * (y2 - y1)) / ((x2 - x1).powi(2) + (y2 - y1).powi(2))).clamp(0.0, 1.0);
    let stops = fill["stops"].as_array().unwrap();
    let offsets: Vec<f64> = stops.iter().map(|stop| stop["offset"].as_f64().unwrap()).collect();
    let upper = offsets.iter().position(|offset| *offset >= t).unwrap_or(stops.len() - 1);
    let lower = upper.saturating_sub(1);
    let span = offsets[upper] - offsets[lower];
    let weight = if span == 0.0 { 0.0 } else { (t - offsets[lower]) / span };
    let colour = |stop: usize, channel: usize| stops[stop]["color"][channel].as_f64().unwrap();
    [0, 1, 2, 3].map(|channel| colour(lower, channel) + weight * (colour(upper, channel) - colour(lower, channel)))
}

fn composite(scene: &[Value], point: (f64, f64)) -> [f64; 3] {
    let mut rgb = [1.0; 3];
    for layer in scene {
        let transform: [f64; 6] = layer.get("transform").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let opacity = layer["opacity"].as_f64().unwrap_or(1.0);
        let local = {
            let back = inverse(transform);
            (back[0] * point.0 + back[2] * point.1 + back[4], back[1] * point.0 + back[3] * point.1 + back[5])
        };
        let mut samples: Vec<([f64; 3], f64)> = Vec::new();
        if layer["kind"] == "image" {
            let [x, y, width, height] = ["x", "y", "width", "height"].map(|key| layer[key].as_f64().unwrap());
            if (x..x + width).contains(&point.0) && (y..y + height).contains(&point.1) {
                let src = layer["image"]["src"].as_str().unwrap().strip_prefix("data:image/png;base64,").unwrap();
                let raster = semio_framework_pixels::decode_png(&semio_framework_io_base64::base64_standard_decode(src).unwrap()).unwrap();
                let column = (((point.0 - x) / width * f64::from(raster.width)) as u32).min(raster.width - 1);
                let row = (((point.1 - y) / height * f64::from(raster.height)) as u32).min(raster.height - 1);
                let offset = ((row * raster.width + column) * 4) as usize;
                let rgba: Vec<f64> = raster.pixels[offset..offset + 4].iter().map(|byte| f64::from(*byte) / 255.0).collect();
                samples.push(([rgba[0], rgba[1], rgba[2]], rgba[3] * opacity));
            }
        } else if let Some(fill) = layer.get("fill").filter(|fill| fill.is_object()) {
            if covered(&contours(&layer["segments"], [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]), local) {
                let rgba = if fill["kind"] == "linearGradient" { gradient(fill, local) } else { [0, 1, 2, 3].map(|channel| fill["color"][channel].as_f64().unwrap()) };
                samples.push(([rgba[0], rgba[1], rgba[2]], rgba[3] * opacity));
            }
        }
        if let Some(stroke) = layer.get("stroke").filter(|stroke| stroke.is_object() && stroke["color"].is_array()) {
            let scale = (transform[0] * transform[3] - transform[1] * transform[2]).abs().sqrt();
            let closed = layer["segments"].as_array().is_some_and(|segments| segments.iter().any(|segment| segment["kind"] == "close"));
            if near(&contours(&layer["segments"], [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]), local, stroke["width"].as_f64().unwrap_or(1.0) / 2.0 / scale.max(1e-9), closed) {
                let colour = &stroke["color"];
                samples.push(([0, 1, 2].map(|channel| colour[channel].as_f64().unwrap()), colour[3].as_f64().unwrap_or(1.0) * opacity));
            }
        }
        for (colour, alpha) in samples {
            for channel in 0..3 {
                let source = if layer["blendMode"] == "multiply" { rgb[channel] * colour[channel] } else { colour[channel] };
                rgb[channel] = rgb[channel] * (1.0 - alpha) + source * alpha;
            }
        }
    }
    rgb
}

#[test]
fn canvas_paint_matches_mupdf_probes() {
    use crate::standards::v1_7::subsets::base::io::import::decode_pdf;
    let fixture: Value = serde_json::from_str(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️paint-fidelity.json")).unwrap()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let snapshot = decode_pdf(case_pdf(case).as_bytes()).unwrap_or_else(|error| panic!("{name}: {error}"));
        let scene: Vec<Value> = serde_json::from_str(&canvas_layers(&snapshot, None, semio_ui_locale::Locale::En)).unwrap();
        assert!(scene.iter().all(|layer| !layer["id"].as_str().unwrap_or("").ends_with(".label")), "{name}: placeholder label painted");
        let tolerance = case["tolerance"].as_f64().unwrap();
        for probe in case["probes"].as_array().unwrap() {
            let [x, y] = [0, 1].map(|axis| probe["at"][axis].as_f64().unwrap());
            let actual = composite(&scene, (x + 0.5, y + 0.5)).map(|value| (value * 255.0).round());
            for channel in 0..3 {
                let expected = probe["rgb"][channel].as_f64().unwrap();
                assert!((actual[channel] - expected).abs() <= tolerance, "{name} at [{x}, {y}]: {actual:?} vs {:?}", probe["rgb"]);
            }
        }
        render_window(&snapshot, semio_ui_locale::Locale::En).expect("paint fidelity scene admission");
    }
}
