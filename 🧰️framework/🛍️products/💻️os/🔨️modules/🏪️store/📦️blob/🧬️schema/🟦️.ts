/** 📦️ Durable identity and metadata of a Store-owned content-addressed blob. */
export interface BlobRef { hash: string; size: bigint; mediaType: string }

/** 📦️ Admits blob metadata without payload bytes or process-local ownership. */
export function parseBlobRef(value: unknown): BlobRef {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("blob reference must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 3 || typeof row.hash !== "string" || typeof row.mediaType !== "string" || typeof row.size !== "bigint" || row.size < 0n || row.size > 18_446_744_073_709_551_615n) throw new Error("blob reference requires hash, unsigned size and mediaType");
  return { hash: row.hash, size: row.size, mediaType: row.mediaType };
}
