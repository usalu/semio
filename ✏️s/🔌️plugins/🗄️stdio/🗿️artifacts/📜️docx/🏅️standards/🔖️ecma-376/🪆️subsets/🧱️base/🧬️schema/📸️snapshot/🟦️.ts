/** 📦️ One losslessly retained OPC package part. */
export interface OpcPart { path: string; contentType: string; bytes: number[] }
/** 🔗️ One OPC relationship. */
export interface OpcRelationship { id: string; relType: string; target: string; targetMode: 'internal' | 'external' }
/** 📦️ The complete OPC container, including the ZIP comment. */
export interface OpcPackage {
  parts: OpcPart[];
  contentTypes: { defaults: [string, string][]; overrides: [string, string][] };
  relationships: Record<string, OpcRelationship[]>;
  comment: string;
}
/** 🏷️ One XML attribute. */
export interface XmlAttr { name: string; value: string }
/** 🌳 Raw XML retained for unmodeled WordprocessingML properties. */
export type XmlNode =
  | { kind: 'element'; name: string; attrs: XmlAttr[]; children: XmlNode[] }
  | { kind: 'text' | 'cData' | 'comment'; text: string }
  | { kind: 'processingInstruction'; target: string; data: string };
/** ✍️ One WordprocessingML text run. */
export interface DocxRun { text: string; bold: boolean; italic: boolean; underline: boolean; extraRunProperties: XmlNode[] }
/** 📄️ One WordprocessingML paragraph. */
export interface DocxParagraph { runs: DocxRun[]; style: string | null; extraParagraphProperties: XmlNode[] }
/** 🔲️ One recursive table cell. */
export interface DocxTableCell { blocks: DocxBlock[]; extraCellProperties: XmlNode[] }
/** ➖️ One table row. */
export interface DocxTableRow { cells: DocxTableCell[]; extraRowProperties: XmlNode[] }
/** 🏛️ One WordprocessingML table. */
export interface DocxTable { rows: DocxTableRow[]; extraTableProperties: XmlNode[] }
/** 🧱️ One ordered document block. */
export type DocxBlock =
  | ({ kind: 'paragraph' } & DocxParagraph)
  | ({ kind: 'table' } & DocxTable);
/** 🎨️ One named document style. */
export interface DocxStyle { id: string; name: string; basedOn: string | null }
/** 📰 The typed semantic document view. */
export interface DocxDocument { body: DocxBlock[]; styles: DocxStyle[] }
/** 📄️ One authoritative XML-bearing OPC part. */
export interface DocxXmlPart { path: string; contentType: string; document: XmlDocument }
/** 🧬️ Complete lossless DOCX snapshot. */
export interface DocxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: DocxXmlPart[];
}
import type { XmlDocument } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
