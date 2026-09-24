use super::*;

#[semio_framework_async_macros::async_test]
async fn build_shared_engine_defaults_to_pooling() {
    let (_engine, pooling_active) = build_shared_engine(SharedEngineConfig::default()).await.expect("pooling engine builds on this host");
    assert!(pooling_active, "pooling allocator should be available in test/CI containers");
}

#[semio_framework_async_macros::async_test]
async fn build_shared_engine_forced_on_demand_reports_on_demand() {
    let cfg = SharedEngineConfig { force_on_demand: true, ..SharedEngineConfig::default() };
    let (_engine, pooling_active) = build_shared_engine(cfg).await.expect("on-demand engine always builds");
    assert!(!pooling_active);
}

/// ⏱️ A core module that never returns: the only way out of `spin` is an epoch interrupt.
const SPIN_FOREVER_WAT: &str = r#"(module (func (export "spin") (loop br 0)))"#;

fn unmetered_engine_and_pool() -> (Engine, WorkerPool) {
    let cfg = SharedEngineConfig { fuel_metering: false, force_on_demand: true, ..SharedEngineConfig::default() };
    let engine = semio_framework_async::block_on(build_shared_engine(cfg)).expect("engine builds").0;
    (engine, WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2)))
}

/// 💤️ With no guest call armed the epoch never advances and no timer is registered: an idle engine
/// costs nothing, however long it exists.
#[test]
fn an_idle_engine_registers_no_timer_and_never_advances() {
    let (engine, pool) = unmetered_engine_and_pool();
    let epoch = EpochDeadlines::new(&engine, &pool);
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert_eq!(epoch.armed_timers(), 0);
    assert_eq!(epoch.advances(), 0);
    pool.shutdown().expect("worker shutdown");
}

/// ⏰️ An armed call is interrupted when its own wall deadline arrives, by exactly one advance —
/// never by a periodic tick before it.
#[test]
fn an_armed_deadline_interrupts_the_guest_with_one_advance() {
    let (engine, pool) = unmetered_engine_and_pool();
    let epoch = EpochDeadlines::new(&engine, &pool);
    let module = wasmtime::Module::new(&engine, SPIN_FOREVER_WAT).expect("spin module");
    let mut store = Store::new(&engine, ());
    let cell = Arc::new(EpochDeadlineCell::default());
    epoch.install(&mut store, Arc::clone(&cell));
    let instance = wasmtime::Instance::new(&mut store, &module, &[]).expect("instantiate spin");
    let spin = instance.get_typed_func::<(), ()>(&mut store, "spin").expect("spin export");
    let started = std::time::Instant::now();
    let demand = epoch.arm(&mut store, &cell, 40);
    let trap = spin.call(&mut store, ()).expect_err("only an interrupt ends spin");
    let elapsed = started.elapsed();
    drop(demand);
    assert!(format!("{trap:?}").to_ascii_lowercase().contains("interrupt"), "an epoch cut, not another trap: {trap:?}");
    assert!(elapsed >= std::time::Duration::from_millis(40), "never before the deadline: {elapsed:?}");
    assert_eq!(epoch.advances(), 1, "the deadline advanced the epoch exactly once");
    assert_eq!(epoch.armed_timers(), 0, "nothing is registered once the only demand is spent");
    pool.shutdown().expect("worker shutdown");
}

