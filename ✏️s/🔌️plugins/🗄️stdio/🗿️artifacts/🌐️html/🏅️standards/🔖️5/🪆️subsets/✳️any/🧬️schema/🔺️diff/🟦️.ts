// 🌳 `HtmlSnapshot.root` is an `HtmlNode`, and `HtmlDiff`/`HtmlNodeDiff` diff that same node tree
// directly -- own types throughout (HTML is not XML; only the general "recursive node tree"
// *structural pattern* is borrowed from svg/xml, per the ticket brief).
import type { HtmlNode, RawTextKind } from '../📸️snapshot/🟦️.ts';
export type { HtmlNode, RawTextKind };

/** 🔺️ Diff for `stdio.html`. `doctype` is tri-state (`null` = cleared, absent = unchanged,
 * present = set). No `snapshot`-shaped full-replace field anywhere -- even a `setSnapshot`
 * mutation's diff is the sparse field-by-field delta below. */
export interface HtmlDiff {
  doctype?: string | null;
  root?: HtmlNodeDiff;
}

/** 🌳 Recursive per-node diff, shaped like the `HtmlNode` it targets. */
export type HtmlNodeDiff =
  | { kind: 'element'; name?: string; attributes?: HtmlAttributesDiff; children?: HtmlChildrenDiff }
  | { kind: 'text'; text?: string }
  | { kind: 'comment'; text?: string }
  | { kind: 'rawText'; parentKind?: RawTextKind; text?: string }
  | { kind: 'replace'; node: HtmlNode };

/** 🏷️ Name-keyed, order-preserving attribute triple. */
export interface HtmlAttributesDiff {
  removed: string[];
  modified: HtmlAttrModified[];
  added: HtmlAttrAdded[];
}

export interface HtmlAttrModified {
  name: string;
  /** absent = the attribute is now valueless. */
  value?: string;
}

export interface HtmlAttrAdded {
  index: number;
  name: string;
  value?: string;
}

/** 🌳 Index-keyed, recursive children triple. */
export interface HtmlChildrenDiff {
  removed: number[];
  modified: HtmlChildModified[];
  added: HtmlChildAdded[];
}

export interface HtmlChildModified {
  index: number;
  diff: HtmlNodeDiff;
}

export interface HtmlChildAdded {
  index: number;
  item: HtmlNode;
}
