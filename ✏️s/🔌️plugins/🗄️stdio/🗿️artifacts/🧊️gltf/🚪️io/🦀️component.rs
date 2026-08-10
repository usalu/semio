//! IO stdio.gltf
pub fn register() {
    crate::artifacts::gltf::io::import::deserializers::artifacts::json::register();
    crate::artifacts::gltf::io::export::serializers::artifacts::json::register();
}
