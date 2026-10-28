//! 🪪️ Schema-neutral compressed IFC GlobalId primitive using system entropy.
pub fn encode_global_id(mut value: u128) -> String {
    let mut encoded = [b'0'; 22];
    for byte in encoded.iter_mut().rev() {
        *byte = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$"[(value & 63) as usize];
        value >>= 6;
    }
    String::from_utf8(encoded.to_vec()).expect("IFC GlobalId alphabet is ASCII")
}
pub fn random_global_id(seed: u64) -> String {
    use std::hash::BuildHasher;
    let instant = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_nanos());
    let random = std::collections::hash_map::RandomState::new();
    let mut value = (u128::from(random.hash_one((instant, seed, 0_u8))) << 64) | u128::from(random.hash_one((instant, seed, 1_u8)));
    value = (value & !(0xf_u128 << 76)) | (4_u128 << 76);
    value = (value & !(3_u128 << 62)) | (2_u128 << 62);
    encode_global_id(value)
}

#[cfg(test)]
#[test]
fn neutral_compressed_identity_vectors() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("🧪️tests/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        assert_eq!(encode_global_id(u128::from_str_radix(row["hex"].as_str().unwrap(), 16).unwrap()), row["expected"].as_str().unwrap());
    }
}

pub fn ifc_entity_identity(id:u64)->String{format!("ifc-{id}")}
pub fn parse_ifc_entity_identity(identity:&str)->Option<u64>{let digits=identity.strip_prefix("ifc-")?;if digits.is_empty()||digits.len()>20||digits.starts_with('0')||!digits.bytes().all(|byte|byte.is_ascii_digit()){return None;}digits.parse().ok().filter(|id|*id>0)}
