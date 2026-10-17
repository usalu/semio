/** 📄️ Directory event page: the bounded `DirectoryEventPageV1` shape and its structural guards. */

import { type DirectoryEvent, type DocumentIndexEntryV1, validDocumentIndexEntryV1 } from "../📜️event/🟦️.ts";

//#region 🔖️Event
export const DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS = 128;
export const DIRECTORY_EVENT_PAGE_MAX_BYTES = 64 * 1024;
export const DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES = 48 * 1024;

/** 📄️ One authenticated, receipt-bound bounded scan of the durable directory log. */
export interface DirectoryEventPageV1 {
  schema: "semio.directory.event-page.v1";
  sessionBindingSha256: string;
  authorizationGeneration: number;
  afterSeqExclusive: number;
  throughSeqInclusive: number;
  hasMore: boolean;
  events: DirectoryEvent[];
  receiptSha256: string;
}

export function directoryEventPageObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("directory-event-page.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  if (required.some((key) => !(key in object)) || Object.keys(object).some((key) => !accepted.has(key))) throw new Error("directory-event-page.invalid-fields");
  return object;
}

export function directoryEventPageHasControl(value: unknown): boolean {
  if (typeof value === "string") return /\p{Cc}/u.test(value);
  if (Array.isArray(value)) return value.some(directoryEventPageHasControl);
  return value !== null && typeof value === "object" && Object.entries(value).some(([key, child]) => /\p{Cc}/u.test(key) || directoryEventPageHasControl(child));
}

export function directoryEventPageInteger(value: unknown, positive = false): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("directory-event-page.invalid-integer");
  return value;
}

export function directoryEventPageHash(value: unknown, nonzero: boolean): string {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || (nonzero && /^0{64}$/u.test(value))) throw new Error("directory-event-page.invalid-hash");
  return value;
}

export function directoryEventPageNestedShapes(body: Record<string, unknown>): void {
  const exact = (value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> => directoryEventPageObject(value, required, optional);
  const texts = (object: Record<string, unknown>, fields: readonly string[]): void => {
    if (fields.some((field) => typeof object[field] !== "string")) throw new Error("directory-event-page.invalid-text");
  };
  const hash = (value: unknown): void => {
    if (!Array.isArray(value) || value.length !== 32 || value.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error("directory-event-page.invalid-byte-hash");
  };
  const scope = (value: unknown): void => {
    const object = exact(value, ["spaceId", "documentId"]);
    texts(object, ["spaceId", "documentId"]);
  };
  const frontier = (value: unknown): void => {
    const object = exact(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
    texts(object, ["documentId", "headEditId"]);
    directoryEventPageInteger(object.headEditOrdinal);
    directoryEventPageInteger(object.lastCommitSeq);
    hash(object.chainHash);
  };
  const descriptor = (value: unknown): void => {
    const object = exact(value, ["spaceId", "documentId", "artifactKind", "artifactSchema", "owner", "packSchemaHash", "bootstrapVersion", "bootstrapFrontier", "bootstrapSnapshotHash"]);
    texts(object, ["spaceId", "documentId", "artifactKind", "artifactSchema", "packSchemaHash", "bootstrapSnapshotHash"]);
    const owner = exact(object.owner, ["pluginId", "packageId", "version", "packageHash"]);
    texts(owner, ["pluginId", "packageId", "version", "packageHash"]);
    const bootstrap = exact(object.bootstrapFrontier, ["headSeq", "commitSeq", "epoch"]);
    directoryEventPageInteger(object.bootstrapVersion, true);
    for (const field of ["headSeq", "commitSeq", "epoch"]) directoryEventPageInteger(bootstrap[field]);
  };
  if (body.kind === "document.announced") descriptor(body.descriptor);
  if (body.kind === "document.indexed") {
    scope(body.scope);
    hash(body.descriptorDigestV1);
    if (!(body.descriptorDigestV1 as number[]).some((byte) => byte !== 0)) throw new Error("directory-event-page.invalid-index-digest");
    const entry = exact(body.entry, ["name", "dialect"]);
    exact(entry.dialect, ["artifactKind", "standard", "subset"]);
    if (!validDocumentIndexEntryV1(entry as unknown as DocumentIndexEntryV1)) throw new Error("directory-event-page.invalid-index");
  }
  if (body.kind === "artifact.checkpoint-published") {
    const checkpoint = exact(body.checkpoint, ["scope", "checkpointId", "descriptorDigestV1", "baselineFrontier", "pack", "spr", "aggregateSha256", "publishedAtMs"], ["parentCheckpointId"]);
    scope(checkpoint.scope);
    hash(checkpoint.checkpointId);
    if (checkpoint.parentCheckpointId !== undefined) hash(checkpoint.parentCheckpointId);
    hash(checkpoint.descriptorDigestV1);
    frontier(checkpoint.baselineFrontier);
    for (const field of ["pack", "spr"] as const) {
      const blob = exact(checkpoint[field], ["sha256", "byteLength"]);
      hash(blob.sha256);
      directoryEventPageInteger(blob.byteLength, true);
    }
    hash(checkpoint.aggregateSha256);
    directoryEventPageInteger(checkpoint.publishedAtMs);
  }
  if (body.kind === "artifact.retention-advanced") {
    const retention = exact(body.retention, ["scope", "retainedCheckpointId", "retainedFloor", "checkpointLineageHead"]);
    scope(retention.scope);
    hash(retention.retainedCheckpointId);
    frontier(retention.retainedFloor);
    hash(retention.checkpointLineageHead);
  }
}
//#endregion 🔖️Event
