/** 📞️ The directory client: stdio format kinds, hub reconnect policy, document link shortage, echo suppression, transient apply refusal, directory transport types and parsers, socket grants and the directory client.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | os root}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { parseDirectorySessionAuthorityJsonV1 } from "../📇️directory/🚪️io/📝️text/🪪️session-authority-v1/🟦️.ts";
import { directoryCommandRequestJson, parseDirectoryCommandReceiptV1, parseDirectoryEventPageV1 } from "../📇️directory/🚪️io/📝️text/🟦️.ts";
import { parseDirectorySpaceAdministrationPageV1 } from "../📇️directory/🚪️io/📝️text/🟦️.ts";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import { fetchWithTimeout, retryWithJitteredBackoff } from "@semio-tech/framework";
import type { DirectoryEvent, DirectoryStreamMessage } from "../📇️directory/🟦️.ts";
import { type DirectorySessionAuthorityV1 } from "../📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts";
import { closeHubSocketV1 } from "../📇️directory/🔌️client/🚪️socket-close/🟦️.ts";
import { DirectoryHttpError } from "../💡️inference/🔌️service/🛂️worker-host/🟦️.ts";
import type { DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEventPageV1 } from "../📇️directory/🧬️schema/🟦️.ts";
import {
  DIRECTORY_COMMAND_RECEIPT_MAX_BYTES,
  DIRECTORY_EVENT_PAGE_MAX_BYTES,
  DIRECTORY_PREFERENCE_PAGE_PATH_V1,
  DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES,
  directoryCommandErrorFromStatus
} from "../📇️directory/🧬️schema/🟦️.ts";
import { encodeClientFrame } from "@semio-tech/framework-replication";
import type { DirectorySpaceAdministrationPageV1, DirectorySpaceListEntryV1, DocumentScope } from "../📇️directory/🟦️.ts";
import { DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES } from "../📇️directory/🟦️.ts";

//#region StdioFormatKinds
/** 🗄️ Normalize `stdio.dwg` / `dwg` to short stdio format kind id. */
export function normalizeStdioFormatKind(value: string): string {
  const trimmed = value.trim();
  const short = trimmed.startsWith("stdio.") ? trimmed.slice("stdio.".length) : trimmed;
  switch (short) {
    case "jpeg":
      return "jpg";
    case "tif":
      return "tiff";
    case "stp":
      return "step";
    case "markdown":
      return "md";
    default:
      return short;
  }
}

/** 🗂️ File-picker accept filter from stdio format kind ids. */
export function mediaAcceptFilterKinds(formatArtifactKinds: readonly string[]): string {
  return formatArtifactKinds
    .map((kind) => normalizeStdioFormatKind(kind))
    .filter(Boolean)
    .map((kind) => `.${kind}`)
    .join(",");
}
//#endregion StdioFormatKinds

//#region 🔖️Directory
//#region 🔖️HubBinding
/** 🏛️ One administration page plus the exact response bytes its receipt covers. */
export interface CanonicalDirectorySpaceAdministrationPageV1 {
  readonly canonicalJson: string;
  readonly page: DirectorySpaceAdministrationPageV1;
}

/** 🔁️ Reconnect backoff shared by every hub transport this package opens — `connectHub` in
 * `🏪️store/👷️worker/🟦️.ts` (artifact sync) and {@link DirectoryClient.stream} both import these
 * (single source of truth; the two used to carry independent copies of the same two numbers). */
export const HUB_RECONNECT_MIN_MS = 500;
export const HUB_RECONNECT_MAX_MS = 30_000;

//#region 🔌️DocumentLinkShortage
/** 🔌️ The capped doubling reconnect and the longest shortage one hub document's link rides out — the TS twin of the
 * kernel's `DOCUMENT_LINK_SHORTAGE_POLICY` (`🏪️store/🔄️sync/🦀️.rs`). The bound is twice the backoff cap.
 * @see ./🔨️modules/🏪️store/🔄️sync/🧬️schema/document-link-shortage/🔣️.json */
export const DOCUMENT_LINK_SHORTAGE_POLICY = Object.freeze({ reconnectMinMs: HUB_RECONNECT_MIN_MS, reconnectMaxMs: HUB_RECONNECT_MAX_MS, shortageBoundMs: 2 * HUB_RECONNECT_MAX_MS });
/** 🚫️ HTTP statuses of a document admission that mean the hub withdrew access: the link is revoked, never retried. */
export const DOCUMENT_LINK_ACCESS_REFUSED_STATUSES: ReadonlySet<number> = new Set([401, 403, 404, 410]);
export type DocumentLinkShortagePolicy = Readonly<{ reconnectMinMs: number; reconnectMaxMs: number; shortageBoundMs: number }>;

/** 🔌️ A hub document's link — one state machine for every shell: while `unlinked` the document stays mounted, local
 * edits apply and queue and the link retries at `retryAtMs`; past the bound it is `expired` (long offline periods are
 * refused), and a hub that withdraws access `revoked` it. Terminal states admit no local edit and never relink. */
export type DocumentLink =
  | Readonly<{ kind: "linked" }>
  | Readonly<{ kind: "unlinked"; sinceMs: number; backoffMs: number; retryAtMs: number }>
  | Readonly<{ kind: "expired"; sinceMs: number; atMs: number }>
  | Readonly<{ kind: "revoked"; atMs: number }>;
export type DocumentLinkEvent = Readonly<{ kind: "failed" | "restored" | "tick" | "refused"; nowMs: number }>;
export type DocumentLinkStatusCode = "linked" | "reconnecting" | "link-expired" | "access-revoked";

