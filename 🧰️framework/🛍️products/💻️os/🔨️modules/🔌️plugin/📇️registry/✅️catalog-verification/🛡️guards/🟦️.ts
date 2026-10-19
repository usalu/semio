/** 🛡️ Guards of the catalog verification: the bounded diagnostic text, the path containment check, the strict json value check, the descriptor enum normalisation and the duplicate json object name refusal. */
import { isAbsolute, relative } from "node:path";
import { CATALOG_DIAGNOSTIC_MAX_BYTES } from "../🟦️.ts";

//#region 🔖️CatalogCompleteness
/** 🧯️ Retains at most the public catalog diagnostic budget without splitting a Unicode scalar. */
export function boundedCatalogDiagnostic(value: unknown): string {
  const message = value instanceof Error ? value.message : String(value);
  if (Buffer.byteLength(message) <= CATALOG_DIAGNOSTIC_MAX_BYTES) return message;
  let output = "";
  for (const scalar of message) {
    if (Buffer.byteLength(output) + Buffer.byteLength(scalar) > CATALOG_DIAGNOSTIC_MAX_BYTES - 3) break;
    output += scalar;
  }
  return `${output}...`;
}

/** 🧭️ Returns true only when `candidate` is `root` or is structurally contained below it. */
export function pathIsWithin(root: string, candidate: string): boolean {
  const rel = relative(root, candidate);
  return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
}

/** 🧬️ Rejects lossy or unsupported JSON values before they enter the pack identity calculation. */
export function isStrictJsonValue(value: unknown): boolean {
  if (value === null || typeof value === "string" || typeof value === "boolean") return true;
  if (typeof value === "number") return Number.isFinite(value) && (!Number.isInteger(value) || Number.isSafeInteger(value));
  if (Array.isArray(value)) return value.every(isStrictJsonValue);
  if (typeof value !== "object") return false;
  return Object.values(value as Record<string, unknown>).every(isStrictJsonValue);
}

/** 🪞️ Normalizes serde's external enum tags and the pack codec's `kind`/`value` tags for comparison. */
export function normalizeCatalogDescriptorEnums(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(normalizeCatalogDescriptorEnums);
  if (value === null || typeof value !== "object") return value;
  const record = value as Record<string, unknown>;
  const keys = Object.keys(record);
  if (keys.length === 2 && keys.includes("kind") && keys.includes("value") && typeof record.kind === "string") return { [record.kind]: normalizeCatalogDescriptorEnums(record.value) };
  if (keys.length === 1 && keys[0] === "kind" && typeof record.kind === "string") return record.kind;
  return Object.fromEntries(Object.entries(record).map(([key, child]) => [key, normalizeCatalogDescriptorEnums(child)]));
}

const scalarToken = /(?:true|false|null|-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)/uy;

/** 🧭️ Rejects duplicated object names before the platform JSON decoder can collapse them. */
export function rejectDuplicateJsonObjectNames(source: string): void {
  let index = 0;
  const whitespace = (): void => {
    while (index < source.length) {
      const code = source.charCodeAt(index);
      if (code === 32 || code === 10 || code === 13 || code === 9 || ((code > 127 || code === 11 || code === 12) && /\s/u.test(source[index]!))) index++;
      else break;
    }
  };
  const string = (): string => {
    if (source[index] !== '"') throw new Error(`expected JSON string at byte ${index}`);
    const start = index++;
    while (index < source.length) {
      const character = source[index++]!;
      if (character === '"') return JSON.parse(source.slice(start, index)) as string;
      if (character === "\\") index++;
    }
    throw new Error("unterminated JSON string");
  };
  const value = (depth: number): void => {
    if (depth > 128) throw new Error("JSON nesting exceeds 128 levels");
    whitespace();
    if (source[index] === "{") {
      index++;
      whitespace();
      const names = new Set<string>();
      if (source[index] === "}") {
        index++;
        return;
      }
      while (true) {
        whitespace();
        const name = string();
        if (names.has(name)) throw new Error(`duplicate object field ${JSON.stringify(name)}`);
        names.add(name);
        whitespace();
        if (source[index++] !== ":") throw new Error(`expected JSON colon at byte ${index - 1}`);
        value(depth + 1);
        whitespace();
        const separator = source[index++];
        if (separator === "}") return;
        if (separator !== ",") throw new Error(`expected JSON object separator at byte ${index - 1}`);
      }
    }
    if (source[index] === "[") {
      index++;
      whitespace();
      if (source[index] === "]") {
        index++;
        return;
      }
      while (true) {
        value(depth + 1);
        whitespace();
        const separator = source[index++];
        if (separator === "]") return;
        if (separator !== ",") throw new Error(`expected JSON array separator at byte ${index - 1}`);
      }
    }
    if (source[index] === '"') {
      string();
      return;
    }
    scalarToken.lastIndex = index;
    const token = scalarToken.exec(source)?.[0];
    if (!token) throw new Error(`invalid JSON value at byte ${index}`);
    index += token.length;
  };
  value(0);
  whitespace();
  if (index !== source.length) throw new Error(`trailing JSON input at byte ${index}`);
}
//#endregion 🔖️CatalogCompleteness
