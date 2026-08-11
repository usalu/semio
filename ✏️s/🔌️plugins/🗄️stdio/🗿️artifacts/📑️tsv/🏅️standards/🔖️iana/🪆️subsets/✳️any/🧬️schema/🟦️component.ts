/** 🧬️ TsvArtifact schema facet — full artifact state, mirrors TsvSnapshot field-for-field. */
import type { TsvLineEnding } from './📸️snapshot/🟦️component.ts';

export interface TsvArtifact {
  schema: string;
  records: string[][];
  trailingNewline: boolean;
  lineEnding: TsvLineEnding;
}
