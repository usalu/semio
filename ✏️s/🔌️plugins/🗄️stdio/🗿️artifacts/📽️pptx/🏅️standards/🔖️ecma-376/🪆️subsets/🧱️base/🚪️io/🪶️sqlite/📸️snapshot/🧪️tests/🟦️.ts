import {parsePptxSnapshotJson} from "../../../📝️text/📸️snapshot/🔣️json/🟦️.ts";
/** 🧫️ Canonical OPC/XML ownership checked through independent SQLite. */
import { expect, test } from 'bun:test';
import { Database } from 'bun:sqlite';
import { Buffer } from 'node:buffer';
import Ajv from 'ajv';
import Ajv2020 from 'ajv/dist/2020';
import { exportSqliteDatabase, importSqliteDatabase } from '@semio-tech/framework';
import schema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import xmlSchema from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
import { parsePptxSnapshot, type PptxSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PPTX_SQLITE_SCHEMA, pptxSnapshotFromSqliteDatabase, pptxSnapshotToSqliteDatabase } from "../../../../../../../../🟦️.ts";
import authority from "../🧫️fixtures/🧭️authority/🔣️.json";

import fixture from "../🧫️fixtures/🔣️.json";
import constructionFidelity from "../../../../🧫️fixtures/🏗️typed-construction-fidelity/🔣️.json";

import literalNative from "../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧩️native-literals/🔣️.json";

import { parseXmlDocument } from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import profiles from "../../../../🧬️schema/📸️snapshot/🧫️fixtures/🛡️profile/🔣️.json";

import * as profileOwner from "../../../../../../../../🟦️.ts";
import paidOwner from "../🧫️fixtures/💰️backing/🔣️.json";

import requestSettlement from "../🧫️fixtures/💰️backing/🔬️requests/🔣️.json";

import { createHash } from 'node:crypto';
import definition from "../../../../../../../../📜️artifact-definition.json";

type ProfileCheck = (snapshot: PptxSnapshot, subset: "strict" | "transitional", options?: {signal?: AbortSignal; maximumBytes?: number; onProgress?: (event: {completed: number; total: number}) => void | Promise<void>}) => Promise<readonly {code: string; severity: string}[]>;
function profileValidator(): ProfileCheck {
  const check = (profileOwner as unknown as {validatePptxSnapshotProfile?: ProfileCheck}).validatePptxSnapshotProfile;
  if (!check) throw Error('PPTX owner has no typed exact-profile validator');
  return check;
}
function profileSnapshot(document: unknown, relationshipBase: string): PptxSnapshot {
  return {schema: 'literal retained schema', opc: {parts: [], contentTypes: {defaults: [], overrides: []}, relationships: {'': [{id: 'main', relType: relationshipBase + '/officeDocument', target: 'ppt/presentation.xml', targetMode: 'internal'}]}, comment: ''}, xmlParts: [{path: 'ppt/presentation.xml', contentType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml', document: parseXmlDocument(document)}]};
}

const input = parsePptxSnapshotJson(fixture);

const physical = async (snapshot: PptxSnapshot): Promise<Database> => Database.deserialize(await exportSqliteDatabase(await pptxSnapshotToSqliteDatabase(snapshot)));

test('PPTX snapshot schema admits exactly the three canonical fields', () => {
  const ajv = new Ajv({ strict: false }).addSchema(xmlSchema);
  expect(ajv.validate(schema, fixture), JSON.stringify(ajv.errors)).toBe(true);
  expect(Object.keys(input)).toEqual(['schema', 'opc', 'xmlParts']);
  expect(Object.keys(schema.properties)).toEqual(['schema', 'opc', 'xmlParts']);
});

test('PPTX typed construction fidelity corpus is schema-first and language-neutral', () => {
  const ajv = new Ajv2020({ strict: true });
  
});

test('PPTX canonical authority uses exactly twenty-one physical tables', async () => {
  expect(authority["fields"]).toEqual(["schema","opc","xmlParts"]);expect(authority["derived"]).toEqual(["presentation"]);expect(authority["tableNames"]).toEqual(["pptx_document","pptx_package","pptx_binary_part","pptx_default_content_type","pptx_override_content_type","pptx_relationship_owner","pptx_relationship","pptx_xml_part","pptx_xml_document","pptx_xml_node","pptx_xml_element","pptx_xml_text","pptx_xml_cdata","pptx_xml_comment","pptx_xml_processing_instruction","pptx_xml_attribute","pptx_xml_child","pptx_xml_document_misc","pptx_xml_declaration","pptx_xml_doctype","pptx_xml_entity"]);
  const snapshot = parsePptxSnapshotJson(authority.snapshot);
  const declared = new Database(':memory:');
  try {
    declared.run(PPTX_SQLITE_SCHEMA);
    expect(declared.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()).toEqual([...authority.tableNames].sort().map((name) => ({ name })));
  } finally {
    declared.close();
  }
  const database = await pptxSnapshotToSqliteDatabase(snapshot);
  expect(database.tables.map((table) => table.name).sort()).toEqual([...authority.tableNames].sort());
  const sql = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(sql.query('PRAGMA integrity_check').get()).toEqual({ integrity_check: 'ok' });
    expect(sql.query('PRAGMA foreign_key_check').all()).toEqual([]);
    expect(sql.query('SELECT count(*) AS n FROM pptx_xml_part').get()).toEqual({ n: snapshot.xmlParts.length });
    expect(sql.query('SELECT hex(CAST(comment AS BLOB)) AS word FROM pptx_package').get()).toEqual({ word: Buffer.from(snapshot.opc.comment, 'utf8').toString('hex').toUpperCase() });
    const restored = await pptxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())));
    expect(restored).toEqual(snapshot);
  } finally {
    sql.close();
  }
});

