/** 💡️ Txt inference schema — document outline (line/word/char counts). */

export interface TxtOutline {
  lineCount: number;
  wordCount: number;
  charCount: number;
}

export interface TxtInference {
  /** @state inferred */
  outline: TxtOutline;
}
