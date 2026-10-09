//! 🚪️ Extension resources and declarative payloads retain their owners through bounded close.

use super::{plugin_internal_fault, ExtensionBundle, ExtensionManifest, ExtensionRequestHandler, Fault, FaultCode, FaultOrigin};
use crate::app::PluginLifecycleStep;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};

/// 🧳️ Owns the resources captured by extension handlers; terminal destruction must be shallow.
pub trait ExtensionResourceOwner: Send + 'static {
    fn invoke(&self, capability: &str, request: &[u8]) -> Result<Vec<u8>, Fault>;
    /// 💡️ The explicitly owned native context used by registered inference executables.
    fn inference_context(&self) -> Option<&dyn std::any::Any> {
        None
    }
    /// 🛑️ Cancels retained inference work in this existing resource family.
    fn cancel_inference(&self, _cancellation_id: &str) -> bool {
        false
    }
    fn begin_close(&mut self);
    /// ♻️ Advances one owner under the complete caller grant; a turn below any demanded axis yields an empty receipt.
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn terminal_is_empty(&self) -> bool;
    /// 📏️ The minimal grant on every independent axis that the next `close_step` needs.
    fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError>;
    fn cancel_close(&mut self) {}
    fn resume_close(&mut self) {}
}

/// 🎟️ Self-funds one turn from its own quote for cold callers that no scheduler grants.
pub(super) fn cold_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn yields(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "extension retirement depth overflow"))?;
    Ok(demand)
}

fn value_fault(error: ValueError) -> Fault {
    close_fault("extension.close-demand", &error.into_message())
}

fn close_fault(code: &str, message: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)
}

impl ExtensionBundle {
    /// 🧷️ Attaches the concrete resource family captured by this bundle's handlers.
    pub fn resource_owner(mut self, owner: impl ExtensionResourceOwner) -> Self {
        assert!(self.resource_owner.is_none(), "extension resource owner already attached");
        *self.resource_owner = Some(Box::new(owner));
        self
    }

    /// 🎯️ Registers a capability implemented directly by the retained resource owner.
    pub fn owned_handler(mut self, capability: impl Into<String>) -> Self {
        assert!(self.resource_owner.is_some(), "owned extension handler requires an explicit resource owner");
        let capability = capability.into();
        assert!(!matches!(capability.as_str(), "extension.close.cancel" | "extension.close.resume"), "extension close control capability is reserved");
        self.insert_handler(capability, ExtensionRequestHandler::Owned);
        self
    }

    pub(super) fn insert_handler(&mut self, capability: String, handler: ExtensionRequestHandler) {
        match self.handlers.binary_search_by(|(registered, _)| registered.cmp(&capability)) {
            Ok(index) => self.handlers[index].1 = handler,
            Err(index) => self.handlers.insert(index, (capability, handler)),
        }
    }

