import type { DocxArtifact as DocxBaseArtifact } from '../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts';
import type { DocxArtifact as DocxStrictArtifact } from '../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🟦️.ts';
import type { DocxArtifact as DocxTransitionalArtifact } from '../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🧬️schema/🟦️.ts';
import type { XlsxArtifact as XlsxBaseArtifact } from '../../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts';
import type { XlsxArtifact as XlsxStrictArtifact } from '../../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🟦️.ts';
import type { XlsxArtifact as XlsxTransitionalArtifact } from '../../../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🟦️.ts';
import type { PptxArtifact as PptxBaseArtifact } from '../../../🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts';
import type { PptxArtifact as PptxStrictArtifact } from '../../../🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🟦️.ts';
import type { PptxArtifact as PptxTransitionalArtifact } from '../../../🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🟦️.ts';
import { parseDocxXmlAddress, type DocxMutation } from '../../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts';

type WithoutEntries<T> = 'entries' extends keyof T ? never : T;

const opc = { parts: [], contentTypes: { defaults: [], overrides: [] }, relationships: {}, comment: '' };
const docx = {
  schema: 's.stdio.docx', opc,
  xmlParts: [{ path: 'custom/document.xml', contentType: 'application/xml', document: {
    declaration: { version: '1.0', quote: 'single' },
    prolog: [{ kind: 'comment', text: 'before' }],
    doctype: { name: 'w:document', prologPosition: 1, declarations: [] },
    root: { kind: 'element', name: 'w:document', attrs: [], children: [] },
    epilog: [{ kind: 'processingInstruction', target: 'after', data: 'retained' }],
  } }],
} satisfies WithoutEntries<DocxBaseArtifact>;
const xlsx = { schema: 's.stdio.xlsx', opc, workbook: { sheets: [], sharedStrings: [] } } satisfies WithoutEntries<XlsxBaseArtifact>;
const pptx = { schema: 's.stdio.pptx', opc, xmlParts: docx.xmlParts, presentation: { slides: [] } } satisfies WithoutEntries<PptxBaseArtifact>;

/** 🧪️ Materialized public artifacts accepted identically by every ECMA-376 subset facet. */
export const officeArtifactFacetTypeProof: readonly [DocxStrictArtifact, DocxTransitionalArtifact, XlsxStrictArtifact, XlsxTransitionalArtifact, PptxStrictArtifact, PptxTransitionalArtifact] = [docx, docx, xlsx, xlsx, pptx, pptx];

const address = parseDocxXmlAddress({ partPath: 'custom/document.xml', nodePath: [0, 1], expectedName: '{}metadata', revision: '0123456789abcdef' });

/** 🧭️ Canonical DOCX forward/inverse mutations share one public address contract. */
export const docxAddressedMutationTypeProof: readonly DocxMutation[] = [
  { mutation: 'setRunText', address, text: 'Grüße' },
  { mutation: 'replaceXmlNode', address, node: { kind: 'text', text: 'restored' } },
  { mutation: 'setRunFormatting', address, bold: true, italic: false, underline: true },
  { mutation: 'setParagraphStyle', address, styleId: null },
  { mutation: 'insertTableRow', address, index: 0, cells: ['Grüße', '文字'] },
  { mutation: 'removeTableRow', address, index: 0 },
  { mutation: 'insertXmlNode', parent: address, index: 0, node: { kind: 'text', text: 'inserted' } },
  { mutation: 'removeXmlNode', parent: address, index: 0, expectedName: '#text', revision: '0123456789abcdef' },
];
