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
//! JSON mutation-spec shape (this module's own, decoded independently by the subject adapter too):
//! `{"kind": "<kebab-case-kind>", "params": {...}}`. `params` field names mirror this subset's own
//! `DxfMutation`/`DxfEntity` field names (`index`, `name`, `layer`, `color`, `linetype`, `font`,
//! `description`, `entityKind`/`layer`/`center`/`radius`/`start`/`end`/`startAngle`/`endAngle`/
//! `position`/`height`/`value`/`points`/`blockName`, `basePoint`, `entities`).
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
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
/// `semantic-dxf-r12-v1` comparison profile declared in `../🔣️oracle.json`.
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
    //#endregion 🔖️JsonHelpers

    //#region 🔖️EntityCodec
    /// 📥️ JSON → `dxf::entities::Entity`, over the six typed kinds this subset itself models
    /// (line/circle/arc/text/solid/insert) — the same set `DxfEntity` declares, `Other` excepted (raw
    /// retention has no meaningful independent-library construction).
    fn build_entity(spec: &Json) -> Result<Entity, String> {
        let specific = match spec.str("entityKind").as_str() {
            "line" => EntityType::Line(Line { p1: point_from(spec, "start"), p2: point_from(spec, "end"), ..Default::default() }),
            "circle" => EntityType::Circle(Circle { center: point_from(spec, "center"), radius: number(spec, "radius"), ..Default::default() }),
            "arc" => EntityType::Arc(DxfArc { center: point_from(spec, "center"), radius: number(spec, "radius"), start_angle: number(spec, "startAngle"), end_angle: number(spec, "endAngle"), ..Default::default() }),
            "text" => EntityType::Text(Text { location: point_from(spec, "position"), text_height: number(spec, "height"), value: spec.str("value"), text_style_name: "STANDARD".to_string(), ..Default::default() }),
            "solid" => {
                let points = spec.array("points");
                let corner = |i: usize| point_of(points.get(i).unwrap_or(&Json::Null));
                EntityType::Solid(Solid { first_corner: corner(0), second_corner: corner(1), third_corner: corner(2), fourth_corner: corner(3), ..Default::default() })
            }
            "insert" => EntityType::Insert(Insert { name: spec.str("blockName"), location: point_from(spec, "position"), ..Default::default() }),
            other => return Err(format!("dxf oracle: unsupported entityKind {other:?}")),
        };
        let mut entity = Entity::new(specific);
        entity.common.layer = spec.str("layer");
        Ok(entity)
    }

    /// 📤️ `dxf::entities::Entity` → JSON, the exact inverse of `build_entity` — used to capture an
    /// entity's pre-mutation value for `insert-entity`/`set-entity`'s own inverses, and recursively
    /// for a block's nested entity list.
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
    fn layer_to_json(layer: &Layer) -> Json {
        obj(vec![("name", Json::String(layer.name.clone())), ("color", Json::Number(layer.color.index().unwrap_or(7) as f64)), ("linetype", Json::String(layer.line_type_name.clone()))])
    }

    fn build_style(spec: &Json) -> Style {
        Style { name: spec.str("name"), primary_font_file_name: spec.str("font"), text_height: 2.5, ..Default::default() }
    }
    fn style_to_json(style: &Style) -> Json {
        obj(vec![("name", Json::String(style.name.clone())), ("font", Json::String(style.primary_font_file_name.clone()))])
    }

    fn build_linetype(spec: &Json) -> LineType {
        LineType { name: spec.str("name"), description: spec.str("description"), ..Default::default() }
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
            "no-mutation" => Ok(()),

            "set-snapshot" => {
                drawing.header.insertion_base = point_from(params, "insertionBase");
                if let Some(Json::Array(layers)) = params.get("layers") {
                    let layers = layers.clone();
                    while drawing.remove_layer(0).is_some() {}
                    for l in &layers {
                        drawing.add_layer(build_layer(l));
                    }
                }
                if let Some(Json::Array(entities)) = params.get("entities") {
                    let entities = entities.clone();
                    while drawing.remove_entity(0).is_some() {}
                    for e in &entities {
                        drawing.add_entity(build_entity(e)?);
                    }
                }
                Ok(())
            }

            "set-header-var" => match params.str("name").as_str() {
                "$INSBASE" => {
                    drawing.header.insertion_base = point_from(params, "value");
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
                insert_layer_at(drawing, index_of(params, "index"), build_layer(params));
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
                let replacement = build_layer(params);
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
                insert_style_at(drawing, index_of(params, "index"), build_style(params));
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
                let font = params.str("font");
                match drawing.styles_mut().find(|s| s.name == name) {
                    Some(slot) => {
                        slot.primary_font_file_name = font;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-style target {name:?} not found")),
                }
            }

            "insert-linetype" => {
                insert_linetype_at(drawing, index_of(params, "index"), build_linetype(params));
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
                let description = params.str("description");
                match drawing.line_types_mut().find(|l| l.name == name) {
                    Some(slot) => {
                        slot.description = description;
                        Ok(())
                    }
                    None => Err(format!("dxf oracle: set-linetype target {name:?} not found")),
                }
            }

            "insert-entity" => {
                let entity = build_entity(params)?;
                insert_entity_at(drawing, index_of(params, "index"), entity);
                Ok(())
            }
            "remove-entity" => {
                drawing.remove_entity(index_of(params, "index"));
                Ok(())
            }
            "set-entity" => {
                let index = index_of(params, "index");
                let replacement = build_entity(params)?;
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
                let block = build_block(params)?;
                insert_block_at(drawing, index_of(params, "index"), block);
                Ok(())
            }
            "remove-block" => {
                drawing.remove_block(index_of(params, "index"));
                Ok(())
            }
            "set-block" => {
                let index = index_of(params, "index");
                let replacement = build_block(params)?;
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

    //#region 🔖️Inverse
    /// ↩️ `DxfMutation::inverse`'s own per-variant contract, transplanted onto `dxf::Drawing`: reads
    /// whatever pre-state it needs from `base` (name/index-aware), returning the `(kind, params)`
    /// pair that undoes `(kind, params)` when applied on top of
    /// `apply_kind(&mut base.clone(), kind, params)`.
    fn inverse_of(base: &Drawing, kind: &str, params: &Json) -> Result<(String, Json), String> {
        let no_op = ("no-mutation".to_string(), Json::Object(vec![]));
        match kind {
            "no-mutation" => Ok(no_op),

            "set-snapshot" => {
                let layers: Vec<Json> = base.layers().map(layer_to_json).collect();
                let entities: Vec<Json> = base.entities().map(entity_projection).collect();
                Ok(("set-snapshot".to_string(), obj(vec![("insertionBase", point_json(&base.header.insertion_base)), ("layers", Json::Array(layers)), ("entities", Json::Array(entities))])))
            }

            "set-header-var" | "remove-header-var" => {
                let name = params.str("name");
                match name.as_str() {
                    "$INSBASE" => Ok(("set-header-var".to_string(), obj(vec![("name", Json::String(name)), ("value", point_json(&base.header.insertion_base))]))),
                    other => Err(format!("dxf oracle: unsupported header var {other:?}")),
                }
            }

            "insert-layer" => Ok(("remove-layer".to_string(), obj(vec![("name", Json::String(params.str("name")))]))),
            "remove-layer" => {
                let name = params.str("name");
                match base.layers().position(|l| l.name == name) {
                    Some(at) => {
                        let mut fields = as_fields(layer_to_json(base.layers().nth(at).expect("position valid")));
                        fields.push(("index".to_string(), Json::Number(at as f64)));
                        Ok(("insert-layer".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }
            "set-layer" => {
                let name = params.str("name");
                match base.layers().find(|l| l.name == name) {
                    Some(layer) => Ok(("set-layer".to_string(), layer_to_json(layer))),
                    None => Ok(("remove-layer".to_string(), obj(vec![("name", Json::String(name))]))),
                }
            }

            "insert-style" => Ok(("remove-style".to_string(), obj(vec![("name", Json::String(params.str("name")))]))),
            "remove-style" => {
                let name = params.str("name");
                match base.styles().position(|s| s.name == name) {
                    Some(at) => {
                        let mut fields = as_fields(style_to_json(base.styles().nth(at).expect("position valid")));
                        fields.push(("index".to_string(), Json::Number(at as f64)));
                        Ok(("insert-style".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }
            "set-style" => {
                let name = params.str("name");
                match base.styles().find(|s| s.name == name) {
                    Some(style) => Ok(("set-style".to_string(), style_to_json(style))),
                    None => Ok(("remove-style".to_string(), obj(vec![("name", Json::String(name))]))),
                }
            }

            "insert-linetype" => Ok(("remove-linetype".to_string(), obj(vec![("name", Json::String(params.str("name")))]))),
            "remove-linetype" => {
                let name = params.str("name");
                match base.line_types().position(|l| l.name == name) {
                    Some(at) => {
                        let mut fields = as_fields(linetype_to_json(base.line_types().nth(at).expect("position valid")));
                        fields.push(("index".to_string(), Json::Number(at as f64)));
                        Ok(("insert-linetype".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }
            "set-linetype" => {
                let name = params.str("name");
                match base.line_types().find(|l| l.name == name) {
                    Some(linetype) => Ok(("set-linetype".to_string(), linetype_to_json(linetype))),
                    None => Ok(("remove-linetype".to_string(), obj(vec![("name", Json::String(name))]))),
                }
            }

            "insert-entity" => Ok(("remove-entity".to_string(), obj(vec![("index", Json::Number(index_of(params, "index") as f64))]))),
            "remove-entity" => {
                let index = index_of(params, "index");
                match base.entities().nth(index) {
                    Some(entity) => {
                        let mut fields = as_fields(entity_to_json(entity)?);
                        fields.push(("index".to_string(), Json::Number(index as f64)));
                        Ok(("insert-entity".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }
            "set-entity" => {
                let index = index_of(params, "index");
                match base.entities().nth(index) {
                    Some(entity) => {
                        let mut fields = as_fields(entity_to_json(entity)?);
                        fields.push(("index".to_string(), Json::Number(index as f64)));
                        Ok(("set-entity".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }

            "insert-block" => Ok(("remove-block".to_string(), obj(vec![("index", Json::Number(index_of(params, "index") as f64))]))),
            "remove-block" => {
                let index = index_of(params, "index");
                match base.blocks().nth(index) {
                    Some(block) => {
                        let mut fields = as_fields(block_to_json(block)?);
                        fields.push(("index".to_string(), Json::Number(index as f64)));
                        Ok(("insert-block".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }
            "set-block" => {
                let index = index_of(params, "index");
                match base.blocks().nth(index) {
                    Some(block) => {
                        let mut fields = as_fields(block_to_json(block)?);
                        fields.push(("index".to_string(), Json::Number(index as f64)));
                        Ok(("set-block".to_string(), Json::Object(fields)))
                    }
                    None => Ok(no_op),
                }
            }

            other => Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        }
    }

    fn as_fields(value: Json) -> Vec<(String, Json)> {
        match value {
            Json::Object(fields) => fields,
            _ => vec![],
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

    pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let base = load(input)?;
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let (inverse_kind, inverse_params) = inverse_of(&base, &kind, &params)?;
        let mut drawing = base;
        apply_kind(&mut drawing, &kind, &params)?;
        apply_kind(&mut drawing, &inverse_kind, &inverse_params)?;
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
mod smoke_tests {
    use super::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_dxf_r12};
    use semio_repo_test_host::parse_json;

    const FIXTURE: &str = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf";

    const ROWS: &[(&str, &str)] = &[
        ("no-mutation", "{}"),
        ("set-snapshot", r#"{"insertionBase": [5, 5, 0], "layers": [{"name": "0", "color": 7, "linetype": "CONTINUOUS"}], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 42}]}"#),
        ("set-header-var", r#"{"name": "$INSBASE", "value": [15, 25, 0]}"#),
        ("remove-header-var", r#"{"name": "$INSBASE"}"#),
        ("insert-layer", r#"{"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}"#),
        ("remove-layer", r#"{"name": "DIMS"}"#),
        ("set-layer", r#"{"name": "DIMS", "color": 4, "linetype": "DASHED"}"#),
        ("insert-style", r#"{"index": 1, "name": "LABELS", "font": "arial.ttf"}"#),
        ("remove-style", r#"{"name": "NOTES"}"#),
        ("set-style", r#"{"name": "NOTES", "font": "romans.shx"}"#),
        ("insert-linetype", r#"{"index": 1, "name": "CENTER", "description": "Center line"}"#),
        ("remove-linetype", r#"{"name": "DASHED"}"#),
        ("set-linetype", r#"{"name": "DASHED", "description": "Dash pattern"}"#),
        ("insert-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}"#),
        ("remove-entity", r#"{"index": 3}"#),
        ("set-entity", r#"{"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}"#),
        ("insert-block", r#"{"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]}"#),
        ("remove-block", r#"{"index": 1}"#),
        ("set-block", r#"{"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]}"#),
    ];

    #[test]
    fn all_kinds_mutate_and_invert_cleanly() {
        assert_eq!(ROWS.len(), 19, "must exercise all 19 declared kinds");
        let input = std::fs::read(FIXTURE).expect("read committed fixture");
        let base_projection = project_dxf_r12(&input).expect("project base fixture");

        for (kind, params) in ROWS {
            let spec_text = format!(r#"{{"kind": "{kind}", "params": {params}}}"#);
            let spec = parse_json(&spec_text).unwrap_or_else(|e| panic!("bad spec JSON for {kind}: {e}"));

            let mutated = oracle_apply_mutation(&input, &spec).unwrap_or_else(|e| panic!("mutate {kind} failed: {e}"));
            assert!(!mutated.is_empty(), "mutate {kind} produced empty bytes");
            let mutated_projection = project_dxf_r12(&mutated).unwrap_or_else(|e| panic!("project mutate {kind} output failed: {e}"));
            if *kind != "no-mutation" {
                assert_ne!(mutated_projection, base_projection, "mutate {kind} produced no semantic change");
            }

            let inverted = oracle_apply_mutation_inverse(&input, &spec).unwrap_or_else(|e| panic!("inverse {kind} failed: {e}"));
            let inverted_projection = project_dxf_r12(&inverted).unwrap_or_else(|e| panic!("project inverse {kind} output failed: {e}"));
            assert_eq!(inverted_projection, base_projection, "inverse {kind} did not restore the base projection");
        }
    }

    #[test]
    fn identity_round_trip_is_not_byte_identical() {
        let input = std::fs::read(FIXTURE).expect("read committed fixture");
        let spec = parse_json(r#"{"kind": "no-mutation", "params": {}}"#).expect("valid spec");
        let output = oracle_apply_mutation(&input, &spec).expect("no-mutation re-encode");
        assert_ne!(output, input, "byte pass-through: dxf-crate re-encode is bit-identical to the input");
        let base_projection = project_dxf_r12(&input).expect("project input");
        let output_projection = project_dxf_r12(&output).expect("project output");
        assert_eq!(base_projection, output_projection, "no-mutation re-encode changed the semantic projection");
    }
}
//#endregion 🔖️SmokeTests
