//! IO stdio.glb
//#region Register
pub fn register() {
    crate::artifacts::glb::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::glb::io::import::deserializers::artifacts::json::register();
    crate::artifacts::glb::io::export::serializers::artifacts::binary::register();
    crate::artifacts::glb::io::export::serializers::artifacts::json::register();
}
//#endregion Register
