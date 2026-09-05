/** 🚪️ block5d → json — TypeScript mirror of the sibling `🦀️.rs` leaf's `json_text`.
 *
 * A genuine second implementation, not a re-export: the field tables below restate the Rust
 * struct declaration order, the writer restates `serde_json`'s escaping and `pack::json`'s float
 * lexeme rule, and `🧪️tests/🟦️.ts` asserts the result is byte-identical to the JSON the Rust leaf
 * produced for every `📚️examples/**\/🗣️.dsl.semio` fixture (`🧫️fixtures/*.json`).
 */

import type { Block5dSnapshot } from "../../../../../../../🧬️schema/📸️snapshot/🟦️";

// #region 🧬️SharedRecordTables
/** 🏷️ One JSON member of a record, in the exact order the Rust struct declares it. */
export type BlockJsonField =
  | Readonly<{ key: string; kind: "text" | "float" | "bool" | "textList" | "floatTuple" }>
  | Readonly<{ key: string; kind: "optionalText" | "optionalFloat" | "optionalFloatTuple" }>
  | Readonly<{ key: string; kind: "record"; fields: readonly BlockJsonField[] }>
  | Readonly<{ key: string; kind: "table"; fields: readonly BlockJsonField[] }>;

/** 🪪️ `BlockKindIdentity` (`✏️s/🔌️plugins/🧱️block/🦀️.rs`). */
export const BLOCK_KIND_IDENTITY_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "label", kind: "text" },
  { key: "variant", kind: "optionalText" },
  { key: "description", kind: "text" },
  { key: "icon", kind: "optionalText" },
  { key: "unit", kind: "optionalText" },
];

/** 🏷️ `BlockAttribute`. */
export const BLOCK_ATTRIBUTE_FIELDS: readonly BlockJsonField[] = [
  { key: "key", kind: "text" },
  { key: "value", kind: "text" },
  { key: "definition", kind: "optionalText" },
];

/** 👤️ `BlockAuthor`. */
export const BLOCK_AUTHOR_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "email", kind: "optionalText" },
];

/** 🔗️ `BlockCompatibilityRule`. */
export const BLOCK_COMPATIBILITY_RULE_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "source", kind: "text" },
  { key: "target", kind: "text" },
  { key: "bidirectional", kind: "bool" },
];

/** 🧱️ `BlockRepresentation`. */
export const BLOCK_REPRESENTATION_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "meshUrl", kind: "optionalText" },
  { key: "tags", kind: "textList" },
  { key: "lod", kind: "optionalText" },
  { key: "description", kind: "text" },
  { key: "attributes", kind: "table", fields: BLOCK_ATTRIBUTE_FIELDS },
];

/** 🎥️ `BlockCamera2d`. */
export const BLOCK_CAMERA_2D_FIELDS: readonly BlockJsonField[] = [
  { key: "x", kind: "float" },
  { key: "y", kind: "float" },
  { key: "zoom", kind: "float" },
];

/** 🎥️ `BlockCamera3d`. */
export const BLOCK_CAMERA_3D_FIELDS: readonly BlockJsonField[] = [
  { key: "position", kind: "floatTuple" },
  { key: "target", kind: "floatTuple" },
  { key: "zoom", kind: "float" },
];

/** 📝️ `BlockMeta`. */
export const BLOCK_META_FIELDS: readonly BlockJsonField[] = [{ key: "description", kind: "text" }];
// #endregion 🧬️SharedRecordTables

// #region 🧬️SubsetRecordTables
/** 🔵️ `Block5dPart2d`. */
export const BLOCK5D_PART_2D_FIELDS: readonly BlockJsonField[] = [
  { key: "shape", kind: "optionalText" },
  { key: "radius", kind: "optionalFloat" },
  { key: "width", kind: "optionalFloat" },
  { key: "height", kind: "optionalFloat" },
  { key: "color", kind: "optionalText" },
  { key: "iconKind", kind: "optionalText" },
];

/** 🧱️ `Block5dPart3d`. */
export const BLOCK5D_PART_3D_FIELDS: readonly BlockJsonField[] = [
  { key: "orientation", kind: "optionalFloatTuple" },
  { key: "scale", kind: "optionalFloatTuple" },
];

/** 🔘️ `Block5dGripKind`. */
export const BLOCK5D_GRIP_KIND_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "label", kind: "text" },
  { key: "color", kind: "text" },
  { key: "defaultRopeKind", kind: "text" },
];

/** 🌱️ `Block5dGripTemplate`. */
export const BLOCK5D_GRIP_TEMPLATE_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "gripKind", kind: "text" },
  { key: "angle", kind: "float" },
  { key: "radius2d", kind: "float" },
  { key: "position", kind: "floatTuple" },
  { key: "direction", kind: "floatTuple" },
  { key: "radius3d", kind: "float" },
];

/** 📸️ `Block5dSnapshot` — the member order the Rust struct declares, including the two
 * `#[value(rename = "2d"/"3d")]` members whose json keys are bare dimension names. */
