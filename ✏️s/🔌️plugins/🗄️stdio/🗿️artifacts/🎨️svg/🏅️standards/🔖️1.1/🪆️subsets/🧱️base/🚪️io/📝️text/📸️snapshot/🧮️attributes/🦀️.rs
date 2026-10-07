//! 📝️ Native SVG attribute, path and typed geometry codecs.
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument,XmlNode,XmlAttr};
fn fmt_num(v: f64) -> String {
    v.to_string()
}

pub fn view_box_to_string(v: &ViewBox) -> String {
    format!("{} {} {} {}", fmt_num(v.min_x), fmt_num(v.min_y), fmt_num(v.width), fmt_num(v.height))
}

pub fn points_to_string(points: &[(f64, f64)]) -> String {
    points.iter().map(|(x, y)| format!("{},{}", fmt_num(*x), fmt_num(*y))).collect::<Vec<_>>().join(" ")
}

pub fn transform_list_to_string(ops: &[TransformOp]) -> String {
    ops.iter()
        .map(|op| match op {
            TransformOp::Matrix { a, b, c, d, e, f } => format!("matrix({},{},{},{},{},{})", fmt_num(*a), fmt_num(*b), fmt_num(*c), fmt_num(*d), fmt_num(*e), fmt_num(*f)),
            TransformOp::Translate { x, y: None } => format!("translate({})", fmt_num(*x)),
            TransformOp::Translate { x, y: Some(y) } => format!("translate({},{})", fmt_num(*x), fmt_num(*y)),
            TransformOp::Scale { x, y: None } => format!("scale({})", fmt_num(*x)),
            TransformOp::Scale { x, y: Some(y) } => format!("scale({},{})", fmt_num(*x), fmt_num(*y)),
            TransformOp::Rotate { angle, center: None } => format!("rotate({})", fmt_num(*angle)),
            TransformOp::Rotate { angle, center: Some((cx, cy)) } => format!("rotate({},{},{})", fmt_num(*angle), fmt_num(*cx), fmt_num(*cy)),
            TransformOp::SkewX { angle } => format!("skewX({})", fmt_num(*angle)),
            TransformOp::SkewY { angle } => format!("skewY({})", fmt_num(*angle)),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn path_data_to_string(cmds: &[PathCommand]) -> String {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn letter(base: char, relative: bool) -> char {
        if relative {
            base.to_ascii_lowercase()
        } else {
            base
        }
    }
    let mut parts = Vec::with_capacity(cmds.len());
    for cmd in cmds {
        parts.push(match cmd {
            PathCommand::MoveTo { x, y, relative } => format!("{} {} {}", letter('M', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::LineTo { x, y, relative } => format!("{} {} {}", letter('L', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::HorizontalLineTo { x, relative } => format!("{} {}", letter('H', *relative), fmt_num(*x)),
            PathCommand::VerticalLineTo { y, relative } => format!("{} {}", letter('V', *relative), fmt_num(*y)),
            PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative } => {
                format!("{} {} {} {} {} {} {}", letter('C', *relative), fmt_num(*x1), fmt_num(*y1), fmt_num(*x2), fmt_num(*y2), fmt_num(*x), fmt_num(*y))
            }
            PathCommand::SmoothCurveTo { x2, y2, x, y, relative } => format!("{} {} {} {} {}", letter('S', *relative), fmt_num(*x2), fmt_num(*y2), fmt_num(*x), fmt_num(*y)),
            PathCommand::QuadraticCurveTo { x1, y1, x, y, relative } => format!("{} {} {} {} {}", letter('Q', *relative), fmt_num(*x1), fmt_num(*y1), fmt_num(*x), fmt_num(*y)),
            PathCommand::SmoothQuadraticCurveTo { x, y, relative } => format!("{} {} {}", letter('T', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative } => {
                format!("{} {} {} {} {} {} {} {}", letter('A', *relative), fmt_num(*rx), fmt_num(*ry), fmt_num(*x_axis_rotation), *large_arc as u8, *sweep as u8, fmt_num(*x), fmt_num(*y))
            }
            PathCommand::ClosePath => "Z".to_string(),
        });
    }
    parts.join(" ")
}

pub(crate) fn apply_presentation_attr(p: &mut PresentationAttrs, name: &str, value: &str) -> bool {
    match name {
        "fill" => p.fill = Some(value.to_string()),
        "stroke" => p.stroke = Some(value.to_string()),
        "stroke-width" => p.stroke_width = Some(value.to_string()),
        "opacity" => p.opacity = Some(value.to_string()),
        "fill-opacity" => p.fill_opacity = Some(value.to_string()),
        "stroke-opacity" => p.stroke_opacity = Some(value.to_string()),
        "font-family" => p.font_family = Some(value.to_string()),
        "font-size" => p.font_size = Some(value.to_string()),
        _ => return false,
    }
    true
}

fn push_presentation_attrs(attrs: &mut Vec<XmlAttr>, p: &PresentationAttrs) {
    let mut push = |name: &str, value: &Option<String>| {
        if let Some(v) = value {
            attrs.push(XmlAttr { name: name.to_string(), value: v.clone() });
        }
    };
    push("fill", &p.fill);
    push("stroke", &p.stroke);
    push("stroke-width", &p.stroke_width);
    push("opacity", &p.opacity);
    push("fill-opacity", &p.fill_opacity);
    push("stroke-opacity", &p.stroke_opacity);
    push("font-family", &p.font_family);
    push("font-size", &p.font_size);
    if !p.extra_style.is_empty() {
        let decls: Vec<String> = p.extra_style.iter().map(|(k, v)| format!("{k}: {v}")).collect();
        attrs.push(XmlAttr { name: "style".to_string(), value: decls.join("; ") });
    }
}

fn push_common_attrs(attrs: &mut Vec<XmlAttr>, common: &CommonAttrs) {
    if let Some(id) = &common.id {
        attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
    }
    if let Some(class) = &common.class {
        attrs.push(XmlAttr { name: "class".into(), value: class.clone() });
    }
    if let Some(t) = &common.transform {
        attrs.push(XmlAttr { name: "transform".into(), value: transform_list_to_string(t) });
    }
    push_presentation_attrs(attrs, &common.presentation);
    attrs.extend(common.extra_attrs.iter().cloned());
}

fn attr_f64(attrs: &[XmlAttr], name: &str, default: f64) -> Result<f64, String> {
    match attr_val(attrs, name) {
        None => Ok(default),
        Some(v) => v.trim().parse::<f64>().map_err(|_| format!("attribute '{name}' is not a number: '{v}'")),
    }
}

fn attr_f64_opt(attrs: &[XmlAttr], name: &str) -> Result<Option<f64>, String> {
    match attr_val(attrs, name) {
        None => Ok(None),
        Some(v) => v.trim().parse::<f64>().map(Some).map_err(|_| format!("attribute '{name}' is not a number: '{v}'")),
    }
}

fn attr_string_opt(attrs: &[XmlAttr], name: &str) -> Option<String> {
    attr_val(attrs, name).map(|s| s.to_string())
}

fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

fn convert_children(children: &[XmlNode]) -> Result<Vec<SvgElement>, String> {
    children.iter().map(svg_element_from_xml_node).collect()
}

pub fn svg_element_from_xml_node(node: &XmlNode) -> Result<SvgElement, String> {
    match node {
        XmlNode::Text { text } => Ok(SvgElement::TextNode(text.clone())),
        XmlNode::CData { text } => Ok(SvgElement::CData(text.clone())),
        XmlNode::Comment { text } => Ok(SvgElement::Comment(text.clone())),
        XmlNode::ProcessingInstruction { target, data } => Ok(SvgElement::ProcessingInstruction { target: target.clone(), data: data.clone() }),
        XmlNode::Element { name, attrs, children } => match local_name(name) {
            "svg" => {
                let common = parse_common_attrs(attrs, &["viewBox", "width", "height", "xmlns"]);
                let view_box = match attr_val(attrs, "viewBox") {
                    Some(v) => Some(parse_view_box(v)?),
                    None => None,
                };
                Ok(SvgElement::Svg { common, view_box, width: attr_string_opt(attrs, "width"), height: attr_string_opt(attrs, "height"), xmlns: attr_string_opt(attrs, "xmlns"), children: convert_children(children)? })
            }
            "rect" => {
                let common = parse_common_attrs(attrs, &["x", "y", "width", "height", "rx", "ry"]);
                Ok(SvgElement::Rect {
                    common,
                    x: attr_f64(attrs, "x", 0.0)?,
                    y: attr_f64(attrs, "y", 0.0)?,
                    width: attr_f64(attrs, "width", 0.0)?,
                    height: attr_f64(attrs, "height", 0.0)?,
                    rx: attr_f64_opt(attrs, "rx")?,
                    ry: attr_f64_opt(attrs, "ry")?,
                })
            }
            "circle" => {
                let common = parse_common_attrs(attrs, &["cx", "cy", "r"]);
                Ok(SvgElement::Circle { common, cx: attr_f64(attrs, "cx", 0.0)?, cy: attr_f64(attrs, "cy", 0.0)?, r: attr_f64(attrs, "r", 0.0)? })
            }
            "ellipse" => {
                let common = parse_common_attrs(attrs, &["cx", "cy", "rx", "ry"]);
                Ok(SvgElement::Ellipse { common, cx: attr_f64(attrs, "cx", 0.0)?, cy: attr_f64(attrs, "cy", 0.0)?, rx: attr_f64(attrs, "rx", 0.0)?, ry: attr_f64(attrs, "ry", 0.0)? })
            }
            "line" => {
                let common = parse_common_attrs(attrs, &["x1", "y1", "x2", "y2"]);
                Ok(SvgElement::Line { common, x1: attr_f64(attrs, "x1", 0.0)?, y1: attr_f64(attrs, "y1", 0.0)?, x2: attr_f64(attrs, "x2", 0.0)?, y2: attr_f64(attrs, "y2", 0.0)? })
            }
            "polyline" => {
                let common = parse_common_attrs(attrs, &["points"]);
                let points = match attr_val(attrs, "points") {
                    Some(v) => parse_points(v)?,
                    None => Vec::new(),
                };
                Ok(SvgElement::Polyline { common, points })
            }
            "polygon" => {
                let common = parse_common_attrs(attrs, &["points"]);
                let points = match attr_val(attrs, "points") {
                    Some(v) => parse_points(v)?,
                    None => Vec::new(),
                };
                Ok(SvgElement::Polygon { common, points })
            }
            "path" => {
                let common = parse_common_attrs(attrs, &["d"]);
                let d = match attr_val(attrs, "d") {
                    Some(v) => parse_path_data(v)?,
                    None => Vec::new(),
                };
                Ok(SvgElement::Path { common, d })
            }
            "g" => Ok(SvgElement::Group { common: parse_common_attrs(attrs, &[]), children: convert_children(children)? }),
            "text" => {
                let common = parse_common_attrs(attrs, &["x", "y"]);
                Ok(SvgElement::Text { common, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, children: convert_children(children)? })
            }
            "tspan" => {
                let common = parse_common_attrs(attrs, &["x", "y"]);
                Ok(SvgElement::Tspan { common, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, children: convert_children(children)? })
            }
            "defs" => Ok(SvgElement::Defs { common: parse_common_attrs(attrs, &[]), children: convert_children(children)? }),
            "linearGradient" => {
                let common = parse_common_attrs(attrs, &["id", "x1", "y1", "x2", "y2"]);
                Ok(SvgElement::LinearGradient {
                    common,
                    id: attr_string_opt(attrs, "id"),
                    x1: attr_string_opt(attrs, "x1"),
                    y1: attr_string_opt(attrs, "y1"),
                    x2: attr_string_opt(attrs, "x2"),
                    y2: attr_string_opt(attrs, "y2"),
                    children: convert_children(children)?,
                })
            }
            "radialGradient" => {
                let common = parse_common_attrs(attrs, &["id", "cx", "cy", "r", "fx", "fy"]);
                Ok(SvgElement::RadialGradient {
                    common,
                    id: attr_string_opt(attrs, "id"),
                    cx: attr_string_opt(attrs, "cx"),
                    cy: attr_string_opt(attrs, "cy"),
                    r: attr_string_opt(attrs, "r"),
                    fx: attr_string_opt(attrs, "fx"),
                    fy: attr_string_opt(attrs, "fy"),
                    children: convert_children(children)?,
                })
            }
            "stop" => {
                let common = parse_common_attrs(attrs, &["offset", "stop-color", "stop-opacity"]);
                Ok(SvgElement::Stop { common, offset: attr_string_opt(attrs, "offset").unwrap_or_default(), stop_color: attr_string_opt(attrs, "stop-color"), stop_opacity: attr_string_opt(attrs, "stop-opacity") })
            }
            "use" => {
                let common = parse_common_attrs(attrs, &["href", "xlink:href", "x", "y", "width", "height"]);
                let href = attr_string_opt(attrs, "href").or_else(|| attr_string_opt(attrs, "xlink:href")).unwrap_or_default();
                Ok(SvgElement::Use { common, href, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, width: attr_f64_opt(attrs, "width")?, height: attr_f64_opt(attrs, "height")? })
            }
            _ => Ok(SvgElement::Unknown { name: name.clone(), attrs: attrs.clone(), children: convert_children(children)? }),
        },
    }
}

pub fn svg_element_to_xml_node(el: &SvgElement) -> XmlNode {
    match el {
        SvgElement::TextNode(t) => XmlNode::Text { text: t.clone() },
        SvgElement::CData(t) => XmlNode::CData { text: t.clone() },
        SvgElement::Comment(t) => XmlNode::Comment { text: t.clone() },
        SvgElement::ProcessingInstruction { target, data } => XmlNode::ProcessingInstruction { target: target.clone(), data: data.clone() },
        SvgElement::Svg { common, view_box, width, height, xmlns, children } => {
            let mut attrs = Vec::new();
            if let Some(vb) = view_box {
                attrs.push(XmlAttr { name: "viewBox".into(), value: view_box_to_string(vb) });
            }
            if let Some(w) = width {
                attrs.push(XmlAttr { name: "width".into(), value: w.clone() });
            }
            if let Some(h) = height {
                attrs.push(XmlAttr { name: "height".into(), value: h.clone() });
            }
            if let Some(x) = xmlns {
                attrs.push(XmlAttr { name: "xmlns".into(), value: x.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "svg".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Rect { common, x, y, width, height, rx, ry } => {
            let mut attrs =
                vec![XmlAttr { name: "x".into(), value: fmt_num(*x) }, XmlAttr { name: "y".into(), value: fmt_num(*y) }, XmlAttr { name: "width".into(), value: fmt_num(*width) }, XmlAttr { name: "height".into(), value: fmt_num(*height) }];
            if let Some(rx) = rx {
                attrs.push(XmlAttr { name: "rx".into(), value: fmt_num(*rx) });
            }
            if let Some(ry) = ry {
                attrs.push(XmlAttr { name: "ry".into(), value: fmt_num(*ry) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "rect".into(), attrs, children: vec![] }
        }
        SvgElement::Circle { common, cx, cy, r } => {
            let mut attrs = vec![XmlAttr { name: "cx".into(), value: fmt_num(*cx) }, XmlAttr { name: "cy".into(), value: fmt_num(*cy) }, XmlAttr { name: "r".into(), value: fmt_num(*r) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "circle".into(), attrs, children: vec![] }
        }
        SvgElement::Ellipse { common, cx, cy, rx, ry } => {
            let mut attrs = vec![XmlAttr { name: "cx".into(), value: fmt_num(*cx) }, XmlAttr { name: "cy".into(), value: fmt_num(*cy) }, XmlAttr { name: "rx".into(), value: fmt_num(*rx) }, XmlAttr { name: "ry".into(), value: fmt_num(*ry) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "ellipse".into(), attrs, children: vec![] }
        }
        SvgElement::Line { common, x1, y1, x2, y2 } => {
            let mut attrs = vec![XmlAttr { name: "x1".into(), value: fmt_num(*x1) }, XmlAttr { name: "y1".into(), value: fmt_num(*y1) }, XmlAttr { name: "x2".into(), value: fmt_num(*x2) }, XmlAttr { name: "y2".into(), value: fmt_num(*y2) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "line".into(), attrs, children: vec![] }
        }
        SvgElement::Polyline { common, points } => {
            let mut attrs = vec![XmlAttr { name: "points".into(), value: points_to_string(points) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "polyline".into(), attrs, children: vec![] }
        }
        SvgElement::Polygon { common, points } => {
            let mut attrs = vec![XmlAttr { name: "points".into(), value: points_to_string(points) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "polygon".into(), attrs, children: vec![] }
        }
        SvgElement::Path { common, d } => {
            let mut attrs = vec![XmlAttr { name: "d".into(), value: path_data_to_string(d) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "path".into(), attrs, children: vec![] }
        }
        SvgElement::Group { common, children } => {
            let mut attrs = Vec::new();
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "g".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Text { common, x, y, children } => {
            let mut attrs = Vec::new();
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "text".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Tspan { common, x, y, children } => {
            let mut attrs = Vec::new();
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "tspan".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Defs { common, children } => {
            let mut attrs = Vec::new();
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "defs".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::LinearGradient { common, id, x1, y1, x2, y2, children } => {
            let mut attrs = Vec::new();
            if let Some(id) = id {
                attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
            }
            if let Some(v) = x1 {
                attrs.push(XmlAttr { name: "x1".into(), value: v.clone() });
            }
            if let Some(v) = y1 {
                attrs.push(XmlAttr { name: "y1".into(), value: v.clone() });
            }
            if let Some(v) = x2 {
                attrs.push(XmlAttr { name: "x2".into(), value: v.clone() });
            }
            if let Some(v) = y2 {
                attrs.push(XmlAttr { name: "y2".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "linearGradient".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::RadialGradient { common, id, cx, cy, r, fx, fy, children } => {
            let mut attrs = Vec::new();
            if let Some(id) = id {
                attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
            }
            if let Some(v) = cx {
                attrs.push(XmlAttr { name: "cx".into(), value: v.clone() });
            }
            if let Some(v) = cy {
                attrs.push(XmlAttr { name: "cy".into(), value: v.clone() });
            }
            if let Some(v) = r {
                attrs.push(XmlAttr { name: "r".into(), value: v.clone() });
            }
            if let Some(v) = fx {
                attrs.push(XmlAttr { name: "fx".into(), value: v.clone() });
            }
            if let Some(v) = fy {
                attrs.push(XmlAttr { name: "fy".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "radialGradient".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Stop { common, offset, stop_color, stop_opacity } => {
            let mut attrs = vec![XmlAttr { name: "offset".into(), value: offset.clone() }];
            if let Some(v) = stop_color {
                attrs.push(XmlAttr { name: "stop-color".into(), value: v.clone() });
            }
            if let Some(v) = stop_opacity {
                attrs.push(XmlAttr { name: "stop-opacity".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "stop".into(), attrs, children: vec![] }
        }
        SvgElement::Use { common, href, x, y, width, height } => {
            let mut attrs = vec![XmlAttr { name: "href".into(), value: href.clone() }];
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            if let Some(w) = width {
                attrs.push(XmlAttr { name: "width".into(), value: fmt_num(*w) });
            }
            if let Some(h) = height {
                attrs.push(XmlAttr { name: "height".into(), value: fmt_num(*h) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "use".into(), attrs, children: vec![] }
        }
        SvgElement::Unknown { name, attrs, children } => XmlNode::Element { name: name.clone(), attrs: attrs.clone(), children: children.iter().map(svg_element_to_xml_node).collect() },
    }
}

pub fn svg_document_to_typed(doc: &XmlDocument) -> Result<SvgElement, String> {
    match &doc.root {
        Some(node) => svg_element_from_xml_node(node),
        None => Err("svg document has no root element".into()),
    }
}

pub fn typed_to_svg_document(root: &SvgElement, doctype: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype>) -> XmlDocument {
    XmlDocument { root: Some(svg_element_to_xml_node(root)), doctype, declaration: None, prolog: Vec::new(), epilog: Vec::new() }
}

struct NumCursor<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> NumCursor<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn new(s: &'a str) -> Self {
        Self { s: s.as_bytes(), pos: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn is_eof(&self) -> bool {
        self.pos >= self.s.len()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_wsp_comma(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') | Some(b',')) {
            self.pos += 1;
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_wsp(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')) {
            self.pos += 1;
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_number(&mut self) -> Result<f64, String> {
        self.skip_wsp_comma();
        let start = self.pos;
        if matches!(self.peek(), Some(b'+') | Some(b'-')) {
            self.pos += 1;
        }
        let mut has_digits = false;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.pos += 1;
            has_digits = true;
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
                has_digits = true;
            }
        }
        if !has_digits {
            self.pos = start;
            return Err(format!("expected number at byte {start}"));
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            let save = self.pos;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            if matches!(self.peek(), Some(b'0'..=b'9')) {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            } else {
                self.pos = save;
            }
        }
        let raw = std::str::from_utf8(&self.s[start..self.pos]).unwrap();
        raw.parse::<f64>().map_err(|_| format!("invalid number '{raw}'"))
    }
    /// 🚩 A path arc-flag is EXACTLY one `0`/`1` byte with no separator required before the next
    /// token -- `A5 5 0 108 8` must decompose the run `108` into flags `1`,`0` then the number `8`
    /// (large-arc=1, sweep=0, x=8), never a naive 2-digit/3-digit number grab. Classic bug source.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_flag(&mut self) -> Result<bool, String> {
        self.skip_wsp_comma();
        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
                Ok(false)
            }
            Some(b'1') => {
                self.pos += 1;
                Ok(true)
            }
            other => Err(format!("expected arc flag (0/1), got {other:?} at byte {}", self.pos)),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_number_list(s: &str) -> Result<Vec<f64>, String> {
    let mut c = NumCursor::new(s);
    let mut out = Vec::new();
    loop {
        c.skip_wsp_comma();
        if c.is_eof() {
            break;
        }
        out.push(c.parse_number()?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_view_box(s: &str) -> Result<ViewBox, String> {
    let nums = parse_number_list(s)?;
    if nums.len() != 4 {
        return Err(format!("viewBox requires exactly 4 numbers, got {}", nums.len()));
    }
    Ok(ViewBox { min_x: nums[0], min_y: nums[1], width: nums[2], height: nums[3] })
}

/// 🔗️ `points="x1,y1 x2,y2 ..."` (polyline/polygon).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_points(s: &str) -> Result<Vec<(f64, f64)>, String> {
    let nums = parse_number_list(s)?;
    if nums.len() % 2 != 0 {
        return Err("points list must have an even number of coordinates".into());
    }
    Ok(nums.chunks(2).map(|c| (c[0], c[1])).collect())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_transform_list(s:&str)->Result<Vec<TransformOp>,String>{let mut values=Vec::new();read_transform_list(s,|value|{values.push(value);Ok(())})?;Ok(values)}
fn read_transform_list(s:&str,mut emit:impl FnMut(TransformOp)->Result<(),String>)->Result<(),String> {
    let mut c = NumCursor::new(s);
    loop {
        c.skip_wsp_comma();
        if c.is_eof() {
            break;
        }
        let start = c.pos;
        while matches!(c.peek(), Some(b'a'..=b'z') | Some(b'A'..=b'Z')) {
            c.pos += 1;
        }
        let name = std::str::from_utf8(&c.s[start..c.pos]).unwrap();
        if name.is_empty() {
            return Err(format!("expected transform function name at byte {}", c.pos));
        }
        c.skip_wsp();
        if c.peek() != Some(b'(') {
            return Err(format!("expected '(' after transform function '{name}'"));
        }
        c.pos += 1;
        let mut storage=[0.0;6];let mut count=0;
        loop {
            c.skip_wsp_comma();
            if c.peek() == Some(b')') {
                break;
            }
            if count==storage.len(){return Err("SVG transform exceeds six arguments".into())}storage[count]=c.parse_number()?;count+=1;
        }
        if c.peek() != Some(b')') {
            return Err(format!("unclosed '(' for transform function '{name}'"));
        }
        c.pos += 1;
        let nums=&storage[..count];
        let op = match name {
            "matrix" if nums.len() == 6 => TransformOp::Matrix { a: nums[0], b: nums[1], c: nums[2], d: nums[3], e: nums[4], f: nums[5] },
            "translate" if nums.len() == 1 => TransformOp::Translate { x: nums[0], y: None },
            "translate" if nums.len() == 2 => TransformOp::Translate { x: nums[0], y: Some(nums[1]) },
            "scale" if nums.len() == 1 => TransformOp::Scale { x: nums[0], y: None },
            "scale" if nums.len() == 2 => TransformOp::Scale { x: nums[0], y: Some(nums[1]) },
            "rotate" if nums.len() == 1 => TransformOp::Rotate { angle: nums[0], center: None },
            "rotate" if nums.len() == 3 => TransformOp::Rotate { angle: nums[0], center: Some((nums[1], nums[2])) },
            "skewX" if nums.len() == 1 => TransformOp::SkewX { angle: nums[0] },
            "skewY" if nums.len() == 1 => TransformOp::SkewY { angle: nums[0] },
            other => return Err(format!("unknown/malformed transform function '{other}' with {} args", nums.len())),
        };
        emit(op)?;
    }
    Ok(())
}

/// 🖊️ Parses a `d` attribute per the SVG path mini-language grammar. Verified against 18 checks
/// (incl. the arc-flag squeeze edge case) in a standalone scratch crate before porting here, per
/// the technique in the ticket's STATUS.md.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_path_data(d:&str)->Result<Vec<PathCommand>,String>{let mut values=Vec::new();read_path_data(d,|value|{values.push(value);Ok(())})?;Ok(values)}
fn read_path_data(d:&str,mut emit:impl FnMut(PathCommand)->Result<(),String>)->Result<(),String> {
    let mut c = NumCursor::new(d);
    // 🔁 `last_letter`/`last_relative` drive implicit command repetition: a number run with no
    // leading letter reuses the previous command -- except a bare `M`/`m` run whose FIRST pair is
    // the moveto and whose SUBSEQUENT pairs become implicit `L`/`l` (SVG spec 8.3.2).
    let mut last_letter: Option<u8> = None;
    let mut last_relative = false;
    loop {
        c.skip_wsp_comma();
        if c.is_eof() {
            break;
        }
        let peeked = c.peek().unwrap();
        let (letter, relative, explicit) = if peeked.is_ascii_alphabetic() {
            c.pos += 1;
            (peeked.to_ascii_uppercase(), peeked.is_ascii_lowercase(), true)
        } else {
            let letter = last_letter.ok_or_else(|| "path data must start with a moveto command".to_string())?;
            (letter, last_relative, false)
        };
        if explicit && letter == b'Z' {
            emit(PathCommand::ClosePath)?;
            last_letter = None;
            continue;
        }
        match letter {
            b'M' => {
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::MoveTo { x, y, relative })?;
                last_letter = Some(b'L');
                last_relative = relative;
            }
            b'L' => {
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::LineTo { x, y, relative })?;
                last_letter = Some(b'L');
                last_relative = relative;
            }
            b'H' => {
                let x = c.parse_number()?;
                emit(PathCommand::HorizontalLineTo { x, relative })?;
                last_letter = Some(b'H');
                last_relative = relative;
            }
            b'V' => {
                let y = c.parse_number()?;
                emit(PathCommand::VerticalLineTo { y, relative })?;
                last_letter = Some(b'V');
                last_relative = relative;
            }
            b'C' => {
                let x1 = c.parse_number()?;
                let y1 = c.parse_number()?;
                let x2 = c.parse_number()?;
                let y2 = c.parse_number()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative })?;
                last_letter = Some(b'C');
                last_relative = relative;
            }
            b'S' => {
                let x2 = c.parse_number()?;
                let y2 = c.parse_number()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::SmoothCurveTo { x2, y2, x, y, relative })?;
                last_letter = Some(b'S');
                last_relative = relative;
            }
            b'Q' => {
                let x1 = c.parse_number()?;
                let y1 = c.parse_number()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::QuadraticCurveTo { x1, y1, x, y, relative })?;
                last_letter = Some(b'Q');
                last_relative = relative;
            }
            b'T' => {
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::SmoothQuadraticCurveTo { x, y, relative })?;
                last_letter = Some(b'T');
                last_relative = relative;
            }
            b'A' => {
                let rx = c.parse_number()?;
                let ry = c.parse_number()?;
                let x_axis_rotation = c.parse_number()?;
                let large_arc = c.parse_flag()?;
                let sweep = c.parse_flag()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                emit(PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative })?;
                last_letter = Some(b'A');
                last_relative = relative;
            }
            other => return Err(format!("unknown path command '{}'", other as char)),
        }
    }
    Ok(())
}

/// 🧩 Real `key: value; key2: value2` parsing (declaration-list split on `;`, each split on the
/// FIRST `:`) -- not a substring/`contains()` hack.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_style_decls(s: &str) -> Vec<(String, String)> {
    s.split(';')
        .filter_map(|decl| {
            let decl = decl.trim();
            if decl.is_empty() {
                return None;
            }
            let mut parts = decl.splitn(2, ':');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key.is_empty() || value.is_empty() {
                return None;
            }
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}





pub(crate) fn parse_common_attrs(attrs: &[XmlAttr], element_specific: &[&str]) -> CommonAttrs {
    let mut common = CommonAttrs::default();
    let style_decls = attr_val(attrs, "style").map(parse_style_decls).unwrap_or_default();
    for a in attrs {
        match a.name.as_str() {
            "id" => common.id = Some(a.value.clone()),
            "class" => common.class = Some(a.value.clone()),
            "style" => {}
            "transform" => match parse_transform_list(&a.value) {
                Ok(ops) => common.transform = Some(ops),
                // 🚧️ malformed transform: kept verbatim rather than fabricated as an empty list.
                Err(_) => common.extra_attrs.push(a.clone()),
            },
            "fill" | "stroke" | "stroke-width" | "opacity" | "fill-opacity" | "stroke-opacity" | "font-family" | "font-size" => {}
            other if element_specific.contains(&other) => {}
            _ => common.extra_attrs.push(a.clone()),
        }
    }
    for a in attrs {
        apply_presentation_attr(&mut common.presentation, &a.name, &a.value);
    }
    for (k, v) in &style_decls {
        if !apply_presentation_attr(&mut common.presentation, k, v) {
            common.presentation.extra_style.push((k.clone(), v.clone()));
        }
    }
    common
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

/// 📝️ Binds a native attribute into its semantic value owner.
pub fn bind_svg_attribute(name:&str,text:&str)->Result<crate::schema::snapshot::SvgAttributeValue,String> {
    use crate::schema::snapshot::{SvgAttributeValue as V,SvgLength,SvgPoint};
    Ok(match name {
        "clip-path" if text.trim().starts_with("url(")=>{let inner=text.trim().strip_prefix("url(").and_then(|v|v.strip_suffix(')')).ok_or("invalid SVG clip-path reference")?.trim().trim_matches(|c|c=='\''||c=='"');if let Some(id)=inner.strip_prefix('#'){V::LocalReference(id.to_owned())}else{V::Text(text.to_owned())}},
        "opacity"|"fill-opacity"|"stroke-opacity"=>V::Number(text.trim().parse::<f64>().map_err(|_|"SVG opacity requires number")?),
        "viewBox"=>V::ViewBox(parse_view_box(text)?),
        "transform"=>V::Transform(parse_transform_list(text)?),
        "points"=>V::Points(parse_points(text)?.into_iter().map(|(x,y)|SvgPoint{x,y}).collect()),
        "d"=>V::PathData(parse_path_data(text)?),
        "width"|"height"|"x"|"y"|"x1"|"y1"|"x2"|"y2"|"cx"|"cy"|"r"|"rx"|"ry"|"fx"|"fy"=>{
            let text=text.trim();let mut cursor=NumCursor::new(text);let magnitude=cursor.parse_number()?;let unit=text[cursor.pos..].trim().to_string();
            V::Length(SvgLength{magnitude,unit})
        },
        _=>V::Text(text.to_string()),
    })
}
/// 📝️ Formats a typed SVG attribute only at emission.
pub fn print_svg_attribute(value:&crate::schema::snapshot::SvgAttributeValue)->String {
    use crate::schema::snapshot::SvgAttributeValue as V;
    match value { V::Text(text)=>text.clone(),V::LocalReference(id)=>format!("url(#{id})"),V::Number(number)=>fmt_num(*number),V::Length(length)=>format!("{}{}",fmt_num(length.magnitude),length.unit),V::ViewBox(value)=>view_box_to_string(value),V::Transform(ops)=>transform_list_to_string(ops),V::Points(points)=>points_to_string(&points.iter().map(|point|(point.x,point.y)).collect::<Vec<_>>()),V::PathData(commands)=>path_data_to_string(commands) }
}
/// 🌳️ Projects native XML into SVG node owners.
pub fn bind_svg_node(node:XmlNode)->Result<crate::schema::snapshot::SvgNode,String> {
    use crate::schema::snapshot::{SvgNode as N,SvgAttr};
    enum Frame{Visit(XmlNode),Finish(String,Vec<SvgAttr>,usize)}
    let mut frames=vec![Frame::Visit(node)];let mut nodes=Vec::new();
    while let Some(frame)=frames.pop(){match frame{
        Frame::Finish(name,attrs,count)=>{let children=nodes.split_off(nodes.len()-count);nodes.push(N::Element{name,attrs,children});},
        Frame::Visit(node)=>match node{
            XmlNode::Element{name,attrs,children}=>{
                let mut owned=Vec::with_capacity(attrs.len());
                for attr in attrs{match bind_svg_attribute(&attr.name,&attr.value){Ok(value)=>owned.push(SvgAttr{name:attr.name,value}),Err(error)=>{
                    let mut native=children;for frame in frames{if let Frame::Visit(node)=frame{native.push(node);}}
                    semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document(XmlDocument{prolog:native,..Default::default()});
                    crate::schema::snapshot::retire_svg_document(crate::schema::snapshot::SvgDocument{prolog:nodes,..Default::default()});return Err(error);
                }}}
                frames.push(Frame::Finish(name,owned,children.len()));frames.extend(children.into_iter().rev().map(Frame::Visit));
            },
            XmlNode::Text{text}=>nodes.push(N::Text{text}),XmlNode::CData{text}=>nodes.push(N::CData{text}),XmlNode::Comment{text}=>nodes.push(N::Comment{text}),XmlNode::ProcessingInstruction{target,data}=>nodes.push(N::ProcessingInstruction{target,data}),
        }
    }}Ok(nodes.pop().expect("one SVG root was bound"))
}
/// 🌳️ Iteratively projects SVG owners into native XML for physical emission.
pub fn native_svg_node(node:&crate::schema::snapshot::SvgNode)->XmlNode {
    use crate::schema::snapshot::SvgNode as N;
    enum Frame<'a>{Visit(&'a N),Finish(String,Vec<XmlAttr>,usize)}
    let mut frames=vec![Frame::Visit(node)];let mut nodes=Vec::new();
    while let Some(frame)=frames.pop(){match frame{
        Frame::Finish(name,attrs,count)=>{let children=nodes.split_off(nodes.len()-count);nodes.push(XmlNode::Element{name,attrs,children});},
        Frame::Visit(node)=>match node{
            N::Element{name,attrs,children}=>{frames.push(Frame::Finish(name.clone(),attrs.iter().map(|attr|XmlAttr{name:attr.name.clone(),value:print_svg_attribute(&attr.value)}).collect(),children.len()));frames.extend(children.iter().rev().map(Frame::Visit));},
            N::Text{text}=>nodes.push(XmlNode::Text{text:text.clone()}),N::CData{text}=>nodes.push(XmlNode::CData{text:text.clone()}),N::Comment{text}=>nodes.push(XmlNode::Comment{text:text.clone()}),N::ProcessingInstruction{target,data}=>nodes.push(XmlNode::ProcessingInstruction{target:target.clone(),data:data.clone()}),
        }
    }}nodes.pop().expect("one native SVG root was projected")
}
/// 🌳️ Admits each native document lane into its owned SVG value.
pub fn bind_svg_document(doc:XmlDocument)->Result<crate::schema::snapshot::SvgDocument,String> {
    use semio_framework_dsl_record::__rt::DecodedFieldOwner;
    let mut native=DecodedFieldOwner::new(doc,semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);
    let mut owned=DecodedFieldOwner::new(crate::schema::snapshot::SvgDocument::default(),crate::schema::snapshot::retire_svg_document);
    owned.as_mut().declaration=native.as_mut().declaration.take();owned.as_mut().doctype=native.as_mut().doctype.take();
    while let Some(node)=native.as_mut().prolog.pop(){owned.as_mut().prolog.push(bind_svg_node(node)?);}owned.as_mut().prolog.reverse();
    if let Some(node)=native.as_mut().root.take(){owned.as_mut().root=Some(bind_svg_node(node)?);}
    while let Some(node)=native.as_mut().epilog.pop(){owned.as_mut().epilog.push(bind_svg_node(node)?);}owned.as_mut().epilog.reverse();
    Ok(owned.take())
}
/// 🌳️ Lowers each SVG document lane for physical XML emission.
pub fn native_svg_document(doc:&crate::schema::snapshot::SvgDocument)->XmlDocument {
    XmlDocument{declaration:doc.declaration.clone(),doctype:doc.doctype.clone(),prolog:doc.prolog.iter().map(native_svg_node).collect(),root:doc.root.as_ref().map(native_svg_node),epilog:doc.epilog.iter().map(native_svg_node).collect()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🚦️controlled/🦀️.rs"]
mod controlled;
pub use controlled::*;
