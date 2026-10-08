/** 🔺️ Sparse Note document changes: assignable document scalars, ordered block-tree rows and keyed asset rows over the authored block, image and link contracts. */
import { parseNoteBlockNode, parseNoteImageAsset, parseNoteRecord, parseNoteTableCell, parseNoteTextChild, noteBoolean, noteString, noteNullable, noteNumber, noteArray, type NoteValueParser, type NoteBlockNode, type NoteImageAsset, type NoteTableCell, type NoteTextChild, type ArtifactLink } from "../🟦️.ts";
import {type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { NoteBlockNode, NoteImageAsset, NoteTableCell, NoteTextChild, ArtifactLink } from "../🟦️.ts";

/** 🎯️ An explicitly assigned optional value: assigning `null` stays distinct from leaving the slot untouched. */
export interface NoteAssigned<T> { value: T }
export interface NoteDiff {
  /** 🧬️ @state artifact */
  schema?: string | null;
  /** 🧬️ @state artifact */
  id?: string | null;
  /** 🧬️ @state artifact */
  title?: NoteAssigned<string | null> | null;
  /** 🧬️ @state artifact */
  blocks?: NoteBlocksDelta | null;
  /** 🧬️ @state artifact */
  gridVisible?: NoteAssigned<boolean | null> | null;
  /** 🧬️ @state artifact */
  gridSpacing?: NoteAssigned<Binary64 | null> | null;
  /** 🧬️ @state artifact */
  gridSubdivisions?: NoteAssigned<Binary64 | null> | null;
  /** 🧬️ @state artifact */
  gridOpacity?: NoteAssigned<Binary64 | null> | null;
  /** 🧬️ @state artifact */
  snapEnabled?: NoteAssigned<boolean | null> | null;
  /** 🧬️ @state artifact */
  snapGridSpacing?: NoteAssigned<Binary64 | null> | null;
  /** 🧬️ @state artifact */
  pencilWidth?: NoteAssigned<Binary64 | null> | null;
  /** 🧬️ @state artifact */
  eraserRadius?: NoteAssigned<Binary64 | null> | null;
  /** 🧬️ @state artifact */
  assets?: NoteAssetsDelta | null;
  /** 🧬️ @state artifact */
  linkedArtifact?: NoteAssigned<ArtifactLink | null> | null;
}
export type NoteTableEdit =
  | { edit: "insertRow"; index: number; cells: NoteTableCell[] }
  | { edit: "removeRow"; index: number }
  | { edit: "insertColumn"; index: number; name: string; cells: NoteTableCell[] }
  | { edit: "removeColumn"; index: number };
export interface NoteBlockPatch {
  name?: string | null; x?: Binary64 | null; y?: Binary64 | null; width?: Binary64 | null; height?: Binary64 | null; rotation?: Binary64 | null; visible?: boolean | null; locked?: boolean | null;
  content?: NoteTextChild | null; fontSize?: Binary64 | null; fontWeight?: string | null; align?: string | null; imageKey?: string | null;
  table?: NoteTableEdit[] | null; tex?: string | null; displayMode?: boolean | null; points?: [Binary64, Binary64][] | null; strokeWidth?: Binary64 | null; color?: [Binary64, Binary64, Binary64, Binary64] | null;
}
export type NoteBlockRow =
  | { row: "add"; parentId: string | null; index: number; block: NoteBlockNode }
  | { row: "remove"; id: string }
  | { row: "move"; id: string; parentId: string | null; index: number }
  | { row: "patch"; id: string; patch: NoteBlockPatch };
export interface NoteBlocksDelta { rows: NoteBlockRow[] }
export type NoteAssetRow = { row: "insert"; key: string; asset: NoteImageAsset } | { row: "replace"; key: string; asset: NoteImageAsset } | { row: "remove"; key: string };
export interface NoteAssetsDelta { rows: NoteAssetRow[] }

const index: NoteValueParser = (value, at) => { if (typeof value !== "number" || !Number.isInteger(value) || value < 0) throw new Error(`${at}: expected an unsigned index`); return value; };
const kind = <T extends string>(members: readonly T[]): NoteValueParser => (value, at) => { if (!members.includes(value as T)) throw new Error(`${at}: expected one of ${members.join(", ")}`); return value; };
const assigned = (parse: NoteValueParser): NoteValueParser => (value, at) => parseNoteRecord(value, { value: noteNullable(parse) }, ["value"], at);
const pair: NoteValueParser = (value, at) => noteArray(noteNumber)(value, at);

/** 📊️ Parses one ordered structural edit of a table block's grid. */
export function parseNoteTableEdit(value: unknown, at = "$"): NoteTableEdit {
  const row = parseNoteRecord(value, { edit: kind(["insertRow", "removeRow", "insertColumn", "removeColumn"] as const), index, name: noteString, cells: noteArray(parseNoteTableCell) }, ["edit", "index"], at);
  return row as unknown as NoteTableEdit;
}

/** 🩹️ Parses the sparse slots one block edit names. */
export function parseNoteBlockPatch(value: unknown, at = "$"): NoteBlockPatch {
  return parseNoteRecord(
    value,
    {
      name: noteNullable(noteString), x: noteNullable(noteNumber), y: noteNullable(noteNumber), width: noteNullable(noteNumber), height: noteNullable(noteNumber), rotation: noteNullable(noteNumber), visible: noteNullable(noteBoolean), locked: noteNullable(noteBoolean),
      content: noteNullable(parseNoteTextChild), fontSize: noteNullable(noteNumber), fontWeight: noteNullable(noteString), align: noteNullable(noteString), imageKey: noteNullable(noteString),
      table: noteNullable(noteArray(parseNoteTableEdit)), tex: noteNullable(noteString), displayMode: noteNullable(noteBoolean), points: noteNullable(noteArray(pair)), strokeWidth: noteNullable(noteNumber), color: noteNullable(noteArray(noteNumber)),
    },
    [],
    at,
  ) as unknown as NoteBlockPatch;
}

/** 🧱️ Parses one ordered block-tree row. */
export function parseNoteBlockRow(value: unknown, at = "$"): NoteBlockRow {
  return parseNoteRecord(value, { row: kind(["add", "remove", "move", "patch"] as const), parentId: noteNullable(noteString), index, block: parseNoteBlockNode, id: noteString, patch: parseNoteBlockPatch }, ["row"], at) as unknown as NoteBlockRow;
}

/** 🗂️ Parses the ordered block-tree delta. */
export function parseNoteBlocksDelta(value: unknown, at = "$"): NoteBlocksDelta {
  return parseNoteRecord(value, { rows: noteArray(parseNoteBlockRow) }, ["rows"], at) as unknown as NoteBlocksDelta;
}

/** 🖼️ Parses one keyed asset row. */
export function parseNoteAssetRow(value: unknown, at = "$"): NoteAssetRow {
  return parseNoteRecord(value, { row: kind(["insert", "replace", "remove"] as const), key: noteString, asset: parseNoteImageAsset }, ["row", "key"], at) as unknown as NoteAssetRow;
}

/** 🖼️ Parses the keyed asset delta. */
export function parseNoteAssetsDelta(value: unknown, at = "$"): NoteAssetsDelta {
  return parseNoteRecord(value, { rows: noteArray(parseNoteAssetRow) }, ["rows"], at) as unknown as NoteAssetsDelta;
}

/** 🧾️ Parses a note diff, preserving explicit nullable assignments. */
export function parseNoteDiff(value: unknown, at = "$"): NoteDiff {
  const scalar = (parse: NoteValueParser) => noteNullable(assigned(parse));
  return parseNoteRecord(
    value,
    {
      schema: noteNullable(noteString), id: noteNullable(noteString), title: scalar(noteString), blocks: noteNullable(parseNoteBlocksDelta),
      gridVisible: scalar(noteBoolean), gridSpacing: scalar(noteNumber), gridSubdivisions: scalar(noteNumber), gridOpacity: scalar(noteNumber), snapEnabled: scalar(noteBoolean), snapGridSpacing: scalar(noteNumber), pencilWidth: scalar(noteNumber), eraserRadius: scalar(noteNumber),
      assets: noteNullable(parseNoteAssetsDelta), linkedArtifact: scalar((link, where) => link),
    },
    [],
    at,
  ) as unknown as NoteDiff;
}
