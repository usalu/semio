/** 💡️ Html inference schema — document outline (element count, max depth, text length). */

export interface HtmlOutline {
  elementCount: number;
  maxDepth: number;
  textLength: number;
}

export interface HtmlInference {
  /** @state inferred */
  outline: HtmlOutline;
}
