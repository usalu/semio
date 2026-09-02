/** 🧬️ TsvArtifact schema facet — full artifact state, mirrors TsvSnapshot field-for-field. */
import type { TsvLineEnding } from './📸️snapshot/🟦️.ts';

export interface TsvArtifact {
  schema: string;
  records: string[][];
  trailingNewline: boolean;
  lineEnding: TsvLineEnding;
}
