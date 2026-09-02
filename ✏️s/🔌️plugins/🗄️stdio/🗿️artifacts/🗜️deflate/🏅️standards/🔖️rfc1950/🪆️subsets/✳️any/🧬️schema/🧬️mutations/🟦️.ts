/** 🧬️ DeflateMutation union — imperative verbs over the typed RFC1950 container fields. */
import type { DeflateSnapshot, DeflateLevelHint } from '../📸️snapshot/🟦️.ts';

export type DeflateMutation =
  | { mutation: 'setSnapshot'; snapshot: DeflateSnapshot }
  | { mutation: 'setCompressionParams'; method: number; windowBits: number; levelHint: DeflateLevelHint }
  | { mutation: 'setPresetDictionary'; dictId?: number }
  | { mutation: 'setPayload'; payload: number[] };