/** 🌐️ The localized line of each shortage status, EN and DE explicit (the fixture's `texts`). */
export const DOCUMENT_LINK_STATUS_TEXT: Readonly<Record<Exclude<DocumentLinkStatusCode, "linked">, Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  reconnecting: Object.freeze({ en: "Connection lost. Your edits are kept and sent when the hub is reachable again.", de: "Verbindung unterbrochen. Ihre Änderungen bleiben erhalten und werden gesendet, sobald der Hub wieder erreichbar ist." }),
  "link-expired": Object.freeze({ en: "The connection was lost for too long. Reconnect to keep editing this document.", de: "Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten." }),
  "access-revoked": Object.freeze({ en: "Your access to this document was removed.", de: "Ihr Zugriff auf dieses Dokument wurde entfernt." }),
});

/** 🌱️ A link opened at `nowMs`: not yet live, first attempt due at once, the shortage bound running. */
export function documentLinkOpened(nowMs: number): DocumentLink {
  return { kind: "unlinked", sinceMs: nowMs, backoffMs: 0, retryAtMs: nowMs };
}

/** ⚙️ The one transition function — twin of the kernel's `DocumentLink::apply`. */
export function documentLinkTransition(link: DocumentLink, event: DocumentLinkEvent, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): DocumentLink {
  if (link.kind === "expired" || link.kind === "revoked") return link;
  if (event.kind === "refused") return { kind: "revoked", atMs: event.nowMs };
  if (event.kind === "restored") return { kind: "linked" };
  if (event.kind === "failed") {
    if (link.kind === "linked") return { kind: "unlinked", sinceMs: event.nowMs, backoffMs: policy.reconnectMinMs, retryAtMs: documentLinkRetryAtMs(event.nowMs, event.nowMs, policy.reconnectMinMs, policy) };
    if (event.nowMs - link.sinceMs >= policy.shortageBoundMs) return { kind: "expired", sinceMs: link.sinceMs, atMs: event.nowMs };
    const backoffMs = Math.min(Math.max(link.backoffMs * 2, policy.reconnectMinMs), policy.reconnectMaxMs);
    return { kind: "unlinked", sinceMs: link.sinceMs, backoffMs, retryAtMs: documentLinkRetryAtMs(link.sinceMs, event.nowMs, backoffMs, policy) };
  }
  return link.kind === "unlinked" && event.nowMs >= documentLinkCeilingAtMs(link.sinceMs, policy) ? { kind: "expired", sinceMs: link.sinceMs, atMs: event.nowMs } : link;
}

/** 🔁️ When the next attempt of a shortage that began at `sinceMs` is due: one backoff after `nowMs`, never past the bound — twin of
 * the kernel's `DocumentLinkShortagePolicy::retry_at`. */
