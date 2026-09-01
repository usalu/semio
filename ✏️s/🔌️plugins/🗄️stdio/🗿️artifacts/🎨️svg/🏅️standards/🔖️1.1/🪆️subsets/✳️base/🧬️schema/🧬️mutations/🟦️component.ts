/** 🧬 Transparent SvgMutation TypeScript aggregate. `SvgMutation` carries
 * `#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]`, so the tag values
 * are the camelCase form of the Rust variant names (e.g. `SetDeclaration` -> `"setDeclaration"`),
 * NOT the kebab-case `semanticKind` slugs this previously used for the tag value (confirmed by each
 * leaf's own `🔣️component.json` manifest, which separates `aggregateVariant: "SetDeclaration"`
 * from `semanticKind: "set-declaration"` / `textOpcode`). */
import type { SetDeclarationPayload } from './✏️set-declaration/🟦️component.ts';
import type { SetDoctypePayload } from './✏️set-doctype/🟦️component.ts';
import type { InsertElementPayload } from './📥️insert-element/🟦️component.ts';
import type { RemoveElementPayload } from './🗑️remove-element/🟦️component.ts';
import type { SetElementNamePayload } from './✏️set-element-name/🟦️component.ts';
import type { SetAttributePayload } from './✏️set-attribute/🟦️component.ts';
import type { SetTextPayload } from './✏️set-text/🟦️component.ts';
import type { SetViewBoxPayload } from './✏️set-view-box/🟦️component.ts';
import type { SetTransformPayload } from './✏️set-transform/🟦️component.ts';
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
