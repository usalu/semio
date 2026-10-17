//! ⚖️ Reading comparison: the near-exact tolerance and the comparison of two readings into a difference report. See [`super`].

use super::*;

//#region ⚖️Compare
pub(super) const NEAR_EXACT: f64 = 1e-12;

/// ⚖️ The largest absolute difference between two readings, and where it is.
///
/// `cad` records 2D entity geometry EXACTLY — points, radii and angles as `f64`, never tessellated —
/// so there is no legitimate "differently discretised but equally valid" reading of a LINE or a
/// CIRCLE the way there is for a tessellated solid. The gate is therefore near-exact, mesh-shaped
/// rather than brep-shaped.
// 🚫️async: pure comparison helper.
pub(super) fn compare(expected: &Reading, actual: &Reading) -> (bool, f64, Vec<String>) {
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
