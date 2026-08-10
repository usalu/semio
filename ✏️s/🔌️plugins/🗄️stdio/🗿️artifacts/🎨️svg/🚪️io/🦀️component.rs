//! IO stdio.svg
pub fn register() {
    crate::artifacts::svg::io::import::deserializers::artifacts::xml::register();
    crate::artifacts::svg::io::export::serializers::artifacts::xml::register();
}
