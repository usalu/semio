/** 🧬️ insert-unknown-chunk direct payload. */
import type { PngChunk } from '../../📸️snapshot/🟦️component.ts';
export interface InsertUnknownChunkMutation {
  readonly index: number;
  readonly chunk: PngChunk;
}
