/** 🧾 `outline` — the semio presentation's own heading/word structure. */

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
