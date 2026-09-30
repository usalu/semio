//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `dxf` 0.6 reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading. `dxf` reads AND
//! writes DXF (unlike a reader-only reference), so it is a genuine differential second producer, not
//! merely an independent projector.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! JSON mutation-spec shape: `{"kind": "<kebab-case-kind>", "params": {...}}`, where `params` IS the leaf wire payload
//! (design §11) — `{index, layer: DxfLayer}`, `{name, style: DxfStyle}`, `{index, entity: DxfEntity}` (externally tagged,
//! `{"circle": {center, radius, layer}}`), `{index, block: DxfBlock}`, `{name, headerVar: DxfHeaderVar}` and
//! `{snapshot: DxfSnapshot}` — read here into `dxf`'s own typed model, independently of this subset's codec.
//!
//! @see ./🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    imp::oracle_apply_mutation(input, spec)
}

/// 🔁️ The identity round trip's own producer: `dxf` parses the document and re-serializes it from its own typed
/// `Drawing` alone.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    imp::oracle_round_trip(input)
}

/// 🔁️ Applies the mutation, then applies its own computed inverse (built from the PRE-mutation
/// state, name/index-aware — mirroring `DxfMutation::inverse`'s own contract) and re-serializes.
/// `apply(inverse(m), apply(m, base)) == base` by the law, so this is the oracle's own independent
/// exercise of that law, not a comparison of the implementation with itself: the forward mutation,
/// the inverse computation and the two applications are all performed by `dxf`, never by this
/// subset's own codec.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    imp::oracle_apply_mutation_inverse(input, spec)
}

