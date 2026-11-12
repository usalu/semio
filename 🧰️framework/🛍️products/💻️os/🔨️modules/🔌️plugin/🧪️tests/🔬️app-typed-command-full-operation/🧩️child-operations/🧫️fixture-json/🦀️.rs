//! 🧫️ Committed language-agnostic fixtures of the child-group laws, read at run time so each law crate stays inside its source gate.
use semio_framework_test_fixtures::fixture_path;

/// 📥️ Reads one committed fixture of the full-operation laws (the semantic-text-wire corpus alone is 859 KB) instead of embedding it with `include_str!`.
/// @see <https://doc.rust-lang.org/std/macro.include_str.html>
pub(crate) fn fixture_json(name: &str) -> serde_json::Value {
    let path = fixture_path!("../../../../🧫️fixtures").join(name).join("🔣️.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
}
