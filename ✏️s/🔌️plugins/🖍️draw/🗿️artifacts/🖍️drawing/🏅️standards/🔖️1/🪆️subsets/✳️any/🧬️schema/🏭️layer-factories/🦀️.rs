//! 🏭️ Drawing layer factories: the default layer base and the constructors of the group, boolean, trace, shape, text and image layers, and the layer constructor by kind label.

use super::*;

//#region 🔖️DocumentHelpers
//#region 🔖️Tree
/// 📄️ Parses the handcrafted DSL fixture once per call — used both for `setActiveExample`'s in-plugin
/// document load and to bridge into the framework's still-JSON-only `App::example`/render-override
/// surfaces, so `SEMIO_DRAW_EXAMPLE_TEXT` stays the single source of truth for the fixture.


pub fn default_layer_base(identity: DrawingIdentity, name: &str) -> DrawingLayerBase {
    DrawingLayerBase { id: identity.into_key(), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() }
}

pub fn create_drawing_group_layer(identity: DrawingIdentity, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Group(DrawingGroupBody {
        isolation:false,
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes::default(),
        },
        children: Default::default(),
    })
}

pub fn create_drawing_boolean_layer(identity: DrawingIdentity, name: &str, operation: &str, children: semio_framework_value_persistent::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>) -> DrawingLayerNode {
    DrawingLayerNode::Boolean(DrawingBooleanBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes::default(),
        },
        operation: operation.into(),
        children,
    })
}

pub fn create_drawing_trace_layer(identity: DrawingIdentity, name: &str, source_key: &str) -> DrawingLayerNode {
    DrawingLayerNode::Trace(DrawingTraceBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes::default(),
        },
        source_key: source_key.into(),
        params: default_drawing_trace_params(),
    })
}

pub fn create_drawing_shape_layer_rect(identity: DrawingIdentity, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Shape(DrawingShapeBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes::default(),
        },
        shape_kind: "rect".into(),
        rect: Some(DrawingRect { x: 0.0, y: 0.0, width: 128.0, height: 96.0 }),
        ellipse: None,
        circle: None,
        line: None,
        polygon: None,
    })
}

pub fn create_drawing_text_layer(identity: DrawingIdentity, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Text(DrawingTextBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes { fill_rule: crate::fill_rule::FillRule::Evenodd, fill: Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }), stroke: None },
        },
        x: 0.0,
        y: 0.0,
        content: "Text".into(),
        size: 24.0,
        font_family:crate::DrawingFontFamily::Anta,
    })
}

pub fn create_drawing_image_layer(identity: DrawingIdentity, name: &str, image_key: &str) -> DrawingLayerNode {
    DrawingLayerNode::Image(DrawingImageBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes::default(),
        },
        image_key: image_key.into(),
        width: 256.0,
        height: 256.0,
    })
}

pub fn create_layer_by_kind(identity: DrawingIdentity, kind: &str) -> DrawingLayerNode {
    if let Some(shape_kind) = kind.strip_prefix("shape:") {
        return match shape_kind {
            "rect" => create_drawing_shape_layer_rect(identity, "Rectangle"),
            "ellipse" => DrawingLayerNode::Shape(DrawingShapeBody {
                base: default_layer_base(identity, "Ellipse"),
                shape_kind: "ellipse".into(),
                rect: None,
                ellipse: Some(DrawingEllipse { cx: 0.0, cy: 0.0, rx: 64.0, ry: 48.0 }),
                circle: None,
                line: None,
                polygon: None,
            }),
            "line" => DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(identity, "Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawingLine { x1: 0.0, y1: 0.0, x2: 128.0, y2: 0.0 }), polygon: None }),
            "polygon" => DrawingLayerNode::Shape(DrawingShapeBody {
                base: default_layer_base(identity, "Polygon"),
                shape_kind: "polygon".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: None,
                polygon: Some(DrawingPolygon { points: vec![[0.0, 0.0], [64.0, 0.0], [32.0, 48.0]].into() }),
            }),
            _ => create_drawing_shape_layer_rect(identity, "Shape"),
        };
    }
    match kind {
        "path" => create_drawing_path_layer(identity, "Path", Vec::new().into()),
        "text" => create_drawing_text_layer(identity, "Text"),
        "image" => create_drawing_image_layer(identity, "Image", "image-source"),
        "group" => create_drawing_group_layer(identity, "Group"),
        "boolean" => create_drawing_boolean_layer(identity, "Boolean", "union", Vec::new().into()),
        "trace" => create_drawing_trace_layer(identity, "Trace", "trace-source"),
        _ => create_drawing_path_layer(identity, "Path", Vec::new().into()),
    }
}
//#endregion 🔖️Tree
//#endregion 🔖️DocumentHelpers
