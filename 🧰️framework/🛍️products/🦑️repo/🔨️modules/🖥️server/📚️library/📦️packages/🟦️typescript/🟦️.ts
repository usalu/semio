// #region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Repo server library: PostgreSQL, auth, events, parsing (Next.js API routes).
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import { createHash } from "crypto";
import {
  createOwnedDatabasePool,
  createOwnedJsonResponse,
  createOwnedServerJobQueue,
  isOwnedServerResponse,
  type OwnedDatabasePool,
  type OwnedServerJob,
  type OwnedServerRequest,
  type OwnedServerResponse,
} from "../../../🎛️coordinator/📦️packages/🟦️typescript/🖥️server-implementations.ts";
// #endregion 🔌️Adapters

// #region 🔖️db
// #region ⏱️Config
// 🗄️Database configuration from environment variables.
const DATABASE_URL = process.env.DATABASE_URL || "postgresql://compose:compose@localhost:5432/compose_repo";

const pool = ephemeralBox<OwnedDatabasePool | null>("framework.products.repo.modules.server.lib.packages.typescript.index.ts.pool", null);

export function getPool(): OwnedDatabasePool {
  if (!pool.current) {
    pool.current = createOwnedDatabasePool(DATABASE_URL, 20);
  }
  return pool.current;
}

export async function closePool(): Promise<void> {
  if (pool.current) {
    await pool.current.end();
    pool.current = null;
  }
}
// #endregion ⏱️Config

// #region ⚙️Types
// Data model types matching the PostgreSQL schema.

export interface Developer {
  id: string;
  email: string;
  github_login: string | null;
  display_name: string;
  trusted: boolean;
  active: boolean;
  role: string;
  discord_user_id: string | null;
  created_at: Date;
  revoked_at: Date | null;
}

export interface DeveloperApiKey {
  id: string;
  developer_id: string;
  key_hash: string;
  label: string;
  created_at: Date;
  last_used_at: Date | null;
  revoked_at: Date | null;
}

export interface Ticket {
  id: string;
  status: string;
  title: string;
  prompt: string;
  summary: string;
  llm: string;
  client: string;
  author: string;
  github_issue: string;
  goal: string;
  parent: string | null;
  created_at: Date;
  closed_at: Date | null;
}

export interface Scope {
  id: string;
  kind: string;
  file_path: string;
  section_path: string;
  definition_name: string;
  start_line: number;
  end_line: number;
  updated_at: Date;
}

export interface Warning {
  id: string;
  kind: string;
  severity: string;
  message: string;
  ticket_id: string;
  scope_id: string;
  created_at: Date;
  acknowledged_at: Date | null;
  ack_by: string;
}

export interface Breach {
  id: string;
  kind: string;
  priority: string;
  scope_id: string;
  file_path: string;
  line: number | null;
  col: number | null;
  summary: string;
  excerpt: string;
  autofixable: boolean;
  detected_at: Date;
  ticket_id: string;
  resolved_at: Date | null;
}

export interface Event {
  id: string;
  kind: string;
  source: string;
  payload_json: unknown;
  created_at: Date;
}

export interface Artifact {
  id: string;
  ticket_id: string;
  file_name: string;
  file_size: number;
  checksum: string;
  mime_kind: string;
  blob_path: string;
  uploaded_at: Date;
}
// #endregion ⚙️Types

// 🔷️#region 🎼️Helpers
function newId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 11)}`;
}
// #endregion 🎼️Helpers

// #region 👓️Developers
// Developer CRUD operations.

export async function getDeveloperByApiKeyHash(keyHash: string): Promise<Developer | null> {
  const p = getPool();
  const result = await p.query(
    `SELECT d.* FROM developers d
     JOIN developer_api_keys k ON d.id = k.developer_id
     WHERE k.key_hash = $1 AND k.revoked_at IS NULL AND d.active = true`,
    [keyHash],
  );
  if (result.rows.length === 0) return null;
  return result.rows[0] as Developer;
}

export async function getDeveloperByEmail(email: string): Promise<Developer | null> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM developers WHERE email = $1`, [email]);
  if (result.rows.length === 0) return null;
  return result.rows[0] as Developer;
}

