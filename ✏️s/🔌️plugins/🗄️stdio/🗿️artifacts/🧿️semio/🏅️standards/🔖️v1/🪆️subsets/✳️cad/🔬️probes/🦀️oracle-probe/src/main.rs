//! 🔬️ Third-party readers for `s.stdio.semio@v1/✳️cad`.
//!
//! Every number this binary emits comes out of a third-party library reading a file. `ruststep`
//! parses Part-21 and resolves the entity graph; the `dxf` crate parses ASCII DXF and also WRITES
//! the DXF fixtures. Nothing here predicts what a mutation ought to produce — it reads two artifacts
//! and reports how they differ, which is what keeps the reference external.
//!
//! ruststep is READ-ONLY by measurement, not by choice: 0.4.0 ships no Part-21 writer at all (no AST
//! type implements `Display`, and `ast::ser::to_record` stops at an in-memory `Record` whose nesting
//! disagrees with its own parser). STEP fixtures are therefore emitted here as deterministic Part-21
//! text and then VERIFIED by ruststep's parser before they are ever hashed — honestly `handcrafted`,
//! never claimed as third-party-generated.
//!
//! @see ../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️cad-subset-oracle.md

use dxf::entities::*;
use dxf::{Drawing, Point};
use ruststep::ast::{EntityInstance, Exchange, Parameter, Record};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::str::FromStr;

//#region 🧾️Json
/// 🧾️ The smallest JSON writer that can express a probe report. A dependency would have to be
/// justified in the ledger for something this size, and the shape here is fixed and tiny.
#[derive(Clone, Debug, PartialEq)]
enum Json {
    Bool(bool),
    Num(f64),
    Int(i64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

// 🚫️async: pure formatting helper.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

impl Json {
    // 🚫️async: pure formatting helper.
    fn render(&self, out: &mut String) {
        match self {
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Int(i) => {
                let _ = write!(out, "{i}");
            }
            Json::Num(n) => {
                if n.is_finite() {
                    let _ = write!(out, "{n:?}");
                } else {
                    out.push_str("null");
                }
            }
            Json::Str(s) => {
                let _ = write!(out, "\"{}\"", escape(s));
            }
            Json::Arr(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    item.render(out);
                }
                out.push(']');
            }
            Json::Obj(fields) => {
                out.push('{');
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    let _ = write!(out, "\"{}\":", escape(k));
                    v.render(out);
                }
                out.push('}');
            }
        }
    }

    // 🚫️async: pure formatting helper.
    fn to_text(&self) -> String {
        let mut out = String::new();
        self.render(&mut out);
        out
    }
}

