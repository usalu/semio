//! 🛂️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §1 + §3; E2-builder-descriptor
//! wires the real assembly). `describe::describe` — build-time only, never called at runtime by the
//! OS. Builds a real `manifest::PackageDescriptor` from the SAME `Plugin`/`PluginManifest` bundle
//! `plugin_runtime::plugin_manifest()` already exposes, plus the builder-declared descriptor extras
//! `plugin_runtime::plugin_descriptor_extras()` carries (`PluginBuilder::try_build()`,
//! `🏗️builder/🦀️.rs`, E2's own owned path — see `PluginDescriptorExtras`'s doc for why
//! these live in a thread-local rather than on `Plugin`/`PluginManifest` themselves).
//!
//! `hashes` stay empty strings for all three fields: `wasm_sha256`/`core_wasm_sha256` can only be
//! computed AFTER this component is built (by the external `semio-framework-plugin-describe` bin
//! crate, packet E1) and `descriptor_sha256` is that same emitter's own two-pass self-hash — none of
//! the three is computable by `describe()` itself, running inside the not-yet-hashed wasm. Unchanged
//! from E1's own placeholder.

use semio_framework::{
    io, kernel, AppDefinition, ComposerEntryDescriptor, ContributedInferenceMetadata, ContributionSet, FileTypeContribution, IoEntryDescriptor, IoEntryDirection, PackageDescriptor, PackageHashes, PackageRole, PanelTabDefinition, PluginManifest,
};

/// 🗂️ Whether `artifact_kind` is owned by `plugin_id` — every plugin's own IO `Dialect.artifact_kind`
/// in the tree is the bare `"s.<plugin_id>"` coordinate (confirmed across every `const DIALECT:
/// Dialect` in `✏️s/🔌️plugins/**/🚪️io/🦀️.rs`, e.g. `"s.note"`, `"s.flow"`, `"s.raster"` —
/// distinct from the separate 3-segment `s.<plugin>.<artifact>` `ArtifactRef`/`ArtifactIdentity`
/// capability-row grammar, `🚪️io/🧬️schema/🦀️component.rs`), with `"s.<plugin_id>.<suffix>"` also
/// accepted for any dialect a plugin registers as a sub-kind of its own.
async fn owns_artifact_kind(plugin_id: &str, artifact_kind: &str) -> bool {
    artifact_kind == format!("s.{plugin_id}") || artifact_kind.starts_with(&format!("s.{plugin_id}."))
}

/// 🗂️ Flattens every app's `AppIo.export_formats`/`import_formats` into one row per distinct format
/// kind — `ContributionSet.file_types` (`📓️design-abi.md` §3), grounded in `AppIo` (E1's own doc on
/// `manifest::FileTypeContribution`).
async fn plugin_file_types(apps: &[AppDefinition]) -> Vec<FileTypeContribution> {
    let mut rows: Vec<FileTypeContribution> = Vec::new();
    for app in apps {
        let mut kinds: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
        kinds.extend(app.io.export_formats.iter());
        kinds.extend(app.io.import_formats.iter());
        for kind in kinds {
            let row = FileTypeContribution { format_kind: kind.clone(), media_type: app.io.document_media_type, imports: app.io.import_formats.contains(kind), exports: app.io.export_formats.contains(kind) };
            if !rows.contains(&row) {
                rows.push(row);
            }
        }
    }
    rows
}

/// 🗂️ Flattens every app's `panel_tabs` — `ContributionSet.panels`.
async fn plugin_panels(apps: &[AppDefinition]) -> Vec<PanelTabDefinition> {
    apps.iter().flat_map(|app| app.panel_tabs.iter().cloned()).collect()
}