export async function listDevelopers(): Promise<Developer[]> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM developers ORDER BY created_at DESC`);
  return result.rows as Developer[];
}

export async function createDeveloper(dev: Omit<Developer, "id" | "created_at" | "revoked_at">): Promise<Developer> {
  const p = getPool();
  const id = newId();
  const result = await p.query(
    `INSERT INTO developers (id, email, github_login, display_name, trusted, active, role, discord_user_id)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *`,
    [id, dev.email, dev.github_login, dev.display_name, dev.trusted, dev.active, dev.role, dev.discord_user_id],
  );
  return result.rows[0] as Developer;
}

export async function createApiKey(developerId: string, keyHash: string, label: string): Promise<DeveloperApiKey> {
  const p = getPool();
  const id = newId();
  const result = await p.query(`INSERT INTO developer_api_keys (id, developer_id, key_hash, label) VALUES ($1, $2, $3, $4) RETURNING *`, [id, developerId, keyHash, label]);
  return result.rows[0] as DeveloperApiKey;
}

export async function revokeApiKey(keyId: string): Promise<void> {
  const p = getPool();
  await p.query(`UPDATE developer_api_keys SET revoked_at = now() WHERE id = $1`, [keyId]);
}
// #endregion 👓️Developers

// #region 📋️Tickets
// Ticket CRUD operations.

export async function upsertTicket(ticket: Ticket): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO tickets (id, status, title, prompt, summary, llm, client, author, github_issue, goal, parent, created_at, closed_at)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
     ON CONFLICT(id) DO UPDATE SET
       status=EXCLUDED.status, title=EXCLUDED.title, prompt=EXCLUDED.prompt,
       summary=EXCLUDED.summary, llm=EXCLUDED.llm, client=EXCLUDED.client,
       author=EXCLUDED.author, github_issue=EXCLUDED.github_issue,
       goal=EXCLUDED.goal, parent=EXCLUDED.parent, closed_at=EXCLUDED.closed_at`,
    [ticket.id, ticket.status, ticket.title, ticket.prompt, ticket.summary, ticket.llm, ticket.client, ticket.author, ticket.github_issue, ticket.goal, ticket.parent, ticket.created_at, ticket.closed_at],
  );
}

export async function getTicket(ticketId: string): Promise<Ticket | null> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM tickets WHERE id = $1`, [ticketId]);
  if (result.rows.length === 0) return null;
  return result.rows[0] as Ticket;
}

export async function listTickets(status?: string): Promise<Ticket[]> {
  const p = getPool();
  if (status) {
    const result = await p.query(`SELECT * FROM tickets WHERE status = $1 ORDER BY created_at DESC`, [status]);
    return result.rows as Ticket[];
  }
  const result = await p.query(`SELECT * FROM tickets ORDER BY created_at DESC`);
  return result.rows as Ticket[];
}

export async function insertTicketFiles(ticketId: string, files: string[]): Promise<void> {
  const p = getPool();
  for (const filePath of files) {
    await p.query(`INSERT INTO ticket_files (ticket_id, file_path) VALUES ($1, $2) ON CONFLICT DO NOTHING`, [ticketId, filePath]);
  }
}
// #endregion 📋️Tickets

// #region 🧩️Scopes
// Scope indexing and claim operations.

export async function replaceScopes(filePath: string, scopes: Scope[]): Promise<void> {
  const p = getPool();
  await p.query(`DELETE FROM scopes WHERE file_path = $1`, [filePath]);
  for (const scope of scopes) {
    await p.query(
      `INSERT INTO scopes (id, kind, file_path, section_path, definition_name, start_line, end_line, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
      [scope.id, scope.kind, scope.file_path, scope.section_path, scope.definition_name, scope.start_line, scope.end_line, scope.updated_at],
    );
  }
}

export async function listScopesByFile(filePath: string): Promise<Scope[]> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM scopes WHERE file_path = $1`, [filePath]);
  return result.rows as Scope[];
}

export async function upsertClaim(ticketId: string, scopeId: string, claimKind: string): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO ticket_claims (ticket_id, scope_id, claim_kind, first_seen_at, last_seen_at)
     VALUES ($1, $2, $3, now(), now())
     ON CONFLICT(ticket_id, scope_id) DO UPDATE SET claim_kind=EXCLUDED.claim_kind, last_seen_at=now()`,
    [ticketId, scopeId, claimKind],
  );
}

