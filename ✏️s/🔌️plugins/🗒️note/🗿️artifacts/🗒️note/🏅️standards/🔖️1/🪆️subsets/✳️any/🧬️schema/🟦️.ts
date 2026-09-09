/** 🗒️ Authored Note fields and block payloads, using shared Store child and link identities. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseArtifactLink, type ArtifactLink } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactLink } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";

export interface NoteArtifact {
  /** 🧬️ @state artifact */
  schema: string;
  /** 🧬️ @state artifact */
  id: string;
  /** 🧬️ @state artifact */
  title?: string | null;
  /** 🧬️ @state artifact */
  blocks: NoteBlockNode[];
  /** 🧬️ @state artifact */
  gridVisible?: boolean | null;
  /** 🧬️ @state artifact */
  gridSpacing?: number | null;
  /** 🧬️ @state artifact */
  gridSubdivisions?: number | null;
  /** 🧬️ @state artifact */
  gridOpacity?: number | null;
  /** 🧬️ @state artifact */
  snapEnabled?: boolean | null;
  /** 🧬️ @state artifact */
  snapGridSpacing?: number | null;
  /** 🧬️ @state artifact */
  pencilWidth?: number | null;
  /** 🧬️ @state artifact */
  eraserRadius?: number | null;
  /** 🧬️ @state artifact */
  assets: Record<string, NoteImageAsset>;
  /** 🧬️ @state artifact */
  linkedArtifact?: ArtifactLink | null;
}

export interface NoteBlockFrame { id: string; name: string; x: number; y: number; width: number; height: number; rotation?: number; visible?: boolean; locked?: boolean }
export type NoteBlockNode = NoteBlockFrame & (
  { kind: "text"; content: NoteTextChild; fontSize: number; fontWeight: string; align: string } |
  { kind: "image"; imageKey: string } |
  { kind: "table"; columns: string[]; rows: NoteTableCell[][] } |
  { kind: "math"; tex: string; displayMode: boolean } |
  { kind: "stroke"; points: [number, number][]; strokeWidth: number; color: [number, number, number, number] } |
  { kind: "group"; children: NoteBlockNode[] }
);
export interface NoteImageAsset { mime: string; data: string; width?: number | null; height?: number | null }
export interface NoteTextChild { handle: ArtifactChild; paragraphs: NoteTextParagraph[] }
export interface NoteTextParagraph { runs: NoteTextRun[] }
export interface NoteTextRun { text: string; bold?: boolean | null; italic?: boolean | null; underline?: boolean | null; link?: string | null }
export interface NoteTableCell { content: string }
export type NoteValueParser = (value: unknown, at: string) => unknown;

