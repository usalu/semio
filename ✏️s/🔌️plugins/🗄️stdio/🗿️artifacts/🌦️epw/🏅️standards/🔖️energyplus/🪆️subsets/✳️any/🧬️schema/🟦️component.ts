/** 🧬️ EpwArtifact schema facet — full artifact state, mirrors EpwSnapshot field-for-field
 * (see ./📸️snapshot/🟦️component.ts for the shared shapes). */
import type { EpwLocation, EpwDataPeriods, EpwRecord } from './📸️snapshot/🟦️component.ts';

export interface EpwArtifact {
  schema: string;
  location: EpwLocation;
  designConditions: string;
  typicalExtremePeriods: string;
  groundTemperatures: string;
  holidaysDst: string;
  comments1: string;
  comments2: string;
  dataPeriods: EpwDataPeriods;
  records: EpwRecord[];
}