export async function listClaimsByTicket(ticketId: string): Promise<Scope[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT s.* FROM scopes s
     JOIN ticket_claims tc ON s.id = tc.scope_id
     WHERE tc.ticket_id = $1`,
    [ticketId],
  );
  return result.rows as Scope[];
}

export async function listConflicts(): Promise<{ scope_id: string; tickets: string[] }[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT tc.scope_id, array_agg(tc.ticket_id) as tickets
     FROM ticket_claims tc
     JOIN tickets t ON tc.ticket_id = t.id
     WHERE t.status = 'open'
     GROUP BY tc.scope_id
     HAVING COUNT(tc.ticket_id) > 1`,
  );
  return result.rows.map((r: { scope_id: string; tickets: string[] }) => ({
    scope_id: r.scope_id,
    tickets: r.tickets,
  }));
}
// #endregion 🧩️Scopes

// #region 🎊️Warnings
// Warning and breach operations.

export async function replaceWarnings(warnings: Warning[]): Promise<void> {
  const p = getPool();
  await p.query(`DELETE FROM warnings WHERE kind = 'conflict'`);
  for (const w of warnings) {
    await p.query(
      `INSERT INTO warnings (id, kind, severity, message, ticket_id, scope_id, created_at, acknowledged_at, ack_by)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
      [w.id, w.kind, w.severity, w.message, w.ticket_id, w.scope_id, w.created_at, w.acknowledged_at, w.ack_by],
    );
  }
}

export async function listWarnings(ticketId?: string): Promise<Warning[]> {
  const p = getPool();
  if (ticketId) {
    const result = await p.query(`SELECT * FROM warnings WHERE ticket_id = $1 ORDER BY created_at DESC`, [ticketId]);
    return result.rows as Warning[];
  }
  const result = await p.query(`SELECT * FROM warnings ORDER BY created_at DESC`);
  return result.rows as Warning[];
}

export async function listBreaches(ticketId?: string): Promise<Breach[]> {
  const p = getPool();
  if (ticketId) {
    const result = await p.query(`SELECT * FROM breaches WHERE ticket_id = $1 ORDER BY detected_at DESC`, [ticketId]);
    return result.rows as Breach[];
  }
  const result = await p.query(`SELECT * FROM breaches ORDER BY detected_at DESC`);
  return result.rows as Breach[];
}
// #endregion 🎊️Warnings

// #region ⛅️Events
// Event persistence operations.

export async function insertEvent(event: Event): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO events (id, kind, source, payload_json, created_at)
     VALUES ($1, $2, $3, $4, $5)`,
    [event.id, event.kind, event.source, JSON.stringify(event.payload_json), event.created_at],
  );
}

export async function listEvents(kind?: string, limit: number = 100): Promise<Event[]> {
  const p = getPool();
  if (kind) {
    const result = await p.query(`SELECT * FROM events WHERE kind = $1 ORDER BY created_at DESC LIMIT $2`, [kind, limit]);
    return result.rows as Event[];
  }
  const result = await p.query(`SELECT * FROM events ORDER BY created_at DESC LIMIT $1`, [limit]);
  return result.rows as Event[];
}

export { newId };
// #endregion ⛅️Events

// #region 🗂️ContributorWork
// Contributor work tracking for conflict detection.

export async function addContributorWork(github: string, kind: string, itemId: string): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO contributor_work (github, kind, item_id) VALUES ($1, $2, $3)
     ON CONFLICT DO NOTHING`,
    [github, kind, itemId],
  );
}

export async function removeContributorWork(github: string, entries: { kind: string; id: string }[]): Promise<void> {
  const p = getPool();
  for (const e of entries) {
    await p.query(`DELETE FROM contributor_work WHERE github = $1 AND kind = $2 AND item_id = $3`, [github, e.kind, e.id]);
  }
}

export async function listContributorsOnItem(kind: string, itemId: string): Promise<string[]> {
  const p = getPool();
  const result = await p.query(`SELECT github FROM contributor_work WHERE kind = $1 AND item_id = $2`, [kind, itemId]);
  return result.rows.map((r: { github: string }) => r.github);
}

export async function removeContributorWorkForCheckpoint(github: string, files: string[]): Promise<void> {
  const p = getPool();
  for (const f of files) {
    await p.query(`DELETE FROM contributor_work WHERE github = $1 AND kind = 'file' AND item_id = $2`, [github, f]);
  }
}
// #endregion 🗂️ContributorWork

// #region 📌️Artifacts
// Artifact storage operations.

export async function insertArtifact(artifact: Artifact): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO artifacts (id, ticket_id, file_name, file_size, checksum, mime_kind, blob_path, uploaded_at)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
    [artifact.id, artifact.ticket_id, artifact.file_name, artifact.file_size, artifact.checksum, artifact.mime_kind, artifact.blob_path, artifact.uploaded_at],
  );
}

export async function listArtifactsByTicket(ticketId: string): Promise<Artifact[]> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM artifacts WHERE ticket_id = $1 ORDER BY uploaded_at DESC`, [ticketId]);
  return result.rows as Artifact[];
}
// #endregion 📌️Artifacts

