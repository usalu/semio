//! ⚙️ Host-owned content-addressed computational engine cache.
//! Kernels register as Engines; plugins only hold EngineHandles — never their own registries.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt;

//#region 🔖️Keys
/// 🔑 Content-addressed cache key — blake3 of `(engine_id, 0, input)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EngineKey(pub [u8; 32]);

/// 🏷️ Opaque handle returned by derive — plugins may store and read, never mint.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineHandle {
    pub key: EngineKey,
    pub engine_id: String,
}
//#endregion 🔖️Keys

//#region 🔖️Faults
/// 💥 Engine derive/read failures.
#[derive(Debug, PartialEq, Eq)]
pub enum EngineFault {
    UnknownEngine(String),
    Compute(String),
    Evicted,
    InvalidInput(String),
}

impl fmt::Display for EngineFault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownEngine(engine) => write!(formatter, "unknown engine: {engine}"),
            Self::Compute(message) => write!(formatter, "compute failed: {message}"),
            Self::Evicted => formatter.write_str("cache miss: handle evicted"),
            Self::InvalidInput(message) => write!(formatter, "invalid input: {message}"),
        }
    }
}

impl std::error::Error for EngineFault {}
//#endregion 🔖️Faults

//#region 🔖️Engine
/// ⚙️ Host-registered pure compute kernel. Plugins never own registries — only handles.
pub trait Engine: Send + Sync + 'static {
    const ENGINE_ID: &'static str;
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault>;
}

/// 🧩 Dyn-compatible compute surface — `Engine` itself is not object-safe (associated const).
trait DynEngine: Send + Sync {
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault>;
}

impl<E: Engine> DynEngine for E {
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> {
        Engine::compute(self, input)
    }
}

/// 🧺 Opaque bag of handles an app may read during handle()/render — populated by host.
pub struct EngineHandles {
    pub handles: Vec<EngineHandle>,
}

impl EngineHandles {
    /// 🫙 Empty handle bag for apps with no pending engine results.
    pub fn empty() -> Self {
        Self { handles: Vec::new() }
    }
}
//#endregion 🔖️Engine

//#region 🔖️Cache
struct CacheEntry {
    output: Vec<u8>,
    byte_len: usize,
}

/// 🧠 Host-owned LRU engine result cache with a byte budget.
///
/// ⚠️ Scope is narrowing to the wasm guest↔host boundary only (the `engine-derive`/`engine-read`
/// imports), where byte serialization is unavoidable regardless of doctrine. It is no longer a
/// general "kernel cache": a kernel that caches derived values outside an artifact facet is state
/// living outside the store. Derived values belong in a `💡️inference` facet keyed by `DepHash`;
/// ephemeral working representations belong in [`EngineRep`], held only for the body of the
/// function that built them. `policyEngineCacheScopeBreaches` enforces the narrowed contract.
pub struct EngineCache {
    engines: HashMap<String, Box<dyn DynEngine>>,
    entries: HashMap<EngineKey, CacheEntry>,
    lru: VecDeque<EngineKey>,
    budget_bytes: usize,
    used_bytes: usize,
}

impl EngineCache {
    /// 🏗️ Empty cache with the given byte budget for stored outputs.
    pub fn new(budget_bytes: usize) -> Self {
        Self { engines: HashMap::new(), entries: HashMap::new(), lru: VecDeque::new(), budget_bytes, used_bytes: 0 }
    }

    /// 📎 Register a kernel under its `ENGINE_ID` (replaces any prior registration).
    pub fn register<E: Engine>(&mut self, engine: E) {
        self.engines.insert(E::ENGINE_ID.to_string(), Box::new(engine));
    }

    /// 🔐 Content-addressed key for `(engine_id, input)`.
    pub fn engine_key(engine_id: &str, input: &[u8]) -> EngineKey {
        let mut data = engine_id.as_bytes().to_vec();
        data.push(0);
        data.extend_from_slice(input);
        EngineKey(*semio_framework_hash::hash(&data).as_bytes())
    }

    /// 🧮 Compute (or hit-cache) and return a content-addressed handle.
    pub fn derive(&mut self, engine_id: &str, input: &[u8]) -> Result<EngineHandle, EngineFault> {
        let key = Self::engine_key(engine_id, input);
        if self.entries.contains_key(&key) {
            self.touch(key);
            return Ok(EngineHandle { key, engine_id: engine_id.to_string() });
        }
        let output = {
            let engine = self.engines.get(engine_id).ok_or_else(|| EngineFault::UnknownEngine(engine_id.to_string()))?;
            engine.compute(input)?
        };
        let byte_len = output.len();
        self.ensure_budget(byte_len);
        self.entries.insert(key, CacheEntry { output, byte_len });
        self.lru.push_back(key);
        self.used_bytes = self.used_bytes.saturating_add(byte_len);
        Ok(EngineHandle { key, engine_id: engine_id.to_string() })
    }

    /// 📖 Read a previously derived output; fails if the entry was LRU-evicted.
    pub fn read(&self, handle: &EngineHandle) -> Result<Vec<u8>, EngineFault> {
        self.entries.get(&handle.key).map(|entry| entry.output.clone()).ok_or(EngineFault::Evicted)
    }

    fn touch(&mut self, key: EngineKey) {
        if let Some(pos) = self.lru.iter().position(|k| *k == key) {
            self.lru.remove(pos);
        }
        self.lru.push_back(key);
    }

    fn ensure_budget(&mut self, needed: usize) {
        while self.used_bytes.saturating_add(needed) > self.budget_bytes {
            let Some(old) = self.lru.pop_front() else {
                break;
            };
            if let Some(entry) = self.entries.remove(&old) {
                self.used_bytes = self.used_bytes.saturating_sub(entry.byte_len);
            }
        }
    }
}
//#endregion 🔖️Cache

//#region 🔖️EngineRep
/// 🧱 A pure, ephemeral, snapshot-derived working representation — halfedge adjacency, a BVH, a
/// brep topology arena, a tessellation buffer.
///
/// Contract:
/// - Built ONLY inside a `🔺️diff` constructor or an `InferredField::{plan,dep_input,compute}` body.
/// - Dropped when that function returns. Never a durable struct field, never `thread_local!`, never
///   carried across a mutation-dispatch boundary.
/// - Deterministic: `build(s)` equals `build(s)` for byte-identical `s`.
/// - Wholly derived: everything it holds is recomputable from the snapshot alone.
///
/// [`build`](EngineRep::build) is deliberately the ONLY constructor. There is no incremental or
/// seeded variant, because a representation grown from a previous representation is no longer
/// recoverable from the snapshot — which is exactly how a cache becomes hidden authoritative state.
pub trait EngineRep<P>: Sized {
    fn build(snapshot: &P) -> Self;
}
//#endregion 🔖️EngineRep

#[cfg(test)]
mod tests {
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
}
