/** 🧬️ SemioDocumentSnapshot — real TS mirror of the Rust snapshot shape (see the sibling
 * `🦀️.rs` for the source of truth). Block tree: Paragraph/Heading/List/Table/Code/
 * Quote/Image/PageBreak, discriminated on `kind`. */

export interface RunStyle {
  bold: boolean;
  italic: boolean;
  underline: boolean;
  size: number | null;
  font: string | null;
  color: string | null;
  link: string | null;
}

export interface DocRun {
  text: string;
  style: RunStyle;
}

export interface DocStyle {
  id: string;
  name: string;
  basedOn: string | null;
}

export interface DocImage {
  id: string;
  mime: string;
  bytes: number[];
}

export interface DocListItem {
  blocks: DocBlock[];
}

export interface DocTableCell {
  blocks: DocBlock[];
}

export interface DocTableRow {
  cells: DocTableCell[];
}

export type DocBlock =
  | { kind: "paragraph"; style_id: string | null; runs: DocRun[] }
  | { kind: "heading"; level: number; style_id: string | null; runs: DocRun[] }
  | { kind: "list"; ordered: boolean; items: DocListItem[] }
  | { kind: "table"; rows: DocTableRow[] }
  | { kind: "code"; language: string | null; text: string }
  | { kind: "quote"; blocks: DocBlock[] }
  | { kind: "image"; image_id: string; alt: string; width: number | null; height: number | null }
  | { kind: "pageBreak" };

export interface SemioDocumentSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ styles: DocStyle[];
  /** @state artifact */ images: DocImage[];
  /** @state artifact */ blocks: DocBlock[];
}
