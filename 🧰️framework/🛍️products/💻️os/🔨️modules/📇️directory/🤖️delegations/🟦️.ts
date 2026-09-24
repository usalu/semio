/** 🤖️ Agent-delegation surface contract — the human-facing half of M6's delegated agent credential:
 * create a scoped, revocable delegation for an AI agent, download the credential file exactly once,
 * see what is outstanding and when each delegation was last used, and withdraw one. Pure projection
 * + request construction: every call leaves as one of the hub's own `/auth/agent-delegations` routes
 * (`📓️m6-agent-principal-in-hub-space.md` §2.6) and this module never holds a capability beyond the
 * object it hands the pane while the human is looking at it.
 *
 * The bounds below are the hub's own (`semio_hub::auth::agent`), so a request this module builds is
 * refused here rather than at the hub, and the credential file it writes is byte-compatible with
 * what `semio-os-mcp --credential-file` decodes (`🌉️mcp/🤖️agent-credential/🦀️.rs`).
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice M6b. */

//#region 🔖️Routes
export const AGENT_DELEGATION_PATH_V1 = "/auth/agent-delegations";
export const AGENT_SESSION_PATH_V1 = "/auth/agent-sessions";
export const AGENT_DELEGATION_CREATE_SCHEMA_V1 = "semio.hub.auth.agent-delegation-create/v1";
export const AGENT_DELEGATION_RECEIPT_SCHEMA_V1 = "semio.hub.auth.agent-delegation-receipt/v1";
export const AGENT_DELEGATION_LIST_SCHEMA_V1 = "semio.hub.auth.agent-delegation-list/v1";
export const AGENT_CREDENTIAL_SCHEMA_V1 = "semio.hub.agent-credential/v1";

/** 📏️ `AGENT_LABEL_MAX_BYTES`, `MIN/MAX_DELEGATION_TTL_SECS` and `AGENT_DELEGATION_PAGE_MAX` as the
 * hub spells them. A value outside these is `malformed-request` there, so it is refused here. */
export const AGENT_LABEL_MAX_BYTES_V1 = 128;
export const AGENT_DELEGATION_MIN_TTL_SECS_V1 = 60;
export const AGENT_DELEGATION_MAX_TTL_SECS_V1 = 90 * 24 * 60 * 60;
export const AGENT_DELEGATION_PAGE_MAX_V1 = 256;

/** ⏳️ The lifetimes the pane offers: a working day, a week (the hub's own default) and a month. */
export const AGENT_DELEGATION_TTL_CHOICES_SECS_V1 = [86400, 604800, 2592000] as const;

/** 🗑️ Revoking one delegation. The id is a capability-free identifier, so it travels in the path. */
export function agentDelegationRevokePathV1(delegationId: string): string {
  if (delegationId.length === 0 || delegationId.length > 256 || /\p{Cc}/u.test(delegationId)) throw new Error("directory.delegations.invalid-delegation");
  return `${AGENT_DELEGATION_PATH_V1}/${encodeURIComponent(delegationId)}`;
}

/** 📋️ Listing this human's delegations in one space. */
export function agentDelegationListPathV1(spaceId: string): string {
  if (spaceId.length === 0 || spaceId.length > 256 || /\p{Cc}/u.test(spaceId)) throw new Error("directory.delegations.invalid-space");
  return `${AGENT_DELEGATION_PATH_V1}?space=${encodeURIComponent(spaceId)}`;
}
//#endregion 🔖️Routes

//#region 🔖️Scope
/** 🎯️ What an agent may do. A closed two-value enum on both sides: `read` maps to `Spectator` and
 * `edit` to `Author` at the hub, and `SpaceRole` has no administrative variant at all, so "an agent
 * can never administer a space" holds by construction rather than by a check. */
export type AgentAudienceV1 = "read" | "edit";

export const AGENT_AUDIENCES_V1: readonly AgentAudienceV1[] = ["read", "edit"];

export function isAgentAudienceV1(value: unknown): value is AgentAudienceV1 {
  return value === "read" || value === "edit";
}

/** 🤖️ The actor string an agent principal carries, `semio_hub::auth::agent::agent_principal_id`'s
 * twin. Derived from the delegation id alone, so it is stable across the agent's sessions and across
 * hub restarts, and structurally impossible to confuse with a human's `user:<id>#<session>`. */
