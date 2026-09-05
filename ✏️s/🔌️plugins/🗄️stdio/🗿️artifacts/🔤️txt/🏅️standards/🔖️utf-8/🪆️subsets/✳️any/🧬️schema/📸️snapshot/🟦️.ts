/** ⏎️ Which newline sequence terminates each line. */
export type LineEnding = 'lf' | 'crLf';

/** 🧬️ TxtSnapshot schema — a text file as a sequence of lines. */
export interface TxtSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ lines: string[];
  /** @state artifact */ trailingNewline: boolean;
  /** @state artifact */ lineEnding: LineEnding;
}
