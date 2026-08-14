/** 🧬️ Logical ECMA-376 PresentationML artifact schema. */
import type { OpcPackage, PptxPhysicalState, PptxPresentation, PptxXmlPart } from './📸️snapshot/🟦️component.ts';
export interface PptxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: PptxXmlPart[];
  /** @state artifact */ presentation: PptxPresentation;
  /** @state artifact */ physical?: PptxPhysicalState;
}