    /// 🔒️ Seals invocation before requesting resource retirement, without walking payloads.
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(owner) = &mut *self.resource_owner {
            owner.begin_close();
        }
    }

    pub fn cancel_close(&mut self) {
        self.close_cancelled = true;
        if let Some(owner) = &mut *self.resource_owner {
            owner.cancel_close();
        }
    }

    pub fn resume_close(&mut self) {
        self.close_cancelled = false;
        if let Some(owner) = &mut *self.resource_owner {
            owner.resume_close();
        }
    }

    /// 📨️ Invokes only an unsealed bundle through its actual registered handler.
    pub fn invoke(&self, capability: &str, request: &[u8]) -> Result<Vec<u8>, Fault> {
        if self.closing {
            return Err(close_fault("extension.closing", "extension resource retirement has sealed invocation"));
        }
        let index =
            self.handlers.binary_search_by(|(registered, _)| registered.as_str().cmp(capability)).map_err(|_| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.unknown-capability"), format!("unknown extension capability '{capability}'")))?;
        match &self.handlers[index].1 {
            ExtensionRequestHandler::Plain(handler) => handler(request),
            ExtensionRequestHandler::Owned => self.resource_owner.as_ref().ok_or_else(|| close_fault("extension.missing-owner", "extension resource owner is absent"))?.invoke(capability, request),
        }
    }

    /// 💡️ Public named inference transport shares the registered service wire gateway and owner.
    pub fn artifact_infer(&self, request: &[u8]) -> Result<Vec<u8>, crate::app::ArtifactInferenceExecutionError> {
        if self.closing {
            return Err(crate::app::ArtifactInferenceExecutionError::new("artifact-inference.context-closing", "extension inference owner is closing"));
        }
        let context =
            self.resource_owner.as_ref().and_then(|owner| owner.inference_context()).ok_or_else(|| crate::app::ArtifactInferenceExecutionError::new("artifact-inference.context-required", "extension resource owner supplies no inference context"))?;
        crate::app::wire_artifact_infer_with_context(request, context)
    }

    /// ⏱️ Retires resource payloads, then individual handler captures, then manifest metadata.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(PluginLifecycleStep::Complete(idle));
        }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(value_fault)?;
        if grant.maximum_depth < demand.depth {
            return Err(close_fault("extension.close-depth", "extension retirement exceeds its admitted depth"));
        }
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(idle));
        }
        if self.close_cancelled {
            return Ok(PluginLifecycleStep::Blocked { reason: "extension close paused" });
        }
        self.begin_close();
        if let Some(owner) = &mut *self.resource_owner {
            if owner.terminal_is_empty() {
                let released_bytes = std::mem::size_of_val(owner.as_ref());
                self.resource_owner.take();
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..idle }));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child)?;
            if step.progress().is_some_and(|progress| !progress.fits(child)) {
                return Err(close_fault("extension.close-budget", "extension resource owner exceeded its independent grant"));
            }
            return match step {
                PluginLifecycleStep::Complete(_) if !owner.terminal_is_empty() => Err(close_fault("extension.close-live-owner", "extension resource owner reported Complete without terminal emptiness")),
                PluginLifecycleStep::Complete(progress) => Ok(PluginLifecycleStep::Progress(progress)),
                step => Ok(step),
            };
        }
        if !self.metadata_retirement.is_empty() {
            return match self.metadata_retirement.step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })? {
                PluginLifecycleStep::Complete(progress) => Ok(PluginLifecycleStep::Progress(progress)),
                step => Ok(step),
            };
        }
        if !self.handlers.is_empty() {
            let (key, _) = self.handlers.pop().expect("exclusive handler retirement retains its key");
            self.metadata_retirement.push(key);
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..idle }));
        }
        if self.handlers.capacity() != 0 {
            self.metadata_retirement.push(std::mem::take(&mut *self.handlers));
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..idle }));
        }
        if !self.manifest_retired || !self.manifest.payload_is_empty() {
            let manifest = std::mem::replace(&mut *self.manifest, ExtensionManifest::empty());
            self.manifest_retired = true;
            self.metadata_retirement.push(manifest);
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..idle }));
        }
        Ok(PluginLifecycleStep::Complete(idle))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.resource_owner.is_none() && self.handlers.is_empty() && self.handlers.capacity() == 0 && self.manifest_retired && self.manifest.payload_is_empty() && self.metadata_retirement.is_empty()
    }

    /// 📏️ Exact per-axis admission required by the next retained allocation move, birth or release.
    pub fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.resource_owner.as_ref() {
            return if owner.terminal_is_empty() { Ok(RetirementDemand { release_bytes: std::mem::size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) } else { nested(owner.retirement_demands(maximum_body_bytes)?) };
        }
        if !self.metadata_retirement.is_empty() {
            return nested(self.metadata_retirement.retirement_demands());
        }
        if !self.handlers.is_empty() {
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<(String, ExtensionRequestHandler)>(), capacity_bytes: push_bytes::<String>(), depth: 1, ..Default::default() });
        }
        if self.handlers.capacity() != 0 {
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of_val(&*self.handlers), capacity_bytes: push_bytes_of(&*self.handlers), depth: 1, ..Default::default() });
        }
        if !self.manifest_retired || !self.manifest.payload_is_empty() {
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<ExtensionManifest>(), capacity_bytes: push_bytes::<ExtensionManifest>(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    /// ❄️ Completes explicit disposal for cold construction and inspection callers.
    pub fn dispose_cold(&mut self) -> Result<(), Fault> {
        self.begin_close();
        self.resume_close();
        loop {
            let demand = self.retirement_demands(0).map_err(value_fault)?;
            match self.close_step(cold_grant(demand))? {
                PluginLifecycleStep::Complete(_) => return Ok(()),
                PluginLifecycleStep::Progress(progress) if progress != RetainedCloneProgress::default() => {}
                _ => return Err(close_fault("extension.cold-close-blocked", "cold extension disposal requires exclusive resource authority")),
            }
        }
    }

    /// 📋️ Transfers the manifest to a cold inspector after explicitly retiring its bundle.
    pub fn into_manifest_cold(mut self) -> Result<ExtensionManifest, Fault> {
        let manifest = std::mem::replace(&mut *self.manifest, ExtensionManifest::empty());
        self.manifest_retired = true;
        self.dispose_cold()?;
        Ok(manifest)
    }
}

impl ExtensionManifest {
    fn payload_is_empty(&self) -> bool {
        self.package_id.capacity() == 0
            && self.extension_id.capacity() == 0
            && self.label.capacity() == 0
            && self.version.capacity() == 0
            && self.extends.capacity() == 0
            && self.capabilities.capacity() == 0
            && self.topic_contributions.capacity() == 0
            && self.dependencies.capacity() == 0
            && self.contributions.capacity() == 0
            && self.capability_requests.capacity() == 0
    }

    pub(super) fn empty() -> Self {
        Self {
            package_id: String::new(),
            extension_id: String::new(),
            label: String::new(),
            version: String::new(),
            extends: String::new(),
            capabilities: Vec::new(),
            topic_contributions: Vec::new(),
            dependencies: Vec::new(),
            contributions: Vec::new(),
            execution: Default::default(),
            capability_requests: Vec::new(),
        }
    }
}

impl Drop for ExtensionBundle {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("extension bundle requires terminal-empty retirement before drop");
            }
            return;
        }
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.manifest);
            std::mem::ManuallyDrop::drop(&mut self.handlers);
            std::mem::ManuallyDrop::drop(&mut self.resource_owner);
            std::mem::ManuallyDrop::drop(&mut self.metadata_retirement);
        }
    }
}