// 🚫️async: pure constructor helper.
fn obj(fields: Vec<(&str, Json)>) -> Json {
    Json::Obj(fields.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
// 🚫️async: pure constructor helper.
fn s(v: &str) -> Json {
    Json::Str(v.to_string())
}
// 🚫️async: pure constructor helper.
fn nums(v: &[f64]) -> Json {
    Json::Arr(v.iter().map(|x| Json::Num(*x)).collect())
}
//#endregion 🧾️Json

//#region 🔬️Report
const PROBE_VERSION: &str = "ruststep@0.4.0 + dxf@0.6.1";

/// 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never
/// computes them, and this binary never computes what a mutation SHOULD have produced.
struct Report {
    probe: &'static str,
    engine_family: &'static str,
    engine_impl: &'static str,
    engine_version: &'static str,
    status: &'static str,
    measurements: Vec<(String, Json)>,
    diagnostics: Vec<(&'static str, String, Option<String>)>,
}

impl Report {
    // 🚫️async: pure constructor helper.
    fn new(probe: &'static str, engine: (&'static str, &'static str, &'static str)) -> Self {
        Report { probe, engine_family: engine.0, engine_impl: engine.1, engine_version: engine.2, status: "ok", measurements: Vec::new(), diagnostics: Vec::new() }
    }
    // 🚫️async: pure mutator helper.
    fn put(&mut self, key: &str, value: Json) {
        self.measurements.push((key.to_string(), value));
    }
    // 🚫️async: pure mutator helper.
    fn diag(&mut self, severity: &'static str, message: String, detail: Option<String>) {
        self.diagnostics.push((severity, message, detail));
    }
    // 🚫️async: pure formatting helper.
    fn emit(self, duration_ms: u128) -> String {
        let mut fields = vec![
            ("schema".to_string(), s("semio.repository-test.probe-report/v2")),
            ("probe".to_string(), s(self.probe)),
            ("probeVersion".to_string(), s(PROBE_VERSION)),
            ("engine".to_string(), obj(vec![("family", s(self.engine_family)), ("implementation", s(self.engine_impl)), ("version", s(self.engine_version))])),
            ("status".to_string(), s(self.status)),
            ("durationMs".to_string(), Json::Int(duration_ms as i64)),
            ("measurements".to_string(), Json::Obj(self.measurements)),
        ];
        if !self.diagnostics.is_empty() {
            fields.push((
                "diagnostics".to_string(),
                Json::Arr(
                    self.diagnostics
                        .into_iter()
                        .map(|(sev, msg, detail)| {
                            let mut d = vec![("severity".to_string(), s(sev)), ("message".to_string(), Json::Str(msg))];
                            if let Some(detail) = detail {
                                d.push(("detail".to_string(), Json::Str(detail)));
                            }
                            Json::Obj(d)
                        })
                        .collect(),
                ),
            ));
        }
        Json::Obj(fields).to_text()
    }
}

const STEP_ENGINE: (&str, &str, &str) = ("ruststep", "ruststep part-21 ast parser", "0.4.0");
const DXF_ENGINE: (&str, &str, &str) = ("dxf-rs", "dxf crate ascii reader/writer", "0.6.1");
//#endregion 🔬️Report

//#region 📐️Readings
/// 📐️ One entity as a THIRD-PARTY READER recovered it — never as our schema declares it.
#[derive(Clone, Debug, PartialEq)]
struct Ent {
    kind: String,
    layer: String,
    geom: Vec<f64>,
    text: String,
}

impl Ent {
    // 🚫️async: pure formatting helper.
    fn json(&self) -> Json {
        obj(vec![("kind", s(&self.kind)), ("layer", s(&self.layer)), ("geom", nums(&self.geom)), ("text", s(&self.text))])
    }
}

/// 📐️ A whole drawing as a third-party reader recovered it.
#[derive(Clone, Debug, PartialEq, Default)]
struct Reading {
    layers: Vec<(String, i64, String, bool)>,
    blocks: Vec<(String, [f64; 2], Vec<Ent>)>,
    entities: Vec<Ent>,
}

impl Reading {
    // 🚫️async: pure formatting helper.
    fn json(&self) -> Json {
        obj(vec![
            ("layers", Json::Arr(self.layers.iter().map(|(n, c, lt, v)| obj(vec![("name", s(n)), ("colorIndex", Json::Int(*c)), ("lineType", s(lt)), ("visible", Json::Bool(*v))])).collect())),
            ("blocks", Json::Arr(self.blocks.iter().map(|(n, bp, ents)| obj(vec![("name", s(n)), ("basePoint", nums(bp)), ("entities", Json::Arr(ents.iter().map(Ent::json).collect()))])).collect())),
            ("entities", Json::Arr(self.entities.iter().map(Ent::json).collect())),
        ])
    }
    // 🚫️async: pure accessor helper.
    fn counts(&self) -> (usize, usize, usize) {
        (self.layers.len(), self.blocks.len(), self.entities.len())
    }
}
//#endregion 📐️Readings

//#region 🖊️DxfRead
/// 🖊️ Reduces one `dxf` crate entity to the reading. The nine shapes our exporter emits are named;
/// anything else is reported by its own discriminant rather than silently coerced into a neighbour.
// 🚫️async: pure codec helper.
fn dxf_ent(e: &Entity) -> Ent {
    let layer = e.common.layer.clone();
    let p = |pt: &Point| vec![pt.x, pt.y];
    match &e.specific {
        EntityType::Line(v) => Ent { kind: "line".into(), layer, geom: [p(&v.p1), p(&v.p2)].concat(), text: String::new() },
        EntityType::Circle(v) => Ent { kind: "circle".into(), layer, geom: vec![v.center.x, v.center.y, v.radius], text: String::new() },
        EntityType::Arc(v) => Ent { kind: "arc".into(), layer, geom: vec![v.center.x, v.center.y, v.radius, v.start_angle, v.end_angle], text: String::new() },
        EntityType::Ellipse(v) => Ent {
            kind: "ellipse".into(),
            layer,
            geom: vec![v.center.x, v.center.y, v.major_axis.x, v.major_axis.y, v.minor_axis_ratio, v.start_parameter, v.end_parameter],
            text: String::new(),
        },
        EntityType::LwPolyline(v) => Ent { kind: "polyline".into(), layer, geom: v.vertices.iter().flat_map(|w| vec![w.x, w.y]).collect(), text: if v.is_closed() { "closed".into() } else { "open".into() } },
        EntityType::Polyline(v) => Ent { kind: "polyline".into(), layer, geom: v.vertices().flat_map(|w| vec![w.location.x, w.location.y]).collect(), text: if v.is_closed() { "closed".into() } else { "open".into() } },
        EntityType::Text(v) => Ent { kind: "text".into(), layer, geom: vec![v.location.x, v.location.y, v.text_height, v.rotation], text: v.value.clone() },
        EntityType::Insert(v) => Ent { kind: "insert".into(), layer, geom: vec![v.location.x, v.location.y, v.x_scale_factor, v.y_scale_factor, v.rotation], text: v.name.clone() },
        EntityType::Solid(v) => Ent {
            kind: "solid".into(),
            layer,
            geom: [p(&v.first_corner), p(&v.second_corner), p(&v.third_corner), p(&v.fourth_corner)].concat(),
            text: String::new(),
        },
        EntityType::RotatedDimension(v) => Ent {
            kind: "dimension".into(),
            layer,
            geom: vec![v.dimension_base.definition_point_1.x, v.dimension_base.definition_point_1.y, v.dimension_base.text_mid_point.x, v.dimension_base.text_mid_point.y],
            text: v.dimension_base.text.clone(),
        },
        other => Ent { kind: format!("unmapped:{}", dxf_discriminant(other)), layer, geom: Vec::new(), text: String::new() },
    }
}

// 🚫️async: pure codec helper.
fn dxf_discriminant(e: &EntityType) -> &'static str {
    match e {
        EntityType::Face3D(_) => "face3d",
        EntityType::Spline(_) => "spline",
        EntityType::MText(_) => "mtext",
        EntityType::ModelPoint(_) => "point",
        EntityType::Trace(_) => "trace",
        EntityType::Seqend(_) => "seqend",
        EntityType::Vertex(_) => "vertex",
        _ => "other",
    }
}

/// 🖊️ Reads one DXF file with the `dxf` crate and reduces it to the reading.
// 🚫️async: pure codec helper.
fn read_dxf(path: &str) -> Result<Reading, String> {
    let drawing = Drawing::load_file(path).map_err(|e| format!("dxf load {path}: {e}"))?;
    let mut layers: Vec<(String, i64, String, bool)> = drawing.layers().map(|l| (l.name.clone(), l.color.index().map(i64::from).unwrap_or(-1), l.line_type_name.clone(), l.is_layer_on)).collect();
    layers.sort_by(|a, b| a.0.cmp(&b.0));
    let blocks = drawing.blocks().map(|b| (b.name.clone(), [b.base_point.x, b.base_point.y], b.entities.iter().map(dxf_ent).collect())).collect();
    let entities = drawing.entities().map(dxf_ent).collect();
    Ok(Reading { layers, blocks, entities })
}
//#endregion 🖊️DxfRead

//#region 📐️StepRead
// 🚫️async: pure codec helper.
fn params(record: &Record) -> Vec<Parameter> {
    match &record.parameter {
        Parameter::List(v) => v.clone(),
        other => vec![other.clone()],
    }
}

// 🚫️async: pure codec helper.
fn as_ref_id(p: &Parameter) -> Option<u64> {
    match p {
        Parameter::Ref(ruststep::ast::Name::Entity(id)) => Some(*id),
        _ => None,
    }
}

// 🚫️async: pure codec helper.
fn as_real(p: &Parameter) -> Option<f64> {
    match p {
        Parameter::Real(v) => Some(*v),
        Parameter::Integer(v) => Some(*v as f64),
        _ => None,
    }
}

// 🚫️async: pure codec helper.
fn as_xy(p: &Parameter) -> Option<[f64; 2]> {
    match p {
        Parameter::List(v) if v.len() >= 2 => Some([as_real(&v[0])?, as_real(&v[1])?]),
        _ => None,
    }
}

/// 📐️ Reads one Part-21 file with ruststep and resolves the `LINE`/`CIRCLE` graph our exporter mints.
///
/// The resolution walks exactly the decomposition the export leaf builds in reverse —
/// `LINE → CARTESIAN_POINT + VECTOR → DIRECTION` and `CIRCLE → AXIS2_PLACEMENT_3D → CARTESIAN_POINT`
/// — using ruststep's own parsed graph and its own reference resolution. Nothing is guessed: an
/// entity whose referents are missing is reported as unresolved rather than defaulted.
// 🚫️async: pure codec helper.
fn read_step(path: &str) -> Result<(Reading, usize), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
    let exchange = Exchange::from_str(&text).map_err(|e| format!("ruststep parse {path}: {e}"))?;
    let mut by_id: BTreeMap<u64, Record> = BTreeMap::new();
    let mut order: Vec<u64> = Vec::new();
    for section in &exchange.data {
        for instance in &section.entities {
            if let EntityInstance::Simple { id, record } = instance {
                by_id.insert(*id, record.clone());
                order.push(*id);
            }
        }
    }
    let point_of = |id: u64| -> Option<[f64; 2]> {
        let r = by_id.get(&id)?;
        if r.name != "CARTESIAN_POINT" {
            return None;
        }
        as_xy(params(r).get(1)?)
    };
    let mut entities = Vec::new();
    let mut unresolved = 0usize;
    for id in &order {
        let record = &by_id[id];
        match record.name.as_str() {
            "LINE" => {
                let p = params(record);
                let start = p.get(1).and_then(as_ref_id).and_then(point_of);
                let vector = p.get(2).and_then(as_ref_id).and_then(|v| by_id.get(&v).cloned());
                let resolved = match (start, vector) {
                    (Some(a), Some(v)) if v.name == "VECTOR" => {
                        let vp = params(&v);
                        let dir = vp.first_ref_dir(&by_id);
                        let magnitude = vp.get(2).and_then(as_real);
                        match (dir, magnitude) {
                            (Some(d), Some(m)) => Some(Ent { kind: "line".into(), layer: String::new(), geom: vec![a[0], a[1], a[0] + d[0] * m, a[1] + d[1] * m], text: String::new() }),
                            _ => None,
                        }
                    }
                    _ => None,
                };
                match resolved {
                    Some(e) => entities.push(e),
                    None => unresolved += 1,
                }
            }
            "CIRCLE" => {
                let p = params(record);
                let placement = p.get(1).and_then(as_ref_id).and_then(|v| by_id.get(&v).cloned());
                let radius = p.get(2).and_then(as_real);
                let centre = placement.filter(|pl| pl.name == "AXIS2_PLACEMENT_3D").and_then(|pl| params(&pl).get(1).and_then(as_ref_id)).and_then(point_of);
                match (centre, radius) {
                    (Some(c), Some(r)) => entities.push(Ent { kind: "circle".into(), layer: String::new(), geom: vec![c[0], c[1], r], text: String::new() }),
                    _ => unresolved += 1,
                }
            }
            _ => {}
        }
    }
    Ok((Reading { layers: Vec::new(), blocks: Vec::new(), entities }, unresolved))
}