// 📌️#region 📜️AuditLog
export async function insertAuditLog(actorId: string | null, action: string, target: string | null, detail: unknown): Promise<void> {
  const p = getPool();
  await p.query(`INSERT INTO audit_log (actor_id, action, target, detail) VALUES ($1, $2, $3, $4)`, [actorId, action, target, detail ? JSON.stringify(detail) : null]);
}
// #endregion 📜️AuditLog

// 📌️#region 🔷️Discord
export async function insertDiscordDelivery(eventId: string, channelId: string): Promise<void> {
  const p = getPool();
  await p.query(`INSERT INTO discord_deliveries (event_id, channel_id) VALUES ($1, $2)`, [eventId, channelId]);
}

export async function markDiscordDeliverySent(id: string): Promise<void> {
  const p = getPool();
  await p.query(`UPDATE discord_deliveries SET status = 'sent', sent_at = now() WHERE id = $1`, [id]);
}

export async function markDiscordDeliveryFailed(id: string, error: string, attempts: number): Promise<void> {
  const p = getPool();
  const status = attempts >= 4 ? "dead" : "failed";
  await p.query(`UPDATE discord_deliveries SET status = $1, last_error = $2, attempts = $3 WHERE id = $4`, [status, error, attempts, id]);
}
// #endregion 🔷️Discord

// #region 🎞️Schema
// Run schema migration from 🗄️.sql.
import { readFileSync } from "fs";
import { join } from "path";

export async function runSchema(): Promise<void> {
  const p = getPool();
  const schemaPath = join(__dirname, "../../../🧬️schema/🐘️postgres/🗄️.sql");
  const schema = readFileSync(schemaPath, "utf-8");
  await p.query(schema);
}
// #endregion 🎞️Schema
// #endregion 🔖️db

// #region 🔖️parsing
// - Detect markdown headings for .md/.mdx files
// - Language-specific definition patterns for Go, TS, JS, Python, C#, Rust, Ruby
// - Build scope IDs deterministically
// #endregion 🧲️Header

// #region ⚙️Types

export interface ParsedSection {
  name: string;
  path: string;
  startLine: number;
  endLine: number;
}

export interface ParsedDefinition {
  name: string;
  startLine: number;
  endLine: number;
}
// #endregion ⚙️Types

