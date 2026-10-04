/** 🧬️ Exact byte-authoritative BMP snapshot. */
export interface BmpSnapshot {
  schema: string;
  bytes: number[];
}

export class stdioBmpV3AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reject = (at: string, why: string): never => {
  throw new stdioBmpV3AnySnapshotGuardRefusal(at, why);
};

export function parseBmpSnapshot(value: unknown, at = "$"): BmpSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return reject(at, "value is not an object");
  const row = value as Record<string, unknown>;
  if (typeof row.schema !== "string") return reject(`${at}.schema`, "value is not a string");
  if (!Array.isArray(row.bytes)) return reject(`${at}.bytes`, "value is not an array");
  const bytes = row.bytes.map((item, index) => Number.isSafeInteger(item) && Number(item) >= 0 && Number(item) <= 255 ? Number(item) : reject(`${at}.bytes[${index}]`, "value is not a byte"));
  return { schema: row.schema, bytes };
}
