import type { OpcPackage, XlsxXmlPart } from './📸️snapshot/🟦️.ts';

export interface XlsxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: XlsxXmlPart[];
}
