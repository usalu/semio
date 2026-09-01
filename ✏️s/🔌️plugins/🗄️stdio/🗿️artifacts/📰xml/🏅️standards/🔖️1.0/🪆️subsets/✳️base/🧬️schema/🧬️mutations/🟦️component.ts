/** 🧬 Transparent XmlMutation TypeScript aggregate. `XmlMutation` carries
 * `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag values
 * are the camelCase form of the Rust variant names, NOT the kebab-case `semanticKind` slugs this
 * previously used for the tag value. */
import type { SetDeclarationPayload } from './✏️set-declaration/🟦️component.ts';
import type { SetDoctypePayload } from './✏️set-doctype/🟦️component.ts';
import type { InsertElementPayload } from './📥️insert-element/🟦️component.ts';
import type { RemoveElementPayload } from './🗑️remove-element/🟦️component.ts';
import type { SetAttributePayload } from './✏️set-attribute/🟦️component.ts';
import type { SetTextPayload } from './✏️set-text/🟦️component.ts';
export type XmlMutation =
  | { readonly mutation: 'setDeclaration'; readonly payload: { readonly phase: 'apply'; readonly value: SetDeclarationPayload } }
  | { readonly mutation: 'setDoctype'; readonly payload: { readonly phase: 'apply'; readonly value: SetDoctypePayload } }
  | { readonly mutation: 'insertElement'; readonly payload: { readonly phase: 'apply'; readonly value: InsertElementPayload } }
  | { readonly mutation: 'removeElement'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveElementPayload } }
  | { readonly mutation: 'setAttribute'; readonly payload: { readonly phase: 'apply'; readonly value: SetAttributePayload } }
  | { readonly mutation: 'setText'; readonly payload: { readonly phase: 'apply'; readonly value: SetTextPayload } };
