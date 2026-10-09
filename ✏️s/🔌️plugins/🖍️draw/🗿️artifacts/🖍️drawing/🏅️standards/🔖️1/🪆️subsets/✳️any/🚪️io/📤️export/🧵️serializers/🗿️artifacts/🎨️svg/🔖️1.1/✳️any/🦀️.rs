//! 🎨️ Drawing scene to typed SVG, preserving authored geometry, paint and affine transforms.

use crate::DrawingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct DrawingIntoSvg;

impl Serializer<DrawingSnapshot> for DrawingIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let (svg_text, _width, _height) = drawing_document_to_svg(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DrawingIntoSvg: {message}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(svg_text)))
    }
}

use crate::schema::{DrawingSceneNode, flatten_drawing_document_to_scene_nodes};
use crate::{FillStyle, GradientStop, PathSegment};
use crate::schema::fill::sampling::PreparedFill;
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::typed_to_svg_document;
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{CommonAttrs, PathCommand, SvgElement, TransformOp, ViewBox};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;

fn attr(name: &str, value: impl ToString) -> XmlAttr { XmlAttr { name: name.into(), value: value.to_string() } }
fn rgb(color: &[f64; 4]) -> String { format!("rgb({},{},{})", color[0] * 255.0, color[1] * 255.0, color[2] * 255.0) }
fn blend(mode: &str) -> &str {
    match mode { "colorDodge" => "color-dodge", "colorBurn" => "color-burn", "hardLight" => "hard-light", "softLight" => "soft-light", "multiply" | "screen" | "overlay" | "darken" | "lighten" | "difference" | "exclusion" | "hue" | "saturation" | "color" | "luminosity" => mode, _ => "normal" }
}

fn stops(values: &semio_framework_value::list::PagedList<GradientStop, {usize::MAX}>) -> Vec<SvgElement> {
    let mut values = values.iter().collect::<Vec<_>>();
    values.sort_by(|a,b| a.offset.total_cmp(&b.offset));
    values.into_iter().map(|stop| SvgElement::Stop { common: CommonAttrs::default(), offset: stop.offset.clamp(0.0,1.0).to_string(), stop_color: Some(rgb(&stop.color)), stop_opacity: Some(stop.color[3].to_string()) }).collect()
}

