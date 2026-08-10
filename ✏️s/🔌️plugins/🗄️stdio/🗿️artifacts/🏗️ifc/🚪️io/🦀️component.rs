//! IO stdio.ifc
pub fn register() {
    crate::artifacts::ifc::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::ifc::io::export::serializers::artifacts::txt::register();
}
