//! 🖊️ Artifact-free independent DXF semantic reader, shared by drawing format consumers.

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
#[path = "🧰️support/🦀️.rs"]
mod reference_support;

#[cfg(feature = "oracles")]
mod reader {
    use super::Json;
    use super::reference_support::{load, point_json, obj};
    use dxf::entities::{Entity, EntityType};
    use dxf::tables::{Layer, LineType, Style};
    use dxf::Block;

    fn entity_to_json(entity: &Entity) -> Result<Json, String> {
        let (kind, mut fields): (&str, Vec<(String, Json)>) = match &entity.specific {
            EntityType::Line(l) => ("line", vec![("start".to_string(), point_json(&l.p1)), ("end".to_string(), point_json(&l.p2))]),
            EntityType::Circle(c) => ("circle", vec![("center".to_string(), point_json(&c.center)), ("radius".to_string(), Json::Number(c.radius))]),
            EntityType::Arc(a) => {
                ("arc", vec![("center".to_string(), point_json(&a.center)), ("radius".to_string(), Json::Number(a.radius)), ("startAngle".to_string(), Json::Number(a.start_angle)), ("endAngle".to_string(), Json::Number(a.end_angle))])
            }
            EntityType::Text(t) => ("text", vec![("position".to_string(), point_json(&t.location)), ("height".to_string(), Json::Number(t.text_height)), ("value".to_string(), Json::String(t.value.clone()))]),
            EntityType::Solid(s) => ("solid", vec![("points".to_string(), Json::Array(vec![point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)]))]),
            EntityType::Insert(i) => ("insert", vec![("blockName".to_string(), Json::String(i.name.clone())), ("position".to_string(), point_json(&i.location))]),
            other => return Err(format!("dxf oracle: cannot capture inverse value for unsupported entity kind {other:?}")),
        };
        fields.push(("entityKind".to_string(), Json::String(kind.to_string())));
        fields.push(("layer".to_string(), Json::String(entity.common.layer.clone())));
        Ok(Json::Object(fields))
    }

    fn entity_projection(entity: &Entity) -> Json {
        entity_to_json(entity).unwrap_or_else(|_| obj(vec![("entityKind", Json::String("other".to_string())), ("layer", Json::String(entity.common.layer.clone()))]))
    }

    fn layer_to_json(layer: &Layer) -> Json {
        obj(vec![("name", Json::String(layer.name.clone())), ("color", Json::Number(layer.color.index().unwrap_or(7) as f64)), ("linetype", Json::String(layer.line_type_name.clone()))])
    }

    fn style_to_json(style: &Style) -> Json {
        obj(vec![("name", Json::String(style.name.clone())), ("font", Json::String(style.primary_font_file_name.clone()))])
    }

    fn linetype_to_json(linetype: &LineType) -> Json {
        obj(vec![("name", Json::String(linetype.name.clone())), ("description", Json::String(linetype.description.clone()))])
    }

    fn block_to_json(block: &Block) -> Result<Json, String> {
        let entities = block.entities.iter().map(entity_to_json).collect::<Result<Vec<_>, String>>()?;
        Ok(obj(vec![("name", Json::String(block.name.clone())), ("basePoint", point_json(&block.base_point)), ("entities", Json::Array(entities))]))
    }

    fn block_projection(block: &Block) -> Json {
        block_to_json(block).unwrap_or_else(|_| obj(vec![("name", Json::String(block.name.clone())), ("basePoint", point_json(&block.base_point)), ("entities", Json::Array(vec![]))]))
    }

    pub fn project_dxf_r12(bytes: &[u8]) -> Result<Json, String> {
        let drawing = load(bytes)?;
        let layers: Vec<Json> = drawing.layers().map(layer_to_json).collect();
        let styles: Vec<Json> = drawing.styles().map(style_to_json).collect();
        let linetypes: Vec<Json> = drawing.line_types().map(linetype_to_json).collect();
        let blocks: Vec<Json> = drawing.blocks().map(block_projection).collect();
        let entities: Vec<Json> = drawing.entities().map(entity_projection).collect();
        Ok(obj(vec![
            ("acadVersion", Json::String(format!("{:?}", drawing.header.version))),
            ("insertionBase", point_json(&drawing.header.insertion_base)),
            ("layers", Json::Array(layers)),
            ("styles", Json::Array(styles)),
            ("linetypes", Json::Array(linetypes)),
            ("blocks", Json::Array(blocks)),
            ("entities", Json::Array(entities)),
        ]))
    }
}

#[cfg(feature = "oracles")]
pub use reader::project_dxf_r12;

/// 🚫️ Refuses an unlinked reference implementation using the original feature contract.
#[cfg(not(feature = "oracles"))]
pub fn project_dxf_r12(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️semantic/🦀️.rs"]
mod semantic_tests;
