//! 🚪️ drawing -> pdf — foreign `Serializer<DrawingSnapshot>` (design.md §3). REAL: paints the
//! document's flattened scene (`flatten_drawing_document_to_scene_nodes`, the same projection the
//! canvas draws) as one PDF 1.4 page of vector content — path/shape/boolean/trace outlines with solid
//! fills, axial/radial shading fills, strokes (width, cap, join, dash), layer opacity and blend mode
//! through isolated transparency forms and ExtGStates, text through Helvetica, and image layers as RGB XObjects with an
//! alpha SMask. The page is the artboard; drawing space (y down) is mapped onto PDF space (y up) by
//! one base CTM, so every node keeps its own matrix verbatim.
//!
//! 🧾️ `IoFidelity::Lossy`: text is limited to Helvetica/WinAnsi (no font embedding), gradient-stop
//! alpha is not represented. Scene nodes carry composed group and text-origin transforms, plus isolated group ancestry.
//!
//! 📖️ Why draw writes its own bytes: `s.stdio.pdf`'s snapshots (1.4 and 1.7) are text-only page
//! models — `encode_pdf` regenerates a content stream FROM `PageDoc.text` and has no path-painting
//! operator emission — so routing through them would lose every outline. The COS syntax needed here
//! (object table, classic `xref`, one page tree) is small enough to state locally and exactly.

use crate::schema::{flatten_drawing_document_to_scene_nodes, resolve_drawing_artboard, DrawingSceneGroup, DrawingSceneNode};
use crate::{DrawingSnapshot, FillStyle, GradientStop, PathSegment, StrokeStyle};
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

/// 📐️ Page size when the document has neither an artboard nor any bounded layer.
const DEFAULT_PAGE: (f64, f64) = (800.0, 600.0);
/// 🧵️ Arc flattening: one cubic per quarter turn keeps the radial error below 0.03 % of the radius.
const ARC_CUBICS_PER_QUARTER: f64 = 1.0;

pub struct DrawingIntoPdf;

impl Serializer<DrawingSnapshot> for DrawingIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let bytes = drawing_document_to_pdf(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DrawingIntoPdf: {message}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}

//#region 🔖️Painter
/// 📄️ The whole document as one PDF 1.4 page (the artboard) of vector content — see the module doc.
pub fn drawing_document_to_pdf(doc: &DrawingSnapshot) -> Result<Vec<u8>, String> {
    let (width, height) = resolve_drawing_artboard(doc).map_or(DEFAULT_PAGE, |artboard| (artboard.width, artboard.height));
    drawing_scene_to_pdf(doc,&flatten_drawing_document_to_scene_nodes(doc),width,height)
}

fn drawing_scene_to_pdf(doc:&DrawingSnapshot,nodes:&[DrawingSceneNode],width:f64,height:f64)->Result<Vec<u8>,String> {
    if !(width.is_finite() && height.is_finite() && width>0.0 && height>0.0) {
        return Err("the drawing artboard has no positive finite size".into());
    }
    let mut writer=PdfWriter::default();
    let mut content=PdfContent::default();
    let mut stack:Vec<(&DrawingSceneGroup,PdfContent)>=Vec::new();
    let mut opened=BTreeSet::new();
    let _=writeln!(content.body,"q 1 0 0 -1 0 {} cm",num(height));
    for node in nodes {
        let common=stack.iter().zip(&node.groups).take_while(|((group,_),next)|*group==*next).count();
        while stack.len()>common {close_group(&mut writer,&mut content,&mut stack,width,height)?;}
        for group in &node.groups[common..] {
            if group.id.is_empty() || !opened.insert(group.id.as_str()) || !valid_composite(group.opacity,&group.blend_mode) {
                return Err("Invalid scene compositing hierarchy".into());
            }
            stack.push((group,PdfContent::default()));
        }
        if !valid_composite(node.opacity,&node.blend_mode) {return Err("Invalid layer compositing properties".into());}
        if !node.visible || node.opacity==0.0 {continue;}
        let target=stack.last_mut().map_or(&mut content,|(_,content)|content);
        if node.opacity!=1.0 || node.blend_mode!="normal" {
            let mut leaf=PdfContent::default();
            paint_node(&mut writer,&mut leaf,doc,node)?;
            let form=writer.transparency_form(width,height,leaf)?;
            writer.paint_form(target,&form,node.opacity,&node.blend_mode);
        } else {paint_node(&mut writer,target,doc,node)?;}
    }
    while !stack.is_empty() {close_group(&mut writer,&mut content,&mut stack,width,height)?;}
    content.body.push_str("Q\n");
    writer.finish(width,height,content)
}