// 🔬️#region 📯️RegionMarker
export function parseRegionMarker(line: string): { name: string; isEnd: boolean } | null {
  let trimmed = line.trim();
  trimmed = trimmed.replace(/^\/\/\s*/, "");
  trimmed = trimmed.replace(/^#\s*/, "");
  trimmed = trimmed.replace(/^--\s*/, "");
  trimmed = trimmed.replace(/^\/\*\s*/, "");
  trimmed = trimmed.replace(/\*\/\s*$/, "");
  trimmed = trimmed.trim();
  if (trimmed.startsWith("#region 🔖️")) {
    return { name: trimmed.replace("#region 🔖️", "").trim(), isEnd: false };
  }
  if (trimmed.startsWith("#endregion 🔖️")) {
    return { name: trimmed.replace("#endregion 🔖️", "").trim(), isEnd: true };
  }
  return null;
}
// #endregion 📯️RegionMarker

// 📰️#region 🪁️MarkdownHeading
export function parseMarkdownHeading(line: string): { level: number; title: string } | null {
  const trimmed = line.trim();
  if (!trimmed.startsWith("#")) return null;
  let level = 0;
  while (level < trimmed.length && trimmed[level] === "#") level++;
  if (level === 0 || level > 6) return null;
  const name = trimmed.slice(level).trim();
  if (!name) return null;
  return { level, title: name };
}
// #endregion 🪁️MarkdownHeading

// 📖️#region 🖋️DefinitionPatterns
export function definitionPatterns(ext: string): RegExp[] {
  switch (ext) {
    case ".go":
      return [/^\s*func\s+(?:\([^)]*\)\s*)?([A-Za-z0-9_]+)/, /^\s*type\s+([A-Za-z0-9_]+)/, /^\s*var\s+([A-Za-z0-9_]+)/, /^\s*const\s+([A-Za-z0-9_]+)/];
    case ".ts":
    case ".tsx":
    case ".js":
    case ".jsx":
      return [/^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z0-9_]+)/, /^\s*(?:export\s+)?class\s+([A-Za-z0-9_]+)/, /^\s*(?:export\s+)?interface\s+([A-Za-z0-9_]+)/, /^\s*(?:export\s+)?type\s+([A-Za-z0-9_]+)/];
    case ".py":
      return [/^\s*def\s+([A-Za-z0-9_]+)/, /^\s*class\s+([A-Za-z0-9_]+)/];
    case ".cs":
      return [/^\s*(?:public|private|protected|internal)?\s*(?:static\s+)?(?:class|struct|interface|enum|record)\s+([A-Za-z0-9_]+)/];
    case ".rs":
      return [/^\s*(?:pub\s+)?fn\s+([A-Za-z0-9_]+)/, /^\s*(?:pub\s+)?struct\s+([A-Za-z0-9_]+)/, /^\s*(?:pub\s+)?enum\s+([A-Za-z0-9_]+)/, /^\s*(?:pub\s+)?trait\s+([A-Za-z0-9_]+)/, /^\s*impl\s+([A-Za-z0-9_]+)/];
    case ".rb":
      return [/^\s*def\s+([A-Za-z0-9_]+)/, /^\s*class\s+([A-Za-z0-9_]+)/, /^\s*module\s+([A-Za-z0-9_]+)/];
    default:
      return [];
  }
}
// #endregion 🖋️DefinitionPatterns

// 📑️#region 🐹️ParseSections
export function parseSectionsFromLines(lines: string[], ext: string): ParsedSection[] {
  const sections: ParsedSection[] = [];
  interface SectionFrame {
    name: string;
    startLine: number;
    level: number;
    path: string;
  }
  const stack: SectionFrame[] = [];

  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i];
    const marker = parseRegionMarker(line);
    if (marker) {
      if (marker.isEnd) {
        if (stack.length > 0) {
          const frame = stack.pop()!;
          sections.push({
            name: frame.name,
            path: frame.path,
            startLine: frame.startLine,
            endLine: lineNumber - 1,
          });
        }
      } else {
        const path = stack.length > 0 ? `${stack[stack.length - 1].path}.${marker.name}` : marker.name;
        stack.push({ name: marker.name, startLine: lineNumber, level: 0, path });
      }
      continue;
    }
    if (ext === ".md" || ext === ".mdx") {
      const heading = parseMarkdownHeading(line);
      if (heading) {
        while (stack.length > 0 && stack[stack.length - 1].level >= heading.level) {
          const frame = stack.pop()!;
          sections.push({
            name: frame.name,
            path: frame.path,
            startLine: frame.startLine,
            endLine: lineNumber - 1,
          });
        }
        const path = stack.length > 0 ? `${stack[stack.length - 1].path}.${heading.title}` : heading.title;
        stack.push({
          name: heading.title,
          startLine: lineNumber,
          level: heading.level,
          path,
        });
      }
    }
  }

  for (const frame of stack) {
    sections.push({
      name: frame.name,
      path: frame.path,
      startLine: frame.startLine,
      endLine: lines.length,
    });
  }
  return sections;
}
// #endregion 🐹️ParseSections

// 📖️#region 💡️ParseDefinitions
export function parseDefinitionsFromLines(lines: string[], patterns: RegExp[]): ParsedDefinition[] {
  const defs: ParsedDefinition[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    for (const pattern of patterns) {
      const match = lines[i].match(pattern);
      if (match && match.length > 1) {
        defs.push({
          name: match[match.length - 1],
          startLine: lineNumber,
          endLine: lineNumber,
        });
        break;
      }
    }
  }
  return defs;
}
// #endregion 💡️ParseDefinitions

// 🔭️#region ⛩️ScopeBuilding
export function buildScopeId(kind: string, filePath: string, sectionPath: string, definition: string): string {
  if (kind === "file") return `file:${filePath}`;
  if (kind === "section") return `section:${filePath}#${sectionPath}`;
  if (sectionPath) return `def:${filePath}#${sectionPath}::${definition}`;
  return `def:${filePath}#${definition}`;
}

