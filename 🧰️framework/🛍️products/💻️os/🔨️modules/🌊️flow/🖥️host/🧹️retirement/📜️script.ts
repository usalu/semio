/** 🧹️ Flow session byte ownership fixtures and independent JSON oracle; source validation only. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";

//#region 🔣️SessionOwnership
const fixture = await Bun.file(new URL("./🔣️.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🧬️.schema.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
const text = fixture.text.text.repeat(fixture.text.repeat); const preview = fixture.preview.text.repeat(fixture.preview.repeat);
const owners = [text, "{}", "mesh", preview, "pending", "geometry", "output", "label", preview, "label", text];
assert.equal(owners.reduce((sum, owner) => sum + Buffer.byteLength(owner), 0), fixture.expected.releasedBytes);
assert(fixture.text.reservedCapacity > Buffer.byteLength(text));
const dagText = fixture.dag.text.repeat(fixture.dag.repeat);
assert.equal(Buffer.byteLength(dagText), fixture.dag.minimumUtf8Bytes);
assert.equal(stableStringify({ label: text }), JSON.stringify({ label: text }));
assert.equal(fixture.scene.retainedVelloRects, 256);
const canvasSource = await Bun.file(new URL("../../../♾️infinite/🖼️canvas/🦀️.rs", import.meta.url)).text();
assert(canvasSource.includes("fn retire_vello_fragment"));
assert(canvasSource.includes("Self::vector_backing_bytes(&encoding.resources.glyph_runs)"));
assert(canvasSource.indexOf("slot.command_backing_bytes = command.retirement_backing_bytes()") < canvasSource.indexOf("slot.command = Some(ManuallyDrop::new(command))"));
const sceneConsumers = [
  ["iconPaintCache", await Bun.file(new URL("../../../♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs", import.meta.url)).text(), "retirement_scene: Cell<Option<infinite::canvas::OpaqueSceneRetirementToken>>"],
  ["boardWorldCache", await Bun.file(new URL("../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs", import.meta.url)).text(), "opaque_scene_retirement: Cell<Option<OpaqueSceneRetirementToken>>"],
  ["engineCanvasPacket", await Bun.file(new URL("../../../📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs", import.meta.url)).text(), "scene_retirement: Option<canvas::OpaqueSceneRetirementToken>"],
] as const;
assert.deepEqual(sceneConsumers.map(([name]) => name), fixture.scene.retainedConsumers);
for (const [, source, retainedOwner] of sceneConsumers) {
  assert(source.includes(retainedOwner));
  assert(source.includes("advance_opaque_scene_retirement"));
}
assert(sceneConsumers[1][1].includes("assert!(turns > 1_600)"));
for (const grant of fixture.grants) {
  let total = 0;
  for (const owner of owners) { let left = Buffer.byteLength(owner); while (left) { const released = Math.min(grant, left); total += released; left -= released; } }
  assert.equal(total, fixture.expected.releasedBytes);
}
for (const mutant of [{ ...fixture, extra: true }, { ...fixture, grants: [16384] }, { ...fixture, dag: { ...fixture.dag, minimumUtf8Bytes: 1600 } }, { ...fixture, scene: { retirementCapacity: 1025 } }, { ...fixture, expected: { ...fixture.expected, zeroGrant: "progress" } }]) assert(!validate(mutant));
console.log("[DEBUG] Flow session-retirement source fixtures=1 hostileRejections=5 bytes=42405 dagBytes=4800 sceneCapacity=1024 sceneCommands=128 scenePathElements=1600 sceneVelloRects=256 sceneConsumers=3 grants=1,64,4096 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🔣️SessionOwnership

//#region 🧹️BridgeSessionClose
const sessionClose = await Bun.file(new URL("../../🕸️wasm/🧪️fixtures/🧹️session-close/🔣️.json", import.meta.url)).json();
const sessionCloseSchema = await Bun.file(new URL("../../🕸️wasm/🧪️fixtures/🧹️session-close/🧬️.schema.json", import.meta.url)).json();
const validateClose = new Ajv({ strict: true, allErrors: true }).compile(sessionCloseSchema);
assert(validateClose(sessionClose), JSON.stringify(validateClose.errors));
const expected = Object.fromEntries(Object.entries(sessionCloseSchema.properties).map(([key, value]) => [key, (value as { const: unknown }).const]));
assert.equal(stableStringify(sessionClose), stableStringify(expected));
for (const mutant of [
  { ...sessionClose, extra: true },
  { ...sessionClose, browser: { ...sessionClose.browser, terminalBeforeClose: true } },
  { ...sessionClose, close: { ...sessionClose.close, retainedBeforePoll: 0 } },
  { ...sessionClose, ordering: ["session-closed", "session-released", "domain-retired"] },
]) assert(!validateClose(mutant));
console.log("[DEBUG] Flow retained-session close fixture=1 hostileRejections=4 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🧹️BridgeSessionClose

//#region 🧑‍🤝‍🧑️BrowserRuntimeLifetime
const runtimeLifetime = await Bun.file(new URL("../../🕸️wasm/🧪️fixtures/🧑‍🤝‍🧑️browser-runtime/🔣️.json", import.meta.url)).json();
const runtimeLifetimeSchema = await Bun.file(new URL("../../🕸️wasm/🧪️fixtures/🧑‍🤝‍🧑️browser-runtime/🧬️.schema.json", import.meta.url)).json();
const validateRuntime = new Ajv({ strict: true, allErrors: true }).compile(runtimeLifetimeSchema);
assert(validateRuntime(runtimeLifetime), JSON.stringify(validateRuntime.errors));
const runtimeExpected = Object.fromEntries(Object.entries(runtimeLifetimeSchema.properties).map(([key, value]) => [key, (value as { const: unknown }).const]));
assert.equal(stableStringify(runtimeLifetime), stableStringify(runtimeExpected));
for (const mutant of [{ ...runtimeLifetime, initialSessions: 1 }, { ...runtimeLifetime, extra: true }, { ...runtimeLifetime, afterCloseA: { ...runtimeLifetime.afterCloseA, globalCloseCalls: 1 } }, { ...runtimeLifetime, receipt: { ...runtimeLifetime.receipt, completion: "control-admitted" } }, { ...runtimeLifetime, openFailure: { ...runtimeLifetime.openFailure, uncertainTransport: { ...runtimeLifetime.openFailure.uncertainTransport, terminal: false } } }]) assert(!validateRuntime(mutant));
console.log("[DEBUG] Flow browser runtime lifetime fixture=1 hostileRejections=5 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🧑‍🤝‍🧑️BrowserRuntimeLifetime
