import artifactReferenceSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import {parseNoteArtifactJson,parseNoteDiffJson,parseNoteSnapshotJson} from "../../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
/** 🧪️ Note document boundaries preserve native block payloads and shared link identity. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import linkSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🔣️.json" with { type: "json" };
import blobSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🔣️.json" with { type: "json" };
import vectors from "../../🧫️fixtures/🪪️document/🔣️.json" with { type: "json" };
import {binary64Value} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type { NoteBlockNode, NoteImageAsset, NoteArtifact } from "../../🟦️.ts";
import type { NoteDiff } from "../../🔺️diff/🟦️.ts";

function assetJson(value: NoteImageAsset): unknown {
  return { ...value, ...("width" in value ? { width: value.width == null ? value.width : binary64Value(value.width) } : {}), ...("height" in value ? { height: value.height == null ? value.height : binary64Value(value.height) } : {}) };
}
function blockJson(value: NoteBlockNode): unknown {
  const frame = { ...value, x: binary64Value(value.x), y: binary64Value(value.y), width: binary64Value(value.width), height: binary64Value(value.height), ...("rotation" in value ? { rotation: binary64Value(value.rotation!) } : {}) };
  switch (value.kind) {
    case "text": return { ...frame, fontSize: binary64Value(value.fontSize) };
    case "stroke": return { ...frame, strokeWidth: binary64Value(value.strokeWidth), color: value.color.map(binary64Value), points: value.points.map(point => point.map(binary64Value)) };
    case "group": return { ...frame, children: value.children.map(blockJson) };
    default: return frame;
  }
}
function documentJson(input: unknown): unknown {
  const value = input as NoteArtifact;
  const result: Record<string, unknown> = { ...value };
  for (const key of ["gridSpacing", "gridSubdivisions", "gridOpacity", "snapGridSpacing", "pencilWidth", "eraserRadius"] as const) if (key in value) result[key] = value[key] == null ? value[key] : binary64Value(value[key]!);
  if ("blocks" in value) result.blocks = value.blocks.map(blockJson);
  if ("assets" in value) result.assets = Object.fromEntries(Object.entries(value.assets).map(([key, value]) => [key, assetJson(value)]));
  return result;
}
function diffJson(input: unknown): unknown {
  const value = input as NoteDiff;
  const result: Record<string, unknown> = { ...value };
  for (const key of ["gridSpacing", "gridSubdivisions", "gridOpacity", "snapGridSpacing", "pencilWidth", "eraserRadius"] as const) if (key in value) result[key] = value[key] == null ? value[key] : binary64Value(value[key]!);
  if (value.artifact != null) result.artifact = documentJson(value.artifact);
  if (value.blocks != null) result.blocks = { ...value.blocks, added: value.blocks.added.map(entry => ({ ...entry, block: blockJson(entry.block) })), patched: value.blocks.patched.map(entry => ({...entry,patch:{...entry.patch,...(entry.patch.block == null ? {} : {block:blockJson(entry.patch.block)})}})) };
  if (value.assets != null) result.assets = { entries: Object.fromEntries(Object.entries(value.assets.entries).map(([key, value]) => [key, value == null ? value : assetJson(value)])) };
  return result;
}

/** 🗒️ Compares production parsers with Ajv and all committed native mutation records. */
export async function testNoteDocumentContractOracle(): Promise<void> {
  const root = join(dirname(fileURLToPath(import.meta.url)), "../..");
  const facets = await Promise.all(["", "📸️snapshot", "🔺️diff"].map(async (facet) => ({ schema: (await import(pathToFileURL(join(root, facet, "🔣️.json")).href)).default, module: await import(pathToFileURL(join(root, facet, "🟦️.ts")).href) })));
  const dependencies = [ioSchema, childSchema, blobSchema, linkSchema];
  assertDocumentContractOracle({
    name: "Note",
    dependencies,
    artifact: { schema: facets[0].schema, parse: parseNoteArtifactJson, nativeJson: documentJson },
    snapshot: { schema: facets[1].schema, parse: parseNoteSnapshotJson, nativeJson: documentJson },
    diff: { schema: facets[2].schema, parse: parseNoteDiffJson, nativeJson: diffJson },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: [...Object.entries(vectors.invalidFields).map(([field, value]) => ({ ...vectors.document, [field]: value })), ...vectors.invalidBlocks.map((block) => ({ ...vectors.document, blocks: [block] }))],
    invalidDiffs: Object.entries(vectors.invalidFields).map(([field, value]) => ({ [field]: value })),
    mutationRoots: readdirSync(join(root, "../..")).map((subset) => join(root, "../..", subset, "🧫️fixtures/🧬️mutations")).filter(existsSync),
    committed: vectors.committed,
  });
  const ajv = new Ajv({ strict: false, validateFormats: false }).addSchema(artifactReferenceSchema);
  for (const schema of [...dependencies, facets[0].schema]) ajv.addSchema(schema);
  const validate = ajv.compile(facets[2].schema);
  for (const diff of vectors.validDiffs) {
    assert.equal(validate(diff), true, JSON.stringify(validate.errors));
    assert.deepEqual(diffJson(parseNoteDiffJson(diff)), diff);
  }
}
