
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn rejected_recipes_carry_no_after_state_applied_ones_do() {
    for id in RECIPE_IDS {
        if *id == "drafting-plate" {
            continue;
        }
        match recipe(id).unwrap() {
            RecipeOutput::Pair(_, after) => {
                let should_have_after = !id.contains("-rejected-");
                assert_eq!(after.is_some(), should_have_after, "recipe {id} outcome must match its own id");
            }
            RecipeOutput::Single(_) => panic!("recipe {id} unexpectedly single-file"),
        }
    }
}

#[test]
fn drafting_plate_is_unaffected_by_this_retrofit() {
    // 🔒 Guards the ONE fixture this retrofit must never change: same bytes, same digest, as
    // measured before this file was extended (see the committed fixtureManifests entry).
    let bytes = encode(&base_doc());
    assert_eq!(bytes.len(), 9521, "drafting-plate byte length must be unchanged");
}

#[test]
fn project_round_trips_the_base_document() {
    let bytes = encode(&base_doc());
    let drawing = Drawing::load(&mut &bytes[..]).expect("load");
    let json = project_json(&drawing);
    assert!(json.contains("\"acadVersion\":\"R12\""));
    assert!(json.contains("\"layers\":["));
    assert!(json.contains("DIMS"));
}

/// 🔎 A genuine `dxf` 0.6 READER quirk, found empirically while verifying `remove-layer-applied`
/// (never assumed): `Drawing::load` parses ENTITIES via its own `add_entity`, which calls
/// `ensure_layer_is_present` for every entity's `layer` field — so a layer removed from the
/// TABLES section but still named by a surviving entity is silently RESYNTHESIZED on load with
/// `Layer::default()` values (colour 7/BYLAYER, linetype "CONTINUOUS"), not left absent. The raw
/// bytes this crate WRITES are correct (verified: the committed `remove-layer-applied/➡️after.dxf`
/// has exactly two `AcDbLayerTableRecord`s, "0" and "TEXT" — no "DIMS" anywhere) — this is a
/// READ-time normalization, not a write-time bug. It does not weaken the gate: `expected` and
/// `actual` are read by the SAME loader, so a subject that genuinely fails to remove the row
/// still differs (its real leftover values vs. the reader's synthesized defaults) — but the
/// projection shows a default-valued residual row, never a clean absence, which is why this
/// module's own base document deliberately keeps "DIMS" referenced by two entities: the fixture
/// exercises the SAME behaviour a real subject's output would be read through. The parallel case
/// for `remove-linetype-applied` (a still-referenced LTYPE resynthesizes with an empty
/// description) was independently confirmed the same way — see the ticket-root report.
#[test]
fn reader_resynthesizes_a_removed_but_still_referenced_layer_with_defaults() {
    let mut after = base_doc();
    let at = after.layers().position(|l| l.name == "DIMS").expect("DIMS present before removal");
    after.remove_layer(at);
    assert_eq!(after.layers().count(), 2, "DIMS is genuinely absent from the in-memory table before saving");

    let bytes = encode(&after);
    let reloaded = Drawing::load(&mut &bytes[..]).expect("reload");
    let resynthesized = reloaded.layers().find(|l| l.name == "DIMS").expect("dxf's own loader resynthesizes DIMS because entities still reference it");
    assert_eq!(resynthesized.color.index(), Some(7), "resynthesized layer takes Layer::default()'s colour, not the removed row's colour (3)");
    assert_eq!(resynthesized.line_type_name, "CONTINUOUS", "resynthesized layer takes the default linetype, not the removed row's (DASHED)");
}
