/** 💡️ Pdf (1.4) inference schema — document outline (page count, word/char counts). */

export interface PdfOutline {
  pageCount: number;
  wordCount: number;
  charCount: number;
}

export interface PdfInference {
  /** @derived */
  outline: PdfOutline;
}
