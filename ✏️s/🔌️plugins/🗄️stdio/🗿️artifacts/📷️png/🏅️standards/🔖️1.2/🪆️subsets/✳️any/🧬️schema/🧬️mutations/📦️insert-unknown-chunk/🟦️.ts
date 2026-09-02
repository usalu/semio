/** 🧬️ insert-unknown-chunk direct payload. */
import type { PngChunk } from '../../📸️snapshot/🟦️.ts';
export interface InsertUnknownChunkMutation {
  readonly index: number;
  readonly chunk: PngChunk;
}
