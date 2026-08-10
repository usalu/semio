//! IO stdio.step
pub fn register() {
    crate::artifacts::step::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::step::io::export::serializers::artifacts::txt::register();
}
