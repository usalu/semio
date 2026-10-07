/** 🪆️ Persisted child identity excludes process-local materializations. */
import { hexLower, sha256 } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";
import { parseArtifactRef, type ArtifactRef } from "../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
export interface ArtifactChild { childId: string; target: ArtifactRef }

/** 🪆️ Decodes only the two fields that cross a child-document boundary. */
export function parseArtifactChild(value: unknown): ArtifactChild {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("artifact child must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 2 || !Object.hasOwn(row, "childId") || !Object.hasOwn(row, "target") || typeof row.childId !== "string") throw new Error("artifact child requires exactly childId and target");
  return { childId: row.childId, target: parseArtifactRef(row.target) };
}

/** 🆔️ TypeScript twin of Rust `store::content_id`: `<prefix>-` + the first 16 lowercase hex digits of SHA-256 over
 * `bytes` (the child schema's `ContentId`). */
export function contentId(prefix: string, bytes: Uint8Array): string {
  return `${prefix}-${hexLower(sha256(bytes).subarray(0, 8))}`;
}
