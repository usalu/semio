use super::*;

struct EchoEngine;

impl Engine for EchoEngine {
    const ENGINE_ID: &'static str = "echo";

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> {
        Ok(input.to_vec())
    }
}

#[test]
fn register_and_derive_echoes_input() {
    let mut cache = EngineCache::new(1024);
    cache.register(EchoEngine);
    let handle = cache.derive("echo", b"hello").expect("derive");
    assert_eq!(handle.engine_id, "echo");
    assert_eq!(cache.read(&handle).expect("read"), b"hello");
}

#[test]
fn derive_twice_same_key_is_cache_hit() {
    let mut cache = EngineCache::new(1024);
    cache.register(EchoEngine);
    let first = cache.derive("echo", b"same").expect("first");
    let second = cache.derive("echo", b"same").expect("second");
    assert_eq!(first.key, second.key);
    assert_eq!(first, second);
    assert_eq!(cache.entries.len(), 1);
    assert_eq!(cache.used_bytes, b"same".len());
}

#[test]
fn eviction_when_budget_exceeded() {
    let mut cache = EngineCache::new(4);
    cache.register(EchoEngine);
    let keep = cache.derive("echo", b"abcd").expect("keep");
    let _ = cache.derive("echo", b"wxyz").expect("evictor");
    assert_eq!(cache.read(&keep), Err(EngineFault::Evicted));
    let survivor = cache.derive("echo", b"wxyz").expect("survivor");
    assert_eq!(cache.read(&survivor).expect("read survivor"), b"wxyz");
    assert_eq!(cache.entries.len(), 1);
    assert_eq!(cache.used_bytes, 4);
}

#[test]
fn unknown_engine_fault() {
    let mut cache = EngineCache::new(64);
    assert_eq!(cache.derive("missing", b"x"), Err(EngineFault::UnknownEngine("missing".into())));
}

#[test]
fn engine_key_is_stable() {
    let a = EngineCache::engine_key("echo", b"payload");
    let b = EngineCache::engine_key("echo", b"payload");
    let c = EngineCache::engine_key("echo", b"other");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[derive(Clone)]
struct CountSnapshot {
    values: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct SumRep {
    total: u32,
    len: usize,
}

impl EngineRep<CountSnapshot> for SumRep {
    fn build(snapshot: &CountSnapshot) -> Self {
        Self { total: snapshot.values.iter().copied().sum(), len: snapshot.values.len() }
    }
}

#[test]
fn engine_rep_build_is_deterministic() {
    let snapshot = CountSnapshot { values: vec![1, 2, 3] };
    assert_eq!(SumRep::build(&snapshot), SumRep::build(&snapshot));
}
