import type { DocxArtifact as DocxBaseArtifact } from '@semio-tech/stdio-docx/standards/ecma-376/subsets/base/schema';
import type { DocxArtifact as DocxStrictArtifact } from '@semio-tech/stdio-docx/standards/ecma-376/subsets/strict/schema';
import type { DocxArtifact as DocxTransitionalArtifact } from '@semio-tech/stdio-docx/standards/ecma-376/subsets/transitional/schema';
import type { XlsxArtifact as XlsxBaseArtifact } from '@semio-tech/stdio-xlsx/standards/ecma-376/subsets/base/schema';
import type { XlsxArtifact as XlsxStrictArtifact } from '@semio-tech/stdio-xlsx/standards/ecma-376/subsets/strict/schema';
import type { XlsxArtifact as XlsxTransitionalArtifact } from '@semio-tech/stdio-xlsx/standards/ecma-376/subsets/transitional/schema';
import type { PptxArtifact as PptxBaseArtifact } from '@semio-tech/stdio-pptx/standards/ecma-376/subsets/base/schema';
import type { PptxArtifact as PptxStrictArtifact } from '@semio-tech/stdio-pptx/standards/ecma-376/subsets/strict/schema';
import type { PptxArtifact as PptxTransitionalArtifact } from '@semio-tech/stdio-pptx/standards/ecma-376/subsets/transitional/schema';
import { parseDocxXmlAddress, type DocxMutation } from '@semio-tech/stdio-docx/standards/ecma-376/subsets/base/schema/mutations';

type WithoutEntries<T> = 'entries' extends keyof T ? never : T;

const opc = { parts: [], contentTypes: { defaults: [], overrides: [] }, relationships: {}, comment: '' };
const docx = {
  schema: 's.stdio.docx', opc,
  xmlParts: [{ path: 'custom/document.xml', contentType: 'application/xml', document: {
    nodes: [], attributes: [], prolog: [], epilog: [], root: null, doctype: null, declaration: null,
  } }],
} satisfies WithoutEntries<DocxBaseArtifact>;
const treeXmlParts = [{ path: 'custom/document.xml', contentType: 'application/xml', document: { prolog: [], epilog: [] } }];
const xlsx = { schema: 's.stdio.xlsx', opc, xmlParts: treeXmlParts } satisfies WithoutEntries<XlsxBaseArtifact>;
const pptx = { schema: 's.stdio.pptx', opc, xmlParts: treeXmlParts } satisfies WithoutEntries<PptxBaseArtifact>;

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
