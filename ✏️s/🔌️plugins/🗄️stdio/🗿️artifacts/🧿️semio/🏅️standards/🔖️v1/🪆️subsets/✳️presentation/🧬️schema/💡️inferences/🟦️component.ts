/** 💡️ Semio presentation inference schema — heading outline + census over masters/layouts/slides. */

export interface SemioPresentationHeadingEntry {
  level: number;
  text: string;
}

export interface SemioPresentationOutline {
  sectionOutline: SemioPresentationHeadingEntry[];
  slideCount: number;
  shapeCount: number;
  blockCount: number;
  wordCount: number;
}

export interface SemioPresentationInference {
  /** @state inferred */
  outline: SemioPresentationOutline;
}
