//! 🧾️ Canonical language-neutral operation wire witness through the independent JSON oracle.
/// 🎟️ Reads the independently authored physical close authority used by original native conformance laws.
pub fn physical_test_close_grant() -> crate::RetainedCloneGrant {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).expect("plain physical close policy");
    let policy=&fixture["physicalCloseGrant"];
    crate::RetainedCloneGrant { maximum_items:policy["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap() as usize }
}
