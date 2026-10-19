//! 🖼️ Base and after drawings: the base drawing and the drawing after each recorded mutation. See [`super`].

use super::*;

//#region 🏭️Fixtures
/// 🧫️ The base drawing every recipe mutates — seven `CadEntity` shapes, three layers, two blocks.
// 🚫️async: pure constructor helper.
pub(super) fn base_drawing() -> Drawing {
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
pub(super) fn after_drawing(recipe: &str) -> Option<Drawing> {
    let mut d = base_drawing();
    match recipe {
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
//#endregion 🏭️Fixtures
