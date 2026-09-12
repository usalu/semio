import { createHash } from "node:crypto";
import { canonicalJson } from "../../🟦️.ts";

export function taxonomyCliRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`${label} must be an object.`);
  return value as Record<string, unknown>;
}


export function taxonomyCliExactKeys(value: Record<string, unknown>, keys: readonly string[], label: string): void {
  const actual = Object.keys(value).sort();
  const expected = [...keys].sort();
  if (canonicalJson(actual) !== canonicalJson(expected)) throw new Error(`${label} must have exact keys ${expected.join(", ")}.`);
}


export function taxonomyCliSha256(value: string | Uint8Array): string {
  return createHash("sha256").update(value).digest("hex");
}


export function taxonomyCliCanonicalJson(value: unknown): string {
  if (Array.isArray(value)) return `[${Array.from(value, (row) => row === undefined ? "null" : taxonomyCliCanonicalJson(row)).join(",")}]`;
  if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    return `{${Object.keys(record).sort().filter((key) => record[key] !== undefined).map((key) => `${JSON.stringify(key)}:${taxonomyCliCanonicalJson(record[key])}`).join(",")}}`;
  }
  const encoded = JSON.stringify(value);
  if (encoded === undefined) throw new Error("Inventory shard JSON must contain serializable data.");
  return encoded;
}


export function taxonomyCliCanonicalArrayDigest(values: readonly unknown[]): string {
  const hash = createHash("sha256");
  hash.update("[");
  for (let index = 0; index < values.length; index += 1) {
    if (index > 0) hash.update(",");
    hash.update(taxonomyCliCanonicalJson(values[index]));
  }
  hash.update("]");
  return hash.digest("hex");
}


/** 🧱️ Yields canonical inventory bytes without encoding either large array as one value. */
export function* taxonomyInventoryCanonicalChunks(value: unknown): Generator<string> {
  const inventory = taxonomyCliRecord(value, "Inventory");
  const fields: { readonly key: string; readonly encoded?: string; readonly rows?: readonly unknown[] }[] = [];
  for (const key of Object.keys(inventory).sort()) {
    if (inventory[key] === undefined) continue;
    if ((key === "entries" || key === "violations") && Array.isArray(inventory[key])) {
      fields.push({ key, rows: inventory[key] as readonly unknown[] });
      continue;
    }
    const encoded = taxonomyCliCanonicalJson(inventory[key]);
    if (typeof encoded === "string") fields.push({ key, encoded });
  }
  yield "{";
  for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex += 1) {
    const field = fields[fieldIndex]!;
    if (fieldIndex > 0) yield ",";
    yield taxonomyCliCanonicalJson(field.key);
    yield ":";
    if (field.rows) {
      yield "[";
      const rows = field.rows;
      for (let rowIndex = 0; rowIndex < rows.length; rowIndex += 1) {
        if (rowIndex > 0) yield ",";
        yield taxonomyCliCanonicalJson(rows[rowIndex]);
      }
      yield "]";
    } else yield field.encoded!;
  }
  yield "}";
}


/** 🧮️ Hashes canonical inventory bytes incrementally without constructing the full JSON document. */
export function taxonomyInventoryIncrementalCanonicalDigest(value: unknown): string {
  const hash = createHash("sha256");
  for (const chunk of taxonomyInventoryCanonicalChunks(value)) hash.update(chunk);
  return hash.digest("hex");
}


export function taxonomyCliByteCompare(left: string, right: string): number {
  return Buffer.compare(Buffer.from(left), Buffer.from(right));
}


export function* taxonomyCliEntryViolations(entries: readonly Record<string, unknown>[]): Generator<unknown> {
  for (const entry of entries) {
    if (!Array.isArray(entry.violations)) throw new Error(`Inventory entry ${JSON.stringify(entry.sourcePath)} must have a violations array.`);
    yield* entry.violations;
  }
}


export function taxonomyCliStableViolations(values: Iterable<unknown>): readonly Record<string, unknown>[] {
  const unique = new Map<string, Record<string, unknown>>();
  for (const value of values) {
    const violation = taxonomyCliRecord(value, "Inventory violation");
    for (const key of ["path", "code", "severity", "message"] as const) if (typeof violation[key] !== "string") throw new Error(`Inventory violation ${key} must be a string.`);
    unique.set(`${violation.path}\0${violation.code}\0${violation.severity}\0${violation.message}`, violation);
  }
  return [...unique.values()].sort((left, right) => {
    for (const key of ["path", "code", "severity", "message"] as const) {
      const comparison = taxonomyCliByteCompare(String(left[key]), String(right[key]));
      if (comparison !== 0) return comparison;
    }
    return 0;
  });
}


export function taxonomyCliInventoryEntries(inventory: Record<string, unknown>): readonly Record<string, unknown>[] {
  if (!Array.isArray(inventory.entries)) throw new Error("Inventory entries must be an array.");
  return inventory.entries.map((value, index) => {
    const entry = taxonomyCliRecord(value, `Inventory entry ${index}`);
    if (typeof entry.sourcePath !== "string" || entry.sourcePath.length === 0 || entry.sourcePath !== entry.sourcePath.normalize("NFC")) throw new Error(`Inventory entry ${index} sourcePath must be non-empty NFC.`);
    if (typeof entry.ownerId !== "string" || entry.ownerId.length === 0 || entry.ownerId !== entry.ownerId.normalize("NFC")) throw new Error(`Inventory entry ${index} ownerId must be non-empty NFC.`);
    return entry;
  });
}


export function taxonomyCliInventoryMetadata(inventory: Record<string, unknown>): Readonly<Record<string, unknown>> {
  return Object.fromEntries(Object.entries(inventory).filter(([key]) => key !== "entries" && key !== "violations"));
}
