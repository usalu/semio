//#region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
/**
 * 🧬️ TypeScript projection of the coordinator wire contracts.
 *
 * Every exported type and `parse<Export>` function implements one `$defs` entry of the sibling
 * {@link https://semio.tech/schema/repo/server/coordinator/schema.json 🔣️.json} draft-07 document.
 * Parsers are hand-written (no code generation, no external validator) and are the single
 * request-validation authority of the coordinator's HTTP surface.
 */
//#endregion 🧲️Header

//#region 🔖️Contracts
/** 🪪️ Canonical `$id` of the schema these parsers implement. */
export const COORDINATOR_SCHEMA_ID = "https://semio.tech/schema/repo/server/coordinator/schema.json";

/** 🎁️ Outcome of one parse, free of any implementation-specific error type. */
export type ParseResult<T> = { readonly success: true; readonly data: T } | { readonly success: false; readonly error: { readonly message: string } };

/** ⏱️ RFC-3339 instant, the spelling every `*_at` column serialises to. */
export type Timestamp = string;

/** ⏱️ A `Timestamp` column that is nullable in the DDL. */
export type NullableTimestamp = Timestamp | null;

/** 🔤️ A `text` column that is nullable in the DDL. */
export type NullableString = string | null;

/** 🔢️ An `integer` column that is nullable in the DDL. */
export type NullableInteger = number | null;

/** 📧️ A developer's e-mail address, the natural key of the auth endpoints. */
export type EmailAddress = string;

export type DeveloperRole = "developer" | "admin" | "owner";
export type TicketStatus = "open" | "closed";
export type ScopeKind = "file" | "section" | "definition";
export type WarningSeverity = "info" | "warning" | "error";

export interface ErrorResponse {
  readonly error: string;
}

export interface StatusResponse {
  readonly status: "ok";
}

export interface AuthWhoAmIResponse {
  readonly id: string;
  readonly email: EmailAddress;
  readonly github_login: NullableString;
  readonly display_name: string;
  readonly role: DeveloperRole;
  readonly trusted: boolean;
}

export interface Developer {
  readonly id: string;
  readonly email: EmailAddress;
  readonly github_login: NullableString;
  readonly display_name: string;
  readonly trusted: boolean;
  readonly active: boolean;
  readonly role: DeveloperRole;
  readonly discord_user_id: NullableString;
  readonly created_at: Timestamp;
  readonly revoked_at: NullableTimestamp;
}

export interface DeveloperApiKey {
  readonly id: string;
  readonly developer_id: string;
  readonly key_hash: string;
  readonly label: string;
  readonly created_at: Timestamp;
  readonly last_used_at: NullableTimestamp;
  readonly revoked_at: NullableTimestamp;
}

export interface CreateDeveloperRequest {
  readonly action: "create-developer";
  readonly email: EmailAddress;
  readonly github_login: string;
  readonly display_name: string;
  readonly trusted: boolean;
  readonly role: DeveloperRole;
}

export interface CreateKeyRequest {
  readonly action: "create-key";
  readonly developer_email: EmailAddress;
  readonly label: string;
}

export interface CreateKeyResponse {
  readonly key: string;
  readonly id: string;
  readonly label: string;
}

export interface RevokeKeyRequest {
  readonly action: "revoke-key";
  readonly key_id: string;
}

export interface Ticket {
  readonly id: string;
  readonly status: TicketStatus;
  readonly title: string;
  readonly prompt: string;
  readonly summary: string;
  readonly llm: string;
  readonly client: string;
  readonly author: string;
  readonly github_issue: string;
  readonly goal: string;
  readonly parent: NullableString;
  readonly created_at: Timestamp;
  readonly closed_at: NullableTimestamp;
}

export interface TicketOpenRequest {
  readonly action: "open";
  readonly ticket_id: string;
  readonly title: string;
  readonly prompt: string;
  readonly llm: string;
  readonly client: string;
  readonly author: string;
  readonly github_issue: string;
  readonly goal: string;
  readonly parent: string | null;
}

