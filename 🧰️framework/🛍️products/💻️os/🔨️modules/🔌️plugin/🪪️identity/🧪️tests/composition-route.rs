//! 🧪 General native and foreign composition authority without target ownership collisions.
use super::*;
use semio_framework_artifact_reference::{ArtifactDialect, Dialect, StandardId, SubsetId};

const TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
const ALPHA: Dialect = Dialect { artifact_kind: "s.test.alpha", standard: StandardId("1"), subset: SubsetId("*") };
const BETA: Dialect = Dialect { artifact_kind: "s.test.beta", standard: StandardId("1"), subset: SubsetId("*") };

fn definition(owner: &str, claims: Vec<ArtifactIdentityClaim>) -> ArtifactDefinition {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(format!("{owner}.composer.test.v1")).unwrap(), ArtifactCapabilityKind::composer()).descriptor(b"test route".to_vec()).unwrap();
    for claim in claims {
        capability = capability.claim(claim).unwrap();
    }
    ArtifactDefinition::new(ArtifactIdentity::parse(owner).unwrap()).capability(capability).unwrap()
}

#[test]
fn native_and_foreign_routes_have_independent_authority() {
    let native = composer_identity_claims("s.stdio.txt", TXT, &[TXT]).unwrap();
    assert_eq!(native[0].namespace().as_str(), "dialect");
    assert_eq!(native[0].value(), "s.stdio.txt@utf-8/*");
    let alpha = composer_identity_claims("s.test.alpha", TXT, &[ALPHA]).unwrap();
    let beta = composer_identity_claims("s.test.beta", TXT, &[BETA]).unwrap();
    assert_eq!(alpha[0].namespace().as_str(), "composition-route");
    assert_ne!(alpha, beta);
    let mut registry = ArtifactDefinitionRegistry::new();
    registry.register(definition("s.stdio.txt", native)).unwrap();
    registry.register(definition("s.test.alpha", alpha.clone())).unwrap();
    registry.register(definition("s.test.beta", beta)).unwrap();
    assert_eq!(registry.len(), 3);
    assert!(registry.register(definition("s.test.other", alpha)).is_err());
    assert_eq!(registry.len(), 3);
    assert!(composer_identity_claims("s.test.alpha", TXT, &[BETA]).is_err());
    assert!(composer_identity_claims("s.test.alpha", TXT, &[ALPHA, BETA]).is_err());
}

#[test]
fn canonical_route_framing_matches_language_neutral_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("composition-route.json")).unwrap();
    for value in fixture["valid"].as_array().unwrap() {
        assert!(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::composition_route(), value.as_str().unwrap()).is_ok());
    }
    for value in fixture["invalid"].as_array().unwrap() {
        assert!(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::composition_route(), value.as_str().unwrap()).is_err());
    }
    let route = ArtifactIdentityClaim::composition_route(&ArtifactDialect::from(ALPHA), &ArtifactDialect::from(TXT)).unwrap();
    assert_eq!(route.value(), fixture["valid"][1].as_str().unwrap());
}