export function documentLinkRetryAtMs(sinceMs: number, nowMs: number, backoffMs: number, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number {
  return Math.min(nowMs + backoffMs, sinceMs + policy.shortageBoundMs);
}

/** ⏳️ The instant a shortage that began at `sinceMs` ends although no attempt answered: the bound plus one capped backoff — twin of
 * the kernel's `DocumentLinkShortagePolicy::ceiling_at`. */
export function documentLinkCeilingAtMs(sinceMs: number, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number {
  return sinceMs + policy.shortageBoundMs + policy.reconnectMaxMs;
}

/** 🚦️ The status code a shell shows for a link. */
export function documentLinkStatus(link: DocumentLink): DocumentLinkStatusCode {
  return link.kind === "linked" ? "linked" : link.kind === "unlinked" ? "reconnecting" : link.kind === "expired" ? "link-expired" : "access-revoked";
}

/** ✍️ Whether a local edit applies (and queues while unlinked); a terminal link admits none. */
export function documentLinkAdmitsLocalEdits(link: DocumentLink): boolean {
  return link.kind === "linked" || link.kind === "unlinked";
}

/** ⏳️ The latest instant an unlinked link can live: its ceiling. A failed attempt at the bound ends it earlier. */
export function documentLinkExpiresAtMs(link: DocumentLink, policy: DocumentLinkShortagePolicy = DOCUMENT_LINK_SHORTAGE_POLICY): number | undefined {
  return link.kind === "unlinked" ? documentLinkCeilingAtMs(link.sinceMs, policy) : undefined;
}
//#endregion 🔌️DocumentLinkShortage

//#region 🔁️DocumentEchoSuppression
/** 🔁️ The envelopes of one server `Commands` frame a replica applies — the TS twin of the kernel's `admit_remote_envelopes`. Echo
 * suppression is by operation identity, never by frame origin: an envelope is applied unless its id is one this replica authored or
 * already applied, and every admitted id is recorded. A frame is never discarded whole — the hub's hello catch-up tail carries anyone's
 * edits, the joiner's own earlier device included (ticket 26/09/23 session 12, run s12i: late joiners saw no history).
 * @see ./🔨️modules/🏪️store/🔄️sync/🧬️schema/document-echo-suppression/🔣️.json */
export function admitRemoteEnvelopes<T>(known: Set<string>, envelopes: readonly T[], idOf: (envelope: T) => string): T[] {
  return envelopes.filter((envelope) => {
    const id = idOf(envelope);
    if (!id || known.has(id)) return false;
    known.add(id);
    return true;
  });
}

/** 🔁️ Records the operations this replica authored, so their echo is never applied a second time. */
export function noteAuthoredEnvelopeIds(known: Set<string>, ids: Iterable<string>): void {
  for (const id of ids) if (id) known.add(id);
}
//#endregion 🔁️DocumentEchoSuppression

//#region ⏳️TransientApplyRefusal
/** ⏳️ The `code` of the one `MutationMessage` a hub answers in `ApplyOutcome::Rejected.messages` when it refused a document batch for a
 * TRANSIENT reason (the db could not admit it now, the sender's command rate is paced out): nothing was applied, so the client keeps the
 * batch and resends it — the TS twin of the hub's `HUB_TRANSIENT_APPLY_REFUSAL_CODE` (`🌎️hub/🚧️refusal`).
 * @see ../../🔨️modules/📡️replication/🚧️apply-refusal/🧬️schema/🔣️.json */
export const HUB_TRANSIENT_APPLY_REFUSAL_CODE = "hub.unavailable";

/** ⏳️ Whether a refusal's `messages` are exactly the hub's transient refusal (`HubTransientApplyRefusalMessageV1`): one message, level
 * `warning`, the declared code, a non-empty reason and no other field. Every other refusal is permanent: the batch is rolled back. */
export function hubTransientApplyRefusalV1(messages: ArrayLike<number>): boolean {
  if (messages.length === 0) return false;
  let parsed: unknown;
  try {
    parsed = JSON.parse(new TextDecoder().decode(Uint8Array.from(messages)));
  } catch {
    return false;
  }
  if (!Array.isArray(parsed) || parsed.length !== 1 || typeof parsed[0] !== "object" || parsed[0] === null) return false;
  const message = parsed[0] as Record<string, unknown>;
  return Object.keys(message).sort().join(",") === "code,level,message" && message.level === "warning" && message.code === HUB_TRANSIENT_APPLY_REFUSAL_CODE && typeof message.message === "string" && message.message.length > 0;
}
//#endregion ⏳️TransientApplyRefusal

// 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet `web-directory`, coordinator
// follow-up on finding 2 — CLAUDE.md "support short connection-shortages [...] not freeze the app"
// means a session that has been healthy for a while must not inherit an escalated backoff from
// earlier, unrelated blips; see {@link DirectoryClient.stream}'s docstring for the mechanism.
/** 🩺️ How long {@link DirectoryClient.stream}'s socket must stay open before a drop is treated as
 * "this connection was genuinely healthy" and resets the reconnect backoff toward
 * {@link HUB_RECONNECT_MIN_MS}. Deliberately set equal to {@link HUB_RECONNECT_MAX_MS}: surviving
 * open for at least one full worst-case backoff cycle is comfortably longer than any legitimate
 * reconnect delay this client would ever impose, so a genuinely flapping server — one that accepts
 * and then immediately drops, on a cycle time far shorter than this — can never cross the threshold
 * by accident. The reset is therefore only ever reachable by a connection that was actually stable,
 * never as a side effect of the accept-then-immediately-drop failure mode the backoff exists to
 * guard against. */
export const HUB_HEALTHY_RESET_MS = HUB_RECONNECT_MAX_MS;

/** 🔌️ A live {@link DirectoryClient.stream} subscription handle. */
export type DirectoryStream = { readonly close: () => void };

/** 📌️ A global stream whose reconnect cursor advances only after a Home page acknowledgement. */
export type DirectoryAcknowledgedStream = DirectoryStream & { readonly acknowledge: (through: number) => void };

/** 🚨️ One closed command-transport denial. It never carries a raw response body or server text. */
export class DirectoryCommandError extends Error {
  readonly code: DirectoryCommandErrorCodeV1;

  constructor(code: DirectoryCommandErrorCodeV1) {
    super(`directory command: ${code}`);
    this.name = "DirectoryCommandError";
    this.code = code;
  }
}

/** 📄️ Exact canonical page bytes plus the bounded header a shell needs for ACK ordering. */
export type CanonicalDirectoryEventPageV1 = Readonly<{
  canonicalJson: string;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  afterSeqExclusive: number;
  throughSeqInclusive: number;
  hasMore: boolean;
  receiptSha256: string;
}>;

/** 🎚️ One page of the preference lane: the canonical page fields plus the principal's own `user.preference-recorded`
 * events it carries, already validated by the page parser. */
export type DirectoryPreferencePageV1 = Readonly<{
  sessionBindingSha256: string;
  authorizationGeneration: number;
  afterSeqExclusive: number;
  throughSeqInclusive: number;
  hasMore: boolean;
  events: readonly Readonly<{ seq: number; schema: string; mutation: string }>[];
}>;

/** ✅️ Exact retained-Home acknowledgement for one still-owned directory page. */
export type DirectoryEventPageAckV1 = Readonly<{
  bootstrapEpoch: number;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  throughSeqInclusive: number;
  receiptSha256: string;
}>;

/** 🚨️ Thrown by every {@link DirectoryClient} REST method on a non-2xx response; the class is owned by the framework worker-host port, which installed owner services share. */
export { DirectoryHttpError };

// 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet `web-directory`, finding 1 —
// every REST call is bounded by a timeout and cancellable by a caller `signal`, so a hung directory
// server degrades ("hub unreachable, staying offline" — the ShellHost boot effect's existing catch)
// instead of hanging the identity/boot path forever awaiting a response that never arrives.
/** ⏱️ Per-request timeout for every {@link DirectoryClient} REST call — generous for a real request,
 * short enough that a hung server still lets the boot path's existing offline fallback run instead of
 * hanging indefinitely. */
export const DIRECTORY_HTTP_TIMEOUT_MS = 10_000;

/** 🎛️ Per-call options every {@link DirectoryClient} method accepts — just a caller-cancellable
 * `signal`; the timeout itself is fixed ({@link DIRECTORY_HTTP_TIMEOUT_MS}) and not caller-tunable. */
export interface DirectoryRequestOptions {
  readonly signal?: AbortSignal;
}

/** 🎫 One-use, non-persistable authority for exactly one directory socket upgrade. */
export type SocketGrantReceiptV1 = Readonly<{
  schema: "semio.hub.socket-grant/v1";
  protocol: "semio.socket.v1";
  grant: string;
  actorId: string;
  expiresAtMs: number;
}>;

/** 📝️ The answer to a document open-plan exchange (`os.directory#/$defs/DocumentSocketGrantReceiptV1`):
 * the actor the next `semio.session.v1` upgrade of the same credential is admitted as, and until when.
 * It carries no secret — the credential itself is what that upgrade presents. */
export type DocumentSocketGrantReceiptV1 = Readonly<{
  schema: "semio.hub.document-socket-grant/v1";
  protocol: "semio.session.v1";
  actorId: string;
  expiresAtMs: number;
}>;

/** 🎫️ The exact session capability `POST /auth/sessions` mints
 * (`📇️directory/🔐️sign-in/🟦️.ts`'s `HUB_SESSION_TOKEN`). Pinned here too, so a proxy's error page can
 * never be installed into the credential-owning worker as a session. */
export const HUB_SESSION_CAPABILITY_PATTERN_V1 = /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u;

export type HubSessionPortRequestV1 =
  | Readonly<{ kind: "initialize"; capability: string }>
  | Readonly<{ kind: "request"; requestId: string; operation: "me" }>
  | Readonly<{ kind: "cancel"; requestId: string }>
  | Readonly<{ kind: "close" }>;

export type HubSessionPortResponseV1 =
  | Readonly<{ kind: "initialized"; ok: boolean }>
  | Readonly<{ kind: "response"; requestId: string; status: number; body: string }>;

function exactRecordKeys(value: Readonly<Record<string, unknown>>, expected: readonly string[]): boolean {
  const keys = Object.keys(value).sort();
  return keys.length === expected.length && keys.every((key, index) => key === [...expected].sort()[index]);
}

function directoryProjectionRecord(value: unknown, keys: readonly string[]): Readonly<Record<string, unknown>> | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Readonly<Record<string, unknown>>;
  return exactRecordKeys(record, keys) ? record : undefined;
}

function directoryProjectionInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

function directoryProjectionSignedInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value);
}

