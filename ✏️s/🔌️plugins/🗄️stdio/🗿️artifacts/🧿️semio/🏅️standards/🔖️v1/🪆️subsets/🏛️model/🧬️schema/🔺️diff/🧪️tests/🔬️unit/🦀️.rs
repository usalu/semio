use super::*;

/// 🧪️ field_sweep: `sweep_a`/`sweep_b` differ in EVERY mutable field across all three
/// collections (one removed, one modified-in-every-field, one added each), and exercise the
/// `parent_id`/`spatial_id` tri-states in both directions (Some->None on spatial, None->Some
/// on elements).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();
    let d = SemioModelDiff::between(&a, &b);

    let spatial = d.spatial.as_ref().expect("spatial diff present");
    assert_eq!(spatial.removed, vec!["gone-spatial".to_string()]);
    assert_eq!(spatial.added.len(), 1);
    let keep_spatial = &spatial.modified.iter().find(|m| m.key == "keep-spatial").expect("keep-spatial modified").diff;
    assert!(keep_spatial.kind.is_some() && keep_spatial.name.is_some() && keep_spatial.placement.is_some());
    assert_eq!(keep_spatial.parent_id, Some(None), "Some->None parent_id tri-state must surface as Some(None)");

    let elements = d.elements.as_ref().expect("elements diff present");
    assert_eq!(elements.removed, vec!["gone-element".to_string()]);
    assert_eq!(elements.added.len(), 1);
    let keep_element = &elements.modified.iter().find(|m| m.key == "keep-element").expect("keep-element modified").diff;
    assert!(keep_element.class.is_some() && keep_element.placement.is_some() && keep_element.geometry.is_some() && keep_element.psets.is_some());
    assert_eq!(keep_element.spatial_id, Some(Some("keep-spatial".to_string())), "None->Some spatial_id tri-state must surface");

    let relations = d.relations.as_ref().expect("relations diff present");
    assert_eq!(relations.removed, vec!["gone-relation".to_string()]);
    assert_eq!(relations.added.len(), 1);
    let keep_relation = &relations.modified.iter().find(|m| m.key == "keep-relation").expect("keep-relation modified").diff;
    assert!(keep_relation.kind.is_some() && keep_relation.from.is_some() && keep_relation.to.is_some());

    assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    assert!(SemioModelDiff::between(&a, &a).is_empty());
}

/// 🧪️ between_roundtrip_law: `between(a,b).apply(a) == b` and the symmetric direction.
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(SemioModelDiff::between(&a, &b).apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    assert_eq!(SemioModelDiff::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
}

/// 🧪️ inverse_law: `d.inverse(base).apply(&d.apply(base)) == base`.
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let a = sweep_a();
    let b = sweep_b();
    let d = SemioModelDiff::between(&a, &b);
    let applied = d.apply(&a).expect("apply must succeed for a well-formed fixture");
    let inv = d.inverse(&a);
    assert_eq!(inv.apply(&applied).expect("apply must succeed for a well-formed fixture"), a);
}

/// 🧪️ absorb_law: `absorb(d1,d2).apply(base) == d2.apply(&d1.apply(base))`, including the
/// canonical add-then-remove-before / add-then-set-field cases from schema-design.md.
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = sweep_a();
    let mid = sweep_b();
    let mut after = sweep_b();
    after.elements.push(SemioModelElement { id: "third-element".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
    after.relations.retain(|r| r.id != "new-relation");

    let d1 = SemioModelDiff::between(&base, &mid);
    let d2 = SemioModelDiff::between(&mid, &after);
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");

    let mut absorbed = d1.clone();
    absorbed.absorb(d2.clone());
    assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), after);

    // Canonical case: Insert(X) absorbed with Remove(X) annihilates the add.
    let mut with_add = SemioModelDiff::default();
    with_add.elements =
        Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![SemioModelElement { id: "temp".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] }] });
    let mut with_remove = SemioModelDiff::default();
    with_remove.elements = Some(NamedTripleDiff { removed: vec!["temp".to_string()], modified: vec![], added: vec![] });
    let mut annihilated = with_add.clone();
    annihilated.absorb(with_remove);
    let elements_diff = annihilated.elements.as_ref().expect("elements diff present after annihilation");
    assert!(elements_diff.added.is_empty() && elements_diff.removed.is_empty(), "add-then-remove-before must annihilate cleanly, got {elements_diff:?}");

    // Canonical case: Insert(X) absorbed with Set(X.field) patches into the carried payload.
    let mut with_set = SemioModelDiff::default();
    with_set.elements = Some(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: "temp".to_string(), diff: SemioModelElementDiff { class: Some(ElementClass::Door), ..Default::default() } }], added: vec![] });
    let mut patched_add = with_add;
    patched_add.absorb(with_set);
    let patched = patched_add.elements.as_ref().expect("elements diff present after patch-into-added");
    assert_eq!(patched.added.len(), 1);
    assert_eq!(patched.added[0].class, ElementClass::Door, "add-then-set-field must patch INTO the carried added payload");
}

/// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` text+binary round trip.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let d = SemioModelDiff::between(&a, &b);

    let printed = d.print_diff();
    assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
    let parsed = SemioModelDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
    assert_eq!(parsed, d);

    let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
    let decoded = SemioModelDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
    assert_eq!(decoded, d);

    // Empty diff also round-trips (the common "no change" case every artifact's codec hits).
    let empty = SemioModelDiff::default();
    assert_eq!(empty.print_diff(), "");
    assert_eq!(SemioModelDiff::parse_diff("").unwrap(), empty);
}
