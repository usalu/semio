/** 🧬️ Transparent BmpMutation union. */
import type { ChangeHeaderFieldsMutation } from './📐️change-header-fields/🟦️component.ts';
import type { InsertPaletteEntryMutation } from './📥️insert-palette-entry/🟦️component.ts';
import type { RemovePaletteEntryMutation } from './📤️remove-palette-entry/🟦️component.ts';
import type { ReplacePaletteEntryMutation } from './🎨️replace-palette-entry/🟦️component.ts';
import type { ReplacePixelDataMutation } from './🟪️replace-pixel-data/🟦️component.ts';
export type BmpMutation =
  | { readonly mutation: 'change-header-fields'; readonly payload: ChangeHeaderFieldsMutation }
  | { readonly mutation: 'insert-palette-entry'; readonly payload: InsertPaletteEntryMutation }
  | { readonly mutation: 'remove-palette-entry'; readonly payload: RemovePaletteEntryMutation }
  | { readonly mutation: 'replace-palette-entry'; readonly payload: ReplacePaletteEntryMutation }
  | { readonly mutation: 'replace-pixel-data'; readonly payload: ReplacePixelDataMutation };
