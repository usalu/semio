const IDENTITY_FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️plugin-identity/🔣️.json");
const COMPONENT_MANIFEST: &str = include_str!("../../📦️packages/🦀️rust/Cargo.toml");

/// 🪪️ The language-agnostic identity tuple, read once and shared by every authority assertion.
fn identity_fixture() -> serde_json::Value {
    serde_json::from_str(IDENTITY_FIXTURE).expect("plugin-identity fixture is JSON")
}

/// 🏗️ The whole guest assembly, named — `plugin()` runs `try_build`'s own `semio:<plugin_id>`
/// package-identity preflight plus every artifact-declaration preflight.
fn assembled_plugin() -> semio_framework_plugin::Plugin<super::ReasoningApps> {
    match super::plugin() {
        Ok(plugin) => plugin,
        Err(error) => panic!("plugin assembly rejected: {error:?}"),
    }
}

/// 🪪️ The real assembled component and its authored Cargo deployment metadata match the identity fixture.
#[semio_framework_async_macros::async_test]
async fn plugin_identity_is_the_same_in_every_authority() {
    let fixture = identity_fixture();
    let plugin_id = fixture["pluginId"].as_str().expect("fixture pluginId");
    let package_id = fixture["packageId"].as_str().expect("fixture packageId");
    assert_eq!(package_id, format!("semio:{plugin_id}"));

    let manifest = assembled_plugin().manifest;
    assert_eq!(manifest.plugin_id, plugin_id);
    assert!(COMPONENT_MANIFEST.contains(&format!("package = \"{package_id}\"")), "Cargo component package is not {package_id}");
    assert!(COMPONENT_MANIFEST.contains(&format!("name = \"{}\"", fixture["packageName"].as_str().expect("fixture packageName"))));
    assert!(COMPONENT_MANIFEST.contains(&format!("variant = \"{}\"", fixture["playgroundVariant"].as_str().expect("fixture playgroundVariant"))), "the playground variant row is the OTHER name and must stay declared");

    let prefix = fixture["artifactKindPrefix"].as_str().expect("fixture artifactKindPrefix");
    assert_eq!(prefix, format!("s.{plugin_id}."));
    for app in &manifest.apps {
        assert!(app.id.starts_with(prefix), "{} is not owned by {plugin_id} under the canonical s.<plugin>.<artifact> grammar", app.id);
    }

    assert!(COMPONENT_MANIFEST.contains(&format!("deployment-directory = \"{}\"", fixture["moduleDirectoryName"].as_str().expect("owner directory"))));
}