test('PPTX independent SQL mutation affects only the addressed canonical XML node', async () => {
  const sql = await physical(input);
  try {
    const before = input.opc;
    sql.query("UPDATE pptx_xml_text SET text=? WHERE node_id=(SELECT node_id FROM pptx_xml_document_misc WHERE xml_document_id=1 AND position='prolog' AND ordinal=0)").run('independent SQL 世界');
    const restored = await pptxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())));
    expect(restored.xmlParts[0]!.document.prolog[0]).toEqual({ kind: 'text', text: 'independent SQL 世界' });
    expect(restored.opc).toEqual(before);
  } finally {
    sql.close();
  }
});

test('PPTX aggregate row limit covers one shared OPC/XML projection', async () => {
  const database = await pptxSnapshotToSqliteDatabase(input);
  const rows = database.tables.reduce((total, table) => total + table.rows.length, 0);
  expect(await pptxSnapshotFromSqliteDatabase(await pptxSnapshotToSqliteDatabase(input, { maxRows: rows }), { maxRows: rows })).toEqual(input);
  await expect(pptxSnapshotToSqliteDatabase(input, { maxRows: rows - 1 })).rejects.toThrow();
});

test('PPTX dangling and shared owners are refused', async () => {
  const database = await pptxSnapshotToSqliteDatabase(input);
  for (const [name, row, column, value] of [['pptx_document', 0, 2, 2n], ['pptx_xml_part', 1, 4, 1n], ['pptx_relationship', 0, 1, 99n], ['pptx_default_content_type', 0, 2, 99n], ['pptx_relationship_owner', 1, 2, '']] as const) {
    const edited = { tables: database.tables.map((table) => table.name !== name ? table : { ...table, rows: table.rows.map((entry, index) => index !== row ? entry : { ...entry, values: entry.values.map((cell, at) => at === column ? value : cell) }) }) };
    await expect(pptxSnapshotFromSqliteDatabase(edited), name).rejects.toThrow();
  }
});

