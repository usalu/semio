//! IO stdio.zip
pub fn register() {
    crate::artifacts::zip::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::zip::io::import::deserializers::artifacts::deflate::register();
    crate::artifacts::zip::io::export::serializers::artifacts::binary::register();
    crate::artifacts::zip::io::export::serializers::artifacts::deflate::register();
}
