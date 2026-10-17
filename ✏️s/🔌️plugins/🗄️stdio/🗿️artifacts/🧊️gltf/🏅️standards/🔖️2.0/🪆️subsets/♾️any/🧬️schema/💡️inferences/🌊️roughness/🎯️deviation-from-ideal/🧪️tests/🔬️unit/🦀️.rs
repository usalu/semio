use crate::standards::v2_0::subsets::any::schema::inferences::roughness::deviation_from_ideal::*;

#[semio_framework_async_macros::async_test]
async fn descriptor_is_versioned_and_cacheable() {
    assert_eq!(descriptor().id, "s.stdio.gltf.inference.deviation-from-ideal.v1");
    assert_eq!(descriptor().algorithm_version, 1);
}
