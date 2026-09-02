/** 🧬️ insert-text-chunk direct payload. */
import type { PngTextChunk } from '../../📸️snapshot/🟦️.ts';
export interface InsertTextChunkMutation {
  readonly index: number;
  readonly chunk: PngTextChunk;
}
