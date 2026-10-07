import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import type { ValidateFunction } from 'ajv';
import { getWorkspaceRoot } from '../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts';
import { semioSchemaAjvV1 } from '../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts';
import { runStdioTypeScriptCompiler } from '../../🏘️composition/🏗️build/🟦️.ts';
import { parseDocxDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parseXlsxDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parsePptxDiff } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';
import { parsePptxSnapshot } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
import { parsePptxSetSnapshotMutation } from '../../../../🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🦠️mutation/🟦️.ts';
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
const wireJson = (value: unknown): Json => JSON.parse(JSON.stringify(value, (_key, item) => typeof item === 'bigint' ? item.toString() : item)) as Json;
const valid = (validate: ValidateFunction, value: Json, label: string): void => assert(validate(value), `${label}: ${JSON.stringify(validate.errors)}`);
const invalid = (validate: ValidateFunction, value: Json, label: string): void => assert(!validate(value), `${label}: stale stub unexpectedly passed`);

/** 🧪️ Validates current OOXML snapshot/diff fixtures through Ajv and their complete TypeScript guards. */
export function testStdioOfficeSchemaContracts(repoRoot = getWorkspaceRoot()): void {
  const artifacts = join(repoRoot, '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts');
  const addressRoot = join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address');
  const addresses = json(join(addressRoot, '🧫️fixtures/🔣️.json')) as { valid: Json[]; invalid: Json[] };
  const validateAddress = semioSchemaAjvV1().compile(json(join(addressRoot, '🔣️.json')));
  for (const address of addresses.valid) {
    valid(validateAddress, address, 'DOCX canonical XML address');
    assert.deepEqual(parseDocxXmlAddress(address), address);
  }
  for (const address of addresses.invalid) {
    invalid(validateAddress, address, 'DOCX malformed XML address');
    assert.throws(() => parseDocxXmlAddress(address));
  }
  const xmlSnapshotSchema = json(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json'));
  const xmlDiffSchema = json(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json'));
  const mutationRoot = join(addressRoot, '..');
  const addressAjv = semioSchemaAjvV1().addSchema(json(join(addressRoot, '🔣️.json'))).addSchema(xmlSnapshotSchema);
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
    const validate = addressAjv.compile(json(join(mutationRoot, row.leaf, '🧬️schema/🔣️.json')));
    for (const address of addresses.valid) valid(validate, { ...row.payload, [row.field]: address }, `DOCX ${row.leaf} shared address`);
    for (const address of addresses.invalid) invalid(validate, { ...row.payload, [row.field]: address }, `DOCX ${row.leaf} malformed address`);
    if ('index' in row.payload) for (const index of [-1, 0.5, Number.MAX_SAFE_INTEGER + 1]) invalid(validate, { ...row.payload, [row.field]: addresses.valid[0], index }, `DOCX ${row.leaf} invalid index`);
  }
  const docxTableRoot = join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/🧭️table-run-projection');
  const docxTable = json(join(docxTableRoot, '🔣️.json')) as { blocks: Json[][]; paths: Json[][] } & { [key: string]: Json };
  assert.equal(docxTable.blocks.length, docxTable.paths.length);
  for (const [index, runs] of docxTable.blocks.entries()) assert.equal(runs.length, docxTable.paths[index].length);
  const sparse = json(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧫️fixtures/🌿️sparse-triples/🔣️.json')) as { valid: Json[]; invalid: Json[] };
  const validateXmlDiff = semioSchemaAjvV1().addSchema(xmlSnapshotSchema).compile(xmlDiffSchema);
  for (const diff of sparse.valid) {
    valid(validateXmlDiff, diff, 'XML sparse triple');
    assert.deepEqual(wireJson(parseXmlDiff(diff)), diff);
  }
  for (const diff of sparse.invalid) {
    invalid(validateXmlDiff, diff, 'XML null triple');
    assert.throws(() => parseXmlDiff(diff));
  }
  for (const row of cases) runStdioTypeScriptCompiler(join(artifacts, row.artifact, '🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts'), ['--noEmit', '--allowImportingTsExtensions']);
  runStdioTypeScriptCompiler(join(repoRoot, '✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📚️office-schema-contract/🧬️public-facets/🟦️.ts'), ['--noEmit', '--allowImportingTsExtensions']);
  for (const row of cases) {
    const base = join(artifacts, row.artifact, '🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base');
    const snapshotSchema = json(join(base, '🧬️schema/📸️snapshot/🔣️.json'));
    const diffSchema = json(join(base, '🧬️schema/🔺️diff/🔣️.json'));
    const fixture = join(base, '🧫️fixtures/🧬️mutations/📸️set-snapshot', row.slug);
    const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(xmlSnapshotSchema).addSchema(xmlDiffSchema);
    const validateSnapshot = ajv.compile(snapshotSchema);
    const validateDiff = ajv.compile(diffSchema);
    for (const state of ['⬅️before', '➡️after']) valid(validateSnapshot, json(join(fixture, '📸️snapshot', state, '🔣️.json')), `${row.artifact} ${state}`);
    const diff = json(join(fixture, '🔺️diff/🔣️.json'));
    valid(validateDiff, diff, `${row.artifact} diff`);
    assert.deepEqual(wireJson(row.parse(diff)), diff, `${row.artifact} TypeScript guard`);
    invalid(validateSnapshot, { schema: `s.stdio.${row.artifact}`, entries: [] }, `${row.artifact} legacy snapshot`);
    invalid(validateDiff, { schema: `s.stdio.${row.artifact}.diff`, bytes: [] }, `${row.artifact} legacy diff`);
  }
  for (const row of publicCases) {
    const subsets = join(artifacts, row.artifact, '🏅️standards/🔖️ecma-376/🪆️subsets');
    const schemas = row.subsets.map((subset) => json(join(subsets, subset, '🧬️schema/🔣️.json')));
    const fixture = json(join(subsets, '🧱️base/🧫️fixtures/🧬️mutations/📸️set-snapshot', row.slug, '📸️snapshot/⬅️before/🔣️.json'));
    const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(xmlSnapshotSchema);
    const validators = [ajv.compile(schemas[0]), ...schemas.slice(1).map((schema) => ajv.compile(schema))];
    for (const [index, validate] of validators.entries()) {
      valid(validate, fixture, `${row.artifact} ${row.subsets[index]} public artifact`);
      invalid(validate, { schema: `s.stdio.${row.artifact}`, entries: [] }, `${row.artifact} ${row.subsets[index]} legacy public artifact`);
    }
  }
  const docx = join(artifacts, '📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base');
  const docxDiffSchema = json(join(docx, '🧬️schema/🔺️diff/🔣️.json')) as { $id: string } & Json;
  const optionalClear = json(join(artifacts, '📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧫️fixtures/🏳️optional-clear/🔣️.json')) as { cases: { name: string; diff: Json }[] };
  const docxAjv = semioSchemaAjvV1().addSchema(xmlSnapshotSchema).addSchema(xmlDiffSchema).addSchema(docxDiffSchema);
  const validateDocxDiff = docxAjv.getSchema(docxDiffSchema.$id)!;
  for (const row of optionalClear.cases) {
    const diff = { xmlParts: { modified: [{ key: 'custom/document.xml', diff: { document: row.diff } }] } };
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
    const diff = { xmlParts: { modified: [{ key: 'custom/document.xml', diff: { document: { root: { kind: 'element', name, children: xmlChildren } } } }] } };
    valid(validateDocxDiff, diff, `DOCX canonical XML property ${name}`);
    assert.deepEqual(wireJson(parseDocxDiff(diff)), diff);
  }
  invalid(validateDocxDiff, { document: { body: {} } }, 'DOCX semantic shadow diff');
  const base = join(artifacts, '📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base');
  const schema = json(join(base, '🧬️schema/🔺️diff/🔣️.json')) as { $id: string } & Json;
  const fixture = json(join(base, '🧬️schema/🔺️diff/🧫️fixtures/🏷️placeholder-kind/🔣️.json')) as { expectedDiff: Json };
  const snapshotSchema = json(join(base, '🧬️schema/📸️snapshot/🔣️.json'));
  const ajv = semioSchemaAjvV1().addSchema(xmlSnapshotSchema).addSchema(xmlDiffSchema).addSchema(snapshotSchema).addSchema(schema);
  valid(ajv.getSchema(schema.$id)!, fixture.expectedDiff, 'PPTX canonical placeholder kind diff');
  assert.deepEqual(wireJson(parsePptxDiff(fixture.expectedDiff)), fixture.expectedDiff);
  assert.throws(() => parsePptxDiff({ presentation: {} }));
  const boundary = json(join(base, '🧫️fixtures/🧭️xml-document-boundaries/🔣️.json')) as { snapshot: Json; diff: Json; setSnapshot: { snapshot: Json } & Json; invalidXmlAttribute: Json };
  const validatePptxSnapshot = ajv.compile(snapshotSchema);
  const validatePptxReplacement = ajv.compile(json(join(base, '🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json')));
  valid(validatePptxSnapshot, boundary.snapshot, 'PPTX XML document boundary snapshot');
  valid(ajv.getSchema(schema.$id)!, boundary.diff, 'PPTX XML document boundary diff');
  valid(validatePptxReplacement, boundary.setSnapshot, 'PPTX XML document boundary replacement');
  assert.deepEqual(wireJson(parsePptxSnapshot(boundary.snapshot)), boundary.snapshot);
  assert.deepEqual(wireJson(parsePptxDiff(boundary.diff)), boundary.diff);
  assert.deepEqual(wireJson(parsePptxSetSnapshotMutation(boundary.setSnapshot).snapshot), boundary.setSnapshot.snapshot);
  const invalidBoundary = parsePptxSnapshot(boundary.snapshot);
  invalidBoundary.xmlParts[0].document.doctype!.prologPosition = 3n;
  assert.throws(() => parsePptxSnapshot(invalidBoundary));
  assert.throws(() => parsePptxDiff({ xmlParts: invalidBoundary.xmlParts }));
  assert.throws(() => parsePptxSetSnapshotMutation({ snapshot: invalidBoundary }));
  const baseline = parsePptxSnapshot(boundary.snapshot);
  const invalidPart = { ...baseline.xmlParts[0], document: { ...baseline.xmlParts[0].document, root: { kind: 'element', name: 'p:presentation', attrs: [boundary.invalidXmlAttribute], children: [] } } };
  const invalidAttributeSnapshot = wireJson({ ...baseline, xmlParts: [invalidPart] });
  const invalidAttributeDiff = wireJson({ xmlParts: [invalidPart] });
  const invalidAttributeReplacement = { mutation: 'setSnapshot', snapshot: invalidAttributeSnapshot };
  invalid(validatePptxSnapshot, invalidAttributeSnapshot, 'PPTX misplaced XML attribute field snapshot');
  invalid(ajv.getSchema(schema.$id)!, invalidAttributeDiff, 'PPTX misplaced XML attribute field diff');
  invalid(validatePptxReplacement, invalidAttributeReplacement, 'PPTX misplaced XML attribute field replacement');
  assert.throws(() => parsePptxSnapshot(invalidAttributeSnapshot));
  assert.throws(() => parsePptxDiff(invalidAttributeDiff));
  assert.throws(() => parsePptxSetSnapshotMutation(invalidAttributeReplacement));
}
