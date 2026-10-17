/** 🔬️ Backbone worker parsers: bounded field parsers and the directory administration and space artifact creation request and status parsers. */

import { canonicalDirectoryCommandV1, directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../📇️directory/🚪️io/📝️text/🟦️.ts";
import type { ArtifactFrontier } from "../../📄️document/🪪️scope/🟦️.ts";
import { DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES } from "../../📇️directory/🧬️schema/📋️admin-page/🟦️.ts";
import { artifactFrontierIsEditedForV1, artifactFrontierIsGenesisForV1 } from "../../📄️document/🪪️scope/🟦️.ts";
import type { DocumentScope } from "../../📇️directory/🟦️.ts";
import type { BackboneWorkerRequest, SpaceArtifactCreationCatalogRefreshRequiredV1, SpaceArtifactCreationCatalogStatusV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationStatusV1 } from "../📨️worker-messages/🟦️.ts";

//#region 🔖️BackboneWorkerProtocol
//#region 🔖️EnvelopeCodec
export function workerWireIdV1(value: unknown): string | null {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) return null;
  return value;
}

function workerWireTextV1(value: unknown): string | null {
  return typeof value === "string" && !value.startsWith(" ") && !value.endsWith(" ") && [...value].length <= 128 && value.length > 0 && !/[\u0000-\u001f\u007f-\u009f]/u.test(value) ? value : null;
}

export function workerWireCreationIdentityV1(value: unknown): string | null {
  return typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value) ? value : null;
}

/** 🪆️ A DIALECT component on the same wire. `subset` is the one component whose canonical value is
 * not an identity at all: `SubsetId::ANY` is the literal `*` (`🚪️io/🧬️schema/🦀️.rs`), the
 * unconstrained base subset every standard carries and the subset of every kind this repository
 * ships. Validating it with `workerWireCreationIdentityV1` therefore rejected EVERY catalog the hub
 * can send: the main thread threw on decode, the shell kept `null`, and `createArtifact`'s kind
 * chooser rendered disabled with zero options on every space of every hub (measured 2026-09-22 on
 * hub 7651, whose own `GET /spaces/{id}/artifact-creations` answers two kinds, both `subset: "*"`). */
function workerWireDialectComponentV1(value: unknown): string | null {
  return value === "*" ? value : workerWireCreationIdentityV1(value);
}

export function workerWireCreationRequestIdV1(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{32}$)[0-9a-f]{32}$/u.test(value) ? value : null;
}

function workerWireCatalogGenerationIdV1(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value) ? value : null;
}

