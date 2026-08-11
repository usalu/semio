/** ⏎️ Which newline sequence terminates each line. */
export type LineEnding = 'lf' | 'crLf';

/** 🧬️ TxtSnapshot schema — a text file as a sequence of lines. */
export interface TxtSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ lines: string[];
  /** @state persistent */ trailingNewline: boolean;
  /** @state persistent */ lineEnding: LineEnding;
}