export function agentPrincipalIdV1(delegationId: string): string {
  return `agent:${delegationId}`;
}

export function isAgentPrincipalV1(actor: string): boolean {
  return actor.startsWith("agent:");
}
//#endregion 🔖️Scope

//#region 🔖️Rows
/** 📋️ One delegation exactly as `GET /auth/agent-delegations` serves it. Never a token, never a
 * selector. `lastUsedAtMs` is when this delegation last minted an agent session — absent means it
 * has never been exchanged. */
export interface AgentDelegationSummaryV1 {
  readonly delegationId: string;
  readonly agentPrincipalId: string;
  readonly agentLabel: string;
  readonly spaceId: string;
  readonly audience: AgentAudienceV1;
  readonly createdAtMs: number;
  readonly expiresAtMs: number;
  readonly revoked: boolean;
  readonly lastUsedAtMs: number | null;
}

/** 🚦️ What a row is right now. Derived from the summary and a clock, never stored: a delegation the
 * hub still reports as live is `expired` the moment its deadline passes, with no round trip. */
export type AgentDelegationStateV1 = "live" | "expired" | "revoked";

export interface AgentDelegationRowV1 extends AgentDelegationSummaryV1 {
  readonly state: AgentDelegationStateV1;
}

export function agentDelegationStateV1(summary: AgentDelegationSummaryV1, nowMs: number): AgentDelegationStateV1 {
  if (summary.revoked) return "revoked";
  return summary.expiresAtMs <= nowMs ? "expired" : "live";
}

/** 📇️ Projects the hub's listing into render rows: live delegations first, then expired, then
 * revoked, each most-recently-created first, ties broken by id. Deterministic and locale-independent
 * so two devices agree on the order. */
export function agentDelegationRowsV1(summaries: readonly AgentDelegationSummaryV1[], nowMs: number): readonly AgentDelegationRowV1[] {
  const order: Readonly<Record<AgentDelegationStateV1, number>> = { live: 0, expired: 1, revoked: 2 };
  return summaries
    .map((summary) => ({ ...summary, state: agentDelegationStateV1(summary, nowMs) }))
    .sort((left, right) => order[left.state] - order[right.state] || right.createdAtMs - left.createdAtMs || (left.delegationId < right.delegationId ? -1 : left.delegationId > right.delegationId ? 1 : 0));
}

/** 🗑️ Whether withdrawing this delegation would still change anything. A revoked one is already
 * withdrawn; an expired one still has a row and a revocation is still the honest verb, because a
 * clock-skewed hub may disagree about the deadline. */
export function agentDelegationRevocableV1(row: AgentDelegationRowV1): boolean {
  return row.state !== "revoked";
}
//#endregion 🔖️Rows

//#region 🔖️Requests
/** 🏗️ Builds the exact `POST /auth/agent-delegations` body, holding it against the hub's own
 * `CreateAgentDelegationRequestV1::verify` bounds so a refusal happens here, with a cause, rather
 * than as an opaque `400`. */
export function createAgentDelegationBodyV1(spaceId: string, agentLabel: string, audience: AgentAudienceV1, ttlSecs: number): string {
  if (spaceId.length === 0 || spaceId.length > 256 || /\p{Cc}/u.test(spaceId)) throw new Error("directory.delegations.invalid-space");
  const label = agentLabel.trim();
  const labelBytes = new TextEncoder().encode(label).byteLength;
  if (labelBytes === 0 || labelBytes > AGENT_LABEL_MAX_BYTES_V1 || /\p{Cc}/u.test(label)) throw new Error("directory.delegations.invalid-label");
  if (!Number.isSafeInteger(ttlSecs) || ttlSecs < AGENT_DELEGATION_MIN_TTL_SECS_V1 || ttlSecs > AGENT_DELEGATION_MAX_TTL_SECS_V1) throw new Error("directory.delegations.invalid-ttl");
  return JSON.stringify({ schema: AGENT_DELEGATION_CREATE_SCHEMA_V1, spaceId, agentLabel: label, audience, ttlSecs });
}

/** 🎁️ The one-time creation receipt. `token` is the plaintext delegation capability, readable
 * exactly once — it is never re-fetchable from any route, so the pane must offer the download before
 * this object is dropped. */