function directoryProjectionText(value: unknown): value is string {
  return typeof value === "string";
}

function directoryProjectionSpaceBase(record: Readonly<Record<string, unknown>>): boolean {
  return directoryProjectionText(record.id)
    && directoryProjectionText(record.name)
    && (record.kind === "atelier" || record.kind === "studio" || record.kind === "archive")
    && (record.visibility === "private" || record.visibility === "public")
    && directoryProjectionInteger(record.memberCount)
    && directoryProjectionInteger(record.documentCount)
    && directoryProjectionSignedInteger(record.createdAtMs)
    && directoryProjectionSignedInteger(record.updatedAtMs);
}

function directoryPublicSpace(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["createdAtMs", "documentCount", "id", "kind", "memberCount", "name", "updatedAtMs", "visibility"]);
  return record !== undefined && directoryProjectionSpaceBase(record) && record.visibility === "public";
}

function directoryMemberSpace(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["activeConnections", "createdAtMs", "documentCount", "id", "kind", "memberCount", "name", "ownerUserId", "role", "updatedAtMs", "visibility"]);
  return record !== undefined
    && directoryProjectionSpaceBase(record)
    && directoryProjectionText(record.ownerUserId)
    && (record.role === "author" || record.role === "spectator")
    && directoryProjectionInteger(record.activeConnections);
}

function directoryMemberSpaceRole(value: unknown, role: "author" | "spectator"): boolean {
  return directoryMemberSpace(value) && (value as Readonly<Record<string, unknown>>).role === role;
}

function directoryOwner(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["packageHash", "packageId", "pluginId", "version"]);
  return record !== undefined
    && directoryProjectionText(record.pluginId)
    && directoryProjectionText(record.packageId)
    && directoryProjectionText(record.version)
    && typeof record.packageHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.packageHash);
}

function directoryPublicDocument(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["artifactKind", "artifactSchema", "documentId", "owner", "packSchemaHash"]);
  return record !== undefined
    && directoryProjectionText(record.documentId)
    && directoryProjectionText(record.artifactKind)
    && directoryProjectionText(record.artifactSchema)
    && directoryOwner(record.owner)
    && typeof record.packSchemaHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.packSchemaHash);
}

function directoryMember(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["displayName", "email", "role", "userId"]);
  return record !== undefined
    && directoryProjectionText(record.userId)
    && directoryProjectionText(record.email)
    && directoryProjectionText(record.displayName)
    && (record.role === "author" || record.role === "spectator");
}

function directoryFrontier(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["commitSeq", "epoch", "headSeq"]);
  return record !== undefined && directoryProjectionInteger(record.headSeq) && directoryProjectionInteger(record.commitSeq) && directoryProjectionInteger(record.epoch);
}

function directoryDescriptor(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["artifactKind", "artifactSchema", "bootstrapFrontier", "bootstrapSnapshotHash", "bootstrapVersion", "documentId", "owner", "packSchemaHash", "spaceId"]);
  return record !== undefined
    && directoryProjectionText(record.spaceId)
    && directoryProjectionText(record.documentId)
    && directoryProjectionText(record.artifactKind)
    && directoryProjectionText(record.artifactSchema)
    && directoryOwner(record.owner)
    && typeof record.packSchemaHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.packSchemaHash)
    && directoryProjectionInteger(record.bootstrapVersion)
    && directoryFrontier(record.bootstrapFrontier)
    && typeof record.bootstrapSnapshotHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.bootstrapSnapshotHash);
}

function directoryDocument(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["commitSeq", "descriptor", "epoch", "headSeq"]);
  return record !== undefined && directoryDescriptor(record.descriptor) && directoryProjectionInteger(record.headSeq) && directoryProjectionInteger(record.commitSeq) && directoryProjectionInteger(record.epoch);
}

/** 🛡️ Strictly decodes one discriminated list projection. */
export function parseDirectorySpaceListEntryV1(value: unknown): DirectorySpaceListEntryV1 {
  const record = directoryProjectionRecord(value, ["access", "space"]);
  if (record?.access === "public" && directoryPublicSpace(record.space)) return record as DirectorySpaceListEntryV1;
  if (record?.access === "member" && directoryMemberSpaceRole(record.space, "spectator")) return record as DirectorySpaceListEntryV1;
  if (record?.access === "author" && directoryMemberSpaceRole(record.space, "author")) return record as DirectorySpaceListEntryV1;
  throw new Error("directory: invalid space list projection");
}

