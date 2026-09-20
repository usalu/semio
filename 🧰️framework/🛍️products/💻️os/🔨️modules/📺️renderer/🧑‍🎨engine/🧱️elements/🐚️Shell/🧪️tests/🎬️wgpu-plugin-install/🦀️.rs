use super::*;

/// 🧪️ Every kind the generated catalog claims resolves to exactly one plugin, and the two spellings a
/// descriptor declares (its own `ArtifactKindSpec.id` and the artifact kind its app surfaces name)
/// both land on the same owner. This is the table `switch_to_app`'s lazy install reads BEFORE a single
/// wasm module exists, so a silently empty one would make the hub unable to open anything it has not
/// already loaded — exactly the `Err("program missing")` this packet removed.
#[test]
fn every_declared_artifact_kind_names_one_owner() {
    let table = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS;
    assert!(table.len() > 1, "the generated catalog must claim more than one artifact kind");
    let mut kinds: Vec<&str> = table.iter().map(|(kind, _)| *kind).collect();
    kinds.sort_unstable();
    let before = kinds.len();
    kinds.dedup();
    assert_eq!(kinds.len(), before, "an artifact kind may be claimed by exactly one plugin");
    for (kind, plugin_id) in table {
        assert_eq!(crate::program_bridge::resolve_artifact_kind_activation_owner(kind), Some(*plugin_id));
    }
    assert_eq!(crate::program_bridge::resolve_artifact_kind_activation_owner("no.such.kind"), None);
    assert_eq!(crate::program_bridge::resolve_artifact_kind_activation_owner(""), None);
}

/// 🧪️ The host plugin and one unrelated plugin both claim their own kinds, so a hub session can name
/// an owner that is NOT itself — the whole point of the lazy install.
#[test]
fn the_host_plugin_is_not_the_owner_of_every_kind() {
    let owners: std::collections::BTreeSet<&str> = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter().map(|(_, plugin_id)| *plugin_id).collect();
    assert!(owners.len() > 1, "more than one plugin must own artifact kinds");
    assert_eq!(crate::program_bridge::resolve_artifact_kind_activation_owner("s.cad.cad"), Some("cad"));
    assert_eq!(crate::program_bridge::resolve_artifact_kind_activation_owner("3d.cad"), Some("cad"));
}

/// 🧪️ `install_plugin` is a no-op for a resident plugin and refuses — rather than panicking or
/// silently succeeding — when this shell holds no program to read the modules root off. An empty
/// `plugins` list is the boot-fault case, not a reason to claim an install happened.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn installing_without_a_resident_program_is_refused() {
    let mut shell = ShellState::new(Vec::new(), "s".into());
    assert!(shell.plugin_install.is_none());
    let outcome = semio_framework_async::block_on(shell.install_plugin("cad"));
    assert_eq!(outcome, Err("plugin modules root is unavailable".to_string()));
    assert!(shell.plugin_install.is_none(), "a refusal before any step boundary retains no install record");
}

/// 🧪️ Cancelling with nothing in flight is inert, and cancelling a settled record clears it — the
/// same "only the install's own next step may report `Cancelled`" posture `cancel_inference_proposal`
/// keeps for the retained inference port.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelling_clears_only_a_settled_install_record() {
    let mut shell = ShellState::new(Vec::new(), "s".into());
    shell.cancel_plugin_install();
    assert!(shell.plugin_install.is_none());
    shell.plugin_install = Some(ShellPluginInstall { plugin_id: "cad".into(), phase: ShellPluginInstallPhase::Failed("boom".into()), cancel: CancelToken::root_now() });
    shell.cancel_plugin_install();
    assert!(shell.plugin_install.is_none());
    let cancel = CancelToken::root_now();
    shell.plugin_install = Some(ShellPluginInstall { plugin_id: "cad".into(), phase: ShellPluginInstallPhase::Loading, cancel: cancel.clone() });
    shell.cancel_plugin_install();
    assert!(shell.plugin_install.is_some(), "an in-flight install stays retained until its own next step boundary settles it");
    assert!(cancel.is_cancelled_now());
}

/// 🧪️ The modules root is read back off a resident program's artifact path, two levels up — the same
/// `<modules_root>/<plugin_id>/<file>.wasm` layout `load_wasm_plugins` publishes. Without a resident
/// program there is nothing to read, which is what the refusal above reports.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_modules_root_comes_from_a_resident_program() {
    assert_eq!(plugin_modules_root_of(&[]), None);
}

/// 🧪️ The opening relay's parser is target-neutral (O3): it is the first thing
/// `handle_open_artifact_relay` runs on BOTH targets now, so an app-only relay — the shape a browser
/// wgpu playground sends when it asks for a foreign kind it has not loaded — must parse into a
/// dialect with no document coordinate and the declared default role.
#[test]
fn an_app_only_relay_parses_into_a_dialect_with_no_document() {
    let args = serde_json::json!({ "artifactRef": "s.cad.cad@1/*" });
    let target = open_artifact_relay_target("os.open-artifact", Some(&args)).expect("app-only relay");
    assert_eq!(target.artifact_ref, "s.cad.cad@1/*");
    assert_eq!(target.dialect.artifact_kind, "s.cad.cad");
    assert_eq!(target.role, semio_framework::AppRole::Editor);
    assert!(target.plugin_id.is_none() && target.app_id.is_none());
    assert!(target.document_id.is_none() && target.schema.is_none());
}

