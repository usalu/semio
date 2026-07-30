-- #region 🧲Header
-- sql/sqlite/repo/🛢️schema.sql
-- 2025 Ueli Saluz <ueli@semio-tech.com>
-- #region 🪬License
-- This program is free software: you can redistribute it and/or modify
-- it under the terms of the GNU Affero General Public License as
-- published by the Free Software Foundation, either version 3 of the
-- License, or (at your option) any later version.
-- This program is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU Affero General Public License for more details.
-- You should have received a copy of the GNU Affero General Public License
-- along with this program.  If not, see <https://www.gnu.org/licenses/>.
-- #endregion 🪬License
-- #endregion 🧲Header
-- #region 🛒Domains
-- unix_s   INTEGER  -- Unix timestamp in seconds (UTC, >= 0)
-- unix_s_day_end   INTEGER  -- Unix timestamp in seconds (UTC, >= 0) year.month.day 23:59:59
-- unix_s_month INTEGER -- Unix timestamp in seconds (UTC, >= 0) year.month.01 00:00:00
-- pos_int  INTEGER  -- Positive integer (> 0)
-- nn_text  TEXT     -- Non-empty trimmed text
-- enum     INTEGER  -- Enum integer with CHECK constraint
-- NOTE: Foreign keys require:
-- PRAGMA foreign_keys = ON;
-- #endregion 🛒Domains
-- #region 🛕Tables
CREATE TABLE IF NOT EXISTS contributor (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alias TEXT NOT NULL UNIQUE CHECK (length (trim(alias)) > 0) -- nn_text
    github TEXT NOT NULL UNIQUE CHECK (length (trim(github)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
);
CREATE TABLE IF NOT EXISTS release (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    initial_month INTEGER NOT NULL CHECK (initial_month >= 0), -- unix_s_month
    number INTEGER NOT NULL CHECK (number > 0), -- pos_int
    due INTEGER CHECK (due IS NULL OR due >= 0), -- unix_s_day_end
    published INTEGER CHECK (published IS NULL OR published >= 0), -- unix_s
    description TEXT NOT NULL CHECK (length (trim(description)) > 0), -- nn_text
    UNIQUE (initial_month, number)
);
CREATE TABLE IF NOT EXISTS release_contributors (
    release_id INTEGER NOT NULL,
    contributor_id INTEGER NOT NULL,
    UNIQUE (release_id, contributor_id),
    FOREIGN KEY (release_id) REFERENCES release (id) ON DELETE CASCADE,
    FOREIGN KEY (contributor_id) REFERENCES contributor (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS version (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    release_id INTEGER NOT NULL,
    contributor_id INTEGER NOT NULL,
    number INTEGER NOT NULL CHECK (number > 0), -- pos_int
    second INTEGER NOT NULL CHECK (second >= 0), -- unix_s
    message TEXT NOT NULL CHECK (length (trim(message)) > 0), -- nn_text
    UNIQUE (release_id, number),
    FOREIGN KEY (release_id) REFERENCES release (id) ON DELETE CASCADE,
    FOREIGN KEY (contributor_id) REFERENCES contributor (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS checkpoint (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version_id INTEGER NOT NULL,
    contributor_id INTEGER NOT NULL,
    number INTEGER NOT NULL CHECK (number > 0), -- pos_int
    second INTEGER NOT NULL CHECK (second >= 0), -- unix_s
    message TEXT NOT NULL CHECK (length (trim(message)) > 0), -- nn_text
    UNIQUE (version_id, contributor_id, number),
    FOREIGN KEY (version_id) REFERENCES version (id) ON DELETE CASCADE,
    FOREIGN KEY (contributor_id) REFERENCES contributor (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS folder_kind (
    id INTEGER PRIMARY KEY, -- enum[🗃️organizational🛅required]
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS folder (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    checkpoint_id INTEGER NOT NULL,
    parent_folder_id INTEGER,
    kind_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT NOT NULL CHECK (length (trim(summary)) > 0), -- nn_text
    UNIQUE (checkpoint_id, parent_folder_id, name),
    FOREIGN KEY (checkpoint_id) REFERENCES checkpoint (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_folder_id) REFERENCES folder (id) ON DELETE CASCADE,
    FOREIGN KEY (kind_id) REFERENCES folder_kind (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS file_kind (
    id INTEGER PRIMARY KEY, -- enum[💻code🥼lab📜script📝docs⚙️config💾binary📋template⚖️license]
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    description TEXT NOT NULL CHECK (length (trim(description)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS file (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    checkpoint_id INTEGER NOT NULL,
    parent_folder_id INTEGER,
    kind_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK (length (trim(name)) > 0), -- nn_text
    extension TEXT NOT NULL CHECK (length (trim(extension)) > 0), -- nn_text
    summary TEXT NOT NULL CHECK (length (trim(summary)) > 0), -- nn_text
    UNIQUE (checkpoint_id, parent_folder_id, kind_id, name, extension),
    FOREIGN KEY (checkpoint_id) REFERENCES checkpoint (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_folder_id) REFERENCES folder (id) ON DELETE CASCADE,
    FOREIGN KEY (kind_id) REFERENCES file_kind (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS technology_kind (
    id INTEGER PRIMARY KEY, -- enum[👤user🧰infrastructure🔬research]
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    description TEXT NOT NULL CHECK (length (trim(description)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS technology (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_id INTEGER NOT NULL,
    kind_id INTEGER NOT NULL,
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT,
    FOREIGN KEY (folder_id) REFERENCES folder (id) ON DELETE CASCADE,
    FOREIGN KEY (kind_id) REFERENCES technology_kind (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS entity (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    technology_id INTEGER,
    bundle_id INTEGER,
    folder_id INTEGER,
    file_id INTEGER,
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT NOT NULL CHECK (length (trim(summary)) > 0), -- nn_text
    specification TEXT NOT NULL CHECK (length (trim(specification)) > 0), -- nn_text
    UNIQUE (name),
    UNIQUE (emoji),
    FOREIGN KEY (technology_id) REFERENCES technology (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS mechanism (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT NOT NULL CHECK (length (trim(summary)) > 0), -- nn_text
    specification TEXT NOT NULL CHECK (length (trim(specification)) > 0), -- nn_text
    UNIQUE (name, emoji)
);
CREATE TABLE IF NOT EXISTS system (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    summary TEXT NOT NULL CHECK (length (trim(summary)) > 0), -- nn_text
    specification TEXT NOT NULL CHECK (length (trim(specification)) > 0), -- nn_text
    UNIQUE (name, emoji)
);
CREATE TABLE IF NOT EXISTS system_entities (
    system_id INTEGER NOT NULL,
    entity_id INTEGER NOT NULL,
    UNIQUE (system_id, entity_id),
    FOREIGN KEY (system_id) REFERENCES system (id) ON DELETE CASCADE,
    FOREIGN KEY (entity_id) REFERENCES entity (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS bundle_kind (
    id INTEGER PRIMARY KEY, -- enum[📚library🛂schema⌨️binary🖱️ui📔examples🌐site🏪assets]
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS bundle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    technology_id INTEGER NOT NULL,
    folder_id INTEGER NOT NULL,
    kind_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT,
    UNIQUE (technology_id, kind_id, name),
    FOREIGN KEY (technology_id) REFERENCES technology (id) ON DELETE CASCADE,
    FOREIGN KEY (folder_id) REFERENCES folder (id) ON DELETE CASCADE,
    FOREIGN KEY (kind_id) REFERENCES bundle_kind (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS section (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT,
    UNIQUE (file_id, name),
    FOREIGN KEY (file_id) REFERENCES file (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS definition_kind (
    id INTEGER PRIMARY KEY, -- enum[🛠️implementation🔌interface🪨constant🧪test]
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    description TEXT NOT NULL CHECK (length (trim(description)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS definition (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id INTEGER NOT NULL,
    kind_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK (length (trim(name)) > 0), -- nn_text
    summary TEXT,
    code TEXT,
    UNIQUE (section_id, kind_id, name),
    FOREIGN KEY (section_id) REFERENCES section (id) ON DELETE CASCADE,
    FOREIGN KEY (kind_id) REFERENCES definition_kind (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS client_kind (
    id INTEGER PRIMARY KEY, -- enum[🎗️vscode🖱️cursor🌊cascade⚙️codex❄️claude-code🦾droid]
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    description TEXT NOT NULL CHECK (length (trim(description)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS agent (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    emoji TEXT NOT NULL UNIQUE CHECK (length (trim(emoji)) > 0), -- nn_text
    name TEXT NOT NULL UNIQUE CHECK (length (trim(name)) > 0), -- nn_text
    prompt TEXT NOT NULL CHECK (length (trim(prompt)) > 0) -- nn_text
);
CREATE TABLE IF NOT EXISTS session (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contributor_id INTEGER NOT NULL,
    client_kind_id INTEGER NOT NULL,
    initial_second INTEGER NOT NULL CHECK (initial_second >= 0), -- unix_s
    agent_id INTEGER NOT NULL,
    session_id TEXT NOT NULL UNIQUE CHECK (length (trim(session_id)) > 0), -- nn_text
    checkpoint_sha TEXT, -- sha of the current checkpoint when session started
    UNIQUE (contributor_id, initial_second),
    FOREIGN KEY (agent_id) REFERENCES agent (id) ON DELETE CASCADE,
    FOREIGN KEY (contributor_id) REFERENCES contributor (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS event_kind (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_kind_id INTEGER,
    FOREIGN KEY (parent_kind_id) REFERENCES event_kind (id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS event (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kind_id INTEGER NOT NULL,
    contributor_id INTEGER NOT NULL,
    second INTEGER NOT NULL CHECK (second >= 0), -- unix_s
    client_id INTEGER NOT NULL, -- event.client
    agent_id INTEGER NOT NULL, -- event.agent
    session_id INTEGER NOT NULL, -- event.session
    policy_id INTEGER NOT NULL, -- event.policy
    ticket_id INTEGER NOT NULL, -- event.ticket
    FOREIGN KEY (kind_id) REFERENCES event_kind (id) ON DELETE CASCADE,
    FOREIGN KEY (contributor_id) REFERENCES contributor (id) ON DELETE CASCADE
);

-- #endregion 🛕Tables
