/** 🚪️ Canonical Directory JSON admission and physical receipt projection. */
import { directoryAdministrationCapabilityAllowsV1, type DirectoryCommand, type DirectoryCommandOutcomeV1, type DirectoryCommandReceiptV1, type DirectoryCommandRequestV1, type DirectoryCommandResultV1, type DirectoryEvent, type DirectoryEventPageV1, type DirectorySpaceAdministrationPageV1, type DirectorySpaceRole, type DirectorySpaceVisibility, type DocumentScope, DIRECTORY_COMMAND_FIELDS, DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA, USER_PREFERENCE_MUTATION_MAX_BYTES, USER_PREFERENCE_SCHEMA_ID_MAX_BYTES, administrationCapabilities, administrationInviteRow, administrationMemberRow, administrationObject, administrationText, administrationTime, administrationWindow, directoryCommandCanonicalResult, directoryCommandRequestId, directoryEventPageHasControl, directoryEventPageHash, directoryEventPageInteger, directoryEventPageNestedShapes, directoryEventPageObject } from "../../🧬️schema/🟦️.ts";
import {
  type DocumentOpenBrowserActorV1,
  type DocumentExecutionTargetBrowserActorV1,
  parseDocumentOpenBrowserActorV1,
  parseDocumentExecutionTargetBrowserActorV1,
  documentBrowserActorLeaseFromPlanV1,
  sameDocumentBrowserActorV1,
} from "../../🧬️schema/🌐️browser-actor/🟦️.ts";
import { type EditedArtifactFrontierV1, isEditedArtifactFrontierV1 as editedArtifactFrontierIsValid } from "../../🧬️schema/📌️document-check-in-v1/🟦️.ts";

/** 🛡️ A preference vocabulary id (`os.config.ui-preferences.v1`) and one JSON object text within its bound — the Rust
 * twin's `valid_user_preference_record_v1`. The hub never reads the object; the vocabulary's own clients do. */
