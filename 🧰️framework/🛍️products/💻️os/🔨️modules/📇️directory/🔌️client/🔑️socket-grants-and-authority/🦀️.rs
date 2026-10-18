//! 🔑️ Socket grants and authority of the directory client: the socket grant receipts with their wiping drops, the document socket authority, admission and expectation and the hub socket grant source. See [`super`].

use super::*;

//#region 🔖️Wire
#[derive(semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", retire_with = "std::mem::drop")]
pub struct SocketGrantReceiptV1 {
    pub schema: String,
    pub protocol: String,
    pub grant: String,
    pub actor_id: String,
    pub expires_at_ms: i64,
}

impl Drop for SocketGrantReceiptV1 {
    fn drop(&mut self) {
        wipe_string(&mut self.grant);
    }
}

/// 📝️ The answer to a document open-plan exchange (`os.directory#/$defs/DocumentSocketGrantReceiptV1`):
/// the actor the next `semio.session.v1` upgrade of the same credential is admitted as, and until
/// when. It carries no secret — the upgrade presents the credential itself.
#[derive(semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentSocketGrantReceiptV1 {
    pub schema: String,
    pub protocol: String,
    pub actor_id: String,
    pub expires_at_ms: i64,
}

/// 🔐 Receipt-free server-selected authority retained by a native document connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSocketAuthorityV1 {
    #[doc(hidden)] pub admitted_lease: Option<DocumentExecutionTargetLeaseFieldsV1>,
    pub hub_origin: String,
    pub expires_at_unix_ms: u64,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub pack_schema_hash: [u8; 32],
    pub surface: DocumentOpenSurfaceV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: DocumentOpenCheckpointV1,
    pub revalidation: DocumentOpenRevalidationV1,
}

impl DocumentSocketAuthorityV1 {
    /// 🏗️ Builds receipt-free authority only from a valid plan and its exact local codec/lease expectation.
    pub fn from_plan(hub_origin: String, plan: &DocumentOpenPlanV1, expectation: &DocumentSocketExpectationV1, now_ms: u64) -> Result<Self, DocumentOpenPlanErrorCodeV1> {
        plan.validate(now_ms)?;
        let pack_schema_hash = decode_lower_hex_32(&plan.artifact.pack_schema_hash).ok_or(DocumentOpenPlanErrorCodeV1::Denied)?;
        if expectation.artifact_schema != plan.artifact.schema
            || pack_schema_hash != expectation.pack_schema_hash
            || expectation.requested_surface_id.as_ref().is_some_and(|surface| surface != &plan.surface.surface_id)
            || expectation.lease.as_ref().is_some_and(|lease| !lease_fields_from_plan_v1(plan, lease.component.byte_length, lease.descriptor.byte_length, lease.browser_actor.byte_length()).is_ok_and(|fields| same_lease_fields_v1(&fields, lease)))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(Self {
            admitted_lease: expectation.lease.clone(),
            hub_origin,
            expires_at_unix_ms: plan.expires_at_unix_ms,
            scope: plan.scope.clone(),
            descriptor_digest_v1: plan.descriptor_digest_v1.clone(),
            catalog: plan.catalog.clone(),
            package: plan.package.clone(),
            artifact: plan.artifact.clone(),
            parent_dialect: plan.parent_dialect.clone(),
            pack_schema_hash,
            surface: plan.surface.clone(),
            browser_actor: plan.browser_actor.clone(),
            grant: plan.grant,
            checkpoint: plan.checkpoint.clone(),
            revalidation: plan.revalidation,
        })
    }

    /// 🧾 Receipt-free plan projection at independently retained byte lengths.
    pub fn lease_fields(&self, component_byte_length: u64, descriptor_byte_length: u64, browser_actor_byte_length: Option<u64>) -> Result<DocumentExecutionTargetLeaseFieldsV1, DocumentOpenPlanErrorCodeV1> {
        let fields = DocumentExecutionTargetLeaseFieldsV1 {
            schema: "semio.os.document-execution-target-lease/v1".to_string(),
            version: 1,
            scope: self.scope.clone(),
            descriptor_digest_v1: self.descriptor_digest_v1.clone(),
            catalog: self.catalog.clone(),
            package: self.package.clone(),
            component: DocumentExecutionTargetComponentV1 { sha256: self.package.component_sha256.clone(), blake3: self.package.component_blake3.clone(), byte_length: component_byte_length },
            descriptor: DocumentExecutionTargetDescriptorV1 { sha256: self.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
            browser_actor: self
                .browser_actor
                .to_lease(DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str(), browser_actor_byte_length)
                .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?,
            artifact: self.artifact.clone(),
            parent_dialect: self.parent_dialect.clone(),
            surface: self.surface.clone(),
            grant: self.grant,
            checkpoint: self.checkpoint.clone(),
            revalidation: self.revalidation,
        };
        fields.validate()?;
        Ok(fields)
    }

    /// ⚖️ The one shared full-field lease relation, replacing every partial surface predicate: it
    /// compares both scope ids, the descriptor digest, catalog generation, all three package
    /// digests, both byte lengths, artifact, parent dialect, every surface field, every grant bit,
    /// the checkpoint and every revalidation generation.
    pub fn matches_lease_fields(&self, expected: &DocumentExecutionTargetLeaseFieldsV1) -> bool {
        self.admitted_lease.as_ref().is_some_and(|admitted| {
            same_lease_fields_v1(admitted, expected) && self.lease_fields(admitted.component.byte_length, admitted.descriptor.byte_length, admitted.browser_actor.byte_length()).is_ok_and(|fields| same_lease_fields_v1(&fields, expected))
        })
    }
}

/// 🎫 One exchanged document socket grant and its receipt-free open authority.
pub struct DocumentSocketAdmissionV1 {
    pub socket: DocumentSocketGrantReceiptV1,
    pub authority: DocumentSocketAuthorityV1,
}

/// 🧬 Local codec expectation plus the complete verified execution-target lease a server-authoritative
/// open plan must satisfy. There is no partial surface expectation: a locally installed target is
/// compared through the one shared full-field relation or not at all.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentSocketExpectationV1 {
    pub artifact_schema: String,
    pub pack_schema_hash: [u8; 32],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub requested_surface_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lease: Option<DocumentExecutionTargetLeaseFieldsV1>,
}

/// 🎫 The one document-socket admission seam both document actors dial through: the open-plan
/// exchange (`POST …/open-plan`) followed by the plan-bound socket grant (`POST …/socket-grants`).
/// The semantics are one body ([`DirectoryClient`]'s `document_admission_*` legs); only the I/O shape is
/// per target: the native actor runs the blocking admission as one `Lane::Io` job, while a browser
/// isolate may not block, so there the same admission is a future on the isolate's own executor.
pub trait HubSocketGrantSource: Send + Sync {
    #[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
    fn admit_document_socket(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str, timeout_ms: u64) -> Result<DocumentSocketAdmissionV1, DirectoryClientError>;
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    fn admit_document_socket<'a>(&'a self, ctx: &'a OperationContext, space_id: &'a str, document_id: &'a str, expectation: &'a DocumentSocketExpectationV1, client_instance_id: &'a str, timeout_ms: u64) -> DocumentSocketAdmissionFuture<'a>;
}

/// ⏳ The browser admission's shape: a future polled by the isolate's executor, never blocked on.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub type DocumentSocketAdmissionFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<DocumentSocketAdmissionV1, DirectoryClientError>> + 'a>>;
//#endregion 🔖️Wire
