//! 🧾️ SVG and PDF export of the drawing kernel: the color, path data, SVG serialization and PDF operations. See [`super`].

use super::*;

// #region 🔖️Export
fn color_css(color: [f64; 4]) -> String {
    let r = (color[0] * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (color[1] * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (color[2] * 255.0).round().clamp(0.0, 255.0) as u8;
    let a = color[3].clamp(0.0, 1.0);
    if (a - 1.0).abs() < f64::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({r},{g},{b},{a:.3})")
    }
}

fn segments_to_svg_d(segments: &[semio_framework_2d::PathSegment]) -> String {
    use semio_framework_2d::PathSegment;
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("M {} {} ", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("L {} {} ", to[0], to[1])),
            PathSegment::Quad { ctrl, to } => out.push_str(&format!("Q {} {} {} {} ", ctrl[0], ctrl[1], to[0], to[1])),
            PathSegment::Cubic { ctrl1, ctrl2, to } => out.push_str(&format!("C {} {} {} {} {} {} ", ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1])),
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                out.push_str(&format!("A {} {} {} {} {} {} {} ", rx, ry, rotation, if *large_arc { 1 } else { 0 }, if *sweep { 1 } else { 0 }, to[0], to[1]));
            }
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

pub(super) fn serialize_svg(scene: &DrawingScene) -> String {
    let mut body = String::new();
    for node in &scene.nodes {
        match &node.node {
            DrawingNode::Text { x, y, content, size } => {
                let [tx, ty] = node.transform.transform_point([*x, *y]);
                let fill = node.fill.as_ref().map_or_else(
                    || "black".into(),
                    |f| match f {
                        FillStyle::Solid { color } => color_css(*color),
                        _ => "black".into(),
                    },
                );
                body.push_str(&format!(r#"<text x="{tx}" y="{ty}" font-size="{size}" fill="{fill}">{content}</text>"#));
            }
            _ => {
                let segments = DrawingStore::node_to_segments(&node.node);
                if segments.is_empty() {
                    continue;
                }
                let d = segments_to_svg_d(&segments);
                let mut attrs = format!(r#"d="{d}""#);
                if let Some(fill) = &node.fill {
                    match fill {
                        FillStyle::Solid { color } => attrs.push_str(&format!(r#" fill="{}""#, color_css(*color))),
                        FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                            let id = format!("lg{}", body.len());
                            body.push_str(&format!(r#"<defs><linearGradient id="{id}" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</linearGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                        FillStyle::RadialGradient { cx, cy, r, stops } => {
                            let id = format!("rg{}", body.len());
                            body.push_str(&format!(r#"<defs><radialGradient id="{id}" cx="{cx}" cy="{cy}" r="{r}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</radialGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                    }
                } else {
                    attrs.push_str(r#" fill="none""#);
                }
                if let Some(stroke) = &node.stroke {
                    attrs.push_str(&format!(r#" stroke="{}" stroke-width="{}""#, color_css(stroke.color), stroke.width));
                }
                body.push_str(&format!("<path {attrs}/>"));
            }
        }
    }
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{body}</svg>"#, scene.width, scene.height, scene.width, scene.height)
}

fn segments_to_pdf_operations(segments: &[semio_framework_2d::PathSegment]) -> String {
    use semio_framework_2d::PathSegment;
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("{} {} m\n", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("{} {} l\n", to[0], to[1])),
            PathSegment::Close => out.push_str("h\n"),
            _ => {}
        }
    }
    out
}

pub(super) fn serialize_pdf(scene: &DrawingScene) -> Vec<u8> {
    let mut stream = String::new();
    stream.push_str("0.1 0.1 0.1 rg\n");
    for node in &scene.nodes {
        let segments = DrawingStore::node_to_segments(&node.node);
        if segments.is_empty() {
            continue;
        }
        stream.push_str(&segments_to_pdf_operations(&segments));
        if node.fill.is_some() {
            stream.push_str("f\n");
        } else if node.stroke.is_some() {
            stream.push_str("S\n");
        }
    }
    let content = stream.as_bytes();
    let objects = [
        "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n",
        "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n",
        &format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources << >> >>\nendobj\n", scene.width, scene.height),
        &format!("4 0 obj\n<< /Length {} >>\nstream\n", content.len()),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = Vec::new();
    for object in &objects[..3] {
        offsets.push(pdf.len());
        pdf.push_str(object);
    }
    offsets.push(pdf.len());
    pdf.push_str(objects[3]);
    pdf.push_str(std::str::from_utf8(content).unwrap_or(""));
    pdf.push_str("\nendstream\nendobj\n");
    let xref = pdf.len();
    pdf.push_str("xref\n0 5\n0000000000 65535 f \n");
    for offset in offsets {
        pdf.push_str(&format!("{:010} 00000 n \n", offset));
    }
    pdf.push_str(&format!("trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"));
    pdf.into_bytes()
}
// #endregion 🔖️Export
