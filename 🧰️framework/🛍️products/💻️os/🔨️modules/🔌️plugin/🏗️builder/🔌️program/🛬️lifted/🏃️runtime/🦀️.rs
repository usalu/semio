//! 🏃️ Lift file: 9 item(s), about 89 lines, of `⚛️reactor/🏃️runtime/🦀️.rs` that belong to crate `plugin-program` (WP2).
//!
//! WP4 owns the source file and cuts the items here (wire_artifact_mutation_plan, key_ordered_wire_value, PluginDescriptorExtras, with_extension_inference_context, wire_list_artifact_mutations, encode_wire_serialized, ...); WP2 wires this file with one `mod` line in the crate root. Until `plugin-program` exists, the source module re-exports the items with `pub use`.
//!
//! 🧭️ WP4 cut (`RESULT-PLUGIN-WP4.md`): `encode_wire_serialized`, `key_ordered_wire_value`, `decode_wire_serialized`, `PluginDescriptorExtras`, `wire_list_artifact_mutations`, `wire_artifact_mutation_plan` and the contributed-mutation-wire test include. Three items of the plan row stay behind: `plugin_internal_fault` moved to `reactor-app` (its lowest user), `cancel_extension_inference` and `with_extension_inference_context` stay in the `extension` lift (they read `EXTENSION_BUNDLE`). The file is mounted by the old `plugin_runtime` module (`mod program_runtime_lift; pub use program_runtime_lift::*;`); WP2 wires it in the crate root and removes that mount in the same change.
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework::kernel::ActivationEvent;
use semio_framework::kernel::CapabilityRequest;
use semio_framework::kernel::QuotaSchema;
use semio_framework::AssetDeclaration;
use semio_framework::ExecutionMode;
use semio_framework::ExtensionPointDeclaration;
use semio_framework::Fault;
use semio_framework_plugin_decl_registry::artifact_registration::WireArtifactMutationPlanRequest;
use semio_framework_plugin_decl_registry::artifact_registration::WireArtifactMutationPlanResult;
use semio_framework_plugin_decl_registry::artifact_registration::contributed_mutation_plan;
use semio_framework_plugin_decl_registry::artifact_registration::mutation_roster_entries;
use semio_framework_plugin_reactor_app::plugin_internal_fault;

/// 🌉️ Wire encode over `T: ToValue` directly — no `serde_json` detour. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: every caller's `T`
/// (`Fault`, `Effect`, `DispatchReport`, …) now carries a first-party `ToValue` impl.
pub fn encode_wire_serialized<T: ToValue>(value: &T) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&key_ordered_wire_value(value.to_value()))
}
/// 🔡️ `value` with the members of every object in key-byte order, through arrays and objects alike — the one order a host
/// reader re-derives from what it decoded. The pack encoder keeps the order a value arrives in and a derived `to_value`
/// arrives in declaration order, which a reader that re-encodes what it read (the history patch of a publication, the
/// document-port control grammar) refuses as non-canonical (live fault F9 on build B1).
fn key_ordered_wire_value(value: DslValue) -> DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(key_ordered_wire_value).collect()),
        DslValue::Object(members) => {
            let mut members: Vec<(String, DslValue)> = members.into_iter().map(|(key, member)| (key, key_ordered_wire_value(member))).collect();
            members.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            DslValue::Object(members)
        }
        scalar => scalar,
    }
}
/// 🌉️ Twin of `encode_wire_serialized` above — wire decode over `T: FromValue` directly.
pub async fn decode_wire_serialized<T: FromValue>(bytes: &[u8]) -> Result<T, Fault> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| plugin_internal_fault(error.to_string()))?;
    T::from_value(value).map_err(|error| plugin_internal_fault(error.to_string()))
}
/// 🛂️ Builder-declared descriptor fields owned by the same Plugin as its manifest.
#[derive(Clone, Default)]
pub struct PluginDescriptorExtras {
    pub package_id: String,
    pub activation_events: Vec<ActivationEvent>,
    pub capability_requests: Vec<CapabilityRequest>,
    pub extension_points: Vec<ExtensionPointDeclaration>,
    pub execution: ExecutionMode,
    pub quotas: QuotaSchema,
    pub assets: Vec<AssetDeclaration>,
}
/// 📖️ This plugin's deterministic `contributor.list-artifact-mutations` roster: every owner
/// document app's `SemanticMutation::kinds()` (`"<document-schema>#<kebab-kind>"`, committed by
/// `PluginBuilder::document_app`/`::try_build`) plus every resolved `ArtifactContribution`'s
/// mutations (`"<target-document-schema>#<plugin-id>:<kebab-kind>"`, committed by
/// `PluginBuilder::try_build`/`ExtensionBundle::contributes`) — `crate::app::mutation_roster_entries`
/// sorts by `mutation_id`, so repeated calls here never reorder or duplicate rows.
pub async fn wire_list_artifact_mutations() -> Vec<u8> {
    encode_wire_serialized(&semio_framework_plugin_decl_registry::artifact_registration::mutation_roster_entries().await)
}
/// 🎯️ `contributor.artifact-mutation-plan`: decodes the target snapshot pack + contributor
/// payload, runs the registered contribution's `CompositeMutationKind::plan` through a fresh
/// `protocol::Planner`, and echoes `artifact_kind`/`mutation_id`/`revision`/`generation` back
/// alongside the resulting `owner_ops`/`label`/`foreign` — mirrors `wire_artifact_infer`'s decode →
/// validate-echo → execute → encode shape (`validate_wire_request_metadata`'s discipline, applied
/// to `artifact_kind`/`mutation_id` instead of the inference metadata tuple: `crate::
/// app::contributed_mutation_plan` rejects a request whose `artifact_kind` does not match the
/// mutation's own registered target before ever touching the snapshot/payload bytes).
pub async fn wire_artifact_mutation_plan(request: &[u8]) -> Result<Vec<u8>, Fault> {
    let request: semio_framework_plugin_decl_registry::artifact_registration::WireArtifactMutationPlanRequest = decode_wire_serialized(request).await?;
    if request.mutation_id.trim().is_empty() {
        return Err(plugin_internal_fault("artifact-mutation-plan requires a non-empty mutation_id"));
    }
    let output = semio_framework_plugin_decl_registry::artifact_registration::contributed_mutation_plan(&request.mutation_id, &request.artifact_kind, &request.snapshot_pack, &request.payload).await.map_err(|error| plugin_internal_fault(error.to_string()))?;
    let result = semio_framework_plugin_decl_registry::artifact_registration::WireArtifactMutationPlanResult {
        artifact_kind: request.artifact_kind,
        mutation_id: request.mutation_id,
        revision: request.revision,
        generation: request.generation,
        owner_ops: output.owner_ops,
        label: output.label,
        foreign: output.foreign,
    };
    Ok(encode_wire_serialized(&result))
}
