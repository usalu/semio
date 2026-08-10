//! dag IO stdio matrix
pub fn register() {
    crate::artifacts::dag::io::import::deserializers::artifacts::csv::register();
    crate::artifacts::dag::io::export::serializers::artifacts::csv::register();
    crate::artifacts::dag::io::import::deserializers::artifacts::json::register();
    crate::artifacts::dag::io::export::serializers::artifacts::json::register();
    crate::artifacts::dag::io::import::deserializers::artifacts::md::register();
    crate::artifacts::dag::io::export::serializers::artifacts::md::register();
    crate::artifacts::dag::io::import::deserializers::artifacts::png::register();
    crate::artifacts::dag::io::export::serializers::artifacts::png::register();
    crate::artifacts::dag::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::dag::io::export::serializers::artifacts::svg::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"] }
pub fn dag_to_wire(from: &crate::artifacts::dag::DagSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(from)
}
pub fn dag_from_wire(bytes: &[u8]) -> Result<crate::artifacts::dag::DagSnapshot, store::PackError> {
    <crate::artifacts::dag::DagSnapshot as store::DocumentPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
