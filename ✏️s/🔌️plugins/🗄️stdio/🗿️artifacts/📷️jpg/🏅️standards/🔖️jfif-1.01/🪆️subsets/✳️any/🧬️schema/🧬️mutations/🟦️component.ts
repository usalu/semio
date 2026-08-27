/** 🧬️ Transparent JpgMutation union. */
import type { ChangeJfifHeaderMutation } from './📐️change-jfif-header/🟦️component.ts';
import type { ReplaceQuantTableMutation } from './📊️replace-quant-table/🟦️component.ts';
import type { RemoveQuantTableMutation } from './📤️remove-quant-table/🟦️component.ts';
import type { ReplaceHuffmanTableMutation } from './🌳️replace-huffman-table/🟦️component.ts';
import type { RemoveHuffmanTableMutation } from './🪓️remove-huffman-table/🟦️component.ts';
import type { ChangeRestartIntervalMutation } from './🔁️change-restart-interval/🟦️component.ts';
import type { InsertOtherSegmentMutation } from './📥️insert-other-segment/🟦️component.ts';
import type { RemoveOtherSegmentMutation } from './🗑️remove-other-segment/🟦️component.ts';
import type { ReplacePixelsMutation } from './🟪️replace-pixels/🟦️component.ts';
import type { ChangeReEncodeQualityMutation } from './🎚️change-re-encode-quality/🟦️component.ts';
export type JpgMutation =
  | { readonly mutation: 'change-jfif-header'; readonly payload: ChangeJfifHeaderMutation }
  | { readonly mutation: 'replace-quant-table'; readonly payload: ReplaceQuantTableMutation }
  | { readonly mutation: 'remove-quant-table'; readonly payload: RemoveQuantTableMutation }
  | { readonly mutation: 'replace-huffman-table'; readonly payload: ReplaceHuffmanTableMutation }
  | { readonly mutation: 'remove-huffman-table'; readonly payload: RemoveHuffmanTableMutation }
  | { readonly mutation: 'change-restart-interval'; readonly payload: ChangeRestartIntervalMutation }
  | { readonly mutation: 'insert-other-segment'; readonly payload: InsertOtherSegmentMutation }
  | { readonly mutation: 'remove-other-segment'; readonly payload: RemoveOtherSegmentMutation }
  | { readonly mutation: 'replace-pixels'; readonly payload: ReplacePixelsMutation }
  | { readonly mutation: 'change-re-encode-quality'; readonly payload: ChangeReEncodeQualityMutation };
