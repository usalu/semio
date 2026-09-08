
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

#[semio_framework_async_macros::async_test]
async fn epoch_ticker_starts_and_stops_cleanly_around_a_deadline_bearing_store() {
    let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig::default()).await.expect("engine builds");
    let mut store = Store::new(&engine, ());
    store.set_epoch_deadline(1);
    store.set_fuel(1_000).expect("consume_fuel is enabled on the shared engine");
    // 🧵️ P1f: `EpochTicker` now drives off a `WorkerPool` `Lane::Timer` job, not a dedicated OS
    // thread — a small, deterministic own-pool (never the crate-wide singleton) so this test
    // stays isolated, matching `semio-framework-os-services`' own `test_pool` convention.
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let ticker = EpochTicker::start(&engine, &pool);
    std::thread::sleep(std::time::Duration::from_millis(10));
    drop(ticker);
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