/// 📐️ Resolves a `VECTOR`'s `DIRECTION` referent to its 2D components.
trait FirstRefDir {
    fn first_ref_dir(&self, by_id: &BTreeMap<u64, Record>) -> Option<[f64; 2]>;
}

impl FirstRefDir for Vec<Parameter> {
    // 🚫️async: pure codec helper.
    fn first_ref_dir(&self, by_id: &BTreeMap<u64, Record>) -> Option<[f64; 2]> {
        let id = as_ref_id(self.get(1)?)?;
        let record = by_id.get(&id)?;
        if record.name != "DIRECTION" {
            return None;
        }
        as_xy(params(record).get(1)?)
    }
}
//#endregion 📐️StepRead

//#region ⚖️Compare
const NEAR_EXACT: f64 = 1e-12;

/// ⚖️ The largest absolute difference between two readings, and where it is.
///
/// `cad` records 2D entity geometry EXACTLY — points, radii and angles as `f64`, never tessellated —
/// so there is no legitimate "differently discretised but equally valid" reading of a LINE or a
/// CIRCLE the way there is for a tessellated solid. The gate is therefore near-exact, mesh-shaped
/// rather than brep-shaped.
// 🚫️async: pure comparison helper.
fn compare(expected: &Reading, actual: &Reading) -> (bool, f64, Vec<String>) {
    let mut worst = 0.0f64;
    let mut problems: Vec<String> = Vec::new();
    let (el, eb, ee) = expected.counts();
    let (al, ab, ae) = actual.counts();
    if (el, eb, ee) != (al, ab, ae) {
        problems.push(format!("counts differ: expected {el} layer(s)/{eb} block(s)/{ee} entity(ies), actual {al}/{ab}/{ae}"));
    }
    for (i, (e, a)) in expected.layers.iter().zip(actual.layers.iter()).enumerate() {
        if e != a {
            problems.push(format!("layer[{i}] differs: {e:?} vs {a:?}"));
        }
    }
    let cmp_ents = |scope: &str, es: &[Ent], as_: &[Ent], problems: &mut Vec<String>, worst: &mut f64| {
        for (i, (e, a)) in es.iter().zip(as_.iter()).enumerate() {
            if e.kind != a.kind {
                problems.push(format!("{scope}[{i}] kind differs: {} vs {}", e.kind, a.kind));
                continue;
            }
            if e.layer != a.layer {
                problems.push(format!("{scope}[{i}] layer differs: {:?} vs {:?}", e.layer, a.layer));
            }
            if e.text != a.text {
                problems.push(format!("{scope}[{i}] text differs: {:?} vs {:?}", e.text, a.text));
            }
            if e.geom.len() != a.geom.len() {
                problems.push(format!("{scope}[{i}] geometry arity differs: {} vs {}", e.geom.len(), a.geom.len()));
                continue;
            }
            for (j, (x, y)) in e.geom.iter().zip(a.geom.iter()).enumerate() {
                let delta = (x - y).abs();
                if delta > *worst {
                    *worst = delta;
                }
                if delta > NEAR_EXACT {
                    problems.push(format!("{scope}[{i}].geom[{j}] differs by {delta:.6e}: {x} vs {y}"));
                }
            }
        }
    };
    cmp_ents("entity", &expected.entities, &actual.entities, &mut problems, &mut worst);
    for (i, (e, a)) in expected.blocks.iter().zip(actual.blocks.iter()).enumerate() {
        if e.0 != a.0 {
            problems.push(format!("block[{i}] name differs: {:?} vs {:?}", e.0, a.0));
        }
        for j in 0..2 {
            let delta = (e.1[j] - a.1[j]).abs();
            if delta > worst {
                worst = delta;
            }
            if delta > NEAR_EXACT {
                problems.push(format!("block[{i}].basePoint[{j}] differs by {delta:.6e}"));
            }
        }
        if e.2.len() != a.2.len() {
            problems.push(format!("block[{i}] entity count differs: {} vs {}", e.2.len(), a.2.len()));
        }
        cmp_ents(&format!("block[{i}].entity"), &e.2, &a.2, &mut problems, &mut worst);
    }
    (problems.is_empty(), worst, problems)
}
//#endregion ⚖️Compare

