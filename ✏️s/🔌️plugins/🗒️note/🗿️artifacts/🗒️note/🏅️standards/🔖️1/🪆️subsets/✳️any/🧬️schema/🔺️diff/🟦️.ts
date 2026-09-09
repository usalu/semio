/** 🔺️ Sparse Note document changes reuse the authored block, image and link contracts. */
import { parseNoteArtifact, parseNoteBlockNode, parseNoteImageAsset, parseNoteRecord, noteDocumentFields, noteString, noteNullable, noteArray, noteMap, type NoteValueParser, type NoteArtifact, type NoteBlockNode, type NoteImageAsset, type ArtifactLink } from "../🟦️.ts";
export type { NoteArtifact, NoteBlockNode, NoteImageAsset, ArtifactLink } from "../🟦️.ts";
export interface NoteDiff {
  /** 🧬️ @state artifact */
  artifact?: NoteArtifact | null;
  /** 🧬️ @state artifact */
  schema?: string | null;
  /** 🧬️ @state artifact */
  id?: string | null;
  /** 🧬️ @state artifact */
  title?: string | null;
  /** 🧬️ @state artifact */
  blocks?: NoteBlocksDelta | null;
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
  assets?: NoteAssetsDelta | null;
  /** 🧬️ @state artifact */
  linkedArtifact?: ArtifactLink | null;
}
export interface NoteAddedBlockEntry { parentId: string | null; index: number | null; block: NoteBlockNode }
export interface NoteBlockPatch { blockJson?: string | null }
export interface NoteBlockPatchEntry { id: string; patch: NoteBlockPatch }
export interface NoteBlocksDelta { added: NoteAddedBlockEntry[]; removed: string[]; patched: NoteBlockPatchEntry[]; reordered?: string[] | null }
export interface NoteAssetsDelta { entries: Record<string, NoteImageAsset | null> }

const index: NoteValueParser = (value, at) => { if (typeof value !== "number" || !Number.isInteger(value) || value < 0) throw new Error(`${at}: expected an unsigned index`); return value; };

/** ➕️ Parses identified insertion position and the complete added block. */
export function parseNoteAddedBlockEntry(value: unknown, at = "$"): NoteAddedBlockEntry {
  return parseNoteRecord(value, { parentId: noteNullable(noteString), index: noteNullable(index), block: parseNoteBlockNode }, ["parentId", "index", "block"], at) as unknown as NoteAddedBlockEntry;
}

/** 🩹️ Preserves the native block patch transport. */
export function parseNoteBlockPatch(value: unknown, at = "$"): NoteBlockPatch {
  return parseNoteRecord(value, { blockJson: noteNullable(noteString) }, [], at) as unknown as NoteBlockPatch;
}

/** 🪪️ Associates a sparse patch with its block identity. */
export function parseNoteBlockPatchEntry(value: unknown, at = "$"): NoteBlockPatchEntry {
  return parseNoteRecord(value, { id: noteString, patch: parseNoteBlockPatch }, ["id", "patch"], at) as unknown as NoteBlockPatchEntry;
}

/** 🗂️ Parses the native identified block collection delta. */
export function parseNoteBlocksDelta(value: unknown, at = "$"): NoteBlocksDelta {
  return parseNoteRecord(value, { added: noteArray(parseNoteAddedBlockEntry), removed: noteArray(noteString), patched: noteArray(parseNoteBlockPatchEntry), reordered: noteNullable(noteArray(noteString)) }, ["added", "removed", "patched"], at) as unknown as NoteBlocksDelta;
}

/** 🖼️ Parses image replacements and removals through the shared asset field owner. */
export function parseNoteAssetsDelta(value: unknown, at = "$"): NoteAssetsDelta {
  return parseNoteRecord(value, { entries: noteMap(noteNullable(parseNoteImageAsset)) }, ["entries"], at) as unknown as NoteAssetsDelta;
}

/** 🧾️ Parses only current document fields, preserving explicit nullable operations. */
export function parseNoteDiff(value: unknown, at = "$"): NoteDiff {
  const fields = Object.fromEntries(Object.entries(noteDocumentFields).map(([key, parse]) => [key, noteNullable(parse)]));
  return parseNoteRecord(value, { ...fields, artifact: noteNullable(parseNoteArtifact), blocks: noteNullable(parseNoteBlocksDelta), assets: noteNullable(parseNoteAssetsDelta) }, [], at) as unknown as NoteDiff;
}
