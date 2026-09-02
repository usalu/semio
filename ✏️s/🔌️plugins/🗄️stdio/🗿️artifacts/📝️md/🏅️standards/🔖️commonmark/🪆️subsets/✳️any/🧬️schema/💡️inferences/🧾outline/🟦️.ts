/** 🧾 `outline` — one named inference: this CommonMark document's own section/word structure. */

export interface MdHeadingEntry {
  level: number;
  text: string;
}

export interface MdOutline {
  sectionOutline: MdHeadingEntry[];
  blockCount: number;
  wordCount: number;
}
