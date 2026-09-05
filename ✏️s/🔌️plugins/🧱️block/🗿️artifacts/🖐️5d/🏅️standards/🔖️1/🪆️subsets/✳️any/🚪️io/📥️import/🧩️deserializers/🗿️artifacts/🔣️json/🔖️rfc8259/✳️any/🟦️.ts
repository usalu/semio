/** 🚪️ block5d ← json — TypeScript mirror of the sibling `🦀️.rs` leaf's `from_json_text`, the
 * exact inverse of `block5dToJsonText` in the `📤️export` leaf (whose field tables it reuses — one
 * declaration of the schema per subset, never two).
 */

import type { Block5dSnapshot } from "../../../../../../../🧬️schema/📸️snapshot/🟦️";
import { type BlockJsonField, BLOCK5D_SNAPSHOT_FIELDS, writeBlockJsonRecord } from "../../../../../../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";

// #region 📥️Reader
/** 🧩️ Rebuilds one record from a parsed JSON object, dropping members the schema does not declare
 * and defaulting every absent non-optional member the way the Rust `FromValue` derive does. */
export function readBlockJsonRecord(value: unknown, fields: readonly BlockJsonField[]): Record<string, unknown> {
  const source = (value ?? {}) as Record<string, unknown>;
  const record: Record<string, unknown> = {};
  for (const field of fields) {
    const member = source[field.key];
    switch (field.kind) {
      case "text":
        record[field.key] = typeof member === "string" ? member : "";
        break;
      case "optionalText":
        if (typeof member === "string") record[field.key] = member;
        break;
      case "float":
        record[field.key] = typeof member === "number" ? member : 0;
        break;
      case "optionalFloat":
        if (typeof member === "number") record[field.key] = member;
        break;
      case "bool":
        record[field.key] = member === true;
        break;
      case "textList":
        record[field.key] = Array.isArray(member) ? member.map(String) : [];
        break;
      case "floatTuple":
        record[field.key] = Array.isArray(member) ? member.map(Number) : [];
        break;
      case "optionalFloatTuple":
        if (Array.isArray(member)) record[field.key] = member.map(Number);
        break;
      case "record":
        record[field.key] = readBlockJsonRecord(member, field.fields);
        break;
      case "table":
        record[field.key] = (Array.isArray(member) ? member : []).map((row) => readBlockJsonRecord(row, field.fields));
        break;
    }
  }
  return record;
}
// #endregion 📥️Reader

// #region 🎯️Entrypoint
/** 🧩️ `s.stdio.json@rfc8259/*` → `s.block.block5d@1/*`. Throws on text that is not a JSON object. */
export function block5dFromJsonText(text: string): Block5dSnapshot {
  const parsed: unknown = JSON.parse(text);
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error("json→block5d: expected a json object at the document root");
  }
  const snapshot = readBlockJsonRecord(parsed, BLOCK5D_SNAPSHOT_FIELDS) as unknown as Block5dSnapshot;
  if (!snapshot.schema) snapshot.schema = "block.5d";
  return snapshot;
}

/** 🔁️ Canonical re-encoding of arbitrary `s.block.block5d` json — the fixed point the parity test asserts. */
export function block5dCanonicalJsonText(text: string): string {
  return writeBlockJsonRecord(block5dFromJsonText(text) as unknown as Record<string, unknown>, BLOCK5D_SNAPSHOT_FIELDS);
}
// #endregion 🎯️Entrypoint
