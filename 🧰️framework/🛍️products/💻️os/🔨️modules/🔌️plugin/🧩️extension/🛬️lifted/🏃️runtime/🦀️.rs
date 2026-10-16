//! 🏃️ Lift file: 23 item(s), about 616 lines, of `⚛️reactor/🏃️runtime/🦀️.rs` that belong to crate `extension` (WP3).
//!
//! WP4 owns the source file and cuts the items here (ExtensionBundle, extension_exports, ExtensionManifest, extension_invoke, foreign_kernel_value_bridge, extension_dispose_cold, ...); WP3 wires this file with one `mod` line in the crate root. Until `extension` exists, the source module re-exports the items with `pub use`.
//!
//! 🧭️ WP4 cut (`RESULT-PLUGIN-WP4.md`): the region 5189 to 5496 of the old runtime file, the two inference wire entry points of `runtime-instances` (`plugin_wire_list_artifact_inference_services`, `plugin_wire_artifact_infer_cold`) and, by plan defect PD7, `cancel_extension_inference` and `with_extension_inference_context`, which read the `EXTENSION_BUNDLE` static of this crate and therefore cannot sit in `plugin-program`. `extension_exports!` is not here: it is a facade macro (12.5.6) in `📣️macros/⚛️wp4-reactor`. The file is mounted by the old `plugin_runtime` module (`mod extension_runtime_lift; pub use extension_runtime_lift::*;`); WP3 wires it in the crate root and removes that mount in the same change, or `EXTENSION_BUNDLE` exists twice.

