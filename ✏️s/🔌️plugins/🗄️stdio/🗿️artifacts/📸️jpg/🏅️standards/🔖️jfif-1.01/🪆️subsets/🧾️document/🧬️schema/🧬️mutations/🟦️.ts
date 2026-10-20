/** 🧬️ Transparent JpgMutation union. */
import type { ChangeJfifHeaderMutation } from './🪪️change-jfif-header/🟦️.ts';
import type { InsertOtherSegmentMutation } from './📥️insert-other-segment/🟦️.ts';
import type { RemoveOtherSegmentMutation } from './🗑️remove-other-segment/🟦️.ts';
import type { ReplacePixelsMutation } from './🔲️replace-pixels/🟦️.ts';
import type { ReplaceImage } from './🖼️replace-image/🟦️.ts';
export type JpgMutation =
  { readonly mutation: 'change-jfif-header'; readonly payload: ChangeJfifHeaderMutation }
  | { readonly mutation: 'insert-other-segment'; readonly payload: InsertOtherSegmentMutation }
  | { readonly mutation: 'remove-other-segment'; readonly payload: RemoveOtherSegmentMutation }
  | { readonly mutation: 'replace-pixels'; readonly payload: ReplacePixelsMutation }
  | { readonly mutation: 'replace-image'; readonly payload: ReplaceImage }
