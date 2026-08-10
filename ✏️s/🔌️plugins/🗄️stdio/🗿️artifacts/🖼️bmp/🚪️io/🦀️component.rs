//! IO stdio.bmp
pub fn register() {
    crate::artifacts::bmp::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::bmp::io::export::serializers::artifacts::binary::register();
}
