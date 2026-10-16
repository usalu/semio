//! 🧱️ The `state` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use std::sync::Arc;
use crate::directory_events::DirectoryEventPageHttpControl;
use directory::os_directory::DocumentDescriptor;
use semio_hub_runtime::startup::DocumentOpenCatalogAuthorityV1;
use semio_hub_ledgers::socket_grants::SocketBindingValidityV1;
use semio_hub_trusted_catalog_descriptor::{TrustedCatalogAsset, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets};
#[cfg(any(test, feature = "testkit"))]
use std::sync::Mutex;

//#region 🔖️State
/// 🔎️ The exact reason one document-open plan lost its socket authority. Six distinct refusals
/// collapse into close code `4401`; this names which. Test-only: the close frame is unchanged.
#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentPlanRefusalV1 {
    PlanInvalid,
    BindingMismatch,
    DescriptorMissing,
    DescriptorDiffers,
    CatalogSelectionDiffers,
    DirectoryRevisionDiffers,
}

#[cfg(any(test, feature = "testkit"))]
static LAST_DOCUMENT_PLAN_REFUSAL: Mutex<Option<DocumentPlanRefusalV1>> = Mutex::new(None);

#[cfg(any(test, feature = "testkit"))]
pub fn record_document_plan_refusal(refusal: DocumentPlanRefusalV1) -> SocketBindingValidityV1 {
    *LAST_DOCUMENT_PLAN_REFUSAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(refusal);
    SocketBindingValidityV1::Unauthorized
}

