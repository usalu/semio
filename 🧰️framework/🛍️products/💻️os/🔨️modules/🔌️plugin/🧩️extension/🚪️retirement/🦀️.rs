//! 🚪️ Extension resources and declarative payloads retain their owners through bounded close.

use super::{plugin_internal_fault, ExtensionBundle, ExtensionManifest, ExtensionRequestHandler, Fault, FaultCode, FaultOrigin, PluginCloseStep};
use store::os_store::SnapshotRetirementStep;

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
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;
    fn terminal_is_empty(&self) -> bool;
    fn next_close_byte_demand(&self) -> usize {
        1
    }
    fn cancel_close(&mut self) {}
    fn resume_close(&mut self) {}
}

pub(super) fn snapshot_close_step(step: SnapshotRetirementStep) -> PluginCloseStep {
    match step {
        SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
        SnapshotRetirementStep::Complete => PluginCloseStep::Complete,
        SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "extension metadata owner is shared" },
    }
}

fn close_fault(code: &str, message: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)
}

pub(super) fn admit_step(step: PluginCloseStep, items: usize, bytes: usize) -> Result<PluginCloseStep, Fault> {
    if matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > items || released_bytes > bytes) {
        return Err(close_fault("extension.close-budget", "extension resource owner exceeded its item or byte grant"));
    }
    Ok(step)
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
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.terminal_is_empty() {
            return Ok(PluginCloseStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.close_cancelled {
            return Ok(PluginCloseStep::Blocked { reason: "extension close paused" });
        }
        self.begin_close();
        if let Some(owner) = &mut *self.resource_owner {
            if owner.terminal_is_empty() {
                let released_bytes = std::mem::size_of_val(owner.as_ref());
                if released_bytes > maximum_bytes {
                    return Ok(PluginCloseStep::AwaitingInput { reason: "extension resource shell requires a larger byte grant" });
                }
                self.resource_owner.take();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes });
            }
            let step = admit_step(owner.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes)?;
            if step == PluginCloseStep::Complete && !owner.terminal_is_empty() {
                return Err(close_fault("extension.close-live-owner", "extension resource owner reported Complete without terminal emptiness"));
            }
            return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 0, released_bytes: 0 } } else { step });
        }
        if !self.metadata_retirement.is_empty() {
            let step = self.metadata_retirement.step(maximum_items, maximum_bytes)?;
            return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 0, released_bytes: 0 } } else { step });
        }
        if !self.handlers.is_empty() {
            let (key, _) = self.handlers.pop().expect("exclusive handler retirement retains its key");
            self.metadata_retirement.push(key);
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.handlers.capacity() != 0 {
            self.metadata_retirement.push(std::mem::take(&mut *self.handlers));
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.manifest_retired || !self.manifest.payload_is_empty() {
            let manifest = std::mem::replace(&mut *self.manifest, ExtensionManifest::empty());
            self.manifest_retired = true;
            self.metadata_retirement.push(manifest);
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.resource_owner.is_none() && self.handlers.is_empty() && self.handlers.capacity() == 0 && self.manifest_retired && self.manifest.payload_is_empty() && self.metadata_retirement.is_empty()
    }

    /// 📏️ Exact byte admission required by the next retained allocation release.
    pub fn next_close_byte_demand(&self) -> usize {
        if let Some(owner) = self.resource_owner.as_ref() {
            return if owner.terminal_is_empty() { std::mem::size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() };
        }
        if !self.metadata_retirement.is_empty() {
            return self.metadata_retirement.next_close_byte_demand();
        }
        1
    }

    /// ❄️ Completes explicit disposal for cold construction and inspection callers.
    pub fn dispose_cold(&mut self) -> Result<(), Fault> {
        self.begin_close();
        self.resume_close();
        loop {
            match self.close_step(64, 65536.max(self.next_close_byte_demand()))? {
                PluginCloseStep::Complete => return Ok(()),
                PluginCloseStep::Pending { .. } => {}
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
    pub(super) fn next_close_byte_demand(&self) -> usize {
        self.current.as_ref().map_or(1, ExtensionBundle::next_close_byte_demand)
    }

    pub(super) fn close_step(&mut self, items: usize, bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.terminal_is_empty() {
            return Ok(PluginCloseStep::Complete);
        }
        if items == 0 || bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.paused {
            return Ok(PluginCloseStep::Blocked { reason: "extension close paused" });
        }
        if !self.has_retirement() {
            self.begin_close();
        }
        let current = self.current.as_mut().expect("installed extension retains a current owner");
        if !current.terminal_is_empty() {
            let step = current.close_step(items, bytes)?;
            return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 0, released_bytes: 0 } } else { step });
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
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }
}

pub(super) trait MetadataOwner: Send + 'static {
    fn release_bytes(&self) -> usize {
        std::mem::size_of_val(self)
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
    pub(super) fn next_close_byte_demand(&self) -> usize {
        self.pending.as_ref().map_or(0, |pending| std::mem::size_of::<PendingMetadata>().saturating_add(pending.owner.release_bytes()))
    }

    pub(super) fn step(&mut self, items: usize, bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.is_empty() {
            return Ok(PluginCloseStep::Complete);
        }
        if items == 0 || bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released_bytes = self.next_close_byte_demand();
        if released_bytes > bytes {
            return Ok(PluginCloseStep::AwaitingInput { reason: "extension metadata allocation requires a larger byte grant" });
        }
        if let Some(pending) = self.pending.take() {
            let PendingMetadata { owner, next } = *pending;
            *self.pending = next;
            owner.expand(self).map_err(plugin_internal_fault)?;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes });
        }
        Ok(PluginCloseStep::Complete)
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
    fn expand(mut self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        if let Some(value) = self.pop() {
            close.push(*self);
            close.push(value);
        }
        Ok(())
    }
}
impl<T: MetadataOwner, U: MetadataOwner> MetadataOwner for (T, U) {
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
    fn expand(self: Box<Self>, close: &mut MetadataRetirement) -> Result<(), String> {
        close.push(self.0);
        Ok(())
    }
}