use super::*;
/// 💡️ Lists only inference services frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (no real suspension
/// point: the found-plugin branch resolves a frozen in-memory assembly).
pub async fn plugin_wire_list_artifact_inference_services<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> Result<Vec<u8>, crate::artifact_inference::ArtifactInferenceExecutionError> {
    {
        let plugin = runtime.plugin.borrow();
        let plugin = plugin.as_ref().ok_or_else(|| crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.not-registered", "no plugin bundle is installed"))?;
        Ok(plugin.wire_list_artifact_inference_services())
    }
}
/// 💡️ Executes only an inference service frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (no real suspension
/// point: the found-plugin branch resolves a frozen in-memory assembly).
pub async fn plugin_wire_artifact_infer_cold<PA: PluginApp>(runtime: &PluginRuntime<PA>, request: &[u8],operation:u64,generation:u64,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<u8>, crate::artifact_inference::ArtifactInferenceExecutionError> {
    {
        let plugin = runtime.plugin.borrow();
        let plugin = plugin.as_ref().ok_or_else(|| crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.not-registered", "no plugin bundle is installed"))?;
        plugin.wire_artifact_infer_cold(request,operation,generation,cx)
    }
}
/// 📨️ Encoded extension request handler owned by its guest bundle.
enum ExtensionRequestHandler {
    Plain(fn(&[u8],&mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault>),
    Owned,
}
#[path = "../../🚪️retirement/🦀️.rs"]
mod extension_retirement;
pub use extension_retirement::{ExtensionResourceOwner,ExtensionInvokeStep};
use extension_retirement::{ExtensionBundleRegistry, MetadataRetirement};
/// 🧩️ Extension guest bundle — no apps; contributes + invoke handlers.
pub struct ExtensionBundle {
    pub manifest: std::mem::ManuallyDrop<ExtensionManifest>,
    handlers: std::mem::ManuallyDrop<Vec<(String, ExtensionRequestHandler)>>,
    resource_owner: std::mem::ManuallyDrop<Option<Box<dyn ExtensionResourceOwner>>>,
    pub(crate) closing: bool,
    close_cancelled: bool,
    manifest_retired: bool,
    metadata_retirement: std::mem::ManuallyDrop<MetadataRetirement>,
    inference_gateway:std::mem::ManuallyDrop<std::sync::Mutex<Option<crate::ArtifactInferenceGateway>>>,
}
/// 🌉️ `CapabilityRequirement`/`PluginDependency`/`ArtifactContributionDescriptor`/`ExecutionMode`
/// are foreign (owned by the `semio-framework` crate's kernel/manifest vocabulary) and only
/// implement `Serialize`/`Deserialize`, not `ToValue`/`FromValue` — orphan rule forbids
/// implementing either trait for them here. `ExtensionManifest` below bridges each through
/// `serde_json` via `#[value(serialize_with = …, deserialize_with = …)]` instead of deriving;
/// generic over `T` so one pair covers both the bare `ExecutionMode` field and the `Vec<_>`
/// fields. `CapabilityRequest` now derives `ToValue`/`FromValue` directly upstream (kernel), so
/// `capability_requests` below uses the default derive instead of this bridge.
mod foreign_kernel_value_bridge {
    pub fn to_value<T: serde::Serialize>(value: &T) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::from(&serde_json::to_value(value).expect("value always serializes"))
    }
    pub fn from_value<T: serde::de::DeserializeOwned>(value: semio_framework_value::DslValue) -> Result<T, semio_framework_value::ValueError> {
        serde_json::from_value(serde_json::Value::from(value)).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}
/// 📦️ Manifest for a runtime-installable extension (WIT `extension::manifest` payload).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ExtensionManifest {
    pub package_id: String,
    pub extension_id: String,
    pub label: String,
    pub version: String,
    pub extends: String,
    #[value(default, skip_serializing_if = "Vec::is_empty", serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub capabilities: Vec<CapabilityRequirement>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_contributions: Vec<TopicContribution>,
    /// 🔗️ Direct plugin dependencies this extension requires — contract freeze §3/§4: `extends`
    /// MUST equal `dependencies[0].plugin_id`; `.extends`/`.depends_on` both assert this on every
    /// call that could violate it. Real `semio_framework::PluginDependency` (not a local mirror —
    /// `semio-framework-plugin` already depends on `semio-framework`, unlike the `.sxt` package
    /// format's own `ExtensionPackageManifest`).
    #[value(default, skip_serializing_if = "Vec::is_empty", serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub dependencies: Vec<semio_framework::PluginDependency>,
    /// 🗂️ Artifact-kind contributions this extension contributes onto artifact kinds it depends on.
    #[value(default, skip_serializing_if = "Vec::is_empty", serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub contributions: Vec<semio_framework::ArtifactContributionDescriptor>,
    /// 🚦️ `📓️design-abi.md` §5 — how this extension's actor runs relative to its host plugin.
    /// Default `Isolated` (`ExecutionMode::default()`), set via `ExtensionBundle::mode(..)`.
    #[value(default, serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub execution: ExecutionMode,
    /// 🙏️ Capability asks the broker resolves at install/link/runtime, set via
    /// `ExtensionBundle::requests(..)` — the NEW broker-scoped `kernel::CapabilityRequest`, not
    /// the older kernel-level `CapabilityRequirement` `capabilities` above carries (see that
    /// field's own doc for why both exist).
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_requests: Vec<CapabilityRequest>,
}
impl ExtensionBundle {
    /// 🧩️ Starts an extension bundle with identity + version.
    /// 🚫️async: E1 — pure struct literal, zero suspension points; reverted per R9 alongside
    /// `depends_on`/`extends`/`assert_extends_matches_primary_dependency` below (their only
    /// sync-closure `catch_unwind` test consumers are language-barred from async). `contributes`
    /// stays genuinely async — real registry I/O, not touched.
    pub fn new(extension_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
        let extension_id = extension_id.into();
        Self {
            manifest: std::mem::ManuallyDrop::new(ExtensionManifest {
                package_id: format!("semio:{extension_id}"),
                extension_id,
                label: label.into(),
                version: version.into(),
                extends: String::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
                execution: ExecutionMode::default(),
                capability_requests: Vec::new(),
            }),
            handlers: std::mem::ManuallyDrop::new(Vec::new()),
            resource_owner: std::mem::ManuallyDrop::new(None),
            closing: false,
            close_cancelled: false,
            manifest_retired: false,
            metadata_retirement: std::mem::ManuallyDrop::new(MetadataRetirement::default()),
            inference_gateway:std::mem::ManuallyDrop::new(std::sync::Mutex::new(None)),
        }
    }
    /// 🔗 Declares the host app/plugin this extension extends.
    pub fn extends(mut self, extends: impl Into<String>) -> Self {
        self.manifest.extends = extends.into();
        self.assert_extends_matches_primary_dependency();
        self
    }
    /// 📦️ Declares the canonical component package identity emitted by `describe()`.
    pub fn package_id(mut self, package_id: impl Into<String>) -> Self {
        self.manifest.package_id = package_id.into();
        self
    }
    /// 🔗️ Declares a direct plugin dependency, pinned exactly (`semio_framework::tree_pin!()`) — contract freeze §3/§4:
    /// the FIRST dependency declared (in call order) must be the same plugin `.extends(...)` names.
    pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionPin) -> Self {
        self.manifest.dependencies.push(semio_framework::PluginDependency::new(plugin_id, version));
        self.assert_extends_matches_primary_dependency();
        self
    }
    /// 🚨️ Contract freeze §3/§4: `extends == dependencies[0].plugin_id`. Panics on violation — the
    /// same misuse-reporting idiom `Plugin::plugin_command`'s `assert!`s already use elsewhere in
    /// this file, since every `ExtensionBundle` builder method is infallible (`-> Self`, never
    /// `Result`). Called from both `.extends`/`.depends_on` so the check holds regardless of call
    /// order; a no-op until both fields are non-empty.
    fn assert_extends_matches_primary_dependency(&self) {
        if self.manifest.extends.is_empty() {
            return;
        }
        if let Some(primary) = self.manifest.dependencies.first() {
            assert_eq!(
                primary.plugin_id, self.manifest.extends,
                "extension {:?} declares extends={:?} but its first dependency is {:?} — contract freeze §3/§4 requires extends == dependencies[0].plugin_id",
                self.manifest.extension_id, self.manifest.extends, primary.plugin_id
            );
        }
    }
    /// 🔒️ Declares a capability requirement for the extension.
    pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
        if !self.manifest.capabilities.contains(&capability) {
            self.manifest.capabilities.push(capability);
        }
        self
    }
    /// 🗂️ Adds an open topic contribution to the extension manifest; `topic` reuses this crate's
    /// own `contributes`/`consumes` metadata vocabulary (e.g. `"cad.computer"`).
    pub fn contributes_topic(mut self, topic: impl Into<String>, payload: DslValue) -> Self {
        self.manifest.topic_contributions.push(TopicContribution::new(topic, payload));
        self
    }
    /// 🗂️ Declares one contribution of mutations/inferences onto an artifact kind owned by a
    /// dependency. Resolved IMMEDIATELY (unlike `PluginBuilder::contributes`, an extension's own id
    /// is already known at `ExtensionBundle::new()`): gate-checked via
    /// `crate::app::register_contributions` and committed to the same process-global runtime
    /// registries `PluginBuilder::try_build` commits to. Panics on a gate violation — same idiom as
    /// `.extends`/`.depends_on` above.
    pub async fn contributes(mut self, contribution: crate::artifact_registration::ArtifactContribution) -> Self {
        let contributor = self.manifest.extension_id.clone();
        let (descriptor, inference_services, mutation_runtime) = contribution.resolve(&contributor);
        crate::artifact_registration::register_contributions(&contributor, &self.manifest.dependencies, std::slice::from_ref(&descriptor)).unwrap_or_else(|error| panic!("extension {contributor:?} contribution rejected: {error}"));
        crate::artifact_inference::preflight_artifact_inference_services(&inference_services).unwrap_or_else(|error| panic!("extension {contributor:?} contributed inference service conflict: {error}"));
        crate::artifact_inference::register_artifact_inference_services(inference_services).unwrap_or_else(|error| panic!("extension {contributor:?} contributed inference service registration failed: {error}"));
        crate::artifact_registration::commit_contributed_mutation_services(mutation_runtime).await.unwrap_or_else(|error| panic!("extension {contributor:?} contributed mutation registration failed: {error}"));
        self.manifest.contributions.push(descriptor);
        self
    }
    /// 🔀️ Registers a capability handler invoked via WIT `extension::invoke`.
    pub fn handler(mut self, capability: impl Into<String>, handler: fn(&[u8],&mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault>) -> Self {
        let capability = capability.into();
        assert!(!matches!(capability.as_str(), "extension.close.cancel" | "extension.close.resume"), "extension close control capability is reserved");
        self.insert_handler(capability, ExtensionRequestHandler::Plain(handler));
        self
    }
    /// 🚦️ Sets how this extension's actor runs relative to its host plugin — default `Isolated`
    /// (`📓️design-abi.md` §5).
    pub fn mode(mut self, mode: ExecutionMode) -> Self {
        self.manifest.execution = mode;
        self
    }
    /// 🙏️ Declares one capability ask the broker resolves at install/link/runtime
    /// (`📓️design-abi.md` §5). Repeatable; idempotent for a byte-identical ask.
    pub fn requests(mut self, request: CapabilityRequest) -> Self {
        if !self.manifest.capability_requests.contains(&request) {
            self.manifest.capability_requests.push(request);
        }
        self
    }
}
#[cfg(test)]
include!("../../../🧪️tests/🔬️plugin-runtime-extension-bundle-dependency/🦀️.rs");
#[cfg(test)]
include!("../../../🧪️tests/🔬️extension-retirement/🦀️.rs");
crate::component_persistent_local! {
    static EXTENSION_BUNDLE: RefCell<ExtensionBundleRegistry> = RefCell::new(ExtensionBundleRegistry::new());
}
/// 📤️ Admits one replacement while retaining the current owner; refusal leaves the candidate untouched.
pub async fn install_extension_bundle(candidate: &mut Option<ExtensionBundle>) -> Result<bool, Fault> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension install authority is busy"))?.install(candidate))
}
/// 📦️ Returns the installed extension manifest (empty defaults when unset).
pub async fn extension_manifest() -> ExtensionManifest {
    EXTENSION_BUNDLE.with(|slot| slot.borrow().current().map_or_else(ExtensionManifest::empty, |bundle| (*bundle.manifest).clone()))
}
/// 🚨️ Requests activation after any admitted replacement has retired its predecessor.
pub async fn extension_activate() -> Result<(), Fault> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension activation authority is busy"))?.activate())
}
/// 🔒️ Seals invocation and retains all installed resource owners for budgeted retirement.
pub async fn extension_deactivate() {
    EXTENSION_BUNDLE.with(|slot| slot.borrow_mut().begin_close());
}
/// ❄️ Explicitly disposes the installed registry for cold inspection and test callers.
pub fn extension_dispose_cold(cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    EXTENSION_BUNDLE.with(|slot| {
        let mut registry = slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension cold disposal authority is busy"))?;
        registry.begin_close();
        registry.resume_close();
        loop {
            let demand = registry.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
            match cx.with_retained_loan(extension_retirement::cold_grant(demand),|same|registry.close_step(same))? {
                PluginLifecycleStep::Complete(_) => return Ok(()),
                PluginLifecycleStep::Progress(progress) if progress != RetainedCloneProgress::default() => {}
                _ => return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"cold extension disposal requires its remaining original caller authority").into()),
            }
        }
    })
}
/// ♻️ Advances the installed registry under the caller's complete grant; a grant below any demanded axis yields an empty receipt.
pub fn extension_close_step(cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension close authority is busy"))?.close_step(cx))
}
pub fn extension_terminal_is_empty() -> bool {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow().is_ok_and(|registry| registry.terminal_is_empty()))
}
/// 📏️ Returns the minimal grant on every independent axis that the next close turn needs, without advancing its cursor.
pub fn extension_retirement_demands(maximum_body_bytes: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension close demand authority is busy"))?.retirement_demands(maximum_body_bytes))
}
pub fn extension_cancel_close() {
    EXTENSION_BUNDLE.with(|slot| slot.borrow_mut().cancel_close());
}
pub fn extension_resume_close() {
    EXTENSION_BUNDLE.with(|slot| slot.borrow_mut().resume_close());
}
pub(crate) fn extension_retirement_turn(cx:&mut semio_framework_job::StepContext<'_>) -> Result<(bool, PluginLifecycleStep), semio_framework_job::OriginalTurnFailure> {
    EXTENSION_BUNDLE.with(|slot| {
        let mut registry = slot.try_borrow_mut().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension retirement authority is busy"))?;
        if !registry.has_retirement() {
            return Ok((false, PluginLifecycleStep::AwaitingInput { reason: "extension retirement awaits an explicit close request" }));
        }
        let step = registry.close_step(cx)?;
        let more_work = registry.has_retirement() && !matches!(step, PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. });
        Ok((more_work, step))
    })
}
/// 🔀️ Dispatches through the current unsealed owner's registered capability handler.
pub async fn extension_invoke(capability: &str, request: &[u8], cx:&mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> {
    let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None})}
    if matches!(capability, "extension.close.cancel" | "extension.close.resume") {
        if !request.is_empty() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("extension.close-control-payload"), "extension close controls require an empty request"));
        }
        if capability == "extension.close.cancel" {
            extension_cancel_close();
        } else {
            extension_resume_close();
        }
        let retained_progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(retained_progress).map_err(|error|FaultFrom::to_fault(&error))?;
        return Ok(ExtensionInvokeStep{payload:Some(Vec::new()),retained_progress,refusal:None});
    }
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension invocation authority is busy"))?.invoke(capability, request, cx))
}
/// 🧷️ Supplies the current extension's retained inference context to the standard gateway.
pub(crate) fn cancel_extension_inference(cancellation_id: &str) -> bool {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow().ok().and_then(|state| state.current().and_then(|bundle| bundle.resource_owner.as_ref()).map(|owner| owner.cancel_inference(cancellation_id))).unwrap_or(false))
}
pub(crate) fn with_original_extension_inference_context<T>(run:impl FnOnce(Option<&dyn std::any::Any>)->Result<T,semio_framework_value::ValueError>)->Result<T,semio_framework_value::ValueError>{
    EXTENSION_BUNDLE.with(|slot|{
        let registry=slot.try_borrow().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension inference owner is busy"))?;
        let bundle=registry.current();
        if bundle.is_some_and(|bundle|bundle.closing){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension inference owner is closing"));}
        run(bundle.and_then(|bundle|bundle.resource_owner.as_ref()).and_then(|owner|owner.inference_context()))
    })
}
pub(crate) fn with_extension_inference_context<T>(run: impl FnOnce(Option<&dyn std::any::Any>) -> Result<T, crate::artifact_inference::ArtifactInferenceExecutionError>) -> Result<T, crate::artifact_inference::ArtifactInferenceExecutionError> {
    EXTENSION_BUNDLE.with(|slot| {
        let registry = slot.try_borrow().map_err(|_| crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.context-busy", "extension inference owner is busy"))?;
        let bundle = registry.current();
        if bundle.is_some_and(|bundle| bundle.closing) {
            return Err(crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.context-closing", "extension inference owner is closing"));
        }
        run(bundle.and_then(|bundle| bundle.resource_owner.as_ref()).and_then(|owner| owner.inference_context()))
    })
}
/// 🚧️ PLACEHOLDER for W1-A. PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS
/// (§6): `extension-world` now exports `contributor` alongside `extension`, but no extension has
/// ever wired inference services — this is a deterministic EMPTY roster
/// (`store::pack_rt::encode_wire_value` of an empty list) until W1-A adds real extension-side
/// inference wiring (mirroring `crate::app::wire_list_artifact_inference_services`, plugin-side).
pub async fn extension_wire_list_artifact_inferences() -> Vec<u8> {
    store::pack_rt::encode_wire_value(&DslValue::Array(Vec::new()))
}
/// 🚧️ PLACEHOLDER for W1-A. Companion to `extension_wire_list_artifact_inferences` — since the
/// roster above is always empty, this can never legitimately be called; it exists only so
/// `contributor.artifact-infer` type-checks on `extension-world`. Always returns a typed `Fault`.
pub async fn extension_wire_artifact_infer_cold() -> Fault {
    plugin_internal_fault("artifact-infer not implemented for extensions (W1-A placeholder)")
}
