/** 🧬️ EpwArtifact schema facet — full artifact state, mirrors EpwSnapshot field-for-field
 * (see ./📸️snapshot/🟦️.ts for the shared shapes). */
import type { EpwSnapshot } from './📸️snapshot/🟦️.ts';
export interface EpwArtifact extends EpwSnapshot {}
