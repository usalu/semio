/** 🧬 Transparent SvgMutation TypeScript aggregate. */
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
  | { readonly mutation: 'set-declaration'; readonly payload: { readonly phase: 'apply'; readonly value: SetDeclarationPayload } }
  | { readonly mutation: 'set-doctype'; readonly payload: { readonly phase: 'apply'; readonly value: SetDoctypePayload } }
  | { readonly mutation: 'insert-element'; readonly payload: { readonly phase: 'apply'; readonly value: InsertElementPayload } }
  | { readonly mutation: 'remove-element'; readonly payload: { readonly phase: 'apply'; readonly value: RemoveElementPayload } }
  | { readonly mutation: 'set-element-name'; readonly payload: { readonly phase: 'apply'; readonly value: SetElementNamePayload } }
  | { readonly mutation: 'set-attribute'; readonly payload: { readonly phase: 'apply'; readonly value: SetAttributePayload } }
  | { readonly mutation: 'set-text'; readonly payload: { readonly phase: 'apply'; readonly value: SetTextPayload } }
  | { readonly mutation: 'set-view-box'; readonly payload: { readonly phase: 'apply'; readonly value: SetViewBoxPayload } }
  | { readonly mutation: 'set-transform'; readonly payload: { readonly phase: 'apply'; readonly value: SetTransformPayload } };
