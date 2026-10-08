/** 🧬 Transparent XmlMutation TypeScript aggregate. `XmlMutation` carries
 * `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag values
 * are the camelCase form of the Rust variant names, NOT the kebab-case `semanticKind` slugs this
 * previously used for the tag value. */
import type { SetDeclarationPayload } from './📣️set-declaration/🟦️.ts';
import type { SetDoctypePayload } from './📜️set-doctype/🟦️.ts';
import type { InsertElementPayload } from './📥️insert-element/🟦️.ts';
import type { RemoveElementPayload } from './🗑️remove-element/🟦️.ts';
import type { SetAttributePayload } from './🏷️set-attribute/🟦️.ts';
import type { SetTextPayload } from './✍️set-text/🟦️.ts';
export type XmlMutation =
  | { readonly mutation: 'setDeclaration'; readonly payload: SetDeclarationPayload }
  | { readonly mutation: 'setDoctype'; readonly payload: SetDoctypePayload }
  | { readonly mutation: 'insertElement'; readonly payload: InsertElementPayload }
  | { readonly mutation: 'removeElement'; readonly payload: RemoveElementPayload }
  | { readonly mutation: 'setAttribute'; readonly payload: SetAttributePayload }
  | { readonly mutation: 'setText'; readonly payload: SetTextPayload };