//#region 🏭️Fixtures
// 🚫️async: pure constructor helper.
fn pt(x: f64, y: f64) -> Point {
    Point::new(x, y, 0.0)
}

// 🚫️async: pure constructor helper.
fn on_layer(kind: EntityType, layer: &str) -> Entity {
    let mut e = Entity::new(kind);
    e.common.layer = layer.to_string();
    e
}

// 🚫️async: pure constructor helper.
fn layer(name: &str, color: u8, line_type: &str, visible: bool) -> dxf::tables::Layer {
    let mut l = dxf::tables::Layer { name: name.to_string(), ..Default::default() };
    l.color = dxf::Color::from_index(color);
    l.line_type_name = line_type.to_string();
    l.is_layer_on = visible;
    l
}

// 🚫️async: pure constructor helper.
fn line(a: (f64, f64), b: (f64, f64), l: &str) -> Entity {
    on_layer(EntityType::Line(Line { p1: pt(a.0, a.1), p2: pt(b.0, b.1), ..Default::default() }), l)
}
// 🚫️async: pure constructor helper.
fn circle(c: (f64, f64), r: f64, l: &str) -> Entity {
    on_layer(EntityType::Circle(Circle { center: pt(c.0, c.1), radius: r, ..Default::default() }), l)
}
// 🚫️async: pure constructor helper.
fn arc(c: (f64, f64), r: f64, a0: f64, a1: f64, l: &str) -> Entity {
    on_layer(EntityType::Arc(Arc { center: pt(c.0, c.1), radius: r, start_angle: a0, end_angle: a1, ..Default::default() }), l)
}
// 🚫️async: pure constructor helper.
fn add_polyline(d: &mut Drawing, points: &[(f64, f64)], closed: bool, l: &str) {
    let mut p = Polyline::default();
    p.set_is_closed(closed);
    for (x, y) in points {
        p.add_vertex(d, Vertex::new(pt(*x, *y)));
    }
    d.add_entity(on_layer(EntityType::Polyline(p), l));
}
// 🚫️async: pure constructor helper.
fn text(p: (f64, f64), height: f64, value: &str, l: &str) -> Entity {
    on_layer(EntityType::Text(Text { location: pt(p.0, p.1), text_height: height, value: value.to_string(), rotation: 0.0, ..Default::default() }), l)
}
// 🚫️async: pure constructor helper.
fn insert(name: &str, p: (f64, f64), l: &str) -> Entity {
    on_layer(EntityType::Insert(Insert { name: name.to_string(), location: pt(p.0, p.1), x_scale_factor: 1.0, y_scale_factor: 1.0, rotation: 0.0, ..Default::default() }), l)
}
// 🚫️async: pure constructor helper.
fn solid(c: [(f64, f64); 4], l: &str) -> Entity {
    on_layer(EntityType::Solid(Solid { first_corner: pt(c[0].0, c[0].1), second_corner: pt(c[1].0, c[1].1), third_corner: pt(c[2].0, c[2].1), fourth_corner: pt(c[3].0, c[3].1), ..Default::default() }), l)
}
/// ⏱️ Pins the four wall-clock header variables the `dxf` crate stamps from `Local::now()`/`Utc::now()`
/// (`$TDCREATE`, `$TDUCREATE`, `$TDUPDATE`, `$TDUUPDATE`) to J2000.0.
///
/// MEASURED, not anticipated: generating the corpus twice and diffing produced
/// `2461280.937766203657` against `2461280.937893518712` on every DXF file — a Julian day number
/// differing in the fractional part, i.e. the moment of the run. A fixture that changes on every
/// invocation cannot be reproduced, and `test fixture reproduce` would have failed all sixteen.
///
/// The pinning goes through the LIBRARY'S OWN parser rather than being patched into the final bytes:
/// the drawing is serialised, the four values are rewritten in that intermediate text, and the
/// result is handed back to `Drawing::load` — which accepts it, proving the edit is still a valid
/// DXF — so the bytes finally committed are ones the `dxf` crate wrote from a state it parsed.
// 🚫️async: pure codec helper.
fn pin_wall_clock(drawing: &Drawing) -> Result<Drawing, String> {
    const PINNED: &str = "2451545.0";
    const TIME_VARS: [&str; 4] = ["$TDCREATE", "$TDUCREATE", "$TDUPDATE", "$TDUUPDATE"];
    let mut buffer: Vec<u8> = Vec::new();
    drawing.save(&mut buffer).map_err(|e| format!("dxf save to buffer: {e}"))?;
    let text = String::from_utf8_lossy(&buffer).into_owned();
    let mut lines: Vec<String> = text.split('\n').map(|l| l.trim_end_matches('\r').to_string()).collect();
    let mut index = 0;
    while index + 2 < lines.len() {
        if TIME_VARS.contains(&lines[index].trim()) && lines[index + 1].trim() == "40" {
            lines[index + 2] = PINNED.to_string();
        }
        index += 1;
    }
    let pinned = lines.join("\r\n");
    Drawing::load(&mut pinned.as_bytes()).map_err(|e| format!("dxf reload after pinning: {e}"))
}

