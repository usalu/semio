//! 🧫️ The puzzle3d committed mutation fixture generator and its canonical-form gate.
//!
//! Every scenario under `🧫️fixtures/🧬️mutations/<family>/<leaf>/<scenario>` keeps its authored meaning, while
//! its before-snapshot, after-snapshot, mutation and diff JSON are the owned codec's own canonical
//! encoding (`pack::json::to_string_pretty` over `ToValue`), the after-snapshot is the mutation
//! applied to the before-snapshot and the diff is the one the mutation produces. Off unless
//! `SEMIO_PUZZLE3D_WRITE_FIXTURES=1`, an ordinary run only asserts that every committed file
//! already equals its regenerated form.

use super::Puzzle3dMutation;
use crate::apply_puzzle3d_mutation;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use semio_framework_value::ToValue;

const FIXTURE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations");

fn canonical<T: ToValue>(value: &T) -> String {
    format!("{}\n", semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value.to_value())))
}

fn regenerated(scenario: &std::path::Path) -> Vec<(std::path::PathBuf, String)> {
    let read = |relative: &str| std::fs::read_to_string(scenario.join(relative)).unwrap_or_else(|error| panic!("{}/{relative}: {error}", scenario.display()));
    let before: Puzzle3dSnapshot = semio_framework_pack_json::from_json_str(&read("📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let mutation: Puzzle3dMutation = semio_framework_pack_json::from_json_str(&read("🦠️mutation/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes");
    let status = serde_json::from_str::<serde_json::Value>(&read("🎯️outcome/🔣️.json")).expect("outcome decodes")["status"].as_str().map(str::to_string);
    if status.as_deref() == Some("rejected") {
        let mut after = before.clone();
        let _ = apply_puzzle3d_mutation(&mut after, &mutation);
        assert_eq!(after, before, "{}: a REJECTED scenario leaves its own before-snapshot untouched", scenario.display());
        assert!(scenario.join("🔺️diff/🚫️.absent").is_file(), "{}: a REJECTED scenario states the absent-diff sentinel instead of a diff", scenario.display());
        return vec![
            (scenario.join("📸️snapshot/⬅️before/🔣️.json"), canonical(&before)),
            (scenario.join("📸️snapshot/➡️after/🔣️.json"), canonical(&before)),
            (scenario.join("🦠️mutation/🔣️.json"), canonical(&mutation)),
        ];
    }
    assert!(matches!(status.as_deref(), Some("applied" | "no-op")), "{}: a puzzle3d mutation scenario is applied, no-op or rejected", scenario.display());
    assert_eq!(serde_json::to_value(&mutation).expect("mutation encodes"), serde_json::from_str::<serde_json::Value>(&canonical(&mutation)).expect("canonical mutation reparses"), "{}: the owned and the oracle mutation encodings disagree", scenario.display());
    let mut after = before.clone();
    apply_puzzle3d_mutation(&mut after, &mutation).expect("mutation applies to its committed before-snapshot");
    let diff: Puzzle3dDiff = <Puzzle3dMutation as protocol::ColdMutation<Puzzle3dSnapshot>>::diff(&mutation, &before).diff().clone();
    if status.as_deref() == Some("no-op") {
        assert_eq!(after, before, "{}: a NO-OP scenario leaves its own before-snapshot untouched", scenario.display());
    }
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
    children(std::path::Path::new(FIXTURE_ROOT)).iter().flat_map(|family| children(family)).flat_map(|leaf| children(&leaf)).filter(|scenario| scenario.join("🦠️mutation/🔣️.json").is_file()).collect()
}

/// 🧫️ Regenerates (with `SEMIO_PUZZLE3D_WRITE_FIXTURES=1`) or gates every committed mutation fixture.
#[test]
fn committed_mutation_fixtures_equal_their_canonical_regeneration() {
    let write = std::env::var("SEMIO_PUZZLE3D_WRITE_FIXTURES").ok().as_deref() == Some("1");
    let scenarios = scenarios();
    assert!(scenarios.len() >= 28, "puzzle3d mutation fixture discovery found only {} scenarios", scenarios.len());
    let mut stale = Vec::new();
    for scenario in &scenarios {
        for (path, text) in regenerated(scenario) {
            let committed = std::fs::read_to_string(&path).expect("committed fixture reads");
            let same = serde_json::from_str::<serde_json::Value>(&committed).expect("committed fixture parses") == serde_json::from_str::<serde_json::Value>(&text).expect("regenerated fixture parses");
            if same {
                continue;
            }
            if write {
                std::fs::write(&path, &text).expect("fixture is writable");
            }
            stale.push(format!("{}\ncommitted={}\nactual={}", path.display(), serde_json::to_string(&committed).expect("committed JSON text"), serde_json::to_string(&text).expect("actual JSON text")));
        }
    }
    assert!(write || stale.is_empty(), "{} committed puzzle3d mutation fixtures are not canonical; regenerate with SEMIO_PUZZLE3D_WRITE_FIXTURES=1:\n{}", stale.len(), stale.join("\n"));
}

/// 🔢️ Exact decimal inputs preserve the native binary64 value through both JSON readers.
#[test]
fn decimal_fixture_inputs_preserve_native_binary64_bits() {
    let cases: Vec<(String, String)> = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("decimal corpus decodes");
    let mut mismatches = Vec::new();
    for (text, expected) in cases {
        let bits = u64::from_str_radix(&expected, 16).expect("binary64 bits decode");
        let native = text.parse::<f64>().expect("native decimal parses");
        let owned: f64 = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned JSON decimal parses");
        let oracle: f64 = serde_json::from_str(&text).expect("oracle JSON decimal parses");
        assert_eq!(native.to_bits(), bits, "native parser: {text}");
        assert_eq!(owned.to_bits(), bits, "owned parser: {text}");
        if oracle.to_bits() != bits {
            mismatches.push(format!("{text}: expected={bits:016x} native={:016x} owned={:016x} oracle={:016x}", native.to_bits(), owned.to_bits(), oracle.to_bits()));
        }
    }
    assert!(mismatches.is_empty(), "third-party decimal parser differs from exact native bits:\n{}", mismatches.join("\n"));
}