export interface TicketCloseRequest {
  readonly action: "close";
  readonly ticket_id: string;
  readonly summary: string;
  readonly files: readonly string[];
}

export interface TicketReopenRequest {
  readonly action: "reopen";
  readonly ticket_id: string;
  readonly prompt: string;
  readonly llm: string;
  readonly title: string;
  readonly client: string;
}

export type TicketListResponse = readonly Ticket[];

export interface TicketDetailResponse {
  readonly ticket: Ticket;
  readonly claims: readonly Scope[];
}

export interface Scope {
  readonly id: string;
  readonly kind: ScopeKind;
  readonly file_path: string;
  readonly section_path: string;
  readonly definition_name: string;
  readonly start_line: number;
  readonly end_line: number;
  readonly updated_at: Timestamp;
}

export type ScopeListResponse = readonly Scope[];

export interface Warning {
  readonly id: string;
  readonly kind: string;
  readonly severity: WarningSeverity;
  readonly message: string;
  readonly ticket_id: string;
  readonly scope_id: string;
  readonly created_at: Timestamp;
  readonly acknowledged_at: NullableTimestamp;
  readonly ack_by: string;
}

export type WarningListResponse = readonly Warning[];

export interface Breach {
  readonly id: string;
  readonly kind: string;
  readonly priority: string;
  readonly scope_id: string;
  readonly file_path: string;
  readonly line: NullableInteger;
  readonly col: NullableInteger;
  readonly summary: string;
  readonly excerpt: string;
  readonly autofixable: boolean;
  readonly detected_at: Timestamp;
  readonly ticket_id: string;
  readonly resolved_at: NullableTimestamp;
}

export type BreachListResponse = readonly Breach[];

export interface FileSnapshot {
  readonly path: string;
  readonly content: string;
}

export interface DiffIngestRequest {
  readonly ticket_id: string;
  readonly repo_id: string;
  readonly patch: string;
  readonly snapshots: readonly FileSnapshot[];
}

export interface DiffIngestResponse {
  readonly changed_files: readonly string[];
  readonly claimed_scopes: readonly string[];
  readonly warnings: readonly Warning[];
  readonly breachs: readonly Breach[];
  readonly blockers: readonly string[];
}

export interface EventPublishRequest {
  readonly kind: string;
  readonly source: string;
  readonly payload: unknown;
}

export interface EventPublishResponse {
  readonly status: "ok";
  readonly event_id: string;
}

export interface Event {
  readonly id: string;
  readonly kind: string;
  readonly source: string;
  readonly payload_json: unknown;
  readonly created_at: Timestamp;
}

export type EventListResponse = readonly Event[];

export interface IndexFileRequest {
  readonly action: "index-file";
  readonly file_path: string;
  readonly content: string;
}

export interface ReindexRequest {
  readonly action: "reindex";
}

export interface ReindexResponse {
  readonly files: number;
}

export interface G3EventEnvelope {
  readonly stream: string;
  readonly sequence: number;
  readonly id: string;
  readonly generation: number;
  readonly type: string;
  readonly payload: unknown;
  readonly checksum: string;
}

export interface G3EventLogContract {
  readonly schema: "semio.coordinator.event/1";
  readonly encoding: "canonical-jsonl-lf";
  readonly scalar: "nonempty-no-nul";
  readonly fields: readonly ["stream", "sequence", "id", "generation", "type", "payload", "checksum"];
  readonly checksum: string;
}
//#endregion 🔖️Contracts

//#region 🧱️Primitives
type Checked<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly message: string };
type Check<T> = (value: unknown, path: string) => Checked<T>;

const TIMESTAMP = /^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$/;
const EMAIL = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const HEX64 = /^[a-f0-9]{64}$/;

function fail(path: string, expectation: string): Checked<never> {
  return { ok: false, message: `${path}: ${expectation}` };
}

