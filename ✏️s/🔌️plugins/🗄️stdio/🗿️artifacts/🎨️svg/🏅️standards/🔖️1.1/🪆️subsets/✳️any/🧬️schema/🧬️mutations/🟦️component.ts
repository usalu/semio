// 🌳 svg embeds xml's NODE model directly (real import; spec-mandated reuse), never redefined
// here -- see xml's own `📸️snapshot/🟦️component.ts` for the canonical shape.
import type { XmlDeclaration, XmlDoctype, XmlNode } from '../../../../../../../📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️component.ts';
export type { XmlDeclaration, XmlDoctype, XmlNode };

/** 📸️ Opaque handle -- the full `SvgSnapshot` shape lives in `../📸️snapshot/🟦️component.ts`
 * (still a placeholder pending that facet's own rewrite; see this wave's report). */
export interface SvgSnapshot {
  schema: string;
  doc: unknown;
}

/** 🧭️ Path from the document root to a node: chain of child indices at each nesting level.
 * `[]` addresses the root itself. */
export type NodePath = number[];

export interface ViewBox {
  minX: number;
  minY: number;
  width: number;
  height: number;
}

export type TransformOp =
  | { op: 'translate'; x: number; y?: number }
  | { op: 'scale'; x: number; y?: number }
  | { op: 'rotate'; angle: number; center?: [number, number] }
  | { op: 'skewX'; angle: number }
  | { op: 'skewY'; angle: number }
  | { op: 'matrix'; a: number; b: number; c: number; d: number; e: number; f: number };

/** 🧬️ SvgMutation union. `insertElement`/`removeElement`'s `parent` addresses the PARENT
 * element (`index` is the position among its children); every other path-carrying variant's
 * `path` addresses the target node itself. */
export type SvgMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: SvgSnapshot }
  | { mutation: 'setDeclaration'; declaration?: XmlDeclaration }
  | { mutation: 'setDoctype'; doctype?: XmlDoctype }
  | { mutation: 'insertElement'; parent: NodePath; index: number; node: XmlNode }
  | { mutation: 'removeElement'; parent: NodePath; index: number }
  | { mutation: 'setElementName'; path: NodePath; name: string }
  | { mutation: 'setAttribute'; path: NodePath; name: string; value?: string }
  | { mutation: 'setText'; path: NodePath; text: string }
  | { mutation: 'setViewBox'; path: NodePath; viewBox?: ViewBox }
  | { mutation: 'setTransform'; path: NodePath; transform?: TransformOp[] };