/// 🧫️ The base drawing every recipe mutates — seven `CadEntity` shapes, three layers, two blocks.
// 🚫️async: pure constructor helper.
fn base_drawing() -> Drawing {
    let mut d = Drawing::new();
    d.header.version = dxf::enums::AcadVersion::R12;
    d.add_layer(layer("0", 7, "CONTINUOUS", true));
    d.add_layer(layer("WALLS", 1, "CONTINUOUS", true));
    let mut door = dxf::Block { name: "DOOR".to_string(), base_point: pt(0.0, 0.0), ..Default::default() };
    door.entities.push(line((0.0, 0.0), (1.0, 0.0), "0"));
    door.entities.push(arc((0.0, 0.0), 1.0, 0.0, 90.0, "0"));
    d.add_block(door);
    let mut window = dxf::Block { name: "WINDOW".to_string(), base_point: pt(5.0, 0.0), ..Default::default() };
    window.entities.push(line((5.0, 0.0), (6.0, 0.0), "0"));
    window.entities.push(line((5.5, 0.0), (5.5, 1.0), "WALLS"));
    d.add_block(window);
    // 🪆️Seven of the nine `CadEntity` shapes. `Ellipse` and `Dimension` are ABSENT ON PURPOSE and the
    // absence is a measured finding, not an oversight: both are R13+ entities, the `dxf` crate will
    // not write either into an R12 document, and this subset's dialect is r12. Our own exporter
    // smuggles them through as bridge-owned RAW GROUP CODES (`DxfEntity::Other`), which is not
    // standard R12 and which the crate does not recover as typed entities. So the oracle's reach is
    // seven shapes, not nine, and those two are registered as uncarried rather than papered over.
    for e in [
        line((0.0, 0.0), (5.0, 0.0), "WALLS"),
        circle((2.0, 2.0), 1.5, "WALLS"),
        arc((4.0, 4.0), 2.0, 30.0, 120.0, "0"),
        text((3.0, 3.0), 0.25, "ROOM A", "0"),
        insert("DOOR", (7.0, 0.0), "0"),
        solid([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], "0"),
    ] {
        d.add_entity(e);
    }
    add_polyline(&mut d, &[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)], false, "WALLS");
    d
}

