/** 🧬️ HtmlArtifact schema — full artifact state, mirrors `HtmlSnapshot` field for field. See the
 * sibling `📸️snapshot/🟦️component.ts` for the canonical `HtmlNode`/`HtmlAttr`/`RawTextKind` shapes. */
import type { HtmlNode } from './📸️snapshot/🟦️component.ts';
export type { HtmlNode };

export interface HtmlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ doctype?: string;
  /** @state artifact */ root: HtmlNode;
}
