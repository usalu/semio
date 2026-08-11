/** 🏷️ XML attribute pair. */
export interface XmlAttr {
  name: string;
  value: string;
}

/** 🌳 XML node: element, text, CDATA, comment, or processing instruction. */
export type XmlNode =
  | { kind: 'element'; name: string; attrs: XmlAttr[]; children: XmlNode[] }
  | { kind: 'text'; text: string }
  | { kind: 'cData'; text: string }
  | { kind: 'comment'; text: string }
  | { kind: 'processingInstruction'; target: string; data: string };

/** 🏳️ Typed `<?xml version="1.0" encoding="..." standalone="..."?>` declaration. */
export interface XmlDeclaration {
  version: string;
  encoding?: string;
  standalone?: boolean;
}

/** 📰 Well-formed XML document root. */
export interface XmlDocument {
  root?: XmlNode;
  doctype?: string;
  declaration?: XmlDeclaration;
}

/** 📸️ Persisted `stdio.xml` snapshot. */
export interface XmlSnapshot {
  schema: string;
  doc: XmlDocument;
}
