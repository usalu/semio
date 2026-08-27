/** 🧬️ change-timestamp direct payload. */
import type { PngTimestamp } from '../../📸️snapshot/🟦️component.ts';
export interface ChangeTimestampMutation {
  readonly time?: PngTimestamp | null;
}
