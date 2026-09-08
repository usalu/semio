
//! 🔗️ Ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS
//! lane W0-C: `Version`/`VersionReq` parse+match matrix, dependency-graph toposort/cycle/
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

//#region 🔖️VersionAndVersionReq
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
async fn version_req_parses_all_five_grammar_forms_and_rejects_unknown_operators() {
    assert_eq!(VersionReq::parse("*").unwrap(), VersionReq::Any);
    assert_eq!(VersionReq::parse("=1.2.3").unwrap(), VersionReq::Exact(Version::new(1, 2, 3)));
    assert_eq!(VersionReq::parse("^1.2.3").unwrap(), VersionReq::Caret(Version::new(1, 2, 3)));
    assert_eq!(VersionReq::parse("~1.2.3").unwrap(), VersionReq::Tilde(Version::new(1, 2, 3)));
    assert_eq!(VersionReq::parse(">=1.2.3").unwrap(), VersionReq::AtLeast(Version::new(1, 2, 3)));
    assert!(matches!(VersionReq::parse("1.2.3").unwrap_err(), VersionReqParseError::UnknownOperator(_)));
    assert!(matches!(VersionReq::parse("^1.x.3").unwrap_err(), VersionReqParseError::Version(_)));
}

#[semio_framework_async_macros::async_test]
async fn version_req_display_round_trips_through_parse() {
    for raw in ["*", "=1.2.3", "^1.2.3", "~1.2.3", ">=1.2.3"] {
        let parsed = VersionReq::parse(raw).unwrap();
        assert_eq!(parsed.to_string(), raw);
        assert_eq!(VersionReq::parse(&parsed.to_string()).unwrap(), parsed);
    }
}

#[semio_framework_async_macros::async_test]
async fn version_req_matches_exact_and_at_least() {
    let exact = VersionReq::parse("=1.2.3").unwrap();
    assert!(exact.matches(&Version::new(1, 2, 3)));
    assert!(!exact.matches(&Version::new(1, 2, 4)));

    let at_least = VersionReq::parse(">=1.2.3").unwrap();
    assert!(at_least.matches(&Version::new(1, 2, 3)));
    assert!(at_least.matches(&Version::new(2, 0, 0)));
    assert!(!at_least.matches(&Version::new(1, 2, 2)));

    assert!(VersionReq::Any.matches(&Version::new(0, 0, 0)));
}

#[semio_framework_async_macros::async_test]
async fn version_req_matches_caret_semantics_across_leading_zero_tiers() {
    let caret_major = VersionReq::parse("^1.2.3").unwrap();
    assert!(caret_major.matches(&Version::new(1, 2, 3)));
    assert!(caret_major.matches(&Version::new(1, 9, 0)), "caret allows minor/patch bumps under the same major");
    assert!(!caret_major.matches(&Version::new(1, 2, 2)), "caret forbids going below the required version");
    assert!(!caret_major.matches(&Version::new(2, 0, 0)), "caret forbids a major bump");

    let caret_zero_major = VersionReq::parse("^0.2.3").unwrap();
    assert!(caret_zero_major.matches(&Version::new(0, 2, 3)));
    assert!(caret_zero_major.matches(&Version::new(0, 2, 9)), "0.x caret allows patch bumps within the same minor");
    assert!(!caret_zero_major.matches(&Version::new(0, 3, 0)), "0.x caret forbids a minor bump");

    let caret_zero_minor = VersionReq::parse("^0.0.3").unwrap();
    assert!(caret_zero_minor.matches(&Version::new(0, 0, 3)));
    assert!(!caret_zero_minor.matches(&Version::new(0, 0, 4)), "0.0.x caret pins the exact patch");
}

#[semio_framework_async_macros::async_test]
async fn version_req_matches_tilde_semantics() {
    let tilde = VersionReq::parse("~1.2.3").unwrap();
    assert!(tilde.matches(&Version::new(1, 2, 3)));
    assert!(tilde.matches(&Version::new(1, 2, 9)), "tilde allows patch bumps");
    assert!(!tilde.matches(&Version::new(1, 3, 0)), "tilde forbids a minor bump");
    assert!(!tilde.matches(&Version::new(1, 2, 2)), "tilde forbids going below the required patch");
}

#[semio_framework_async_macros::async_test]
async fn plugin_dependency_serde_round_trips_as_a_plain_string() {
    let dependency = PluginDependency::new("cad", VersionReq::parse("^1.0.0").unwrap());
    let json = serde_json::to_value(&dependency).unwrap();
    assert_eq!(json, serde_json::json!({ "pluginId": "cad", "version": "^1.0.0" }));
    let round_tripped: PluginDependency = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, dependency);
}
//#endregion 🔖️VersionAndVersionReq

