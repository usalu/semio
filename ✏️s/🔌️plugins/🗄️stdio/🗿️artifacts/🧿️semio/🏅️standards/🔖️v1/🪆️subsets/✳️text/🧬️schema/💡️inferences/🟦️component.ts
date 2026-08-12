/** 💡️ Semio text inference schema — word/mark census + distinct languages used. */

export interface SemioTextProfile {
  wordCount: number;
  charCount: number;
  runCount: number;
  markCount: number;
  languages: string[];
}

export interface SemioTextInference {
  /** @state inferred */
  profile: SemioTextProfile;
}