fn valid_composite(opacity:f64,blend:&str)->bool {
    opacity.is_finite() && (0.0..=1.0).contains(&opacity) && matches!(blend,"normal"|"multiply"|"screen"|"overlay"|"darken"|"lighten"|"colorDodge"|"colorBurn"|"hardLight"|"softLight"|"difference"|"exclusion"|"hue"|"saturation"|"color"|"luminosity")
}

fn close_group(writer:&mut PdfWriter,root:&mut PdfContent,stack:&mut Vec<(&DrawingSceneGroup,PdfContent)>,width:f64,height:f64)->Result<(),String> {
    let (group,content)=stack.pop().expect("an open compositing group");
    let form=writer.transparency_form(width,height,content)?;
    let parent=stack.last_mut().map_or(root,|(_,content)|content);
    writer.paint_form(parent,&form,group.opacity,&group.blend_mode);
    Ok(())
}

fn paint_node(writer: &mut PdfWriter, content: &mut PdfContent, doc: &DrawingSnapshot, node: &DrawingSceneNode) -> Result<(), String> {
    let resources=&mut content.resources;
    let content=&mut content.body;
    let fill_alpha = match &node.fill {
        Some(FillStyle::Solid { color }) => color[3],
        Some(_) | None => 1.0,
    };
    let stroke_alpha = node.stroke.as_ref().map_or(1.0, |stroke| stroke.color[3]);
    let has_fill = node.fill.as_ref().is_some_and(fill_paints);
    let has_stroke = node.stroke.as_ref().is_some_and(|stroke| stroke.width > 0.0 && stroke.color[3] > 0.0);
    let path = path_ops(&node.segments);
    let paints_geometry = !path.is_empty() && (has_fill || has_stroke);
    if !paints_geometry && node.text.is_none() && node.image.is_none() {
        return Ok(());
    }
    content.push_str("q\n");
    let [a, b, c, d, e, f] = node.transform;
    if node.transform != [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] {
        let _ = writeln!(content, "{} {} {} {} {} {} cm", num(a), num(b), num(c), num(d), num(e), num(f));
    }
    let state = writer.ext_g_state(fill_alpha.clamp(0.0, 1.0), stroke_alpha.clamp(0.0, 1.0), "normal");
    if let Some(state) = state {
        resources.insert(state.clone());
        let _ = writeln!(content, "/{state} gs");
    }
    if paints_geometry {
        let fill_rule_star = node.fill_rule.as_deref() == Some("evenodd");
        let paint = |fill: bool, stroke: bool| -> &'static str {
            match (fill, stroke, fill_rule_star) {
                (true, true, true) => "B*",
                (true, true, false) => "B",
                (true, false, true) => "f*",
                (true, false, false) => "f",
                (false, true, _) => "S",
                (false, false, _) => "n",
            }
        };
        if has_stroke {
            stroke_ops(content, node.stroke.as_ref().expect("stroke checked above"));
        }
        match node.fill.as_ref().filter(|_| has_fill) {
            Some(FillStyle::Solid { color }) => {
                let _ = writeln!(content, "{} {} {} rg", num(color[0]), num(color[1]), num(color[2]));
                content.push_str(&path);
                let _ = writeln!(content, "{}", paint(true, has_stroke));
            }
            Some(gradient) => {
                // 🌈️ Shading fills paint through the outline as a clip, then the outline strokes on top.
                let shading = writer.shading(gradient)?;
                resources.insert(shading.clone());
                content.push_str("q\n");
                content.push_str(&path);
                let _ = writeln!(content, "{}", if fill_rule_star { "W* n" } else { "W n" });
                let _ = writeln!(content, "/{shading} sh");
                content.push_str("Q\n");
                if has_stroke {
                    content.push_str(&path);
                    content.push_str("S\n");
                }
            }
            None => {
                content.push_str(&path);
                let _ = writeln!(content, "{}", paint(false, has_stroke));
            }
        }
    }
    if let Some(text) = &node.text {
        if !text.content.is_empty() && text.size > 0.0 && (has_fill || has_stroke) {
            let font = writer.helvetica();
            resources.insert(font.clone());
            if has_stroke { stroke_ops(content, node.stroke.as_ref().expect("stroke checked above")); }
            let gradient = node.fill.as_ref().filter(|fill| has_fill && !matches!(fill, FillStyle::Solid { .. }));
            let shading = gradient.map(|fill| writer.shading(fill)).transpose()?;
            if let Some(shading)=&shading {resources.insert(shading.clone());}
            let color = match &node.fill { Some(FillStyle::Solid { color }) => *color, _ => [0.0, 0.0, 0.0, 1.0] };
            for (index, line) in semio_framework_2d::text::drawing_text_lines(&text.content).enumerate() {
                if line.is_empty() { continue; }
                let y = num(text.size + index as f64 * text.size * semio_framework_2d::text::DRAWING_TEXT_LINE_HEIGHT);
                let value = pdf_string(line);
                if let Some(shading) = &shading {
                    let _ = writeln!(content, "q BT /{font} {} Tf 7 Tr 1 0 0 -1 0 {y} Tm ({value}) Tj ET /{shading} sh Q", num(text.size));
                }
                if shading.is_none() || has_stroke {
                    let mode = if has_fill && shading.is_none() { if has_stroke { 2 } else { 0 } } else { 1 };
                    let _ = writeln!(content, "BT /{font} {} Tf {} {} {} rg {mode} Tr 1 0 0 -1 0 {y} Tm ({value}) Tj ET", num(text.size), num(color[0]), num(color[1]), num(color[2]));
                }
            }
        }
    }
    if let Some(image) = &node.image {
        if image.width > 0.0 && image.height > 0.0 {
            if let Some(name) = writer.image(doc, &image.src)? {
                resources.insert(name.clone());
                // 🖼️ The unit square's top row is v = 1: under the flipped page a `-h` scale with a
                // `+h` offset keeps the image upright at the node origin.
                let _ = writeln!(content, "q {} 0 0 {} 0 {} cm /{name} Do Q", num(image.width), num(-image.height), num(image.height));
            }
        }
    }
    content.push_str("Q\n");
    Ok(())
}