function text(minLength = 0): Check<string> {
  return (value, path) => {
    if (typeof value !== "string") return fail(path, "expected string");
    if (value.length < minLength) return fail(path, `expected at least ${minLength} character(s)`);
    return { ok: true, value };
  };
}

function pattern(expression: RegExp, expectation: string): Check<string> {
  return (value, path) => {
    if (typeof value !== "string") return fail(path, "expected string");
    if (!expression.test(value)) return fail(path, expectation);
    return { ok: true, value };
  };
}

const timestamp = pattern(TIMESTAMP, "expected an ISO-8601 date-time");
const email = pattern(EMAIL, "expected an email address");
const checksum = pattern(HEX64, "expected a lowercase sha-256 digest");

const boolean: Check<boolean> = (value, path) => (typeof value === "boolean" ? { ok: true, value } : fail(path, "expected boolean"));

function integer(minimum?: number): Check<number> {
  return (value, path) => {
    if (typeof value !== "number" || !Number.isInteger(value)) return fail(path, "expected integer");
    if (minimum !== undefined && value < minimum) return fail(path, `expected at least ${minimum}`);
    return { ok: true, value };
  };
}

const anyJson: Check<unknown> = (value) => ({ ok: true, value });

function literal<const T extends string>(expected: T): Check<T> {
  return (value, path) => (value === expected ? { ok: true, value: expected } : fail(path, `expected ${JSON.stringify(expected)}`));
}

function oneOf<const T extends readonly string[]>(values: T): Check<T[number]> {
  return (value, path) => (typeof value === "string" && values.includes(value) ? { ok: true, value: value as T[number] } : fail(path, `expected one of ${values.join(", ")}`));
}

function nullable<T>(check: Check<T>): Check<T | null> {
  return (value, path) => (value === null ? { ok: true, value: null } : check(value, path));
}

function list<T>(item: Check<T>): Check<readonly T[]> {
  return (value, path) => {
    if (!Array.isArray(value)) return fail(path, "expected array");
    const values: T[] = [];
    for (let index = 0; index < value.length; index++) {
      const checked = item(value[index], `${path}.${index}`);
      if (!checked.ok) return checked;
      values.push(checked.value);
    }
    return { ok: true, value: values };
  };
}

type Fields<T> = { readonly [K in keyof T]: { readonly check: Check<T[K]>; readonly fallback?: () => T[K] } };

function required<T>(check: Check<T>): { readonly check: Check<T> } {
  return { check };
}

function optional<T>(check: Check<T>, fallback: () => T): { readonly check: Check<T>; readonly fallback: () => T } {
  return { check, fallback };
}

function record<T>(fields: Fields<T>): Check<T> {
  return (value, path) => {
    if (typeof value !== "object" || value === null || Array.isArray(value)) return fail(path, "expected object");
    const source = value as Readonly<Record<string, unknown>>;
    for (const key of Object.keys(source)) if (!(key in fields)) return fail(`${path}.${key}`, "unexpected property");
    const data: Record<string, unknown> = {};
    for (const [key, field] of Object.entries(fields) as [string, { check: Check<unknown>; fallback?: () => unknown }][]) {
      if (source[key] === undefined) {
        if (!field.fallback) return fail(`${path}.${key}`, "required property missing");
        data[key] = field.fallback();
        continue;
      }
      const checked = field.check(source[key], `${path}.${key}`);
      if (!checked.ok) return checked;
      data[key] = checked.value;
    }
    return { ok: true, value: data as T };
  };
}

function parser<T>(check: Check<T>): (value: unknown) => ParseResult<T> {
  return (value) => {
    const checked = check(value, "value");
    return checked.ok ? { success: true, data: checked.value } : { success: false, error: { message: checked.message } };
  };
}
//#endregion 🧱️Primitives

//#region 🧬️Checks
const role = oneOf(["developer", "admin", "owner"] as const);
const status = oneOf(["open", "closed"] as const);
const scopeKind = oneOf(["file", "section", "definition"] as const);
const severity = oneOf(["info", "warning", "error"] as const);

const errorResponse = record<ErrorResponse>({ error: required(text(1)) });

