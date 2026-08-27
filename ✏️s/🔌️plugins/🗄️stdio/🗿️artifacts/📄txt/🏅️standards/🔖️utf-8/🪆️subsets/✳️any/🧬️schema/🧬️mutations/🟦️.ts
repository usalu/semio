/** 🧬 Transparent TxtMutation TypeScript aggregate. */
import type { SetTrailingNewlinePayload } from './↩️set-trailing-newline/🟦️.ts';
import type { SetLineEndingPayload } from './🔚️set-line-ending/🟦️.ts';
import type { InsertLinePayload } from './📥️insert-line/🟦️.ts';
import type { RemoveLinePayload } from './🗑️remove-line/🟦️.ts';
import type { SetLinePayload } from './✏️set-line/🟦️.ts';
export type TxtMutation =
  | { readonly mutation: 'set-trailing-newline'; readonly payload: SetTrailingNewlinePayload }
  | { readonly mutation: 'set-line-ending'; readonly payload: SetLineEndingPayload }
  | { readonly mutation: 'insert-line'; readonly payload: InsertLinePayload }
  | { readonly mutation: 'remove-line'; readonly payload: RemoveLinePayload }
  | { readonly mutation: 'set-line'; readonly payload: SetLinePayload };
