//! 🪪️ Actual builder admission laws. Every law in this file shares ONE test process with
//! `strict_artifact_identity_owned_tree_and_definition_channels_publish`, which legitimately and
//! irreversibly publishes the `s.testkit.w1c-fixture` tree into the process-global schema, codec, io
//! and format registries. A rejection law therefore differences [`publication_witness`] around its
//! candidate instead of asserting an absolute emptiness that its sibling's success would break.

use crate::app::{declarations::fixture, ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, Plugin};
use store::os_io::ArtifactKindId;

fn definition(kind: &str) -> ArtifactDefinition {
    ArtifactDefinition::new(ArtifactIdentity::parse(kind).expect("syntactically valid definition identity"))
}

fn declaration(kind: &str) -> ArtifactDeclaration {
    let schema = fixture::build_declaration().standards[0].subsets[0].schema.descriptor.clone();
    let capability = ArtifactCapability::new(ArtifactIdentity::parse(&format!("{kind}.schema.fixture")).unwrap(), ArtifactCapabilityKind::schema())
        .descriptor(b"admission fixture".to_vec())
        .unwrap()
        .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), schema.id).unwrap())
        .unwrap();
    ArtifactDeclaration::builder(definition(kind).capability(capability).unwrap()).schema(schema).try_build().expect("inert declaration")
}

/// 🪪️ Exactly what the `s.testkit.w1c-fixture` tree would publish, read out of the four
/// process-global registries a declaration reaches. A rejected candidate must leave this reading
/// identical — that is the law, and unlike an absolute "nothing is registered" it holds whether or
/// not this binary has already run the sibling law that publishes the same tree for real.
async fn publication_witness() -> Vec<bool> {
    let mut witness = Vec::new();
    for (id, codec) in [("s.testkit.w1c-fixture@1/*", "semio.testkit.w1c-fixture.std1-any/v1"), ("s.testkit.w1c-fixture@1/strict", "semio.testkit.w1c-fixture.std1-strict/v1"), ("s.testkit.w1c-fixture@2/*", "semio.testkit.w1c-fixture.std2-any/v1")] {
        witness.push(semio_framework_schema::artifact_schema_descriptor_registered(id));
        witness.push(store::document_codec(codec).await.expect("registry available").is_some());
    }
    witness.push(semio_framework::io::io_mechanism::io_entries().iter().any(|row| row.from.artifact_kind == "s.testkit.w1c-fixture" || row.into.artifact_kind == "s.testkit.w1c-fixture"));
    witness.push(semio_framework::io::format_descriptor("s.testkit.w1c-fixture@1").expect("registry available").is_some());
    witness
}

/// 🚫️ The rejected candidate published nothing it did not already find.
async fn assert_published_nothing(before: &[bool]) {
    assert_eq!(publication_witness().await, before, "a rejected candidate must reach no registry");
}

#[semio_framework_async_macros::async_test]
async fn strict_artifact_identity_all_builder_channels_reject_before_publication() {
    let before = publication_witness().await;
    for (kind, code) in [("s.fixture", "plugin-assembly.artifact-kind"), ("s.other.document", "plugin-assembly.artifact-owner")] {
        let old = Plugin::<fixture::FixtureApps>::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").artifact(declaration(kind)).try_build();
        assert_eq!(old.err().expect("old declaration denied").code, code);
        let only = Plugin::<fixture::FixtureApps>::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").artifact_definition(definition(kind)).try_build();
        assert_eq!(only.err().expect("definition-only denied").code, code);
        assert_published_nothing(&before).await;
    }
    let mut foreign = fixture::build_declaration();
    foreign.kind = ArtifactKindId::parse("s.other.document").expect("canonical foreign kind");
    let tree = Plugin::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").declare_artifact(foreign).try_build();
    assert_eq!(tree.err().expect("foreign tree denied").code, "plugin-assembly.artifact-owner");
    assert_published_nothing(&before).await;
    let mut foreign_subset = fixture::build_declaration();
    foreign_subset.standards[0].subsets[0].dialect.artifact_kind = "s.other.document";
    let tree = Plugin::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").declare_artifact(foreign_subset).try_build();
    assert_eq!(tree.err().expect("foreign subset denied").code, "plugin-assembly.artifact-owner");
    assert_published_nothing(&before).await;
}

#[semio_framework_async_macros::async_test]
async fn strict_artifact_identity_mixed_channels_publish_nothing() {
    let before = publication_witness().await;
    let old = Plugin::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").declare_artifact(fixture::build_declaration()).artifact(declaration("s.other.document")).try_build();
    assert_eq!(old.err().expect("mixed old denied").code, "plugin-assembly.artifact-owner");
    assert_published_nothing(&before).await;
    let only = Plugin::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").declare_artifact(fixture::build_declaration()).artifact_definition(definition("s.other.document")).try_build();
    assert_eq!(only.err().expect("mixed definition denied").code, "plugin-assembly.artifact-owner");
    assert_published_nothing(&before).await;
}

#[test]
fn strict_artifact_identity_matches_independent_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-admission/🔣️.json")).expect("neutral fixture");
    let rows = fixture["cases"].as_array().expect("cases");
    assert_eq!(rows.len(), 15);
    for row in rows {
        let result = crate::app::preflight_artifact_identity(row["plugin"].as_str().unwrap(), row["kind"].as_str().unwrap());
        assert_eq!(result.err().map(|error| error.code).unwrap_or_else(|| "accepted".into()), row["code"].as_str().unwrap(), "{}", row["id"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn strict_artifact_identity_owned_tree_and_definition_channels_publish() {
    let stdio = Plugin::<fixture::FixtureApps>::builder("stdio").label("Admission").version("0.1.0").package_id("semio:stdio").artifact_definition(definition("s.stdio.json")).try_build().expect("owned definition");
    assert_eq!(stdio.manifest.plugin_id, "stdio");
    assert!(stdio.artifact_definitions().definitions().all(|definition| ArtifactKindId::parse(definition.identity().as_str()).unwrap().plugin() == stdio.manifest.plugin_id));
    let tree = Plugin::builder("testkit").label("Admission").version("0.1.0").package_id("semio:testkit").declare_artifact(fixture::build_declaration()).try_build().expect("owned tree");
    assert_eq!(tree.manifest.plugin_id, "testkit");
    assert_eq!(tree.manifest.apps.len(), 6);
    assert!(semio_framework_schema::artifact_schema_descriptor_registered("s.testkit.w1c-fixture@1/*"));
    assert!(store::document_codec("semio.testkit.w1c-fixture.std1-any/v1").await.unwrap().is_some());
}
