-- #region 🧲Header
-- [🧰repo📚postgres📐schema](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql)
-- 2025 Ueli Saluz <ueli@semio-tech.com>
-- AGPL-3.0
-- Canonical PostgreSQL schema for repo persistence. All durable state lives here.

-- Specs:
-- - Use timestamptz everywhere.
-- - Use text for IDs (ticket IDs are path-based like "2026/04/01/slug").
-- - Use jsonb for flexible payload storage.
-- - Mirror current SQLite schema semantics but use PostgreSQL features.
-- - Include pg-boss schema via its own init.
-- #endregion 🧲Header

-- #region 🖇️Extensions
-- [🧰repo📚postgres📐schema🔖extensions](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Extensions)
-- Required PostgreSQL extensions.
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
-- #endregion 🖇️Extensions

-- #region 👓Developers
-- [🧰repo📚postgres📐schema🔖developers](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Developers)
-- Identity and auth tables for trusted developer access control.

CREATE TABLE IF NOT EXISTS developers (
    id              TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    email           TEXT UNIQUE NOT NULL,
    github_login    TEXT UNIQUE,
    display_name    TEXT NOT NULL DEFAULT '',
    trusted         BOOLEAN NOT NULL DEFAULT false,
    active          BOOLEAN NOT NULL DEFAULT true,
    role            TEXT NOT NULL DEFAULT 'developer' CHECK (role IN ('developer', 'admin', 'owner')),
    discord_user_id TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at      TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS developer_api_keys (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    developer_id TEXT NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
    key_hash     TEXT NOT NULL,
    label        TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_used_at TIMESTAMPTZ,
    revoked_at   TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_developer_api_keys_hash ON developer_api_keys(key_hash) WHERE revoked_at IS NULL;

CREATE TABLE IF NOT EXISTS audit_log (
    id         TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    actor_id   TEXT REFERENCES developers(id),
    action     TEXT NOT NULL,
    target     TEXT,
    detail     JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_audit_log_created ON audit_log(created_at DESC);
-- #endregion 👓Developers

-- #region 📐Repos
-- [🧰repo📚postgres📐schema🔖repos](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Repos)
-- Repository registration table.

CREATE TABLE IF NOT EXISTS repos (
    id         TEXT PRIMARY KEY,
    name       TEXT NOT NULL DEFAULT '',
    path       TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- #endregion 📐Repos

-- #region 🧬Kits
-- [🧰repo📚postgres📐schema🔖kits](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Kits)
-- Durable kit persistence for version control, sessions, and normalized snapshots.

CREATE TABLE IF NOT EXISTS kits (
    id                              TEXT PRIMARY KEY,
    repo_id                         TEXT REFERENCES repos(id) ON DELETE SET NULL,
    name                            TEXT NOT NULL DEFAULT '',
    description                     TEXT NOT NULL DEFAULT '',
    icon                            TEXT NOT NULL DEFAULT '',
    image                           TEXT NOT NULL DEFAULT '',
    preview                         TEXT NOT NULL DEFAULT '',
    version                         TEXT NOT NULL DEFAULT '',
    remote                          TEXT NOT NULL DEFAULT '',
    homepage                        TEXT NOT NULL DEFAULT '',
    license                         TEXT NOT NULL DEFAULT '',
    uri                             TEXT NOT NULL DEFAULT '',
    initial_snapshot_id             TEXT NOT NULL DEFAULT '',
    current_checkpoint_id           TEXT NOT NULL DEFAULT '',
    current_materialized_snapshot_id TEXT NOT NULL DEFAULT '',
    created_at                      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at                      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_kits_repo ON kits(repo_id, updated_at DESC);

CREATE TABLE IF NOT EXISTS kit_snapshots (
    id                   TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    kit_id               TEXT NOT NULL REFERENCES kits(id) ON DELETE CASCADE,
    snapshot_kind        TEXT NOT NULL CHECK (snapshot_kind IN ('initial', 'materialized', 'draft-base', 'session-cache')),
    source_checkpoint_id TEXT NOT NULL DEFAULT '',
    name                 TEXT NOT NULL DEFAULT '',
    description          TEXT NOT NULL DEFAULT '',
    icon                 TEXT NOT NULL DEFAULT '',
    image                TEXT NOT NULL DEFAULT '',
    preview              TEXT NOT NULL DEFAULT '',
    version              TEXT NOT NULL DEFAULT '',
    remote               TEXT NOT NULL DEFAULT '',
    homepage             TEXT NOT NULL DEFAULT '',
    license              TEXT NOT NULL DEFAULT '',
    uri                  TEXT NOT NULL DEFAULT '',
    source_json          JSONB NOT NULL DEFAULT '{}',
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (kit_id, id)
);

CREATE INDEX IF NOT EXISTS idx_kit_snapshots_kit_kind ON kit_snapshots(kit_id, snapshot_kind, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_kit_snapshots_checkpoint ON kit_snapshots(source_checkpoint_id) WHERE source_checkpoint_id <> '';

CREATE TABLE IF NOT EXISTS kit_checkpoints (
    id                        TEXT PRIMARY KEY,
    kit_id                    TEXT NOT NULL REFERENCES kits(id) ON DELETE CASCADE,
    parent_checkpoint_id      TEXT REFERENCES kit_checkpoints(id) ON DELETE SET NULL,
    ordinal_on_main_backbone  BIGINT NOT NULL DEFAULT 0,
    message                   TEXT NOT NULL DEFAULT '',
    hash                      TEXT NOT NULL DEFAULT '',
    checkpointed_at           TIMESTAMPTZ,
    release_marked_at         TIMESTAMPTZ,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (kit_id, id)
);

CREATE INDEX IF NOT EXISTS idx_kit_checkpoints_parent ON kit_checkpoints(kit_id, parent_checkpoint_id, created_at);
CREATE INDEX IF NOT EXISTS idx_kit_checkpoints_hash ON kit_checkpoints(kit_id, hash);
CREATE INDEX IF NOT EXISTS idx_kit_checkpoints_main_backbone ON kit_checkpoints(kit_id, ordinal_on_main_backbone DESC);

CREATE TABLE IF NOT EXISTS kit_checkpoint_authors (
    checkpoint_id TEXT NOT NULL REFERENCES kit_checkpoints(id) ON DELETE CASCADE,
    author_id     TEXT NOT NULL,
    ordinal       INT NOT NULL DEFAULT 0,
    PRIMARY KEY (checkpoint_id, author_id)
);

CREATE INDEX IF NOT EXISTS idx_kit_checkpoint_authors_ordinal ON kit_checkpoint_authors(checkpoint_id, ordinal);

CREATE TABLE IF NOT EXISTS kit_checkpoint_changes (
    id            TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    checkpoint_id TEXT NOT NULL REFERENCES kit_checkpoints(id) ON DELETE CASCADE,
    ordinal       INT NOT NULL DEFAULT 0,
    change_kind   TEXT NOT NULL DEFAULT 'inferred',
    author        TEXT NOT NULL DEFAULT '',
    changed_at    TIMESTAMPTZ,
    forward_json  JSONB NOT NULL DEFAULT '[]',
    inverse_json  JSONB NOT NULL DEFAULT '[]',
    UNIQUE (checkpoint_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_alternatives (
    id                 TEXT PRIMARY KEY,
    kit_id             TEXT NOT NULL REFERENCES kits(id) ON DELETE CASCADE,
    name               TEXT NOT NULL,
    root_checkpoint_id TEXT NOT NULL REFERENCES kit_checkpoints(id) ON DELETE RESTRICT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (kit_id, name)
);

CREATE TABLE IF NOT EXISTS kit_alternative_checkpoints (
    alternative_id TEXT NOT NULL REFERENCES kit_alternatives(id) ON DELETE CASCADE,
    checkpoint_id  TEXT NOT NULL REFERENCES kit_checkpoints(id) ON DELETE CASCADE,
    ordinal        INT NOT NULL,
    PRIMARY KEY (alternative_id, checkpoint_id),
    UNIQUE (alternative_id, ordinal)
);

CREATE INDEX IF NOT EXISTS idx_kit_alternative_checkpoints_tip ON kit_alternative_checkpoints(alternative_id, ordinal DESC);

CREATE TABLE IF NOT EXISTS kit_sessions (
    id             TEXT PRIMARY KEY,
    kit_id         TEXT NOT NULL REFERENCES kits(id) ON DELETE CASCADE,
    client_id      TEXT NOT NULL DEFAULT '',
    person_id      TEXT NOT NULL DEFAULT '',
    read_only      BOOLEAN NOT NULL DEFAULT false,
    opened_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at      TIMESTAMPTZ,
    last_seen_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_kit_sessions_open ON kit_sessions(kit_id, opened_at DESC) WHERE closed_at IS NULL;

CREATE TABLE IF NOT EXISTS kit_drafts (
    id                    TEXT PRIMARY KEY,
    session_id            TEXT NOT NULL REFERENCES kit_sessions(id) ON DELETE CASCADE,
    base_checkpoint_id    TEXT REFERENCES kit_checkpoints(id) ON DELETE SET NULL,
    target_alternative_id TEXT REFERENCES kit_alternatives(id) ON DELETE SET NULL,
    before_snapshot_id    TEXT REFERENCES kit_snapshots(id) ON DELETE SET NULL,
    before_snapshot_json  JSONB NOT NULL DEFAULT '{}',
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at             TIMESTAMPTZ,
    CHECK (NOT (target_alternative_id IS NOT NULL AND base_checkpoint_id IS NULL))
);

CREATE INDEX IF NOT EXISTS idx_kit_drafts_session ON kit_drafts(session_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_kit_drafts_open ON kit_drafts(session_id, created_at DESC) WHERE closed_at IS NULL;

CREATE TABLE IF NOT EXISTS kit_transactions (
    id          TEXT PRIMARY KEY,
    draft_id    TEXT NOT NULL REFERENCES kit_drafts(id) ON DELETE CASCADE,
    state       TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'finalized', 'aborted')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at   TIMESTAMPTZ,
    UNIQUE (draft_id, id)
);

CREATE INDEX IF NOT EXISTS idx_kit_transactions_draft ON kit_transactions(draft_id, created_at DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_kit_transactions_one_open_per_draft ON kit_transactions(draft_id) WHERE state = 'open';

CREATE TABLE IF NOT EXISTS kit_transaction_changes (
    id               TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    transaction_id   TEXT NOT NULL REFERENCES kit_transactions(id) ON DELETE CASCADE,
    ordinal          INT NOT NULL DEFAULT 0,
    change_kind      TEXT NOT NULL DEFAULT 'inferred',
    forward_json     JSONB NOT NULL DEFAULT '[]',
    inverse_json     JSONB NOT NULL DEFAULT '[]',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (transaction_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_releases (
    checkpoint_id             TEXT PRIMARY KEY REFERENCES kit_checkpoints(id) ON DELETE CASCADE,
    initial_snapshot_id       TEXT REFERENCES kit_snapshots(id) ON DELETE SET NULL,
    materialized_snapshot_id  TEXT REFERENCES kit_snapshots(id) ON DELETE SET NULL,
    change_list_json          JSONB NOT NULL DEFAULT '[]',
    released_at               TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_kit_releases_materialized ON kit_releases(materialized_snapshot_id) WHERE materialized_snapshot_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS kit_snapshot_people (
    snapshot_id    TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id    TEXT NOT NULL,
    ordinal        INT NOT NULL DEFAULT 0,
    name           TEXT NOT NULL DEFAULT '',
    email          TEXT NOT NULL DEFAULT '',
    role           TEXT NOT NULL DEFAULT '',
    rank_value     BIGINT,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_concepts (
    snapshot_id    TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id    TEXT NOT NULL,
    ordinal        INT NOT NULL DEFAULT 0,
    name           TEXT NOT NULL DEFAULT '',
    description    TEXT NOT NULL DEFAULT '',
    order_value    BIGINT,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_tags (
    snapshot_id    TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id    TEXT NOT NULL,
    ordinal        INT NOT NULL DEFAULT 0,
    name           TEXT NOT NULL DEFAULT '',
    order_value    BIGINT,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_families (
    snapshot_id    TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id    TEXT NOT NULL,
    ordinal        INT NOT NULL DEFAULT 0,
    name           TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_typologies (
    snapshot_id           TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id           TEXT NOT NULL,
    ordinal               INT NOT NULL DEFAULT 0,
    name                  TEXT NOT NULL DEFAULT '',
    description           TEXT NOT NULL DEFAULT '',
    icon                  TEXT NOT NULL DEFAULT '',
    folder_external_id    TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_family_endpoints (
    snapshot_id           TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    family_external_id    TEXT NOT NULL,
    external_id           TEXT NOT NULL,
    ordinal               INT NOT NULL DEFAULT 0,
    name                  TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, family_external_id, ordinal),
    FOREIGN KEY (snapshot_id, family_external_id) REFERENCES kit_snapshot_families(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_family_endpoint_compatibility (
    snapshot_id                    TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    family_endpoint_external_id    TEXT NOT NULL,
    compatible_endpoint_external_id TEXT NOT NULL,
    ordinal                        INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, family_endpoint_external_id, compatible_endpoint_external_id),
    FOREIGN KEY (snapshot_id, family_endpoint_external_id) REFERENCES kit_snapshot_family_endpoints(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, compatible_endpoint_external_id) REFERENCES kit_snapshot_family_endpoints(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_qualities (
    snapshot_id      TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    owner_kind       TEXT NOT NULL CHECK (owner_kind IN ('kit', 'kind', 'kind-endpoint', 'kind-connector', 'kind-representation', 'layout')),
    owner_external_id TEXT NOT NULL DEFAULT '',
    external_id      TEXT NOT NULL,
    ordinal          INT NOT NULL DEFAULT 0,
    key_name         TEXT NOT NULL DEFAULT '',
    value_text       TEXT,
    unit_name        TEXT,
    definition_uri   TEXT,
    description      TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, owner_kind, owner_external_id, external_id),
    UNIQUE (snapshot_id, owner_kind, owner_external_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_quality_benchmarks (
    snapshot_id       TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    quality_external_id TEXT NOT NULL,
    ordinal           INT NOT NULL DEFAULT 0,
    external_id       TEXT NOT NULL,
    name              TEXT NOT NULL DEFAULT '',
    min_value         DOUBLE PRECISION,
    max_value         DOUBLE PRECISION,
    min_excluded      BOOLEAN NOT NULL DEFAULT false,
    max_excluded      BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (snapshot_id, quality_external_id, external_id),
    UNIQUE (snapshot_id, quality_external_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_files (
    snapshot_id               TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id               TEXT NOT NULL,
    ordinal                   INT NOT NULL DEFAULT 0,
    parent_folder_external_id TEXT NOT NULL DEFAULT '',
    url                       TEXT NOT NULL DEFAULT '',
    mime_kind                 TEXT NOT NULL DEFAULT '',
    size_value                BIGINT,
    hash_value                TEXT NOT NULL DEFAULT '',
    description               TEXT NOT NULL DEFAULT '',
    created_value             TIMESTAMPTZ,
    updated_value             TIMESTAMPTZ,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_folders (
    snapshot_id    TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    external_id    TEXT NOT NULL,
    ordinal        INT NOT NULL DEFAULT 0,
    path           TEXT NOT NULL DEFAULT '',
    description    TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_entities (
    snapshot_id             TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    typology_external_id    TEXT NOT NULL,
    external_id             TEXT NOT NULL,
    ordinal                 INT NOT NULL DEFAULT 0,
    folder_external_id      TEXT NOT NULL DEFAULT '',
    location_external_id    TEXT NOT NULL DEFAULT '',
    name                    TEXT NOT NULL DEFAULT '',
    description             TEXT NOT NULL DEFAULT '',
    icon                    TEXT NOT NULL DEFAULT '',
    image                   TEXT NOT NULL DEFAULT '',
    variant                 TEXT NOT NULL DEFAULT '',
    stock_value             BIGINT,
    is_abstract             BOOLEAN,
    is_virtual              BOOLEAN,
    unit_name               TEXT NOT NULL DEFAULT '',
    created_value           TIMESTAMPTZ,
    updated_value           TIMESTAMPTZ,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, typology_external_id, ordinal),
    FOREIGN KEY (snapshot_id, typology_external_id) REFERENCES kit_snapshot_typologies(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_family_refs (
    snapshot_id        TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id   TEXT NOT NULL,
    family_external_id TEXT NOT NULL,
    ordinal            INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, kind_external_id, family_external_id),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, family_external_id) REFERENCES kit_snapshot_families(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_author_refs (
    snapshot_id       TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id  TEXT NOT NULL,
    person_external_id TEXT NOT NULL,
    ordinal           INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, kind_external_id, person_external_id),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, person_external_id) REFERENCES kit_snapshot_people(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_concept_refs (
    snapshot_id         TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id    TEXT NOT NULL,
    concept_external_id TEXT NOT NULL,
    ordinal             INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, kind_external_id, concept_external_id),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, concept_external_id) REFERENCES kit_snapshot_concepts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_tag_refs (
    snapshot_id       TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id  TEXT NOT NULL,
    tag_external_id   TEXT NOT NULL,
    ordinal           INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, kind_external_id, tag_external_id),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, tag_external_id) REFERENCES kit_snapshot_tags(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_endpoints (
    snapshot_id             TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id        TEXT NOT NULL,
    external_id             TEXT NOT NULL,
    ordinal                 INT NOT NULL DEFAULT 0,
    family_name             TEXT NOT NULL DEFAULT '',
    mandatory               BOOLEAN,
    t_value                 DOUBLE PRECISION,
    description             TEXT NOT NULL DEFAULT '',
    point_json              JSONB NOT NULL DEFAULT '{}',
    direction_json          JSONB NOT NULL DEFAULT '{}',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, kind_external_id, ordinal),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_endpoint_compatibility (
    snapshot_id                     TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    endpoint_external_id            TEXT NOT NULL,
    compatible_family_name          TEXT NOT NULL,
    ordinal                         INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, endpoint_external_id, compatible_family_name),
    FOREIGN KEY (snapshot_id, endpoint_external_id) REFERENCES kit_snapshot_kind_endpoints(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_connectors (
    snapshot_id             TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id        TEXT NOT NULL,
    external_id             TEXT NOT NULL,
    ordinal                 INT NOT NULL DEFAULT 0,
    endpoint_external_id    TEXT NOT NULL DEFAULT '',
    code                    TEXT NOT NULL DEFAULT '',
    description             TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, kind_external_id, ordinal),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_kind_representations (
    snapshot_id             TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    kind_external_id        TEXT NOT NULL,
    external_id             TEXT NOT NULL,
    ordinal                 INT NOT NULL DEFAULT 0,
    file_external_id        TEXT NOT NULL DEFAULT '',
    url                     TEXT NOT NULL DEFAULT '',
    description             TEXT NOT NULL DEFAULT '',
    payload_json            JSONB NOT NULL DEFAULT '{}',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, kind_external_id, ordinal),
    FOREIGN KEY (snapshot_id, kind_external_id) REFERENCES kit_snapshot_kind_entities(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layouts (
    snapshot_id                 TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    typology_external_id        TEXT NOT NULL,
    external_id                 TEXT NOT NULL,
    ordinal                     INT NOT NULL DEFAULT 0,
    folder_external_id          TEXT NOT NULL DEFAULT '',
    active_stratum_external_id  TEXT NOT NULL DEFAULT '',
    location_external_id        TEXT NOT NULL DEFAULT '',
    name                        TEXT NOT NULL DEFAULT '',
    description                 TEXT NOT NULL DEFAULT '',
    icon                        TEXT NOT NULL DEFAULT '',
    image                       TEXT NOT NULL DEFAULT '',
    unit_name                   TEXT NOT NULL DEFAULT '',
    is_abstract                 BOOLEAN,
    can_scale                   BOOLEAN,
    can_mirror                  BOOLEAN,
    created_value               TIMESTAMPTZ,
    updated_value               TIMESTAMPTZ,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, typology_external_id, ordinal),
    FOREIGN KEY (snapshot_id, typology_external_id) REFERENCES kit_snapshot_typologies(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_family_refs (
    snapshot_id         TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id  TEXT NOT NULL,
    family_external_id  TEXT NOT NULL,
    ordinal             INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, layout_external_id, family_external_id),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, family_external_id) REFERENCES kit_snapshot_families(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_author_refs (
    snapshot_id         TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id  TEXT NOT NULL,
    person_external_id  TEXT NOT NULL,
    ordinal             INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, layout_external_id, person_external_id),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, person_external_id) REFERENCES kit_snapshot_people(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_concept_refs (
    snapshot_id         TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id  TEXT NOT NULL,
    concept_external_id TEXT NOT NULL,
    ordinal             INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, layout_external_id, concept_external_id),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, concept_external_id) REFERENCES kit_snapshot_concepts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_tag_refs (
    snapshot_id         TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id  TEXT NOT NULL,
    tag_external_id     TEXT NOT NULL,
    ordinal             INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, layout_external_id, tag_external_id),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, tag_external_id) REFERENCES kit_snapshot_tags(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_pieces (
    snapshot_id             TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id      TEXT NOT NULL,
    external_id             TEXT NOT NULL,
    ordinal                 INT NOT NULL DEFAULT 0,
    kind_external_id        TEXT NOT NULL DEFAULT '',
    child_layout_external_id TEXT NOT NULL DEFAULT '',
    name                    TEXT NOT NULL DEFAULT '',
    description             TEXT NOT NULL DEFAULT '',
    plane_json              JSONB NOT NULL DEFAULT '{}',
    center_json             JSONB NOT NULL DEFAULT '{}',
    scale_json              JSONB NOT NULL DEFAULT '{}',
    mirror_plane_json       JSONB NOT NULL DEFAULT '{}',
    color                   TEXT NOT NULL DEFAULT '',
    hidden                  BOOLEAN,
    locked                  BOOLEAN,
    payload_json            JSONB NOT NULL DEFAULT '{}',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, layout_external_id, ordinal),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_connections (
    snapshot_id               TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id        TEXT NOT NULL,
    external_id               TEXT NOT NULL,
    ordinal                   INT NOT NULL DEFAULT 0,
    connected_side_json       JSONB NOT NULL DEFAULT '{}',
    connecting_side_json      JSONB NOT NULL DEFAULT '{}',
    gap_value                 DOUBLE PRECISION,
    shift_value               DOUBLE PRECISION,
    rise_value                DOUBLE PRECISION,
    rotation_value            DOUBLE PRECISION,
    turn_value                DOUBLE PRECISION,
    tilt_value                DOUBLE PRECISION,
    x_value                   DOUBLE PRECISION,
    y_value                   DOUBLE PRECISION,
    description               TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, layout_external_id, ordinal),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_strata (
    snapshot_id           TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id    TEXT NOT NULL,
    external_id           TEXT NOT NULL,
    ordinal               INT NOT NULL DEFAULT 0,
    name                  TEXT NOT NULL DEFAULT '',
    description           TEXT NOT NULL DEFAULT '',
    color                 TEXT NOT NULL DEFAULT '',
    order_value           BIGINT,
    visible               BOOLEAN,
    locked                BOOLEAN,
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, layout_external_id, ordinal),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_groups (
    snapshot_id           TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id    TEXT NOT NULL,
    external_id           TEXT NOT NULL,
    ordinal               INT NOT NULL DEFAULT 0,
    name                  TEXT NOT NULL DEFAULT '',
    description           TEXT NOT NULL DEFAULT '',
    color                 TEXT NOT NULL DEFAULT '',
    icon                  TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, external_id),
    UNIQUE (snapshot_id, layout_external_id, ordinal),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_layout_group_piece_refs (
    snapshot_id          TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    group_external_id    TEXT NOT NULL,
    piece_external_id    TEXT NOT NULL,
    ordinal              INT NOT NULL DEFAULT 0,
    PRIMARY KEY (snapshot_id, group_external_id, piece_external_id),
    FOREIGN KEY (snapshot_id, group_external_id) REFERENCES kit_snapshot_layout_groups(snapshot_id, external_id) ON DELETE CASCADE,
    FOREIGN KEY (snapshot_id, piece_external_id) REFERENCES kit_snapshot_layout_pieces(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS kit_snapshot_properties (
    snapshot_id          TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    owner_kind           TEXT NOT NULL CHECK (owner_kind IN ('kit', 'kind', 'layout', 'piece')),
    owner_external_id    TEXT NOT NULL DEFAULT '',
    external_id          TEXT NOT NULL,
    ordinal              INT NOT NULL DEFAULT 0,
    quality_external_id  TEXT NOT NULL DEFAULT '',
    key_name             TEXT NOT NULL DEFAULT '',
    value_text           TEXT NOT NULL DEFAULT '',
    unit_name            TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, owner_kind, owner_external_id, external_id),
    UNIQUE (snapshot_id, owner_kind, owner_external_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_attributes (
    snapshot_id          TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    owner_kind           TEXT NOT NULL CHECK (owner_kind IN ('kit', 'kind', 'layout', 'piece', 'connection', 'kind-endpoint', 'kind-connector', 'kind-representation')),
    owner_external_id    TEXT NOT NULL DEFAULT '',
    external_id          TEXT NOT NULL,
    ordinal              INT NOT NULL DEFAULT 0,
    key_name             TEXT NOT NULL DEFAULT '',
    value_text           TEXT NOT NULL DEFAULT '',
    definition_text      TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, owner_kind, owner_external_id, external_id),
    UNIQUE (snapshot_id, owner_kind, owner_external_id, ordinal)
);

CREATE TABLE IF NOT EXISTS kit_snapshot_metrics (
    snapshot_id          TEXT NOT NULL REFERENCES kit_snapshots(id) ON DELETE CASCADE,
    layout_external_id   TEXT NOT NULL,
    external_id          TEXT NOT NULL,
    ordinal              INT NOT NULL DEFAULT 0,
    key_name             TEXT NOT NULL DEFAULT '',
    value_text           TEXT,
    unit_name            TEXT,
    definition_text      TEXT,
    description          TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (snapshot_id, layout_external_id, external_id),
    UNIQUE (snapshot_id, layout_external_id, ordinal),
    FOREIGN KEY (snapshot_id, layout_external_id) REFERENCES kit_snapshot_layouts(snapshot_id, external_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_kit_snapshot_kind_entities_name ON kit_snapshot_kind_entities(snapshot_id, name);
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_layouts_name ON kit_snapshot_layouts(snapshot_id, name);
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_layout_pieces_kind ON kit_snapshot_layout_pieces(snapshot_id, kind_external_id);
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_layout_connections_layout ON kit_snapshot_layout_connections(snapshot_id, layout_external_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_properties_owner ON kit_snapshot_properties(snapshot_id, owner_kind, owner_external_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_attributes_owner ON kit_snapshot_attributes(snapshot_id, owner_kind, owner_external_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_metrics_layout ON kit_snapshot_metrics(snapshot_id, layout_external_id, ordinal);
-- #endregion 🧬Kits

-- #region 📋Tickets
-- [🧰repo📚postgres📐schema🔖tickets](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Tickets)
-- Ticket lifecycle tables for tracked work items.

CREATE TABLE IF NOT EXISTS tickets (
    id           TEXT PRIMARY KEY,
    status       TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'closed')),
    title        TEXT NOT NULL DEFAULT '',
    prompt       TEXT NOT NULL DEFAULT '',
    summary      TEXT NOT NULL DEFAULT '',
    llm          TEXT NOT NULL DEFAULT '',
    client       TEXT NOT NULL DEFAULT '',
    author       TEXT NOT NULL DEFAULT '',
    github_issue TEXT NOT NULL DEFAULT '',
    goal         TEXT NOT NULL DEFAULT '',
    parent       TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at    TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_tickets_status_created ON tickets(status, created_at DESC);

CREATE TABLE IF NOT EXISTS ticket_files (
    id        TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    UNIQUE(ticket_id, file_path)
);

CREATE INDEX IF NOT EXISTS idx_ticket_files_ticket ON ticket_files(ticket_id);
-- #endregion 📋Tickets

-- #region 🧩Scopes
-- [🧰repo📚postgres📐schema🔖scopes](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Scopes)
-- Code scope indexing and claim tracking.

CREATE TABLE IF NOT EXISTS scopes (
    id              TEXT PRIMARY KEY,
    kind            TEXT NOT NULL CHECK (kind IN ('file', 'section', 'definition')),
    file_path       TEXT NOT NULL,
    section_path    TEXT NOT NULL DEFAULT '',
    definition_name TEXT NOT NULL DEFAULT '',
    start_line      INT NOT NULL DEFAULT 0,
    end_line        INT NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS ticket_claims (
    ticket_id     TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    scope_id      TEXT NOT NULL REFERENCES scopes(id) ON DELETE CASCADE,
    claim_kind    TEXT NOT NULL DEFAULT 'touched',
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (ticket_id, scope_id)
);
-- #endregion 🧩Scopes

-- #region 🎊Warnings
-- [🧰repo📚postgres📐schema🔖warnings](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Warnings)
-- Warning and breach detection tables.

CREATE TABLE IF NOT EXISTS warnings (
    id              TEXT PRIMARY KEY,
    kind            TEXT NOT NULL,
    severity        TEXT NOT NULL DEFAULT 'info',
    message         TEXT NOT NULL DEFAULT '',
    ticket_id       TEXT NOT NULL DEFAULT '',
    scope_id        TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    acknowledged_at TIMESTAMPTZ,
    ack_by          TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_warnings_ticket ON warnings(ticket_id, created_at DESC);

CREATE TABLE IF NOT EXISTS breaches (
    id          TEXT PRIMARY KEY,
    kind        TEXT NOT NULL,
    priority    TEXT NOT NULL DEFAULT 'low',
    scope_id    TEXT NOT NULL DEFAULT '',
    file_path   TEXT NOT NULL DEFAULT '',
    line        INT,
    col         INT,
    summary     TEXT NOT NULL DEFAULT '',
    excerpt     TEXT NOT NULL DEFAULT '',
    autofixable BOOLEAN NOT NULL DEFAULT false,
    detected_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ticket_id   TEXT NOT NULL DEFAULT '',
    resolved_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_breaches_ticket ON breaches(ticket_id, detected_at DESC);
-- #endregion 🎊Warnings

-- #region ⛅Events
-- [🧰repo📚postgres📐schema🔖events](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Events)
-- Event log for all system and user actions.

CREATE TABLE IF NOT EXISTS events (
    id           TEXT PRIMARY KEY,
    kind         TEXT NOT NULL,
    source       TEXT NOT NULL DEFAULT '',
    payload_json JSONB NOT NULL DEFAULT '{}',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_events_created ON events(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_events_kind_created ON events(kind, created_at DESC);
-- #endregion ⛅Events

-- #region 🗂️ContributorWork
-- [🧰repo📚postgres📐schema🔖contributorwork](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/ContributorWork)
-- Contributor work tracking for conflict detection.

CREATE TABLE IF NOT EXISTS contributor_work (
    github  TEXT NOT NULL,
    kind    TEXT NOT NULL,
    item_id TEXT NOT NULL,
    PRIMARY KEY (github, kind, item_id)
);
-- #endregion 🗂️ContributorWork

-- #region ❄️Goals
-- [🧰repo📚postgres📐schema🔖goals](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Goals)
-- Goal tracking tables.

CREATE TABLE IF NOT EXISTS goals (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'closed')),
    parent      TEXT,
    due_date    DATE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at   TIMESTAMPTZ
);
-- #endregion ❄️Goals

-- #region 📌Artifacts
-- [🧰repo📚postgres📐schema🔖artifacts](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Artifacts)
-- Uploaded ticket folder artifacts.

CREATE TABLE IF NOT EXISTS artifacts (
    id          TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    ticket_id   TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    file_name   TEXT NOT NULL,
    file_size   BIGINT NOT NULL DEFAULT 0,
    checksum    TEXT NOT NULL DEFAULT '',
    mime_kind   TEXT NOT NULL DEFAULT 'application/gzip',
    blob_path   TEXT NOT NULL DEFAULT '',
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_artifacts_ticket ON artifacts(ticket_id);
-- #endregion 📌Artifacts

-- #region 🔷Discord
-- [🧰repo📚postgres📐schema🔖discord](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Discord)
-- Discord delivery tracking.

CREATE TABLE IF NOT EXISTS discord_channels (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    event_kind   TEXT NOT NULL,
    channel_id   TEXT NOT NULL,
    webhook_url  TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS discord_deliveries (
    id          TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    event_id    TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    channel_id  TEXT NOT NULL DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed', 'dead')),
    attempts    INT NOT NULL DEFAULT 0,
    last_error  TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at     TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_discord_deliveries_status ON discord_deliveries(status) WHERE status = 'pending';
-- #endregion 🔷Discord

-- #region 🖼️Migration
-- [🧰repo📚postgres📐schema🔖migration](repo://p/i/repo/b/l/postgres/f/🛢️schema.sql/s/Migration)
-- Migration ledger for tracking history imports.

CREATE TABLE IF NOT EXISTS migration_runs (
    id         TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    source     TEXT NOT NULL,
    status     TEXT NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'completed', 'failed')),
    rows_ok    INT NOT NULL DEFAULT 0,
    rows_fail  INT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at   TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS migration_failures (
    id          TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    run_id      TEXT NOT NULL REFERENCES migration_runs(id) ON DELETE CASCADE,
    source_path TEXT NOT NULL DEFAULT '',
    reason      TEXT NOT NULL DEFAULT '',
    raw_data    JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- #endregion 🖼️Migration