/// 🛡️ A far watchdog registers one timer and advances nothing while calls run and return inside it;
/// an advance meant for another store only costs a store whose own deadline is still ahead one
/// callback, never an interrupt.
#[test]
fn a_far_watchdog_costs_one_registration_and_another_stores_deadline_does_not_cut_it() {
    let (engine, pool) = unmetered_engine_and_pool();
    let epoch = EpochDeadlines::new(&engine, &pool);
    let mut watched = Store::new(&engine, ());
    let watched_cell = Arc::new(EpochDeadlineCell::default());
    epoch.install(&mut watched, Arc::clone(&watched_cell));
    let far = epoch.arm(&mut watched, &watched_cell, 30_000);
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert_eq!(epoch.advances(), 0);
    assert_eq!(epoch.armed_timers(), 1);
    let module = wasmtime::Module::new(&engine, SPIN_FOREVER_WAT).expect("spin module");
    let mut spinning = Store::new(&engine, ());
    let spinning_cell = Arc::new(EpochDeadlineCell::default());
    epoch.install(&mut spinning, Arc::clone(&spinning_cell));
    let instance = wasmtime::Instance::new(&mut spinning, &module, &[]).expect("instantiate spin");
    let spin = instance.get_typed_func::<(), ()>(&mut spinning, "spin").expect("spin export");
    let near = epoch.arm(&mut spinning, &spinning_cell, 20);
    assert!(spin.call(&mut spinning, ()).is_err());
    drop(near);
    assert_eq!(epoch.advances(), 1);
    let watched_instance = wasmtime::Instance::new(&mut watched, &module, &[]).expect("instantiate on the watched store");
    let watched_spin = watched_instance.get_typed_func::<(), ()>(&mut watched, "spin").expect("spin export");
    drop(far);
    let short = epoch.arm(&mut watched, &watched_cell, 30);
    let started = std::time::Instant::now();
    assert!(watched_spin.call(&mut watched, ()).is_err());
    assert!(started.elapsed() >= std::time::Duration::from_millis(30), "the watched store ran to its own deadline, not the other store's");
    drop(short);
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn shared_engine_config_hash_is_deterministic_and_config_sensitive() {
    let cfg = SharedEngineConfig::default();
    let a = shared_engine_config_hash(&cfg, true).await;
    let b = shared_engine_config_hash(&cfg, true).await;
    assert_eq!(a, b);
    let c = shared_engine_config_hash(&cfg, false).await;
    assert_ne!(a, c, "pooling vs on-demand must be different cache namespaces");
}

#[semio_framework_async_macros::async_test]
async fn compiled_cache_path_is_namespaced_by_both_hashes() {
    let root = Path::new("/tmp/semio-cache-test");
    let engine_hash = [1u8; 32];
    let package_hash = [2u8; 32];
    let path = compiled_cache_path(root, &engine_hash, &package_hash).await;
    assert!(path.starts_with(root));
    assert!(path.to_string_lossy().ends_with(&format!("{}.cwasm", hex_encode(&package_hash))));
}

#[semio_framework_async_macros::async_test]
async fn compiled_component_round_trips_through_cache_for_a_deterministic_component() {
    let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig::default()).await.expect("engine builds");
    let component = Component::from_binary(&engine, minimal_component_without_actor_world()).expect("compile deterministic component");
    let cache_dir = std::env::temp_dir().join(format!("semio-compiled-cache-test-{}", std::process::id()));
    let cache_path = compiled_cache_path(&cache_dir, &shared_engine_config_hash(&SharedEngineConfig::default(), true).await, &[3u8; 32]).await;
    assert!(load_compiled_component(&engine, &cache_path).await.is_none(), "cache must start empty");
    store_compiled_component(&component, &cache_path).await.expect("write compiled cache entry");
    let restored = load_compiled_component(&engine, &cache_path).await.expect("cache hit after writing");
    drop(restored);
    let _ = std::fs::remove_dir_all(&cache_dir);
}

/// ⛽️ An unmetered engine is its own compiled-code namespace — a metered `.cwasm` never loads into it
/// — the metered default keeps the namespace every existing cache entry already lives in, and a
/// `Store` on it takes no fuel at all, so only its epoch deadline bounds a guest call.
#[semio_framework_async_macros::async_test]
async fn an_unmetered_engine_is_its_own_cache_namespace_and_grants_no_fuel() {
    let metered = SharedEngineConfig::default();
    let unmetered = SharedEngineConfig { fuel_metering: false, ..SharedEngineConfig::default() };
    assert!(metered.fuel_metering, "every host but the gateway keeps fuel metering");
    assert_ne!(shared_engine_config_hash(&metered, true).await, shared_engine_config_hash(&unmetered, true).await);
    let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig { force_on_demand: true, ..unmetered }).await.expect("unmetered engine builds");
    let mut store = Store::new(&engine, ());
    assert!(store.set_fuel(1_000).is_err(), "an unmetered engine has no fuel to grant");
    store.set_epoch_deadline(1);
}
