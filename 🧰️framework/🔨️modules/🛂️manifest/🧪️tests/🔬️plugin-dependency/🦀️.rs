
//! 🔗️ Ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS
//! lane W0-C: `Version`/`VersionPin` parse+match matrix, dependency-graph toposort/cycle/
//! validation, and manifest serde round-trips (absent-field defaults included).
use super::*;

async fn manifest(plugin_id: &str, version: &str, dependencies: Vec<PluginDependency>) -> PluginManifest {
    PluginManifest {
        plugin_id: plugin_id.into(),
        label: plugin_id.into(),
        version: version.into(),
        apps: Vec::new(),
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies,
        contributions: Vec::new(),
    }
}

//#region 🔖️VersionAndVersionPin
fn pin(major: u64, minor: u64, patch: u64) -> VersionPin {
    VersionPin(Version::new(major, minor, patch))
}

#[semio_framework_async_macros::async_test]
async fn version_parses_valid_triples_and_rejects_malformed_input() {
    assert_eq!(Version::parse("1.2.3").unwrap(), Version::new(1, 2, 3));
    assert_eq!(Version::parse("0.0.0").unwrap(), Version::new(0, 0, 0));
    assert!(matches!(Version::parse("1.2").unwrap_err(), VersionParseError::Malformed(_)));
    assert!(matches!(Version::parse("1.2.3.4").unwrap_err(), VersionParseError::Malformed(_)));
    assert!(matches!(Version::parse("1.x.3").unwrap_err(), VersionParseError::NonNumeric(_, seg) if seg == "x"));
    assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
}

#[semio_framework_async_macros::async_test]
async fn version_ord_matches_semver_precedence() {
    assert!(Version::new(1, 0, 0) < Version::new(1, 0, 1));
    assert!(Version::new(1, 0, 0) < Version::new(1, 1, 0));
    assert!(Version::new(1, 0, 0) < Version::new(2, 0, 0));
    assert!(Version::new(1, 9, 9) < Version::new(2, 0, 0));
}

#[semio_framework_async_macros::async_test]
async fn version_pin_parses_only_the_exact_form_and_refuses_every_range() {
    assert_eq!(VersionPin::parse("=1.2.3").unwrap(), pin(1, 2, 3));
    assert_eq!(VersionPin::parse(" =0.1.0 ").unwrap(), pin(0, 1, 0));
    for range in ["*", "^1.2.3", "~1.2.3", ">=1.2.3", "1.2.3", ""] {
        assert!(matches!(VersionPin::parse(range).unwrap_err(), VersionPinParseError::NotExact(_)), "{range:?} must be refused");
    }
    assert!(matches!(VersionPin::parse("=1.x.3").unwrap_err(), VersionPinParseError::Version(_)));
}

#[semio_framework_async_macros::async_test]
async fn version_pin_display_round_trips_through_parse() {
    for raw in ["=0.1.0", "=1.2.3"] {
        let parsed = VersionPin::parse(raw).unwrap();
        assert_eq!(parsed.to_string(), raw);
        assert_eq!(VersionPin::parse(&parsed.to_string()).unwrap(), parsed);
    }
}

#[semio_framework_async_macros::async_test]
async fn version_pin_matches_only_its_exact_version() {
    let exact = pin(1, 2, 3);
    assert!(exact.matches(&Version::new(1, 2, 3)));
    for other in [Version::new(1, 2, 4), Version::new(1, 2, 2), Version::new(1, 3, 0), Version::new(2, 0, 0), Version::new(0, 0, 0)] {
        assert!(!exact.matches(&other), "{exact} must not match {other}");
    }
    assert!(exact.matches_raw("1.2.3"));
    assert!(!exact.matches_raw("1.2"));
    assert!(!exact.matches_raw("not-a-version"));
}

#[semio_framework_async_macros::async_test]
async fn version_pin_of_tree_accepts_only_a_strict_triple() {
    assert_eq!(VersionPin::of_tree("0.1.0"), pin(0, 1, 0));
    assert_eq!(VersionPin::of_tree("12.30.4"), pin(12, 30, 4));
    for malformed in ["0.1", "0.1.0.1", "0.1.0-alpha", "", ".1.0", "0..0"] {
        assert!(std::panic::catch_unwind(|| VersionPin::of_tree(malformed)).is_err(), "{malformed:?} must not pin");
    }
    assert_eq!(crate::tree_pin!(), VersionPin::parse(concat!("=", env!("CARGO_PKG_VERSION"))).unwrap());
}

