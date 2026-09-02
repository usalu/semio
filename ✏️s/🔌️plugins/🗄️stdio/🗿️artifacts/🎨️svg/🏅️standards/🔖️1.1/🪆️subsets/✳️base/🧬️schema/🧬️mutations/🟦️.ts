/** 🧬 Transparent SvgMutation TypeScript aggregate. `SvgMutation` carries
 * `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag values
 * are the camelCase form of the Rust variant names (e.g. `SetDeclaration` -> `"setDeclaration"`),
 * NOT the kebab-case `semanticKind` slugs this previously used for the tag value (confirmed by each
 * leaf's own `🔣️.json` manifest, which separates `aggregateVariant: "SetDeclaration"`
 * from `semanticKind: "set-declaration"` / `textOpcode`). */
import type { SetDeclarationPayload } from './✏️set-declaration/🟦️.ts';
import type { SetDoctypePayload } from './✏️set-doctype/🟦️.ts';
import type { InsertElementPayload } from './📥️insert-element/🟦️.ts';
import type { RemoveElementPayload } from './🗑️remove-element/🟦️.ts';
import type { SetElementNamePayload } from './✏️set-element-name/🟦️.ts';
import type { SetAttributePayload } from './✏️set-attribute/🟦️.ts';
import type { SetTextPayload } from './✏️set-text/🟦️.ts';
import type { SetViewBoxPayload } from './✏️set-view-box/🟦️.ts';
import type { SetTransformPayload } from './✏️set-transform/🟦️.ts';
export type SvgMutation =
  | { readonly mutation: 'setDeclaration'; readonly payload: { readonly phase: 'apply'; readonly value: SetDeclarationPayload } }
  | { readonly mutation: 'setDoctype'; readonly payload: { readonly phase: 'apply'; readonly value: SetDoctypePayload } }
  | { readonly mutation: 'insertElement'; readonly payload: { readonly phase: 'apply'; readonly value: InsertElementPayload } }
  | { readonly mutation: 'removeElement'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveElementPayload } }
  | { readonly mutation: 'setElementName'; readonly payload: { readonly phase: 'apply'; readonly value: SetElementNamePayload } }
  | { readonly mutation: 'setAttribute'; readonly payload: { readonly phase: 'apply'; readonly value: SetAttributePayload } }
  | { readonly mutation: 'setText'; readonly payload: { readonly phase: 'apply'; readonly value: SetTextPayload } }
  | { readonly mutation: 'setViewBox'; readonly payload: { readonly phase: 'apply'; readonly value: SetViewBoxPayload } }
  | { readonly mutation: 'setTransform'; readonly payload: { readonly phase: 'apply'; readonly value: SetTransformPayload } };