/** 🪪️ Checks an exact object field set and validates every retained field through its owner. */
export function parseNoteRecord(value: unknown, parsers: Readonly<Record<string, NoteValueParser>>, required: readonly string[], at: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected an object`);
  const row = value as Record<string, unknown>;
  if (required.some((key) => !Object.hasOwn(row, key)) || Object.keys(row).some((key) => !Object.hasOwn(parsers, key))) throw new Error(`${at}: fields do not match the Note contract`);
  return Object.fromEntries(Object.entries(row).map(([key, value]) => [key, parsers[key](value, `${at}.${key}`)]));
}

export const noteString: NoteValueParser = (value, at) => { if (typeof value !== "string") throw new Error(`${at}: expected a string`); return value; };
export const noteNumber: NoteValueParser = (value, at) => { if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: expected a finite number`); return value; };
export const noteBoolean: NoteValueParser = (value, at) => { if (typeof value !== "boolean") throw new Error(`${at}: expected a boolean`); return value; };
export const noteNullable = (parse: NoteValueParser): NoteValueParser => (value, at) => value === null ? null : parse(value, at);
export const noteArray = (parse: NoteValueParser): NoteValueParser => (value, at) => {
  if (!Array.isArray(value)) throw new Error(`${at}: expected an array`);
  return value.map((item, index) => parse(item, `${at}[${index}]`));
};
export const noteMap = (parse: NoteValueParser): NoteValueParser => (value, at) => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a map`);
  return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, parse(item, `${at}.${key}`)]));
};
const tuple = (length: number): NoteValueParser => (value, at) => {
  if (!Array.isArray(value) || value.length !== length) throw new Error(`${at}: expected ${length} coordinates`);
  return value.map((item, index) => noteNumber(item, `${at}[${index}]`));
};

/** 🖼️ Parses image metadata and optional native dimensions. */
export function parseNoteImageAsset(value: unknown, at = "$"): NoteImageAsset {
  return parseNoteRecord(value, { mime: noteString, data: noteString, width: noteNullable(noteNumber), height: noteNullable(noteNumber) }, ["mime", "data"], at) as unknown as NoteImageAsset;
}

/** 📝️ Parses one authored text run while preserving its optional marks. */
export function parseNoteTextRun(value: unknown, at = "$"): NoteTextRun {
  return parseNoteRecord(value, { text: noteString, bold: noteNullable(noteBoolean), italic: noteNullable(noteBoolean), underline: noteNullable(noteBoolean), link: noteNullable(noteString) }, ["text"], at) as unknown as NoteTextRun;
}

/** ¶️ Parses one paragraph's native run sequence. */
export function parseNoteTextParagraph(value: unknown, at = "$"): NoteTextParagraph {
  return parseNoteRecord(value, { runs: noteArray(parseNoteTextRun) }, ["runs"], at) as unknown as NoteTextParagraph;
}

/** 🪆️ Parses a durable text child record and its shared composition handle. */
export function parseNoteTextChild(value: unknown, at = "$"): NoteTextChild {
  return parseNoteRecord(value, { handle: parseArtifactChild, paragraphs: noteArray(parseNoteTextParagraph) }, ["handle", "paragraphs"], at) as unknown as NoteTextChild;
}

/** 🧮️ Parses one table cell's authored text. */
export function parseNoteTableCell(value: unknown, at = "$"): NoteTableCell {
  return parseNoteRecord(value, { content: noteString }, ["content"], at) as unknown as NoteTableCell;
}

/** 🧱️ Parses the exact payload of all six native Note block variants. */
export function parseNoteBlockNode(value: unknown, at = "$"): NoteBlockNode {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a block`);
  const kind = (value as Record<string, unknown>).kind;
  const variants: Record<string, Record<string, NoteValueParser>> = {
    text: { content: parseNoteTextChild, fontSize: noteNumber, fontWeight: noteString, align: noteString },
    image: { imageKey: noteString },
    table: { columns: noteArray(noteString), rows: noteArray(noteArray(parseNoteTableCell)) },
    math: { tex: noteString, displayMode: noteBoolean },
    stroke: { points: noteArray(tuple(2)), strokeWidth: noteNumber, color: tuple(4) },
    group: { children: noteArray(parseNoteBlockNode) },
  };
  if (typeof kind !== "string" || !Object.hasOwn(variants, kind)) throw new Error(`${at}: unknown block kind`);
  const fields = { kind: noteString, id: noteString, name: noteString, x: noteNumber, y: noteNumber, width: noteNumber, height: noteNumber, rotation: noteNumber, visible: noteBoolean, locked: noteBoolean, ...variants[kind] };
  return parseNoteRecord(value, fields, ["kind", "id", "name", "x", "y", "width", "height", ...Object.keys(variants[kind])], at) as unknown as NoteBlockNode;
}

export const noteDocumentFields: Readonly<Record<string, NoteValueParser>> = {
  schema: noteString, id: noteString, title: noteNullable(noteString), blocks: noteArray(parseNoteBlockNode),
  gridVisible: noteNullable(noteBoolean), gridSpacing: noteNullable(noteNumber), gridSubdivisions: noteNullable(noteNumber), gridOpacity: noteNullable(noteNumber),
  snapEnabled: noteNullable(noteBoolean), snapGridSpacing: noteNullable(noteNumber), pencilWidth: noteNullable(noteNumber), eraserRadius: noteNullable(noteNumber),
  assets: noteMap(parseNoteImageAsset), linkedArtifact: noteNullable(parseArtifactLink),
};

/** 📄️ Parses the complete authored artifact without editor configuration or transient fields. */
export function parseNoteArtifact(value: unknown, at = "$"): NoteArtifact {
  return parseNoteRecord(value, noteDocumentFields, ["schema", "id", "blocks", "assets"], at) as unknown as NoteArtifact;
}
