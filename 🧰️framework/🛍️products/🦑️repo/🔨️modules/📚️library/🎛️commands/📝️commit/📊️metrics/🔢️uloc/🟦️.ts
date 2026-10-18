function countJsonKeysValue(v: unknown): number {
  if (v === null || typeof v !== "object") return 0;
  if (Array.isArray(v)) {
    let n = 0;
    for (const item of v) n += countJsonKeysValue(item);
    return n;
  }
  const o = v as Record<string, unknown>;
  let n = Object.keys(o).length;
  for (const vv of Object.values(o)) n += countJsonKeysValue(vv);
  return n;
}

/** 🧮️Counts JSON object keys recursively (aligned with repo `loc` Data JSON rules). */
export function countJsonKeys(text: string): number {
  const t = text.trim();
  if (!t) return 0;
  try {
    return countJsonKeysValue(JSON.parse(t));
  } catch {
    return 0;
  }
}

type JsonUlocUnits = Map<string, string>;

function jsonUlocUnitFingerprint(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) {
    return `array:${value.map((item) => (item !== null && typeof item === "object" ? (Array.isArray(item) ? "[]" : "{}") : JSON.stringify(item))).join("\\u0000")}`;
  }
  return `object:${Object.keys(value as Record<string, unknown>)
    .map((key) => JSON.stringify(key))
    .join("\\u0000")}`;
}

function collectJsonUlocUnits(value: unknown, path: string, units: JsonUlocUnits): void {
  if (Array.isArray(value)) {
    for (const [index, item] of value.entries()) collectJsonUlocUnits(item, `${path}/${index}`, units);
    return;
  }
  if (value === null || typeof value !== "object") return;
  for (const [key, item] of Object.entries(value as Record<string, unknown>)) {
    const nextPath = `${path}/${JSON.stringify(key)}`;
    units.set(nextPath, jsonUlocUnitFingerprint(item));
    collectJsonUlocUnits(item, nextPath, units);
  }
}

function jsonUlocUnits(text: string): JsonUlocUnits | null {
  try {
    const value: unknown = JSON.parse(text);
    const units: JsonUlocUnits = new Map();
    collectJsonUlocUnits(value, "", units);
    return units.size > 0 ? units : null;
  } catch {
    return null;
  }
}

/** 🧮️Counts JSON additions, semantic edits, and removals in the same key-based ULOC unit as totals. */
export function diffJsonUloc(before: string | null, after: string | null): { added: number; removed: number; edited: number } | null {
  const beforeUnits = before === null ? new Map() : jsonUlocUnits(before);
  const afterUnits = after === null ? new Map() : jsonUlocUnits(after);
  if (beforeUnits === null || afterUnits === null) return null;
  let added = 0;
  let removed = 0;
  let edited = 0;
  for (const [path, fingerprint] of afterUnits) {
    const previous = beforeUnits.get(path);
    if (previous === undefined) added++;
    else if (previous !== fingerprint) edited++;
  }
  for (const path of beforeUnits.keys()) if (!afterUnits.has(path)) removed++;
  return { added, removed, edited };
}

function physicalLineCount(text: string): number {
  if (!text.length) return 0;
  return text.split(/\r?\n/).length;
}

/** 📏️LOC for one tracked file body (JSON key count when applicable). */
export function countUnifiedLocForFile(rel: string, data: string): number {
  const ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
  if (ext === ".json" || ext === ".jsonc") {
    const keys = countJsonKeys(data);
    if (keys > 0) return keys;
    if (!data.trim()) return 0;
    return physicalLineCount(data);
  }
  return physicalLineCount(data);
}
