/** ✒️ Owned VML drawing mutation payload. */
import { parseXmlDocument, type XmlDocument } from "../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export interface InsertVmlPart { readonly path: string; readonly document: XmlDocument; readonly index?: number; readonly override_index?: number }
export function parseInsertVmlPart(value: unknown): InsertVmlPart {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("expected VML payload");
  const record = value as Record<string, unknown>;
  const position = (name: string): number | undefined => {
    const found = record[name];
    if (found === undefined) return undefined;
    if (typeof found !== "number" || !Number.isSafeInteger(found) || found < 0) throw new Error("invalid VML payload fields");
    return found;
  };
  const index = position("index"), override_index = position("override_index");
  if (Object.keys(record).length !== 2 + (index === undefined ? 0 : 1) + (override_index === undefined ? 0 : 1) || typeof record.path !== "string" || !("document" in record)) throw new Error("invalid VML payload fields");
  return { path: record.path, document: parseXmlDocument(record.document), ...(index === undefined ? {} : { index }), ...(override_index === undefined ? {} : { override_index }) };
}