/// 💡️ This plugin's frozen first-party inference roster (`owner == plugin_id`), including
/// metadata-only ActionBus cold routes that deliberately have no synchronous service facade.
async fn plugin_inference_services<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, plugin_id: &str) -> Vec<ContributedInferenceMetadata> {
    let bytes = crate::plugin_runtime::plugin_wire_list_artifact_inference_services(runtime).await.unwrap_or_default();
    std::str::from_utf8(&bytes)
        .ok()
        .and_then(|text| dsl::os_pack::json::from_json_str::<Vec<crate::app::WireArtifactInferenceMetadata>>(text).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|metadata| metadata.owner == plugin_id)
        .map(|metadata| ContributedInferenceMetadata {
            owner: metadata.owner.clone(),
            artifact_kind: metadata.artifact_kind,
            artifact_schema: metadata.artifact_schema,
            artifact_schema_version: metadata.artifact_schema_version,
            document_schema: metadata.document_schema,
            document_schema_version: metadata.document_schema_version,
            inference_schema: metadata.inference_schema,
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            contributor: metadata.owner,
            depends_on: Vec::new(),
        })
        .collect()
}

/// 🚪️ This plugin's own registered IO composer routes (`writes.artifact_kind` owned by
/// `plugin_id`) — `ContributionSet.io_entries`/`composer_entries`, reading the real
/// `io::list_composer_entries()` registry (`🚪️io/🦀️.rs`'s `IO_REGISTRY`). Each `(writes,
/// reads)` composer row yields one `IoEntryDescriptor{owner: writes, counterpart: read, direction:
/// Import}` per read dialect — `writes` is composed FROM `reads` (`ComposerEntry`'s own doc), so
/// `Import` is the faithful direction from this package's (the `writes` owner's) perspective.
///
/// ⚠️ `ContributionSet.mutation_services` has NO equivalent version-tracked registry to read from:
/// the owner mutation roster (`crate::app::mutation_roster_entries`, `WireMutationRosterEntry`)
/// never carries `schema_version`/`algorithm_version` for a document app's own `SemanticMutation::
/// kinds()` rows (only the inference registry tracks versions) — `ContributedMutationMetadata`
/// requires both. Populating it would mean fabricating version numbers nothing in the codebase
/// declares, so it stays empty, the exact "not invented" discipline E1 already applied to
/// `menus`/`themes`.
async fn plugin_io_contributions(plugin_id: &str) -> (Vec<IoEntryDescriptor>, Vec<ComposerEntryDescriptor>) {
    let mut io_entries = Vec::new();
    let mut composer_entries = Vec::new();
    for (writes, reads) in io::list_composer_entries().await.unwrap_or_default() {
        if !owns_artifact_kind(plugin_id, &writes.artifact_kind).await {
            continue;
        }
        for read in &reads {
            io_entries.push(IoEntryDescriptor { owner: writes.clone(), counterpart: read.clone(), direction: IoEntryDirection::Import });
        }
        composer_entries.push(ComposerEntryDescriptor { writes, reads });
    }
    (io_entries, composer_entries)
}

/// 🗂️ Assembles `ContributionSet` from what `manifest` and the process-global runtime registries
/// actually declare — see each field helper's own doc. `menus`/`themes` stay empty (E1's own survey,
/// unchanged); `mutation_services` stays empty (see `plugin_io_contributions`'s doc).
async fn plugin_contributions<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, manifest: &PluginManifest) -> ContributionSet {
    let (io_entries, composer_entries) = plugin_io_contributions(&manifest.plugin_id).await;
    ContributionSet {
        commands: manifest.commands.clone(),
        menus: Vec::new(),
        file_types: plugin_file_types(&manifest.apps).await,
        panels: plugin_panels(&manifest.apps).await,
        themes: Vec::new(),
        topic_contributions: manifest.topic_contributions.clone(),
        artifact_contributions: manifest.contributions.clone(),
        inference_services: plugin_inference_services(runtime, &manifest.plugin_id).await,
        mutation_services: Vec::new(),
        io_entries,
        composer_entries,
    }
}

fn encode_package_descriptor(descriptor: &PackageDescriptor) -> Vec<u8> {
    let value = dsl::to_dsl_value(descriptor).unwrap_or_else(|error| {
        let bounded = error.chars().take(1024).collect::<String>();
        panic!("package descriptor structural encoding failed: {bounded}")
    });
    store::pack_rt::encode_wire_value(&value)
}