export interface AgentDelegationReceiptV1 {
  readonly delegationId: string;
  readonly agentPrincipalId: string;
  readonly agentLabel: string;
  readonly spaceId: string;
  readonly audience: AgentAudienceV1;
  readonly expiresAtMs: number;
  readonly token: string;
}

/** 🔑️ The exact delegation token shape the hub mints and `semio-os-mcp` accepts:
 * `delegation.v1.<32 lower-hex>.<64 lower-hex>`, 111 bytes. One byte either way is not a token. */
export function isAgentDelegationTokenV1(value: string): boolean {
  return /^delegation\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(value);
}

function boundedText(value: unknown, max: number): string | null {
  return typeof value === "string" && value.length > 0 && value.length <= max && !/\p{Cc}/u.test(value) ? value : null;
}

function finiteMs(value: unknown): number | null {
  return typeof value === "number" && Number.isSafeInteger(value) ? value : null;
}

/** 📥️ Reads one `AgentDelegationReceiptV1`. Every field is bounds-checked rather than trusted, and a
 * body whose `token` is not exactly a delegation capability is refused — a proxy's error page must
 * never be presented to a human as a credential to save. */
export function parseAgentDelegationReceiptV1(source: string): AgentDelegationReceiptV1 {
  let value: unknown;
  try {
    value = JSON.parse(source);
  } catch {
    throw new Error("directory.delegations.malformed-receipt");
  }
  if (value === null || typeof value !== "object") throw new Error("directory.delegations.malformed-receipt");
  const body = value as Record<string, unknown>;
  if (body["schema"] !== AGENT_DELEGATION_RECEIPT_SCHEMA_V1) throw new Error("directory.delegations.malformed-receipt");
  const delegationId = boundedText(body["delegationId"], 256);
  const agentPrincipalId = boundedText(body["agentPrincipalId"], 264);
  const agentLabel = boundedText(body["agentLabel"], AGENT_LABEL_MAX_BYTES_V1);
  const spaceId = boundedText(body["spaceId"], 256);
  const expiresAtMs = finiteMs(body["expiresAtMs"]);
  const token = typeof body["token"] === "string" ? body["token"] : "";
  if (delegationId === null || agentPrincipalId === null || agentLabel === null || spaceId === null || expiresAtMs === null) throw new Error("directory.delegations.malformed-receipt");
  if (!isAgentAudienceV1(body["audience"])) throw new Error("directory.delegations.malformed-receipt");
  if (!isAgentDelegationTokenV1(token)) throw new Error("directory.delegations.malformed-receipt");
  if (agentPrincipalId !== agentPrincipalIdV1(delegationId)) throw new Error("directory.delegations.malformed-receipt");
  return { delegationId, agentPrincipalId, agentLabel, spaceId, audience: body["audience"], expiresAtMs, token };
}

/** 📥️ Reads `GET /auth/agent-delegations`. A malformed row is dropped rather than failing the whole
 * listing: one corrupt delegation must not hide every other one from the human who owns them. */
export function parseAgentDelegationListV1(source: string): readonly AgentDelegationSummaryV1[] {
  let value: unknown;
  try {
    value = JSON.parse(source);
  } catch {
    return [];
  }
  if (value === null || typeof value !== "object") return [];
  const rows = (value as { delegations?: unknown }).delegations;
  if (!Array.isArray(rows)) return [];
  const summaries: AgentDelegationSummaryV1[] = [];
  for (const entry of rows.slice(0, AGENT_DELEGATION_PAGE_MAX_V1)) {
    if (entry === null || typeof entry !== "object") continue;
    const row = entry as Record<string, unknown>;
    const delegationId = boundedText(row["delegationId"], 256);
    const agentLabel = boundedText(row["agentLabel"], AGENT_LABEL_MAX_BYTES_V1);
    const spaceId = boundedText(row["spaceId"], 256);
    const createdAtMs = finiteMs(row["createdAtMs"]);
    const expiresAtMs = finiteMs(row["expiresAtMs"]);
    if (delegationId === null || agentLabel === null || spaceId === null || createdAtMs === null || expiresAtMs === null) continue;
    if (!isAgentAudienceV1(row["audience"])) continue;
    summaries.push({
      delegationId,
      agentPrincipalId: agentPrincipalIdV1(delegationId),
      agentLabel,
      spaceId,
      audience: row["audience"],
      createdAtMs,
      expiresAtMs,
      revoked: row["revoked"] === true,
      lastUsedAtMs: finiteMs(row["lastUsedAtMs"]),
    });
  }
  return summaries;
}
//#endregion 🔖️Requests

