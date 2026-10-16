//! 🗃️ The table DDL and constraint text the backend applies when it opens a fresh store.

//#region 🔖️Schema
pub(super) const SCHEMA: &str = "\
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS hub_share_grant (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT,
    FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_directory_format (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema TEXT NOT NULL,
    version INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_space (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at INTEGER NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('atelier', 'studio', 'archive')),
    visibility TEXT NOT NULL CHECK (visibility IN ('private', 'public'))
);
CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    PRIMARY KEY (space_id, user_id)
);
CREATE TABLE IF NOT EXISTS hub_document_index (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id),
    FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_document_descriptor (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    document_id TEXT NOT NULL,
    artifact_kind TEXT NOT NULL,
    artifact_schema TEXT NOT NULL,
    owner_plugin_id TEXT NOT NULL,
    owner_package_id TEXT NOT NULL,
    owner_version TEXT NOT NULL,
    owner_package_hash TEXT NOT NULL,
    pack_schema_hash TEXT NOT NULL,
    bootstrap_version INTEGER NOT NULL,
    bootstrap_head_seq INTEGER NOT NULL,
    bootstrap_commit_seq INTEGER NOT NULL,
    bootstrap_epoch INTEGER NOT NULL,
    bootstrap_snapshot_hash TEXT NOT NULL,
    announced_at INTEGER NOT NULL,
    PRIMARY KEY (space_id, document_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_checkpoint (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK (length(checkpoint_id) = 32),
    parent_checkpoint_id BLOB CHECK (parent_checkpoint_id IS NULL OR length(parent_checkpoint_id) = 32),
    descriptor_digest BLOB NOT NULL CHECK (length(descriptor_digest) = 32),
    frontier_document_id TEXT NOT NULL,
    head_edit_ordinal INTEGER NOT NULL,
    head_edit_id TEXT NOT NULL,
    last_commit_seq INTEGER NOT NULL,
    chain_hash BLOB NOT NULL CHECK (length(chain_hash) = 32),
    pack_sha256 BLOB NOT NULL CHECK (length(pack_sha256) = 32),
    pack_byte_length INTEGER NOT NULL,
    spr_sha256 BLOB NOT NULL CHECK (length(spr_sha256) = 32),
    spr_byte_length INTEGER NOT NULL,
    aggregate_sha256 BLOB NOT NULL CHECK (length(aggregate_sha256) = 32),
    published_at INTEGER NOT NULL,
    event_seq INTEGER NOT NULL UNIQUE,
    active INTEGER NOT NULL CHECK (active IN (0, 1)),
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id, checkpoint_id),
    FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_artifact_checkpoint_active ON hub_artifact_checkpoint (space_id, document_id) WHERE active = 1;
CREATE INDEX IF NOT EXISTS idx_artifact_checkpoint_lineage ON hub_artifact_checkpoint (space_id, document_id, event_seq);
CREATE TABLE IF NOT EXISTS hub_artifact_retention (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    retained_checkpoint_id BLOB NOT NULL CHECK (length(retained_checkpoint_id) = 32),
    floor_document_id TEXT NOT NULL,
    floor_head_edit_ordinal INTEGER NOT NULL,
    floor_head_edit_id TEXT NOT NULL,
    floor_last_commit_seq INTEGER NOT NULL,
    floor_chain_hash BLOB NOT NULL CHECK (length(floor_chain_hash) = 32),
    checkpoint_lineage_head BLOB NOT NULL CHECK (length(checkpoint_lineage_head) = 32),
    event_seq INTEGER NOT NULL UNIQUE,
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id),
    FOREIGN KEY (space_id, document_id, retained_checkpoint_id) REFERENCES hub_artifact_checkpoint(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    identity_provider TEXT NOT NULL,
    identity_subject_digest BLOB NOT NULL CHECK (length(identity_subject_digest) = 32),
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT,
    authorization_generation INTEGER NOT NULL CHECK (authorization_generation >= 1),
    device_instance_id TEXT NOT NULL,
    session_kind TEXT NOT NULL CHECK (session_kind IN ('external', 'development-local', 'agent'))
);
CREATE TABLE IF NOT EXISTS hub_agent_delegation (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    delegating_user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    agent_label TEXT NOT NULL,
    audience TEXT NOT NULL CHECK (audience IN ('read', 'edit')),
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT
);
CREATE INDEX IF NOT EXISTS idx_agent_delegation_owner ON hub_agent_delegation (space_id, delegating_user_id, created_at DESC);
CREATE TABLE IF NOT EXISTS hub_artifact_creation_fact (
    actor_user_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision BETWEEN 1 AND 3),
    phase TEXT NOT NULL CHECK (phase IN ('accepted','prepared','committed','cancelled','failed')),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    deadline_ms INTEGER NOT NULL,
    recorded_at_ms INTEGER NOT NULL,
    payload TEXT NOT NULL,
    PRIMARY KEY (actor_user_id, request_id, revision)
);
CREATE UNIQUE INDEX IF NOT EXISTS hub_artifact_creation_document ON hub_artifact_creation_fact(space_id, document_id) WHERE revision = 1;
CREATE INDEX IF NOT EXISTS hub_artifact_creation_recovery ON hub_artifact_creation_fact(phase, deadline_ms, actor_user_id, request_id);
CREATE TABLE IF NOT EXISTS hub_sync_session (
    id TEXT PRIMARY KEY,
    auth_session_id TEXT REFERENCES hub_auth_session(id) ON DELETE SET NULL,
    authorization_generation INTEGER NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    surface TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    authenticated_email TEXT,
    space_role TEXT,
    client_label TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    disconnected_at INTEGER
);
CREATE TABLE IF NOT EXISTS hub_space_invite (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT,
    accepted_at INTEGER,
    accepted_event_id TEXT,
    CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))
);
CREATE TABLE IF NOT EXISTS hub_auth_audit (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    id TEXT NOT NULL UNIQUE,
    occurred_at INTEGER NOT NULL,
    event_kind TEXT NOT NULL,
    auth_session_id TEXT,
    target_user_id TEXT,
    actor_user_id TEXT,
    provider TEXT,
    outcome_code TEXT NOT NULL,
    reason_code TEXT,
    correlation_id TEXT NOT NULL,
    peer_class TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_admin_operation_audit (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    request_id TEXT NOT NULL,
    intent_digest TEXT NOT NULL CHECK (length(intent_digest) = 64),
    operation_id TEXT NOT NULL,
    occurred_at INTEGER NOT NULL,
    phase TEXT NOT NULL CHECK (phase IN ('accepted', 'succeeded', 'failed', 'cancelled')),
    terminal INTEGER NOT NULL CHECK (terminal IN (0, 1)),
    intent_kind TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_id TEXT NOT NULL,
    principal_user_id TEXT NOT NULL,
    principal_session_id TEXT NOT NULL,
    principal_generation INTEGER NOT NULL CHECK (principal_generation >= 1),
    correlation_id TEXT NOT NULL,
    event_seq_first INTEGER,
    event_seq_last INTEGER,
    outcome_code TEXT NOT NULL,
    reason_code TEXT,
    UNIQUE (request_id, terminal)
);
CREATE TABLE IF NOT EXISTS hub_admin_operation_effect_receipt (
    operation_id TEXT NOT NULL,
    intent_digest TEXT NOT NULL CHECK (length(intent_digest) = 64),
    committed_at INTEGER NOT NULL,
    outcome_code TEXT NOT NULL,
    event_seq_first INTEGER,
    event_seq_last INTEGER,
    PRIMARY KEY (operation_id, intent_digest),
    UNIQUE (operation_id),
    CHECK ((event_seq_first IS NULL) = (event_seq_last IS NULL))
);
CREATE TABLE IF NOT EXISTS hub_directory_command_receipt (
    actor_user_id TEXT NOT NULL,
    request_id TEXT NOT NULL CHECK (length(request_id) = 32),
    command_sha256 TEXT NOT NULL CHECK (length(command_sha256) = 64),
    result_kind TEXT NOT NULL CHECK (result_kind IN ('none', 'invite')),
    disposition TEXT NOT NULL CHECK (disposition IN ('pending', 'completed')),
    event_seq_first INTEGER,
    event_seq_last INTEGER,
    receipt_sha256 TEXT,
    claimed_at INTEGER NOT NULL,
    completed_at INTEGER,
    PRIMARY KEY (actor_user_id, request_id)
);
CREATE TABLE IF NOT EXISTS hub_checkpoint_publication_receipt (
    actor_user_id TEXT NOT NULL,
    correlation_id TEXT NOT NULL CHECK (length(correlation_id) = 32),
    command_sha256 TEXT NOT NULL CHECK (length(command_sha256) = 64),
    disposition TEXT NOT NULL CHECK (disposition IN ('pending', 'completed')),
    checkpoint_id BLOB CHECK (checkpoint_id IS NULL OR length(checkpoint_id) = 32),
    claimed_at INTEGER NOT NULL,
    completed_at INTEGER,
    PRIMARY KEY (actor_user_id, correlation_id)
);
CREATE TABLE IF NOT EXISTS hub_directory_event (
    seq INTEGER PRIMARY KEY AUTOINCREMENT CHECK (seq <= 9007199254740991),
    id TEXT NOT NULL UNIQUE,
    hlc_physical INTEGER NOT NULL,
    hlc_logical INTEGER NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT,
    user_id TEXT,
    kind TEXT NOT NULL,
    payload TEXT NOT NULL,
    recorded_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_artifact_authority_journal (
    event_seq INTEGER PRIMARY KEY REFERENCES hub_directory_event(seq),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK (length(checkpoint_id) = 32),
    payload TEXT NOT NULL,
    UNIQUE (space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_checkpoint_private (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK (length(checkpoint_id) = 32),
    event_seq INTEGER NOT NULL UNIQUE REFERENCES hub_artifact_authority_journal(event_seq),
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id, checkpoint_id),
    FOREIGN KEY (space_id, document_id, checkpoint_id) REFERENCES hub_artifact_checkpoint(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_ledger_head (
    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
    generation INTEGER NOT NULL CHECK(generation >= 0)
);
INSERT OR IGNORE INTO hub_artifact_cas_ledger_head(singleton, generation) VALUES (1, 0);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_barrier_identity (
    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
    coordinator_id BLOB NOT NULL CHECK(length(coordinator_id) = 32)
);
INSERT OR IGNORE INTO hub_artifact_cas_barrier_identity(singleton, coordinator_id) VALUES (1, randomblob(32));
CREATE TABLE IF NOT EXISTS hub_artifact_cas_ledger_journal (
    generation INTEGER PRIMARY KEY CHECK(generation >= 1),
    operation TEXT NOT NULL CHECK(operation IN ('reserve', 'publish', 'retention', 'space-delete')),
    space_id TEXT NOT NULL,
    document_id TEXT,
    checkpoint_id BLOB CHECK(checkpoint_id IS NULL OR length(checkpoint_id) = 32),
    write_epoch INTEGER,
    expires_at_ms INTEGER,
    event_seq INTEGER,
    plan BLOB,
    CHECK((operation IN ('reserve', 'publish') AND document_id IS NOT NULL AND checkpoint_id IS NOT NULL AND write_epoch >= 1 AND expires_at_ms IS NOT NULL AND plan IS NOT NULL) OR operation IN ('retention', 'space-delete'))
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reservation (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    generation INTEGER NOT NULL UNIQUE,
    write_epoch INTEGER NOT NULL CHECK(write_epoch >= 1),
    expires_at_ms INTEGER NOT NULL,
    plan BLOB NOT NULL,
    PRIMARY KEY(space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reservation_object (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
    object_digest BLOB NOT NULL CHECK(length(object_digest) = 32),
    PRIMARY KEY(space_id, document_id, checkpoint_id, kind, object_digest),
    FOREIGN KEY(space_id, document_id, checkpoint_id) REFERENCES hub_artifact_cas_reservation(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reference (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    generation INTEGER NOT NULL UNIQUE,
    write_epoch INTEGER NOT NULL CHECK(write_epoch >= 1),
    plan BLOB NOT NULL,
    PRIMARY KEY(space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reference_object (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
    object_digest BLOB NOT NULL CHECK(length(object_digest) = 32),
    PRIMARY KEY(space_id, document_id, checkpoint_id, kind, object_digest),
    FOREIGN KEY(space_id, document_id, checkpoint_id) REFERENCES hub_artifact_cas_reference(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_delete_lease (
    space_id TEXT PRIMARY KEY,
    fence_epoch INTEGER NOT NULL CHECK(fence_epoch >= 1),
    lease_token BLOB CHECK(lease_token IS NULL OR length(lease_token) = 32),
    expires_at_ms INTEGER,
    CHECK((lease_token IS NULL AND expires_at_ms IS NULL) OR (lease_token IS NOT NULL AND expires_at_ms >= 1))
);
CREATE INDEX IF NOT EXISTS idx_artifact_cas_journal_scope ON hub_artifact_cas_ledger_journal(space_id, document_id, checkpoint_id, generation);
CREATE INDEX IF NOT EXISTS idx_artifact_cas_reservation_object_lookup ON hub_artifact_cas_reservation_object(space_id, kind, object_digest);
CREATE INDEX IF NOT EXISTS idx_artifact_cas_reference_object_lookup ON hub_artifact_cas_reference_object(space_id, kind, object_digest);
CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_space_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_directory_event_space ON hub_directory_event (space_id, recorded_at);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
CREATE INDEX IF NOT EXISTS idx_sync_session_space ON hub_sync_session (space_id, disconnected_at);
CREATE INDEX IF NOT EXISTS idx_space_invite_space ON hub_space_invite (space_id);
CREATE INDEX IF NOT EXISTS idx_share_grant_scope ON hub_share_grant (space_id, document_id);
CREATE INDEX IF NOT EXISTS idx_auth_session_user_active ON hub_auth_session (user_id, revoked_at);
CREATE INDEX IF NOT EXISTS idx_auth_session_identity_active ON hub_auth_session (identity_provider, identity_subject_digest, revoked_at);
CREATE INDEX IF NOT EXISTS idx_auth_audit_occurred ON hub_auth_audit (occurred_at, sequence);
CREATE INDEX IF NOT EXISTS idx_admin_operation_audit_sequence ON hub_admin_operation_audit (sequence);
CREATE INDEX IF NOT EXISTS idx_admin_operation_audit_request ON hub_admin_operation_audit (request_id, sequence);
CREATE INDEX IF NOT EXISTS idx_admin_operation_audit_operation ON hub_admin_operation_audit (operation_id, sequence);
CREATE INDEX IF NOT EXISTS idx_directory_command_receipt_actor ON hub_directory_command_receipt (actor_user_id, request_id);
";
//#endregion 🔖️Schema
