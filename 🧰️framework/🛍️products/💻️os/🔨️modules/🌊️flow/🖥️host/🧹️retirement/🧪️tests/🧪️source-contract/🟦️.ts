/** 🧹️ Flow session byte ownership fixtures and independent JSON oracle; source validation only. */
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

//#region 🔣️SessionOwnership
const fixture = await Bun.file(new URL("../🧫️fixtures/🧹️session-retirement/🔣️.json", import.meta.url)).json();


const text = fixture.text.text.repeat(fixture.text.repeat); const preview = fixture.preview.text.repeat(fixture.preview.repeat);
const owners = [text, "{}", "mesh", preview, "pending", "geometry", "output", "label", preview, "label", text];
assert.equal(owners.reduce((sum, owner) => sum + Buffer.byteLength(owner), 0), fixture.expected.releasedBytes);
assert(fixture.text.reservedCapacity > Buffer.byteLength(text));
const dagText = fixture.dag.text.repeat(fixture.dag.repeat);
assert.equal(Buffer.byteLength(dagText), fixture.dag.minimumUtf8Bytes);
assert.equal(stableStringify({ label: text }), JSON.stringify({ label: text }));
assert.equal(fixture.scene.retainedVelloRects, 256);
const canvasSource = readModuleTreeSource(new URL("../../../../../../../../🔨️modules/🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs", import.meta.url));
assert(canvasSource.includes("fn retire_vello_fragment"));
assert(canvasSource.includes("Self::vector_backing_bytes(&encoding.resources.glyph_runs)"));
assert(canvasSource.indexOf("slot.command_backing_bytes = command.retirement_backing_bytes()") < canvasSource.indexOf("slot.command = Some(ManuallyDrop::new(command))"));
const sceneConsumers = [
  ["iconPaintCache", readModuleTreeSource(new URL("../../../../../♾️infinite/🎲️board/⚙️engine/🦀️.rs", import.meta.url)), "retirement_scene: Cell<Option<semio_framework_canvas::OpaqueSceneRetirementToken>>", "advance_opaque_scene_retirement"],
  ["boardWorldCache", readModuleTreeSource(new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🕹️gestures/🦀️.rs", import.meta.url)), "opaque_scene_retirement: Cell<Option<OpaqueSceneRetirementToken>>", "advance_opaque_scene_retirement"],
  ["engineCanvasPacket", await Bun.file(new URL("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs", import.meta.url)).text(), "scene_retirement: ManuallyDrop<canvas::CanvasSceneRetirement>", "self.scene_retirement.close_step(&mut self.scene,grant)"],
] as const;
assert.deepEqual(sceneConsumers.map(([name]) => name), fixture.scene.retainedConsumers);
for (const [, source, retainedOwner, advance] of sceneConsumers) {
  assert(source.includes(retainedOwner));
  assert(source.includes(advance));
}
const boardWorldLaws = readModuleTreeSource(new URL("../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🧪️tests/🔬️board-host/🦀️.rs", import.meta.url));
const boardWorldRetirementLaw = boardWorldLaws.slice(boardWorldLaws.indexOf("fn board_world_scene_retirement_retains_exact_token_until_backing_is_released"));
assert(boardWorldRetirementLaw.slice(0, boardWorldRetirementLaw.indexOf("#[test]")).includes("assert!(turns > 1_600)"));
for (const grant of fixture.grants) {
  let total = 0;
  for (const owner of owners) { let left = Buffer.byteLength(owner); while (left) { const released = Math.min(grant, left); total += released; left -= released; } }
  assert.equal(total, fixture.expected.releasedBytes);
}

//#endregion 🔣️SessionOwnership

//#region 🧹️BridgeSessionClose
const sessionClose = await Bun.file(new URL("../../../../🕸️wasm/🧪️tests/🧫️fixtures/🧹️session-close/🔣️.json", import.meta.url)).json();


assert.deepEqual(JSON.parse(stableStringify(sessionClose)), sessionClose);

//#endregion 🧹️BridgeSessionClose

//#region 🧑‍🤝‍🧑️BrowserRuntimeLifetime
const runtimeLifetime = await Bun.file(new URL("../../../../🕸️wasm/🧪️tests/🧫️fixtures/🧑‍🤝‍🧑browser-runtime/🔣️.json", import.meta.url)).json();


assert.deepEqual(JSON.parse(stableStringify(runtimeLifetime)), runtimeLifetime);

//#endregion 🧑‍🤝‍🧑️BrowserRuntimeLifetime