export function parseDirectoryAdministrationWorkerRequestV1(parsed: Readonly<Record<string, unknown>>): Extract<BackboneWorkerRequest, { readonly kind: `directory-administration-${string}` }> {
  const operationEpoch = parsed.operationEpoch;
  if (typeof operationEpoch !== "number" || !Number.isSafeInteger(operationEpoch) || operationEpoch < 0) throw new Error("backbone worker request: invalid administration operation epoch");
  const fields: Readonly<Record<string, readonly string[]>> = {
    "directory-administration-open": ["kind", "operationEpoch", "spaceId"],
    "directory-administration-refresh": ["kind", "operationEpoch", ...(parsed.cursor === undefined ? [] : ["cursor"])],
    "directory-administration-submit": ["kind", "operationEpoch", "requestId", "command"],
    "directory-administration-capability-request": ["kind", "operationEpoch"],
    "directory-administration-capability-result": ["kind", "operationEpoch", "transferEpoch", "copied"],
    "directory-administration-close": ["kind", "operationEpoch"],
  };
  const accepted = fields[String(parsed.kind)];
  if (accepted === undefined || Object.keys(parsed).sort().join(",") !== [...accepted].sort().join(",")) throw new Error("backbone worker request: invalid administration fields");
  switch (parsed.kind) {
    case "directory-administration-open": {
      const spaceId = typeof parsed.spaceId === "string" && new TextEncoder().encode(parsed.spaceId).length <= 256 && parsed.spaceId.length > 0 && !/\p{Cc}/u.test(parsed.spaceId) ? parsed.spaceId : null;
      if (spaceId === null) throw new Error("backbone worker request: invalid administration space");
      return { kind: parsed.kind, operationEpoch, spaceId };
    }
    case "directory-administration-refresh": {
      if (parsed.cursor === undefined) return { kind: parsed.kind, operationEpoch };
      if (typeof parsed.cursor !== "string" || parsed.cursor.length === 0 || parsed.cursor.length > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !/^[A-Za-z0-9._-]+$/u.test(parsed.cursor)) throw new Error("backbone worker request: invalid administration cursor");
      return { kind: parsed.kind, operationEpoch, cursor: parsed.cursor };
    }
    case "directory-administration-submit": {
      const requestId = workerWireCreationRequestIdV1(parsed.requestId);
      if (requestId === null) throw new Error("backbone worker request: invalid administration request id");
      const command = canonicalDirectoryCommandV1(parsed.command);
      directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, command));
      if (command.kind === "record-user-preference") throw new Error("backbone worker request: a user preference is not an administration command");
      const spaceId = command.kind === "create-space" ? undefined : command.kind === "announce-document" ? command.descriptor.spaceId : command.spaceId;
      if (spaceId !== undefined && workerWireIdV1(spaceId) === null) throw new Error("backbone worker request: invalid administration command space");
      return { kind: parsed.kind, operationEpoch, requestId, command };
    }
    case "directory-administration-capability-request": return { kind: parsed.kind, operationEpoch };
    case "directory-administration-capability-result": {
      if (typeof parsed.transferEpoch !== "number" || !Number.isSafeInteger(parsed.transferEpoch) || parsed.transferEpoch < 1 || typeof parsed.copied !== "boolean") throw new Error("backbone worker request: invalid administration capability result");
      return { kind: parsed.kind, operationEpoch, transferEpoch: parsed.transferEpoch, copied: parsed.copied };
    }
    case "directory-administration-close": return { kind: parsed.kind, operationEpoch };
    default: throw new Error("backbone worker request: invalid administration kind");
  }
}

export function parseSpaceArtifactCreationWorkerRequestV1(parsed: Readonly<Record<string, unknown>>): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" | "space-artifact-create-cancel" }> {
  const create = parsed.kind === "space-artifact-create";
  const expected = create ? "expectedCatalogGenerationId,kind,kindId,name,requestId,spaceId" : "kind,requestId,spaceId";
  if (Object.keys(parsed).sort().join(",") !== expected) throw new Error("backbone worker request: invalid space artifact creation fields");
  const requestId = workerWireCreationRequestIdV1(parsed.requestId),
    spaceId = workerWireCreationIdentityV1(parsed.spaceId);
  if (requestId === null || spaceId === null) throw new Error("backbone worker request: invalid space artifact creation owner");
  if (!create) return { kind: "space-artifact-create-cancel", requestId, spaceId };
  const expectedCatalogGenerationId = workerWireCatalogGenerationIdV1(parsed.expectedCatalogGenerationId),
    kindId = workerWireCreationIdentityV1(parsed.kindId),
    name = workerWireTextV1(parsed.name);
  if (expectedCatalogGenerationId === null || kindId === null || name === null) throw new Error("backbone worker request: invalid space artifact creation intent");
  return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId, kindId, name };
}

/** 🔄️ Validates the request-bound local refresh disposition emitted for an initial POST conflict. */
export function parseSpaceArtifactCreationCatalogRefreshRequiredV1(value: unknown): SpaceArtifactCreationCatalogRefreshRequiredV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog refresh: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,kind,requestId,spaceId") throw new Error("space artifact creation catalog refresh: invalid fields");
  const requestId = workerWireCreationRequestIdV1(row.requestId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-catalog-refresh-required" || requestId === null || spaceId === null || catalogGenerationId === null) throw new Error("space artifact creation catalog refresh: invalid owner");
  return { kind: "space-artifact-creation-catalog-refresh-required", requestId, spaceId, catalogGenerationId };
}

