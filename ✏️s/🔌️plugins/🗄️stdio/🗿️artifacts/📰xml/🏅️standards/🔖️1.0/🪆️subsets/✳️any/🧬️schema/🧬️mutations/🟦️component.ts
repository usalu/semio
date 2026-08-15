import type { XmlDeclaration, XmlDoctype, XmlNode, XmlSnapshot } from '../📸️snapshot/🟦️component.ts';

/** 🧭️ Path from the document root to a node: chain of child indices at each nesting level.
 * `[]` addresses the root itself. */
export type XmlNodePath = number[];

/** 🧬️ XmlMutation union. `insertElement`/`removeElement`'s `path` addresses the PARENT element
 * (`index` is the position among its children); every other path-carrying variant's `path`
 * addresses the target node itself. */
export type XmlMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: XmlSnapshot }
  | { mutation: 'setDeclaration'; declaration?: XmlDeclaration }
  | { mutation: 'setDoctype'; doctype?: XmlDoctype }
  | { mutation: 'insertElement'; path: XmlNodePath; index: number; node: XmlNode }
  | { mutation: 'removeElement'; path: XmlNodePath; index: number }
  | { mutation: 'setAttribute'; path: XmlNodePath; name: string; value?: string }
  | { mutation: 'setText'; path: XmlNodePath; text: string };
