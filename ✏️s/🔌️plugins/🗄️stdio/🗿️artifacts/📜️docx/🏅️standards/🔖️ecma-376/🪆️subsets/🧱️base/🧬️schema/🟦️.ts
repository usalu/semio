import type { DocxXmlPart, RetainedOpcPackage } from './📸️snapshot/🟦️.ts';

/** 🧬️ Complete materialized DOCX artifact state. */
export interface DocxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: RetainedOpcPackage;
  /** @state artifact */ xmlParts: DocxXmlPart[];
}
