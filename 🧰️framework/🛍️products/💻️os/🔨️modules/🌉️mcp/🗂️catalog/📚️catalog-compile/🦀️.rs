//! 📚️ Catalog compile: the catalog source, its error, the compiled catalog and the compile entry points. See [`super`].

use super::*;

//#region 🔖️CatalogSource
/// 📥️ Everything `compile()` walks — `📋️master.md` §3.2's `CatalogSource{descriptors, os_commands,
/// shell, gateway}` verbatim. `shell`/`gateway` are pre-built `CapabilityDefinition`s (P9's
/// `os_shell::shell_capabilities()` and this crate's own core-tool set respectively) — everything
/// else is compiled FROM the manifest source types.
#[derive(Clone, Debug, Default)]
pub struct CatalogSource {
    pub descriptors: Vec<manifest::PackageDescriptor>,
    pub os_commands: Vec<manifest::CommandDefinition>,
    pub shell: Vec<CapabilityDefinition>,
    pub gateway: Vec<CapabilityDefinition>,
}
//#endregion 🔖️CatalogSource

//#region 🔖️CatalogError
#[derive(Debug, Clone, PartialEq)]
pub enum CatalogError {
    DuplicateCapabilityId(String),
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogError::DuplicateCapabilityId(id) => write!(f, "duplicate capability id: {id}"),
        }
    }
}

impl std::error::Error for CatalogError {}
//#endregion 🔖️CatalogError

//#region 🔖️Catalog
/// 📚️ The compiled, sorted, content-addressed capability catalog — `hash` changes if and only if
/// `entries` changes (blake3 over the canonical JSON serialization of the sorted entry vector, so
/// compiling the SAME `CatalogSource` twice is byte-identical, proven in `🧪️Tests` below).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
pub struct Catalog {
    pub hash: String,
    pub entries: Vec<CapabilityDefinition>,
}

impl Catalog {
    /// 🔎️ Binary-search lookup by id — valid because `entries` is always sorted by `id.0`.
    pub fn get(&self, id: &str) -> Option<&CapabilityDefinition> {
        self.entries.binary_search_by(|entry| entry.id.as_str().cmp(id)).ok().map(|index| &self.entries[index])
    }

    /// 🔧️ Every capability whose `exposure` is `Direct` — the seam `root::component`'s tool
    /// registration walks to publish `tools/list` entries beyond the hand-registered core three.
    pub fn direct_exposures(&self) -> impl Iterator<Item = &CapabilityDefinition> {
        self.entries.iter().filter(|entry| matches!(entry.exposure, ToolExposure::Direct { .. }))
    }
}

fn compute_catalog_entries_hash(entries: &[CapabilityDefinition]) -> String {
    let bytes = serde_json::to_vec(entries).expect("capability catalog entries always serialize");
    framework_hash::hash_bytes(&bytes)
}
//#endregion 🔖️Catalog

//#region 🔖️Compile
fn insert_capability(entries: &mut BTreeMap<String, CapabilityDefinition>, capability: CapabilityDefinition) -> Result<(), CatalogError> {
    let id = capability.id.0.clone();
    if entries.contains_key(&id) {
        return Err(CatalogError::DuplicateCapabilityId(id));
    }
    entries.insert(id, capability);
    Ok(())
}

/// 🏭️ Compiles a `CatalogSource` into a sorted, hashed `Catalog` — `📋️master.md` §3.2. Walks, per
/// descriptor: every app's window-kind actions (skipping the 21 framework-injected ids, folded
/// separately into `framework.*` — see `framework_capabilities`), app-scope commands, mode-scope
/// commands, plugin-scope commands, and the four contribution categories (inference/mutation/io/
/// composer → `Query`/`Job`); then folds every collected `DialogDefinition`/`ExampleDefinition` id
/// into the two gateway-owned `ui.dialog.open`/`artifact.create` capabilities; then walks
/// `source.os_commands` and appends `source.shell`/`source.gateway` verbatim.
pub fn compile(source: &CatalogSource, locale: Locale, terminology: Terminology) -> Result<Catalog, CatalogError> {
    compile_with_audiences(source, locale, terminology, AGENT_AUDIENCES)
}

