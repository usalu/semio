import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch, compare } from "fast-json-patch";
import type { NoteCompositeWindowConfig, NoteCompositeWindowTransient } from "../../🧬️schema/🟦️.ts";

type WindowCase = { id: string; config: NoteCompositeWindowConfig; transient: NoteCompositeWindowTransient };
type Fixture = {
  appConfig: Record<string, never>;
  foreignAppConfig: Record<string, unknown>;
  document: { bytes: string; gridVisible: boolean; gridSpacing: number };
  windows: [WindowCase, WindowCase];
};

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const windowSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const noConfigSchema = { type: "object", additionalProperties: false, maxProperties: 0 } as const;

export function testNoteEmptyConfigOwnership(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(windowSchema);
  const validateConfig = ajv.getSchema(`${windowSchema.$id}#/definitions/NoteCompositeWindowConfig`);
  const validateTransient = ajv.getSchema(`${windowSchema.$id}#/definitions/NoteCompositeWindowTransient`);
  const validateNoConfig = ajv.compile(noConfigSchema);
  assert(validateConfig && validateTransient);
  assert(validateNoConfig(fixture.appConfig), JSON.stringify(validateNoConfig.errors));
  assert(!validateNoConfig(fixture.foreignAppConfig));
  for (const window of fixture.windows) {
    assert(validateConfig(window.config), JSON.stringify(validateConfig.errors));
    assert(validateTransient(window.transient), JSON.stringify(validateTransient.errors));
  }

  const documentBefore = structuredClone(fixture.document);
  const appConfigBefore = structuredClone(fixture.appConfig);
  const rightBefore = structuredClone(fixture.windows[1]);
  const left = structuredClone(fixture.windows[0]);
  const changedConfig: NoteCompositeWindowConfig = { camera: { x: 12.5, y: -6.5, zoom: 3.5 } };
  const stagedTransient: NoteCompositeWindowTransient = { engagementInput: "Alpha" };
  left.config = applyPatch(left.config, compare(left.config, changedConfig), true, false).newDocument;
  left.transient = applyPatch(left.transient, compare(left.transient, stagedTransient), true, false).newDocument;
  assert.deepEqual(left.config, changedConfig);
  assert.deepEqual(left.transient, stagedTransient);
  assert.deepEqual(fixture.windows[1], rightBefore);
  assert.deepEqual(fixture.document, documentBefore);
  assert.deepEqual(fixture.appConfig, appConfigBefore);

  left.transient = applyPatch(left.transient, compare(left.transient, { engagementInput: "" }), true, false).newDocument;
  assert.deepEqual(left.transient, { engagementInput: "" });
  assert.deepEqual(fixture.windows[1], rightBefore);
  assert.deepEqual(fixture.document, documentBefore);
  assert.deepEqual(fixture.appConfig, appConfigBefore);
}

if (import.meta.main) {
  testNoteEmptyConfigOwnership();
  console.log("note-empty-config ajv=valid foreign=reject json-patch=valid window-isolation=valid document=unchanged");
}
