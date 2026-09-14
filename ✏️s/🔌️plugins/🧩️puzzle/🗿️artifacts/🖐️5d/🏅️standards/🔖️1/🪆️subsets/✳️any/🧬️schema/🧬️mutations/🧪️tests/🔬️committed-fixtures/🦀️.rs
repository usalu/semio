//! 🧫️ The puzzle5d committed mutation fixture generator and its canonical-form gate.
//!
//! Every scenario under `🧫️fixtures/🧬️mutations/<leaf>/<scenario>` keeps its authored meaning, while
//! its before-snapshot, after-snapshot, mutation and diff JSON are the owned codec's own canonical
//! encoding (`pack::json::to_string_pretty` over `ToValue`), the after-snapshot is the mutation
//! applied to the before-snapshot and the diff is the one the mutation produces. Off unless
//! `SEMIO_PUZZLE5D_WRITE_FIXTURES=1`, an ordinary run only asserts that every committed file
//! already equals its regenerated form.

use super::{apply_puzzle5d_mutation, Puzzle5dMutation};
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;
use semio_framework_os_kernel::ToValue;

const FIXTURE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");

fn canonical<T: ToValue>(value: &T) -> String {
    format!("{}\n", pack::json::to_string_pretty(&pack::json::from_dsl_value(&value.to_value())))
}

fn regenerated(scenario: &std::path::Path) -> Vec<(std::path::PathBuf, String)> {
    let read = |relative: &str| std::fs::read_to_string(scenario.join(relative)).unwrap_or_else(|error| panic!("{}/{relative}: {error}", scenario.display()));
    let before: Puzzle5dSnapshot = dsl::json::from_json_str(&read("📸️snapshot/⬅️before/🔣️.json")).expect("before snapshot decodes");
    let mutation: Puzzle5dMutation = dsl::json::from_json_str(&read("🦠️mutation/🔣️.json")).expect("mutation decodes");
    let status = serde_json::from_str::<serde_json::Value>(&read("🎯️outcome/🔣️.json")).expect("outcome decodes")["status"].as_str().map(str::to_string);
    assert_eq!(status.as_deref(), Some("applied"), "{}: every puzzle5d mutation scenario is an applied one", scenario.display());
    assert_eq!(serde_json::to_value(&mutation).expect("mutation encodes"), serde_json::from_str::<serde_json::Value>(&canonical(&mutation)).expect("canonical mutation reparses"), "{}: the owned and the oracle mutation encodings disagree", scenario.display());
    let mut after = before.clone();
    apply_puzzle5d_mutation(&mut after, &mutation).expect("mutation applies to its committed before-snapshot");
    let diff: Puzzle5dDiff = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation, &before).diff().clone();
    vec![
        (scenario.join("📸️snapshot/⬅️before/🔣️.json"), canonical(&before)),
        (scenario.join("📸️snapshot/➡️after/🔣️.json"), canonical(&after)),
        (scenario.join("🦠️mutation/🔣️.json"), canonical(&mutation)),
        (scenario.join("🔺️diff/🔣️.json"), canonical(&diff)),
    ]
}

fn scenarios() -> Vec<std::path::PathBuf> {
    let children = |path: &std::path::Path| {
        let mut entries = std::fs::read_dir(path).expect("fixture directory reads").map(|entry| entry.expect("fixture entry reads").path()).filter(|path| path.is_dir()).collect::<Vec<_>>();
        entries.sort();
        entries
    };
    children(std::path::Path::new(FIXTURE_ROOT)).iter().flat_map(|leaf| children(leaf)).filter(|scenario| scenario.join("🦠️mutation/🔣️.json").is_file()).collect()
}

/// 🧫️ Regenerates (with `SEMIO_PUZZLE5D_WRITE_FIXTURES=1`) or gates every committed mutation fixture.
#[test]
fn committed_mutation_fixtures_equal_their_canonical_regeneration() {
    let write = std::env::var("SEMIO_PUZZLE5D_WRITE_FIXTURES").ok().as_deref() == Some("1");
    let scenarios = scenarios();
    assert!(scenarios.len() >= 28, "puzzle5d mutation fixture discovery found only {} scenarios", scenarios.len());
    let mut stale = Vec::new();
    for scenario in &scenarios {
        for (path, text) in regenerated(scenario) {
            let committed = std::fs::read_to_string(&path).expect("committed fixture reads");
            let same = serde_json::from_str::<serde_json::Value>(&committed).expect("committed fixture parses") == serde_json::from_str::<serde_json::Value>(&text).expect("regenerated fixture parses");
            if same {
                continue;
            }
            if write {
                std::fs::write(&path, text).expect("fixture is writable");
            }
            stale.push(path.display().to_string());
        }
    }
    assert!(write || stale.is_empty(), "{} committed puzzle5d mutation fixtures are not canonical; regenerate with SEMIO_PUZZLE5D_WRITE_FIXTURES=1:\n{}", stale.len(), stale.join("\n"));
}
