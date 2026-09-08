mod artifact_contribution_tests {
    use super::*;

    async fn semantics(kind: &str) -> semio_framework::ContributedMutationSemantics {
        semio_framework::ContributedMutationSemantics { verb: "add".into(), entity: "thing".into(), kind: kind.into(), record: "AddedThing".into() }
    }

    async fn descriptor_with_mutation(artifact_kind: &str, mutation_id: &str, kind: &str) -> semio_framework::ArtifactContributionDescriptor {
        semio_framework::ArtifactContributionDescriptor {
            artifact_kind: artifact_kind.into(),
            mutations: vec![semio_framework::ContributedMutationMetadata { mutation_id: mutation_id.into(), semantics: semantics(kind).await, schema_version: 1, algorithm_version: 1 }],
            inferences: Vec::new(),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn dependency_gating_rejects_a_contribution_onto_a_non_dependency() {
        let descriptor = descriptor_with_mutation("s.dep.target", "dep.document#contributor:add-thing", "add-thing").await;
        let error = register_contributions("contributor", &[], std::slice::from_ref(&descriptor)).expect_err("missing dependency must be rejected");
        assert!(matches!(error, ContributionRegistrationError::DependencyNotDeclared { owner, .. } if owner == "dep"));
        let dependencies = vec![semio_framework::PluginDependency::new("dep", semio_framework::VersionReq::Any)];
        register_contributions("contributor", &dependencies, std::slice::from_ref(&descriptor)).expect("direct dependency must be accepted");
    }

    #[semio_framework_async_macros::async_test]
    async fn id_namespacing_rejects_a_collision_with_an_owner_kind() {
        let dependencies = vec![semio_framework::PluginDependency::new("dep", semio_framework::VersionReq::Any)];
        let bare = descriptor_with_mutation("s.dep.target", "dep.document#add-thing", "add-thing").await;
        let error = register_contributions("contributor", &dependencies, std::slice::from_ref(&bare)).expect_err("bare owner-shaped id must be rejected");
        assert!(matches!(error, ContributionRegistrationError::CollidesWithOwnerKind(id) if id == "dep.document#add-thing"));
        let well_formed = descriptor_with_mutation("s.dep.target", "dep.document#contributor:add-thing", "add-thing").await;
        register_contributions("contributor", &dependencies, std::slice::from_ref(&well_formed)).expect("well-formed contributed id must be accepted");
    }

    // 🚫️async: E4 fn-pointer slot — `commit_owner_mutation_roster` takes
    // `&[fn() -> (&'static str, &'static [SemanticDescriptor])]`, a plain sync fn pointer; see R2/R9.
    fn owner_roster_provider() -> (&'static str, &'static [::protocol::SemanticDescriptor]) {
        const KINDS: &[::protocol::SemanticDescriptor] =
            &[::protocol::SemanticDescriptor { verb: "add", entity: "widget", kind: "add-widget", record: "AddedWidget" }, ::protocol::SemanticDescriptor { verb: "remove", entity: "widget", kind: "remove-widget", record: "RemovedWidget" }];
        ("roster-determinism-test.document", KINDS)
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_roster_entries_are_deterministic_across_repeated_calls() {
        let providers: &[OwnerMutationRoster] = &[owner_roster_provider];
        commit_owner_mutation_roster(providers).await.expect("commit owner roster");
        let first = mutation_roster_entries().await;
        let second = mutation_roster_entries().await;
        assert_eq!(first, second);
        let mut sorted = first.clone();
        sorted.sort_by(|a, b| a.mutation_id.cmp(&b.mutation_id));
        assert_eq!(first, sorted);
        assert!(first.iter().any(|entry| entry.mutation_id == "roster-determinism-test.document#add-widget"));
    }
}