export function validUserPreferenceRecordV1(schema: unknown, mutation: unknown): boolean {
  if (typeof schema !== "string" || schema.length === 0 || new TextEncoder().encode(schema).length > USER_PREFERENCE_SCHEMA_ID_MAX_BYTES || !/^[a-z0-9][a-z0-9.-]*[a-z0-9]$/u.test(schema)) return false;
  if (typeof mutation !== "string" || mutation.length < 2 || new TextEncoder().encode(mutation).length > USER_PREFERENCE_MUTATION_MAX_BYTES) return false;
  try {
    const value: unknown = JSON.parse(mutation);
    return value !== null && typeof value === "object" && !Array.isArray(value);
  } catch {
    return false;
  }
}
/** 📇️ Validates one directory event outside a page (the space index lane's events), by the page's own event rules. */
export function parseDirectoryEventV1(value: unknown): DirectoryEvent {
  return directoryEventPageEvent(value);
}
function directoryEventPageEvent(value: unknown): DirectoryEvent {
  const event = directoryEventPageObject(value, ["seq", "id", "hlc", "actor", "body", "recordedAtMs"], ["spaceId", "userId"]);
  directoryEventPageInteger(event.seq, true);
  if (
    typeof event.id !== "string" ||
    typeof event.recordedAtMs !== "number" ||
    !Number.isSafeInteger(event.recordedAtMs) ||
    (event.spaceId !== undefined && typeof event.spaceId !== "string") ||
    (event.userId !== undefined && typeof event.userId !== "string")
  )
    throw new Error("directory-event-page.invalid-event");
  directoryEventPageInteger(directoryEventPageObject(event.hlc, ["physicalMs", "logical"]).logical);
  const physicalMs = (event.hlc as Record<string, unknown>).physicalMs;
  if (typeof physicalMs !== "number" || !Number.isSafeInteger(physicalMs)) throw new Error("directory-event-page.invalid-time");
  const actor = directoryEventPageObject(event.actor, ["kind", "id"]);
  if ((actor.kind !== "user" && actor.kind !== "admin" && actor.kind !== "system") || typeof actor.id !== "string") throw new Error("directory-event-page.invalid-actor");
  if (event.body === null || typeof event.body !== "object" || Array.isArray(event.body)) throw new Error("directory-event-page.invalid-event-body");
  const body = event.body as Record<string, unknown>;
  const bodyFields: Record<string, readonly string[]> = {
    "user.created": ["kind", "userId", "email", "displayName"],
    "space.created": ["kind", "spaceId", "name", "spaceKind", "visibility", "ownerUserId"],
    "space.renamed": ["kind", "spaceId", "name"],
    "space.visibility-changed": ["kind", "spaceId", "visibility"],
    "space.archived": ["kind", "spaceId"],
    "space.deleted": ["kind", "spaceId"],
    "member.upserted": ["kind", "spaceId", "userId", "role"],
    "member.removed": ["kind", "spaceId", "userId"],
    "invite.redeemed": ["kind", "spaceId", "userId", "inviteId", "role"],
    "document.announced": ["kind", "descriptor"],
    "document.indexed": ["kind", "scope", "descriptorDigestV1", "entry"],
    "artifact.checkpoint-published": ["kind", "checkpoint"],
    "artifact.retention-advanced": ["kind", "retention"],
    "user.preference-recorded": ["kind", "userId", "schema", "mutation"],
  };
  const fields = typeof body.kind === "string" ? bodyFields[body.kind] : undefined;
  if (!fields) throw new Error("directory-event-page.invalid-event-kind");
  directoryEventPageObject(body, fields);
  const textFields: Record<string, readonly string[]> = {
    "user.created": ["userId", "email", "displayName"],
    "space.created": ["spaceId", "name", "ownerUserId"],
    "space.renamed": ["spaceId", "name"],
    "space.visibility-changed": ["spaceId"],
    "space.archived": ["spaceId"],
    "space.deleted": ["spaceId"],
    "member.upserted": ["spaceId", "userId"],
    "member.removed": ["spaceId", "userId"],
    "invite.redeemed": ["spaceId", "userId", "inviteId"],
    "user.preference-recorded": ["userId", "schema", "mutation"],
  };
  if ((textFields[body.kind as string] ?? []).some((field) => typeof body[field] !== "string")) throw new Error("directory-event-page.invalid-event-text");
  if (body.kind === "user.preference-recorded" && (!validUserPreferenceRecordV1(body.schema, body.mutation) || event.spaceId !== undefined || event.userId !== body.userId || (body.userId as string).length === 0)) throw new Error("directory-event-page.invalid-user-preference");
  if (
    (body.kind === "space.created" && body.spaceKind !== "atelier" && body.spaceKind !== "studio" && body.spaceKind !== "archive") ||
    ((body.kind === "space.created" || body.kind === "space.visibility-changed") && body.visibility !== "private" && body.visibility !== "public") ||
    ((body.kind === "member.upserted" || body.kind === "invite.redeemed") && body.role !== "author" && body.role !== "spectator")
  )
    throw new Error("directory-event-page.invalid-event-vocabulary");
  directoryEventPageNestedShapes(body);
  if (body.kind === "document.indexed" && (event.spaceId !== (body.scope as DocumentScope).spaceId || typeof event.userId !== "string" || event.userId.length === 0 || new TextEncoder().encode(event.userId).length > 256)) throw new Error("directory-event-page.invalid-index-author");
  if (directoryEventPageHasControl(event)) throw new Error("directory-event-page.control-character");
  if (new TextEncoder().encode(JSON.stringify(event)).length > DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES) throw new Error("directory-event-page.event-too-large");
  return event as unknown as DirectoryEvent;
}
async function directoryEventPageSha256(text: string): Promise<string> {
  const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}