const statusResponse = record<StatusResponse>({ status: required(literal("ok")) });

const authWhoAmIResponse = record<AuthWhoAmIResponse>({
  id: required(text(1)),
  email: required(email),
  github_login: required(nullable(text())),
  display_name: required(text()),
  role: required(role),
  trusted: required(boolean),
});

const developer = record<Developer>({
  id: required(text(1)),
  email: required(email),
  github_login: required(nullable(text())),
  display_name: required(text()),
  trusted: required(boolean),
  active: required(boolean),
  role: required(role),
  discord_user_id: required(nullable(text())),
  created_at: required(timestamp),
  revoked_at: required(nullable(timestamp)),
});

const developerApiKey = record<DeveloperApiKey>({
  id: required(text(1)),
  developer_id: required(text(1)),
  key_hash: required(text(1)),
  label: required(text()),
  created_at: required(timestamp),
  last_used_at: required(nullable(timestamp)),
  revoked_at: required(nullable(timestamp)),
});

const createDeveloperRequest = record<CreateDeveloperRequest>({
  action: required(literal("create-developer")),
  email: required(email),
  github_login: optional(text(), () => ""),
  display_name: required(text(1)),
  trusted: optional(boolean, () => false),
  role: optional(role, () => "developer"),
});

const createKeyRequest = record<CreateKeyRequest>({
  action: required(literal("create-key")),
  developer_email: required(email),
  label: optional(text(), () => "cli"),
});

const createKeyResponse = record<CreateKeyResponse>({ key: required(text(1)), id: required(text(1)), label: required(text()) });

const revokeKeyRequest = record<RevokeKeyRequest>({ action: required(literal("revoke-key")), key_id: required(text(1)) });

const ticket = record<Ticket>({
  id: required(text(1)),
  status: required(status),
  title: required(text()),
  prompt: required(text()),
  summary: required(text()),
  llm: required(text()),
  client: required(text()),
  author: required(text()),
  github_issue: required(text()),
  goal: required(text()),
  parent: required(nullable(text())),
  created_at: required(timestamp),
  closed_at: required(nullable(timestamp)),
});

const ticketOpenRequest = record<TicketOpenRequest>({
  action: required(literal("open")),
  ticket_id: required(text(1)),
  title: required(text(1)),
  prompt: optional(text(), () => ""),
  llm: optional(text(), () => ""),
  client: optional(text(), () => ""),
  author: optional(text(), () => ""),
  github_issue: optional(text(), () => ""),
  goal: optional(text(), () => ""),
  parent: optional(nullable(text()), () => null),
});

const ticketCloseRequest = record<TicketCloseRequest>({
  action: required(literal("close")),
  ticket_id: required(text(1)),
  summary: required(text(1)),
  files: optional(list(text()), () => []),
});

const ticketReopenRequest = record<TicketReopenRequest>({
  action: required(literal("reopen")),
  ticket_id: required(text(1)),
  prompt: required(text(1)),
  llm: optional(text(), () => ""),
  title: optional(text(), () => ""),
  client: optional(text(), () => ""),
});

const scope = record<Scope>({
  id: required(text(1)),
  kind: required(scopeKind),
  file_path: required(text()),
  section_path: required(text()),
  definition_name: required(text()),
  start_line: required(integer(0)),
  end_line: required(integer(0)),
  updated_at: required(timestamp),
});

const ticketDetailResponse = record<TicketDetailResponse>({ ticket: required(ticket), claims: required(list(scope)) });

const warning = record<Warning>({
  id: required(text(1)),
  kind: required(text()),
  severity: required(severity),
  message: required(text()),
  ticket_id: required(text()),
  scope_id: required(text()),
  created_at: required(timestamp),
  acknowledged_at: required(nullable(timestamp)),
  ack_by: required(text()),
});

