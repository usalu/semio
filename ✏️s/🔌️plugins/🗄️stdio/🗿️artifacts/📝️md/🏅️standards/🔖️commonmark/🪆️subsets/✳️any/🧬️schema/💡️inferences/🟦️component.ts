/** 💡️ Md inference schema — document outline (heading outline, block/word counts). */

export interface MdHeadingEntry {
  level: number;
  text: string;
}

export interface MdOutline {
  sectionOutline: MdHeadingEntry[];
  blockCount: number;
  wordCount: number;
}

export interface MdInference {
  /** @state inferred */
  outline: MdOutline;
}