/// 🧫️ Applies one recipe's AFTER state. Each arm is a hand-authored drawing edit expressing exactly
/// one mutation KIND — it is not our production mutation code, and it is not derived from it.
// 🚫️async: pure constructor helper.
fn after_drawing(recipe: &str) -> Option<Drawing> {
    let mut d = base_drawing();
    match recipe {
        "no-mutation-identity" => {}
        "set-snapshot-replaces-drawing" => {
            d = Drawing::new();
            d.header.version = dxf::enums::AcadVersion::R12;
            d.add_layer(layer("GRID", 4, "CONTINUOUS", true));
            d.add_entity(line((0.0, 0.0), (10.0, 0.0), "GRID"));
            d.add_entity(circle((5.0, 5.0), 2.5, "GRID"));
        }
        "add-layer-hidden-services" => {
            d.add_layer(layer("SERVICES", 3, "DASHED", false));
        }
        "remove-layer-scratch" => {
            let kept: Vec<_> = d.layers().filter(|l| l.name != "WALLS").cloned().collect();
            d = rebuild_with_layers(d, kept);
        }
        "set-layer-walls-color" => {
            for l in d.layers_mut() {
                if l.name == "WALLS" {
                    l.color = dxf::Color::from_index(5);
                }
            }
        }
        "add-block-door" => {
            let mut b = dxf::Block { name: "COLUMN".to_string(), base_point: pt(9.0, 9.0), ..Default::default() };
            b.entities.push(circle((9.0, 9.0), 0.3, "0"));
            d.add_block(b);
        }
        "remove-block-window" => {
            let kept: Vec<_> = d.blocks().filter(|b| b.name != "WINDOW").cloned().collect();
            d = rebuild_with_blocks(d, kept);
        }
        "set-block-base-point-door" => {
            for b in d.blocks_mut() {
                if b.name == "DOOR" {
                    b.base_point = pt(0.5, 0.25);
                }
            }
        }
        "add-entity-arc-fillet" => {
            d.add_entity(arc((6.0, 6.0), 0.75, 180.0, 270.0, "WALLS"));
        }
        "remove-entity-middle-polyline" => {
            let kept: Vec<_> = d.entities().filter(|e| !matches!(e.specific, EntityType::Polyline(_))).cloned().collect();
            d = rebuild_with_entities(d, kept);
        }
        "set-entity-layer-text-to-annotations" => {
            d.add_layer(layer("ANNOTATIONS", 2, "CONTINUOUS", true));
            let mut kept: Vec<_> = d.entities().cloned().collect();
            for e in kept.iter_mut() {
                if matches!(e.specific, EntityType::Text(_)) {
                    e.common.layer = "ANNOTATIONS".to_string();
                }
            }
            d = rebuild_with_entities(d, kept);
        }
        "set-entity-geometry-circle-radius" => {
            let mut kept: Vec<_> = d.entities().cloned().collect();
            for e in kept.iter_mut() {
                if let EntityType::Circle(c) = &mut e.specific {
                    c.radius = 2.25;
                }
            }
            d = rebuild_with_entities(d, kept);
        }
        // 🚨️The deliberate COUNTEREXAMPLE for the gate: same mutation, wrong answer. The radius is
        // off by 0.05 and the centre by 0.1 — small enough that a loose tolerance would wave it
        // through, which is exactly what makes it a useful negative.
        "set-entity-geometry-circle-radius-counterexample" => {
            let mut kept: Vec<_> = d.entities().cloned().collect();
            for e in kept.iter_mut() {
                if let EntityType::Circle(c) = &mut e.specific {
                    c.radius = 2.30;
                    c.center = pt(2.1, 2.0);
                }
            }
            d = rebuild_with_entities(d, kept);
        }
        "add-block-entity-door-swing" => {
            for b in d.blocks_mut() {
                if b.name == "DOOR" {
                    b.entities.push(line((0.0, 0.0), (0.0, 1.0), "0"));
                }
            }
        }
        "remove-block-entity-window-mullion" => {
            for b in d.blocks_mut() {
                if b.name == "WINDOW" {
                    b.entities.truncate(1);
                }
            }
        }
        "set-block-entity-layer-door-leaf" => {
            d.add_layer(layer("LEAF", 6, "CONTINUOUS", true));
            for b in d.blocks_mut() {
                if b.name == "DOOR" {
                    if let Some(first) = b.entities.first_mut() {
                        first.common.layer = "LEAF".to_string();
                    }
                }
            }
        }
        "set-block-entity-geometry-window-pane" => {
            for b in d.blocks_mut() {
                if b.name == "WINDOW" {
                    if let Some(EntityType::Line(l)) = b.entities.first_mut().map(|e| &mut e.specific) {
                        l.p2 = pt(6.5, 0.0);
                    }
                }
            }
        }
        _ => return None,
    }
    Some(d)
}

// 🚫️async: pure constructor helper.
fn rebuild_with_entities(source: Drawing, entities: Vec<Entity>) -> Drawing {
    let mut d = Drawing::new();
    d.header.version = dxf::enums::AcadVersion::R12;
    for l in source.layers().cloned().collect::<Vec<_>>() {
        d.add_layer(l);
    }
    for b in source.blocks().cloned().collect::<Vec<_>>() {
        d.add_block(b);
    }
    for e in entities {
        d.add_entity(e);
    }
    d
}

// 🚫️async: pure constructor helper.
fn rebuild_with_layers(source: Drawing, layers: Vec<dxf::tables::Layer>) -> Drawing {
    let mut d = Drawing::new();
    d.header.version = dxf::enums::AcadVersion::R12;
    for l in layers {
        d.add_layer(l);
    }
    for b in source.blocks().cloned().collect::<Vec<_>>() {
        d.add_block(b);
    }
    for e in source.entities().cloned().collect::<Vec<_>>() {
        d.add_entity(e);
    }
    d
}

// 🚫️async: pure constructor helper.
fn rebuild_with_blocks(source: Drawing, blocks: Vec<dxf::Block>) -> Drawing {
    let mut d = Drawing::new();
    d.header.version = dxf::enums::AcadVersion::R12;
    for l in source.layers().cloned().collect::<Vec<_>>() {
        d.add_layer(l);
    }
    for b in blocks {
        d.add_block(b);
    }
    for e in source.entities().cloned().collect::<Vec<_>>() {
        d.add_entity(e);
    }
    d
}

/// 📐️ The STEP side. `cad`'s step export reaches Line and Circle only, so these recipes stay inside
/// that vocabulary on purpose — a recipe touching a layer or a block would be unwitnessable by
/// construction and is registered against dxf alone instead.
// 🚫️async: pure constructor helper.
fn step_entities(recipe: &str, after: bool) -> Option<Vec<(String, Vec<f64>)>> {
    let base = vec![("line".to_string(), vec![0.0, 0.0, 5.0, 0.0]), ("circle".to_string(), vec![2.0, 2.0, 1.5])];
    let mut e = base.clone();
    if !after {
        return Some(e);
    }
    match recipe {
        "step-no-mutation-identity" => {}
        "step-set-snapshot-replaces-entities" => {
            e = vec![("line".to_string(), vec![0.0, 0.0, 10.0, 0.0]), ("circle".to_string(), vec![5.0, 5.0, 2.5])];
        }
        "step-add-entity-circle" => {
            e.push(("circle".to_string(), vec![8.0, 1.0, 0.75]));
        }
        "step-remove-entity-line" => {
            e.remove(0);
        }
        "step-set-entity-geometry-circle-radius" => {
            e[1].1[2] = 2.25;
        }
        "step-set-entity-geometry-circle-radius-counterexample" => {
            e[1].1[2] = 2.30;
            e[1].1[0] = 2.1;
        }
        _ => return None,
    }
    Some(e)
}

