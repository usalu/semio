import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv, { type AnySchema } from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";

type SpawnedAppEntry = {
  id: string;
  pluginId: string;
  instanceId: number;
  appId: string;
  label: string;
  breadcrumb: string[];
};

type HostPanelState = {
  activePanelTab: string;
  spawnedApps: SpawnedAppEntry[];
  activeSpawnedId?: string;
};

type Fixture = {
  capacities: { panelJsonChars: number; identifierChars: number; spawnedApps: number };
  base: HostPanelState;
  selectionPatch: Operation[];
  expected: HostPanelState;
  configuredPanelLeaves: { valid: string[]; invalid: string[] };
};

const schema = JSON.parse(readFileSync(new URL("../../🔣️.json", import.meta.url), "utf8")) as AnySchema;
const fixture = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/📌️panel-state/🔣️.json", import.meta.url), "utf8")) as Fixture;
const control = /[\u0000-\u001f\u007f]/u;

function identifier(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && Array.from(value).length <= fixture.capacities.identifierChars && !control.test(value);
}

function independentState(value: unknown): value is HostPanelState {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !["activePanelTab", "spawnedApps", "activeSpawnedId"].includes(key))) return false;
  if (!identifier(row.activePanelTab)) return false;
  if (!Array.isArray(row.spawnedApps) || row.spawnedApps.length > fixture.capacities.spawnedApps) return false;
  const ids = new Set<string>();
  for (const value of row.spawnedApps) {
    if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
    const entry = value as Record<string, unknown>;
    if (Object.keys(entry).sort().join("|") !== "appId|breadcrumb|id|instanceId|label|pluginId") return false;
    if (!identifier(entry.id) || !identifier(entry.pluginId) || !identifier(entry.appId) || ids.has(entry.id)) return false;
    if (!Number.isInteger(entry.instanceId) || (entry.instanceId as number) < 0 || (entry.instanceId as number) > 0xffff_ffff) return false;
    if (typeof entry.label !== "string" || Array.from(entry.label).length > fixture.capacities.panelJsonChars) return false;
    if (!Array.isArray(entry.breadcrumb) || entry.breadcrumb.length > fixture.capacities.spawnedApps || entry.breadcrumb.some((part) => typeof part !== "string" || Array.from(part).length > fixture.capacities.panelJsonChars)) return false;
    ids.add(entry.id);
  }
  if (row.activeSpawnedId !== undefined && (!identifier(row.activeSpawnedId) || !ids.has(row.activeSpawnedId))) return false;
  return Array.from(JSON.stringify(value)).length <= fixture.capacities.panelJsonChars;
}

/** 📌️ Proves the neutral host panel state is strict, bounded and preserves spawned ownership. */
export function testHostPanelStateSchema(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = ajv.compile(schema);
  const schemaBounds = schema as AnySchema & {
    properties: { spawnedApps: { maxItems: number } };
    $defs: { PanelSelection: { maxLength: number }; Identifier: { maxLength: number }; CarriedText: { maxLength: number } };
  };
  assert.equal(schemaBounds.properties.spawnedApps.maxItems, fixture.capacities.spawnedApps);
  assert.equal(schemaBounds.$defs.PanelSelection.maxLength, fixture.capacities.identifierChars);
  assert.equal(schemaBounds.$defs.Identifier.maxLength, fixture.capacities.identifierChars);
  assert.equal(schemaBounds.$defs.CarriedText.maxLength, fixture.capacities.panelJsonChars);
  for (const state of [fixture.base, fixture.expected]) {
    assert(validate(state), JSON.stringify(validate.errors));
    assert(independentState(state));
  }
  const patched = applyPatch(structuredClone(fixture.base), fixture.selectionPatch, true, false).newDocument as HostPanelState;
  const projected = { ...structuredClone(fixture.base), activePanelTab: fixture.expected.activePanelTab };
  assert.deepEqual(patched, projected);
  assert.deepEqual(patched, fixture.expected);
  assert.deepEqual(patched.spawnedApps, fixture.base.spawnedApps);
  assert.equal(patched.activeSpawnedId, fixture.base.activeSpawnedId);

  assert(fixture.configuredPanelLeaves.valid.every(identifier));
  assert(fixture.configuredPanelLeaves.invalid.every((id) => !identifier(id)));

  const invalid: { state: unknown; schemaValid: boolean }[] = [
    { state: { ...fixture.base, programs: [] }, schemaValid: false },
    { state: { ...fixture.base, activePanelTab: "x".repeat(fixture.capacities.identifierChars + 1) }, schemaValid: false },
    { state: { ...fixture.base, spawnedApps: [...fixture.base.spawnedApps, { ...fixture.base.spawnedApps[0] }] }, schemaValid: false },
    { state: { ...fixture.base, spawnedApps: [{ ...fixture.base.spawnedApps[0] }, { ...fixture.base.spawnedApps[1], id: fixture.base.spawnedApps[0].id }] }, schemaValid: true },
    { state: { ...fixture.base, spawnedApps: Array.from({ length: fixture.capacities.spawnedApps + 1 }, (_, index) => ({ ...fixture.base.spawnedApps[0], id: `spawned-${index}` })) }, schemaValid: false },
    { state: { ...fixture.base, activeSpawnedId: "spawned-missing" }, schemaValid: true },
    { state: { ...fixture.base, spawnedApps: [{ ...fixture.base.spawnedApps[0], label: "x".repeat(fixture.capacities.panelJsonChars) }] }, schemaValid: true },
    { state: { ...fixture.base, spawnedApps: [{ ...fixture.base.spawnedApps[0], foreign: true }] }, schemaValid: false },
  ];
  for (const row of invalid) {
    assert.equal(Boolean(validate(row.state)), row.schemaValid);
    assert.equal(independentState(row.state), false);
  }
}

if (import.meta.main) {
  testHostPanelStateSchema();
  console.log("host-panel-state ajv=valid independent=valid json-patch=valid strict-capacity=valid spawned-preservation=valid");
}