pub(super) struct ExtensionBundleRegistry {
    current: Option<ExtensionBundle>,
    replacement: Option<ExtensionBundle>,
    active: bool,
    activation_requested: bool,
    retire_all: bool,
    paused: bool,
}

impl ExtensionBundleRegistry {
    pub(super) const fn new() -> Self {
        Self { current: None, replacement: None, active: false, activation_requested: false, retire_all: false, paused: false }
    }

    pub(super) fn current(&self) -> Option<&ExtensionBundle> {
        self.current.as_ref()
    }

    pub(super) fn install(&mut self, candidate: &mut Option<ExtensionBundle>) -> Result<bool, Fault> {
        if candidate.is_none() {
            return Err(close_fault("extension.install-missing", "extension install candidate is absent"));
        }
        if self.replacement.is_some() {
            return Ok(false);
        }
        if let Some(current) = &mut self.current {
            current.begin_close();
            current.resume_close();
            self.replacement = candidate.take();
        } else {
            self.current = candidate.take();
        }
        self.active = false;
        self.activation_requested = false;
        self.retire_all = false;
        self.paused = false;
        Ok(true)
    }

    pub(super) fn activate(&mut self) -> Result<(), Fault> {
        if self.replacement.is_some() && !self.retire_all {
            self.activation_requested = true;
            return Ok(());
        }
        let current = self.current.as_ref().ok_or_else(|| close_fault("extension.missing", "extension bundle not installed"))?;
        if current.closing {
            return Err(close_fault("extension.closing", "extension bundle is retiring"));
        }
        self.active = true;
        Ok(())
    }

    pub(super) fn invoke(&self, capability: &str, request: &[u8]) -> Result<Vec<u8>, Fault> {
        if !self.active {
            return Err(close_fault("extension.inactive", "extension not activated"));
        }
        self.current.as_ref().ok_or_else(|| close_fault("extension.missing", "extension bundle not installed"))?.invoke(capability, request)
    }

    pub(super) fn begin_close(&mut self) {
        self.active = false;
        self.activation_requested = false;
        self.retire_all = true;
        if let Some(current) = &mut self.current {
            current.begin_close();
        }
    }

    pub(super) fn cancel_close(&mut self) {
        self.paused = true;
        if let Some(current) = &mut self.current {
            current.cancel_close();
        }
    }

