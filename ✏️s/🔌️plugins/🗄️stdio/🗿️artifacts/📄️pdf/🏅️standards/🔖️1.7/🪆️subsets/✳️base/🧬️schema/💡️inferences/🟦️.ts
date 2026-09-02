/** 💡️ Pdf (1.7) inference schema — document outline (page count, word count, title). */

export interface Pdf17Outline {
  pageCount: number;
  wordCount: number;
  title?: string;
}

export interface Pdf17Inference {
  /** @derived */
  outline: Pdf17Outline;
}