pub async fn describe_plugin<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Vec<u8> {
    let manifest = crate::plugin_runtime::plugin_manifest(runtime).await;
    let extras = crate::plugin_runtime::plugin_descriptor_extras().await;
    let contributions = plugin_contributions(runtime, &manifest).await;
    let descriptor = PackageDescriptor {
        descriptor_version: 1,
        package_id: extras.package_id,
        role: PackageRole::Plugin,
        manifest,
        activation_events: extras.activation_events,
        capability_requests: extras.capability_requests,
        extension_points: extras.extension_points,
        execution: extras.execution,
        quotas: extras.quotas,
        contributions,
        assets: extras.assets,
        hashes: PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() },
    };
    encode_package_descriptor(&descriptor)
}

/// 🧩️ E1-describe: the `extension_exports!` counterpart of `describe_plugin` — added alongside it
/// (this file's own owner packet, per its header doc) so `descriptor_is_fresh()` has something to
/// call for an extension crate too, not just a plugin one. `ExtensionManifest`'s fields (guest-side,
/// `🔌️plugin/🦀️.rs`) map field-for-field onto `PluginManifest` (`extension_id` ->
/// `plugin_id`; no `apps`/`examples`/`commands`/`artifact_kinds` — an extension has none of those).
/// E2-builder-descriptor: `execution`/`capability_requests` now come straight off `ExtensionManifest`
/// (`ExtensionBundle::mode(..)`/`.requests(..)`) — no side channel needed here, unlike
/// `describe_plugin`, because `ExtensionManifest` itself lives inside `plugin_runtime`, E2's own
/// owned region, so it could just gain the fields directly. `extension_points`/`activation_events`
/// stay empty: extension points are published BY host plugins (`PluginBuilder::extension_point`),
/// never by the extension attaching to one; an extension's own activation is entirely driven by the
/// host's `ExtensionPointDeclaration.activation`, not a declaration of its own.
pub async fn describe_extension() -> Vec<u8> {
    let extension = crate::plugin_runtime::extension_manifest().await;
    let package_id = extension.package_id.clone();
    let manifest = PluginManifest {
        plugin_id: extension.extension_id,
        label: extension.label,
        version: extension.version,
        apps: Vec::new(),
        examples: Vec::new(),
        capabilities: extension.capabilities,
        topic_contributions: extension.topic_contributions.clone(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies: extension.dependencies,
        contributions: extension.contributions.clone(),
    };
    let contributions = ContributionSet {
        commands: Vec::new(),
        menus: Vec::new(),
        file_types: Vec::new(),
        panels: Vec::new(),
        themes: Vec::new(),
        topic_contributions: extension.topic_contributions,
        artifact_contributions: extension.contributions,
        inference_services: Vec::new(),
        mutation_services: Vec::new(),
        io_entries: Vec::new(),
        composer_entries: Vec::new(),
    };
    let descriptor = PackageDescriptor {
        descriptor_version: 1,
        package_id,
        role: PackageRole::Extension,
        manifest,
        activation_events: Vec::new(),
        capability_requests: extension.capability_requests,
        extension_points: Vec::new(),
        execution: extension.execution,
        quotas: kernel::QuotaSchema::default(),
        contributions,
        assets: Vec::new(),
        hashes: PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() },
    };
    encode_package_descriptor(&descriptor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn package_descriptor_advertises_metadata_only_cold_inference_routes() {
        let metadata = crate::app::ArtifactInferenceServiceMetadata {
            owner: "describe-routed-inference",
            artifact_kind: "s.describe.route",
            artifact_schema: "s.describe.route",
            artifact_schema_version: 1,
            document_schema: "s.describe.route",
            document_schema_version: 1,
            inference_schema: "s.describe.route.solve",
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
        };
        let plugin = crate::app::Plugin::<crate::app::NoPluginApp>::builder(metadata.owner).label("Describe Routed Inference").version("0.1.0").routed_inference(metadata).try_build().expect("routed plugin assembles");
        let runtime = crate::plugin_runtime::PluginRuntime::new();
        crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
        let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
        let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect("descriptor shape decodes");
        assert_eq!(descriptor.contributions.inference_services.len(), 1);
        let route = &descriptor.contributions.inference_services[0];
        assert_eq!((route.owner.as_str(), route.artifact_kind.as_str(), route.inference_schema.as_str()), (metadata.owner, metadata.artifact_kind, metadata.inference_schema));
        assert!(crate::app::artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none());
    }
}
