use super::*;

struct Echo;
impl Engine for Echo {
    const ENGINE_ID: &'static str = "echo";
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> { Ok(input.to_vec()) }
}
struct Replacement;
impl Engine for Replacement {
    const ENGINE_ID: &'static str = "echo";
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> { Ok(input.iter().copied().chain(input.iter().copied()).collect()) }
}

#[test]
fn oversized_output_is_retained_after_all_prior_entries_are_evicted() {
    let mut cache = EngineCache::new(2);
    cache.register(Echo);
    let prior = cache.derive("echo", b"aa").unwrap();
    let oversized = cache.derive("echo", b"large").unwrap();
    assert_eq!(cache.read(&prior), Err(EngineFault::Evicted));
    assert_eq!(cache.read(&oversized).unwrap(), b"large");
    assert_eq!(cache.used_bytes, 5);
    assert_eq!(cache.entries.len(), 1);
}

#[test]
fn registration_replacement_retains_prior_output_and_uses_new_engine_for_new_inputs() {
    let mut cache = EngineCache::new(64);
    cache.register(Echo);
    let prior = cache.derive("echo", b"aa").unwrap();
    cache.register(Replacement);
    assert_eq!(cache.derive("echo", b"aa").unwrap(), prior);
    assert_eq!(cache.read(&prior).unwrap(), b"aa");
    let fresh = cache.derive("echo", b"bb").unwrap();
    assert_eq!(cache.read(&fresh).unwrap(), b"bbbb");
    assert_eq!(cache.entries.len(), 2);
}

#[test]
fn reads_use_key_identity_and_leave_recency_and_stored_bytes_unchanged() {
    let mut cache = EngineCache::new(4);
    cache.register(Echo);
    let a = cache.derive("echo", b"aa").unwrap();
    let b = cache.derive("echo", b"bb").unwrap();
    let foreign = EngineHandle { key: a.key, engine_id: "different-engine".into() };
    let mut detached = cache.read(&foreign).unwrap();
    detached[0] = b'z';
    assert_eq!(cache.read(&a).unwrap(), b"aa");
    assert_eq!(cache.lru.front(), Some(&a.key));
    let c = cache.derive("echo", b"cc").unwrap();
    assert_eq!(cache.read(&a), Err(EngineFault::Evicted));
    assert_eq!(cache.read(&b).unwrap(), b"bb");
    assert_eq!(cache.read(&c).unwrap(), b"cc");
}

struct ModeEngine<const KIND: u8>;
impl<const KIND: u8> Engine for ModeEngine<KIND> {
    const ENGINE_ID: &'static str = match KIND { 0 | 1 => "echo", 2 => "empty", 3 => "bad", _ => "invalid" };
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> {
        match KIND {
            0 => Ok(input.to_vec()),
            1 => Ok(input.iter().copied().chain(input.iter().copied()).collect()),
            2 => Ok(Vec::new()),
            3 => Err(EngineFault::Compute("controlled fault".into())),
            _ => Err(EngineFault::InvalidInput("controlled invalid input".into())),
        }
    }
}

fn input_bytes(text: &str) -> Vec<u8> {
    assert_eq!(text.len() % 2, 0);
    text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

#[test]
fn portable_cache_turns_match_independent_weighted_lru_reference() {
    use serde_json::{json, Value};
    let corpus: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let keys = corpus["keys"].as_array().unwrap();
    let scenarios = corpus["scenarios"].as_array().unwrap();
    assert_eq!(scenarios.len(), 11);
    for scenario in scenarios {
        let mut cache = EngineCache::new(scenario["budgetBytes"].as_u64().unwrap() as usize);
        let mut handles = HashMap::<String, EngineHandle>::new();
        let mut labels = HashMap::<EngineKey, String>::new();
        let mut actual = Vec::new();
        for operation in scenario["operations"].as_array().unwrap() {
            let mut result = match operation["op"].as_str().unwrap() {
                "register" => {
                    match operation["mode"].as_str().unwrap() {
                        "echo" => cache.register(ModeEngine::<0>),
                        "double" => cache.register(ModeEngine::<1>),
                        "empty" => cache.register(ModeEngine::<2>),
                        "compute-fault" => cache.register(ModeEngine::<3>),
                        "invalid-input" => cache.register(ModeEngine::<4>),
                        mode => panic!("unknown fixture engine mode {mode}"),
                    }
                    json!({"kind":"registered"})
                }
                "derive" => {
                    let id = operation["key"].as_str().unwrap();
                    let row = keys.iter().find(|row| row["id"] == id).unwrap();
                    match cache.derive(row["engineId"].as_str().unwrap(), &input_bytes(row["inputHex"].as_str().unwrap())) {
                        Ok(handle) => {
                            labels.insert(handle.key, id.to_owned());
                            handles.insert(operation["handle"].as_str().unwrap().to_owned(), handle);
                            json!({"kind":"derived","key":id})
                        }
                        Err(error) => { let code = match error { EngineFault::UnknownEngine(_) => "UnknownEngine", EngineFault::Compute(_) => "Compute", EngineFault::InvalidInput(_) => "InvalidInput", EngineFault::Evicted => "Evicted" }; json!({"kind":"fault","code":code}) },
                    }
                }
                op @ ("read" | "read-forged" | "read-key") => {
                    let handle = if op == "read-key" {
                        let id = operation["key"].as_str().unwrap();
                        let row = keys.iter().find(|row| row["id"] == id).unwrap();
                        EngineHandle { key: EngineCache::engine_key(row["engineId"].as_str().unwrap(), &input_bytes(row["inputHex"].as_str().unwrap())), engine_id: row["engineId"].as_str().unwrap().into() }
                    } else {
                        let original = handles.get(operation["handle"].as_str().unwrap()).unwrap();
                        EngineHandle { key: original.key, engine_id: if op == "read-forged" { operation["engineId"].as_str().unwrap().into() } else { original.engine_id.clone() } }
                    };
                    match cache.read(&handle) {
                        Ok(value) => json!({"kind":"read","outputHex":value.iter().map(|byte| format!("{byte:02x}")).collect::<String>()}),
                        Err(error) => { assert_eq!(error, EngineFault::Evicted); json!({"kind":"fault","code":"Evicted"}) }
                    }
                }
                op => panic!("unknown fixture operation {op}"),
            };
            result["entries"] = json!(cache.entries.len());
            result["usedBytes"] = json!(cache.used_bytes);
            result["lru"] = json!(cache.lru.iter().map(|key| labels.get(key).unwrap()).collect::<Vec<_>>());
            actual.push(result);
        }
        assert_eq!(json!(actual), scenario["expected"], "{}", scenario["id"]);
    }
}