export function parseHubSessionPortRequestV1(value: unknown): HubSessionPortRequestV1 | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Readonly<Record<string, unknown>>;
  if (record.kind === "initialize" && exactRecordKeys(record, ["kind", "capability"]) && typeof record.capability === "string" && HUB_SESSION_CAPABILITY_PATTERN_V1.test(record.capability)) return record as HubSessionPortRequestV1;
  if (record.kind === "request" && exactRecordKeys(record, ["kind", "operation", "requestId"]) && record.operation === "me" && typeof record.requestId === "string" && /^[0-9a-f-]{36}$/u.test(record.requestId)) return record as HubSessionPortRequestV1;
  if (record.kind === "cancel" && exactRecordKeys(record, ["kind", "requestId"]) && typeof record.requestId === "string" && /^[0-9a-f-]{36}$/u.test(record.requestId)) return record as HubSessionPortRequestV1;
  if (record.kind === "close" && exactRecordKeys(record, ["kind"])) return record as HubSessionPortRequestV1;
  return undefined;
}

export function parseHubSessionPortResponseV1(value: unknown): HubSessionPortResponseV1 | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Readonly<Record<string, unknown>>;
  if (record.kind === "initialized" && exactRecordKeys(record, ["kind", "ok"]) && typeof record.ok === "boolean") return record as HubSessionPortResponseV1;
  if (record.kind !== "response" || !exactRecordKeys(record, ["body", "kind", "requestId", "status"]) || typeof record.requestId !== "string" || !/^[0-9a-f-]{36}$/u.test(record.requestId) || typeof record.status !== "number" || !Number.isSafeInteger(record.status) || record.status < 100 || record.status > 599 || typeof record.body !== "string" || new TextEncoder().encode(record.body).byteLength > 1024 * 1024) return undefined;
  return record as HubSessionPortResponseV1;
}

/** 🔐 Narrow authority boundary that alone may retain an upstream session/share credential. */
export interface SocketGrantIssuerV1 {
  issueDirectory(options?: DirectoryRequestOptions): Promise<SocketGrantReceiptV1>;
  issueDirectoryScoped(scope: DocumentScope, options?: DirectoryRequestOptions): Promise<SocketGrantReceiptV1>;
}

/** 🌉 Credential-owning request port. Browser implementations target the local BFF; native
 * implementations inject the inherited-envelope bearer without exposing it to persisted bindings. */
export interface SocketGrantRequestPortV1 {
  post(path: string, options?: DirectoryRequestOptions): Promise<unknown>;
}

/** 🏗️ Builds the only socket issuer from a credential-owning request port. */
export function createSocketGrantIssuerV1(port: SocketGrantRequestPortV1): SocketGrantIssuerV1 {
  return {
    issueDirectory: async (options) => parseSocketGrantReceiptV1(await port.post("/directory/socket-grants", options)),
    issueDirectoryScoped: async (scope, options) => parseSocketGrantReceiptV1(await port.post(`/directory/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}/socket-grants`, options)),
  };
}

const SOCKET_GRANT_PATTERN = /^socket\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/;
const SOCKET_ACTOR_PATTERN = /^hub\.v1\.[0-9a-f]{64}$/;

/** 🛡️ Validates the schema-first receipt before any grant reaches a WebSocket API. */
export function parseSocketGrantReceiptV1(value: unknown): SocketGrantReceiptV1 {
  if (typeof value !== "object" || value === null) throw new Error("socket grant: invalid receipt");
  const receipt = value as Record<string, unknown>;
  if (
    receipt.schema !== "semio.hub.socket-grant/v1" ||
    receipt.protocol !== "semio.socket.v1" ||
    typeof receipt.grant !== "string" ||
    receipt.grant.length !== 107 ||
    !SOCKET_GRANT_PATTERN.test(receipt.grant) ||
    typeof receipt.actorId !== "string" ||
    !SOCKET_ACTOR_PATTERN.test(receipt.actorId) ||
    !Number.isSafeInteger(receipt.expiresAtMs)
  ) throw new Error("socket grant: invalid receipt");
  return receipt as SocketGrantReceiptV1;
}

/** 🛡️ Validates a document socket grant: exactly four fields, no capability. */
export function parseDocumentSocketGrantReceiptV1(value: unknown): DocumentSocketGrantReceiptV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("document socket grant: invalid receipt");
  const receipt = value as Record<string, unknown>;
  if (
    Object.keys(receipt).sort().join(",") !== "actorId,expiresAtMs,protocol,schema" ||
    receipt.schema !== "semio.hub.document-socket-grant/v1" ||
    receipt.protocol !== "semio.session.v1" ||
    typeof receipt.actorId !== "string" ||
    !SOCKET_ACTOR_PATTERN.test(receipt.actorId) ||
    !Number.isSafeInteger(receipt.expiresAtMs) ||
    (receipt.expiresAtMs as number) < 1
  ) throw new Error("document socket grant: invalid receipt");
  return receipt as DocumentSocketGrantReceiptV1;
}

/** 📡 Exact ordered subprotocol offer required by the v1 upgrade boundary. */
export function socketGrantProtocolsV1(receipt: SocketGrantReceiptV1): readonly ["semio.socket.v1", string] {
  return [receipt.protocol, receipt.grant];
}

/**
 * 📡️ Typed facade over the directory hub's REST/WS surface (contract-freeze §C2). Constructed per
 * identity (`baseUrl` + optional bearer `token`, mutated in place by {@link mintSession}), reused for
 * every call. No client-side caching or optimistic mutation of the read model — the hub log is the
 * single writer (contract §C6).
 */
