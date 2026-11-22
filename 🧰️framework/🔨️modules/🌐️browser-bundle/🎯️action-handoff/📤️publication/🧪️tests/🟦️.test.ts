import { expect, test } from "bun:test";
import Ajv from "ajv";
import { encodePackValue } from "../../../../🎁️packs/🟦️.ts";
import { decodeBrowserActorHostEffectsV1, encodeBrowserActorHostEffectV1, publishBrowserActorHostEffectsV1 } from "../🟦️.ts";
import fixture from "./🔣️.json";

const nonempty = { type: "string", minLength: 1 };
const target = new Ajv().compile({
  type: "object",
  required: ["pluginId", "appId"],
  additionalProperties: false,
  properties: {
    pluginId: nonempty,
    appId: nonempty,
    osInstanceId: nonempty,
    document: {
      type: "object",
      required: ["documentId", "schema"],
      additionalProperties: false,
      properties: { documentId: nonempty, schema: nonempty, spaceId: nonempty },
    },
  },
});

test("signed Studio and Terrain effects preserve exact first-party addresses and inputs", () => {
  for (const vector of fixture.valid) {
    const { name, wire, expected } = vector;
    const args = "args" in vector ? vector.args : undefined;
    const lowered = args === undefined ? wire : { ...wire, val: { ...wire.val, args: { tag: "some", val: encodePackValue(args) } } };
    const desired = args === undefined || !expected.replayShellCommand
      ? expected
      : { replayShellCommand: { ...expected.replayShellCommand, args } };
    const bytes = encodeBrowserActorHostEffectV1(lowered);
    expect(bytes, name).toEqual(Array.from(encodePackValue(desired)));
    const [result] = decodeBrowserActorHostEffectsV1([bytes]);
    expect(result, name).toEqual(desired);
    if (result && "openPluginInstance" in result) {
      expect(target(result.openPluginInstance), name).toBe(true);
    }
  }
  for (const wire of fixture.invalid) expect(() => encodeBrowserActorHostEffectV1(wire)).toThrow();
  const forged = { openPluginInstance: { pluginId: "gis", appId: "s.gis.gisterrain@1/*#editor", document: { documentId: "", schema: "s.gis.gisterrain" } } };
  expect(target(forged.openPluginInstance)).toBe(false);
  expect(() => decodeBrowserActorHostEffectsV1([Array.from(encodePackValue(forged))])).toThrow();
});

test("file picker effects use the original WIT request identity and enforce canonical pack size", () => {
  const vector = fixture.valid.find(({ name }) => name === "pick-ascii-elevation")!;
  const wire = { ...vector.wire, val: { ...vector.wire.val, req: 1163085650n } };
  expect(decodeBrowserActorHostEffectsV1([encodeBrowserActorHostEffectV1(wire)])).toEqual([vector.expected]);
  expect(() => encodeBrowserActorHostEffectV1({ ...wire, val: { ...wire.val, req: 9007199254740992n } })).toThrow();
  expect(() => encodeBrowserActorHostEffectV1({ tag: "download-media-export", val: { filename: "terrain.asc", mimeType: "text/plain", data: " ".repeat(262144) } })).toThrow();
});

test("retired Studio owners cannot open a Terrain document", async () => {
  const vector = fixture.valid.find(({ name }) => name === "open-persisted-terrain")!;
  const bytes = encodeBrowserActorHostEffectV1(vector.wire);
  const received: unknown[] = [];
  const publish = (effect: unknown) => { received.push(effect); };
  await expect(publishBrowserActorHostEffectsV1([bytes], () => false, publish)).rejects.toThrow("retired");
  await expect(publishBrowserActorHostEffectsV1([[...bytes, 0]], () => true, publish)).rejects.toThrow();
  expect(received).toEqual([]);
  await publishBrowserActorHostEffectsV1([bytes], () => true, publish);
  expect(received).toEqual([vector.expected]);
});
