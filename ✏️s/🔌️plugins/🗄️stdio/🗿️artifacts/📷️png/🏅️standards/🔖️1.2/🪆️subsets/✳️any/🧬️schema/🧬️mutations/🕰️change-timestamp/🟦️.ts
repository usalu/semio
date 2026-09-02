/** 🧬️ change-timestamp direct payload. */
import type { PngTimestamp } from '../../📸️snapshot/🟦️.ts';
export interface ChangeTimestampMutation {
  readonly time?: PngTimestamp | null;
}
