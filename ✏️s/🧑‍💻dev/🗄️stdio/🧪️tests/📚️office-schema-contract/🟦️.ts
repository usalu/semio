import type { ScriptInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import contractFixture from './🧫️fixtures/🔣️.json';
import type { ValidateFunction } from 'ajv';
import { getWorkspaceRoot } from '../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts';
import { semioSchemaAjvV1, addSemioMutationLeafSchemasV1 } from '@semio-tech/schema-ts/oracles/validator';
import { runStdioTypeScriptCompiler } from '../../🏘️composition/🏗️build/🟦️.ts';
import { parseDocxDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parseXlsxDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parsePptxDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parsePptxSnapshot } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
import { parsePptxSnapshotJson } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts';
import { parsePptxMutation } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts';
import { parseXmlDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parseDocxXmlAddress } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts';

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type OfficeCase = Readonly<{ artifact: string; slug: string; parse: (value: unknown) => unknown }>;
type PublicOfficeCase = Readonly<{ artifact: string; slug: string; subsets: readonly string[] }>;

const cases: readonly OfficeCase[] = [
  { artifact: '📜️docx', slug: '🅱️bolds', parse: parseDocxDiff },
  { artifact: '📕️xlsx', slug: '🧮️widens', parse: parseXlsxDiff },
  { artifact: '📽️pptx', slug: '🏷️retitles', parse: parsePptxDiff },
];
const publicCases: readonly PublicOfficeCase[] = [
  { artifact: '📜️docx', slug: '🅱️bolds', subsets: ['🧱️base', '📏️strict', '🔄️transitional'] },
  { artifact: '📕️xlsx', slug: '🧮️widens', subsets: ['🧱️base', '🔒️strict', '🌉️transitional'] },
  { artifact: '📽️pptx', slug: '🏷️retitles', subsets: ['🧱️base', '🔒️strict', '🌉️transitional'] },
];

const json = (path: string): Json => JSON.parse(readFileSync(path, 'utf8')) as Json;
const schemaJson = (path: string) => { const value=json(path);if(typeof value!=="boolean"&&(value===null||typeof value!=="object"||Array.isArray(value)))throw Error("Actual JSON Schema object or boolean required");return value; };
const identifiedSchemaJson = (path: string) => { const value=schemaJson(path);if(typeof value==="boolean"||typeof value.$id!=="string")throw Error("Identified actual JSON Schema required");return {...value,$id:value.$id}; };
const wireJson = (value: unknown): Json => JSON.parse(JSON.stringify(value, (_key, item) => typeof item === 'bigint' ? item.toString() : item)) as Json;
const valid = (validate: ValidateFunction, value: unknown, label: string): void => assert(validate(value), `${label}: ${JSON.stringify(validate.errors)}`);
const invalid = (validate: ValidateFunction, value: unknown, label: string): void => assert(!validate(value), `${label}: stale stub unexpectedly passed`);

/** 🧪️ Validates current OOXML snapshot/diff fixtures through Ajv and their complete TypeScript guards. */
export function testStdioOfficeSchemaFixtures(repoRoot = getWorkspaceRoot()): void {
  const artifacts = join(repoRoot, '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts');
  const addressRoot = join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address');
  const addresses = json(join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/🧬️mutations/🧭️xml-address/🔣️.json')) as { valid: Json[]; invalid: Json[] };
  const validateAddress = semioSchemaAjvV1().compile(schemaJson(join(addressRoot, '🔣️.json')));
  for (const address of addresses.valid) {
    valid(validateAddress, address, 'DOCX canonical XML address');
    assert.deepEqual(parseDocxXmlAddress(address), address);
  }
  for (const address of addresses.invalid) {
    invalid(validateAddress, address, 'DOCX malformed XML address');
    assert.throws(() => parseDocxXmlAddress(address));
  }
  const xmlSnapshotSchema = schemaJson(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json'));
  const xmlDiffSchema = schemaJson(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json'));
  const mutationRoot = join(addressRoot, '..');
  const addressAjv = semioSchemaAjvV1().addSchema(schemaJson(join(addressRoot, '🔣️.json'))).addSchema(xmlSnapshotSchema);
  const addressedLeaves: { leaf: string; field: string; payload: { [key: string]: Json } }[] = [
    { leaf: '🔤set-run-text', field: 'address', payload: { mutation: 'setRunText', text: 'changed' } },
    { leaf: '🧩️replace-xml-node', field: 'address', payload: { mutation: 'replaceXmlNode', node: { kind: 'text', text: 'restored' } } },
    { leaf: '🎨set-run-formatting', field: 'address', payload: { mutation: 'setRunFormatting', bold: true, italic: false, underline: true } },
    { leaf: '🖌️set-paragraph-style', field: 'address', payload: { mutation: 'setParagraphStyle', styleId: null } },
    { leaf: '➕️insert-table-row', field: 'address', payload: { mutation: 'insertTableRow', index: 0, cells: ['Grüße', '文字'] } },
    { leaf: '➖️remove-table-row', field: 'address', payload: { mutation: 'removeTableRow', index: 0 } },
    { leaf: '🧩️insert-xml-node', field: 'parent', payload: { mutation: 'insertXmlNode', index: 0, node: { kind: 'text', text: 'inserted' } } },
    { leaf: '🧹️remove-xml-node', field: 'parent', payload: { mutation: 'removeXmlNode', index: 0, expectedName: '#text', revision: '0123456789abcdef' } },
  ];
  for (const row of addressedLeaves) {
    const validate = addressAjv.compile(schemaJson(join(mutationRoot, row.leaf, '🧬️schema/🔣️.json')));
    for (const address of addresses.valid) valid(validate, { ...row.payload, [row.field]: address }, `DOCX ${row.leaf} shared address`);
    for (const address of addresses.invalid) invalid(validate, { ...row.payload, [row.field]: address }, `DOCX ${row.leaf} malformed address`);
    if ('index' in row.payload) for (const index of [-1, 0.5, Number.MAX_SAFE_INTEGER + 1]) invalid(validate, { ...row.payload, [row.field]: addresses.valid[0]!, index }, `DOCX ${row.leaf} invalid index`);
  }
  const docxTableRoot = join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/🧭️table-run-projection');
  const docxTable = json(join(docxTableRoot, '🔣️.json')) as { blocks: Json[][]; paths: Json[][] } & { [key: string]: Json };
  assert.equal(docxTable.blocks.length, docxTable.paths.length);
  for (const [index, runs] of docxTable.blocks.entries()) assert.equal(runs.length, docxTable.paths[index]!.length);
  const sparse = json(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/🔺️diff/🌿️sparse-triples/🔣️.json')) as { valid: Json[]; invalid: Json[] };
  const validateXmlDiff = semioSchemaAjvV1().addSchema(xmlSnapshotSchema).compile(xmlDiffSchema);
  for (const diff of sparse.valid) {
    valid(validateXmlDiff, diff, 'XML sparse triple');
    assert.deepEqual(wireJson(parseXmlDiff(diff)), diff);
  }
  for (const diff of sparse.invalid) {
    invalid(validateXmlDiff, diff, 'XML null triple');
    assert.throws(() => parseXmlDiff(diff));
  }
  for (const row of cases) {
    const base = join(artifacts, row.artifact, '🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base');
    const snapshotSchema = schemaJson(join(base, '🧬️schema/📸️snapshot/🔣️.json'));
    const diffSchema = schemaJson(join(base, '🧬️schema/🔺️diff/🔣️.json'));
    const fixture = contractFixture.snapshots[row.artifact as keyof typeof contractFixture.snapshots];
    const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(xmlSnapshotSchema).addSchema(xmlDiffSchema);
    const validateSnapshot = ajv.compile(snapshotSchema);
    const validateDiff = ajv.compile(diffSchema);
    for (const state of ['⬅️before', '➡️after']) valid(validateSnapshot, fixture[state as keyof typeof fixture] as Json, `${row.artifact} ${state}`);
    const diff = contractFixture.diff;
    valid(validateDiff, diff, `${row.artifact} diff`);
    assert.deepEqual(wireJson(row.parse(diff)), diff, `${row.artifact} TypeScript guard`);
    invalid(validateSnapshot, { schema: `s.stdio.${row.artifact}`, entries: [] }, `${row.artifact} legacy snapshot`);
    invalid(validateDiff, { schema: `s.stdio.${row.artifact}.diff`, bytes: [] }, `${row.artifact} legacy diff`);
  }
  for (const row of publicCases) {
    const subsets = join(artifacts, row.artifact, '🏅️standards/🔖️ecma-376/🪆️subsets');
    const schemas = row.subsets.map((subset) => schemaJson(join(subsets, subset, '🧬️schema/🔣️.json')));
    const fixture = contractFixture.snapshots[row.artifact as keyof typeof contractFixture.snapshots]['⬅️before'];
    const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(xmlSnapshotSchema);
    const validators = [ajv.compile(schemas[0]!), ...schemas.slice(1).map((schema) => ajv.compile(schema))];
    for (const [index, validate] of validators.entries()) {
      valid(validate, fixture, `${row.artifact} ${row.subsets[index]} public artifact`);
      invalid(validate, { schema: `s.stdio.${row.artifact}`, entries: [] }, `${row.artifact} ${row.subsets[index]} legacy public artifact`);
    }
  }
  const docx = join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base');
  const docxDiffSchema = identifiedSchemaJson(join(docx, '🧬️schema/🔺️diff/🔣️.json'));
  const optionalClear = json(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/🔺️diff/🏳️optional-clear/🔣️.json')) as { cases: { name: string; diff: Json }[] };
  const docxAjv = semioSchemaAjvV1().addSchema(xmlSnapshotSchema).addSchema(xmlDiffSchema).addSchema(docxDiffSchema);
  const validateDocxDiff = docxAjv.getSchema(docxDiffSchema.$id)!;
  for (const row of optionalClear.cases) {
    const diff = { xmlParts: { modified: [{ id: 'custom/document.xml', patch: { document: row.diff } }] } };
    valid(validateDocxDiff, diff, `DOCX canonical XML ${row.name}`);
    assert.deepEqual(wireJson(parseDocxDiff(diff)), diff);
  }
  const xmlChildren = {
    removed: [],
    modified: [{ index: 0, diff: { kind: 'element', attributes: { removed: [], modified: [], added: [], order: ['second', 'first'] } } }],
    added: [{ index: 0, item: { kind: 'comment', text: 'retained' } }],
  };
  const propertyNodes = ['w:rPr', 'w:pPr', 'w:tcPr', 'w:trPr', 'w:tblPr'];
  for (const name of propertyNodes) {
    const diff = { xmlParts: { modified: [{ id: 'custom/document.xml', patch: { document: { root: { kind: 'element', name, children: xmlChildren } } } }] } };
    valid(validateDocxDiff, diff, `DOCX canonical XML property ${name}`);
    assert.deepEqual(wireJson(parseDocxDiff(diff)), diff);
  }
  invalid(validateDocxDiff, { document: { body: {} } }, 'DOCX semantic shadow diff');
  const base = join(artifacts, '📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base');
  const schema = identifiedSchemaJson(join(base, '🧬️schema/🔺️diff/🔣️.json'));
  const placeholderDiff = { xmlParts: { modified: [{ id: 'ppt/slides/slide1.xml', patch: { document: { root: { kind: 'element', attributes: { modified: [{ name: 'type', value: 'title' }] } } } } }] } };
  const snapshotSchema = schemaJson(join(base, '🧬️schema/📸️snapshot/🔣️.json'));
  const ajv = semioSchemaAjvV1().addSchema(xmlSnapshotSchema).addSchema(xmlDiffSchema).addSchema(snapshotSchema).addSchema(schema);
  valid(ajv.getSchema(schema.$id)!, placeholderDiff, 'PPTX canonical placeholder kind diff');
  assert.deepEqual(wireJson(parsePptxDiff(placeholderDiff)), placeholderDiff);
  invalid(ajv.getSchema(schema.$id)!, { presentation: {} }, 'PPTX removed semantic shadow');
  assert.throws(() => parsePptxDiff({ xmlParts: null }));
  const boundary = json(join(base, '🧪️tests/🧫️fixtures/🧭️xml-document-boundaries/🔣️.json')) as { snapshot: Json; invalidXmlAttribute: Json };
  const validatePptxSnapshot = ajv.compile(snapshotSchema);
  const mutations = join(base, '🧬️schema/🧬️mutations');
  const mutationAjv = addSemioMutationLeafSchemasV1(semioSchemaAjvV1().addSchema(xmlSnapshotSchema).addSchema(snapshotSchema).addSchema(schemaJson(join(mutations, '🧭️xml-address/🔣️.json'))), pathToFileURL(mutations));
  const validatePptxMutation = mutationAjv.compile(schemaJson(join(mutations, '🔣️.json')));
  assert.equal(contractFixture.pptx.mutations.length, 12);
  for (const mutation of contractFixture.pptx.mutations) { valid(validatePptxMutation, mutation, 'PPTX canonical mutation'); assert.deepEqual(wireJson(parsePptxMutation(mutation)), mutation); }
  for (const address of contractFixture.pptx.invalidAddresses) { const mutation = { mutation: 'replaceXmlNode', address, node: contractFixture.pptx.mutations[7]!.node }; invalid(validatePptxMutation, mutation, 'PPTX malformed revision address'); assert.throws(() => parsePptxMutation(mutation)); }
  assert.throws(() => parsePptxMutation({ mutation: 'setSnapshot', snapshot: boundary.snapshot }));
  valid(validatePptxSnapshot, boundary.snapshot, 'PPTX XML document boundary snapshot');
  const typedBoundary = parsePptxSnapshotJson(boundary.snapshot);
  const boundaryDiff = { xmlParts: { inserted: [{ index: 0, row: typedBoundary.xmlParts[0] }] } };
  valid(ajv.getSchema(schema.$id)!, wireJson(boundaryDiff), 'PPTX XML document boundary diff');
  assert.deepEqual(wireJson(parsePptxSnapshot(typedBoundary)), boundary.snapshot);
  assert.deepEqual(wireJson(parsePptxDiff(boundaryDiff)), wireJson(boundaryDiff));
  const invalidBoundary = parsePptxSnapshot(typedBoundary);
  invalidBoundary.xmlParts[0]!.document.doctype!.prologPosition = -1n;
  invalid(validatePptxSnapshot, wireJson(invalidBoundary), 'PPTX unsigned doctype position');
  assert.throws(() => parsePptxSnapshot(invalidBoundary));
  assert.throws(() => parsePptxDiff({ xmlParts: { inserted: [{ index: 0, row: invalidBoundary.xmlParts[0] }] } }));
  const baseline = parsePptxSnapshot(typedBoundary);
  const invalidPart = { ...baseline.xmlParts[0], document: { ...baseline.xmlParts[0]!.document, root: { kind: 'element', name: 'p:presentation', attrs: [boundary.invalidXmlAttribute], children: [] } } };
  const invalidAttributeSnapshot = wireJson({ ...baseline, xmlParts: [invalidPart] });
  const invalidAttributeDiff = { xmlParts: { inserted: [{ index: 0, row: invalidPart }] } };
  const invalidAttributeReplacement = { mutation: 'replaceXmlNode', address: contractFixture.pptx.address, node: invalidPart.document.root };
  invalid(validatePptxSnapshot, invalidAttributeSnapshot, 'PPTX misplaced XML attribute field snapshot');
  invalid(ajv.getSchema(schema.$id)!, wireJson(invalidAttributeDiff), 'PPTX misplaced XML attribute field diff');
  invalid(validatePptxMutation, invalidAttributeReplacement, 'PPTX misplaced XML attribute field replacement');
  assert.throws(() => parsePptxSnapshotJson(invalidAttributeSnapshot));
  assert.throws(() => parsePptxDiff(invalidAttributeDiff));
  assert.throws(() => parsePptxMutation(invalidAttributeReplacement));
}

/** 🧪️ Runs current schema/parser laws before the original owned TypeScript compiler checks. */
export async function testStdioOfficeSchemaContracts(invocation: ScriptInvocation, repoRoot = getWorkspaceRoot()): Promise<void> {
  testStdioOfficeSchemaFixtures(repoRoot);
  for (const row of cases) await runStdioTypeScriptCompiler(join(repoRoot, '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts', row.artifact, '🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts'), invocation);
  await runStdioTypeScriptCompiler(join(repoRoot, '✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📚️office-schema-contract/🧬️public-facets/🟦️.ts'), invocation);
}