const breach = record<Breach>({
  id: required(text(1)),
  kind: required(text()),
  priority: required(text()),
  scope_id: required(text()),
  file_path: required(text()),
  line: required(nullable(integer())),
  col: required(nullable(integer())),
  summary: required(text()),
  excerpt: required(text()),
  autofixable: required(boolean),
  detected_at: required(timestamp),
  ticket_id: required(text()),
  resolved_at: required(nullable(timestamp)),
});

const fileSnapshot = record<FileSnapshot>({ path: required(text(1)), content: required(text()) });

const diffIngestRequest = record<DiffIngestRequest>({
  ticket_id: required(text(1)),
  repo_id: optional(text(), () => ""),
  patch: required(text(1)),
  snapshots: optional(list(fileSnapshot), () => []),
});

const diffIngestResponse = record<DiffIngestResponse>({
  changed_files: required(list(text())),
  claimed_scopes: required(list(text())),
  warnings: required(list(warning)),
  breachs: required(list(breach)),
  blockers: required(list(text())),
});

const eventPublishRequest = record<EventPublishRequest>({
  kind: required(text(1)),
  source: optional(text(), () => ""),
  payload: optional(anyJson, () => ({})),
});

const eventPublishResponse = record<EventPublishResponse>({ status: required(literal("ok")), event_id: required(text(1)) });

const event = record<Event>({
  id: required(text(1)),
  kind: required(text(1)),
  source: required(text()),
  payload_json: required(anyJson),
  created_at: required(timestamp),
});

const indexFileRequest = record<IndexFileRequest>({
  action: required(literal("index-file")),
  file_path: required(text(1)),
  content: optional(text(), () => ""),
});

const reindexRequest = record<ReindexRequest>({ action: required(literal("reindex")) });

const reindexResponse = record<ReindexResponse>({ files: required(integer(0)) });

const g3EventEnvelope = record<G3EventEnvelope>({
  stream: required(text(1)),
  sequence: required(integer(1)),
  id: required(text(1)),
  generation: required(integer(1)),
  type: required(text(1)),
  payload: required(anyJson),
  checksum: required(checksum),
});

const G3_EVENT_FIELDS = ["stream", "sequence", "id", "generation", "type", "payload", "checksum"] as const;

const g3EventFields: Check<G3EventLogContract["fields"]> = (value, path) => {
  const checked = list(text(1))(value, path);
  if (!checked.ok) return checked;
  if (checked.value.length !== G3_EVENT_FIELDS.length || G3_EVENT_FIELDS.some((name, index) => checked.value[index] !== name)) return fail(path, `expected ${JSON.stringify(G3_EVENT_FIELDS)}`);
  return { ok: true, value: G3_EVENT_FIELDS };
};

const g3EventLogContract = record<G3EventLogContract>({
  schema: required(literal("semio.coordinator.event/1")),
  encoding: required(literal("canonical-jsonl-lf")),
  scalar: required(literal("nonempty-no-nul")),
  fields: required(g3EventFields),
  checksum: required(literal("sha256(stream+nul+sequence+nul+id+nul+generation+nul+type+nul+canonical-payload)")),
});
//#endregion 🧬️Checks

//#region 🚪️Parsers
/** 🔀️ Reads the command discriminator shared by the auth, repo and ticket endpoints. */
export function parseCommandAction(value: unknown): ParseResult<string> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return { success: false, error: { message: "value: expected object" } };
  const action = (value as Readonly<Record<string, unknown>>).action;
  if (typeof action !== "string" || action.length === 0) return { success: false, error: { message: "value.action: expected non-empty string" } };
  return { success: true, data: action };
}

