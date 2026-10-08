import type { DocxXmlAddress } from './🧭️xml-address/🟦️.ts';
export { parseDocxXmlAddress, DocxXmlAddressGuardRefusal, type DocxXmlAddress } from './🧭️xml-address/🟦️.ts';

/** 🧭️ Projected semantic block address retained by the remaining block-level commands. */
export interface DocxBlockPath {
  readonly segments?: readonly { readonly blockIndex: number; readonly row: number; readonly cell: number }[];
  readonly index: number;
}

/** 🧬️ Complete DOCX mutation union. */
export type DocxMutation =
  | { readonly mutation: 'insertBlock'; readonly path: DocxBlockPath; readonly block: import('../📸️snapshot/🟦️.ts').DocxBlock }
  | { readonly mutation: 'removeBlock'; readonly path: DocxBlockPath }
  | { readonly mutation: 'setBlockContent'; readonly path: DocxBlockPath; readonly block: import('../📸️snapshot/🟦️.ts').DocxBlock }
  | { readonly mutation: 'setRunText'; readonly address: DocxXmlAddress; readonly text: string }
  | { readonly mutation: 'replaceXmlNode'; readonly address: DocxXmlAddress; readonly node: import('../📸️snapshot/🟦️.ts').XmlNode }
  | { readonly mutation: 'setRunFormatting'; readonly address: DocxXmlAddress; readonly bold: boolean; readonly italic: boolean; readonly underline: boolean }
  | { readonly mutation: 'setParagraphStyle'; readonly address: DocxXmlAddress; readonly styleId: string | null }
  | { readonly mutation: 'insertTableRow'; readonly address: DocxXmlAddress; readonly index: number; readonly cells: readonly string[] }
  | { readonly mutation: 'removeTableRow'; readonly address: DocxXmlAddress; readonly index: number }
  | { readonly mutation: 'insertXmlNode'; readonly parent: DocxXmlAddress; readonly index: number; readonly node: import('../📸️snapshot/🟦️.ts').XmlNode }
  | { readonly mutation: 'removeXmlNode'; readonly parent: DocxXmlAddress; readonly index: number; readonly expectedName: string; readonly revision: string }
  | { readonly mutation: 'insertStyle'; readonly style: import('../📸️snapshot/🟦️.ts').DocxStyle }
  | { readonly mutation: 'removeStyle'; readonly id: string }
  | { readonly mutation: 'setStyleName'; readonly id: string; readonly name: string }
  | { readonly mutation: 'setStyleBasedOn'; readonly id: string; readonly based_on: string | null }
  | { readonly mutation: 'setPart'; readonly path: string; readonly content_type: string; readonly payload: DocxPartContent; readonly index?: number; readonly override_index?: number }
  | { readonly mutation: 'removePart'; readonly path: string }
  | { readonly mutation: 'setRelationship'; readonly owner: string; readonly id: string; readonly relType: string; readonly target: string; readonly external?: boolean; readonly index?: number }
  | { readonly mutation: 'removeRelationship'; readonly owner: string; readonly id: string }
  | { readonly mutation: 'setContentType'; readonly isOverride?: boolean; readonly name: string; readonly contentType: string; readonly index?: number }
  | { readonly mutation: 'removeContentType'; readonly isOverride?: boolean; readonly name: string };

/** 📦️ Authoritative part content independent of physical XML parsing. */
export type DocxPartContent = {readonly kind:"xml";readonly document:import("../📸️snapshot/🟦️.ts").XmlDocument}|{readonly kind:"binary";readonly bytes:readonly number[]};
export type DocxSetPart = Extract<DocxMutation,{mutation:"setPart"}>;