/** 🛡️ Validates the exact worker-visible projection of the Hub's durable creation status. */
export function parseSpaceArtifactCreationStatusV1(value: unknown): SpaceArtifactCreationStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation status: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  const phases: readonly SpaceArtifactCreationPhaseV1[] = ["accepted", "preparing", "ready", "indeterminate", "failed", "cancelled"];
  const phase = phases.includes(row.phase as SpaceArtifactCreationPhaseV1) ? (row.phase as SpaceArtifactCreationPhaseV1) : null;
  const requestId = workerWireCreationRequestIdV1(row.requestId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-status" || phase === null || requestId === null || spaceId === null || catalogGenerationId === null) throw new Error("space artifact creation status: invalid owner");
  const expected = phase === "ready" ? "catalogGenerationId,kind,phase,ready,requestId,spaceId" : "catalogGenerationId,kind,phase,requestId,spaceId";
  if (Object.keys(row).sort().join(",") !== expected) throw new Error("space artifact creation status: invalid fields");
  if (phase !== "ready") return { kind: "space-artifact-creation-status", requestId, spaceId, catalogGenerationId, phase };
  if (typeof row.ready !== "object" || row.ready === null || Array.isArray(row.ready)) throw new Error("space artifact creation status: invalid ready record");
  const ready = row.ready as Readonly<Record<string, unknown>>;
  if (Object.keys(ready).sort().join(",") !== "artifactId,artifactSchema,kindId,parentDialect") throw new Error("space artifact creation status: invalid ready fields");
  if (typeof ready.parentDialect !== "object" || ready.parentDialect === null || Array.isArray(ready.parentDialect)) throw new Error("space artifact creation status: invalid parent dialect");
  const parentDialect = ready.parentDialect as Readonly<Record<string, unknown>>;
  if (Object.keys(parentDialect).sort().join(",") !== "artifactKind,standard,subset") throw new Error("space artifact creation status: invalid parent dialect fields");
  const artifactId = typeof ready.artifactId === "string" && /^artifact-(?!0{32}$)[0-9a-f]{32}$/u.test(ready.artifactId) ? ready.artifactId : null,
    kindId = workerWireCreationIdentityV1(ready.kindId),
    artifactSchema = workerWireCreationIdentityV1(ready.artifactSchema),
    artifactKind = workerWireCreationIdentityV1(parentDialect.artifactKind),
    standard = workerWireDialectComponentV1(parentDialect.standard),
    subset = workerWireDialectComponentV1(parentDialect.subset);
  if (artifactId === null || kindId === null || artifactSchema === null || artifactKind === null || standard === null || subset === null) throw new Error("space artifact creation status: invalid ready identity");
  return { kind: "space-artifact-creation-status", requestId, spaceId, catalogGenerationId, phase, ready: { artifactId, kindId, artifactSchema, parentDialect: { artifactKind, standard, subset } } };
}

/** 🗂️ Validates the presentation-only selected-current creation catalog from the worker. A member's creation kind
 * (`kindId`, e.g. `2d.drawing`) and the dialect whose surface opens it (`dialect.artifactKind`, e.g. `s.draw.drawing`)
 * are the hub catalog's two independent names; neither is derived from the other here. */
