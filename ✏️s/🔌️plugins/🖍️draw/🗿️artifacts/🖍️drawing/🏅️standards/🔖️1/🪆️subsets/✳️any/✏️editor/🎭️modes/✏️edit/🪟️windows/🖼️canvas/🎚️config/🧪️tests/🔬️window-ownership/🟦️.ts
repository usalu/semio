import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import { applyDrawingCanvasWindowConfigMutation, type DrawingCanvasWindowConfig, type DrawingCanvasWindowConfigMutation } from "../../🧬️schema/🟦️.ts";
import { applyDrawingCanvasWindowTransientMutation, type DrawingCanvasWindowTransient, type DrawingCanvasWindowTransientMutation } from "../../../🫧️transient/🧬️schema/🟦️.ts";
import { parseDrawingPresence } from "../../../../../../../👥️presence/🧬️schema/🟦️.ts";

type WindowInstance = { id: string; windowKindId: string };
type Fixture = {
  document: unknown;
  presence: unknown;
  windowInstances: WindowInstance[];
  baseConfig: DrawingCanvasWindowConfig;
  baseTransient: DrawingCanvasWindowTransient;
  configMutations: { windowId: string; windowKindId: string; mutation: DrawingCanvasWindowConfigMutation }[];
  transientMutations: { windowId: string; windowKindId: string; mutation: DrawingCanvasWindowTransientMutation }[];
  rejections: { windowId: string; claimedWindowKindId: string; code: string }[];
};

const here = fileURLToPath(new URL(".", import.meta.url));
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const fixture = readJson(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`) as Fixture;
const configSchema = readJson(`${here}/../../🧬️schema/🔣️.json`);
const transientSchema = readJson(`${here}/../../../🫧️transient/🧬️schema/🔣️.json`);
const presenceSchema = readJson(`${here}/../../../../../../../👥️presence/🧬️schema/🔣️.json`);
const viewportSchema = readJson(`${here}/../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json`);

const target = (instances: WindowInstance[], id: string, expectedKind: string): void => {
  const instance = instances.find((candidate) => candidate.id === id);
  if (!instance) throw new Error("drawing-window-stale");
  if (instance.windowKindId !== expectedKind) throw new Error("drawing-window-kind-mismatch");
};

export function testDrawingCanvasWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(viewportSchema);
  assert(ajv.compile(configSchema)(fixture.baseConfig));
  assert(ajv.compile(transientSchema)(fixture.baseTransient));
  assert(ajv.compile(presenceSchema)(fixture.presence));
  assert.deepEqual(parseDrawingPresence(fixture.presence), fixture.presence);
  const documentBytes = JSON.stringify(fixture.document);
  const configs: Record<string, DrawingCanvasWindowConfig> = {};
  const configOracle: Record<string, DrawingCanvasWindowConfig> = {};
  for (const row of fixture.configMutations) {
    target(fixture.windowInstances, row.windowId, row.windowKindId);
    configs[row.windowId] = applyDrawingCanvasWindowConfigMutation(configs[row.windowId] ?? fixture.baseConfig, row.mutation);
    configOracle[row.windowId] = applyPatch(structuredClone(configOracle[row.windowId] ?? fixture.baseConfig), [{ op: "replace", path: "", value: row.mutation.config }], false, false).newDocument;
  }
  assert.deepEqual(configs, configOracle);
  assert.notDeepEqual(configs["drawing-left"].viewport, configs["drawing-right"].viewport);
  const transients: Record<string, DrawingCanvasWindowTransient> = {};
  const transientOracle: Record<string, DrawingCanvasWindowTransient> = {};
  for (const row of fixture.transientMutations) {
    target(fixture.windowInstances, row.windowId, row.windowKindId);
    transients[row.windowId] = applyDrawingCanvasWindowTransientMutation(transients[row.windowId] ?? fixture.baseTransient, row.mutation);
    transientOracle[row.windowId] = applyPatch(structuredClone(transientOracle[row.windowId] ?? fixture.baseTransient), [{ op: "replace", path: "", value: row.mutation.transient }], false, false).newDocument;
  }
  assert.deepEqual(transients, transientOracle);
  assert.deepEqual(JSON.parse(JSON.stringify(configs)), configs);
  assert.equal(JSON.stringify(fixture.document), documentBytes);
  for (const rejection of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejection.windowId, rejection.claimedWindowKindId), new RegExp(rejection.code));
  console.log(`[DEBUG] drawing-canvas-window-ownership configs=${Object.keys(configs).length} transients=${Object.keys(transients).length} reload=config-restored-transient-reset`);
}

testDrawingCanvasWindowOwnershipOracle();
