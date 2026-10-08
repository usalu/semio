/** 🧬️ Transparent TiffMutation union. */
import type { InsertIfdMutation } from './📥️insert-ifd/🟦️.ts';
import type { RemoveIfdMutation } from './📤️remove-ifd/🟦️.ts';
import type { ReplaceTagMutation } from './🏷️replace-tag/🟦️.ts';
import type { RemoveTagMutation } from './🗑️remove-tag/🟦️.ts';
import type { PaintRegionMutation } from './🎨️paint-region/🟦️.ts';
import type { ReplaceSamplesMutation } from './🧮️replace-samples/🟦️.ts';
export type TiffMutation =
  | { readonly mutation: 'insert-ifd'; readonly payload: InsertIfdMutation }
  | { readonly mutation: 'remove-ifd'; readonly payload: RemoveIfdMutation }
  | { readonly mutation: 'replace-tag'; readonly payload: ReplaceTagMutation }
  | { readonly mutation: 'remove-tag'; readonly payload: RemoveTagMutation }
  | { readonly mutation: 'paint-region'; readonly payload: PaintRegionMutation }
  | { readonly mutation: 'replace-samples'; readonly payload: ReplaceSamplesMutation }
