//! 🛡️ Capability traits for administrator operations and idempotency receipts.

use super::*;

//#region AdminOperationDirectory
/// 🛡️ Administrator operations: the overview counts, the operation audit and the private effect receipts.
pub trait AdminOperationDirectory: Send + Sync + 'static {
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts>;
    /// 🧾 Appends exactly one accepted or terminal administrator operation fact.
    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord>;
    /// 🔎 Reads the at-most-two facts for one bounded idempotency request key.
    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    /// 🎯 Reads the at-most-two facts for one server-issued operation identifier.
    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    /// 🧷️ Reads an exact private writer receipt without exposing capability material.
    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>>;
    /// 📄 Reads one backend-ordered bounded operation-audit page.
    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
}
//#endregion AdminOperationDirectory

//#region ReceiptDirectory
/// 🧾️ Idempotency receipts: directory command receipts and checkpoint publication claims.
pub trait ReceiptDirectory: Send + Sync + 'static {
    /// 🔐️ Atomically claims one `(authenticated user id, request id)` idempotency key or reads the
    /// row already under it. An equal key with an unequal command digest is `Conflict` and never
    /// executes. This is the single serialization point the command writer holds across its whole
    /// decide/append sequence, so a duplicate request can never mint a second invitation.
    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1>;
    /// 🧾️ Records the durable completion of one claimed key. The caller publishes only after this
    /// returns, so a delivered event is always covered by a durable receipt.
    async fn complete_directory_command_receipt(&self, completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord>;
    /// 🧹️ Releases one claimed key whose command failed before any durable event was appended.
    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()>;
    /// 🆔️ Atomically claims or reads one author/correlation/exact-command checkpoint publication.
    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1>;
    /// 🧹️ Releases an equal still-pending claim; a substituted digest never removes its owner.
    async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()>;
}
//#endregion ReceiptDirectory