/** 📥️ Parses one canonical page and verifies exact fields, ranges, size, and SHA-256 receipt. */
export async function parseDirectoryEventPageV1(source: string): Promise<DirectoryEventPageV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_EVENT_PAGE_MAX_BYTES) throw new Error("directory-event-page.too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "sessionBindingSha256", "authorizationGeneration", "afterSeqExclusive", "throughSeqInclusive", "hasMore", "events", "receiptSha256"]);
  if (object.schema !== "semio.directory.event-page.v1" || typeof object.hasMore !== "boolean" || !Array.isArray(object.events) || object.events.length > DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS) throw new Error("directory-event-page.invalid-envelope");
  const afterSeqExclusive = directoryEventPageInteger(object.afterSeqExclusive);
  const throughSeqInclusive = directoryEventPageInteger(object.throughSeqInclusive);
  if (afterSeqExclusive > throughSeqInclusive) throw new Error("directory-event-page.invalid-range");
  const events = object.events.map(directoryEventPageEvent);
  let previous = afterSeqExclusive;
  for (const event of events) {
    if (event.seq <= previous || event.seq > throughSeqInclusive) throw new Error("directory-event-page.invalid-event-range");
    previous = event.seq;
  }
  const page: DirectoryEventPageV1 = {
    schema: object.schema,
    sessionBindingSha256: directoryEventPageHash(object.sessionBindingSha256, true),
    authorizationGeneration: directoryEventPageInteger(object.authorizationGeneration, true),
    afterSeqExclusive,
    throughSeqInclusive,
    hasMore: object.hasMore,
    events,
    receiptSha256: directoryEventPageHash(object.receiptSha256, false),
  };
  if (JSON.stringify(page) !== source) throw new Error("directory-event-page.noncanonical");
  const { receiptSha256, ...unsigned } = page;
  if ((await directoryEventPageSha256(JSON.stringify(unsigned))) !== receiptSha256) throw new Error("directory-event-page.receipt-mismatch");
  return page;
}
/** 🧭 Reconstructs one closed command in declaration order after an order-independent carrier. */
export function canonicalDirectoryCommandV1(value: unknown): DirectoryCommand {
  const command = directoryEventPageObject(value, ["kind"], Object.values(DIRECTORY_COMMAND_FIELDS).flat());
  const fields = typeof command.kind === "string" ? DIRECTORY_COMMAND_FIELDS[command.kind] : undefined;
  if (!fields) throw new Error("directory-command.invalid-kind");
  directoryEventPageObject(command, fields);
  const canonical: Record<string, unknown> = {};
  for (const field of fields) canonical[field] = command[field];
  if (directoryEventPageHasControl(command)) throw new Error("directory-command.control-character");
  const text = (field: string): string => {
    const value = command[field];
    if (typeof value !== "string") throw new Error(`directory-command.invalid-${field}`);
    return value;
  };
  const role = (value: unknown): DirectorySpaceRole => {
    if (value !== "author" && value !== "spectator") throw new Error("directory-command.invalid-role");
    return value;
  };
  const visibility = (value: unknown): DirectorySpaceVisibility => {
    if (value !== "private" && value !== "public") throw new Error("directory-command.invalid-visibility");
    return value;
  };
  switch (command.kind) {
    case "create-space":
      text("name");
      if (command.spaceKind !== "atelier" && command.spaceKind !== "studio" && command.spaceKind !== "archive") throw new Error("directory-command.invalid-space-kind");
      visibility(command.visibility);
      break;
    case "rename-space": text("spaceId"); text("name"); break;
    case "set-visibility": text("spaceId"); visibility(command.visibility); break;
    case "archive-space":
    case "delete-space": text("spaceId"); break;
    case "upsert-member": text("spaceId"); text("email"); role(command.role); break;
    case "remove-member": text("spaceId"); text("userId"); break;
    case "create-invite": text("spaceId"); role(command.role); directoryEventPageInteger(command.ttlSecs, true); break;
    case "revoke-invite": text("spaceId"); text("inviteId"); break;
    case "record-user-preference": if (!validUserPreferenceRecordV1(command.schema, command.mutation)) throw new Error("directory-command.invalid-user-preference"); break;
    case "announce-document": {
      directoryEventPageNestedShapes({ kind: "document.announced", descriptor: command.descriptor });
      const descriptor = command.descriptor as Record<string, unknown>;
      const owner = descriptor.owner as Record<string, unknown>;
      const frontier = descriptor.bootstrapFrontier as Record<string, unknown>;
      for (const [field, candidate] of [
        ["spaceId", descriptor.spaceId], ["documentId", descriptor.documentId], ["artifactKind", descriptor.artifactKind], ["artifactSchema", descriptor.artifactSchema],
        ["owner.pluginId", owner.pluginId], ["owner.packageId", owner.packageId], ["owner.version", owner.version],
      ] as const) if (typeof candidate !== "string" || candidate.trim().length === 0) throw new Error(`directory-command.invalid-${field}`);
      for (const [field, candidate] of [["owner.packageHash", owner.packageHash], ["packSchemaHash", descriptor.packSchemaHash], ["bootstrapSnapshotHash", descriptor.bootstrapSnapshotHash]] as const) {
        if (typeof candidate !== "string" || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(candidate)) throw new Error(`directory-command.invalid-${field}`);
      }
      if ((frontier.commitSeq as number) > (frontier.headSeq as number)) throw new Error("directory-command.invalid-bootstrap-frontier");
      canonical.descriptor = {
        spaceId: descriptor.spaceId,
        documentId: descriptor.documentId,
        artifactKind: descriptor.artifactKind,
        artifactSchema: descriptor.artifactSchema,
        owner: { pluginId: owner.pluginId, packageId: owner.packageId, version: owner.version, packageHash: owner.packageHash },
        packSchemaHash: descriptor.packSchemaHash,
        bootstrapVersion: descriptor.bootstrapVersion,
        bootstrapFrontier: { headSeq: frontier.headSeq, commitSeq: frontier.commitSeq, epoch: frontier.epoch },
        bootstrapSnapshotHash: descriptor.bootstrapSnapshotHash,
      };
      break;
    }
  }
  return canonical as unknown as DirectoryCommand;
}
/** 🛡️ Decodes one declaration-ordered canonical command and validates every scalar. */
export function parseDirectoryCommandV1(value: unknown): DirectoryCommand {
  const canonical = canonicalDirectoryCommandV1(value);
  if (JSON.stringify(canonical) !== JSON.stringify(value)) throw new Error("directory-command.noncanonical-command");
  return canonical;
}
/** 🔐️ The one canonical command digest both the hub and every client derive independently. */
export async function directoryCommandSha256(command: DirectoryCommand): Promise<string> {
  return directoryEventPageSha256(JSON.stringify(command));
}
/** 🆕️ Seals one request around an already-minted correlation id. */
export function sealDirectoryCommandRequestV1(requestId: string, command: DirectoryCommand): DirectoryCommandRequestV1 {
  return { schema: "semio.directory.command-request.v1", requestId: directoryCommandRequestId(requestId), command: parseDirectoryCommandV1(command) };
}
/** 🧾️ Returns the canonical UTF-8 JSON both peers hash and count bytes over. */
export function directoryCommandRequestJson(request: DirectoryCommandRequestV1): string {
  const canonical: DirectoryCommandRequestV1 = { schema: request.schema, requestId: request.requestId, command: request.command };
  const json = JSON.stringify(canonical);
  if (new TextEncoder().encode(json).length > DIRECTORY_COMMAND_REQUEST_MAX_BYTES) throw new Error("directory-command.request-too-large");
  return json;
}
/** 📥️ Parses exactly one canonical request, rejecting padding, unknown fields, and oversize bodies. */
export function parseDirectoryCommandRequestV1(source: string): DirectoryCommandRequestV1 {
  if (new TextEncoder().encode(source).length > DIRECTORY_COMMAND_REQUEST_MAX_BYTES) throw new Error("directory-command.request-too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "requestId", "command"]);
  if (object.schema !== "semio.directory.command-request.v1") throw new Error("directory-command.invalid-envelope");
  const request: DirectoryCommandRequestV1 = { schema: object.schema, requestId: directoryCommandRequestId(object.requestId), command: parseDirectoryCommandV1(object.command) };
  if (JSON.stringify(request) !== source) throw new Error("directory-command.noncanonical");
  return request;
}
/** 🔐️ Seals one completion by hashing its declaration-ordered unsigned canonical JSON — the exact
 * twin of the Rust `DirectoryCommandReceiptV1::seal`. */
export async function sealDirectoryCommandReceiptV1(requestId: string, commandSha256: string, outcome: DirectoryCommandOutcomeV1, events: readonly DirectoryEvent[], result: DirectoryCommandResultV1): Promise<DirectoryCommandReceiptV1> {
  const unsigned = { schema: "semio.directory.command-receipt.v1" as const, requestId: directoryCommandRequestId(requestId), commandSha256: directoryEventPageHash(commandSha256, false), outcome, events: [...events], result };
  return { ...unsigned, receiptSha256: await directoryEventPageSha256(JSON.stringify(unsigned)) };
}
/** 📥️ Parses exactly one canonical receipt bound to the request that asked for it. */
export async function parseDirectoryCommandReceiptV1(source: string, request: DirectoryCommandRequestV1): Promise<DirectoryCommandReceiptV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES) throw new Error("directory-command.receipt-too-large");
  const object = directoryEventPageObject(JSON.parse(source), ["schema", "requestId", "commandSha256", "outcome", "events", "result", "receiptSha256"]);
  if (
    object.schema !== "semio.directory.command-receipt.v1" ||
    (object.outcome !== "accepted" && object.outcome !== "previously-accepted" && object.outcome !== "secret-undeliverable") ||
    !Array.isArray(object.events) ||
    object.events.length > DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS
  )
    throw new Error("directory-command.invalid-envelope");
  const events = object.events.map(directoryEventPageEvent);
  let previous = 0;
  for (const event of events) {
    if (event.seq <= previous) throw new Error("directory-command.invalid-event-range");
    previous = event.seq;
  }
  const receipt: DirectoryCommandReceiptV1 = {
    schema: object.schema,
    requestId: directoryCommandRequestId(object.requestId),
    commandSha256: directoryEventPageHash(object.commandSha256, false),
    outcome: object.outcome,
    events,
    result: directoryCommandCanonicalResult(object.result),
    receiptSha256: directoryEventPageHash(object.receiptSha256, false),
  };
  if (receipt.outcome !== "accepted" && (receipt.result.kind !== "none" || receipt.events.length > 0)) throw new Error("directory-command.redaction-violated");
  if (receipt.requestId !== request.requestId || receipt.commandSha256 !== (await directoryCommandSha256(request.command))) throw new Error("directory-command.request-mismatch");
  if (JSON.stringify(receipt) !== source) throw new Error("directory-command.noncanonical");
  const { receiptSha256, ...unsigned } = receipt;
  if ((await directoryEventPageSha256(JSON.stringify(unsigned))) !== receiptSha256) throw new Error("directory-command.receipt-mismatch");
  return receipt;
}
async function administrationSha256(text: string): Promise<string> {
  const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}
