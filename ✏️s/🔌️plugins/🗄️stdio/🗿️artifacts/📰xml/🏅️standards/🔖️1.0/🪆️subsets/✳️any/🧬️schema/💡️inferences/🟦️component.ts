/** 💡️ Xml inference schema — document outline (element count, max depth, hasDoctype). */

export interface XmlOutline {
  elementCount: number;
  maxDepth: number;
  hasDoctype: boolean;
}

export interface XmlInference {
  /** @state inferred */
  outline: XmlOutline;
}