export class DirectoryClient {
  private readonly baseUrl: string;
  private readonly requestBaseUrl: string;
  private readonly socketGrantIssuer: SocketGrantIssuerV1 | undefined;
  private readonly request: typeof fetchWithTimeout;

  constructor(baseUrl: string, options: { readonly requestBaseUrl?: string; readonly socketGrantIssuer?: SocketGrantIssuerV1; readonly request?: typeof fetchWithTimeout } = {}) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.requestBaseUrl = (options.requestBaseUrl ?? baseUrl).replace(/\/+$/, "");
    this.socketGrantIssuer = options.socketGrantIssuer;
    this.request = options.request ?? fetchWithTimeout;
  }

  private headers(json: boolean): Record<string, string> {
    const headers: Record<string, string> = {};
    if (json) headers["content-type"] = "application/json";
    return headers;
  }

  /** 📨️ {@link FetchTimeoutResponse} plus the one extra accessor this class needs — declared locally
   * per this module's own body accessing only `json()` beyond the base shape. */
  private async getJson<T>(path: string, options?: DirectoryRequestOptions): Promise<T> {
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  private async postJson<T>(path: string, body: unknown, options?: DirectoryRequestOptions): Promise<T> {
    const response = await this.request(
      `${this.requestBaseUrl}${path}`,
      { method: "POST", credentials: "include", headers: this.headers(true), body: JSON.stringify(body) },
      { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal },
    );
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: POST ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  /** 🪪️ `GET /auth/sessions/me` — `null` on 401 (no/expired session, the normal "not signed in"
   * outcome the boot flow branches on), throws {@link DirectoryHttpError} on any other failure. A
   * timeout/abort surfaces as a plain `Error` (no `.status`) — same "hub unreachable" shape the
   * caller's `directoryRejectionStatus`/identity-bootstrap catch already treats as offline, not a
   * boot-blocking failure (finding 1: this is what stops a hung server from hanging the boot path). */
  async me(options?: DirectoryRequestOptions): Promise<DirectorySessionAuthorityV1 | null> {
    try {
      const response = await this.request(`${this.requestBaseUrl}/auth/sessions/me`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
      if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET /auth/sessions/me failed (${response.status})`);
      return parseDirectorySessionAuthorityJsonV1(await response.text());
    } catch (error) {
      if (error instanceof DirectoryHttpError && error.status === 401) return null;
      throw error;
    }
  }

  async spaces(options?: DirectoryRequestOptions): Promise<readonly DirectorySpaceListEntryV1[]> {
    const value = await this.getJson<unknown>("/directory/spaces", options);
    if (!Array.isArray(value)) throw new Error("directory: invalid space list projection");
    return value.map(parseDirectorySpaceListEntryV1);
  }

  /** 🏛️ Fetches one bounded canonical administration page, preserving the exact receipt bytes.
   * `cursor` advances precisely the window it was issued for; every other window restarts. */
  async spaceAdministrationPage(id: string, cursor?: string, options?: DirectoryRequestOptions): Promise<CanonicalDirectorySpaceAdministrationPageV1> {
    if (cursor !== undefined && (cursor.length === 0 || cursor.length > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !/^[A-Za-z0-9._-]+$/u.test(cursor))) throw new Error("directory space administration: invalid cursor");
    const path = cursor === undefined ? `/directory/spaces/${encodeURIComponent(id)}` : `/directory/spaces/${encodeURIComponent(id)}?cursor=${cursor}`;
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    const canonicalJson = await response.text();
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory space administration: cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES) throw new Error("directory space administration: response too large");
    let page: DirectorySpaceAdministrationPageV1;
    try {
      page = await parseDirectorySpaceAdministrationPageV1(canonicalJson);
    } catch {
      throw new Error("directory space administration: invalid canonical response");
    }
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory space administration: cancelled");
    if (page.spaceId !== id) throw new Error("directory space administration: response space mismatch");
    return { canonicalJson, page };
  }

  /** 🧾️ Posts one sealed V1 request and parses only a raw-byte-capped canonical receipt bound to it.
   * A non-2xx becomes a closed {@link DirectoryCommandError} code; the response body is never read,
   * logged, or surfaced. The caller's `signal` cancels the HTTP wait only — a command already past
   * its server linearization point stays committed, so the operation becomes indeterminate. */
  async command(request: DirectoryCommandRequestV1, options?: DirectoryRequestOptions): Promise<DirectoryCommandReceiptV1> {
    const body = directoryCommandRequestJson(request);
    let response: FetchTimeoutResponse;
    try {
      response = await this.request(`${this.requestBaseUrl}/directory/commands`, { method: "POST", credentials: "include", headers: this.headers(true), body }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    } catch {
      throw new DirectoryCommandError(options?.signal?.aborted === true ? "cancelled" : "transport");
    }
    if (!response.ok) throw new DirectoryCommandError(directoryCommandErrorFromStatus(response.status));
    const canonicalJson = await response.text();
    if (options?.signal?.aborted === true) throw new DirectoryCommandError("cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES) throw new DirectoryCommandError("too-large");
    try {
      return await parseDirectoryCommandReceiptV1(canonicalJson, request);
    } catch {
      throw new DirectoryCommandError("invalid");
    }
  }

  async events(since: number, options?: DirectoryRequestOptions): Promise<readonly DirectoryEvent[]> {
    return this.getJson<DirectoryEvent[]>(`/directory/events?since=${encodeURIComponent(String(since))}`, options);
  }

  /** 📄️ Fetches and validates the original canonical page text without a JSON parse/reserialize gap. */
  async eventPage(after: number, options?: DirectoryRequestOptions): Promise<CanonicalDirectoryEventPageV1> {
    if (!Number.isSafeInteger(after) || after < 0) throw new Error("directory event page: invalid after frontier");
    const path = `/directory/event-page/v1?after=${after}`;
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    const canonicalJson = await response.text();
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory event page: cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_EVENT_PAGE_MAX_BYTES) throw new Error("directory event page: response too large");
    let page: DirectoryEventPageV1;
    try {
      page = await parseDirectoryEventPageV1(canonicalJson);
    } catch {
      throw new Error("directory event page: invalid canonical response");
    }
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory event page: cancelled");
    if (page.afterSeqExclusive !== after) throw new Error("directory event page: response frontier mismatch");
    return {
      canonicalJson,
      sessionBindingSha256: page.sessionBindingSha256,
      authorizationGeneration: page.authorizationGeneration,
      afterSeqExclusive: page.afterSeqExclusive,
      throughSeqInclusive: page.throughSeqInclusive,
      hasMore: page.hasMore,
      receiptSha256: page.receiptSha256,
    };
  }

  /** 🎚️ Fetches one page of the caller's own preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): the same canonical,
   * receipt-bound page as {@link eventPage}, holding only `user.preference-recorded` events of the principal — a page with
   * anything else, or with another user's event, is refused. */
  async preferencePage(after: number, userId: string, options?: DirectoryRequestOptions): Promise<DirectoryPreferencePageV1> {
    if (!Number.isSafeInteger(after) || after < 0) throw new Error("directory preference page: invalid after frontier");
    const path = `${DIRECTORY_PREFERENCE_PAGE_PATH_V1}?after=${after}`;
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    const canonicalJson = await response.text();
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory preference page: cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_EVENT_PAGE_MAX_BYTES) throw new Error("directory preference page: response too large");
    let page: DirectoryEventPageV1;
    try {
      page = await parseDirectoryEventPageV1(canonicalJson);
    } catch {
      throw new Error("directory preference page: invalid canonical response");
    }
    if (page.afterSeqExclusive !== after) throw new Error("directory preference page: response frontier mismatch");
    const events = page.events.map((event) => {
      if (event.body.kind !== "user.preference-recorded" || event.userId !== userId) throw new Error("directory preference page: foreign event");
      return { seq: event.seq, schema: event.body.schema, mutation: event.body.mutation };
    });
    return { sessionBindingSha256: page.sessionBindingSha256, authorizationGeneration: page.authorizationGeneration, afterSeqExclusive: page.afterSeqExclusive, throughSeqInclusive: page.throughSeqInclusive, hasMore: page.hasMore, events };
  }

  /** 🔌️ Opens the explicitly global directory stream, which is not document-scope authority. */
  stream(since: number, onMessage: (message: DirectoryStreamMessage) => void): DirectoryStream {
    return this.streamFor(undefined, since, onMessage, undefined, true);
  }

  /** 📌️ Opens a global wakeup stream that never treats observed events or heartbeats as committed. */
  streamAcknowledged(since: number, onMessage: (message: DirectoryStreamMessage) => void): DirectoryAcknowledgedStream {
    if (!Number.isSafeInteger(since) || since < 0) throw new Error("directory stream: invalid acknowledged frontier");
    return this.streamFor(undefined, since, onMessage, undefined, false);
  }

  /** 🎯️ Opens one exact document-scoped directory stream and terminally reports membership revocation. */
  streamScoped(scope: DocumentScope, since: number, onMessage: (message: DirectoryStreamMessage) => void, onRevoked?: () => void): DirectoryStream {
    if (!scope.spaceId || !scope.documentId || new TextEncoder().encode(scope.spaceId).byteLength > 4096 || new TextEncoder().encode(scope.documentId).byteLength > 4096) throw new Error("directory stream: invalid scope");
    return this.streamFor(scope, since, onMessage, onRevoked, true);
  }

  /** 🔌️ Fresh protected grant + scoped/global socket — subscribes from `since`, replays gap-free, then goes
   * live; text (JSON) frames, one {@link DirectoryStreamMessage} each (contract §C2, unlike the
   * binary `protocol_wire` the artifact sync hub channel speaks). Auto-reconnects via
   * {@link retryWithJitteredBackoff} (finding 2 — full jitter avoids a thundering herd when many
   * shells' directory sockets drop together, e.g. a hub restart), resuming from the highest
   * `seq`/`headSeq` this subscription has actually observed — never the caller's original `since` —
   * so a reconnect never replays a gap or a duplicate; this resume-from-`lastSeq` behaviour is
   * unchanged from before. Never throws into the caller: a malformed frame is dropped, a socket
   * error/close only feeds the reconnect loop.
   *
   * 🩺️ Coordinator follow-up: reconnecting after a connection that proved itself open for at least
   * {@link HUB_HEALTHY_RESET_MS} (see its docstring for why that threshold specifically can't be
   * crossed by a flapping server) resets the backoff — the next redial lands near
   * {@link HUB_RECONNECT_MIN_MS} instead of wherever the delay had climbed to from earlier, unrelated
   * blips (CLAUDE.md "support short connection-shortages"). Mechanism: `retryWithJitteredBackoff`'s
   * attempt counter lives inside ONE call and cannot be reset from outside it, so instead of one
   * call for the stream's whole life, {@link runCycles} below starts a FRESH call (fresh counter)
   * each time a cycle ends because its connection proved healthy before dropping — `connectOnce`
   * resolves (instead of rejecting) in exactly that case to end the current call as a "success". The
   * very first `fn()` of the next cycle is a synthetic, immediate rejection (never opens a socket) so
   * `retryWithJitteredBackoff`'s own jitter still inserts a `[MIN, 2·MIN]` pause before the real
   * redial — reusing its jitter math for that pause rather than reinventing it, and avoiding an
   * instant reconnect that would defeat jitter's whole point of not synchronizing many clients onto
   * the same instant. Mirrors `🏪️store/👷️worker/🟦️.ts`'s `connectHubOnce`/`connectHub` idiom for the
   * base reconnect loop; the health-reset addition here has no counterpart there (routed to that
   * file's own owning packet). */
  private streamFor(scope: DocumentScope | undefined, since: number, onMessage: (message: DirectoryStreamMessage) => void, onRevoked: (() => void) | undefined, trackObservedFrontier: boolean): DirectoryAcknowledgedStream {
    const abort = new AbortController();
    let socket: WebSocket | null = null;
    let lastSeq = since;
    let healthy = false; // 🩺️ set once THIS cycle's socket has been open for HUB_HEALTHY_RESET_MS.

    const wsUrl = (): string => {
      const wsBase = this.baseUrl.replace(/^http/, "ws");
      const query = new URLSearchParams();
      query.set("since", String(lastSeq));
      const path = scope
        ? `/directory/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}/socket/v1`
        : "/directory/socket/v1";
      return `${wsBase}${path}?${query.toString()}`;
    };

    /** 🔌️ One WS connection attempt. Resolves once {@link close} aborts (a clean shutdown) OR once
     * the socket closes after having been open for {@link HUB_HEALTHY_RESET_MS} (a proven-healthy
     * drop — ends this cycle as a "success" so {@link runCycles} starts a fresh, reset one); rejects
     * on every other close/error/construct-throw, feeding the current cycle's growing jitter exactly
     * like before. The health timer is armed on open and always cleared on close, whichever reason —
     * never left pending past this promise settling. */
    const connectOnce = async (): Promise<void> => {
      const issuer = this.socketGrantIssuer;
      if (!issuer) throw new Error("directory stream: socket grant issuer unavailable");
      const receipt = parseSocketGrantReceiptV1(await (scope ? issuer.issueDirectoryScoped(scope, { signal: abort.signal }) : issuer.issueDirectory({ signal: abort.signal })));
      if (receipt.expiresAtMs <= Date.now()) throw new Error("directory stream: expired socket grant");
      return new Promise<void>((resolve, reject) => {
        if (abort.signal.aborted) {
          reject(abort.signal.reason ?? new Error("directory stream: closed"));
          return;
        }
        let ws: WebSocket;
        try {
          ws = new WebSocket(wsUrl(), [...socketGrantProtocolsV1(receipt)]);
        } catch (error) {
          reject(error);
          return;
        }
        socket = ws;
        const onAbort = (): void => ws.close();
        abort.signal.addEventListener("abort", onAbort, { once: true });
        let healthyTimer: ReturnType<typeof setTimeout> | null = null;
        ws.onopen = () => {
          if (ws.protocol !== "semio.socket.v1") {
            closeHubSocketV1(ws, "protocolMismatch");
            return;
          }
          ws.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: "semio.directory.v1", pack_schema_hash: new Array(32).fill(0), resume_token: null, frontier: null } }, "command"));
          healthyTimer = setTimeout(() => {
            healthy = true;
          }, HUB_HEALTHY_RESET_MS);
        };
        ws.onmessage = (event: unknown) => {
          try {
            const data = (event as { data: unknown }).data;
            const message = JSON.parse(String(data)) as DirectoryStreamMessage;
            if (trackObservedFrontier && message.kind === "event") lastSeq = Math.max(lastSeq, message.event.seq);
            if (trackObservedFrontier && !scope && message.kind === "heartbeat") lastSeq = Math.max(lastSeq, message.headSeq);
            onMessage(message);
          } catch {
            // 🛟️ malformed frame — dropped, never thrown into the caller.
          }
        };
        ws.onclose = (event: CloseEvent) => {
          abort.signal.removeEventListener("abort", onAbort);
          if (healthyTimer != null) clearTimeout(healthyTimer);
          if (socket === ws) socket = null;
          if (scope && event.code === 4401) {
            onRevoked?.();
            abort.abort(new Error("directory stream: scope revoked"));
            resolve();
            return;
          }
          if (abort.signal.aborted || healthy) {
            resolve();
            return;
          }
          reject(new Error("directory stream: socket closed"));
        };
        ws.onerror = () => {
          try {
            ws.close();
          } catch {
            // 🛟️ already closing.
          }
        };
      });
    };

    /** 🔁️ Runs {@link connectOnce} through {@link retryWithJitteredBackoff} for one cycle at a time,
     * forever, until `close()` aborts. A cycle ends in success only when `connectOnce` resolves —
     * either the manual-close case, or the health-reset case — never by exhausting retries (the
     * underlying primitive retries forever on failure by design). On a health-reset success, the NEXT
     * cycle is built "primed": its `fn` synthetically rejects once, immediately, before ever touching
     * the network, purely so the retry primitive's own jitter inserts a `[MIN, 2·MIN]` pause ahead of
     * the real redial — see this method's docstring for why that beats both an instant reconnect and
     * hand-rolling a second jitter formula. */
    async function runCycles(): Promise<void> {
      let primeNextCycle = false;
      for (;;) {
        healthy = false;
        let primed = !primeNextCycle;
        const fn = (): Promise<void> => {
          if (!primed) {
            primed = true;
            return Promise.reject(new Error("directory stream: healthy-reset pause"));
          }
          return connectOnce();
        };
        try {
          await retryWithJitteredBackoff(fn, { minMs: HUB_RECONNECT_MIN_MS, maxMs: HUB_RECONNECT_MAX_MS, signal: abort.signal });
        } catch {
          return; // 🛑 only reachable via an aborted signal — `close()` was called.
        }
        if (abort.signal.aborted) return; // 🛑 resolved via the manual-close path above.
        primeNextCycle = healthy; // 🩺️ resolved via the health-reset path — start the next cycle primed.
      }
    }

    void runCycles();

    return {
      acknowledge: (through: number) => {
        if (!Number.isSafeInteger(through) || through < lastSeq) throw new Error("directory stream: invalid acknowledged frontier");
        lastSeq = through;
      },
      close: () => {
        abort.abort();
        socket?.close();
      },
    };
  }
}
//#endregion 🔖️HubBinding
//#endregion 🔖️Directory
