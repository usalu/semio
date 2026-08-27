/** 🧬 Transparent JsonMutation TypeScript aggregate. */
import type { SetMemberPayload } from './✏️set-member/🟦️component.ts';
import type { RemoveMemberPayload } from './🗑️remove-member/🟦️component.ts';
import type { InsertArrayElementPayload } from './📥️insert-array-element/🟦️component.ts';
import type { RemoveArrayElementPayload } from './🗑️remove-array-element/🟦️component.ts';
import type { SetScalarPayload } from './✏️set-scalar/🟦️component.ts';
export type JsonMutation =
  | { readonly mutation: 'set-member'; readonly payload: { readonly phase: 'apply'; readonly value: SetMemberPayload } }
  | { readonly mutation: 'remove-member'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveMemberPayload } }
  | { readonly mutation: 'insert-array-element'; readonly payload: { readonly phase: 'apply'; readonly value: InsertArrayElementPayload } }
  | { readonly mutation: 'remove-array-element'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveArrayElementPayload } }
  | { readonly mutation: 'set-scalar'; readonly payload: { readonly phase: 'apply'; readonly value: SetScalarPayload } };
