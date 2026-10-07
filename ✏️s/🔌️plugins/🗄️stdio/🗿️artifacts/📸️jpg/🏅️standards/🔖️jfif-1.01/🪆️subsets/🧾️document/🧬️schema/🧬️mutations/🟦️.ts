/** 🧬️ Transparent JpgMutation union. */
import type { SetSnapshot } from './📸️set-snapshot/🟦️.ts';
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';
import type { ChangeJfifHeaderMutation } from './🪪️change-jfif/🟦️.ts';
import type { ReplaceQuantTableMutation } from './📊️replace-quant/🟦️.ts';
import type { RemoveQuantTableMutation } from './🧹️remove-quant/🟦️.ts';
import type { ReplaceHuffmanTableMutation } from './🌳️replace-huffman/🟦️.ts';
import type { RemoveHuffmanTableMutation } from './🪓️remove-huffman/🟦️.ts';
import type { ChangeRestartIntervalMutation } from './🔁️change-restart/🟦️.ts';
import type { InsertOtherSegmentMutation } from './📥️insert-other/🟦️.ts';
import type { RemoveOtherSegmentMutation } from './🗑️remove-other/🟦️.ts';
import type { ReplacePixelsMutation } from './🔲️replace-pixels/🟦️.ts';
export type JpgMutation =
  | { readonly mutation: 'patch-snapshot'; readonly payload: { readonly patch: SnapshotPatch } }
  | { readonly mutation: 'change-jfif-header'; readonly payload: ChangeJfifHeaderMutation }
  | { readonly mutation: 'replace-quant-table'; readonly payload: ReplaceQuantTableMutation }
  | { readonly mutation: 'remove-quant-table'; readonly payload: RemoveQuantTableMutation }
  | { readonly mutation: 'replace-huffman-table'; readonly payload: ReplaceHuffmanTableMutation }
  | { readonly mutation: 'remove-huffman-table'; readonly payload: RemoveHuffmanTableMutation }
  | { readonly mutation: 'change-restart-interval'; readonly payload: ChangeRestartIntervalMutation }
  | { readonly mutation: 'insert-other-segment'; readonly payload: InsertOtherSegmentMutation }
  | { readonly mutation: 'remove-other-segment'; readonly payload: RemoveOtherSegmentMutation }
  | { readonly mutation: 'replace-pixels'; readonly payload: ReplacePixelsMutation }
  | { readonly mutation: 'set-snapshot'; readonly payload: SetSnapshot };
