/** 🧬️ Logical ECMA-376 PresentationML snapshot schema. */
export interface OpcRelationship { id: string; relType: string; target: string; targetMode: 'internal' | 'external' }
export interface OpcPart { path: string; contentType: string; bytes: number[] }
export interface OpcPackage {
  contentTypes: { defaults: [string, string][]; overrides: [string, string][] };
  parts: OpcPart[];
  relationships: Record<string, OpcRelationship[]>;
}
export interface XmlAttr { name: string; value: string }
export type XmlNode =
  | { kind: 'element'; name: string; attrs: XmlAttr[]; children: XmlNode[] }
  | { kind: 'text' | 'cData' | 'comment'; text: string }
  | { kind: 'processingInstruction'; target: string; data: string };
export interface XmlDocument { declaration?: { version: string; encoding?: string; standalone?: boolean }; prolog: XmlNode[]; root?: XmlNode }
export interface PptxXmlPart { path: string; contentType: string; document: XmlDocument }
export interface PptxTransform { x: number; y: number; cx: number; cy: number }
export interface PptxRun { text: string; bold: boolean; italic: boolean; fontSize?: number }
export interface PptxParagraph { runs: PptxRun[] }
export type PptxShape =
  | { shapeKind: 'textBox'; textFrame: PptxParagraph[]; position: PptxTransform }
  | { shapeKind: 'picture'; blipRelId: string; position: PptxTransform }
  | { shapeKind: 'placeholder'; kind: string; textFrame: PptxParagraph[]; position: PptxTransform }
  | { shapeKind: 'other'; node: XmlNode };
export interface PptxSlide { shapes: PptxShape[] }
export interface PptxPresentation { slides: PptxSlide[] }
export interface PptxPhysicalState { archive: unknown; semanticBlake3: number[] }
export interface PptxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: PptxXmlPart[];
  /** @state artifact */ presentation: PptxPresentation;
  /** @state artifact */ physical?: PptxPhysicalState;
}