export const BLOCK5D_SNAPSHOT_FIELDS: readonly BlockJsonField[] = [
  { key: "schema", kind: "text" },
  { key: "partKind", kind: "record", fields: BLOCK_KIND_IDENTITY_FIELDS },
  { key: "2d", kind: "record", fields: BLOCK5D_PART_2D_FIELDS },
  { key: "3d", kind: "record", fields: BLOCK5D_PART_3D_FIELDS },
  { key: "representations", kind: "table", fields: BLOCK_REPRESENTATION_FIELDS },
  { key: "gripKinds", kind: "table", fields: BLOCK5D_GRIP_KIND_FIELDS },
  { key: "grips", kind: "table", fields: BLOCK5D_GRIP_TEMPLATE_FIELDS },
  { key: "compatibility", kind: "table", fields: BLOCK_COMPATIBILITY_RULE_FIELDS },
  { key: "attributes", kind: "table", fields: BLOCK_ATTRIBUTE_FIELDS },
  { key: "authors", kind: "table", fields: BLOCK_AUTHOR_FIELDS },
  { key: "camera2d", kind: "record", fields: BLOCK_CAMERA_2D_FIELDS },
  { key: "camera3d", kind: "record", fields: BLOCK_CAMERA_3D_FIELDS },
  { key: "meta", kind: "record", fields: BLOCK_META_FIELDS },
];
// #endregion 🧬️SubsetRecordTables

// #region 🔢️FloatFormat
/**
 * 🔢️ `f64` → JSON lexeme, byte-identical to the Rust leaf's writer
 * (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs::write_float`): shortest round-tripping digits,
 * fixed notation while the decimal exponent is in `-5..=15` (a whole number always keeping an
 * explicit `.0` so it never collapses onto its integer twin), exponential with a signed `e±` suffix
 * otherwise. `Number.prototype.toExponential()` with no argument is specified to emit exactly the
 * shortest uniquely-identifying digit string, which is the same digit string Rust's `{:e}` picks.
 */
export function formatBlockFloat(value: number): string {
  if (!Number.isFinite(value)) return "null";
  if (value === 0) return Object.is(value, -0) ? "-0.0" : "0.0";
  const negative = value < 0;
  const [mantissa, exponentText] = Math.abs(value).toExponential().split("e");
  const digits = mantissa.replace(".", "");
  const exponent = Number(exponentText);
  const sign = negative ? "-" : "";
  if (exponent >= -5 && exponent <= 15) {
    if (exponent >= digits.length - 1) return `${sign}${digits}${"0".repeat(exponent - (digits.length - 1))}.0`;
    if (exponent >= 0) return `${sign}${digits.slice(0, exponent + 1)}.${digits.slice(exponent + 1)}`;
    return `${sign}0.${"0".repeat(-exponent - 1)}${digits}`;
  }
  const fraction = digits.length > 1 ? `.${digits.slice(1)}` : "";
  return `${sign}${digits[0]}${fraction}e${exponent >= 0 ? "+" : "-"}${Math.abs(exponent)}`;
}
// #endregion 🔢️FloatFormat

// #region 🔤️StringEscape
/**
 * 🔤️ JSON string literal, byte-identical to the Rust writer: only `"`, `\\` and the C0 controls are
 * escaped (`\b \f \n \r \t` shorthands, `\u00XX` otherwise); every non-ASCII character passes
 * through raw, matching `serde_json`'s default writer.
 */
export function writeBlockJsonString(text: string): string {
  let out = '"';
  for (const character of text) {
    switch (character) {
      case '"':
        out += '\\"';
        break;
      case "\\":
        out += "\\\\";
        break;
      case "\b":
        out += "\\b";
        break;
      case "\f":
        out += "\\f";
        break;
      case "\n":
        out += "\\n";
        break;
      case "\r":
        out += "\\r";
        break;
      case "\t":
        out += "\\t";
        break;
      default:
        out += character < " " ? `\\u${character.codePointAt(0)!.toString(16).padStart(4, "0")}` : character;
    }
  }
  return out + '"';
}
// #endregion 🔤️StringEscape

// #region ✍️Writer
/** ✍️ Writes one record as compact rfc8259, members in declaration order, optionals skipped. */
export function writeBlockJsonRecord(record: Record<string, unknown>, fields: readonly BlockJsonField[]): string {
  const members: string[] = [];
  for (const field of fields) {
    const value = record[field.key];
    if ((field.kind === "optionalText" || field.kind === "optionalFloat" || field.kind === "optionalFloatTuple") && (value === undefined || value === null)) continue;
    members.push(`${writeBlockJsonString(field.key)}:${writeBlockJsonValue(value, field)}`);
  }
  return `{${members.join(",")}}`;
}

function writeBlockJsonValue(value: unknown, field: BlockJsonField): string {
  switch (field.kind) {
    case "text":
    case "optionalText":
      return writeBlockJsonString(String(value ?? ""));
    case "float":
    case "optionalFloat":
      return formatBlockFloat(Number(value ?? 0));
    case "bool":
      return value ? "true" : "false";
    case "textList":
      return `[${((value ?? []) as string[]).map(writeBlockJsonString).join(",")}]`;
    case "floatTuple":
    case "optionalFloatTuple":
      return `[${((value ?? []) as number[]).map(formatBlockFloat).join(",")}]`;
    case "record":
      return writeBlockJsonRecord((value ?? {}) as Record<string, unknown>, field.fields);
    case "table":
      return `[${((value ?? []) as Record<string, unknown>[]).map((row) => writeBlockJsonRecord(row, field.fields)).join(",")}]`;
  }
}
// #endregion ✍️Writer

// #region 🎯️Entrypoint
/** 🧵️ `s.block.block5d@1/*` → `s.stdio.json@rfc8259/*` — the exact bytes the Rust `Block5dIntoJson` writes. */
export function block5dToJsonText(snapshot: Block5dSnapshot): string {
  return writeBlockJsonRecord(snapshot as unknown as Record<string, unknown>, BLOCK5D_SNAPSHOT_FIELDS);
}
// #endregion 🎯️Entrypoint