#[semio_framework_async_macros::async_test]
async fn plugin_dependency_serde_round_trips_as_an_exact_pin_string() {
    let dependency = PluginDependency::new("cad", pin(1, 0, 0));
    let json = serde_json::to_value(&dependency).unwrap();
    assert_eq!(json, serde_json::json!({ "pluginId": "cad", "version": "=1.0.0" }));
    let round_tripped: PluginDependency = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, dependency);
    assert_eq!(<PluginDependency as FromValue>::from_value(dependency.to_value()).unwrap(), dependency);
}

#[semio_framework_async_macros::async_test]
async fn a_range_dependency_is_refused_on_every_decode_path() {
    for range in ["*", "^0.1.0", "~0.1.0", ">=0.1.0", "0.1.0"] {
        assert!(serde_json::from_value::<PluginDependency>(serde_json::json!({ "pluginId": "cad", "version": range })).is_err(), "serde must refuse {range:?}");
        let value = DslValue::object([("pluginId".to_string(), DslValue::String("cad".into())), ("version".to_string(), DslValue::String(range.into()))]);
        assert!(<PluginDependency as FromValue>::from_value(value).is_err(), "the descriptor codec must refuse {range:?}");
    }
}
//#endregion 🔖️VersionAndVersionPin

//#region 🔖️DependencyGraphTests
#[semio_framework_async_macros::async_test]
async fn resolve_load_order_toposorts_a_diamond() {
    // base <- {left, right} <- top: two valid topological orders exist; the tie-break must
    // deterministically pick `left` before `right`.
    let manifests = vec![
        manifest("top", "1.0.0", vec![PluginDependency::new("left", pin(1, 0, 0)), PluginDependency::new("right", pin(1, 0, 0))]).await,
        manifest("left", "1.0.0", vec![PluginDependency::new("base", pin(1, 0, 0))]).await,
        manifest("right", "1.0.0", vec![PluginDependency::new("base", pin(1, 0, 0))]).await,
        manifest("base", "1.0.0", vec![]).await,
    ];
    let order = resolve_load_order(&manifests).unwrap();
    assert_eq!(order, vec!["base", "left", "right", "top"]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_is_deterministic_regardless_of_input_order() {
    let forward = vec![manifest("a", "1.0.0", vec![]).await, manifest("b", "1.0.0", vec![PluginDependency::new("a", pin(1, 0, 0))]).await, manifest("c", "1.0.0", vec![PluginDependency::new("a", pin(1, 0, 0))]).await];
    let mut shuffled = forward.clone();
    shuffled.reverse();
    assert_eq!(resolve_load_order(&forward).unwrap(), resolve_load_order(&shuffled).unwrap());
    assert_eq!(resolve_load_order(&forward).unwrap(), vec!["a", "b", "c"]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_reports_missing_dependency() {
    let manifests = vec![manifest("a", "1.0.0", vec![PluginDependency::new("ghost", pin(1, 0, 0))]).await];
    let error = resolve_load_order(&manifests).unwrap_err();
    assert_eq!(error, DependencyGraphError::MissingDependency { plugin_id: "a".into(), depends_on: "ghost".into() });
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_reports_version_mismatch() {
    let manifests = vec![manifest("a", "1.0.0", vec![PluginDependency::new("b", pin(2, 0, 0))]).await, manifest("b", "1.0.0", vec![]).await];
    let error = resolve_load_order(&manifests).unwrap_err();
    assert_eq!(error, DependencyGraphError::VersionMismatch { plugin_id: "a".into(), depends_on: "b".into(), required: "=2.0.0".into(), actual: "1.0.0".into() });
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_names_every_member_of_a_cycle() {
    let manifests = vec![
        manifest("a", "1.0.0", vec![PluginDependency::new("b", pin(1, 0, 0))]).await,
        manifest("b", "1.0.0", vec![PluginDependency::new("c", pin(1, 0, 0))]).await,
        manifest("c", "1.0.0", vec![PluginDependency::new("a", pin(1, 0, 0))]).await,
    ];
    let error = resolve_load_order(&manifests).unwrap_err();
    match error {
        DependencyGraphError::Cycle { members } => {
            let mut sorted = members.clone();
            sorted.sort();
            assert_eq!(sorted, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            assert_eq!(members.len(), 3, "every plugin on the 3-cycle must be named");
        }
        other => panic!("expected a Cycle error, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_accepts_a_self_satisfying_empty_graph() {
    assert_eq!(resolve_load_order(&[]).unwrap(), Vec::<String>::new());
}

#[semio_framework_async_macros::async_test]
async fn dependents_returns_direct_dependents_sorted() {
    let manifests = vec![
        manifest("a", "1.0.0", vec![]).await,
        manifest("b", "1.0.0", vec![PluginDependency::new("a", pin(1, 0, 0))]).await,
        manifest("c", "1.0.0", vec![PluginDependency::new("a", pin(1, 0, 0))]).await,
        manifest("d", "1.0.0", vec![PluginDependency::new("b", pin(1, 0, 0))]).await,
    ];
    assert_eq!(dependents(&manifests, "a"), vec!["b".to_string(), "c".to_string()]);
    assert_eq!(dependents(&manifests, "b"), vec!["d".to_string()]);
    assert!(dependents(&manifests, "d").is_empty());
}
//#endregion 🔖️DependencyGraphTests

//#region 🔖️ManifestSerdeTests
#[semio_framework_async_macros::async_test]
async fn plugin_manifest_dependencies_and_contributions_default_absent_on_the_wire() {
    let bare = serde_json::json!({
        "pluginId": "flow",
        "label": "Flow",
        "version": "1.0.0",
        "apps": [],
        "examples": [],
    });
    let parsed: PluginManifest = serde_json::from_value(bare).unwrap();
    assert!(parsed.dependencies.is_empty());
    assert!(parsed.contributions.is_empty());

    let serialized = serde_json::to_value(&parsed).unwrap();
    assert!(serialized.get("dependencies").is_none(), "empty dependencies must be skipped, not emitted as []");
    assert!(serialized.get("contributions").is_none(), "empty contributions must be skipped, not emitted as []");
}

#[semio_framework_async_macros::async_test]
async fn artifact_contribution_descriptor_round_trips() {
    let descriptor = ArtifactContributionDescriptor {
        artifact_kind: "s.cad.building".into(),
        mutations: vec![ContributedMutationMetadata {
            mutation_id: "s.cad.building#aec-building:add-floor".into(),
            semantics: ContributedMutationSemantics { verb: "add".into(), entity: "floor".into(), kind: "structural".into(), record: "aec.floor".into() },
            schema_version: 1,
            algorithm_version: 1,
        }],
        inferences: vec![ContributedInferenceMetadata {
            owner: "aec-building".into(),
            artifact_kind: "s.cad.building".into(),
            artifact_schema: "s.cad.building".into(),
            artifact_schema_version: 1,
            inference_schema: "s.aec-building.load-path".into(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            contributor: "aec-building".into(),
            depends_on: vec!["s.cad.building#topology".into()],
            payload: None,
        }],
    };
    let json = serde_json::to_value(&descriptor).unwrap();
    let round_tripped: ArtifactContributionDescriptor = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, descriptor);
}

#[semio_framework_async_macros::async_test]
async fn plugin_manifest_with_dependencies_and_contributions_round_trips() {
    let manifest = PluginManifest {
        dependencies: vec![PluginDependency::new("cad", pin(1, 0, 0))],
        contributions: vec![ArtifactContributionDescriptor { artifact_kind: "s.cad.building".into(), mutations: Vec::new(), inferences: Vec::new() }],
        ..manifest("aec-building", "0.1.0", Vec::new()).await
    };
    let json = serde_json::to_value(&manifest).unwrap();
    let round_tripped: PluginManifest = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, manifest);
}
//#endregion 🔖️ManifestSerdeTests
