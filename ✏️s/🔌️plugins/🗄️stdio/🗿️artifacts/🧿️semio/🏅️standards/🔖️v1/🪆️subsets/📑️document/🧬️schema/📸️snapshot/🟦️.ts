/** 🧬️ SemioDocumentSnapshot — real TS mirror of the Rust snapshot shape (see the sibling
 * `🦀️.rs` for the source of truth). Block tree: Paragraph/Heading/List/Table/Code/
 * Quote/Image/PageBreak, discriminated on `kind`. */

export interface RunStyle {
  bold: boolean;
  italic: boolean;
  underline: boolean;
  size?: number;
  font?: string;
  color?: string;
  link?: string;
}

export interface DocRun {
  text: string;
  style: RunStyle;
}

export interface DocStyle {
  id: string;
  name: string;
  basedOn?: string;
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
  | { kind: "paragraph"; styleId?: string; runs: DocRun[] }
  | { kind: "heading"; level: number; styleId?: string; runs: DocRun[] }
  | { kind: "list"; ordered: boolean; items: DocListItem[] }
  | { kind: "table"; rows: DocTableRow[] }
  | { kind: "code"; language?: string; text: string }
  | { kind: "quote"; blocks: DocBlock[] }
  | { kind: "image"; imageId: string; alt: string; width?: number; height?: number }
  | { kind: "pageBreak" };

export interface SemioDocumentSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ styles: DocStyle[];
  /** @state artifact */ images: DocImage[];
  /** @state artifact */ blocks: DocBlock[];
}
