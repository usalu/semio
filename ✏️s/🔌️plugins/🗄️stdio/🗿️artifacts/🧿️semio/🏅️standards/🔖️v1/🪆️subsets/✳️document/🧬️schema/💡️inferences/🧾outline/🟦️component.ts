/** 🧾 `outline` — the semio document's heading/word/block-derived outline. */

export interface SemioDocumentHeadingEntry {
  level: number;
  text: string;
}

export interface SemioDocumentOutline {
  sectionOutline: SemioDocumentHeadingEntry[];
  blockCount: number;
  wordCount: number;
}
