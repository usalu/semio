//! 🧪️ Canonical field hashing matches independent Node crypto across arbitrary byte grants.
use super::*;

#[path="🟰️equality.rs"]
mod equality;

#[test]
fn canonical_image_hash_matches_the_independent_neutral_digest_per_byte_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let image: SemioImageSnapshot = semio_framework_pack_json::from_json_str(&fixture["image"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for grant in fixture["grants"].as_array().unwrap() {
        let budget = grant.as_u64().unwrap() as usize;
        let mut work = RasterImageContentHash::new();
        let mut turns = 0;
        loop {
            let progress = work.advance(&image,budget).unwrap();
            assert!(progress.processed_bytes <= budget);
            turns += 1;
            assert!(turns < 10_000);
            if progress.done { break; }
        }
        let bytes = work.finish().unwrap();
        let digest = framework_hash::hex_lower(&bytes);
        assert_eq!(digest,fixture["digest"].as_str().unwrap());
        assert_eq!(raster_image_content_id("probe",&image),format!("probe-{digest}"));
        let key = raster_image_key(bytes);
        assert_eq!(key,format!("image-{digest}"));
        assert_eq!(key.len(),RASTER_IMAGE_KEY_BYTES);
        assert_eq!(key.capacity(),RASTER_IMAGE_KEY_BYTES);
        let child=raster_image_child_id(bytes);
        assert_eq!(child,format!("raster-asset-{digest}"));
        assert_eq!(child.len(),RASTER_IMAGE_CHILD_ID_BYTES);
        assert_eq!(child.capacity(),RASTER_IMAGE_CHILD_ID_BYTES);
    }
}
