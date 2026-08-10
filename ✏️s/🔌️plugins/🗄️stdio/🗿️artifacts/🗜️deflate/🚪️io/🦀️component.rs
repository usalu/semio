//! IO stdio.deflate
pub fn register() {
    crate::artifacts::deflate::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::deflate::io::export::serializers::artifacts::binary::register();
}