    pub(super) fn resume_close(&mut self) {
        self.paused = false;
        if let Some(current) = &mut self.current {
            current.resume_close();
        }
    }

    pub(super) fn has_retirement(&self) -> bool {
        self.current.as_ref().is_some_and(|bundle| bundle.closing) || self.replacement.is_some()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.current.is_none() && self.replacement.is_none()
    }
    pub(super) fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        match self.current.as_ref() {
            Some(current) if !current.terminal_is_empty() => nested(current.retirement_demands(maximum_body_bytes)?),
            Some(_) => Ok(RetirementDemand { depth: 1, ..Default::default() }),
            None => Ok(Default::default()),
        }
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(PluginLifecycleStep::Complete(idle));
        }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(value_fault)?;
        if grant.maximum_depth < demand.depth {
            return Err(close_fault("extension.close-depth", "extension registry retirement exceeds its admitted depth"));
        }
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(idle));
        }
        if self.paused {
            return Ok(PluginLifecycleStep::Blocked { reason: "extension close paused" });
        }
        if !self.has_retirement() {
            self.begin_close();
        }
        let current = self.current.as_mut().expect("installed extension retains a current owner");
        if !current.terminal_is_empty() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            return match current.close_step(child)? {
                PluginLifecycleStep::Complete(progress) => Ok(PluginLifecycleStep::Progress(progress)),
                step => Ok(step),
            };
        }
        self.current.take();
        if let Some(mut replacement) = self.replacement.take() {
            if self.retire_all {
                replacement.begin_close();
            }
            self.current = Some(replacement);
            self.active = self.activation_requested && !self.retire_all;
            self.activation_requested = false;
        }
        let progress = RetainedCloneProgress { copied_items: 1, ..idle };
        Ok(if self.terminal_is_empty() { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) })
    }
}

fn push_bytes<T>() -> usize {
    std::mem::size_of::<PendingMetadata>().saturating_add(std::mem::size_of::<T>())
}

fn push_bytes_of<T>(_: &T) -> usize {
    push_bytes::<T>()
}

pub(super) trait MetadataOwner: Send + 'static {
    fn release_bytes(&self) -> usize {
        std::mem::size_of_val(self)
    }
    /// 🧮️ Exact capacity of the pending nodes that expanding this owner births.
    fn expansion_bytes(&self) -> usize {
        0
    }
    fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String>;
}

pub(super) struct PendingMetadata {
    owner: Box<dyn MetadataOwner>,
    next: Option<Box<PendingMetadata>>,
}

#[derive(Default)]
pub(super) struct MetadataRetirement {
    pending: std::mem::ManuallyDrop<Option<Box<PendingMetadata>>>,
}

impl MetadataRetirement {
    pub(super) fn push<T: MetadataOwner>(&mut self, value: T) {
        *self.pending = Some(Box::new(PendingMetadata { owner: Box::new(value), next: self.pending.take() }));
    }
    pub(super) fn is_empty(&self) -> bool {
        self.pending.is_none()
    }
    pub(super) fn retirement_demands(&self) -> RetirementDemand {
        self.pending.as_ref().map_or_else(Default::default, |pending| RetirementDemand {
            capacity_bytes: pending.owner.expansion_bytes(),
            release_bytes: std::mem::size_of::<PendingMetadata>().saturating_add(pending.owner.release_bytes()),
            depth: 1,
            ..Default::default()
        })
    }

    pub(super) fn step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let idle = RetainedCloneProgress::default();
        if self.is_empty() {
            return Ok(PluginLifecycleStep::Complete(idle));
        }
        let demand = self.retirement_demands();
        if grant.maximum_depth < demand.depth {
            return Err(close_fault("extension.close-depth", "extension metadata retirement exceeds its admitted depth"));
        }
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(idle));
        }
        if let Some(pending) = self.pending.take() {
            let PendingMetadata { owner, next } = *pending;
            *self.pending = next;
            owner.expand(self).map_err(plugin_internal_fault)?;
        }
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes, ..idle };
        Ok(if self.is_empty() { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) })
    }
}

impl Drop for MetadataRetirement {
    fn drop(&mut self) {
        if !self.is_empty() {
            if !std::thread::panicking() {
                panic!("extension metadata requires terminal-empty retirement before drop");
            }
            return;
        }
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.pending);
        }
    }
}

