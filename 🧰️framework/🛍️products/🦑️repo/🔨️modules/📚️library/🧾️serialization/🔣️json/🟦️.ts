type JsonRecord = Record<string, unknown>;

function canonicalArrayKey(value: unknown): string | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  const row = value as JsonRecord;
  const keys = ["operationId", "sourcePath", "path", "id", "destinationPath", "code", "relativeRoot", "structuredLocation"];
  const parts = keys.filter((key) => typeof row[key] === "string").map((key) => `${key}:${row[key] as string}`);
  return parts.length > 0 ? parts.join("\u0000") : null;
}

function canonicalValue(value: unknown): unknown {
  if (Array.isArray(value)) {
    const rows = value.map(canonicalValue);
    if (rows.every((row) => canonicalArrayKey(row) !== null)) return [...rows].sort((a, b) => Buffer.from(canonicalArrayKey(a) as string).compare(Buffer.from(canonicalArrayKey(b) as string)));
    return rows;
  }
  if (!value || typeof value !== "object") return value;
  const source = value as JsonRecord;
  const target: JsonRecord = Object.create(null);
  for (const key of Object.keys(source).sort()) {
    if (source[key] !== undefined) target[key] = canonicalValue(source[key]);
  }
  return target;
}

/** 🧾️ Serializes repository-owned records with recursively sorted keys and contract-identifier arrays. */
export function canonicalJson(value: unknown): string {
  const encoded = JSON.stringify(canonicalValue(value));
  if (encoded === undefined) throw new Error("Repository JSON requires a serializable top-level value");
  return encoded;
}
