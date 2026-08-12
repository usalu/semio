/** 💡️ Note inference schema — block outline (flattened block names + word/block counts). */

export interface NoteOutline {
  sectionOutline: string[];
  blockCount: number;
  wordCount: number;
}

export interface NoteInference {
  /** @state inferred */
  outline: NoteOutline;
}
