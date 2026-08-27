/** 🧬 Transparent TxtMutation TypeScript aggregate. */
import type { SetTrailingNewlinePayload } from './✏️set-trailing-newline/🟦️component.ts';
import type { SetLineEndingPayload } from './✏️set-line-ending/🟦️component.ts';
import type { InsertLinePayload } from './📥️insert-line/🟦️component.ts';
import type { RemoveLinePayload } from './🗑️remove-line/🟦️component.ts';
import type { SetLinePayload } from './✏️set-line/🟦️component.ts';
export type TxtMutation =
  | { readonly mutation: 'set-trailing-newline'; readonly payload: SetTrailingNewlinePayload }
  | { readonly mutation: 'set-line-ending'; readonly payload: SetLineEndingPayload }
  | { readonly mutation: 'insert-line'; readonly payload: InsertLinePayload }
  | { readonly mutation: 'remove-line'; readonly payload: RemoveLinePayload }
  | { readonly mutation: 'set-line'; readonly payload: SetLinePayload };
