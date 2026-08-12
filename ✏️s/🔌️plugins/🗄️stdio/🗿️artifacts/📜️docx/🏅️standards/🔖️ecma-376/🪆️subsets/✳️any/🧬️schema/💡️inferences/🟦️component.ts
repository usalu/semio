/** 💡️ Docx inference schema — document outline (paragraph/table/word counts). */

export interface DocxOutline {
  paragraphCount: number;
  tableCount: number;
  wordCount: number;
}

export interface DocxInference {
  /** @state inferred */
  outline: DocxOutline;
}
