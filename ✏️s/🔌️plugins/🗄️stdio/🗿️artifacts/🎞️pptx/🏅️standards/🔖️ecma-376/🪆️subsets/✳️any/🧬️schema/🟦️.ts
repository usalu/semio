/** 🧬️ Logical ECMA-376 PresentationML artifact schema. */
import type { OpcPackage, PptxPresentation, PptxXmlPart } from './📸️snapshot/🟦️.ts';
export interface PptxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: PptxXmlPart[];
  /** @state artifact */ presentation: PptxPresentation;
}
