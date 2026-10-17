/** 🧪️ Layout parent facets retain the live drawing payload while enforcing its child identity. */
import { artifactReferenceSchema } from "@semio-tech/schema-ts/artifact-reference";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle } from "@semio-tech/schema-ts/oracles/document";
import { schemaSchema as ioSchema } from "@semio-tech/framework-io-schema-ts";
import { storeBlobSchema as blobSchema } from "@semio-tech/framework-os/modules/store/blob/schema";
import { storeChildSchema as childSchema } from "@semio-tech/framework-os/modules/store/child/schema";
import { storeLinkSchema as linkSchema } from "@semio-tech/framework-os/modules/store/link/schema";
import { drawingSnapshotSchema as drawingSchema } from "@semio-tech/stdio-semio/standards/v1/subsets/drawing/schema/snapshot";
import { dictionarySchema } from "@semio-tech/value-ts/dictionary";
import { subsetsAnySchema as artifactSchema } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/schema";
import { anySnapshotSchema as snapshotSchema } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/schema/snapshot";
import { anyDiffSchema as diffSchema } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/schema/diff";
import {layoutArtifactFromNativeJson,layoutArtifactNativeJson} from "@semio-tech/layout-layout-ts/standards/1/subsets/any/io/text/snapshot/native-json";
import {layoutDiffFromNativeJson,layoutDiffNativeJson} from "@semio-tech/layout-layout-ts/standards/1/subsets/any/io/text/diff/native-json";
import { parseLayoutSnapshot } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/schema/snapshot";
import { parseLayoutDiff } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/schema/diff";
import { contractDocumentFixture as vectors } from "@semio-tech/layout-layout-ts/standards/1/subsets/any/testing/fixtures/contract/document";
import { test } from "bun:test";
import { bindLayoutBackgroundDrawing } from "../🔗️binding/🟦️.ts";
bindLayoutBackgroundDrawing();

const fields = ["schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFields", "backgroundDrawing", "referencedModel"] as const;

/** 🪪️ Validates the neutral wrapper, independent Ajv schemas and every committed mutation result. */
export function testLayoutDocumentContractOracle(): void {
  const document = structuredClone(vectors.document) as Record<string, any>;
  const missingContent = structuredClone(document);
  delete missingContent.backgroundDrawing.content;
  const modelAsChild = structuredClone(document);
  modelAsChild.referencedModel = { childId: "model", target: document.referencedModel.target };
  const missingDrawingTexts = structuredClone(vectors.diff) as Record<string, unknown>;
  delete missingDrawingTexts.drawingTexts;
  const invalidEntry = structuredClone(vectors.diff);
  invalidEntry.dataFields.entries.inserted[0].row = { questionId: "unsigned", value: { kind: "unsigned", high: -1, low: 0 } } as (typeof invalidEntry.dataFields.entries.inserted)[0]["row"];
  assertDocumentContractOracle({
    name: "Layout",
    dependencies: [ioSchema, blobSchema, childSchema, linkSchema, drawingSchema, dictionarySchema,artifactReferenceSchema],
    artifact: { schema: artifactSchema, parse: layoutArtifactFromNativeJson, nativeJson:value=>layoutArtifactNativeJson(value as ReturnType<typeof layoutArtifactFromNativeJson>) },
    snapshot: { schema: snapshotSchema, parse: layoutArtifactFromNativeJson, nativeJson:value=>layoutArtifactNativeJson(value as ReturnType<typeof layoutArtifactFromNativeJson>) },
    diff: { schema: diffSchema, parse: layoutDiffFromNativeJson, nativeJson:value=>layoutDiffNativeJson(value as ReturnType<typeof layoutDiffFromNativeJson>) },
    validDiffs: [{ input: vectors.diff, output: vectors.diff }],
    invalidDiffs: [{ ...vectors.diff, artifact: null }, missingDrawingTexts, { ...vectors.diff, dataFields: { dictionary: vectors.document.dataFields } }, invalidEntry],
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: [{ ...document, camera: { x: 0, y: 0, zoom: 1 } }, missingContent, modelAsChild],
    mutationRoots: [fileURLToPath(new URL("../../../../🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 102, diffs: 45 },
  });
  assert.deepEqual(Object.keys(artifactSchema.properties), fields);
  assert.deepEqual(Object.keys(snapshotSchema.properties), fields);
  assert.deepEqual(Object.keys(diffSchema.properties), [...fields.slice(0,-1), "drawingTexts", fields.at(-1)]);
  assert.deepEqual(layoutDiffNativeJson(layoutDiffFromNativeJson(vectors.diff)), vectors.diff);
  const data = layoutDiffFromNativeJson(vectors.diff).dataFields!.entries!;
  assert.deepEqual(data.inserted[0].row.value, { kind: "unsigned", value: (2n ** 64n) - 1n });
  assert.deepEqual(data.inserted[1].row.value, { kind: "bytes", value: Uint8Array.of(0, 128, 255) });
  assert.deepEqual(data.inserted[2].row.value, { kind: "signed", value: -1n });
  assert.deepEqual(data.inserted[3].row.value, { kind: "float", value: { bits: 1n << 63n } });
  const child = layoutArtifactFromNativeJson(document).backgroundDrawing!;
  assert.equal(child.handle.childId, child.handle.target.artifactId);
  assert.deepEqual(child.handle.target.dialect, { artifactKind: "s.stdio.semio", standard: "v1", subset: "drawing" });
  assert.deepEqual((layoutArtifactNativeJson(layoutArtifactFromNativeJson(document)) as typeof document).backgroundDrawing.content, document.backgroundDrawing.content, "inline drawing remains available to live exporters");
  const wrongIdentity = structuredClone(document);
  wrongIdentity.backgroundDrawing.handle.target.artifactId = "wrong";
  assert.equal(layoutArtifactFromNativeJson(wrongIdentity).backgroundDrawing!.handle.target.artifactId,"wrong");
}

test("Layout document contract oracle holds with the Semio drawing bound", () => { testLayoutDocumentContractOracle(); });
