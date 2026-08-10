//! IO stdio.dwg
pub fn register() {
    crate::artifacts::dwg::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::dwg::io::export::serializers::artifacts::binary::register();
}
