//! IO stdio.stl
pub fn register() {
    crate::artifacts::stl::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::stl::io::export::serializers::artifacts::binary::register();
}
