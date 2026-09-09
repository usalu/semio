/** 📸️ Persisted Note projection uses the same field parsers and nested domain types. */
import { parseNoteRecord, noteDocumentFields, type NoteBlockNode, type NoteImageAsset, type ArtifactLink } from "../🟦️.ts";
export type { NoteBlockNode, NoteImageAsset, ArtifactLink } from "../🟦️.ts";
export interface NoteSnapshot {
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
  assets?: Record<string, NoteImageAsset>;
  /** 🧬️ @state artifact */
  linkedArtifact?: ArtifactLink | null;
}

/** 📸️ Empty assets may be omitted by the native snapshot codec. */
export function parseNoteSnapshot(value: unknown, at = "$"): NoteSnapshot {
  return parseNoteRecord(value, noteDocumentFields, ["schema", "id", "blocks"], at) as unknown as NoteSnapshot;
}
