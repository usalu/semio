//! 📤️ Drawing export: the serializers that write a drawing as SVG, PDF, PNG, JSON, DWG and DXF, and the owned export job.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_geometry::drawing_layer_world_bounds;
use semio_s_artifact_draw_drawing_model::flatten_drawing_layers;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_draw_drawing_model::PathSegment;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::PathSegment as SemioPathSegment;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_draw_drawing_model::FillStyle;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_draw_drawing_model::DrawingSceneNode;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::DrawStyle as SemioDrawStyle;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::DrawNode as SemioDrawNode;
use semio_s_artifact_draw_drawing_io::image;
use semio_s_artifact_draw_drawing_io::text;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::DrawCanvas as SemioDrawCanvas;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::DrawLayer as SemioDrawLayer;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_draw_drawing_scene::flatten_drawing_document_to_scene_nodes;
fn resolve_drawing_document_artboard(doc: &DrawingSnapshot) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (artboard.width.max(1.0).round() as u32, artboard.height.max(1.0).round() as u32);
    }
    let mut max_x: f64 = 1024.0;
    let mut max_y: f64 = 1024.0;
    for layer in flatten_drawing_layers(&doc.layers) {
        if let Some((x, y, width, height)) = drawing_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// 🌉️ [DrawingTransform]'s 6-value affine matrix → semio's [SemioTransform] (Z-only rotation
/// quaternion, axis scale, zero-z translation) — the same decomposition stdio's own svg↔drawing
/// bridge applies on its side (`matrix_to_semio_transform` in that leaf).
fn matrix_to_semio_transform(matrix: [f64; 6]) -> SemioTransform {
    let transform = semio_s_artifact_draw_drawing_model::affine::drawing_matrix_to_transform(matrix);
    SemioTransform {
        translation: SemioPoint3 { x: transform.x, y: transform.y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (transform.rotation / 2.0).sin(), w: (transform.rotation / 2.0).cos() },
        scale: SemioPoint3 { x: transform.scale_x, y: transform.scale_y, z: 1.0 },
    }
}

/// ✏️ Drawing's own [PathSegment] → semio's [SemioPathSegment] — same SVG-command grammar, field
/// renames only (no geometry recomputed).
fn to_semio_path_segment(segment: &PathSegment) -> SemioPathSegment {
    match *segment {
        PathSegment::Move { to } => SemioPathSegment::MoveTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Line { to } => SemioPathSegment::LineTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Quad { ctrl, to } => SemioPathSegment::QuadTo { c: SemioPoint2 { x: ctrl[0], y: ctrl[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Cubic { ctrl1, ctrl2, to } => SemioPathSegment::CubicTo { c1: SemioPoint2 { x: ctrl1[0], y: ctrl1[1] }, c2: SemioPoint2 { x: ctrl2[0], y: ctrl2[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => SemioPathSegment::ArcTo { rx, ry, x_rotation: rotation, large_arc, sweep, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Close => SemioPathSegment::Close,
    }
}

/// 🎨️ [FillStyle::Solid]/[StrokeStyle] → [SemioRgba] — `DrawingStyle` is solid-color-only, so
/// gradients have no representable equivalent and are honestly dropped (matching the pre-migration
/// SVG renderer's own gradient fallback: no fill, not a fabricated flat color).
fn solid_fill_to_semio_rgba(fill: &FillStyle) -> Option<SemioRgba> {
    match fill {
        FillStyle::Solid { color } => Some(SemioRgba { r: color[0] as f32, g: color[1] as f32, b: color[2] as f32, a: color[3] as f32 }),
        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => None,
    }
}

/// 🎨️ Interns one [DrawingSceneNode]'s fill/stroke/opacity as a named [SemioDrawStyle] and returns
/// its name, or `None` when the node carries no representable presentation at all. 🕳️ stdio_gap:
/// `blend_mode`/`fill_rule` have no `DrawingStyle` field and `Group`/`Image` nodes have no opacity
/// slot at all (only `Path`/`Text` reference a style) — both honestly dropped, not fabricated.
fn intern_semio_style(styles: &mut Vec<SemioDrawStyle>, node: &DrawingSceneNode) -> Option<String> {
    let fill = node.fill.as_ref().and_then(solid_fill_to_semio_rgba);
    let stroke = node.stroke.as_ref().map(|style| SemioRgba { r: style.color[0] as f32, g: style.color[1] as f32, b: style.color[2] as f32, a: style.color[3] as f32 });
    let stroke_width = node.stroke.as_ref().map(|style| style.width);
    let opacity = if (node.opacity - 1.0).abs() > f64::EPSILON { Some(node.opacity as f32) } else { None };
    if fill.is_none() && stroke.is_none() && opacity.is_none() {
        return None;
    }
    let name = format!("style{}", styles.len());
    styles.push(SemioDrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity });
    Some(name)
}

/// 🖍️ One [DrawingSceneNode] → semio's recursive [SemioDrawNode]: each becomes its own `Group`
/// carrying the node's baked world transform, wrapping exactly one Path/Text/Image leaf (mirrors
/// the pre-migration SVG renderer's own `<g transform="matrix(...)"><path/></g>` shape).
fn semio_drawing_node_from_scene_node(doc:&DrawingSnapshot,node: &DrawingSceneNode, styles: &mut Vec<SemioDrawStyle>) -> Result<Option<SemioDrawNode>,String> {
    let style = intern_semio_style(styles, node);
    if let Some(text) = &node.text {
        let children = semio_framework_2d::text::drawing_text_lines(&text.content).enumerate().filter(|(_, line)| !line.is_empty()).map(|(index, line)| {
            SemioDrawNode::Text { value: line.to_owned(), at: SemioPoint2 { x: 0.0, y: text.size + index as f64 * text.size * semio_framework_2d::text::DRAWING_TEXT_LINE_HEIGHT }, style: style.clone() }
        }).collect();
        return Ok(Some(SemioDrawNode::Group { transform: matrix_to_semio_transform(node.transform), children }));
    }
    let leaf = if let Some(image) = &node.image {
        let asset=doc.assets.get(&image.asset_id).ok_or_else(||format!("Missing scene image asset: {}",image.asset_id))?;
        let mime="image/png".into();let bytes=semio_s_artifact_draw_drawing_io::image::drawing_image_png(asset)?;
        SemioDrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: image.width, height: image.height, mime, bytes }
    } else {
        let segments: Vec<SemioPathSegment> = node.segments.iter().map(to_semio_path_segment).collect();
        if segments.is_empty() {
            return Ok(None);
        }
        SemioDrawNode::Path { segments, style }
    };
    Ok(Some(SemioDrawNode::Group { transform: matrix_to_semio_transform(node.transform), children: vec![leaf] }))
}

/// 🌉️ Builds a real [SemioDrawingSnapshot] from this plugin's own domain document — the semio hub
/// side of drawing's domain↔semio bridge. [flatten_drawing_document_to_scene_nodes] has already resolved
/// booleans/traces/curve-flattening, so every scene node here is a concrete leaf.
pub fn drawing_document_to_semio_drawing(doc: &DrawingSnapshot) -> Result<SemioDrawingSnapshot,String> {
    let (width, height) = resolve_drawing_document_artboard(doc);
    let mut styles = Vec::new();
    let mut children=Vec::new();for node in &flatten_drawing_document_to_scene_nodes(doc){if let Some(child)=semio_drawing_node_from_scene_node(doc,node,&mut styles)?{children.push(child);}}
    Ok(SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: SemioDrawCanvas { width: width as f64, height: height as f64, background: None },
        styles,
        layers: vec![SemioDrawLayer { id: "root".into(), name: doc.title.as_ref().map(|title| title.to_string_owner()).unwrap_or_else(|| "root".into()), visible: true, root: SemioDrawNode::Group { transform: SemioTransform::identity(), children } }],
    })
}

/// 🎨️ Exports the authored scene through the typed SVG serializer.
pub fn drawing_document_to_svg(doc: &DrawingSnapshot) -> Result<(String, u32, u32), String> {
    use crate::serializers::artifacts as export;
    export::svg::v1_1::any::drawing_document_to_svg(doc)
}

pub fn drawing_document_json_to_svg(value: &semio_framework_value::DslValue) -> Result<(String, u32, u32), String> {
    let doc: DrawingSnapshot = semio_framework_value::FromValue::from_value(value.clone()).map_err(|error: semio_framework_value::ValueError| error.to_string())?;
    drawing_document_to_svg(&doc)
}

//#region 🪢️TaxonomyMounts
#[path = "📦️owned/🦀️.rs"]
pub mod owned;
#[path = "."]
pub mod serializers {
    #[path = "."]
    pub mod artifacts {
        #[path = "."]
        pub mod dwg {
            #[path = "."]
            pub mod v_ac1018 {
                #[path = "🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod dxf {
            #[path = "."]
            pub mod v_r12 {
                #[path = "🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod json {
            #[path = "."]
            pub mod v_rfc8259 {
                #[path = "🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod pdf {
            #[path = "."]
            pub mod v1_4 {
                #[path = "🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod png {
            #[path = "."]
            pub mod v1_2 {
                #[path = "🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
        #[path = "."]
        pub mod svg {
            #[path = "."]
            pub mod v1_1 {
                #[path = "🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                pub mod any;
            }
        }
    }
}
//#endregion 🪢️TaxonomyMounts
