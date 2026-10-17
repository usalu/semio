//! 🖊️ DXF and STEP readers: the DXF entity reader and discriminant, the Part-21 parameter accessors and the STEP reader with its first reference helper. See [`super`].

use super::*;

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
pub(super) fn read_dxf(path: &str) -> Result<Reading, String> {
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
pub(super) fn read_step(path: &str) -> Result<(Reading, usize), String> {
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