//#region 🔖️DependencyGraphTests
#[semio_framework_async_macros::async_test]
async fn resolve_load_order_toposorts_a_diamond() {
    // base <- {left, right} <- top: two valid topological orders exist; the tie-break must
    // deterministically pick `left` before `right`.
    let manifests = vec![
        manifest("top", "1.0.0", vec![PluginDependency::new("left", VersionReq::Any), PluginDependency::new("right", VersionReq::Any)]).await,
        manifest("left", "1.0.0", vec![PluginDependency::new("base", VersionReq::Any)]).await,
        manifest("right", "1.0.0", vec![PluginDependency::new("base", VersionReq::Any)]).await,
        manifest("base", "1.0.0", vec![]).await,
    ];
    let order = resolve_load_order(&manifests).await.unwrap();
    assert_eq!(order, vec!["base", "left", "right", "top"]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_is_deterministic_regardless_of_input_order() {
    let forward = vec![manifest("a", "1.0.0", vec![]).await, manifest("b", "1.0.0", vec![PluginDependency::new("a", VersionReq::Any)]).await, manifest("c", "1.0.0", vec![PluginDependency::new("a", VersionReq::Any)]).await];
    let mut shuffled = forward.clone();
    shuffled.reverse();
    assert_eq!(resolve_load_order(&forward).await.unwrap(), resolve_load_order(&shuffled).await.unwrap());
    assert_eq!(resolve_load_order(&forward).await.unwrap(), vec!["a", "b", "c"]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_reports_missing_dependency() {
    let manifests = vec![manifest("a", "1.0.0", vec![PluginDependency::new("ghost", VersionReq::Any)]).await];
    let error = resolve_load_order(&manifests).await.unwrap_err();
    assert_eq!(error, DependencyGraphError::MissingDependency { plugin_id: "a".into(), depends_on: "ghost".into() });
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_reports_version_mismatch() {
    let manifests = vec![manifest("a", "1.0.0", vec![PluginDependency::new("b", VersionReq::parse("^2.0.0").unwrap())]).await, manifest("b", "1.0.0", vec![]).await];
    let error = resolve_load_order(&manifests).await.unwrap_err();
    assert_eq!(error, DependencyGraphError::VersionMismatch { plugin_id: "a".into(), depends_on: "b".into(), required: "^2.0.0".into(), actual: "1.0.0".into() });
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_order_names_every_member_of_a_cycle() {
    let manifests = vec![
        manifest("a", "1.0.0", vec![PluginDependency::new("b", VersionReq::Any)]).await,
        manifest("b", "1.0.0", vec![PluginDependency::new("c", VersionReq::Any)]).await,
        manifest("c", "1.0.0", vec![PluginDependency::new("a", VersionReq::Any)]).await,
    ];
    let error = resolve_load_order(&manifests).await.unwrap_err();
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
    assert_eq!(resolve_load_order(&[]).await.unwrap(), Vec::<String>::new());
}

#[semio_framework_async_macros::async_test]
async fn dependents_returns_direct_dependents_sorted() {
    let manifests = vec![
        manifest("a", "1.0.0", vec![]).await,
        manifest("b", "1.0.0", vec![PluginDependency::new("a", VersionReq::Any)]).await,
        manifest("c", "1.0.0", vec![PluginDependency::new("a", VersionReq::Any)]).await,
        manifest("d", "1.0.0", vec![PluginDependency::new("b", VersionReq::Any)]).await,
    ];
    assert_eq!(dependents(&manifests, "a").await, vec!["b".to_string(), "c".to_string()]);
    assert_eq!(dependents(&manifests, "b").await, vec!["d".to_string()]);
    assert!(dependents(&manifests, "d").await.is_empty());
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
            document_schema: "s.cad.document".into(),
            document_schema_version: 1,
            inference_schema: "s.aec-building.load-path".into(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            contributor: "aec-building".into(),
            depends_on: vec!["s.cad.building#topology".into()],
        }],
    };
    let json = serde_json::to_value(&descriptor).unwrap();
    let round_tripped: ArtifactContributionDescriptor = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, descriptor);
}

#[semio_framework_async_macros::async_test]
async fn plugin_manifest_with_dependencies_and_contributions_round_trips() {
    let manifest = PluginManifest {
        dependencies: vec![PluginDependency::new("cad", VersionReq::parse("^1.0.0").unwrap())],
        contributions: vec![ArtifactContributionDescriptor { artifact_kind: "s.cad.building".into(), mutations: Vec::new(), inferences: Vec::new() }],
        ..manifest("aec-building", "0.1.0", Vec::new()).await
    };
    let json = serde_json::to_value(&manifest).unwrap();
    let round_tripped: PluginManifest = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, manifest);
}
//#endregion 🔖️ManifestSerdeTests