//#region 🔖️Credential
/** 📄️ One downloadable credential file: the name to save it under and its exact bytes. */
export interface AgentCredentialFileV1 {
  readonly fileName: string;
  readonly contents: string;
  readonly mediaType: string;
}

/** 📄️ A file name that is safe on every filesystem and still says which agent it belongs to. */
export function agentCredentialFileNameV1(receipt: AgentDelegationReceiptV1): string {
  const slug = receipt.agentLabel.toLowerCase().replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "").slice(0, 48);
  return `semio-agent-${slug.length > 0 ? slug : "delegation"}.json`;
}

/** 📄️ Renders the credential file `semio-os-mcp --credential-file` decodes, in the schema's own key
 * order (`🌉️mcp/🤖️agent-credential/🦀️.rs`). This is the ONLY place the one-time token leaves the
 * receipt, and it leaves as a file the human saves — never into a URL, a log line or the connection
 * book. The file must be saved `0600`: the MCP refuses a group- or world-readable one, which is what
 * the pane tells the human alongside the download. */
export function agentCredentialFileV1(receipt: AgentDelegationReceiptV1, hubOrigin: string): AgentCredentialFileV1 {
  if (hubOrigin.length === 0 || /\p{Cc}/u.test(hubOrigin)) throw new Error("directory.delegations.invalid-origin");
  if (!isAgentDelegationTokenV1(receipt.token)) throw new Error("directory.delegations.invalid-token");
  const contents = `${JSON.stringify({ schema: AGENT_CREDENTIAL_SCHEMA_V1, hubOrigin, spaceId: receipt.spaceId, audience: receipt.audience, token: receipt.token }, null, 2)}\n`;
  return { fileName: agentCredentialFileNameV1(receipt), contents, mediaType: "application/json" };
}

/** 🖥️ The exact command line that makes the saved file an agent principal in this space, shown next
 * to the download so the human has nothing to reconstruct. `<path>` is deliberately a placeholder:
 * this module never learns where the browser put the file. */
export function agentCredentialCommandV1(receipt: AgentDelegationReceiptV1, hubOrigin: string): string {
  return `semio-os-mcp stdio --hub ${hubOrigin} --space ${receipt.spaceId} --credential-file <path>`;
}
//#endregion 🔖️Credential

//#region 🔖️McpClient
/** 🔌️ Schema ids of the host credential install (`📇️directory/🧬️schema/🔣️.json`
 * `AgentCredentialInstallRequestV1`/`AgentCredentialInstallReceiptV1`). */
export const AGENT_CREDENTIAL_INSTALL_SCHEMA_V1 = "semio.os.agent-credential-install/v1";
export const AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1 = "semio.os.agent-credential-install-receipt/v1";

/** 🛰️ The loopback route a development host serves to install one credential file where
 * `semio-os-mcp --credential-file` reads it, and to remove it again once the delegation is withdrawn. */
export const AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1 = "/__semio/agent-credentials";

/** 🎟️ The MCP scopes a client configured for one delegation is granted: exactly what the hub
 * audience admits and nothing a `read` agent could not use anyway. */
export const AGENT_MCP_SCOPES_BY_AUDIENCE_V1: Readonly<Record<AgentAudienceV1, string>> = {
  read: "workspace.read",
  edit: "workspace.read,artifact.write,inference.execute",
};

/** 🚀️ How this host starts `semio-os-mcp`: the executable plus every argument up to and including
 * the transport, e.g. `bun <repo>/📜️script.ts dev mcp stdio os` or `<dir>/semio-os-mcp stdio`. */
export interface AgentMcpLauncherV1 {
  readonly command: string;
  readonly args: readonly string[];
}

/** 📥️ What a host installs: the delegation it belongs to and the exact credential file bytes. */
export interface AgentCredentialInstallRequestV1 {
  readonly schema: typeof AGENT_CREDENTIAL_INSTALL_SCHEMA_V1;
  readonly delegationId: string;
  readonly contents: string;
}

