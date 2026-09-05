/** 🧬 Transparent JsonMutation TypeScript aggregate. `JsonMutation` carries
 * `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag values
 * are the camelCase form of the Rust variant names, NOT the kebab-case `semanticKind` slugs this
 * previously used for the tag value. */
import type { SetMemberPayload } from './✏️set-member/🟦️.ts';
import type { RemoveMemberPayload } from './🗑️remove-member/🟦️.ts';
import type { InsertArrayElementPayload } from './📥️insert-array-element/🟦️.ts';
import type { RemoveArrayElementPayload } from './📤️remove-array-element/🟦️.ts';
import type { SetScalarPayload } from './🔢️set-scalar/🟦️.ts';
export type JsonMutation =
  | { readonly mutation: 'setMember'; readonly payload: { readonly phase: 'apply'; readonly value: SetMemberPayload } }
  | { readonly mutation: 'removeMember'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveMemberPayload } }
  | { readonly mutation: 'insertArrayElement'; readonly payload: { readonly phase: 'apply'; readonly value: InsertArrayElementPayload } }
  | { readonly mutation: 'removeArrayElement'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveArrayElementPayload } }
  | { readonly mutation: 'setScalar'; readonly payload: { readonly phase: 'apply'; readonly value: SetScalarPayload } };
