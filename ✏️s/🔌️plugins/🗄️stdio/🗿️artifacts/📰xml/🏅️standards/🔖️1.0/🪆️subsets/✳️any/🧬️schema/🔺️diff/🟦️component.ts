import type { XmlDeclaration, XmlNode } from '../📸️snapshot/🟦️component.ts';

/** 🔺️ Diff for `stdio.xml`. `declaration`/`doctype` are tri-state (`null` = cleared, absent =
 * unchanged, present = set). */
export interface XmlDiff {
  prolog?: XmlNode[];
  declaration?: XmlDeclaration | null;
  doctype?: string | null;
  root?: XmlNodeDiff;
}

/** 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets. */
export type XmlNodeDiff =
  | { kind: 'element'; name?: string; attributes?: XmlAttributesDiff; children?: XmlChildrenDiff }
  | { kind: 'text'; text?: string }
  | { kind: 'replace'; node?: XmlNode };

/** 🏷️ Name-keyed, order-preserving attribute triple. */
export interface XmlAttributesDiff {
  removed: string[];
  modified: XmlAttrModified[];
  added: XmlAttrAdded[];
}

export interface XmlAttrModified {
  name: string;
  value: string;
}

export interface XmlAttrAdded {
  index: number;
  name: string;
  value: string;
}

/** 🌳 Index-keyed, recursive children triple. */
export interface XmlChildrenDiff {
  removed: number[];
  modified: XmlChildModified[];
  added: XmlChildAdded[];
}

export interface XmlChildModified {
  index: number;
  diff: XmlNodeDiff;
}

export interface XmlChildAdded {
  index: number;
  item: XmlNode;
}
