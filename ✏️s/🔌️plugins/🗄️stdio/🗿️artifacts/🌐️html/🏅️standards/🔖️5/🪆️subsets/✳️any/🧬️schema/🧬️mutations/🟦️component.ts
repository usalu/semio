/** 🧬️ HtmlMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the HtmlMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface HtmlMutationEntry {
  key: string;
  value: string;
}
export interface HtmlMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: HtmlMutationEntry[];
}
