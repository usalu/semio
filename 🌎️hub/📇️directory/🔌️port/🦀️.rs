//! 🔌️ The capability traits every persistence backend implements. The port is partitioned along the sections the backends share, so a caller bounds itself on exactly the capabilities it uses, for example `D: SpaceDirectory + EventLog`.

use super::*;
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::model::{
    AdminDirectoryOverviewCounts, AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AdminSpaceSummaryRecord, AgentDelegationRecord, AgentDelegationRow, AuthAuditRecord, AuthSessionIssue, AuthSessionRecord,
    CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CredentialAuditFactV1, DirectoryCommandClaimV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, DirectoryPrincipalCeilingV1, DirectoryPrincipalV1, InviteRecord,
    InviteRedemptionCommit, IssuedAgentDelegation, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, RevokedAuthSession,
    ShareTokenRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceAdministrationInviteRow, SpaceAdministrationMemberRow, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord,
};
use semio_hub_directory_contract::{now_ms, InviteCapability, InviteRedemptionScopeHintV1, NewDirectoryEvent, SessionCapability, ShareCapability};

#[path = "🎟️capabilities/🦀️.rs"]
mod capabilities;
pub use capabilities::{InviteDirectory, ShareTokenDirectory};

#[path = "👤️accounts/🦀️.rs"]
mod accounts;
pub use accounts::{AgentDelegationDirectory, AuthSessionDirectory, CredentialDirectory, UserDirectory};

#[path = "🏘️spaces/🦀️.rs"]
mod spaces;
pub use spaces::{DocumentDirectory, PrincipalAuthority, SpaceDirectory};

#[path = "🏭️artifact-creation/🦀️.rs"]
mod artifact_creation;
pub use artifact_creation::ArtifactCreationDirectory;

#[path = "🧺️artifact-cas/🦀️.rs"]
mod artifact_cas;
pub use artifact_cas::ArtifactCasDirectory;

#[path = "🛡️administration/🦀️.rs"]
mod administration;
pub use administration::{AdminOperationDirectory, ReceiptDirectory};

#[path = "🔄️sync-sessions/🦀️.rs"]
mod sync_sessions;
pub use sync_sessions::SyncSessionDirectory;

#[path = "📜️event-log/🦀️.rs"]
mod event_log;
pub use event_log::EventLog;

#[path = "📽️projections/🦀️.rs"]
mod projections;
pub use projections::ProjectionRebuild;