fn fill_paints(fill: &FillStyle) -> bool {
    match fill {
        FillStyle::Solid { color } => color[3] > 0.0,
        FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => !stops.is_empty(),
    }
}

fn stroke_ops(content: &mut String, stroke: &StrokeStyle) {
    let cap = match stroke.cap {
        crate::StrokeCap::Butt => 0,
        crate::StrokeCap::Round => 1,
        crate::StrokeCap::Square => 2,
    };
    let join = match stroke.join {
        crate::StrokeJoin::Miter => 0,
        crate::StrokeJoin::Round => 1,
        crate::StrokeJoin::Bevel => 2,
    };
    let _ = writeln!(content, "{} {} {} RG {} w {cap} J {join} j", num(stroke.color[0]), num(stroke.color[1]), num(stroke.color[2]), num(stroke.width));
    match stroke.dash.as_deref() {
        Some(dash) if !dash.is_empty() && dash.iter().all(|value| value.is_finite() && *value >= 0.0) && dash.iter().any(|value| *value > 0.0) => {
            let pattern = dash.iter().map(|value| num(*value)).collect::<Vec<_>>().join(" ");
            let _ = writeln!(content, "[{pattern}] 0 d");
        }
        _ => content.push_str("[] 0 d\n"),
    }
}
//#endregion 🔖️Painter

