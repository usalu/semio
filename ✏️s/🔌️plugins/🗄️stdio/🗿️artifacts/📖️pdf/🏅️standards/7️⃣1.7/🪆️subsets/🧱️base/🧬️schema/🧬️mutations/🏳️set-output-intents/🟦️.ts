/** 🏳️ Direct set-output-intents TypeScript payload. */
import type { PdfOutputIntent } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetOutputIntentsMutation {
  mutation: 'setOutputIntents';
  intents: PdfOutputIntent[];
}
