//! ♻️ Rebuilds and the STEP writer: the drawing rebuilds with replaced entities, layers and blocks, the STEP entity list and the deterministic Part-21 writer. See [`super`].

use super::*;

//#region 🏭️Fixtures
// 🚫️async: pure constructor helper.
pub(super) fn rebuild_with_entities(source: Drawing, entities: Vec<Entity>) -> Drawing {
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
pub(super) fn rebuild_with_layers(source: Drawing, layers: Vec<dxf::tables::Layer>) -> Drawing {
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
pub(super) fn rebuild_with_blocks(source: Drawing, blocks: Vec<dxf::Block>) -> Drawing {
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
pub(super) fn step_entities(recipe: &str, after: bool) -> Option<Vec<(String, Vec<f64>)>> {
    let base = vec![("line".to_string(), vec![0.0, 0.0, 5.0, 0.0]), ("circle".to_string(), vec![2.0, 2.0, 1.5])];
    let mut e = base.clone();
    if !after {
        return Some(e);
    }
    match recipe {
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
pub(super) fn write_step(entities: &[(String, Vec<f64>)]) -> String {
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
//#endregion 🏭️Fixtures