//#region 🔖️Paths
/// 🛤️ Path construction operators for one segment list. Quads become cubics, SVG endpoint arcs become
/// cubic runs, and `Z` closes back to the subpath start (which also seeds the current point).
fn path_ops(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    let mut current = [0.0, 0.0];
    let mut start = [0.0, 0.0];
    let mut open = false;
    for segment in segments {
        match segment {
            PathSegment::Move { to } => {
                let _ = writeln!(out, "{} {} m", num(to[0]), num(to[1]));
                current = *to;
                start = *to;
                open = true;
            }
            PathSegment::Line { to } => {
                if !open {
                    let _ = writeln!(out, "{} {} m", num(current[0]), num(current[1]));
                    start = current;
                    open = true;
                }
                let _ = writeln!(out, "{} {} l", num(to[0]), num(to[1]));
                current = *to;
            }
            PathSegment::Quad { ctrl, to } => {
                if !open {
                    let _ = writeln!(out, "{} {} m", num(current[0]), num(current[1]));
                    start = current;
                    open = true;
                }
                let c1 = [current[0] + 2.0 / 3.0 * (ctrl[0] - current[0]), current[1] + 2.0 / 3.0 * (ctrl[1] - current[1])];
                let c2 = [to[0] + 2.0 / 3.0 * (ctrl[0] - to[0]), to[1] + 2.0 / 3.0 * (ctrl[1] - to[1])];
                let _ = writeln!(out, "{} {} {} {} {} {} c", num(c1[0]), num(c1[1]), num(c2[0]), num(c2[1]), num(to[0]), num(to[1]));
                current = *to;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                if !open {
                    let _ = writeln!(out, "{} {} m", num(current[0]), num(current[1]));
                    start = current;
                    open = true;
                }
                let _ = writeln!(out, "{} {} {} {} {} {} c", num(ctrl1[0]), num(ctrl1[1]), num(ctrl2[0]), num(ctrl2[1]), num(to[0]), num(to[1]));
                current = *to;
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                if !open {
                    let _ = writeln!(out, "{} {} m", num(current[0]), num(current[1]));
                    start = current;
                    open = true;
                }
                for [c1, c2, end] in arc_to_cubics(current, *rx, *ry, rotation.to_radians(), *large_arc, *sweep, *to) {
                    let _ = writeln!(out, "{} {} {} {} {} {} c", num(c1[0]), num(c1[1]), num(c2[0]), num(c2[1]), num(end[0]), num(end[1]));
                }
                current = *to;
            }
            PathSegment::Close => {
                if open {
                    out.push_str("h\n");
                    current = start;
                    open = false;
                }
            }
        }
    }
    out
}

