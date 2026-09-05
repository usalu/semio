/** 🧬️ Transparent TiffMutation union. */
import type { ChangeByteOrderMutation } from './🧭️change-byte-order/🟦️.ts';
import type { InsertIfdMutation } from './📥️insert-ifd/🟦️.ts';
import type { RemoveIfdMutation } from './📤️remove-ifd/🟦️.ts';
import type { ReplaceTagMutation } from './🏷️replace-tag/🟦️.ts';
import type { RemoveTagMutation } from './🗑️remove-tag/🟦️.ts';
import type { ReplacePixelsMutation } from './🔲️replace-pixels/🟦️.ts';
export type TiffMutation =
  | { readonly mutation: 'change-byte-order'; readonly payload: ChangeByteOrderMutation }
  | { readonly mutation: 'insert-ifd'; readonly payload: InsertIfdMutation }
  | { readonly mutation: 'remove-ifd'; readonly payload: RemoveIfdMutation }
  | { readonly mutation: 'replace-tag'; readonly payload: ReplaceTagMutation }
  | { readonly mutation: 'remove-tag'; readonly payload: RemoveTagMutation }
  | { readonly mutation: 'replace-pixels'; readonly payload: ReplacePixelsMutation };
