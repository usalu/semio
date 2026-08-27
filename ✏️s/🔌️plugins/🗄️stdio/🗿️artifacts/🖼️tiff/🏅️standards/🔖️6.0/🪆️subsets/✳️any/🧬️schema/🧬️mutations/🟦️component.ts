/** 🧬️ Transparent TiffMutation union. */
import type { ChangeByteOrderMutation } from './🧭️change-byte-order/🟦️component.ts';
import type { InsertIfdMutation } from './📥️insert-ifd/🟦️component.ts';
import type { RemoveIfdMutation } from './📤️remove-ifd/🟦️component.ts';
import type { ReplaceTagMutation } from './🏷️replace-tag/🟦️component.ts';
import type { RemoveTagMutation } from './🗑️remove-tag/🟦️component.ts';
import type { ReplacePixelsMutation } from './🟪️replace-pixels/🟦️component.ts';
export type TiffMutation =
  | { readonly mutation: 'change-byte-order'; readonly payload: ChangeByteOrderMutation }
  | { readonly mutation: 'insert-ifd'; readonly payload: InsertIfdMutation }
  | { readonly mutation: 'remove-ifd'; readonly payload: RemoveIfdMutation }
  | { readonly mutation: 'replace-tag'; readonly payload: ReplaceTagMutation }
  | { readonly mutation: 'remove-tag'; readonly payload: RemoveTagMutation }
  | { readonly mutation: 'replace-pixels'; readonly payload: ReplacePixelsMutation };