export function parseSpaceArtifactCreationCatalogV1(value: unknown): SpaceArtifactCreationCatalogV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,clientInstanceId,kind,kinds,spaceId") throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = workerWireCreationIdentityV1(row.spaceId),
    clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId);
  const catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-catalog" || spaceId === null || clientInstanceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || row.kinds.length === 0 || row.kinds.length > 64)
    throw new Error("space artifact creation catalog: invalid owner");
  const kinds = row.kinds.map((value) => {
    if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog: invalid kind");
    const kind = value as Readonly<Record<string, unknown>>;
    if (Object.keys(kind).sort().join(",") !== "dialect,kindId,label,schema" || typeof kind.dialect !== "object" || kind.dialect === null || Array.isArray(kind.dialect) || typeof kind.label !== "object" || kind.label === null || Array.isArray(kind.label))
      throw new Error("space artifact creation catalog: invalid kind fields");
    const dialect = kind.dialect as Readonly<Record<string, unknown>>,
      label = kind.label as Readonly<Record<string, unknown>>;
    if (Object.keys(dialect).sort().join(",") !== "artifactKind,standard,subset" || Object.keys(label).sort().join(",") !== "de,en") throw new Error("space artifact creation catalog: invalid nested fields");
    const kindId = workerWireCreationIdentityV1(kind.kindId),
      schema = workerWireCreationIdentityV1(kind.schema),
      artifactKind = workerWireCreationIdentityV1(dialect.artifactKind),
      standard = workerWireDialectComponentV1(dialect.standard),
      subset = workerWireDialectComponentV1(dialect.subset),
      en = workerWireTextV1(label.en),
      de = workerWireTextV1(label.de);
    if (kindId === null || schema === null || artifactKind === null || standard === null || subset === null || en === null || de === null) throw new Error("space artifact creation catalog: invalid kind identity");
    return { kindId, schema, dialect: { artifactKind, standard, subset }, label: { en, de } };
  });
  if (kinds.some((entry, index) => index > 0 && kinds[index - 1]!.kindId >= entry.kindId)) throw new Error("space artifact creation catalog: invalid order");
  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId, kinds };
}

/** 🗂️ Validates a catalog presentation update without granting any catalog contents. */
export function parseSpaceArtifactCreationCatalogStatusV1(value: unknown): SpaceArtifactCreationCatalogStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog status: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "clientInstanceId,kind,phase,spaceId") throw new Error("space artifact creation catalog status: invalid fields");
  const clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    phase = row.phase === "loading" || row.phase === "ready" || row.phase === "unavailable" ? row.phase : null;
  if (row.kind !== "space-artifact-creation-catalog-status" || clientInstanceId === null || spaceId === null || phase === null) throw new Error("space artifact creation catalog status: invalid owner");
  return { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId, phase };
}

export function workerWireClientInstanceIdV1(value: unknown): string | null {
  const id = workerWireIdV1(value);
  return id !== null && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(id) ? id : null;
}

export function workerWireSha256V1(value: unknown): string | null {
  return typeof value === "string" && /^[0-9a-f]{64}$/u.test(value) ? value : null;
}

export function workerWireArtifactFrontierV1(value: unknown, scope: DocumentScope): ArtifactFrontier | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "chainHash,documentId,headEditId,headEditOrdinal,lastCommitSeq") return null;
  const chainHash = row.chainHash;
  if (!Array.isArray(chainHash) || chainHash.length !== 32 || chainHash.some(byte => !Number.isInteger(byte) || (byte as number) < 0 || (byte as number) > 255)) return null;
  if (typeof row.documentId !== "string" || typeof row.headEditId !== "string" || !Number.isSafeInteger(row.headEditOrdinal) || !Number.isSafeInteger(row.lastCommitSeq)) return null;
  const frontier: ArtifactFrontier = { documentId: row.documentId, headEditOrdinal: row.headEditOrdinal as number, headEditId: row.headEditId, lastCommitSeq: row.lastCommitSeq as number, chainHash: chainHash as number[] };
  return (artifactFrontierIsGenesisForV1(scope, frontier) || artifactFrontierIsEditedForV1(scope, frontier)) && frontier.lastCommitSeq <= frontier.headEditOrdinal ? frontier : null;
}

export function workerWireScopeV1(value: unknown, documentId: string): DocumentScope | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "documentId,spaceId") return null;
  const spaceId = workerWireIdV1(row.spaceId);
  const scopedDocumentId = workerWireIdV1(row.documentId);
  return spaceId === null || scopedDocumentId === null || scopedDocumentId !== documentId ? null : { spaceId, documentId };
}
//#endregion 🔖️EnvelopeCodec
//#endregion 🔖️BackboneWorkerProtocol
