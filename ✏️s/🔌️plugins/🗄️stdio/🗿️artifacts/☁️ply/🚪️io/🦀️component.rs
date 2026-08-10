//! IO stdio.ply
pub fn register() {
    crate::artifacts::ply::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::ply::io::export::serializers::artifacts::txt::register();
}
