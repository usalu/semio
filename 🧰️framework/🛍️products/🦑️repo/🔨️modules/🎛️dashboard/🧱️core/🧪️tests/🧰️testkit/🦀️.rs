//! 🥒️ The test kit of the dashboard crates: the proof that a feature file and the tests that name its scenarios agree.

/// 🥒️ Fails unless the scenarios of a feature are exactly the titles of `proofs`, in order, and every test a title names is a test of one of `sources`.
pub fn assert_proved(feature: &str, sources: &[&str], proofs: &[(&str, &[&str])]) {
    let titles: Vec<&str> = feature.lines().filter_map(|line| line.trim().strip_prefix("Scenario: ")).collect();
    assert_eq!(titles, proofs.iter().map(|(title, _)| *title).collect::<Vec<_>>());
    let is_test = |name: &str| sources.iter().any(|source| source.find(&format!("fn {name}(")).is_some_and(|at| source[..at].trim_end().ends_with("#[test]")));
    for (title, names) in proofs {
        assert!(!names.is_empty(), "{title}: no test proves it");
        for name in *names { assert!(is_test(name), "{title}: {name} is no test"); }
    }
}