export const parseTimestamp = parser(timestamp);
export const parseNullableTimestamp = parser(nullable(timestamp));
export const parseNullableString = parser(nullable(text()));
export const parseNullableInteger = parser(nullable(integer()));
export const parseEmailAddress = parser(email);
export const parseDeveloperRole = parser(role);
export const parseErrorResponse = parser(errorResponse);
export const parseStatusResponse = parser(statusResponse);
export const parseAuthWhoAmIResponse = parser(authWhoAmIResponse);
export const parseDeveloper = parser(developer);
export const parseDeveloperApiKey = parser(developerApiKey);
export const parseCreateDeveloperRequest = parser(createDeveloperRequest);
export const parseCreateKeyRequest = parser(createKeyRequest);
export const parseCreateKeyResponse = parser(createKeyResponse);
export const parseRevokeKeyRequest = parser(revokeKeyRequest);
export const parseTicket = parser(ticket);
export const parseTicketOpenRequest = parser(ticketOpenRequest);
export const parseTicketCloseRequest = parser(ticketCloseRequest);
export const parseTicketReopenRequest = parser(ticketReopenRequest);
export const parseTicketListResponse = parser(list(ticket));
export const parseTicketDetailResponse = parser(ticketDetailResponse);
export const parseScope = parser(scope);
export const parseScopeListResponse = parser(list(scope));
export const parseWarning = parser(warning);
export const parseWarningListResponse = parser(list(warning));
export const parseBreach = parser(breach);
export const parseBreachListResponse = parser(list(breach));
export const parseFileSnapshot = parser(fileSnapshot);
export const parseDiffIngestRequest = parser(diffIngestRequest);
export const parseDiffIngestResponse = parser(diffIngestResponse);
export const parseEventPublishRequest = parser(eventPublishRequest);
export const parseEventPublishResponse = parser(eventPublishResponse);
export const parseEvent = parser(event);
export const parseEventListResponse = parser(list(event));
export const parseIndexFileRequest = parser(indexFileRequest);
export const parseReindexRequest = parser(reindexRequest);
export const parseReindexResponse = parser(reindexResponse);
export const parseG3EventEnvelope = parser(g3EventEnvelope);
export const parseG3EventLogContract = parser(g3EventLogContract);

/** 🗂️ Every export id of the schema mapped to its parser, keyed exactly like the `$defs` entries. */
export const COORDINATOR_PARSERS: Readonly<Record<string, (value: unknown) => ParseResult<unknown>>> = {
  Timestamp: parseTimestamp,
  NullableTimestamp: parseNullableTimestamp,
  NullableString: parseNullableString,
  NullableInteger: parseNullableInteger,
  EmailAddress: parseEmailAddress,
  DeveloperRole: parseDeveloperRole,
  ErrorResponse: parseErrorResponse,
  StatusResponse: parseStatusResponse,
  AuthWhoAmIResponse: parseAuthWhoAmIResponse,
  Developer: parseDeveloper,
  DeveloperApiKey: parseDeveloperApiKey,
  CreateDeveloperRequest: parseCreateDeveloperRequest,
  CreateKeyRequest: parseCreateKeyRequest,
  CreateKeyResponse: parseCreateKeyResponse,
  RevokeKeyRequest: parseRevokeKeyRequest,
  Ticket: parseTicket,
  TicketOpenRequest: parseTicketOpenRequest,
  TicketCloseRequest: parseTicketCloseRequest,
  TicketReopenRequest: parseTicketReopenRequest,
  TicketListResponse: parseTicketListResponse,
  TicketDetailResponse: parseTicketDetailResponse,
  Scope: parseScope,
  ScopeListResponse: parseScopeListResponse,
  Warning: parseWarning,
  WarningListResponse: parseWarningListResponse,
  Breach: parseBreach,
  BreachListResponse: parseBreachListResponse,
  FileSnapshot: parseFileSnapshot,
  DiffIngestRequest: parseDiffIngestRequest,
  DiffIngestResponse: parseDiffIngestResponse,
  EventPublishRequest: parseEventPublishRequest,
  EventPublishResponse: parseEventPublishResponse,
  Event: parseEvent,
  EventListResponse: parseEventListResponse,
  IndexFileRequest: parseIndexFileRequest,
  ReindexRequest: parseReindexRequest,
  ReindexResponse: parseReindexResponse,
  G3EventEnvelope: parseG3EventEnvelope,
  G3EventLogContract: parseG3EventLogContract,
};
//#endregion 🚪️Parsers
