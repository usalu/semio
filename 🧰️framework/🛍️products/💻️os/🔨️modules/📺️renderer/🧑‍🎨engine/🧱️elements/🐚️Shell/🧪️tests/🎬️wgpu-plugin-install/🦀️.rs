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
