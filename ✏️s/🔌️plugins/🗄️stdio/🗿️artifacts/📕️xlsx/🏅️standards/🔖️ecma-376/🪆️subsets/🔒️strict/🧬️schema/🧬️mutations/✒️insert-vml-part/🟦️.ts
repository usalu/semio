/** ✒️ Owned VML drawing mutation payload. */
import { parseXmlDocument, type XmlDocument } from "../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export interface InsertVmlPart { readonly path: string; readonly document: XmlDocument; readonly index?: number }
export function parseInsertVmlPart(value: unknown): InsertVmlPart {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("expected VML payload");
  const record = value as Record<string, unknown>;
  const index = record.index;
  if (Object.keys(record).length !== (index === undefined ? 2 : 3) || typeof record.path !== "string" || !("document" in record) || (index !== undefined && (typeof index !== "number" || !Number.isSafeInteger(index) || index < 0))) throw new Error("invalid VML payload fields");
  return index === undefined ? { path: record.path, document: parseXmlDocument(record.document) } : { path: record.path, document: parseXmlDocument(record.document), index };
}