export function buildScopesForFile(path: string, content: string): Scope[] {
  const lines = content.split("\n");
  const ext = "." + (path.split(".").pop() || "").toLowerCase();
  const now = new Date();
  const entries: Scope[] = [];

  entries.push({
    id: buildScopeId("file", path, "", ""),
    kind: "file",
    file_path: path,
    section_path: "",
    definition_name: "",
    start_line: 1,
    end_line: lines.length,
    updated_at: now,
  });

  const sections = parseSectionsFromLines(lines, ext);
  for (const s of sections) {
    entries.push({
      id: buildScopeId("section", path, s.path, ""),
      kind: "section",
      file_path: path,
      section_path: s.path,
      definition_name: "",
      start_line: s.startLine,
      end_line: s.endLine,
      updated_at: now,
    });
  }

  const sectionByLine: Record<number, string> = {};
  for (const s of sections) {
    for (let line = s.startLine; line <= s.endLine; line++) {
      sectionByLine[line] = s.path;
    }
  }

  const patterns = definitionPatterns(ext);
  const defs = parseDefinitionsFromLines(lines, patterns);
  for (const d of defs) {
    const sp = sectionByLine[d.startLine] || "";
    entries.push({
      id: buildScopeId("definition", path, sp, d.name),
      kind: "definition",
      file_path: path,
      section_path: sp,
      definition_name: d.name,
      start_line: d.startLine,
      end_line: d.endLine,
      updated_at: now,
    });
  }

  return entries;
}
// #endregion ⛩️ScopeBuilding

// #region 🐍️DiffParsing
// Unified diff parser ported from Go server.

export interface DiffHunk {
  oldRange: { start: number; end: number };
  newRange: { start: number; end: number };
}

export interface DiffFile {
  path: string;
  hunks: DiffHunk[];
  deleted: boolean;
  created: boolean;
}

const hunkHeaderRe = /@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/;

export function parseUnifiedDiff(patch: string): DiffFile[] {
  const lines = patch.split("\n");
  const files: DiffFile[] = [];
  let current: DiffFile | null = null;

  for (const line of lines) {
    if (line.startsWith("diff --git ")) {
      const parts = line.split(" ");
      if (parts.length >= 4) {
        const path = parts[3].replace(/^b\//, "");
        current = { path, hunks: [], deleted: false, created: false };
        files.push(current);
      }
      continue;
    }
    if (line.startsWith("+++ ") && current) {
      if (line.includes("/dev/null")) current.deleted = true;
      continue;
    }
    if (line.startsWith("@@ ") && current) {
      const match = line.match(hunkHeaderRe);
      if (match) {
        const oldStart = parseInt(match[1]);
        const oldCount = match[2] ? parseInt(match[2]) : 1;
        const newStart = parseInt(match[3]);
        const newCount = match[4] ? parseInt(match[4]) : 1;
        current.hunks.push({
          oldRange: { start: oldStart, end: oldStart + oldCount - 1 },
          newRange: { start: newStart, end: newStart + newCount - 1 },
        });
      }
    }
  }
  return files;
}
// #endregion 🐍️DiffParsing
// #endregion 🔖️parsing

// #region 🔖️auth
// 🔷️#region 🩻️Hashing
export function hashApiKey(key: string): string {
  return createHash("sha256").update(key).digest("hex");
}
// #endregion 🩻️Hashing

// #region 📎️Auth
// Authenticate a request by extracting the Bearer token and resolving to a developer.

export async function authenticateRequest(request: OwnedServerRequest): Promise<Developer | null> {
  const authHeader = request.headers.get("Authorization");
  if (!authHeader) return null;
  const parts = authHeader.split(" ");
  if (parts.length !== 2 || parts[0] !== "Bearer") return null;
  const apiKey = parts[1];
  if (!apiKey) return null;
  const keyHash = hashApiKey(apiKey);
  const developer = await getDeveloperByApiKeyHash(keyHash);
  if (!developer) return null;
  if (!developer.active || !developer.trusted) return null;
  return developer;
}

export function unauthorizedResponse(message: string = "unauthorized"): OwnedServerResponse {
  return createOwnedJsonResponse({ error: message }, 401);
}

export function forbiddenResponse(message: string = "forbidden"): OwnedServerResponse {
  return createOwnedJsonResponse({ error: message }, 403);
}

// 🔐️Require authentication and trusted developer status.
export async function requireAuth(request: OwnedServerRequest): Promise<{ developer: Developer } | OwnedServerResponse> {
  const developer = await authenticateRequest(request);
  if (!developer) return unauthorizedResponse();
  return { developer };
}

// 👑️Require admin or owner role.
export async function requireAdmin(request: OwnedServerRequest): Promise<{ developer: Developer } | OwnedServerResponse> {
  const developer = await authenticateRequest(request);
  if (!developer) return unauthorizedResponse();
  if (developer.role !== "admin" && developer.role !== "owner") {
    return forbiddenResponse("admin access required");
  }
  return { developer };
}

// 📩️Type guard to check if auth result is an error response.
export function isAuthError(result: { developer: Developer } | OwnedServerResponse): result is OwnedServerResponse {
  return isOwnedServerResponse(result);
}
// #endregion 📎️Auth
// #endregion 🔖️auth

// #region 🔖️events
// #region 🌡️Publish
// Publish an event: persist to DB and queue Discord delivery.

export async function publishEvent(kind: string, source: string, payload: unknown): Promise<Event> {
  const event: Event = {
    id: newId(),
    kind,
    source,
    payload_json: payload,
    created_at: new Date(),
  };
  await insertEvent(event);
  // Queue Discord delivery for every event
  try {
    await insertDiscordDelivery(event.id, "");
  } catch {
    // Non-critical - event is already persisted
  }
  return event;
}
// #endregion 🌡️Publish

// #region 🔷️Discord
// Discord notification helpers.

const DISCORD_WEBHOOK = process.env.DISCORD_WEBHOOK_URL || "";

export async function sendDiscordMessage(title: string, body: string): Promise<boolean> {
  if (!DISCORD_WEBHOOK) return false;
  try {
    const response = await fetch(DISCORD_WEBHOOK, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ content: `${title}\n${body}` }),
      signal: AbortSignal.timeout(5000),
    });
    return response.ok;
  } catch {
    return false;
  }
}

