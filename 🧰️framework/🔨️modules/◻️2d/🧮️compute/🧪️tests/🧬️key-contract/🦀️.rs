use super::*;
use serde_json::Value;

fn decode_hex(text: &str) -> Vec<u8> {
    assert_eq!(text.len() % 2, 0);
    text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

#[test]
fn canonical_key_vectors_match_independent_blake3_and_distinguish_nul_tuples() {
    let corpus: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let keys = corpus["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 24);
    let reference: Vec<Value> = keys.iter().map(|row| {
        let preimage = decode_hex(row["preimageHex"].as_str().unwrap());
        serde_json::json!({"id":row["id"],"keyHex":blake3::hash(&preimage).to_hex().to_string()})
    }).collect();
    if let Ok(output) = std::env::var("SEMIO_TEST_ARTIFACT_DIR") {
        std::fs::create_dir_all(&output).unwrap();
        std::fs::write(std::path::Path::new(&output).join("compute-key-blake3-oracle.json"), serde_json::to_vec_pretty(&reference).unwrap()).unwrap();
    }
    for row in keys {
        let input = decode_hex(row["inputHex"].as_str().unwrap());
        let preimage = decode_hex(row["preimageHex"].as_str().unwrap());
        let actual = EngineCache::engine_key(row["engineId"].as_str().unwrap(), &input);
        assert_eq!(actual.0, *blake3::hash(&preimage).as_bytes(), "{}", row["id"]);
    }
    for row in corpus["distinctTuples"].as_array().unwrap() {
        let key = |id: &str| {
            let row = keys.iter().find(|row| row["id"] == id).unwrap();
            EngineCache::engine_key(row["engineId"].as_str().unwrap(), &decode_hex(row["inputHex"].as_str().unwrap()))
        };
        assert_ne!(key(row["left"].as_str().unwrap()), key(row["right"].as_str().unwrap()), "{}", row["id"]);
    }
}
