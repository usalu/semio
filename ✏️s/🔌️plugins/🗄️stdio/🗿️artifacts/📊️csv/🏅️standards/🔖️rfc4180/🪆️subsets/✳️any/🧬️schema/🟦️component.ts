/** 🧬️ CsvArtifact schema facet — full artifact state, mirrors CsvSnapshot field-for-field
 * (see ./📸️snapshot/🟦️component.ts for CsvField/CsvRecord). */
import type { CsvRecord } from './📸️snapshot/🟦️component.ts';

export interface CsvArtifact {
  schema: string;
  hasHeader: boolean;
  records: CsvRecord[];
}