impl MetadataOwner for String {
    fn release_bytes(&self) -> usize {
        std::mem::size_of::<Self>().saturating_add(self.capacity())
    }
    fn expand(self: Box<Self>, _close: &mut MetadataRetirement) -> Result<(), String> {
        Ok(())
    }
}
impl MetadataOwner for super::DslValue {
    fn expansion_bytes(&self) -> usize {
        match self {
            Self::String(value) => push_bytes_of(value),
            Self::Array(values) => push_bytes_of(values),
            Self::Object(values) => push_bytes_of(values),
            _ => 0,
        }
    }
    fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        match *self {
            Self::String(value) => close.push(value),
            Self::Array(values) => close.push(values),
            Self::Object(values) => close.push(values),
            _ => {}
        }
        Ok(())
    }
}
impl<T: MetadataOwner> MetadataOwner for Vec<T> {
    fn release_bytes(&self) -> usize {
        std::mem::size_of::<Self>().saturating_add(if self.is_empty() { self.capacity().saturating_mul(std::mem::size_of::<T>()) } else { 0 })
    }
    fn expansion_bytes(&self) -> usize {
        if self.is_empty() { 0 } else { push_bytes::<Self>().saturating_add(push_bytes::<T>()) }
    }
    fn expand(mut self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        if let Some(value) = self.pop() {
            close.push(*self);
            close.push(value);
        }
        Ok(())
    }
}
impl<T: MetadataOwner, U: MetadataOwner> MetadataOwner for (T, U) {
    fn expansion_bytes(&self) -> usize {
        push_bytes::<T>().saturating_add(push_bytes::<U>())
    }
    fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        let (first, second) = *self;
        close.push(first);
        close.push(second);
        Ok(())
    }
}
impl MetadataOwner for ExtensionRequestHandler {
    fn expand(self: Box<Self>, _close: &mut MetadataRetirement) -> Result<(), String> {
        Ok(())
    }
}
impl<T: MetadataOwner> MetadataOwner for Option<T> {
    fn expansion_bytes(&self) -> usize {
        if self.is_some() { push_bytes::<T>() } else { 0 }
    }
    fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        if let Some(value) = *self {
            close.push(value);
        }
        Ok(())
    }
}

macro_rules! retire_metadata {
    ($type:ty, $($field:ident),+ $(,)?) => {
        impl MetadataOwner for $type {
            fn expansion_bytes(&self) -> usize {
                0usize $(.saturating_add(push_bytes_of(&self.$field)))+
            }
            fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
                $(close.push(self.$field);)+
                Ok(())
            }
        }
    };
}

retire_metadata!(ExtensionManifest, package_id, extension_id, label, version, extends, capabilities, topic_contributions, dependencies, contributions, capability_requests);
retire_metadata!(semio_framework::TopicContribution, topic, payload);
retire_metadata!(semio_framework::PluginDependency, plugin_id);
retire_metadata!(semio_framework::ArtifactContributionDescriptor, artifact_kind, mutations, inferences);
retire_metadata!(semio_framework::ContributedMutationMetadata, mutation_id, semantics);
retire_metadata!(semio_framework::ContributedMutationSemantics, verb, entity, kind, record);
retire_metadata!(semio_framework::ContributedInferenceMetadata, owner, artifact_kind, artifact_schema, inference_schema, contributor, depends_on, payload);
retire_metadata!(semio_framework::InferencePayloadContract, payload_schema_id, input_schema, output_schema, progress_unit, artifact_binding, commit);
retire_metadata!(semio_framework::InferenceArtifactBinding, field, encoding);
retire_metadata!(semio_framework::InferenceCommitBinding, action);
retire_metadata!(semio_framework::kernel::CapabilityRequest, id, scope, reason);

impl MetadataOwner for semio_framework::CapabilityRequirement {
    fn expand(self: Box<Self>, _close: &mut MetadataRetirement) -> Result<(), String> {
        Ok(())
    }
}
impl MetadataOwner for semio_framework::kernel::CapabilityId {
    fn expansion_bytes(&self) -> usize {
        push_bytes_of(&self.0)
    }
    fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        close.push(self.0);
        Ok(())
    }
}
