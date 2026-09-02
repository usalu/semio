/** 🧬️ CsvArtifact schema facet — full artifact state, mirrors CsvSnapshot field-for-field
 * (see ./📸️snapshot/🟦️.ts for CsvField/CsvRecord). */
import type { CsvRecord } from './📸️snapshot/🟦️.ts';

export interface CsvArtifact {
  schema: string;
  hasHeader: boolean;
  records: CsvRecord[];
}