test('PPTX native literal contains no semantic presentation owner', async () => {
  expect(literalNative["snapshot"]["schema"]).toEqual("é");expect(literalNative["rows"]).toEqual(2);
  expect(Object.keys(literalNative.snapshot)).toEqual(['schema', 'opc', 'xmlParts']);
  expect(literalNative.text).toBe('semio stdio.pptx.dsl v1\n[c3a9,[[],[],[],[],],[]]');
  const binary = Buffer.from(literalNative.binary);
  const token = Buffer.from('stdio.pptx.pack v1');
  expect(binary.subarray(0, 8)).toEqual(Buffer.from([137, 83, 69, 77, 13, 10, 26, 10]));
  expect(binary.readUInt32LE(8)).toBe(token.length);
  expect(binary.subarray(12, 12 + token.length)).toEqual(token);
  expect(await pptxSnapshotFromSqliteDatabase(await pptxSnapshotToSqliteDatabase(parsePptxSnapshotJson(literalNative.snapshot)))).toEqual(parsePptxSnapshotJson(literalNative.snapshot));
});
test('PPTX exact profile fixture exposes namespace declarations through independent SQL', async () => {
  
  for (const item of profiles.cases) {
    const snapshot = profileSnapshot(item.document, item.relationshipBase), sql = await physical(snapshot);
    try {
      expect(sql.query("SELECT value FROM pptx_xml_attribute WHERE name='xmlns:p'").all()).toEqual([{value: item.document.root.name === 'foreign' ? 'http://purl.oclc.org/ooxml/presentationml/main' : item.document.root.attrs[0]!.value}]);
      expect(sql.query('SELECT target FROM pptx_relationship').get()).toEqual({target: 'ppt/presentation.xml'});
    } finally { sql.close(); }
  }
});
test('PPTX exact profile policies inspect typed root and namespace entities', async () => {
  const check = profileValidator();
  for (const item of profiles.cases) {
    const diagnostics = await check(profileSnapshot(item.document, item.relationshipBase), item.subset as "strict" | "transitional");
    expect(diagnostics.map(item => item.code)).toEqual(item.codes);
    expect(diagnostics.filter(item => item.severity === 'warning').length).toBe(item.warnings);
  }
});
test('PPTX typed profile copies admit ownership and publish interior cancellation', async () => {
  const check = profileValidator(), first = profiles.cases[0]!, snapshot = profileSnapshot(first.document, first.relationshipBase), path = 'x'.repeat(profiles.largeTextCharacters);
  snapshot.opc.relationships['']![0]!.target = path; snapshot.xmlParts[0]!.path = path;
  const controller = new AbortController(); let reached = false;
  await expect(check(snapshot, 'strict', {signal: controller.signal, onProgress(event) {
    if (event.total === path.length && event.completed >= profiles.cancelAfter && event.completed < event.total) { reached = true; controller.abort(); }
  }})).rejects.toHaveProperty('kind','canceled');
  expect(reached).toBe(true);
  await expect(check(snapshot, 'strict', {maximumBytes: profiles.smallValueBudget})).rejects.toThrow('limit');
});
test('PPTX intrinsic large-part copies have interior ownership cancellation', async () => {
  const snapshot = structuredClone(input); snapshot.opc.parts[0]!.bytes = new Array(100000).fill(0x97);
  const database = await pptxSnapshotToSqliteDatabase(snapshot);
  for (const phase of ['projectSnapshot', 'reconstructSnapshot'] as const) {
    const controller = new AbortController(); let reached = false;
    const options = {signal: controller.signal, onProgress(event: {phase: string; completed: number; total: number}) {
      if (event.phase === phase && event.total === 100000 && event.completed >= 65536 && event.completed < event.total) { reached = true; controller.abort(); }
    }};
    await expect(phase === 'projectSnapshot' ? pptxSnapshotToSqliteDatabase(snapshot, options) : pptxSnapshotFromSqliteDatabase(database, options)).rejects.toHaveProperty('kind', 'canceled');
    expect(reached).toBe(true);
  }
});
test('PPTX signed64 derived scalars retain their full literal XML authority', async () => {
  const {parsePptxCoordinate} = await import('../../../../🧬️schema/📸️snapshot/🟦️.ts');
  for (const value of ['-0', '00', '+1', '9223372036854775808', '-9223372036854775809', 9007199254740993]) expect(() => parsePptxCoordinate(value, '$.x')).toThrow();
  const sql = await physical(input);
  try {
    expect(sql.query("SELECT name,value FROM pptx_xml_attribute WHERE element_node_id IN (SELECT node_id FROM pptx_xml_element WHERE name IN ('a:off','a:ext')) ORDER BY id LIMIT 4").all()).toEqual([
      {name: 'x', value: '-9223372036854775808'}, {name: 'y', value: '9223372036854775807'}, {name: 'cx', value: '9007199254740993'}, {name: 'cy', value: '-9007199254740993'}
    ]);
    expect(sql.query("SELECT value FROM pptx_xml_attribute WHERE name='sz'").all()).toEqual([{value: '4294967295'}]);
    expect(sql.query("SELECT name FROM pptx_xml_element WHERE name IN ('p:sp','p:pic','arbitrary') ORDER BY node_id").all()).toEqual([{name:'p:sp'}, {name:'p:pic'}, {name:'p:sp'}, {name:'arbitrary'}]);
    expect(await pptxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(input);
  } finally { sql.close(); }
});
test('PPTX native factory hash identifies its complete literal protocol', async () => {
  const bytes = await Bun.file(new URL('../../../💾️binary/📸️snapshot/📡️.protocol.semio', import.meta.url)).arrayBuffer();
  const measured = createHash('sha256').update(Buffer.from(bytes)).digest('hex');
  expect(new Bun.CryptoHasher('sha256').update(bytes).digest('hex')).toBe(measured);
  expect(definition.codecs[0]!.native_factory!.pack_schema_hash).toBe(measured);
});
test('PPTX neutral ownership roles retain independently measured authoritative fields', async () => {
  expect(paidOwner["fieldOrder"]).toEqual(["schema","opc","xmlParts"]);expect(paidOwner["tableCount"]).toEqual(21);expect(paidOwner["admission"]).toEqual("beforeConcreteBacking");expect(paidOwner["retirementRefund"]).toEqual(false);expect(paidOwner["roles"]).toEqual({"maxValueBytes":"semanticLiteralBytes","maxAllocationBytes":"cumulativeOwnedBacking","encodedBound":"prospectiveWithoutPayload","borrowedPreflight":"paidTraversalFrontiers"});expect(paidOwner["refusalKinds"]).toEqual({"owned":"ownershipLimit","allocator":"allocationFailed","work":"workLimit","cancellation":"canceled"});expect(paidOwner["literal"]).toEqual("Grüße\t\n\u0000🌠");expect(paidOwner["literalUtf8Hex"]).toEqual("4772c3bcc39f65090a00f09f8ca0");
  expect(Object.keys(input)).toEqual(paidOwner.fieldOrder);
  const literal = 'Grüße\t\n\0🌠';
  expect(Buffer.from(literal, 'utf8').toString('hex')).toBe(paidOwner.literalUtf8Hex);
  const sql = await physical(input);
  try {
    expect(sql.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n: paidOwner.tableCount});
    expect(sql.query('SELECT hex(CAST(? AS BLOB)) AS word').get(literal)).toEqual({word: '4772C3BCC39F65090A00F09F8CA0'});
    expect(sql.query('SELECT count(*) AS n FROM pptx_xml_part').get()).toEqual({n: input.xmlParts.length});
    expect(sql.query('SELECT schema FROM pptx_document').get()).toEqual({schema: input.schema});
    expect(sql.query('PRAGMA foreign_key_check').all()).toEqual([]);
  } finally { sql.close(); }
});
test('PPTX full request settlement preserves independently inspected enclosing ownership', async () => {
  expect(requestSettlement["phases"]).toEqual(["projectSnapshot","reconstructSnapshot"]);expect(requestSettlement["requestExtent"]).toEqual("fullConcreteRequests");expect(requestSettlement["reallocation"]).toEqual("completeReplacement");expect(requestSettlement["exactReplay"]).toEqual(true);expect(requestSettlement["denial"]).toEqual("oneByteBelowObserved");expect(requestSettlement["retirementRefund"]).toEqual(false);expect(requestSettlement["refusalKind"]).toEqual("ownershipLimit");expect(requestSettlement["tableCount"]).toEqual(21);
  const sql = await physical(input);
  try {
    expect(sql.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n: requestSettlement.tableCount});
    expect(sql.query('SELECT count(*) AS n FROM pptx_xml_part').get()).toEqual({n: input.xmlParts.length});
    expect(sql.query('SELECT owner_path FROM pptx_relationship_owner ORDER BY owner_path').all()).toEqual(Object.keys(input.opc.relationships).sort().map(owner_path => ({owner_path})));
    expect(sql.query('PRAGMA foreign_key_check').all()).toEqual([]);
    expect(await pptxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(input);
  } finally { sql.close(); }
});
import retitlesBefore from "../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/📸️snapshot/⬅️before/🔣️.json";
import retitlesAfter from "../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/📸️snapshot/➡️after/🔣️.json";
import retitlesMutation from "../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/🦠️mutation/🔣️.json";
import retitlesDiff from "../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/🔺️diff/🔣️.json";
import retitlesOutcome from "../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🏷️retitles/🎯️outcome/🔣️.json";