/** 🧾️ Where the host put the credential (owner-only) and how it starts the MCP gateway. */
export interface AgentCredentialInstallReceiptV1 {
  readonly schema: typeof AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1;
  readonly credentialPath: string;
  readonly launcher: AgentMcpLauncherV1;
}

/** 🧩️ One stdio server entry in the shape MCP clients read from `.mcp.json` /
 * `claude_desktop_config.json`. */
export interface AgentMcpServerEntryV1 {
  readonly type: "stdio";
  readonly command: string;
  readonly args: readonly string[];
}

/** 📄️ The client configuration one delegation produces: exactly one named server. */
export interface AgentMcpClientConfigV1 {
  readonly mcpServers: Readonly<Record<string, AgentMcpServerEntryV1>>;
}

/** 🏷️ The installed credential's file name: derived from the delegation id, so two delegations with
 * the same label never overwrite each other's still-live credential. */
export function agentCredentialInstallFileNameV1(delegationId: string): string {
  if (!/^[A-Za-z0-9_-]{1,128}$/u.test(delegationId)) throw new Error("directory.delegations.invalid-delegation");
  return `semio-agent-${delegationId}.json`;
}

/** 📥️ Seals the install request for one freshly minted delegation. */
export function agentCredentialInstallRequestV1(receipt: AgentDelegationReceiptV1, hubOrigin: string): AgentCredentialInstallRequestV1 {
  agentCredentialInstallFileNameV1(receipt.delegationId);
  return { schema: AGENT_CREDENTIAL_INSTALL_SCHEMA_V1, delegationId: receipt.delegationId, contents: agentCredentialFileV1(receipt, hubOrigin).contents };
}

/** 🚫️ The host serves no credential install (a plain browser, a static deployment): an ordinary
 * state the pane answers with the download and the command, never a failure. */
export class AgentCredentialInstallUnavailableV1 extends Error {
  constructor() {
    super("directory.delegations.install-unavailable");
    this.name = "AgentCredentialInstallUnavailableV1";
  }
}

/** 🧾️ Parses a host's install receipt, refusing anything that is not an absolute credential path and
 * a non-empty launcher. */
export function parseAgentCredentialInstallReceiptV1(body: string): AgentCredentialInstallReceiptV1 {
  const value = JSON.parse(body) as { schema?: unknown; credentialPath?: unknown; launcher?: { command?: unknown; args?: unknown } };
  const args = value.launcher?.args;
  const absolute = typeof value.credentialPath === "string" && (value.credentialPath.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(value.credentialPath));
  if (value.schema !== AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1 || !absolute || typeof value.launcher?.command !== "string" || value.launcher.command.length === 0 || !Array.isArray(args) || !args.every((arg) => typeof arg === "string")) {
    throw new Error("directory.delegations.invalid-install-receipt");
  }
  return { schema: AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1, credentialPath: value.credentialPath as string, launcher: { command: value.launcher.command, args: args as string[] } };
}

/** 🏷️ The server name a client lists this agent under: `semio-` plus the agent label's slug, so a
 * human reading their client's server list sees which delegation each entry is. */
export function agentMcpServerNameV1(receipt: AgentDelegationReceiptV1): string {
  const slug = receipt.agentLabel.toLowerCase().replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "").slice(0, 48);
  return `semio-${slug.length > 0 ? slug : "agent"}`;
}

/** 📄️ The complete MCP client configuration for one delegation: the host's launcher, bound to the
 * hub and space the delegation names, authenticated by the installed credential file, with the
 * scopes its audience admits. The token itself never appears — only the path of the owner-only file
 * that holds it. Law: `🤖️delegations/🧫️fixtures/🔌️mcp-client-config.json`. */
export function agentMcpClientConfigV1(receipt: AgentDelegationReceiptV1, hubOrigin: string, installed: AgentCredentialInstallReceiptV1): AgentMcpClientConfigV1 {
  if (hubOrigin.length === 0 || /\p{Cc}/u.test(hubOrigin)) throw new Error("directory.delegations.invalid-origin");
  return {
    mcpServers: {
      [agentMcpServerNameV1(receipt)]: {
        type: "stdio",
        command: installed.launcher.command,
        args: [...installed.launcher.args, "--hub", hubOrigin, "--space", receipt.spaceId, "--credential-file", installed.credentialPath, "--scopes", AGENT_MCP_SCOPES_BY_AUDIENCE_V1[receipt.audience]],
      },
    },
  };
}

