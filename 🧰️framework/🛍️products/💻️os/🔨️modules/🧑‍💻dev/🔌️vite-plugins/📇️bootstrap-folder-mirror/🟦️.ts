/** 📇️ The canonical bootstrap folder mirror of the dev server's backbone: the frontier, reserve and owner shapes, their validation, and the reserve, stage, publish and retire steps over the per-folder sqlite store. */
import { createHash, randomBytes } from "node:crypto";
import { mkdirSync } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";
import { BACKBONE_ENDPOINT_PATH, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, backboneKindFromUri, decodeDocumentArchiveBytes } from "@semio-tech/framework-os";
import type { BackboneSqliteHandle } from "../🗃️sqlite-constructor/🟦️.ts";
import { backboneDbHandleFor, canonicalBootstrapMirrorError } from "../🗄️backbone-storage/🟦️.ts";

//#region BackboneVitePlugin
export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = `${BACKBONE_ENDPOINT_PATH}/canonical-bootstrap`;
export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES = DOCUMENT_ARCHIVE_MAXIMUM_BYTES;
export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES = CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES + 10;
const SHA256_HEX = /^[0-9a-f]{64}$/u;

export type CanonicalBootstrapFolderMirrorFrontierV1 = {
  readonly documentId: string;
  readonly headEditOrdinal: number;
  readonly headEditId: string;
  readonly lastCommitSeq: number;
  readonly chainSha256: string;
};

export type CanonicalBootstrapFolderMirrorReserveV1 = {
  readonly schema: "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1";
  readonly artifactSchema: string;
  readonly descriptorDigestV1: string;
  readonly aggregateSha256: string;
  readonly baselineFrontier: CanonicalBootstrapFolderMirrorFrontierV1;
};

export type CanonicalBootstrapFolderMirrorOwnerV1 = {
  readonly schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1";
  readonly epoch: number;
  readonly capability: string;
};

export type CanonicalBootstrapFolderMirrorControlV1 = { readonly epoch: number; readonly capability: string };

function canonicalBootstrapFolderMirrorDbPath(uri: string): string {
  const folder = uri.slice("folder://".length);
  if (backboneKindFromUri(uri) !== "folder" || !isAbsolute(folder)) throw canonicalBootstrapMirrorError("invalid");
  return join(folder, ".semio", "documents.db");
}

function validateCanonicalBootstrapDocumentId(documentId: string): void {
  if (documentId.length === 0 || new TextEncoder().encode(documentId).byteLength > 512 || documentId.includes("\0")) throw canonicalBootstrapMirrorError("invalid");
}

function validateCanonicalBootstrapFrontier(frontier: CanonicalBootstrapFolderMirrorFrontierV1, documentId: string): void {
  const chainIsCanonical = SHA256_HEX.test(frontier?.chainSha256 ?? "");
  const chainIsZero = chainIsCanonical && frontier.chainSha256 === "0".repeat(64);
  const genesis =
    frontier?.documentId === documentId &&
    frontier.headEditOrdinal === 0 &&
    frontier.headEditId === "" &&
    frontier.lastCommitSeq === 0 &&
    chainIsZero;
  const edited =
    frontier?.documentId === documentId &&
    Number.isSafeInteger(frontier.headEditOrdinal) &&
    frontier.headEditOrdinal > 0 &&
    Number.isSafeInteger(frontier.lastCommitSeq) &&
    frontier.lastCommitSeq > 0 &&
    typeof frontier.headEditId === "string" &&
    frontier.headEditId.length > 0 &&
    new TextEncoder().encode(frontier.headEditId).byteLength <= 512 &&
    !/\p{Cc}/u.test(frontier.headEditId) &&
    chainIsCanonical &&
    !chainIsZero;
  if (!genesis && !edited) throw canonicalBootstrapMirrorError("invalid");
}

function validateCanonicalBootstrapReserve(request: CanonicalBootstrapFolderMirrorReserveV1, documentId: string): void {
  if (
    request?.schema !== "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1" ||
    typeof request.artifactSchema !== "string" ||
    request.artifactSchema.length === 0 ||
    !SHA256_HEX.test(request.descriptorDigestV1) ||
    !SHA256_HEX.test(request.aggregateSha256)
  )
    throw canonicalBootstrapMirrorError("invalid");
  validateCanonicalBootstrapFrontier(request.baselineFrontier, documentId);
}

export function validateCanonicalBootstrapControl(control: CanonicalBootstrapFolderMirrorControlV1): void {
  if (!Number.isSafeInteger(control?.epoch) || control.epoch < 1 || typeof control.capability !== "string" || !SHA256_HEX.test(control.capability)) throw canonicalBootstrapMirrorError("invalid");
}

async function canonicalBootstrapFolderMirrorDb(uri: string): Promise<BackboneSqliteHandle> {
  const dbPath = canonicalBootstrapFolderMirrorDbPath(uri);
  mkdirSync(dirname(dbPath), { recursive: true });
  return backboneDbHandleFor(dbPath);
}