/** 📥️ Parses one canonical administration page and verifies fields, ordering, size, and receipt. */
export async function parseDirectorySpaceAdministrationPageV1(source: string): Promise<DirectorySpaceAdministrationPageV1> {
  if (new TextEncoder().encode(source).length > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES) throw new Error("space-administration-page.too-large");
  const parsed = JSON.parse(source);
  const access = (parsed as Record<string, unknown> | null)?.access;
  const common = ["access", "schema", "sessionBindingSha256", "authorizationGeneration", "spaceId", "space"];
  const shape =
    access === "public"
      ? [...common, "documents", "receiptSha256"]
      : access === "member"
        ? [...common, "members", "documents", "receiptSha256"]
        : access === "author"
          ? [...common, "members", "documents", "invites", "capabilities", "receiptSha256"]
          : undefined;
  if (shape === undefined) throw new Error("space-administration-page.invalid-access");
  const object = administrationObject(parsed, shape);
  if (object.schema !== DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA) throw new Error("space-administration-page.invalid-schema");
  const sessionBindingSha256 = administrationText(object.sessionBindingSha256);
  if (!/^[0-9a-f]{64}$/u.test(sessionBindingSha256)) throw new Error("space-administration-page.invalid-binding");
  const authorizationGeneration = administrationTime(object.authorizationGeneration);
  const spaceId = administrationText(object.spaceId);
  const receiptSha256 = administrationText(object.receiptSha256);
  if (!/^[0-9a-f]{64}$/u.test(receiptSha256)) throw new Error("space-administration-page.invalid-receipt");
  const anonymous = authorizationGeneration === 0 && /^0{64}$/u.test(sessionBindingSha256);
  const bound = authorizationGeneration >= 1 && !/^0{64}$/u.test(sessionBindingSha256);
  if (!(anonymous || bound) || (access !== "public" && !bound)) throw new Error("space-administration-page.invalid-binding");
  const space = object.space as Record<string, unknown> | null;
  if (space === null || typeof space !== "object" || space.id !== spaceId) throw new Error("space-administration-page.space-mismatch");
  if (access === "public" && space.visibility !== "public") throw new Error("space-administration-page.space-mismatch");
  if (access === "member" && space.role !== "spectator") throw new Error("space-administration-page.space-mismatch");
  if (access === "author" && space.role !== "author") throw new Error("space-administration-page.space-mismatch");
  const documents = administrationWindow<unknown>(object.documents, (row) => row);
  const members = access === "public" ? undefined : administrationWindow(object.members, administrationMemberRow);
  if (members !== undefined) {
    let previous: string | undefined;
    for (const row of members.rows) {
      if (previous !== undefined && previous >= row.userId) throw new Error("space-administration-page.member-order");
      previous = row.userId;
    }
  }
  const invites = access === "author" ? administrationWindow(object.invites, administrationInviteRow) : undefined;
  if (invites !== undefined) {
    let previous: readonly [number, string] | undefined;
    for (const row of invites.rows) {
      if (previous !== undefined && !(previous[0] > row.createdAtMs || (previous[0] === row.createdAtMs && previous[1] > row.inviteId))) throw new Error("space-administration-page.invite-order");
      previous = [row.createdAtMs, row.inviteId];
    }
  }
  const capabilities = access === "author" ? administrationCapabilities(object.capabilities) : undefined;
  const base = { access, schema: object.schema, sessionBindingSha256, authorizationGeneration, spaceId, space: object.space };
  const page = (access === "public"
    ? { ...base, documents, receiptSha256 }
    : access === "member"
      ? { ...base, members, documents, receiptSha256 }
      : { ...base, members, documents, invites, capabilities, receiptSha256 }) as unknown as DirectorySpaceAdministrationPageV1;
  if (JSON.stringify(page) !== source) throw new Error("space-administration-page.noncanonical");
  const { receiptSha256: _receipt, ...unsigned } = page as Record<string, unknown> & { receiptSha256: string };
  if ((await administrationSha256(JSON.stringify(unsigned))) !== receiptSha256) throw new Error("space-administration-page.receipt-mismatch");
  return page;
}

/** 🛂️ Admits a canonical wire command before evaluating typed administration capabilities. */
export function directoryAdministrationCommandAllowedV1(page: DirectorySpaceAdministrationPageV1 | null, spaceId: string, command: unknown): boolean {
  try { return directoryAdministrationCapabilityAllowsV1(page, spaceId, parseDirectoryCommandV1(command)); } catch { return false; }
}
