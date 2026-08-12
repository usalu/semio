/** 💡️ Pptx inference schema — document outline (slide/shape/word counts). */

export interface PptxOutline {
  slideCount: number;
  shapeCount: number;
  wordCount: number;
}

export interface PptxInference {
  /** @state inferred */
  outline: PptxOutline;
}
