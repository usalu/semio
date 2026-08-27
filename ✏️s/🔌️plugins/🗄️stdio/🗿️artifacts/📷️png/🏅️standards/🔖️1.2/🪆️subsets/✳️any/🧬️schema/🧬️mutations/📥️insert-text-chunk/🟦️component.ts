/** 🧬️ insert-text-chunk direct payload. */
import type { PngTextChunk } from '../../📸️snapshot/🟦️component.ts';
export interface InsertTextChunkMutation {
  readonly index: number;
  readonly chunk: PngTextChunk;
}