/// 📄️ Independent semantic projection of a DXF R12 document, read back by `dxf` itself (never by
/// this subset's own codec) — used to compare the oracle's and the subject's results under the
/// `semantic-dxf-r12-v1` comparison profile declared in `./🔣️.json`.
#[cfg(feature = "oracles")]
pub fn project_dxf_r12(bytes: &[u8]) -> Result<Json, String> {
    imp::project_dxf_r12(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
#[cfg(not(feature = "oracles"))]
pub fn project_dxf_r12(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

/// 🔒️ Every `dxf`-linked helper lives inside this ONE `cfg`-gated module, so the non-`oracles` build
/// never even parses a `dxf::` path — a single `cfg` at the module boundary instead of one on every
/// function.
#[cfg(feature = "oracles")]
mod imp {
    use super::Json;
    use dxf::entities::{Arc as DxfArc, Circle, Entity, EntityType, Insert, Line, Solid, Text};
    use dxf::tables::{Layer, LineType, Style};
    use dxf::enums::AcadVersion;
    use dxf::{Block, Color, Drawing, Point};

    //#region 🔖️LoadSave
    /// 📥️ `dxf` fully parses the ASCII group-code stream into its own typed `Drawing` — never a
    /// byte-level read of this subset's own model.
    fn load(bytes: &[u8]) -> Result<Drawing, String> {
        Drawing::load(&mut &bytes[..]).map_err(|error| format!("dxf oracle: load failed: {error:?}"))
    }

    /// 📤️ Re-serializes from `dxf`'s own typed model alone.
    fn save(drawing: &Drawing) -> Result<Vec<u8>, String> {
        let mut out: Vec<u8> = Vec::new();
        drawing.save(&mut out).map_err(|error| format!("dxf oracle: save failed: {error:?}"))?;
        Ok(out)
    }
    //#endregion 🔖️LoadSave

    //#region 🔖️JsonHelpers
    fn number(v: &Json, key: &str) -> f64 {
        match v.get(key) {
            Some(Json::Number(n)) => *n,
            _ => 0.0,
        }
    }
    fn index_of(v: &Json, key: &str) -> usize {
        number(v, key).max(0.0) as usize
    }
    fn coord(arr: &Json, i: usize) -> f64 {
        match arr {
            Json::Array(items) => match items.get(i) {
                Some(Json::Number(n)) => *n,
                _ => 0.0,
            },
            _ => 0.0,
        }
    }
    fn point_of(arr: &Json) -> Point {
        Point::new(coord(arr, 0), coord(arr, 1), coord(arr, 2))
    }
    fn point_from(v: &Json, key: &str) -> Point {
        point_of(v.get(key).unwrap_or(&Json::Null))
    }
    fn point_json(p: &Point) -> Json {
        Json::Array(vec![Json::Number(p.x), Json::Number(p.y), Json::Number(p.z)])
    }
    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
    fn member(v: &Json, key: &str) -> Result<Json, String> {
        v.get(key).cloned().ok_or_else(|| format!("dxf oracle: params carry no `{key}`"))
    }
    //#endregion 🔖️JsonHelpers

    //#region 🔖️EntityCodec
    /// 📥️ The externally tagged `DxfEntity` wire (`{"circle": {center, radius, layer}}`) → `dxf::entities::Entity`, over
    /// the six typed kinds this subset itself models (line/circle/arc/text/solid/insert) — `Other` excepted (raw
    /// retention has no meaningful independent-library construction).
    fn build_entity(wire: &Json) -> Result<Entity, String> {
        let (tag, spec) = match wire {
            Json::Object(fields) if fields.len() == 1 => (fields[0].0.as_str(), &fields[0].1),
            other => return Err(format!("dxf oracle: a DxfEntity is a one-member tagged object, found {other:?}")),
        };
        let specific = match tag {
            "line" => EntityType::Line(Line { p1: point_from(spec, "start"), p2: point_from(spec, "end"), ..Default::default() }),
            "circle" => EntityType::Circle(Circle { center: point_from(spec, "center"), radius: number(spec, "radius"), ..Default::default() }),
            "arc" => EntityType::Arc(DxfArc { center: point_from(spec, "center"), radius: number(spec, "radius"), start_angle: number(spec, "startAngle"), end_angle: number(spec, "endAngle"), ..Default::default() }),
            "text" => EntityType::Text(Text { location: point_from(spec, "position"), text_height: number(spec, "height"), value: spec.str("value"), text_style_name: "STANDARD".to_string(), ..Default::default() }),
            "solid" => {
                let points = spec.array("points");
                let corner = |i: usize| point_of(points.get(i).unwrap_or(&Json::Null));
                EntityType::Solid(Solid { first_corner: corner(0), second_corner: corner(1), third_corner: corner(2), fourth_corner: corner(3), ..Default::default() })
            }
            "insert" => {
                let scale = spec.get("scale").cloned().unwrap_or(Json::Null);
                EntityType::Insert(Insert { name: spec.str("blockName"), location: point_from(spec, "position"), x_scale_factor: coord(&scale, 0), y_scale_factor: coord(&scale, 1), z_scale_factor: coord(&scale, 2), rotation: number(spec, "rotation"), ..Default::default() })
            }
            other => return Err(format!("dxf oracle: unsupported DxfEntity tag {other:?}")),
        };
        let mut entity = Entity::new(specific);
        entity.common.layer = spec.str("layer");
        Ok(entity)
    }

    /// 📤️ `dxf::entities::Entity` → the externally tagged `DxfEntity` wire, the exact inverse of [`build_entity`] — how an
    /// entity's pre-mutation value travels in an inverse spec.
    fn entity_wire(entity: &Entity) -> Result<Json, String> {
        let (tag, mut fields): (&str, Vec<(&str, Json)>) = match &entity.specific {
            EntityType::Line(l) => ("line", vec![("start", point_json(&l.p1)), ("end", point_json(&l.p2))]),
            EntityType::Circle(c) => ("circle", vec![("center", point_json(&c.center)), ("radius", Json::Number(c.radius))]),
            EntityType::Arc(a) => ("arc", vec![("center", point_json(&a.center)), ("radius", Json::Number(a.radius)), ("startAngle", Json::Number(a.start_angle)), ("endAngle", Json::Number(a.end_angle))]),
            EntityType::Text(t) => ("text", vec![("position", point_json(&t.location)), ("height", Json::Number(t.text_height)), ("value", Json::String(t.value.clone()))]),
            EntityType::Solid(s) => ("solid", vec![("points", Json::Array(vec![point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)]))]),
            EntityType::Insert(i) => ("insert", vec![("blockName", Json::String(i.name.clone())), ("position", point_json(&i.location)), ("scale", Json::Array(vec![Json::Number(i.x_scale_factor), Json::Number(i.y_scale_factor), Json::Number(i.z_scale_factor)])), ("rotation", Json::Number(i.rotation))]),
            other => return Err(format!("dxf oracle: cannot capture inverse value for unsupported entity kind {other:?}")),
        };
        fields.push(("layer", Json::String(entity.common.layer.clone())));
        Ok(obj(vec![(tag, obj(fields))]))
    }

    /// 📄️ `dxf::entities::Entity` → the flat `semantic-dxf-r12-v1` projection shape (`entityKind` plus its fields), and
    /// recursively for a block's nested entity list.
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

    /// 📄️ Semantic projection of one entity, for `project_dxf_r12`.
    fn entity_projection(entity: &Entity) -> Json {
        entity_to_json(entity).unwrap_or_else(|_| obj(vec![("entityKind", Json::String("other".to_string())), ("layer", Json::String(entity.common.layer.clone()))]))
    }
    //#endregion 🔖️EntityCodec

    //#region 🔖️TableCodecs
    fn build_layer(spec: &Json) -> Layer {
        Layer { name: spec.str("name"), color: Color::from_index(number(spec, "color").max(0.0) as u8), line_type_name: spec.str("linetype"), ..Default::default() }
    }
    fn layer_wire(layer: &Layer) -> Json {
        obj(vec![("name", Json::String(layer.name.clone())), ("color", Json::Number(layer.color.index().unwrap_or(7) as f64)), ("linetype", Json::String(layer.line_type_name.clone())), ("flags", Json::Number(0.0))])
    }
    fn layer_to_json(layer: &Layer) -> Json {
        obj(vec![("name", Json::String(layer.name.clone())), ("color", Json::Number(layer.color.index().unwrap_or(7) as f64)), ("linetype", Json::String(layer.line_type_name.clone()))])
    }

    fn build_style(spec: &Json) -> Style {
        Style { name: spec.str("name"), primary_font_file_name: spec.str("fontName"), text_height: 2.5, ..Default::default() }
    }
    fn style_wire(style: &Style) -> Json {
        obj(vec![("name", Json::String(style.name.clone())), ("flags", Json::Number(0.0)), ("fontName", Json::String(style.primary_font_file_name.clone()))])
    }
    fn style_to_json(style: &Style) -> Json {
        obj(vec![("name", Json::String(style.name.clone())), ("font", Json::String(style.primary_font_file_name.clone()))])
    }

    fn build_linetype(spec: &Json) -> LineType {
        LineType { name: spec.str("name"), description: spec.str("description"), ..Default::default() }
    }
    fn linetype_wire(linetype: &LineType) -> Json {
        obj(vec![("name", Json::String(linetype.name.clone())), ("flags", Json::Number(0.0)), ("description", Json::String(linetype.description.clone()))])
    }
    fn linetype_to_json(linetype: &LineType) -> Json {
        obj(vec![("name", Json::String(linetype.name.clone())), ("description", Json::String(linetype.description.clone()))])
    }

    fn build_block(spec: &Json) -> Result<Block, String> {
        let entities = spec.array("entities").iter().map(build_entity).collect::<Result<Vec<_>, String>>()?;
        Ok(Block { name: spec.str("name"), layer: "0".to_string(), base_point: point_from(spec, "basePoint"), entities, ..Default::default() })
    }
    fn block_to_json(block: &Block) -> Result<Json, String> {
        let entities = block.entities.iter().map(entity_to_json).collect::<Result<Vec<_>, String>>()?;
        Ok(obj(vec![("name", Json::String(block.name.clone())), ("basePoint", point_json(&block.base_point)), ("entities", Json::Array(entities))]))
    }
    fn block_wire(block: &Block) -> Result<Json, String> {
        let entities = block.entities.iter().map(entity_wire).collect::<Result<Vec<_>, String>>()?;
        Ok(obj(vec![("name", Json::String(block.name.clone())), ("basePoint", point_json(&block.base_point)), ("entities", Json::Array(entities))]))
    }
    fn block_projection(block: &Block) -> Json {
        block_to_json(block).unwrap_or_else(|_| obj(vec![("name", Json::String(block.name.clone())), ("basePoint", point_json(&block.base_point)), ("entities", Json::Array(vec![]))]))
    }
    //#endregion 🔖️TableCodecs

    //#region 🔖️HeaderVar
    /// 🏷️ `$INSBASE` is the one generic `$VAR` this oracle mutates directly — `dxf`'s `Header` is a
    /// fixed typed struct (no arbitrary `$VAR` insertion), so `set-header-var`/`remove-header-var`
    /// are exercised against a header point every DXF R12 file actually persists. `$INSUNITS` was
    /// tried first and rejected: `dxf`'s own generated `Header::add_code_pairs` only emits it for
    /// `version >= AcadVersion::R2000` (confirmed against `target/.../out/generated/header.rs`), so
    /// it never survives a save/reload of an R12 document at all — not a representable R12 mutation
    /// target through this reference library, regardless of what this module set in memory.
    /// `$INSBASE` (`header.insertion_base`) has no such gate — written unconditionally every save.
    //#endregion 🔖️HeaderVar

    //#region 🔖️OrderedRebuild
    /// 🧱️ `dxf::Drawing::add_*` only appends; a true insert-at-`index` needs the whole ordered
    /// collection rebuilt. Repeated per collection kind since `Drawing` exposes no shared trait over
    /// its five ordered tables.
    fn insert_layer_at(drawing: &mut Drawing, index: usize, layer: Layer) {
        let mut items: Vec<Layer> = drawing.layers().cloned().collect();
        items.insert(index.min(items.len()), layer);
        while drawing.remove_layer(0).is_some() {}
        for item in items {
            drawing.add_layer(item);
        }
    }
    fn insert_style_at(drawing: &mut Drawing, index: usize, style: Style) {
        let mut items: Vec<Style> = drawing.styles().cloned().collect();
        items.insert(index.min(items.len()), style);
        while drawing.remove_style(0).is_some() {}
        for item in items {
            drawing.add_style(item);
        }
    }
    fn insert_linetype_at(drawing: &mut Drawing, index: usize, linetype: LineType) {
        let mut items: Vec<LineType> = drawing.line_types().cloned().collect();
        items.insert(index.min(items.len()), linetype);
        while drawing.remove_line_type(0).is_some() {}
        for item in items {
            drawing.add_line_type(item);
        }
    }
    fn insert_block_at(drawing: &mut Drawing, index: usize, block: Block) {
        let mut items: Vec<Block> = drawing.blocks().cloned().collect();
        items.insert(index.min(items.len()), block);
        while drawing.remove_block(0).is_some() {}
        for item in items {
            drawing.add_block(item);
        }
    }
    fn insert_entity_at(drawing: &mut Drawing, index: usize, entity: Entity) {
        let mut items: Vec<Entity> = drawing.entities().cloned().collect();
        items.insert(index.min(items.len()), entity);
        while drawing.remove_entity(0).is_some() {}
        for item in items {
            drawing.add_entity(item);
        }
    }
    //#endregion 🔖️OrderedRebuild

    //#region 🔖️Apply
    /// ▶️ Performs one mutation kind against `drawing` in place — the forward half both
    /// `oracle_apply_mutation` and `oracle_apply_mutation_inverse` share (the latter calls it twice:
    /// the mutation, then its own computed inverse).
    fn apply_kind(drawing: &mut Drawing, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "set-snapshot" => {
                *drawing = snapshot_drawing(&member(params, "snapshot")?)?;
                Ok(())
            }

            "set-header-var" => match params.str("name").as_str() {
                "$INSBASE" => {
                    drawing.header.insertion_base = header_point(&member(params, "headerVar")?)?;
                    Ok(())
                }
                other => Err(format!("dxf oracle: unsupported header var {other:?}")),
            },
            "remove-header-var" => match params.str("name").as_str() {
                "$INSBASE" => {
                    drawing.header.insertion_base = Point::origin();
                    Ok(())
                }
                other => Err(format!("dxf oracle: unsupported header var {other:?}")),
            },

            "insert-layer" => {
                insert_layer_at(drawing, index_of(params, "index"), build_layer(&member(params, "layer")?));
                Ok(())
            }
            "remove-layer" => {
                let name = params.str("name");
                let at: Option<usize> = drawing.layers().position(|l| l.name == name);
                if let Some(at) = at {
                    drawing.remove_layer(at);
                }
                Ok(())
            }
            "set-layer" => {
                let name = params.str("name");
                let replacement = build_layer(&member(params, "layer")?);
                match drawing.layers_mut().find(|l| l.name == name) {
                    Some(slot) => {
                        slot.color = replacement.color;
                        slot.line_type_name = replacement.line_type_name;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-layer target {name:?} not found")),
                }
            }

            "insert-style" => {
                insert_style_at(drawing, index_of(params, "index"), build_style(&member(params, "style")?));
                Ok(())
            }
            "remove-style" => {
                let name = params.str("name");
                let at: Option<usize> = drawing.styles().position(|s| s.name == name);
                if let Some(at) = at {
                    drawing.remove_style(at);
                }
                Ok(())
            }
            "set-style" => {
                let name = params.str("name");
                let font = member(params, "style")?.str("fontName");
                match drawing.styles_mut().find(|s| s.name == name) {
                    Some(slot) => {
                        slot.primary_font_file_name = font;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-style target {name:?} not found")),
                }
            }

            "insert-linetype" => {
                insert_linetype_at(drawing, index_of(params, "index"), build_linetype(&member(params, "linetype")?));
                Ok(())
            }
            "remove-linetype" => {
                let name = params.str("name");
                let at: Option<usize> = drawing.line_types().position(|l| l.name == name);
                if let Some(at) = at {
                    drawing.remove_line_type(at);
                }
                Ok(())
            }
            "set-linetype" => {
                let name = params.str("name");
                let description = member(params, "linetype")?.str("description");
                match drawing.line_types_mut().find(|l| l.name == name) {
                    Some(slot) => {
                        slot.description = description;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-linetype target {name:?} not found")),
                }
            }

            "insert-entity" => {
                let entity = build_entity(&member(params, "entity")?)?;
                insert_entity_at(drawing, index_of(params, "index"), entity);
                Ok(())
            }
            "remove-entity" => {
                drawing.remove_entity(index_of(params, "index"));
                Ok(())
            }
            "set-entity" => {
                let index = index_of(params, "index");
                let replacement = build_entity(&member(params, "entity")?)?;
                match drawing.entities_mut().nth(index) {
                    Some(slot) => {
                        slot.specific = replacement.specific;
                        slot.common.layer = replacement.common.layer;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-entity target index {index} not found")),
                }
            }

            "insert-block" => {
                let block = build_block(&member(params, "block")?)?;
                insert_block_at(drawing, index_of(params, "index"), block);
                Ok(())
            }
            "remove-block" => {
                drawing.remove_block(index_of(params, "index"));
                Ok(())
            }
            "set-block" => {
                let index = index_of(params, "index");
                let replacement = build_block(&member(params, "block")?)?;
                match drawing.blocks_mut().nth(index) {
                    Some(slot) => {
                        slot.base_point = replacement.base_point;
                        slot.entities = replacement.entities;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-block target index {index} not found")),
                }
            }

            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Apply

    //#region 🔖️SnapshotWire
    /// 🏷️ The point a `$INSBASE` `DxfHeaderVar` wire carries (`value` is the `{"kind": "point", "value": [x, y, z]}` DxfValue).
    fn header_point(header_var: &Json) -> Result<Point, String> {
        let value = member(header_var, "value")?;
        match value.str("kind").as_str() {
            "point" => Ok(point_from(&value, "value")),
            other => Err(format!("dxf oracle: `{}` must carry a point, found a {other:?} value", header_var.str("name"))),
        }
    }

    /// 📸️ A whole new drawing built by `dxf` from the `DxfSnapshot` wire alone — `set-snapshot` replaces the document, so
    /// nothing of the input survives. `Drawing::new()`'s ensured default table entries are dropped so the tables hold exactly
    /// what the snapshot declares; the header honours `$ACADVER` and `$INSBASE`, the two variables this reference models.
    fn snapshot_drawing(snapshot: &Json) -> Result<Drawing, String> {
        let mut drawing = Drawing::new();
        for header_var in snapshot.array("headerVars") {
            match header_var.str("name").as_str() {
                "$ACADVER" => drawing.header.version = AcadVersion::from(member(&header_var, "value")?.str("value")).map_err(|error| format!("dxf oracle: $ACADVER: {error:?}"))?,
                "$INSBASE" => drawing.header.insertion_base = header_point(&header_var)?,
                other => return Err(format!("dxf oracle: unsupported header var {other:?}")),
            }
        }
        while drawing.remove_layer(0).is_some() {}
        while drawing.remove_style(0).is_some() {}
        while drawing.remove_line_type(0).is_some() {}
        let tables = snapshot.get("tables").cloned().unwrap_or(Json::Null);
        for layer in tables.array("layers") {
            drawing.add_layer(build_layer(&layer));
        }
        for style in tables.array("styles") {
            drawing.add_style(build_style(&style));
        }
        for linetype in tables.array("linetypes") {
            drawing.add_line_type(build_linetype(&linetype));
        }
        for block in snapshot.array("blocks") {
            drawing.add_block(build_block(&block)?);
        }
        for entity in snapshot.array("entities") {
            drawing.add_entity(build_entity(&entity)?);
        }
        Ok(drawing)
    }
    //#endregion 🔖️SnapshotWire

    //#region 🔖️Inverse
    /// ↩️ What undoes a forward `(kind, params)` applied to `base`.
    enum Undo {
        /// 🚫️ Nothing to undo: the forward step had nothing to act on.
        Nothing,
        /// 🔁️ Apply this `(kind, wire params)` on top of the forward result.
        Apply(String, Json),
        /// 📦️ Put the original drawing back — a whole-document replacement is undone by the whole original document, which
        /// `dxf`'s typed `Header` cannot restate as a `DxfSnapshot` wire.
        Original,
    }

    /// ↩️ `DxfMutation::inverse`'s own per-variant contract, transplanted onto `dxf::Drawing`: reads whatever pre-state it
    /// needs from `base` (name/index-aware) and answers the undo as leaf wire params.
    fn inverse_of(base: &Drawing, kind: &str, params: &Json) -> Result<Undo, String> {
        let apply = |kind: &str, params: Json| Ok(Undo::Apply(kind.to_string(), params));
        let name = params.str("name");
        let index = index_of(params, "index");
        let insbase = |name: String| obj(vec![("name", Json::String(name.clone())), ("headerVar", obj(vec![("name", Json::String(name)), ("groupCode", Json::Number(10.0)), ("value", obj(vec![("kind", Json::String("point".to_string())), ("value", point_json(&base.header.insertion_base))]))]))]);
        match kind {
            "set-snapshot" => Ok(Undo::Original),

            "set-header-var" | "remove-header-var" => match name.as_str() {
                "$INSBASE" => apply("set-header-var", insbase(name)),
                other => Err(format!("dxf oracle: unsupported header var {other:?}")),
            },

            "insert-layer" => apply("remove-layer", obj(vec![("name", Json::String(member(params, "layer")?.str("name")))])),
            "remove-layer" => match base.layers().position(|l| l.name == name) {
                Some(at) => apply("insert-layer", obj(vec![("index", Json::Number(at as f64)), ("layer", layer_wire(base.layers().nth(at).expect("position valid")))])),
                None => Ok(Undo::Nothing),
            },
            "set-layer" => match base.layers().find(|l| l.name == name) {
                Some(layer) => apply("set-layer", obj(vec![("name", Json::String(name.clone())), ("layer", layer_wire(layer))])),
                None => apply("remove-layer", obj(vec![("name", Json::String(name))])),
            },

            "insert-style" => apply("remove-style", obj(vec![("name", Json::String(member(params, "style")?.str("name")))])),
            "remove-style" => match base.styles().position(|s| s.name == name) {
                Some(at) => apply("insert-style", obj(vec![("index", Json::Number(at as f64)), ("style", style_wire(base.styles().nth(at).expect("position valid")))])),
                None => Ok(Undo::Nothing),
            },
            "set-style" => match base.styles().find(|s| s.name == name) {
                Some(style) => apply("set-style", obj(vec![("name", Json::String(name.clone())), ("style", style_wire(style))])),
                None => apply("remove-style", obj(vec![("name", Json::String(name))])),
            },

            "insert-linetype" => apply("remove-linetype", obj(vec![("name", Json::String(member(params, "linetype")?.str("name")))])),
            "remove-linetype" => match base.line_types().position(|l| l.name == name) {
                Some(at) => apply("insert-linetype", obj(vec![("index", Json::Number(at as f64)), ("linetype", linetype_wire(base.line_types().nth(at).expect("position valid")))])),
                None => Ok(Undo::Nothing),
            },
            "set-linetype" => match base.line_types().find(|l| l.name == name) {
                Some(linetype) => apply("set-linetype", obj(vec![("name", Json::String(name.clone())), ("linetype", linetype_wire(linetype))])),
                None => apply("remove-linetype", obj(vec![("name", Json::String(name))])),
            },

            "insert-entity" => apply("remove-entity", obj(vec![("index", Json::Number(index as f64))])),
            "remove-entity" | "set-entity" => match base.entities().nth(index) {
                Some(entity) => apply(if kind == "remove-entity" { "insert-entity" } else { "set-entity" }, obj(vec![("index", Json::Number(index as f64)), ("entity", entity_wire(entity)?)])),
                None => Ok(Undo::Nothing),
            },

            "insert-block" => apply("remove-block", obj(vec![("index", Json::Number(index as f64))])),
            "remove-block" | "set-block" => match base.blocks().nth(index) {
                Some(block) => apply(if kind == "remove-block" { "insert-block" } else { "set-block" }, obj(vec![("index", Json::Number(index as f64)), ("block", block_wire(block)?)])),
                None => Ok(Undo::Nothing),
            },

            other => Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Entry
    pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let mut drawing = load(input)?;
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        apply_kind(&mut drawing, &kind, &params)?;
        save(&drawing)
    }

    pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        save(&load(input)?)
    }

    pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let base = load(input)?;
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let undo = inverse_of(&base, &kind, &params)?;
        let mut drawing = load(input)?;
        apply_kind(&mut drawing, &kind, &params)?;
        match undo {
            Undo::Nothing => {}
            Undo::Apply(inverse_kind, inverse_params) => apply_kind(&mut drawing, &inverse_kind, &inverse_params)?,
            Undo::Original => drawing = base,
        }
        save(&drawing)
    }

    /// 📄️ Semantic projection of a DXF R12 document. Handles, owner pointers and any R13+ subclass
    /// marker a writer still emits are excluded: not normative.
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
    //#endregion 🔖️Entry
}

//#region 🔖️SmokeTests
/// 🧪️ Scratch smoke coverage exercising every planned `mutate-<kind>`/`inverse-<kind>` JSON row
/// against the real committed fixture before the feature file locks them in — ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️smoke/🦀️.rs"]
mod smoke_tests;
//#endregion 🔖️SmokeTests
