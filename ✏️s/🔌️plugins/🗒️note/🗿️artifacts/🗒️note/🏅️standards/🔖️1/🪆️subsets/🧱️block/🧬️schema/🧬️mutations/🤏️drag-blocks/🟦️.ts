/** 🤏️ Note mutation — `DragBlocks` payload parser: a relative drag of several blocks (each with its subtree) by one offset. */
import type { DragBlocks } from "../../../../✳️any/🧬️schema/🧬️mutations/🟦️.ts";

/** 🚫️ Refuses a malformed payload with the JSON path of the offending value. */
const reject = (at: string, why: string): never => {
  throw new Error(`${at}: ${why}`);
};

/** 🔢️ A JSON number (JSON has no non-finite numbers, so the schema's `number` is exactly a finite one). */
const finite = (value: unknown, at: string): number => (typeof value === "number" && Number.isFinite(value) ? value : reject(at, "expected a finite number"));

/** 🎯️ The addressed block ids: at least one, none repeated. */
function parseIds(value: unknown, at: string): string[] {
  if (!Array.isArray(value) || value.length === 0) return reject(at, "expected at least one block id");
  const ids = value.map((item, index) => (typeof item === "string" ? item : reject(`${at}[${index}]`, "expected a block id")));
  if (new Set(ids).size !== ids.length) reject(at, "ids repeat a block");
  return ids;
}

/** 🤏️ Parses a tagged `dragBlocks` record exactly as the leaf schema admits it. */
export function parseDragBlocks(value: unknown, at = "$"): DragBlocks {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return reject(at, "expected an object");
  const row = value as Readonly<Record<string, unknown>>;
  for (const key of Object.keys(row)) if (!["mutation", "ids", "dx", "dy"].includes(key)) reject(`${at}.${key}`, "unknown field");
  if (row.mutation !== "dragBlocks") reject(`${at}.mutation`, "expected the dragBlocks tag");
  return { ids: parseIds(row.ids, `${at}.ids`), dx: finite(row.dx, `${at}.dx`), dy: finite(row.dy, `${at}.dy`) };
}
