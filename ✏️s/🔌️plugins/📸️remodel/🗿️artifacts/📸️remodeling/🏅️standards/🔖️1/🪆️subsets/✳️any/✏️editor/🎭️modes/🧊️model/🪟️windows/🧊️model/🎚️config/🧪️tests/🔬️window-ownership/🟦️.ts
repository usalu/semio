import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv, { type AnySchema } from "ajv/dist/2020";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import { applyPatch } from "fast-json-patch";
import { applyRemodelingModelWindowConfigMutation, parseRemodelingModelWindowConfig, type RemodelingModelWindowConfig, type RemodelingModelWindowConfigMutation } from "../../🧬️schema/🟦️.ts";
import { applyRemodelingFramesWindowConfigMutation, type RemodelingFramesWindowConfig, type RemodelingFramesWindowConfigMutation } from "../../../../../../📷️capture/🪟️windows/🖼️frames/🎚️config/🧬️schema/🟦️.ts";
import { applyRemodelingReportWindowConfigMutation, type RemodelingReportWindowConfig, type RemodelingReportWindowConfigMutation } from "../../../../../../🔍️analyze/🪟️windows/📊️report/🎚️config/🧬️schema/🟦️.ts";

type Owner = "model" | "frames" | "report";
type WindowInstance = { id: string; windowKindId: string };
type OwnerConfig = { model: RemodelingModelWindowConfig; frames: RemodelingFramesWindowConfig; report: RemodelingReportWindowConfig };
type OwnerMutation = { model: RemodelingModelWindowConfigMutation; frames: RemodelingFramesWindowConfigMutation; report: RemodelingReportWindowConfigMutation };
type MutationRow<O extends Owner = Owner> = { owner: O; windowId: string; windowKindId: string; mutation: OwnerMutation[O] };
type Fixture = {
  document: unknown;
  windowInstances: WindowInstance[];
  base: OwnerConfig;
  mutations: MutationRow[];
  rejections: { windowId: string; claimedWindowKindId: string; code: string }[];
};

const here = fileURLToPath(new URL(".", import.meta.url));
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const fixture = readJson(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`) as Fixture;
const schemaRoot = `${here}/../../../../../../`;
const schemas: Record<Owner, AnySchema> = {
  model: readJson(`${here}/../../🧬️schema/🔣️.json`),
  frames: readJson(`${schemaRoot}/📷️capture/🪟️windows/🖼️frames/🎚️config/🧬️schema/🔣️.json`),
  report: readJson(`${schemaRoot}/🔍️analyze/🪟️windows/📊️report/🎚️config/🧬️schema/🔣️.json`),
};
const viewportSchema = readJson(`${here}/../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔣️.json`);
const kinds: Record<Owner, string> = { model: "remodeling-main", frames: "remodeling-frames", report: "remodeling-report" };

const target = (instances: WindowInstance[], id: string, kind: string): void => {
  const instance = instances.find((candidate) => candidate.id === id);
  if (!instance) throw new Error("remodel-window-stale");
  if (instance.windowKindId !== kind) throw new Error("remodel-window-kind");
};

const apply = <O extends Owner>(owner: O, base: OwnerConfig[O], mutation: OwnerMutation[O]): OwnerConfig[O] => {
  if (owner === "model") return applyRemodelingModelWindowConfigMutation(base as RemodelingModelWindowConfig, mutation as RemodelingModelWindowConfigMutation) as OwnerConfig[O];
  if (owner === "frames") return applyRemodelingFramesWindowConfigMutation(base as RemodelingFramesWindowConfig, mutation as RemodelingFramesWindowConfigMutation) as OwnerConfig[O];
  return applyRemodelingReportWindowConfigMutation(base as RemodelingReportWindowConfig, mutation as RemodelingReportWindowConfigMutation) as OwnerConfig[O];
};

export function testRemodelWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addMetaSchema(draft7);
  ajv.addKeyword({ keyword: "x-semio-state", metaSchema: { type: "string" } });
  ajv.addSchema(viewportSchema);
  const validate = Object.fromEntries(Object.entries(schemas).map(([owner, schema]) => [owner, ajv.compile(schema)])) as Record<Owner, ReturnType<Ajv["compile"]>>;
  for (const owner of Object.keys(fixture.base) as Owner[]) assert(validate[owner](fixture.base[owner]), JSON.stringify(validate[owner].errors));
  assert.deepEqual(parseRemodelingModelWindowConfig(fixture.base.model), fixture.base.model);
  const documentBytes = JSON.stringify(fixture.document);
  const actual: Record<string, OwnerConfig[Owner]> = {};
  const oracle: Record<string, OwnerConfig[Owner]> = {};
  for (const row of fixture.mutations) {
    target(fixture.windowInstances, row.windowId, kinds[row.owner]);
    const base = structuredClone(fixture.base[row.owner]);
    actual[row.windowId] = apply(row.owner, base, row.mutation);
    oracle[row.windowId] = applyPatch(base, [{ op: "replace", path: "", value: row.mutation.config }], false, false).newDocument;
    assert(validate[row.owner](actual[row.windowId]), JSON.stringify(validate[row.owner].errors));
    if (row.owner === "model") assert.deepEqual(parseRemodelingModelWindowConfig(actual[row.windowId]), actual[row.windowId]);
  }
  assert.deepEqual(actual, oracle);
  assert.notDeepEqual(actual["remodel-model-left"], actual["remodel-model-right"]);
  assert.notDeepEqual(actual["remodel-frames-left"], actual["remodel-frames-right"]);
  assert.notDeepEqual(actual["remodel-report-left"], actual["remodel-report-right"]);
  assert.equal(JSON.stringify(fixture.document), documentBytes);
  for (const rejection of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejection.windowId, rejection.claimedWindowKindId), new RegExp(rejection.code));
  console.log("[DEBUG] remodel-window-ownership windows=6 owners=3 schemas=ajv implementation=typescript oracle=json-patch document=stable");
}

testRemodelWindowOwnershipOracle();
