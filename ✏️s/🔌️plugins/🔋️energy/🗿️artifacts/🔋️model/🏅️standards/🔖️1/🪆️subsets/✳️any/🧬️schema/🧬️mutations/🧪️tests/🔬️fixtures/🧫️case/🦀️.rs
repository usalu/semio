//! 🧫️ Case of the energy mutation fixtures: the committed-vector shape with its run-time loader and the fixture root. See [`super`].

use crate::mutations::EnergyModelMutation;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_test_fixtures::fixture_path;

/// 🧫️ One committed specification vector and the typed scenario it was generated from.
pub struct Case {
    pub kind: &'static str,
    pub directory: &'static str,
    pub before: String,
    pub after: String,
    pub mutation: String,
    pub diff: String,
    pub outcome: String,
    pub scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation),
}

impl Case {
    /// 🧫️ Reads one committed `(before, mutation, after, diff, outcome)` vector from its fixture bundle at run time; a leaf
    /// not yet materialized reads empty documents, which every law then refuses by name.
    pub fn load(kind: &'static str, directory: &'static str, scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation)) -> Self {
        let bundle = fixture_root().join(directory);
        let read = |leaf: &str| std::fs::read_to_string(bundle.join(leaf)).unwrap_or_default();
        Case {
            kind,
            directory,
            before: read("📸️snapshot/⬅️before/🔣️.json"),
            after: read("📸️snapshot/➡️after/🔣️.json"),
            mutation: read("🦠️mutation/🔣️.json"),
            diff: read("🔺️diff/🔣️.json"),
            outcome: read("🎯️outcome/🔣️.json"),
            scenario,
        }
    }
}

/// 📁️ The committed `🧫️fixtures/🧬️mutations/<domain>/<operation>/<case>` tree, resolved from this crate's manifest.
pub fn fixture_root() -> std::path::PathBuf {
    fixture_path!("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations")
}