#[cfg(any(test, feature = "testkit"))]
pub fn last_document_plan_refusal() -> Option<DocumentPlanRefusalV1> {
    LAST_DOCUMENT_PLAN_REFUSAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

#[cfg(any(test, feature = "testkit"))]
pub struct TestLiveGate {
    pub document_subscribed: tokio::sync::Semaphore,
    pub document_release: tokio::sync::Semaphore,
    directory_subscribed: tokio::sync::Semaphore,
    directory_release: tokio::sync::Semaphore,
    pub socket_before_welcome: tokio::sync::Semaphore,
    pub socket_welcome_release: tokio::sync::Semaphore,
    pub socket_after_welcome: tokio::sync::Semaphore,
    pub socket_bootstrap_release: tokio::sync::Semaphore,
    pub socket_command_received: tokio::sync::Semaphore,
    pub socket_command_release: tokio::sync::Semaphore,
    pub socket_lag_received: tokio::sync::Semaphore,
    pub socket_lag_release: tokio::sync::Semaphore,
    pub socket_broadcast_pause_enabled: std::sync::atomic::AtomicBool,
    pub socket_broadcast_received: tokio::sync::Semaphore,
    pub socket_broadcast_release: tokio::sync::Semaphore,
    pub socket_rebootstrap_read: tokio::sync::Semaphore,
    pub socket_directory_pause_enabled: std::sync::atomic::AtomicBool,
    pub socket_directory_admitted: tokio::sync::Semaphore,
    pub socket_directory_release: tokio::sync::Semaphore,
    pub socket_admin_revoke_pause_enabled: std::sync::atomic::AtomicBool,
    pub socket_admin_revoke_admitted: tokio::sync::Semaphore,
    pub socket_admin_revoke_release: tokio::sync::Semaphore,
    pub socket_global_send_pause: Mutex<Option<(String, u8)>>,
    pub socket_global_send_admitted: tokio::sync::Semaphore,
    pub socket_session_revoke_attempted: tokio::sync::Semaphore,
    pub socket_global_send_release: tokio::sync::Semaphore,
    pub socket_scoped_send_mode: std::sync::atomic::AtomicU8,
    pub socket_scoped_send_admitted: tokio::sync::Semaphore,
    pub socket_scoped_send_release: tokio::sync::Semaphore,
    pub socket_membership_remove_enabled: std::sync::atomic::AtomicBool,
    pub socket_membership_remove_admitted: tokio::sync::Semaphore,
    pub socket_membership_remove_release: tokio::sync::Semaphore,
    pub directory_command_pause_user: Mutex<Option<(String, bool)>>,
    pub directory_command_attempted: tokio::sync::Semaphore,
    pub directory_command_admitted: tokio::sync::Semaphore,
    pub directory_command_release: tokio::sync::Semaphore,
    pub admin_effect_pause_enabled: std::sync::atomic::AtomicBool,
    pub admin_effect_admitted: tokio::sync::Semaphore,
    pub admin_effect_release: tokio::sync::Semaphore,
    pub directory_event_page_fence_enabled: std::sync::atomic::AtomicBool,
    pub directory_event_page_read_admitted: tokio::sync::Semaphore,
    pub directory_event_page_read_release: tokio::sync::Semaphore,
    pub directory_event_page_control: Mutex<Option<Arc<DirectoryEventPageHttpControl>>>,
    pub check_in_pause_enabled: std::sync::atomic::AtomicBool,
    pub check_in_admitted: tokio::sync::Semaphore,
    pub check_in_release: tokio::sync::Semaphore,
    pub socket_commit_pause_enabled: std::sync::atomic::AtomicBool,
    pub socket_commit_admitted: tokio::sync::Semaphore,
    pub socket_commit_release: tokio::sync::Semaphore,
    pub socket_frame_deadline: Mutex<Option<std::time::Duration>>,
}

#[cfg(any(test, feature = "testkit"))]
impl Default for TestLiveGate {
    fn default() -> Self {
        Self {
            document_subscribed: tokio::sync::Semaphore::new(0),
            document_release: tokio::sync::Semaphore::new(0),
            directory_subscribed: tokio::sync::Semaphore::new(0),
            directory_release: tokio::sync::Semaphore::new(0),
            socket_before_welcome: tokio::sync::Semaphore::new(0),
            socket_welcome_release: tokio::sync::Semaphore::new(0),
            socket_after_welcome: tokio::sync::Semaphore::new(0),
            socket_bootstrap_release: tokio::sync::Semaphore::new(0),
            socket_command_received: tokio::sync::Semaphore::new(0),
            socket_command_release: tokio::sync::Semaphore::new(0),
            socket_lag_received: tokio::sync::Semaphore::new(0),
            socket_lag_release: tokio::sync::Semaphore::new(0),
            socket_broadcast_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_broadcast_received: tokio::sync::Semaphore::new(0),
            socket_broadcast_release: tokio::sync::Semaphore::new(0),
            socket_rebootstrap_read: tokio::sync::Semaphore::new(0),
            socket_directory_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_directory_admitted: tokio::sync::Semaphore::new(0),
            socket_directory_release: tokio::sync::Semaphore::new(0),
            socket_admin_revoke_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_admin_revoke_admitted: tokio::sync::Semaphore::new(0),
            socket_admin_revoke_release: tokio::sync::Semaphore::new(0),
            socket_global_send_pause: Mutex::new(None),
            socket_global_send_admitted: tokio::sync::Semaphore::new(0),
            socket_session_revoke_attempted: tokio::sync::Semaphore::new(0),
            socket_global_send_release: tokio::sync::Semaphore::new(0),
            socket_scoped_send_mode: std::sync::atomic::AtomicU8::new(0),
            socket_scoped_send_admitted: tokio::sync::Semaphore::new(0),
            socket_scoped_send_release: tokio::sync::Semaphore::new(0),
            socket_membership_remove_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_membership_remove_admitted: tokio::sync::Semaphore::new(0),
            socket_membership_remove_release: tokio::sync::Semaphore::new(0),
            directory_command_pause_user: Mutex::new(None),
            directory_command_attempted: tokio::sync::Semaphore::new(0),
            directory_command_admitted: tokio::sync::Semaphore::new(0),
            directory_command_release: tokio::sync::Semaphore::new(0),
            admin_effect_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            admin_effect_admitted: tokio::sync::Semaphore::new(0),
            admin_effect_release: tokio::sync::Semaphore::new(0),
            directory_event_page_fence_enabled: std::sync::atomic::AtomicBool::new(false),
            directory_event_page_read_admitted: tokio::sync::Semaphore::new(0),
            directory_event_page_read_release: tokio::sync::Semaphore::new(0),
            directory_event_page_control: Mutex::new(None),
            check_in_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            check_in_admitted: tokio::sync::Semaphore::new(0),
            check_in_release: tokio::sync::Semaphore::new(0),
            socket_commit_pause_enabled: std::sync::atomic::AtomicBool::new(false),
            socket_commit_admitted: tokio::sync::Semaphore::new(0),
            socket_commit_release: tokio::sync::Semaphore::new(0),
            socket_frame_deadline: Mutex::new(None),
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub struct TestDocumentOpenPlanIssueGate {
    pub admitted: tokio::sync::Semaphore,
    pub release: tokio::sync::Semaphore,
}

#[cfg(any(test, feature = "testkit"))]
impl Default for TestDocumentOpenPlanIssueGate {
    fn default() -> Self {
        Self { admitted: tokio::sync::Semaphore::new(0), release: tokio::sync::Semaphore::new(0) }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub struct TestDocumentOpenCatalog {
    pub generation_id: String,
    pub open_targets: Box<[VerifiedDocumentOpenSelectionV1]>,
    pub component: std::sync::Arc<[u8]>,
    pub descriptor: std::sync::Arc<[u8]>,
    pub browser_actor: Option<std::sync::Arc<[u8]>>,
}

#[cfg(any(test, feature = "testkit"))]
impl DocumentOpenCatalogAuthorityV1 for TestDocumentOpenCatalog {
    fn generation_id(&self) -> &str {
        &self.generation_id
    }

    fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets> {
        if current_generation != self.generation_id || self.component.is_empty() || self.descriptor.is_empty() {
            return None;
        }
        let selection = DocumentOpenCatalogAuthorityV1::resolve_document_open(self, descriptor, requested_surface_id, writable)?;
        Some(VerifiedExecutionTargetAssets { selection, component: TrustedCatalogAsset::resident(std::sync::Arc::clone(&self.component)), descriptor: std::sync::Arc::clone(&self.descriptor), browser_actor: self.browser_actor.clone().map(TrustedCatalogAsset::resident) })
    }

    fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        let mut matches = self.open_targets.iter().filter(|target| {
            target.package.plugin_id == descriptor.owner.plugin_id
                && target.package.package_id == descriptor.owner.package_id
                && target.package.version == descriptor.owner.version
                && target.package.component_sha256 == descriptor.owner.package_hash
                && target.artifact.kind == descriptor.artifact_kind
                && target.artifact.schema == descriptor.artifact_schema
                && target.artifact.pack_schema_hash == descriptor.pack_schema_hash
                && target.grant.write == writable
                && requested_surface_id.is_none_or(|surface_id| target.surface.surface_id == surface_id)
        });
        let selected = matches.next()?.clone();
        matches.next().is_none().then_some(selected)
    }
}
//#endregion 🔖️State
