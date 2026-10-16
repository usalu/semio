use super::*;

//#region 🔖️Fixtures
fn ui_map(entries: impl IntoIterator<Item = (String, ui_contract::UiValue)>) -> ui_contract::UiMap {
    let mut builder = ui_contract::UiMapBuilder::try_new().expect("fixed map builder");
    for (key, value) in entries {
        builder.push(key, value).expect("ascending fixed map page");
    }
    builder.finish()
}
//#endregion 🔖️Fixtures
//#region ⏭️ResumableCursor
#[test]
fn retained_map_page_advances_each_key_once_without_rewalking_prior_entries() {
    let _guard = testkit::registry_guard();
    let value = ui_contract::UiValue::Map(ui_map([("a".to_owned(), ui_contract::UiValue::Null), ("b".to_owned(), ui_contract::UiValue::Null), ("c".to_owned(), ui_contract::UiValue::Null)]));
    let mut cursor = SurfaceSemanticCensusCursor::default();
    cursor.push_value(&value).expect("fixed value depth");
    let mut steps = 0;
    while cursor.depth > 0 {
        cursor.value_step().expect("retained map cursor progress");
        steps += 1;
        assert!(steps <= 11, "three entries must never rewalk prior pages");
    }
    assert_eq!(steps, 11);
}
#[test]
fn allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation() {
    let _guard = testkit::registry_guard();
    let mut owner = Vec::<u64>::new();
    let mut usage = SurfaceReconcileUsage::default();
    let limits = SurfaceReconcileLimits { max_nodes: 0, max_items: 0, max_bytes: 0, max_identifier_bytes: 0 };
    let fault = admit_vec_backing(&mut owner, &mut usage, limits).expect_err("first backing slot exceeds zero cap");
    assert!(owner.capacity() >= 1, "allocate-inspect retains the exact allocated page on refusal");
    assert!(owner.is_empty(), "no logical item mutates before page admission");
    assert!(matches!(fault, SurfaceReconcileFault::Credits { .. }));

    let actual_capacity = owner.capacity();
    let exact = SurfaceReconcileLimits { max_nodes: 0, max_items: actual_capacity, max_bytes: actual_capacity * size_of::<u64>(), max_identifier_bytes: 0 };
    let mut admitted = Vec::<u64>::new();
    let mut usage = SurfaceReconcileUsage::default();
    admit_vec_backing(&mut admitted, &mut usage, exact).expect("actual inspected cap admits");
    admitted.push(7);
    assert_eq!(admitted, [7]);
}
//#endregion ⏭️ResumableCursor
