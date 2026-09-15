/** 🔁️ Which channel a guest's `dispatchAction` host effect re-enters, over
 * `🧫️fixtures/🔁️host-effect-invocation/🔣️.json`. Both renderer targets answer through
 * {@link hostEffectInvocationV1}: React's `encodeEffect*Invocation` and the wgpu frame worker's
 * contributions re-arm. */
import { describe, expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🔁️host-effect-invocation/🔣️.json";
import { appCommandIdsV1, hostEffectInvocationV1, type HostEffectDispatchScope, type PluginManifest } from "../../🟦️.ts";

const scope = fixture.scope as HostEffectDispatchScope;
const commandIds = fixture.appCommandIds as readonly string[];

describe("🔁️ host effect invocation", () => {
  for (const row of fixture.cases) {
    test(`${row.action} re-enters the ${row.expect.kind} channel`, () => {
      const answered = hostEffectInvocationV1(scope, commandIds, row.action, row.args as Record<string, unknown> | undefined);
      expect(answered.kind).toBe(row.expect.kind);
      expect(JSON.parse(JSON.stringify(answered.invocation))).toEqual(row.expect.invocation);
    });
  }

  test("a reserved verb addressed as an app command is exactly the defect this rule removes", () => {
    const reserved = fixture.cases.find((row) => row.action === "toolRunStart")!;
    expect(commandIds).not.toContain(reserved.action);
    expect(hostEffectInvocationV1(scope, [...commandIds, reserved.action], reserved.action, reserved.args as Record<string, unknown>).kind).toBe("command");
  });

  test("the app's declared command ids come off the manifest the host already holds", () => {
    const manifest = { pluginId: scope.pluginId, label: "", version: "1", apps: [{ id: scope.appId, commands: commandIds.map((id) => ({ id })) }, { id: "other", commands: [{ id: "elsewhere" }] }], workflows: [], examples: [] } as unknown as PluginManifest;
    expect(appCommandIdsV1(manifest, scope.appId)).toEqual(commandIds);
    expect(appCommandIdsV1(manifest, "missing")).toEqual([]);
  });
});
