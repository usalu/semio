//! IO stdio.md
pub fn register() {
    crate::artifacts::md::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::md::io::export::serializers::artifacts::txt::register();
}
