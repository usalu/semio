// 🌳 `SvgSnapshot.doc` wraps an `XmlDocument`, and `SvgDiff`/`SvgNodeDiff` diff that same node
// tree directly, per the plan's spec-mandated-reuse rule -- svg embeds xml's NODE model (real
// import, the canonical shape), but declares its own DIFF types below.
import type { XmlDeclaration, XmlDoctype, XmlNode } from '../../../../../../../📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️component.ts';
export type { XmlDeclaration, XmlDoctype, XmlNode };

/** 🔺️ Diff for `stdio.svg`. `declaration`/`doctype` are tri-state (`null` = cleared, absent =
 * unchanged, present = set). No `snapshot`-shaped full-replace field anywhere -- even a
 * `setSnapshot` mutation's diff is the sparse field-by-field delta below. */
export interface SvgDiff {
  prolog?: XmlNode[];
  declaration?: XmlDeclaration | null;
  doctype?: XmlDoctype | null;
  root?: SvgNodeDiff;
}

/** 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets. */
export type SvgNodeDiff =
  | { kind: 'element'; name?: string; attributes?: SvgAttributesDiff; children?: SvgChildrenDiff }
  | { kind: 'text'; text?: string }
  | { kind: 'replace'; node?: XmlNode };

/** 🏷️ Name-keyed, order-preserving attribute triple. */
export interface SvgAttributesDiff {
  removed: string[];
  modified: SvgAttrModified[];
  added: SvgAttrAdded[];
}

export interface SvgAttrModified {
  name: string;
  value: string;
}

export interface SvgAttrAdded {
  index: number;
  name: string;
  value: string;
}

/** 🌳 Index-keyed, recursive children triple. */
export interface SvgChildrenDiff {
  removed: number[];
  modified: SvgChildModified[];
  added: SvgChildAdded[];
}

export interface SvgChildModified {
  index: number;
  diff: SvgNodeDiff;
}

export interface SvgChildAdded {
  index: number;
  item: XmlNode;
}
