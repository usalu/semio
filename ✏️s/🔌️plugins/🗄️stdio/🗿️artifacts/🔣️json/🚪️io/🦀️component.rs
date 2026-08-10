//! IO stdio.json
pub fn register() {
    crate::artifacts::json::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::json::io::export::serializers::artifacts::txt::register();
}
