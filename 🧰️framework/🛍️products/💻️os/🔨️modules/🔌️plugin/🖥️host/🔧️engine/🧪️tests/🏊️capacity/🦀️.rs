//! 🏊️ Native pool admission uses the same finite table ceiling as the Store.

use super::{BudgetLimiter, MAXIMUM_GUEST_TABLE_ELEMENTS, SharedEngineConfig, build_shared_engine};
use wasmtime::component::{Component, Linker};
use wasmtime::{ResourceLimiter, Store};

fn static_component(table_elements: usize, defined_globals: usize) -> String {
    let globals = "(global i64 (i64.const 0))".repeat(defined_globals);
    format!("(component (core module $m (table {table_elements} funcref) {globals}) (core instance $i (instantiate $m)))")
}

#[test]
fn pooled_guest_admission_covers_the_shipped_draw_capacity_with_a_finite_store_bound() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let value = |key: &str| usize::try_from(fixture[key].as_u64().unwrap()).unwrap();
    let table_elements = value("tableElements");
    let defined_globals = value("definedGlobals");
    let maximum_table_elements = value("maximumTableElements");
    let refused_table_elements = value("refusedTableElements");
    let config = SharedEngineConfig { total_component_instances: 2, ..SharedEngineConfig::default() };
    let (engine, pooling) = semio_framework_async::block_on(build_shared_engine(config)).unwrap();
    assert!(pooling, "the native allocation law runs on the ordinary pool");
    assert!(!engine.get_parallel_compilation(), "cold guest code generation uses one compiler thread");
    let component = Component::new(&engine, static_component(table_elements, defined_globals).as_bytes())
        .expect("Draw-sized table and VM context fit before instantiation");
    let mut store = Store::new(&engine, BudgetLimiter::default());
    store.limiter(|state| state);
    store.set_fuel(10_000_000).unwrap();
    store.set_epoch_deadline(1);
    let linker = Linker::new(&engine);
    semio_framework_async::block_on(linker.instantiate_async(&mut store, &component))
        .expect("the per-store bound admits the same Draw-sized table");

    assert_eq!(MAXIMUM_GUEST_TABLE_ELEMENTS as usize, maximum_table_elements);
    let mut limiter = BudgetLimiter::default();
    assert!(limiter.table_growing(table_elements, maximum_table_elements, None).unwrap());
    assert!(!limiter.table_growing(maximum_table_elements, refused_table_elements, None).unwrap());
    let refusal = Component::new(&engine, static_component(refused_table_elements, 0).as_bytes())
        .err()
        .expect("the pool also refuses a table above the ceiling");
    assert!(format!("{refusal:#}").contains("module table does not fit in pooling allocator requirements"));
}
