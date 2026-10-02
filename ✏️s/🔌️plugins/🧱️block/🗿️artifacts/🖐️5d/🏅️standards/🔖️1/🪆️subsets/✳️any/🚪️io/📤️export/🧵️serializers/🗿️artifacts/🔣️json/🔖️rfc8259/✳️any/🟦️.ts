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

/** 📸️ `Block5dSnapshot` — the member order and canonical camel-case keys declared by Rust. */
export const BLOCK5D_SNAPSHOT_FIELDS: readonly BlockJsonField[] = [
  { key: "schema", kind: "text" },
  { key: "partKind", kind: "record", fields: BLOCK_KIND_IDENTITY_FIELDS },
  { key: "part2d", kind: "record", fields: BLOCK5D_PART_2D_FIELDS },
  { key: "part3d", kind: "record", fields: BLOCK5D_PART_3D_FIELDS },
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


export {block5dToJsonText} from '../../../../../../🔣️json/🟦️.ts';