/// 🏭️ [`compile`] with an explicit audience projection — see [`CapabilityAudience`]. Every
/// declaration is still walked and still fights for its id (a duplicate stays
/// [`CatalogError::DuplicateCapabilityId`] even when the colliding pair is filtered out), so
/// narrowing the published set can never hide a real collision; only the FINAL entry vector, and
/// therefore `hash`, is filtered.
///
/// 🌱️ `artifact.create` has exactly ONE definition in a compiled catalog: `🗿️artifact`'s own
/// invocable `artifact_create` tool when the source carries it (the live gateway always does),
/// otherwise this module's catalog-only projection. The declared templates are an ARGUMENT of
/// that verb, never a second capability wearing its id — which is what made a live catalog with
/// both sources refuse to compile at all (`duplicate capability id: artifact.create`).
pub fn compile_with_audiences(source: &CatalogSource, locale: Locale, terminology: Terminology, audiences: &[CapabilityAudience]) -> Result<Catalog, CatalogError> {
    let mut entries: BTreeMap<String, CapabilityDefinition> = BTreeMap::new();
    let mut all_apps: Vec<&manifest::AppDefinition> = Vec::new();
    let mut dialog_ids: Vec<String> = Vec::new();
    let mut template_ids: Vec<String> = Vec::new();

    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.clone();
        let plugin_label = descriptor.manifest.label.clone();

        for app in descriptor.manifest.apps.iter() {
            let app_id = app.id.clone();
            let artifact_kind = app.dialect.artifact_kind.clone();

            for (action, window_kind_id) in app_action_verbs(app)? {
                if is_framework_injected_action(action) {
                    continue;
                }
                let id = format!("{plugin_id}.{app_id}.{}", action.id);
                let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: Some(app_id.clone()), window_kind_id: Some(window_kind_id.to_string()), mode_id: None };
                let source_ref = CapabilitySource::Action { plugin_id: plugin_id.clone(), app_id: app_id.clone(), window_kind_id: window_kind_id.to_string(), action_id: action.id.clone() };
                insert_capability(&mut entries, capability_from_action(&id, owner, Some(artifact_kind.clone()), action, source_ref, locale, terminology))?;
            }

            for command in &app.commands {
                let id = format!("{plugin_id}.{app_id}.cmd.{}", command.id);
                let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: Some(app_id.clone()), window_kind_id: None, mode_id: None };
                let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: Some(app_id.clone()), mode_id: None, command_id: command.id.clone() };
                insert_capability(&mut entries, capability_from_command(&id, owner, Some(artifact_kind.clone()), command, source_ref, locale, terminology))?;
            }

            for mode in app.modes.iter() {
                for command in &mode.commands {
                    let id = format!("{plugin_id}.{app_id}.mode.{}.{}", mode.id, command.id);
                    let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: Some(app_id.clone()), window_kind_id: None, mode_id: Some(mode.id.clone()) };
                    let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: Some(app_id.clone()), mode_id: Some(mode.id.clone()), command_id: command.id.clone() };
                    insert_capability(&mut entries, capability_from_command(&id, owner, Some(artifact_kind.clone()), command, source_ref, locale, terminology))?;
                }
            }

            for dialog in &app.dialogs {
                dialog_ids.push(dialog.id.clone());
            }

            all_apps.push(app);
        }

        for command in &descriptor.manifest.commands {
            let id = format!("{plugin_id}.cmd.{}", command.id);
            let owner = CapabilityOwner::Plugin { plugin_id: plugin_id.clone(), label: Some(plugin_label.clone()), app_id: None, window_kind_id: None, mode_id: None };
            let source_ref = CapabilitySource::Command { plugin_id: Some(plugin_id.clone()), app_id: None, mode_id: None, command_id: command.id.clone() };
            insert_capability(&mut entries, capability_from_command(&id, owner, None, command, source_ref, locale, terminology))?;
        }

        for example in &descriptor.manifest.examples {
            template_ids.push(format!("{plugin_id}:{}", example.id));
        }

        for entry in &descriptor.contributions.inference_services {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "infer", entry, CapabilityKind::Query))?;
        }
        for entry in &descriptor.contributions.mutation_services {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "mutate", entry, CapabilityKind::Job))?;
        }
        for entry in &descriptor.contributions.io_entries {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "io", entry, CapabilityKind::Job))?;
        }
        for entry in &descriptor.contributions.composer_entries {
            insert_capability(&mut entries, capability_from_contribution(&plugin_id, &plugin_label, "compose", entry, CapabilityKind::Job))?;
        }
    }

    for capability in framework_capabilities(&all_apps, locale, terminology).into_values() {
        insert_capability(&mut entries, capability)?;
    }

    if !dialog_ids.is_empty() {
        dialog_ids.sort();
        dialog_ids.dedup();
        insert_capability(&mut entries, ui_dialog_open_capability(&dialog_ids))?;
    }
    for command in &source.os_commands {
        insert_capability(&mut entries, capability_from_os_command(command, locale, terminology))?;
    }

    for capability in &source.shell {
        insert_capability(&mut entries, capability.clone())?;
    }
    for capability in &source.gateway {
        insert_capability(&mut entries, capability.clone())?;
    }

    if !template_ids.is_empty() {
        template_ids.sort();
        template_ids.dedup();
        match entries.get_mut("artifact.create") {
            Some(capability) => capability.input_schema = with_artifact_create_templates(capability.input_schema.clone(), &template_ids),
            None => insert_capability(&mut entries, artifact_create_capability(&template_ids))?,
        }
    }

    let sorted: Vec<CapabilityDefinition> = entries.into_values().filter(|capability| audiences.contains(&capability.audience)).collect();
    let hash = compute_catalog_entries_hash(&sorted);
    Ok(Catalog { hash, entries: sorted })
}
//#endregion 🔖️Compile