/// 🧪️ The exact hand-off the browser relay performs: the parsed dialect's kind is the key the
/// generated activation table answers with a plugin that is NOT resident in a single-plugin
/// playground, which is what makes `install_plugin` + `switch_to_app` a real foreign-kind open
/// rather than a no-op. Both halves of this pair compile for `wasm32-unknown-unknown`.
#[test]
fn a_parsed_relay_kind_resolves_to_its_declared_owner() {
    let args = serde_json::json!({ "artifactRef": "s.cad.cad@1/*", "role": "editor" });
    let target = open_artifact_relay_target("os.open-artifact", Some(&args)).expect("relay");
    assert_eq!(crate::program_bridge::resolve_artifact_kind_activation_owner(&target.dialect.artifact_kind), Some("cad"));
    let owners: std::collections::BTreeSet<&str> = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter().map(|(_, plugin_id)| *plugin_id).collect();
    assert!(owners.len() > 1, "the relay must be able to name an owner other than whichever plugin hosts the shell");
}

/// 🧪️ Both coordinate pairs are all-or-nothing: a half app ref and a half document ref are refused
/// with their own codes rather than silently opening the wrong thing.
#[test]
fn half_a_coordinate_pair_is_refused() {
    let partial_app = serde_json::json!({ "artifactRef": "s.cad.cad@1/*", "pluginId": "cad" });
    assert_eq!(open_artifact_relay_target("os.open-artifact", Some(&partial_app)).unwrap_err(), "opening.partial-app-ref");
    let partial_document = serde_json::json!({ "artifactRef": "s.cad.cad@1/*", "documentId": "index" });
    assert_eq!(open_artifact_relay_target("os.open-artifact", Some(&partial_document)).unwrap_err(), "opening.partial-document-ref");
    let no_ref = serde_json::json!({ "documentId": "index", "schema": "s.cad" });
    assert_eq!(open_artifact_relay_target("os.open-artifact", Some(&no_ref)).unwrap_err(), "opening.invalid-artifact-ref");
    assert_eq!(open_artifact_relay_target("os.open-artifact", None).unwrap_err(), "opening.invalid-args");
}

/// 🧪️ `os.open-artifact-with` is the explicit-app spelling: it refuses a relay that names no app,
/// where the plain `os.open-artifact` falls back to the kind's declared activation owner.
#[test]
fn the_explicit_spelling_requires_an_app_ref() {
    let args = serde_json::json!({ "artifactRef": "s.cad.cad@1/*" });
    assert_eq!(open_artifact_relay_target("os.open-artifact-with", Some(&args)).unwrap_err(), "opening.explicit-app-required");
    let with_app = serde_json::json!({ "artifactRef": "s.cad.cad@1/*", "pluginId": "cad", "appId": "s.cad.cad@1/*#editor" });
    let target = open_artifact_relay_target("os.open-artifact-with", Some(&with_app)).expect("explicit relay");
    assert_eq!(target.plugin_id.as_deref(), Some("cad"));
    assert_eq!(target.app_id.as_deref(), Some("s.cad.cad@1/*#editor"));
    assert_eq!(target.role, semio_framework::AppRole::Editor);
}

/// 🧪️ A surface-suffixed artifact ref carries its own role; disagreeing with an explicit `role`
/// argument — or with the app ref's own surface — is a refusal, never a silent pick of one of them.
#[test]
fn a_surface_ref_must_agree_with_every_other_role_spelling() {
    let mismatch = serde_json::json!({ "artifactRef": "s.cad.cad@1/*#viewer", "role": "editor" });
    assert_eq!(open_artifact_relay_target("os.open-artifact", Some(&mismatch)).unwrap_err(), "opening.role-mismatch");
    let app_mismatch = serde_json::json!({ "artifactRef": "s.cad.cad@1/*#viewer", "pluginId": "cad", "appId": "s.cad.cad@1/*#editor" });
    assert_eq!(open_artifact_relay_target("os.open-artifact", Some(&app_mismatch)).unwrap_err(), "opening.app-mismatch");
    let agreeing = serde_json::json!({ "artifactRef": "s.cad.cad@1/*#viewer" });
    let target = open_artifact_relay_target("os.open-artifact", Some(&agreeing)).expect("surface relay");
    assert_eq!(target.role, semio_framework::AppRole::Viewer);
    assert_eq!(target.artifact_ref, "s.cad.cad@1/*", "the surface suffix is normalized off the stored coordinate");
}

/// 🧪️ The browser's document half refuses out loud: the relay's wasm32 arm reads this key, and an
/// untranslated one would paint the key itself into the banner.
#[test]
fn the_browser_document_refusal_is_localized() {
    let english = shell_chrome_string("open-artifact.browser-document", false);
    let german = shell_chrome_string("open-artifact.browser-document", true);
    assert!(english.contains("document sync"), "{english}");
    assert!(german.contains("Dokumentsynchronisierung"), "{german}");
    assert_ne!(english, german);
}