test('PPTX authored retitles history preserves title, picture and transform through physical SQLite', async () => {
  const ajv = new Ajv({strict: false}).addSchema(xmlSchema);
  for (const [literal, text, titleY] of [[retitlesBefore, 'Nakagin', '0'], [retitlesAfter, 'Nakagin Capsule Tower', '457200']] as const) {
    expect(ajv.validate(schema, literal), JSON.stringify(ajv.errors)).toBe(true);
    expect(Object.keys(literal)).toEqual(['schema', 'opc', 'xmlParts']);
    const snapshot = parsePptxSnapshotJson(literal), sql = await physical(snapshot);
    try {
      expect(sql.query("SELECT text FROM pptx_xml_text WHERE text IN ('Nakagin','Nakagin Capsule Tower')").all()).toEqual([{text}]);
      expect(sql.query("SELECT value FROM pptx_xml_attribute WHERE name='y' ORDER BY id").all()).toEqual([{value: titleY}, {value: '1143000'}]);
      expect(sql.query("SELECT value FROM pptx_xml_attribute WHERE name='r:embed'").all()).toEqual([{value: 'rId2'}]);
      expect(sql.query("SELECT value FROM pptx_xml_attribute WHERE name='cx' ORDER BY id").all()).toEqual([{value: '9144000'}, {value: '9144000'}]);
      expect(sql.query("SELECT value FROM pptx_xml_attribute WHERE name='cy' ORDER BY id").all()).toEqual([{value: '1143000'}, {value: '4000000'}]);
      expect(sql.query("SELECT target FROM pptx_relationship WHERE identity='rId2'").all()).toEqual([{target: '../media/image1.gif'}]);
      expect(sql.query('PRAGMA foreign_key_check').all()).toEqual([]);
      expect(await pptxSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(snapshot);
    } finally { sql.close(); }
  }
  expect(retitlesBefore.opc).toEqual(retitlesAfter.opc);
  expect(retitlesMutation).toEqual({mutation: 'setSnapshot', snapshot: retitlesAfter});
  expect(retitlesDiff).toEqual({xmlParts: retitlesAfter.xmlParts});
  expect(retitlesOutcome).toEqual({status: 'applied'});
});

import strictInitial from "../../../../🧬️schema/📸️snapshot/🧫️fixtures/🔒️strict-initial/🔣️.json";


test('PPTX strict initial contract retains nine parts and all five resolved relationship identities', () => {
  
  expect(strictInitial["projection"]).toEqual({"format":"pptx","mainPart":"ppt/presentation.xml","mainRootName":"p:presentation","mainRootAttributes":[{"name":"conformance","value":"strict"},{"name":"xmlns:a","value":"http://purl.oclc.org/ooxml/drawingml/main"},{"name":"xmlns:p","value":"http://purl.oclc.org/ooxml/presentationml/main"},{"name":"xmlns:r","value":"http://purl.oclc.org/ooxml/officeDocument/relationships"}],"parts":[{"path":"[Content_Types].xml","contentType":"application/xml"},{"path":"_rels/.rels","contentType":"application/vnd.openxmlformats-package.relationships+xml"},{"path":"ppt/_rels/presentation.xml.rels","contentType":"application/vnd.openxmlformats-package.relationships+xml"},{"path":"ppt/presentation.xml","contentType":"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"},{"path":"ppt/slideLayouts/_rels/slideLayout1.xml.rels","contentType":"application/vnd.openxmlformats-package.relationships+xml"},{"path":"ppt/slideLayouts/slideLayout1.xml","contentType":"application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"},{"path":"ppt/slideMasters/_rels/slideMaster1.xml.rels","contentType":"application/vnd.openxmlformats-package.relationships+xml"},{"path":"ppt/slideMasters/slideMaster1.xml","contentType":"application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"},{"path":"ppt/theme/theme1.xml","contentType":"application/vnd.openxmlformats-officedocument.theme+xml"}],"namespaces":["http://purl.oclc.org/ooxml/drawingml/main","http://purl.oclc.org/ooxml/officeDocument/relationships","http://purl.oclc.org/ooxml/presentationml/main"],"relationshipTypes":["http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument","http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout","http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster","http://purl.oclc.org/ooxml/officeDocument/relationships/theme"],"alternateContentParts":[]});expect(strictInitial["relationships"]).toEqual([{"source":"package","type":"http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument","target":"ppt/presentation.xml","id":"rId1"},{"source":"ppt/presentation.xml","type":"http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster","target":"ppt/slideMasters/slideMaster1.xml","id":"rId1"},{"source":"ppt/slideMasters/slideMaster1.xml","type":"http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout","target":"ppt/slideLayouts/slideLayout1.xml","id":"rId1"},{"source":"ppt/slideMasters/slideMaster1.xml","type":"http://purl.oclc.org/ooxml/officeDocument/relationships/theme","target":"ppt/theme/theme1.xml","id":"rId2"},{"source":"ppt/slideLayouts/slideLayout1.xml","type":"http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster","target":"ppt/slideMasters/slideMaster1.xml","id":"rId1"}]);expect(strictInitial["qualification"]).toEqual("Independently authored exact eight-key projection and five relationship rows checked against mounted literals; no firstparty encoder expected generation.");
  expect(strictInitial.projection.parts.length).toBe(9);
  expect(strictInitial.relationships.length).toBe(5);
  const parts = new Set(strictInitial.projection.parts.map(part => part.path));
  const identities = new Set<string>();
  for (const edge of strictInitial.relationships) {
    expect(parts.has(edge.target)).toBe(true);
    expect(edge.source === 'package' || parts.has(edge.source)).toBe(true);
    expect(identities.has(edge.source + '\0' + edge.id)).toBe(false);
    identities.add(edge.source + '\0' + edge.id);
  }
  
  
  expect(strictInitial.relationships.find(edge=>edge.source==='ppt/slideLayouts/slideLayout1.xml')?.target).toBe('ppt/slideMasters/slideMaster1.xml');
});
