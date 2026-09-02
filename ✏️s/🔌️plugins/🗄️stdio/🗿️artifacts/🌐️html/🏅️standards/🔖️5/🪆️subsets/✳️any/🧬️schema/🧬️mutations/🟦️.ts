// 🌳 `HtmlNode` is NOT redefined here -- see the sibling `../📸️snapshot/🟦️.ts`.
import type { HtmlNode } from '../📸️snapshot/🟦️.ts';
export type { HtmlNode };

/** 📸️ Opaque handle -- the full `HtmlSnapshot` shape lives in `../📸️snapshot/🟦️.ts`. */
export interface HtmlSnapshot {
  schema: string;
  doctype?: string;
  root: HtmlNode;
}

/** 🧭️ Path from the document root to a node: chain of child indices at each nesting level.
 * `[]` addresses the root itself. */
export type NodePath = number[];

/** 🧬️ HtmlMutation union. `insertNode`/`removeNode`'s `parent` addresses the PARENT element
 * (`index` is the position among its children); every other path-carrying variant's `path`
 * addresses the target node itself. `setAttribute`'s `value` is tri-state: absent = remove the
 * attribute, `null` = set/keep it valueless, string = set its value. */
export type HtmlMutation =
  | { mutation: 'setSnapshot'; snapshot: HtmlSnapshot }
  | { mutation: 'setDoctype'; doctype?: string }
  | { mutation: 'insertNode'; parent: NodePath; index: number; node: HtmlNode }
  | { mutation: 'removeNode'; parent: NodePath; index: number }
  | { mutation: 'setElementName'; path: NodePath; name: string }
  | { mutation: 'setAttribute'; path: NodePath; name: string; value?: string | null }
  | { mutation: 'setText'; path: NodePath; text: string }
  | { mutation: 'setComment'; path: NodePath; text: string }
  | { mutation: 'setRawText'; path: NodePath; text: string };