/// 🌀️ SVG endpoint arc → cubic Bézier runs (W3C SVG 1.1 appendix F.6.5, then one cubic per
/// quarter turn). A degenerate radius or coincident endpoints degrade to a straight line.
fn arc_to_cubics(from: [f64; 2], rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<[[f64; 2]; 3]> {
    let line = || vec![[from, to, to]];
    if from == to {
        return Vec::new();
    }
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    if rx == 0.0 || ry == 0.0 {
        return line();
    }
    let (sin_phi, cos_phi) = rotation.sin_cos();
    let dx2 = (from[0] - to[0]) / 2.0;
    let dy2 = (from[1] - to[1]) / 2.0;
    let x1p = cos_phi * dx2 + sin_phi * dy2;
    let y1p = -sin_phi * dx2 + cos_phi * dy2;
    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let rx2 = rx * rx;
    let ry2 = ry * ry;
    let numerator = (rx2 * ry2 - rx2 * y1p * y1p - ry2 * x1p * x1p).max(0.0);
    let denominator = rx2 * y1p * y1p + ry2 * x1p * x1p;
    if denominator == 0.0 {
        return line();
    }
    let mut coefficient = (numerator / denominator).sqrt();
    if large_arc == sweep {
        coefficient = -coefficient;
    }
    let cxp = coefficient * (rx * y1p / ry);
    let cyp = coefficient * (-(ry * x1p) / rx);
    let cx = cos_phi * cxp - sin_phi * cyp + (from[0] + to[0]) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (from[1] + to[1]) / 2.0;
    let angle = |ux: f64, uy: f64, vx: f64, vy: f64| -> f64 {
        let dot = ux * vx + uy * vy;
        let len = (ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt();
        if len == 0.0 {
            return 0.0;
        }
        let mut value = (dot / len).clamp(-1.0, 1.0).acos();
        if ux * vy - uy * vx < 0.0 {
            value = -value;
        }
        value
    };
    let theta1 = angle(1.0, 0.0, (x1p - cxp) / rx, (y1p - cyp) / ry);
    let mut delta = angle((x1p - cxp) / rx, (y1p - cyp) / ry, (-x1p - cxp) / rx, (-y1p - cyp) / ry);
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    } else if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    let pieces = ((delta.abs() / std::f64::consts::FRAC_PI_2) * ARC_CUBICS_PER_QUARTER).ceil().max(1.0) as usize;
    let step = delta / pieces as f64;
    let kappa = 4.0 / 3.0 * (step / 4.0).tan();
    let point = |theta: f64| -> [f64; 2] {
        let (sin_t, cos_t) = theta.sin_cos();
        [cx + rx * cos_t * cos_phi - ry * sin_t * sin_phi, cy + rx * cos_t * sin_phi + ry * sin_t * cos_phi]
    };
    let derivative = |theta: f64| -> [f64; 2] {
        let (sin_t, cos_t) = theta.sin_cos();
        [-rx * sin_t * cos_phi - ry * cos_t * sin_phi, -rx * sin_t * sin_phi + ry * cos_t * cos_phi]
    };
    let mut out = Vec::with_capacity(pieces);
    let mut theta = theta1;
    let mut start = from;
    for index in 0..pieces {
        let next = theta + step;
        let end = if index + 1 == pieces { to } else { point(next) };
        let d1 = derivative(theta);
        let d2 = derivative(next);
        out.push([[start[0] + kappa * d1[0], start[1] + kappa * d1[1]], [end[0] - kappa * d2[0], end[1] - kappa * d2[1]], end]);
        theta = next;
        start = end;
    }
    out
}
//#endregion 🔖️Paths

//#region 🔖️Writer
#[derive(Default)]
struct PdfContent {
    body:String,
    resources:BTreeSet<String>,
}

/// 🧾️ One PDF file under construction: the object table plus the page's resource dictionaries.
#[derive(Default)]
struct PdfWriter {
    objects: Vec<Vec<u8>>,
    font: Option<u32>,
    ext_g_states: BTreeMap<String, String>,
    shadings: Vec<(String, u32)>,
    images: BTreeMap<String, Option<String>>,
    image_objects: Vec<(String, u32)>,
    form_objects: Vec<(String,u32)>,
    resource_entries:BTreeMap<String,(&'static str,String)>,
}

impl PdfWriter {
    fn allocate(&mut self, body: Vec<u8>) -> u32 {
        self.objects.push(body);
        self.objects.len() as u32
    }

    fn reserve(&mut self) -> u32 {
        self.allocate(Vec::new())
    }

    fn set(&mut self, number: u32, body: Vec<u8>) {
        self.objects[number as usize - 1] = body;
    }

    fn helvetica(&mut self) -> String {
        if self.font.is_none() {
            let number = self.allocate(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".to_vec());
            self.font = Some(number);
            self.resource_entries.insert("F1".into(),("Font",format!("{number} 0 R")));
        }
        "F1".into()
    }

    /// 🎚️ One ExtGState per distinct (fill alpha, stroke alpha, blend mode); `None` when the node
    /// needs no state change at all.
    fn ext_g_state(&mut self, fill_alpha: f64, stroke_alpha: f64, blend_mode: &str) -> Option<String> {
        let blend = match blend_mode {
            "multiply" => "Multiply",
            "screen" => "Screen",
            "overlay" => "Overlay",
            "darken" => "Darken",
            "lighten" => "Lighten",
            "colorDodge" => "ColorDodge",
            "colorBurn" => "ColorBurn",
            "hardLight" => "HardLight",
            "softLight" => "SoftLight",
            "difference" => "Difference",
            "exclusion" => "Exclusion",
            "hue" => "Hue",
            "saturation" => "Saturation",
            "color" => "Color",
            "luminosity" => "Luminosity",
            _ => "Normal",
        };
        if fill_alpha >= 1.0 && stroke_alpha >= 1.0 && blend == "Normal" {
            return None;
        }
        let dictionary = format!("<< /Type /ExtGState /ca {} /CA {} /BM /{blend} >>", num(fill_alpha), num(stroke_alpha));
        if let Some(name) = self.ext_g_states.get(&dictionary) {
            return Some(name.clone());
        }
        let name = format!("GS{}", self.ext_g_states.len() + 1);
        self.resource_entries.insert(name.clone(),("ExtGState",dictionary.clone()));
        self.ext_g_states.insert(dictionary, name.clone());
        Some(name)
    }

    /// 🌈️ An axial (type 2) or radial (type 3) shading with a stitched stop function.
    fn shading(&mut self, fill: &FillStyle) -> Result<String, String> {
        let (kind, coords, stops) = match fill {
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => (2, format!("{} {} {} {}", num(*x1), num(*y1), num(*x2), num(*y2)), stops),
            FillStyle::RadialGradient { cx, cy, r, stops } => (3, format!("{} {} 0 {} {} {}", num(*cx), num(*cy), num(*cx), num(*cy), num(r.abs())), stops),
            FillStyle::Solid { .. } => return Err("a solid fill is not a shading".into()),
        };
        let function = self.stop_function(stops)?;
        let number = self.allocate(format!("<< /ShadingType {kind} /ColorSpace /DeviceRGB /Coords [{coords}] /Function {function} 0 R /Extend [true true] >>").into_bytes());
        let name = format!("Sh{}", self.shadings.len() + 1);
        self.shadings.push((name.clone(), number));
        self.resource_entries.insert(name.clone(),("Shading",format!("{number} 0 R")));
        Ok(name)
    }

    fn stop_function(&mut self, stops: &[GradientStop]) -> Result<u32, String> {
        let mut sorted: Vec<&GradientStop> = stops.iter().filter(|stop| stop.offset.is_finite()).collect();
        if sorted.is_empty() {
            return Err("a gradient needs at least one stop".into());
        }
        sorted.sort_by(|left, right| left.offset.total_cmp(&right.offset));
        let rgb = |stop: &GradientStop| format!("{} {} {}", num(stop.color[0]), num(stop.color[1]), num(stop.color[2]));
        if sorted.len() == 1 {
            let color = rgb(sorted[0]);
            return Ok(self.allocate(format!("<< /FunctionType 2 /Domain [0 1] /C0 [{color}] /C1 [{color}] /N 1 >>").into_bytes()));
        }
        // 🧵️ Offsets are clamped into [0, 1] and made strictly increasing so the stitching bounds are legal.
        let mut offsets: Vec<f64> = sorted.iter().map(|stop| stop.offset.clamp(0.0, 1.0)).collect();
        for index in 1..offsets.len() {
            if offsets[index] <= offsets[index - 1] {
                offsets[index] = (offsets[index - 1] + 1e-6).min(1.0);
            }
        }
        if offsets.len() == 2 && offsets[0] <= 0.0 && offsets[1] >= 1.0 {
            return Ok(self.allocate(format!("<< /FunctionType 2 /Domain [0 1] /C0 [{}] /C1 [{}] /N 1 >>", rgb(sorted[0]), rgb(sorted[1])).into_bytes()));
        }
        // 🪡️ Pad to the domain ends with flat pieces so /Extend does not have to guess the end colors.
        let mut pieces: Vec<(f64, f64, String, String)> = Vec::new();
        if offsets[0] > 0.0 {
            pieces.push((0.0, offsets[0], rgb(sorted[0]), rgb(sorted[0])));
        }
        for index in 1..sorted.len() {
            pieces.push((offsets[index - 1], offsets[index], rgb(sorted[index - 1]), rgb(sorted[index])));
        }
        if offsets[offsets.len() - 1] < 1.0 {
            let last = sorted[sorted.len() - 1];
            pieces.push((offsets[offsets.len() - 1], 1.0, rgb(last), rgb(last)));
        }
        let mut functions = Vec::with_capacity(pieces.len());
        for (_, _, c0, c1) in &pieces {
            functions.push(self.allocate(format!("<< /FunctionType 2 /Domain [0 1] /C0 [{c0}] /C1 [{c1}] /N 1 >>").into_bytes()));
        }
        let bounds = pieces.iter().skip(1).map(|(start, _, _, _)| num(*start)).collect::<Vec<_>>().join(" ");
        let encode = pieces.iter().map(|_| "0 1").collect::<Vec<_>>().join(" ");
        let refs = functions.iter().map(|number| format!("{number} 0 R")).collect::<Vec<_>>().join(" ");
        Ok(self.allocate(format!("<< /FunctionType 3 /Domain [0 1] /Functions [{refs}] /Bounds [{bounds}] /Encode [{encode}] >>").into_bytes()))
    }

    /// 🖼️ One RGB image XObject (+ alpha SMask) per distinct asset source; an asset that does not
    /// decode is skipped (`None`) rather than failing the whole export.
    fn image(&mut self, doc: &DrawingSnapshot, src: &str) -> Result<Option<String>, String> {
        if let Some(name) = self.images.get(src) {
            return Ok(name.clone());
        }
        let Some(raster) = decode_image_source(doc, src) else {
            self.images.insert(src.to_owned(), None);
            return Ok(None);
        };
        let pixel_count = raster.width as usize * raster.height as usize;
        let mut rgb = Vec::with_capacity(pixel_count * 3);
        let mut alpha = Vec::with_capacity(pixel_count);
        let mut opaque = true;
        for pixel in raster.pixels.chunks_exact(4) {
            rgb.extend_from_slice(&pixel[..3]);
            alpha.push(pixel[3]);
            opaque &= pixel[3] == 255;
        }
        let smask = if opaque {
            None
        } else {
            let data = zlib(&alpha)?;
            Some(self.allocate(stream(&format!("/Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceGray /BitsPerComponent 8 /Filter /FlateDecode", raster.width, raster.height), &data)))
        };
        let data = zlib(&rgb)?;
        let smask_entry = smask.map_or(String::new(), |number| format!(" /SMask {number} 0 R"));
        let number = self.allocate(stream(&format!("/Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode{smask_entry}", raster.width, raster.height), &data));
        let name = format!("Im{}", self.image_objects.len() + 1);
        self.image_objects.push((name.clone(), number));
        self.resource_entries.insert(name.clone(),("XObject",format!("{number} 0 R")));
        self.images.insert(src.to_owned(), Some(name.clone()));
        Ok(Some(name))
    }

    fn transparency_form(&mut self,width:f64,height:f64,content:PdfContent)->Result<String,String> {
        let resources=self.resources(&content.resources);
        let data=zlib(content.body.as_bytes())?;
        let number=self.allocate(stream(&format!("/Type /XObject /Subtype /Form /FormType 1 /BBox [0 0 {} {}] /Group << /S /Transparency /CS /DeviceRGB /I true /K false >> /Resources {resources} /Filter /FlateDecode",num(width),num(height)),&data));
        let name=format!("Fm{}",self.form_objects.len()+1);
        self.form_objects.push((name.clone(),number));
        self.resource_entries.insert(name.clone(),("XObject",format!("{number} 0 R")));
        Ok(name)
    }

    fn paint_form(&mut self,content:&mut PdfContent,name:&str,opacity:f64,blend:&str) {
        content.body.push_str("q\n");
        if let Some(state)=self.ext_g_state(opacity,opacity,blend) {
            let _=writeln!(content.body,"/{state} gs");
            content.resources.insert(state);
        }
        let _=writeln!(content.body,"/{name} Do\nQ");
        content.resources.insert(name.into());
    }

    fn resources(&self,used:&BTreeSet<String>)->String {
        let mut kinds:BTreeMap<&str,Vec<String>>=BTreeMap::new();
        for name in used {
            let (kind,entry)=self.resource_entries.get(name).expect("a declared PDF paint resource");
            kinds.entry(kind).or_default().push(format!("/{name} {entry}"));
        }
        let mut resources=String::from("<<");
        for (kind,entries) in kinds {let _=write!(resources," /{kind} << {} >>",entries.join(" "));}
        resources.push_str(" >>");
        resources
    }

    /// 📦️ Catalog, page tree, content and scoped resources, followed by the classic cross-reference table.
    fn finish(mut self, width: f64, height: f64, content: PdfContent) -> Result<Vec<u8>, String> {
        let catalog = self.reserve();
        let pages = self.reserve();
        let page = self.reserve();
        let compressed = zlib(content.body.as_bytes())?;
        let contents = self.allocate(stream("/Filter /FlateDecode", &compressed));
        let resources=self.resources(&content.resources);
        self.set(catalog, format!("<< /Type /Catalog /Pages {pages} 0 R >>").into_bytes());
        self.set(pages, format!("<< /Type /Pages /Kids [{page} 0 R] /Count 1 >>").into_bytes());
        self.set(page, format!("<< /Type /Page /Parent {pages} 0 R /MediaBox [0 0 {} {}] /Contents {contents} 0 R /Resources {resources} >>", num(width), num(height)).into_bytes());

        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");
        let mut offsets = Vec::with_capacity(self.objects.len());
        for (index, object) in self.objects.iter().enumerate() {
            offsets.push(body.len());
            body.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            body.extend_from_slice(object);
            body.extend_from_slice(b"\nendobj\n");
        }
        let xref_offset = body.len();
        let count = self.objects.len() + 1;
        body.extend_from_slice(format!("xref\n0 {count}\n0000000000 65535 f \n").as_bytes());
        for offset in offsets {
            body.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        body.extend_from_slice(format!("trailer\n<< /Size {count} /Root {catalog} 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n").as_bytes());
        Ok(body)
    }
}

fn stream(dictionary_entries: &str, data: &[u8]) -> Vec<u8> {
    let mut out = format!("<< {dictionary_entries} /Length {} >>\nstream\n", data.len()).into_bytes();
    out.extend_from_slice(data);
    out.extend_from_slice(b"\nendstream");
    out
}

fn zlib(data: &[u8]) -> Result<Vec<u8>, String> {
    semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress_deterministic(data).map_err(|error| format!("flate: {error}"))
}

/// 🖼️ A scene image `src` is the asset's `data:` URI (or its bare base64 body); decode it to RGBA
/// through the same PNG path the trace layer uses, honouring the asset's declared size.
fn decode_image_source(doc: &DrawingSnapshot, src: &str) -> Option<semio_framework_pixels::RasterImage> {
    let body = match src.strip_prefix("data:") {
        Some(rest) => rest.split_once(',').map_or(rest, |(_, data)| data),
        None => src,
    };
    let bytes = base64_codec::base64_standard_decode(body).ok()?;
    let decoded = semio_framework_pixels::decode_png(&bytes).ok()?;
    let declared = doc.assets.values().find(|asset| asset.data == src || src.ends_with(asset.data.as_str())).and_then(|asset| asset.width.zip(asset.height));
    Some(match declared {
        Some((width, height)) if width > 0 && height > 0 && (width, height) != (decoded.width, decoded.height) => semio_framework_pixels::resize_bilinear(&decoded, width, height),
        _ => decoded,
    })
}
//#endregion 🔖️Writer

//#region 🔖️Lexical
/// 🔢️ A PDF real: at most four decimals, no trailing zeros, no exponent, `0` for anything non-finite.
fn num(value: f64) -> String {
    if !value.is_finite() {
        return "0".into();
    }
    let mut text = format!("{value:.4}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        text = "0".into();
    }
    text
}

/// 🔤️ A literal string in WinAnsi: parentheses and backslashes escaped, Latin-1 as octal escapes,
/// anything wider replaced by `?`.
fn pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '(' | ')' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if (ch as u32) < 0x20 => {}
            ch if ch.is_ascii() => out.push(ch),
            ch if (ch as u32) <= 0xFF => {
                let _ = write!(out, "\\{:03o}", ch as u32);
            }
            _ => out.push('?'),
        }
    }
    out
}
//#endregion 🔖️Lexical

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