/** 🪪️ Mints and persists the sole current folder-mirror epoch; reserving a successor makes every prior stage and published pair non-current in the same transaction. */
export async function reserveCanonicalBootstrapFolderMirror(uri: string, documentId: string, request: CanonicalBootstrapFolderMirrorReserveV1): Promise<CanonicalBootstrapFolderMirrorOwnerV1> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapReserve(request, documentId);
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const capability = randomBytes(32).toString("hex");
  const reserve = db.transaction(() => {
    const prior = db.query("SELECT epoch FROM canonical_bootstrap_owner WHERE document_id = ?1").get(documentId) as { epoch?: number } | null;
    const epoch = Number(prior?.epoch ?? 0) + 1;
    if (!Number.isSafeInteger(epoch)) throw canonicalBootstrapMirrorError("conflict");
    db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1", [documentId]);
    db.run(
      "INSERT INTO canonical_bootstrap_owner (document_id, epoch, capability, state, artifact_schema, descriptor_digest_v1, aggregate_sha256, baseline_frontier_json, updated_at) VALUES (?1, ?2, ?3, 'reserved', ?4, ?5, ?6, ?7, ?8) ON CONFLICT(document_id) DO UPDATE SET epoch = excluded.epoch, capability = excluded.capability, state = excluded.state, artifact_schema = excluded.artifact_schema, descriptor_digest_v1 = excluded.descriptor_digest_v1, aggregate_sha256 = excluded.aggregate_sha256, baseline_frontier_json = excluded.baseline_frontier_json, updated_at = excluded.updated_at",
      [documentId, epoch, capability, request.artifactSchema, request.descriptorDigestV1, request.aggregateSha256, JSON.stringify(request.baselineFrontier), Date.now()],
    );
    return epoch;
  });
  return { schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1", epoch: reserve(), capability };
}

/** 🧱️ Stages one bounded, exactly framed canonical recursive archive without changing folder-visible state. */
export async function stageCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1, payload: Uint8Array): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  if (payload.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES) throw canonicalBootstrapMirrorError("too-large");
  let pack: Uint8Array, spr: Uint8Array;
  try {
    const archive = decodeDocumentArchiveBytes(payload);
    pack = Uint8Array.from(archive.parent_pack);
    spr = Uint8Array.from(archive.parent_spr);
  } catch {
    throw canonicalBootstrapMirrorError("invalid");
  }
  if (pack.byteLength + spr.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES) throw canonicalBootstrapMirrorError("too-large");
  const aggregate = createHash("sha256").update(pack).update(spr).digest("hex");
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const stage = db.transaction(() => {
    const owner = db.query("SELECT aggregate_sha256 AS aggregateSha256 FROM canonical_bootstrap_owner WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'").get(documentId, control.epoch, control.capability) as { aggregateSha256?: string } | null;
    if (owner?.aggregateSha256 !== aggregate) throw canonicalBootstrapMirrorError("conflict");
    db.run("INSERT INTO canonical_bootstrap_archive_stage (document_id, epoch, archive, aggregate_sha256) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(document_id, epoch) DO UPDATE SET archive = excluded.archive, aggregate_sha256 = excluded.aggregate_sha256", [documentId, control.epoch, payload, aggregate]);
  });
  stage();
}

/** 📣️ Makes a staged pair current only while the exact server-minted epoch remains reserved. */
export async function publishCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const publish = db.transaction(() => {
    const stage = db
      .query("SELECT 1 AS present FROM canonical_bootstrap_owner owner JOIN canonical_bootstrap_archive_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1 AND owner.epoch = ?2 AND owner.capability = ?3 AND owner.state = 'reserved' AND stage.aggregate_sha256 = owner.aggregate_sha256")
      .get(documentId, control.epoch, control.capability) as { present?: number } | null;
    if (stage?.present !== 1) throw canonicalBootstrapMirrorError("conflict");
    const changed = db.run("UPDATE canonical_bootstrap_owner SET state = 'published', updated_at = ?4 WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'", [documentId, control.epoch, control.capability, Date.now()]);
    if (changed.changes !== 1) throw canonicalBootstrapMirrorError("conflict");
  });
  publish();
}

/** 🧹️ Retires only the exact current epoch; a stale owner cannot hide or alter its successor. */
export async function retireCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const retire = db.transaction(() => {
    const changed = db.run("UPDATE canonical_bootstrap_owner SET state = 'retired', updated_at = ?4 WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state IN ('reserved', 'published')", [documentId, control.epoch, control.capability, Date.now()]);
    if (changed.changes !== 1) throw canonicalBootstrapMirrorError("conflict");
    db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1 AND epoch = ?2", [documentId, control.epoch]);
  });
  retire();
}
//#endregion BackboneVitePlugin
