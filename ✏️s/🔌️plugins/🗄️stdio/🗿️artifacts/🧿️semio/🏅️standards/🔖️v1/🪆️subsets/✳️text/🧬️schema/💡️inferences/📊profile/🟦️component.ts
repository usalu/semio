/** 📊 `profile` — the semio text's own word/mark census + distinct languages used. */

export interface SemioTextProfile {
  wordCount: number;
  charCount: number;
  runCount: number;
  markCount: number;
  languages: string[];
}
