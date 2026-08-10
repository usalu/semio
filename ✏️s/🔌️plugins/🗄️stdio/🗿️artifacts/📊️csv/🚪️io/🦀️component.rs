//! IO stdio.csv
pub fn register() {
    crate::artifacts::csv::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::csv::io::export::serializers::artifacts::txt::register();
}