/// 📐️ Emits deterministic Part-21 text in exactly the decomposition `SemioCadToStep` builds.
///
/// HONEST PROVENANCE: this text is OURS. `ruststep` 0.4.0 has no Part-21 writer — no AST type
/// implements `Display` and `ast::ser::to_record` stops at an in-memory `Record` whose nesting
/// disagrees with its own parser — so no third party in this repository can write STEP. Every file
/// produced here is immediately re-read by `read_step` and refused unless ruststep recovers exactly
/// the geometry that went in; that is verification by a third party, not generation by one, and the
/// fixture class says `handcrafted` accordingly.
// 🚫️async: pure codec helper.
fn write_step(entities: &[(String, Vec<f64>)]) -> String {
    let mut out = String::new();
    out.push_str("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio-cad-export','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n");
    let mut id = 0u64;
    let mut next = || {
        id += 1;
        id
    };
    for (kind, g) in entities {
        match kind.as_str() {
            "line" => {
                let (ax, ay, bx, by) = (g[0], g[1], g[2], g[3]);
                let (dx, dy) = (bx - ax, by - ay);
                let magnitude = (dx * dx + dy * dy).sqrt();
                let (ndx, ndy) = if magnitude > 0.0 { (dx / magnitude, dy / magnitude) } else { (1.0, 0.0) };
                let p = next();
                let _ = writeln!(out, "#{p}=CARTESIAN_POINT('',({ax:?},{ay:?},0.0));");
                let dir = next();
                let _ = writeln!(out, "#{dir}=DIRECTION('',({ndx:?},{ndy:?},0.0));");
                let vec_id = next();
                let _ = writeln!(out, "#{vec_id}=VECTOR('',#{dir},{magnitude:?});");
                let l = next();
                let _ = writeln!(out, "#{l}=LINE('',#{p},#{vec_id});");
            }
            "circle" => {
                let (cx, cy, r) = (g[0], g[1], g[2]);
                let p = next();
                let _ = writeln!(out, "#{p}=CARTESIAN_POINT('',({cx:?},{cy:?},0.0));");
                let placement = next();
                let _ = writeln!(out, "#{placement}=AXIS2_PLACEMENT_3D('',#{p},$,$);");
                let c = next();
                let _ = writeln!(out, "#{c}=CIRCLE('',#{placement},{r:?});");
            }
            _ => {}
        }
    }
    out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    out
}

const DXF_RECIPES: &[&str] = &[
    "no-mutation-identity",
    "set-snapshot-replaces-drawing",
    "add-layer-hidden-services",
    "remove-layer-scratch",
    "set-layer-walls-color",
    "add-block-door",
    "remove-block-window",
    "set-block-base-point-door",
    "add-entity-arc-fillet",
    "remove-entity-middle-polyline",
    "set-entity-layer-text-to-annotations",
    "set-entity-geometry-circle-radius",
    "add-block-entity-door-swing",
    "remove-block-entity-window-mullion",
    "set-block-entity-layer-door-leaf",
    "set-block-entity-geometry-window-pane",
];

const STEP_RECIPES: &[&str] = &["step-no-mutation-identity", "step-set-snapshot-replaces-entities", "step-add-entity-circle", "step-remove-entity-line", "step-set-entity-geometry-circle-radius"];

/// 🏭️ Writes one recipe's bundle into `<out>/<recipe>/`.
// 🚫️async: filesystem entry point.
fn generate(recipe: &str, out_root: &str) -> Result<Vec<String>, String> {
    let dir = format!("{out_root}/{recipe}");
    std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir {dir}: {e}"))?;
    let mut written = Vec::new();
    if DXF_RECIPES.contains(&recipe) {
        for (name, drawing) in [("before.dxf", base_drawing()), ("after.dxf", after_drawing(recipe).ok_or_else(|| format!("unknown recipe {recipe}"))?)] {
            let path = format!("{dir}/{name}");
            pin_wall_clock(&drawing)?.save_file(&path).map_err(|e| format!("dxf save {path}: {e}"))?;
            written.push(path);
        }
        if recipe == "set-entity-geometry-circle-radius" {
            let path = format!("{dir}/counterexample-after.dxf");
            let counterexample = after_drawing("set-entity-geometry-circle-radius-counterexample").ok_or("counterexample")?;
            pin_wall_clock(&counterexample)?.save_file(&path).map_err(|e| format!("dxf save {path}: {e}"))?;
            written.push(path);
        }
    } else if STEP_RECIPES.contains(&recipe) {
        for (name, after) in [("before.step", false), ("after.step", true)] {
            let entities = step_entities(recipe, after).ok_or_else(|| format!("unknown recipe {recipe}"))?;
            let path = format!("{dir}/{name}");
            std::fs::write(&path, write_step(&entities)).map_err(|e| format!("write {path}: {e}"))?;
            verify_step_roundtrip(&path, &entities)?;
            written.push(path);
        }
        if recipe == "step-set-entity-geometry-circle-radius" {
            let entities = step_entities("step-set-entity-geometry-circle-radius-counterexample", true).ok_or("counterexample")?;
            let path = format!("{dir}/counterexample-after.step");
            std::fs::write(&path, write_step(&entities)).map_err(|e| format!("write {path}: {e}"))?;
            verify_step_roundtrip(&path, &entities)?;
            written.push(path);
        }
    } else {
        return Err(format!("unknown recipe {recipe}"));
    }
    Ok(written)
}

/// ✅️ Refuses to emit a STEP fixture ruststep cannot read back to exactly what went in.
// 🚫️async: filesystem verification helper.
fn verify_step_roundtrip(path: &str, entities: &[(String, Vec<f64>)]) -> Result<(), String> {
    let (reading, unresolved) = read_step(path)?;
    if unresolved != 0 {
        return Err(format!("{path}: ruststep left {unresolved} entity(ies) unresolved"));
    }
    if reading.entities.len() != entities.len() {
        return Err(format!("{path}: ruststep recovered {} entity(ies), wrote {}", reading.entities.len(), entities.len()));
    }
    for (i, (want, got)) in entities.iter().zip(reading.entities.iter()).enumerate() {
        if want.0 != got.kind {
            return Err(format!("{path}: entity {i} kind {} became {}", want.0, got.kind));
        }
        for (j, (x, y)) in want.1.iter().zip(got.geom.iter()).enumerate() {
            if (x - y).abs() > 1e-9 {
                return Err(format!("{path}: entity {i} coordinate {j} {x} became {y}"));
            }
        }
    }
    Ok(())
}
//#endregion 🏭️Fixtures

