/** 🧱 AddBlock payload mirror. */
import type { PlaybookBlockShape } from '../../🟦️component.ts';
export interface AddBlock {
  stepId: string;
  block: PlaybookBlockShape;
  index?: number;
}
