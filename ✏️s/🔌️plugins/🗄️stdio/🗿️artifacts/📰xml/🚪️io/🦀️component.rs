//! IO stdio.xml
pub fn register() {
    crate::artifacts::xml::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::xml::io::export::serializers::artifacts::txt::register();
}
