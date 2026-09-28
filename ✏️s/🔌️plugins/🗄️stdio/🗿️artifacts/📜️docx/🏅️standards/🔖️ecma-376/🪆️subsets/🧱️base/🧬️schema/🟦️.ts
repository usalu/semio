import type { DocxXmlPart, OpcPackage } from './📸️snapshot/🟦️.ts';

/** 🧬️ Complete materialized DOCX artifact state. */
export interface DocxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: DocxXmlPart[];
}
