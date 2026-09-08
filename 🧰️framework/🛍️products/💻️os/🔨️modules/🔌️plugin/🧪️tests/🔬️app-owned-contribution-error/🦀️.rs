mod owned_contribution_error_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn owned_errors_preserve_their_messages() {
        let dependency = ContributionRegistrationError::DependencyNotDeclared { plugin_id: "consumer".into(), artifact_kind: "s.owner.kind".into(), owner: "owner".into() };
        assert_eq!(dependency.to_string(), "contribution targets artifact kind \"s.owner.kind\" (owned by plugin \"owner\"), which is not a direct dependency of \"consumer\"");
        let malformed = ContributionRegistrationError::MalformedMutationId { plugin_id: "consumer".into(), mutation_id: "bad".into() };
        assert_eq!(malformed.to_string(), "contributed mutation id \"bad\" does not match the frozen grammar `<target-document-schema>#consumer:<kebab-kind>`");
    }
}
