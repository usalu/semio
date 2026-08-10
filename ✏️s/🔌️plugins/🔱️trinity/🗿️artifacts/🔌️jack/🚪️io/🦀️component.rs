//! jack IO stdio matrix
pub fn register() {
    crate::artifacts::jack::io::import::deserializers::artifacts::csv::register();
    crate::artifacts::jack::io::import::deserializers::artifacts::json::register();
    crate::artifacts::jack::io::import::deserializers::artifacts::md::register();
    crate::artifacts::jack::io::import::deserializers::artifacts::png::register();
    crate::artifacts::jack::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::jack::io::export::serializers::artifacts::csv::register();
    crate::artifacts::jack::io::export::serializers::artifacts::json::register();
    crate::artifacts::jack::io::export::serializers::artifacts::md::register();
    crate::artifacts::jack::io::export::serializers::artifacts::png::register();
    crate::artifacts::jack::io::export::serializers::artifacts::svg::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"]
}
