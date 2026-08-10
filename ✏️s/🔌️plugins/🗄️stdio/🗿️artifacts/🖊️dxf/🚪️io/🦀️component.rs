//! IO stdio.dxf
pub fn register() {
    crate::artifacts::dxf::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::dxf::io::export::serializers::artifacts::txt::register();
}
