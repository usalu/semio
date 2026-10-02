//! ⚙️ Neutral content-addressed computational engine cache and ephemeral representations.
//! Higher host composition registers Engines and supplies EngineHandles to plugins.

use std::collections::{HashMap, VecDeque};
use std::fmt;

//#region 🔖️Keys
/// 🔑 BLAKE3 of UTF-8 identity length, identity bytes, input length and input bytes; lengths are u64 little-endian.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EngineKey(pub [u8; 32]);

/// 🏷️ Opaque handle returned by derive — plugins may store and read, never mint.
#[derive(Clone, Debug, PartialEq, Eq)]
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

/// 🧠 Neutral LRU engine result cache with a byte budget and higher host registration.
///
/// Constructor ownership belongs to this canonical compute module and the retained plugin host.
/// Derived artifact values belong in a `💡️inference` facet keyed by `DepHash`;
/// ephemeral working representations belong in [`EngineRep`], held for the body of their constructor.
/// `policyDissolvedEngineCacheScopeBreaches` enforces constructor ownership.
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
        let mut hasher = semio_framework_hash::Hasher::new();
        hasher.update(&(engine_id.len() as u64).to_le_bytes());
        hasher.update(engine_id.as_bytes());
        hasher.update(&(input.len() as u64).to_le_bytes());
        hasher.update(input);
        EngineKey(*hasher.finalize().as_bytes())
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧬️cache-contract/🦀️.rs"]
mod cache_contract;

#[cfg(test)]
#[path = "🧪️tests/🧬️key-contract/🦀️.rs"]
mod key_contract;
