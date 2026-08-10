//! IO stdio.txt
pub fn register() {
    crate::artifacts::txt::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::txt::io::export::serializers::artifacts::binary::register();
}