//#region 🚪️Entry
// 🚫️async: pure argument helper.
fn values(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag && i + 1 < args.len() {
            out.push(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

/// 🧭️ A carrier that produced the SAME reading for both sides of a real mutation did not encode it.
///
/// This rule belongs to the WITNESS question ("did the carrier record the change at all?") and to
/// that question only. It is derived from the data rather than from a table: DXF drops the layer for
/// `Ellipse` and `Dimension`, and STEP drops layers, blocks and seven of the nine entity shapes
/// outright. In every one of those cases the two readings come back identical, and reporting that as
/// a passing `ok` would let the mutation clear a gate standing on the ABSENCE of the evidence.
///
/// 🚨️It must NEVER be applied to the AGREEMENT question ("does the oracle's answer match ours?"),
/// where two identical readings are precisely what success looks like. Wiring one rule into both
/// questions made the gate report `unsupported` for a known-GOOD pair — caught by validating the
/// gate in both directions, which is the whole reason that is mandatory.
// 🚫️async: pure predicate helper.
fn unwitnessed(mutation: &str, before: &Reading, after: &Reading) -> bool {
    mutation != "no-mutation" && !mutation.is_empty() && before == after
}

fn main() {
    let started = std::time::Instant::now();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().cloned().unwrap_or_default();
    let inputs = values(&args, "--input");
    let mutation = values(&args, "--mutation").first().cloned().unwrap_or_default();
    let engine = if command.starts_with("step") { STEP_ENGINE } else { DXF_ENGINE };
    let probe: &'static str = match command.as_str() {
        "dxf-read" => "dxf-read",
        "dxf-witness" => "dxf-witness",
        "dxf-compare" => "dxf-compare",
        "step-read" => "step-read",
        "step-witness" => "step-witness",
        "step-compare" => "step-compare",
        "generate" => "generate",
        _ => {
            eprintln!("[probe] unknown command {command:?} — expected dxf-read | dxf-witness | dxf-compare | step-read | step-witness | step-compare | generate");
            std::process::exit(2);
        }
    };

    if command == "generate" {
        let out = values(&args, "--out").first().cloned().unwrap_or_default();
        let only = values(&args, "--only");
        let recipes: Vec<String> = if only.is_empty() { DXF_RECIPES.iter().chain(STEP_RECIPES.iter()).map(|r| r.to_string()).collect() } else { only };
        let mut failed = 0;
        for recipe in &recipes {
            match generate(recipe, &out) {
                Ok(files) => println!("[generate] {recipe}: {} file(s)", files.len()),
                Err(e) => {
                    eprintln!("[generate] {recipe}: {e}");
                    failed += 1;
                }
            }
        }
        std::process::exit(if failed > 0 { 1 } else { 0 });
    }

    let mut report = Report::new(probe, engine);
    let read = |path: &str| -> Result<(Reading, usize), String> { if command.starts_with("step") { read_step(path) } else { read_dxf(path).map(|r| (r, 0)) } };

    match probe {
        "dxf-read" | "step-read" => match inputs.first() {
            None => {
                report.status = "failed";
                report.diag("error", "no --input given".into(), None);
            }
            Some(path) => match read(path) {
                Err(e) => {
                    report.status = "failed";
                    report.diag("error", e, None);
                }
                Ok((reading, unresolved)) => {
                    let (l, b, en) = reading.counts();
                    report.put("layerCount", Json::Int(l as i64));
                    report.put("blockCount", Json::Int(b as i64));
                    report.put("entityCount", Json::Int(en as i64));
                    report.put("unresolvedEntities", Json::Int(unresolved as i64));
                    report.put("reading", reading.json());
                }
            },
        },
        "dxf-compare" | "step-compare" | "dxf-witness" | "step-witness" => {
            let witnessing = probe.ends_with("witness");
            if inputs.len() != 2 {
                report.status = "failed";
                report.diag("error", format!("{probe} needs exactly two --input, got {}", inputs.len()), None);
            } else {
                match (read(&inputs[0]), read(&inputs[1])) {
                    (Err(e), _) | (_, Err(e)) => {
                        report.status = "failed";
                        report.diag("error", e, None);
                    }
                    (Ok((left, ul)), Ok((right, ur))) => {
                        let (equal, worst, problems) = compare(&left, &right);
                        report.put("bothRead", Json::Bool(true));
                        report.put("unresolvedEntities", Json::Int((ul + ur) as i64));
                        report.put("readingsEqual", Json::Bool(equal));
                        report.put("maxAbsoluteDelta", Json::Num(worst));
                        report.put("toleranceUsed", Json::Num(NEAR_EXACT));
                        report.put("leftCounts", nums(&[left.counts().0 as f64, left.counts().1 as f64, left.counts().2 as f64]));
                        report.put("rightCounts", nums(&[right.counts().0 as f64, right.counts().1 as f64, right.counts().2 as f64]));
                        report.put("differenceCount", Json::Int(problems.len() as i64));
                        report.put("differences", Json::Arr(problems.iter().take(24).map(|p| Json::Str(p.clone())).collect()));
                        if witnessing {
                            // 🔎️WITNESS: the inputs are BEFORE and AFTER, and the question is whether
                            // the carrier recorded the mutation at all.
                            let witnessed = !unwitnessed(&mutation, &left, &right);
                            report.put("carrierWitnessed", Json::Bool(witnessed));
                            if !witnessed {
                                report.status = "unsupported";
                                report.diag(
                                    "warning",
                                    format!("this carrier does not encode mutation {mutation:?}"),
                                    Some("before and after read back identical, so the carrier dropped whatever the mutation changed; reporting unsupported rather than a passing ok, which would stand on the absence of the evidence".into()),
                                );
                            }
                        } else {
                            // ⚖️AGREE: the inputs are EXPECTED and ACTUAL, and equality IS the pass.
                            report.put("agree", Json::Bool(equal));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    println!("{}", report.emit(started.elapsed().as_millis()));
}
//#endregion 🚪️Entry