fn presentation(node: &DrawingSceneNode, index: usize, defs: &mut Vec<SvgElement>) -> Result<CommonAttrs,String> {
    let mut common = CommonAttrs::default().with_fill("none").with_stroke("none");
    let constant=node.fill.as_ref().map(PreparedFill::new).transpose().map_err(|message|format!("SVG layer {}: {message}",node.id))?.and_then(|fill|fill.constant_color());
    match (&node.fill,constant) {
        (_,Some(color)) => {
            common.presentation.fill = Some(rgb(&color));
            common.presentation.fill_opacity = Some(color[3].to_string());
        }
        (Some(fill),None) => {
            let id = format!("draw-gradient-{index}");
            let gradient_common = CommonAttrs { extra_attrs: vec![attr("gradientUnits", "userSpaceOnUse")], ..Default::default() };
            let element = match fill {
                FillStyle::LinearGradient { x1,y1,x2,y2,stops: values } => SvgElement::LinearGradient { common: gradient_common, id: Some(id.clone()), x1: Some(x1.to_string()), y1: Some(y1.to_string()), x2: Some(x2.to_string()), y2: Some(y2.to_string()), children: stops(values) },
                FillStyle::RadialGradient { cx,cy,r,stops: values } => SvgElement::RadialGradient { common: gradient_common, id: Some(id.clone()), cx: Some(cx.to_string()), cy: Some(cy.to_string()), r: Some(r.to_string()), fx: Some(cx.to_string()), fy: Some(cy.to_string()), children: stops(values) },
                FillStyle::Solid { .. } => unreachable!(),
            };
            defs.push(element);
            common.presentation.fill = Some(format!("url(#{id})"));
        }
        (None,None) => {}
    }
    if let Some(stroke) = node.stroke.as_ref().filter(|stroke| stroke.width.is_finite() && stroke.width > 0.0) {
        common.presentation.stroke = Some(rgb(&stroke.color));
        common.presentation.stroke_opacity = Some(stroke.color[3].to_string());
        common.presentation.stroke_width = Some(stroke.width.to_string());
        common.extra_attrs.extend([attr("stroke-linecap", stroke.cap.as_str()), attr("stroke-linejoin", stroke.join.as_str())]);
        if let Some(dash) = &stroke.dash { common.extra_attrs.push(attr("stroke-dasharray", dash.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "))); }
    }
    common.extra_attrs.push(attr("fill-rule", node.fill_rule.as_deref().unwrap_or("evenodd")));
    Ok(common)
}

fn path_command(segment: &PathSegment) -> PathCommand {
    match *segment {
        PathSegment::Move { to: [x,y] } => PathCommand::MoveTo { x,y,relative:false },
        PathSegment::Line { to: [x,y] } => PathCommand::LineTo { x,y,relative:false },
        PathSegment::Quad { ctrl: [x1,y1], to: [x,y] } => PathCommand::QuadraticCurveTo { x1,y1,x,y,relative:false },
        PathSegment::Cubic { ctrl1: [x1,y1],ctrl2: [x2,y2],to: [x,y] } => PathCommand::CurveTo { x1,y1,x2,y2,x,y,relative:false },
        PathSegment::Arc { rx,ry,rotation,large_arc,sweep,to: [x,y] } => PathCommand::Arc { rx,ry,x_axis_rotation:rotation,large_arc,sweep,x,y,relative:false },
        PathSegment::Close => PathCommand::ClosePath,
    }
}

fn finite_numbers(node: &DrawingSceneNode) -> bool {
    let finite = |values: &[f64]| values.iter().all(|value| value.is_finite());
    let geometry = node.segments.iter().all(|segment| match segment {
        PathSegment::Move { to } | PathSegment::Line { to } => finite(to),
        PathSegment::Quad { ctrl,to } => finite(ctrl) && finite(to),
        PathSegment::Cubic { ctrl1,ctrl2,to } => finite(ctrl1) && finite(ctrl2) && finite(to),
        PathSegment::Arc { rx,ry,rotation,to,.. } => finite(&[*rx,*ry,*rotation]) && finite(to),
        PathSegment::Close => true,
    });
    let fill = node.fill.as_ref().is_none_or(|fill| {
        let (coordinates,stops) = match fill {
            FillStyle::Solid { color } => return finite(color),
            FillStyle::LinearGradient { x1,y1,x2,y2,stops } => (finite(&[*x1,*y1,*x2,*y2]),stops),
            FillStyle::RadialGradient { cx,cy,r,stops } => (finite(&[*cx,*cy,*r]),stops),
        };
        coordinates && stops.iter().all(|stop| stop.offset.is_finite() && finite(&stop.color))
    });
    geometry && fill && node.opacity.is_finite() && finite(&node.transform)
        && node.stroke.as_ref().is_none_or(|stroke| stroke.width.is_finite() && finite(&stroke.color) && stroke.dash.as_ref().is_none_or(|dash| dash.iter().all(|value| value.is_finite())))
        && node.text.as_ref().is_none_or(|text| text.size.is_finite())
        && node.image.as_ref().is_none_or(|image| finite(&[image.width,image.height]))
}

fn close_group(stack:&mut Vec<(&crate::schema::DrawingSceneGroup,Vec<SvgElement>)>,root:&mut Vec<SvgElement>) {
    let (group,children)=stack.pop().expect("open compositing group");
    let mut common=CommonAttrs::default().with_opacity(group.opacity.to_string());
    common.extra_attrs.push(attr("data-group-id",&group.id));
    common.presentation.extra_style.push(("isolation".into(),"isolate".into()));
    common.presentation.extra_style.push(("mix-blend-mode".into(),blend(&group.blend_mode).into()));
    let node=SvgElement::Group {common,children};
    if let Some((_,children))=stack.last_mut() {children.push(node);}else {root.push(node);}
}

/// 🖼️ Serializes scene-space nodes using the first-party SVG/XML vocabulary and writer.
pub fn drawing_scene_to_svg(nodes: &[DrawingSceneNode], view_box: [f64;4],assets:&semio_framework_value::paged::PagedMap<crate::DrawingImageAsset,{usize::MAX}>) -> Result<String,String> {
    let [x,y,width,height] = view_box;
    if !view_box.iter().all(|value| value.is_finite()) || width <= 0.0 || height <= 0.0 { return Err("SVG view box must have finite coordinates and positive dimensions".into()); }
    let mut defs = Vec::new();
    let mut children = Vec::new();
    let mut stack:Vec<(&crate::schema::DrawingSceneGroup,Vec<SvgElement>)>=Vec::new();
    let mut opened=std::collections::BTreeSet::new();
    for (index,node) in nodes.iter().enumerate().filter(|(_,node)| node.visible) {
        if !finite_numbers(node) { return Err(format!("SVG layer {} geometry and paint must be finite", node.id)); }
        if node.opacity <= 0.0 { continue; }
        if !node.transform.iter().all(|value| value.is_finite()) { return Err("SVG layer transform must be finite".into()); }
        let mut common_depth=0;
        while common_depth<stack.len() && common_depth<node.groups.len() && stack[common_depth].0==&node.groups[common_depth] {common_depth+=1;}
        while stack.len()>common_depth {close_group(&mut stack,&mut children);}
        for group in &node.groups[common_depth..] {
            if group.id.is_empty() || !opened.insert(group.id.clone()) || !group.opacity.is_finite() || !(0.0..=1.0).contains(&group.opacity) || group.blend_mode!="normal" && blend(&group.blend_mode)=="normal" {return Err("Invalid scene compositing hierarchy".into());}
            stack.push((group,Vec::new()));
        }
        let mut common = presentation(node,index,&mut defs)?;
        let leaf = if let Some(text) = &node.text {
            common.presentation.font_size = Some(text.size.to_string());
            common.presentation.font_family = Some("ui-sans-serif, system-ui, sans-serif".into());
            common.extra_attrs.push(attr("xml:space", "preserve"));
            let lines = semio_framework_2d::text::drawing_text_lines(&text.content).enumerate().filter(|(_,line)| !line.is_empty()).map(|(index,line)| SvgElement::Tspan {
                common: CommonAttrs::default(), x: Some(0.0), y: Some(text.size + index as f64 * text.size * semio_framework_2d::text::DRAWING_TEXT_LINE_HEIGHT), children: vec![SvgElement::TextNode(line.into())]
            }).collect();
            SvgElement::Text { common,x:None,y:None,children:lines }
        } else if let Some(image) = &node.image {
            let src=crate::standards::v1::subsets::any::io::image::drawing_image_data_uri(assets.get(&image.asset_id).ok_or_else(||format!("Missing scene image asset: {}",image.asset_id))?)?;
            SvgElement::Unknown { name:"image".into(), attrs:vec![attr("x",0),attr("y",0),attr("width",image.width),attr("height",image.height),attr("preserveAspectRatio","none"),attr("href",&src),attr("xlink:href",&src)], children:Vec::new() }
        } else { SvgElement::Path { common,d:node.segments.iter().map(path_command).collect() } };
        let [a,b,c,d,e,f] = node.transform;
        let mut wrapper = CommonAttrs::default().with_transform(vec![TransformOp::Matrix { a,b,c,d,e,f }]).with_opacity(node.opacity.to_string());
        wrapper.extra_attrs.push(attr("data-layer-id", &node.id));
        if node.blend_mode != "normal" { wrapper.presentation.extra_style.push(("mix-blend-mode".into(),blend(&node.blend_mode).into())); }
        let node=SvgElement::Group { common:wrapper,children:vec![leaf] };
        if let Some((_,children))=stack.last_mut() {children.push(node);}else {children.push(node);}
    }
    while !stack.is_empty() {close_group(&mut stack,&mut children);}
    if !defs.is_empty() { children.insert(0,SvgElement::Defs { common:CommonAttrs::default(),children:defs }); }
    let common = CommonAttrs { extra_attrs: vec![attr("version","1.1"),attr("xmlns:xlink","http://www.w3.org/1999/xlink")], ..Default::default() };
    let root = SvgElement::Svg { common,view_box:Some(ViewBox { min_x:x,min_y:y,width,height }),width:Some(width.to_string()),height:Some(height.to_string()),xmlns:Some("http://www.w3.org/2000/svg".into()),children };
    xml_document_to_text_checked(&typed_to_svg_document(&root,None))
}

pub fn drawing_document_to_svg(doc: &DrawingSnapshot) -> Result<(String,u32,u32),String> {
    let nodes = flatten_drawing_document_to_scene_nodes(doc);
    let view_box = if let Some(board) = &doc.artboard { [0.0,0.0,board.width,board.height] } else {
        let [x1,y1,x2,y2] = crate::schema::geometry::framing::drawing_scene_bounds(None,&nodes);
        [x1,y1,(x2-x1).max(1.0),(y2-y1).max(1.0)]
    };
    let svg = drawing_scene_to_svg(&nodes,view_box,&doc.assets)?;
    Ok((svg,view_box[2].ceil() as u32,view_box[3].ceil() as u32))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="🧵️write/🦀️.rs"]
pub mod write;
