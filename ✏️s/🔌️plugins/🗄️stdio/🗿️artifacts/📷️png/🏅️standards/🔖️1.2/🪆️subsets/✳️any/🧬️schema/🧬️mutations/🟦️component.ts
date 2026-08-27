/** 🧬️ Transparent PngMutation union. */
import type { ChangeHeaderMutation } from './📐️change-header/🟦️component.ts';
import type { ReplacePaletteMutation } from './🎨️replace-palette/🟦️component.ts';
import type { ChangeTransparencyMutation } from './👁️change-transparency/🟦️component.ts';
import type { ChangeGammaMutation } from './🌗️change-gamma/🟦️component.ts';
import type { ChangeChromaticitiesMutation } from './🌈️change-chromaticities/🟦️component.ts';
import type { ChangeSrgbIntentMutation } from './🖌️change-srgb-intent/🟦️component.ts';
import type { ChangePhysicalDimsMutation } from './📏️change-physical-dims/🟦️component.ts';
import type { ChangeTimestampMutation } from './🕰️change-timestamp/🟦️component.ts';
import type { ChangeBackgroundMutation } from './🖼️change-background/🟦️component.ts';
import type { InsertTextChunkMutation } from './📥️insert-text-chunk/🟦️component.ts';
import type { RemoveTextChunkMutation } from './🗑️remove-text-chunk/🟦️component.ts';
import type { ReplaceTextChunkMutation } from './✏️replace-text-chunk/🟦️component.ts';
import type { ReplacePixelsMutation } from './🟪️replace-pixels/🟦️component.ts';
import type { InsertUnknownChunkMutation } from './📦️insert-unknown-chunk/🟦️component.ts';
import type { RemoveUnknownChunkMutation } from './📤️remove-unknown-chunk/🟦️component.ts';
export type PngMutation =
  | { readonly mutation: 'change-header'; readonly payload: ChangeHeaderMutation }
  | { readonly mutation: 'replace-palette'; readonly payload: ReplacePaletteMutation }
  | { readonly mutation: 'change-transparency'; readonly payload: ChangeTransparencyMutation }
  | { readonly mutation: 'change-gamma'; readonly payload: ChangeGammaMutation }
  | { readonly mutation: 'change-chromaticities'; readonly payload: ChangeChromaticitiesMutation }
  | { readonly mutation: 'change-srgb-intent'; readonly payload: ChangeSrgbIntentMutation }
  | { readonly mutation: 'change-physical-dims'; readonly payload: ChangePhysicalDimsMutation }
  | { readonly mutation: 'change-timestamp'; readonly payload: ChangeTimestampMutation }
  | { readonly mutation: 'change-background'; readonly payload: ChangeBackgroundMutation }
  | { readonly mutation: 'insert-text-chunk'; readonly payload: InsertTextChunkMutation }
  | { readonly mutation: 'remove-text-chunk'; readonly payload: RemoveTextChunkMutation }
  | { readonly mutation: 'replace-text-chunk'; readonly payload: ReplaceTextChunkMutation }
  | { readonly mutation: 'replace-pixels'; readonly payload: ReplacePixelsMutation }
  | { readonly mutation: 'insert-unknown-chunk'; readonly payload: InsertUnknownChunkMutation }
  | { readonly mutation: 'remove-unknown-chunk'; readonly payload: RemoveUnknownChunkMutation };