/** 🧾️ The configuration as the text a human pastes into their client's configuration file. */
export function agentMcpClientConfigJsonV1(config: AgentMcpClientConfigV1): string {
  return `${JSON.stringify(config, null, 2)}\n`;
}
//#endregion 🔖️McpClient

//#region 🔖️Errors
/** 🚫️ Closed delegation failure classes, so the pane names a cause instead of a status. Mirrors
 * `AgentErrorCodeV1` plus the two the browser itself can observe. */
export type AgentDelegationErrorCodeV1 = "malformed-request" | "invalid-delegation" | "delegation-revoked" | "delegation-expired" | "forbidden" | "rate-limited" | "directory-unavailable" | "unreachable" | "cancelled";

/** 🌐️ Maps one response status to its closed code. The hub serves exactly one status per code
 * (`AgentErrorCodeV1::status`), so this inverse is total. */
export function agentDelegationErrorFromStatusV1(status: number): AgentDelegationErrorCodeV1 {
  if (status === 400) return "malformed-request";
  if (status === 401) return "invalid-delegation";
  if (status === 403) return "forbidden";
  if (status === 429) return "rate-limited";
  if (status === 503) return "directory-unavailable";
  return "unreachable";
}

/** 🚫️ Reads the hub's own `semio.hub.auth.agent-error/v1` body when it sent one, falling back to the
 * status. A `403` is `forbidden` unless the hub said it was a revocation or an expiry — the three
 * share a status precisely so an id cannot be probed, and only a caller that proved the secret is
 * told which. */
export function agentDelegationErrorFromResponseV1(status: number, body: string): AgentDelegationErrorCodeV1 {
  let value: unknown;
  try {
    value = JSON.parse(body);
  } catch {
    return agentDelegationErrorFromStatusV1(status);
  }
  if (value === null || typeof value !== "object") return agentDelegationErrorFromStatusV1(status);
  const named = (value as { schema?: unknown; error?: unknown });
  if (named.schema !== "semio.hub.auth.agent-error/v1") return agentDelegationErrorFromStatusV1(status);
  const codes: readonly AgentDelegationErrorCodeV1[] = ["malformed-request", "invalid-delegation", "delegation-revoked", "delegation-expired", "forbidden", "rate-limited", "directory-unavailable"];
  const code = codes.find((candidate) => candidate === named.error);
  return code ?? agentDelegationErrorFromStatusV1(status);
}
//#endregion 🔖️Errors

//#region 🔖️Phase
/** 🔄️ The delegation pane's own load/mutate phase, the spaces surface's `SpaceBrowserPhaseV1` twin. */
export type AgentDelegationPhaseV1 = "idle" | "loading" | "ready" | "submitting" | "failed";
//#endregion 🔖️Phase

//#region 🔖️Attribution
/** 👥️ One actor as a collaborator should read it. The identity stays whatever the hub minted — an
 * opaque `hub.v1.<sha256>` socket actor, distinct per session, which is what keeps per-actor undo
 * separating an agent's edits from the delegating human's — and only the *rendering* changes: an
 * agent is `agent:<its delegation label>`, a person is their display name, and the opaque id is
 * shown only when there is no name at all, because an unlabelled actor is still better named by its
 * id than by a guess. */
export function actorDisplayV1(actor: string, label: string | null | undefined, isAgent: boolean): string {
  const named = typeof label === "string" && label.trim().length > 0 ? label.trim() : null;
  if (isAgent) return named === null ? agentPrincipalIdV1(actor) : `agent:${named}`;
  return named ?? actor;
}

/** 👥️ Resolves an actor id against a roster, for a surface (history, attribution, a conflict notice)
 * that has an actor string and no peer. An actor nobody is currently holding is returned unchanged —
 * inventing a name for a departed collaborator would be worse than the id. */
export function actorDisplayFromRosterV1(actor: string, roster: readonly Readonly<{ actor: string; label?: string | null; isAgent?: boolean }>[]): string {
  const peer = roster.find((entry) => entry.actor === actor);
  return peer === undefined ? actor : actorDisplayV1(actor, peer.label, peer.isAgent === true);
}
//#endregion 🔖️Attribution
