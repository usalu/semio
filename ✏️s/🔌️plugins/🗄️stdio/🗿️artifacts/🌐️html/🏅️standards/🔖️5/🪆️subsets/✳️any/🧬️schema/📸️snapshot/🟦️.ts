/** 🧬️ HtmlSnapshot schema — own HtmlNode recursive tree model (own types, HTML is not XML). */
export interface HtmlAttr {
  name: string;
  /** `undefined` = valueless boolean attribute (e.g. `disabled`). */
  value?: string;
}

export type RawTextKind = 'script' | 'style';

export type HtmlNode =
  | { kind: 'element'; name: string; attributes: HtmlAttr[]; children: HtmlNode[] }
  | { kind: 'text'; text: string }
  | { kind: 'comment'; text: string }
  | { kind: 'rawText'; parentKind: RawTextKind; text: string };

export interface HtmlSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ doctype?: string;
  /** @state artifact */ root: HtmlNode;
}
