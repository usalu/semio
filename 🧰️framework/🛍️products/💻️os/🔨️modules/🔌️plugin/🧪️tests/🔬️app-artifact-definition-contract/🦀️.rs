mod artifact_definition_contract_tests {
    use super::*;

    fn identity(value: &str) -> ArtifactIdentity {
        ArtifactIdentity::parse(value).unwrap()
    }

    fn capability(owner: &ArtifactIdentity, segment: &str, kind: ArtifactCapabilityKind) -> ArtifactCapability {
        let identity = match kind.as_str() {
            "standard" => owner.standard("v4"),
            "profile" => owner.standard("v4").and_then(|identity| identity.profile(segment)),
            "source-dialect" => owner.standard("v4").and_then(|identity| identity.source_dialect(segment)),
            "representation" => owner.standard("v4").and_then(|identity| identity.representation(segment)),
            "codec" => owner.standard("v4").and_then(|identity| identity.codec(segment, "v1")),
            "mutation" => owner.mutation(segment, "v1"),
            "inference" => owner.inference(segment, "v1"),
            "localization" => owner.child("localization").and_then(|identity| identity.child("en")),
            category => owner.child(category).and_then(|identity| identity.child(segment)),
        }
        .unwrap();
        ArtifactCapability::new(identity, kind).descriptor(b"test".to_vec()).unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn plural_definition_carries_every_artifact_capability_without_a_dispatch_edit() {
        let owner = identity("s.stdio.ifc");
        let schema = capability(&owner, "schema", ArtifactCapabilityKind::schema()).claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.stdio.ifc.v4").unwrap()).unwrap();
        let standard = capability(&owner, "standard", ArtifactCapabilityKind::standard());
        let profile = capability(&owner, "profile", ArtifactCapabilityKind::profile());
        let dialect = capability(&owner, "source-dialect", ArtifactCapabilityKind::source_dialect()).claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.ifc.standard.v4.dialect.any").unwrap()).unwrap();
        let representation = capability(&owner, "representation", ArtifactCapabilityKind::representation()).claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::mime(), "application/ifc").unwrap()).unwrap();
        let codec = capability(&owner, "codec", ArtifactCapabilityKind::codec()).claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "stdio.ifc.v4.pack").unwrap()).unwrap();
        let mutation = capability(&owner, "mutation", ArtifactCapabilityKind::mutation());
        let inference = capability(&owner, "inference", ArtifactCapabilityKind::inference());
        let resource = capability(&owner, "resource", ArtifactCapabilityKind::resource()).claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "ifc").unwrap()).unwrap();
        let localization = capability(&owner, "localization", ArtifactCapabilityKind::localization())
            .localization(ArtifactLocalization::new(ArtifactLocale::parse("en").unwrap(), "Industry Foundation Classes").unwrap())
            .unwrap()
            .localization(ArtifactLocalization::new(ArtifactLocale::parse("de").unwrap(), "Industriegrundklassen").unwrap())
            .unwrap();
        let conformance = capability(&owner, "conformance-suite", ArtifactCapabilityKind::conformance_suite()).claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension_implementation(), "ifc-v4-conformance").unwrap()).unwrap();
        let definition = [schema, standard, profile, dialect, representation, codec, mutation, inference, resource, localization, conformance]
            .into_iter()
            .try_fold(ArtifactDefinition::new(owner.clone()), |definition, capability| definition.capability(capability))
            .unwrap();

        assert_eq!(definition.capabilities().count(), 11);
        assert!(definition.validate().is_ok());
        assert!(definition.capabilities().any(|capability| capability.identity().as_str() == "s.stdio.ifc.standard.v4"));
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically() {
        let owner_a = identity("s.stdio.ifc");
        let owner_b = identity("s.stdio.json");
        let namespaces = [ArtifactIdentityNamespace::schema(), ArtifactIdentityNamespace::dialect(), ArtifactIdentityNamespace::codec(), ArtifactIdentityNamespace::mime(), ArtifactIdentityNamespace::extension()];
        for (index, namespace) in namespaces.into_iter().enumerate() {
            let value = format!("shared-{index}");
            let first =
                ArtifactDefinition::new(owner_a.clone()).capability(capability(&owner_a, &format!("first-{index}"), ArtifactCapabilityKind::resource()).claim(ArtifactIdentityClaim::new(namespace.clone(), value.clone()).unwrap()).unwrap()).unwrap();
            let second = ArtifactDefinition::new(owner_b.clone()).capability(capability(&owner_b, &format!("second-{index}"), ArtifactCapabilityKind::resource()).claim(ArtifactIdentityClaim::new(namespace, value).unwrap()).unwrap()).unwrap();
            let mut registry = ArtifactDefinitionRegistry::new();
            registry.register(first).unwrap();
            let error = registry.register(second).unwrap_err();
            assert_eq!(error.code(), "artifact-definition.conflicting-claim");
            assert_eq!(registry.len(), 1);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn identities_and_locales_are_explicit_and_conflicts_do_not_overwrite() {
        assert!(ArtifactIdentity::parse("s.stdio..ifc").is_err());
        assert!(ArtifactLocale::parse("EN").is_err());
        let owner = identity("s.stdio.ifc");
        let localized = capability(&owner, "localized", ArtifactCapabilityKind::localization()).localization(ArtifactLocalization::new(ArtifactLocale::parse("en").unwrap(), "Ifc").unwrap()).unwrap();
        let duplicate = localized.clone().localization(ArtifactLocalization::new(ArtifactLocale::parse("en").unwrap(), "IFC").unwrap()).unwrap_err();
        assert_eq!(duplicate.code(), "artifact-definition.duplicate-locale");

        let mut registry = ArtifactDefinitionRegistry::new();
        registry.register(ArtifactDefinition::new(owner.clone()).capability(localized).unwrap()).unwrap();
        let error = registry.register(ArtifactDefinition::new(owner)).unwrap_err();
        assert_eq!(error.code(), "artifact-definition.conflicting-artifact");
        assert_eq!(registry.len(), 1);
    }
}
