/** 💡️ Semio document inference schema — heading/word/block-derived document outline. */

export interface SemioDocumentHeadingEntry {
  level: number;
  text: string;
}

export interface SemioDocumentOutline {
  sectionOutline: SemioDocumentHeadingEntry[];
  blockCount: number;
  wordCount: number;
}

export interface SemioDocumentInference {
  /** @state inferred */
  outline: SemioDocumentOutline;
}