// 📡️Route event kind to Discord channel tier.
export function getDiscordChannel(eventKind: string): string {
  if (eventKind.startsWith("ticket.")) return "#tickets";
  if (eventKind.includes("warning") || eventKind.includes("breach")) return "#quality";
  if (eventKind.startsWith("goal.")) return "#goals";
  if (eventKind.startsWith("checkpoint")) return "#operations";
  return "#activity";
}
// #endregion 🔷️Discord
// #endregion 🔖️events

// #region 🔖️worker
// #region 🌊️Jobs
// Job handler definitions.

interface DiscordSendJob {
  deliveryId: string;
  title: string;
  body: string;
  attempt: number;
}

async function handleDiscordSend(jobs: OwnedServerJob<DiscordSendJob>[]) {
  for (const job of jobs) {
    const { deliveryId, title, body, attempt } = job.data;
    const success = await sendDiscordMessage(title, body);
    if (success) {
      await markDiscordDeliverySent(deliveryId);
    } else {
      await markDiscordDeliveryFailed(deliveryId, "delivery failed", attempt + 1);
    }
  }
}

interface ReindexJob {
  repoRoot: string;
}

async function handleReindex(jobs: OwnedServerJob<ReindexJob>[]) {
  for (const job of jobs) {
    console.log(`[worker] reindex job for ${job.data.repoRoot}`);
  }
}
// #endregion 🌊️Jobs

// #region 🌩️Main
/** @emoji 🌩️ Starts pg-boss workers (separate process entry via `🟦️worker.ts`). */
export async function runRepoServerWorker(): Promise<void> {
  const boss = createOwnedServerJobQueue(DATABASE_URL);

  boss.on("error", (error) => console.error("[pg-boss error]", error));

  await boss.start();
  console.log("[worker] pg-boss started");

  await boss.work<DiscordSendJob>("discord.send", handleDiscordSend);
  await boss.work<ReindexJob>("repo.reindex", handleReindex);

  console.log("[worker] listening for jobs");

  process.on("SIGINT", async () => {
    console.log("[worker] shutting down...");
    await boss.stop();
    process.exit(0);
  });

  process.on("SIGTERM", async () => {
    console.log("[worker] shutting down...");
    await boss.stop();
    process.exit(0);
  });
}
// #endregion 🌩️Main
// #endregion 🔖️worker
