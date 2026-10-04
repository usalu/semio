/// <reference path="./🗄️.sql.d.ts" />
/** 📕️ PPTX literal package and authoritative XML relations. */
import sql from './🗄️.sql' with { type: 'text' };
import type { PptxSnapshot } from '../🟦️.ts';
import {
  artifactSqliteCheckpoint,
  artifactSqliteDocument,
  artifactSqliteInteger,
  artifactSqliteOrderedRowsControlled,
  artifactSqliteTables,
  artifactSqliteText,
  artifactSqliteTextBytes,
  artifactSqliteValueBudget,
  type ArtifactSqliteOptions,
} from '../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts';
import type { SqliteDatabase, SqliteRow } from '../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';
import { measureXmlDocuments, projectXmlDocuments, reconstructXmlDocuments, type XmlSqliteTables } from '../../../../../../../../📰️xml/🟦️.ts';
import { measureOpcPackage, projectOpcPackage, reconstructOpcPackage, type OpcSqliteTables } from '../../../../../../../../🎒️zip/📦️opc/🪶️sqlite/🟦️.ts';

export const PPTX_SQLITE_SCHEMA: string = sql;
const OPC: OpcSqliteTables = { package: 'pptx_package', part: 'pptx_binary_part', defaultType: 'pptx_default_content_type', overrideType: 'pptx_override_content_type', relationshipOwner: 'pptx_relationship_owner', relationship: 'pptx_relationship' };
const XML: XmlSqliteTables = { document: 'pptx_xml_document', node: 'pptx_xml_node', element: 'pptx_xml_element', text: 'pptx_xml_text', cdata: 'pptx_xml_cdata', comment: 'pptx_xml_comment', processingInstruction: 'pptx_xml_processing_instruction', attribute: 'pptx_xml_attribute', child: 'pptx_xml_child', documentMisc: 'pptx_xml_document_misc', declaration: 'pptx_xml_declaration', doctype: 'pptx_xml_doctype', entity: 'pptx_xml_entity' };
const rowsLimit = (count: number, options: ArtifactSqliteOptions): void => {
  if (!Number.isSafeInteger(count) || count > (options.maxRows ?? 1_000_000)) throw new Error('PPTX row limit');
};
const table = (database: SqliteDatabase, name: string): readonly SqliteRow[] => {
  const found = database.tables.find((candidate) => candidate.name === name);
  if (!found) throw new Error('PPTX declared table missing');
  return found.rows;
};

/** 🏗️ Projects the sole OPC and XML-part authority under one aggregate budget. */
export async function pptxSnapshotToSqliteDatabase(snapshot: PptxSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, 'projectSnapshot', 0, 0);
  const opc = await measureOpcPackage(snapshot.opc, options);
  let bytes = opc.bytes + 16 + artifactSqliteTextBytes(snapshot.schema);
  const documents = [];
  for (const [ordinal, part] of snapshot.xmlParts.entries()) {
    if (ordinal % 256 === 0) await artifactSqliteCheckpoint(options, 'projectSnapshot', ordinal, snapshot.xmlParts.length);
    bytes += 32 + artifactSqliteTextBytes(part.contentType);
    artifactSqliteValueBudget(bytes, options);
    documents.push({ schema: part.path, doc: part.document });
  }
  const xml = await measureXmlDocuments(documents, options);
  rowsLimit(opc.rows + 1 + snapshot.xmlParts.length + xml.rows, options);
  artifactSqliteValueBudget(bytes + xml.bytes, options);
  const database = await projectOpcPackage(snapshot.opc, await projectXmlDocuments(documents, sql, XML, options), OPC, options);
  const root: SqliteRow = { rowid: 1n, values: [1n, snapshot.schema, 1n] };
  const parts: SqliteRow[] = snapshot.xmlParts.map((part, ordinal) => {
    const rowid = BigInt(ordinal + 1);
    return { rowid, values: [rowid, 1n, BigInt(ordinal), part.contentType, rowid] };
  });
  const result = { tables: database.tables.map((candidate) => candidate.name === 'pptx_document' ? { ...candidate, rows: [root] } : candidate.name === 'pptx_xml_part' ? { ...candidate, rows: parts } : candidate) };
  await artifactSqliteTables(result, sql, options);
  return result;
}

/** 🧱️ Reconstructs only package and XML-part owners, rejecting hidden XML documents. */
export async function pptxSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<PptxSnapshot> {
  await artifactSqliteTables(database, sql, options);
  const root = artifactSqliteDocument(table(database, 'pptx_document'));
  if (root.values.length !== 3 || artifactSqliteInteger(root, 2) !== 1n) throw new Error('PPTX package owner invalid');
  const schema = artifactSqliteText(root, 1);
  const opc = await reconstructOpcPackage(database, OPC, options);
  const parts = await artifactSqliteOrderedRowsControlled(table(database, 'pptx_xml_part'), 2, options);
  const documents = await reconstructXmlDocuments(database, sql, XML, options);
  if (parts.length !== documents.length) throw new Error('PPTX XML part/document cardinality disagrees');
  const identities = new Set<bigint>();
  const xmlParts: PptxSnapshot['xmlParts'] = [];
  for (const [ordinal, part] of parts.entries()) {
    if (ordinal % 256 === 0) await artifactSqliteCheckpoint(options, 'reconstructSnapshot', ordinal, parts.length);
    if (part.values.length !== 5 || part.rowid <= 0n || artifactSqliteInteger(part, 0) !== part.rowid || identities.has(part.rowid) || artifactSqliteInteger(part, 1) !== 1n || artifactSqliteInteger(part, 4) !== BigInt(ordinal + 1)) throw new Error('PPTX XML part owner invalid');
    identities.add(part.rowid);
    const document = documents[ordinal]!;
    xmlParts.push({ path: document.schema, contentType: artifactSqliteText(part, 3), document: document.doc });
  }
  return { schema, opc, xmlParts };
}
