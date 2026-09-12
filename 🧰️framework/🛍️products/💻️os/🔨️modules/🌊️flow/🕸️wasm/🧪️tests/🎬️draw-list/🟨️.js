//! 🎬️ Conformance twin for `canvas::draw_list`: the JavaScript replayer, measured on the SAME
//! fixture and the SAME encoded draw list the Rust law pins
//! (`♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/{🔣️.json,📐️expected-draw-list.json}`).
//!
//! Two implementations of one contract, so the fixture is read from where the encoder lives rather
//! than copied here — a drift in either direction fails on one side or the other. The assertion is
//! on the recorded DRAW CALLS, not on pixels: a recording 2D context is the only oracle that can
//! say "this replay reached `roundRect`, not a flattened polyline, and at the camera's coordinates".

import { readFile } from "node:fs/promises";
import { deepStrictEqual } from "node:assert";
import { FLOW_DRAW_LIST_VERSION, FlowPresentation, renderFlowCanvas, replayFlowDrawList } from "../../📦️packages/🟨️javascript/🖥️flow-host.js";

const equal = (actual, expected, law) => {
  if (actual !== expected) throw new Error(`${law}: ${actual} !== ${expected}`);
};

const fixtureUrl = new URL("../../../../♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/🔣️.json", import.meta.url);
const expectationUrl = new URL("../../../../♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json", import.meta.url);
const fixture = JSON.parse(await readFile(fixtureUrl, "utf8"));
const drawList = JSON.parse(await readFile(expectationUrl, "utf8"));

/** 📝️ A 2D context that records every call instead of rasterizing. `setTransform` is flattened to
 * its six coefficients so a camera regression shows up as a numeric diff, not as a missing pixel. */
function recordingContext() {
  const calls = [];
  const record = (name) => (...args) => void calls.push([name, ...args]);
  return {
    calls,
    canvas: { width: 0, height: 0 },
    fillStyle: "",
    strokeStyle: "",
    lineWidth: 0,
    lineCap: "",
    lineJoin: "",
    lineDashOffset: 0,
    globalAlpha: 1,
    globalCompositeOperation: "source-over",
    setTransform: record("setTransform"),
    beginPath: record("beginPath"),
    closePath: record("closePath"),
    moveTo: record("moveTo"),
    lineTo: record("lineTo"),
    quadraticCurveTo: record("quadraticCurveTo"),
    bezierCurveTo: record("bezierCurveTo"),
    rect: record("rect"),
    roundRect: record("roundRect"),
    arc: record("arc"),
    arcTo: record("arcTo"),
    setLineDash: record("setLineDash"),
    clip: record("clip"),
    save: record("save"),
    restore: record("restore"),
    clearRect: record("clearRect"),
    fillRect: record("fillRect"),
    fill(...args) {
      calls.push(["fill", this.fillStyle, ...args]);
    },
    stroke() {
      calls.push(["stroke", this.strokeStyle, this.lineWidth, this.lineCap]);
    },
  };
}

const identity = [1, 0, 0, 1, 0, 0];

equal(drawList.version, FLOW_DRAW_LIST_VERSION, "draw-list-version");
equal(drawList.truncated, false, "draw-list-not-truncated");

//#region 🎬️ReplayShape
const context = recordingContext();
const drawn = replayFlowDrawList(context, drawList, identity);
equal(drawn, drawList.commands.filter((command) => ["f", "s", "i"].includes(command[0])).length, "replayed-every-drawing-command");

const names = context.calls.map((call) => call[0]);
equal(names.filter((name) => name === "arc").length, fixture.ports.length * 2, "ports-replay-as-arcs");
equal(names.filter((name) => name === "roundRect").length, fixture.nodes.length * 2, "nodes-replay-as-round-rects");
equal(names.filter((name) => name === "bezierCurveTo").length, fixture.wires.length, "wires-replay-as-cubic-curves");
equal(names.filter((name) => name === "lineTo").length, fixture.grid.lines.length, "grid-replays-as-line-segments");
equal(names.filter((name) => name === "clip").length, 1, "one-viewport-clip");
equal(names.filter((name) => name === "save").length, 1, "one-layer-pushed");
equal(names.filter((name) => name === "restore").length, 1, "one-layer-popped");
equal(names.at(-1), "restore", "layer-stack-balances");
equal(context.calls.some((call) => call[0] === "fillRect"), false, "no-placeholder-rectangles");
console.log("[DEBUG] Flow draw-list replay reproduced every primitive of the shared fixture as its own 2D primitive");
//#endregion 🎬️ReplayShape

//#region 🎥️ReplayCamera
// 📐️ The encoder bakes `camera_content_affine(camera, viewport)` into every command. A replay must
// apply it verbatim — the placeholder painter this replaces drew at raw world coordinates, which is
// exactly the failure this law names.
const zoom = fixture.camera.zoom;
const transforms = context.calls.filter((call) => call[0] === "setTransform").map((call) => call.slice(1));
if (transforms.length < 2) throw new Error("replay-emitted-no-transforms");
for (const [a, b, c, d] of transforms.slice(1)) {
  equal(Math.abs(a - zoom) < 1e-6 && Math.abs(d - zoom) < 1e-6 && b === 0 && c === 0, true, `camera-zoom-applied:${a},${b},${c},${d}`);
}
const firstNode = fixture.nodes[0];
const nodeCall = context.calls.find((call) => call[0] === "roundRect");
equal(Math.abs(nodeCall[3] - firstNode.width) < 1e-6, true, "node-body-keeps-its-world-width-under-the-camera-affine");
equal(nodeCall[1] !== firstNode.x, true, "node-body-is-not-drawn-at-its-raw-world-origin");

// 🔍️ A device-pixel base transform must multiply the camera, not replace it.
const retina = recordingContext();
replayFlowDrawList(retina, drawList, [2, 0, 0, 2, 0, 0]);
const retinaTransforms = retina.calls.filter((call) => call[0] === "setTransform").map((call) => call.slice(1));
for (let index = 1; index < transforms.length; index += 1) {
  deepStrictEqual(retinaTransforms[index].map((value) => Number((value / 2).toFixed(6))), transforms[index].map((value) => Number(value.toFixed(6))), `device-pixel-scale-composes:${index}`);
}
console.log("[DEBUG] Flow draw-list replay applied the camera affine and composed the device-pixel scale over it");
//#endregion 🎥️ReplayCamera

//#region 🚧️ReplayRefusals
equal(replayFlowDrawList(recordingContext(), { version: FLOW_DRAW_LIST_VERSION + 1, commands: drawList.commands }, identity), 0, "refuses-unknown-version");
equal(replayFlowDrawList(recordingContext(), undefined, identity), 0, "refuses-absent-draw-list");
equal(replayFlowDrawList(null, drawList, identity), 0, "refuses-absent-context");

const unbalanced = recordingContext();
replayFlowDrawList(unbalanced, { version: FLOW_DRAW_LIST_VERSION, commands: [["po"], ["po"]] }, identity);
equal(unbalanced.calls.length, 0, "a-pop-without-a-push-never-unwinds-the-caller-state");

const leaked = recordingContext();
replayFlowDrawList(leaked, { version: FLOW_DRAW_LIST_VERSION, commands: [["pc", 0, identity, ["r", 0, 0, 4, 4]]] }, identity);
equal(leaked.calls.filter((call) => call[0] === "restore").length, 1, "an-unclosed-layer-is-unwound-before-returning");

const arcless = recordingContext();
delete arcless.roundRect;
replayFlowDrawList(arcless, drawList, identity);
equal(arcless.calls.filter((call) => call[0] === "arcTo").length, fixture.nodes.length * 2 * 4, "round-rects-fall-back-to-exact-corner-arcs");
console.log("[DEBUG] Flow draw-list replay refused every malformed list and unwound its own layer stack");
//#endregion 🚧️ReplayRefusals

//#region 🙈️DevicelessReplay
// 🪞️ Twin of the Rust law `without_a_gpu_adapter_the_frame_replays_a_list_that_covers_the_node_layout`.
//
// A viewer with no WebGPU adapter is a real user, so a frame that arrives as `present: "2d"` must be
// replayed — non-empty, and at the camera's OWN screen coordinates, which is what puts the picture
// where the node layout is instead of somewhere off the viewport. Measured live before this row
// existed: the graph canvas carried 0 ink pixels for a whole session because the deviceless frame
// reached a canvas that could no longer give a 2D context and the refusal was swallowed.

/** 📐️ Screen-space box of every recorded primitive — its own coordinates through the transform in
 * force when it was issued. The JavaScript mirror of the Rust law's `command_screen_bounds`. */
function recordedPrimitives(calls) {
  let transform = [1, 0, 0, 1, 0, 0];
  const at = (x, y) => [transform[0] * x + transform[2] * y + transform[4], transform[1] * x + transform[3] * y + transform[5]];
  const box = (name, points) => ({ name, x0: Math.min(...points.map(([x]) => x)), y0: Math.min(...points.map(([, y]) => y)), x1: Math.max(...points.map(([x]) => x)), y1: Math.max(...points.map(([, y]) => y)) });
  const primitives = [];
  for (const [name, ...args] of calls) {
    if (name === "setTransform") transform = args.slice(0, 6);
    else if (name === "roundRect" || name === "rect") primitives.push(box(name, [at(args[0], args[1]), at(args[0] + args[2], args[1] + args[3])]));
    else if (name === "arc") primitives.push(box(name, [at(args[0] - args[2], args[1] - args[2]), at(args[0] + args[2], args[1] + args[2])]));
    else if (name === "moveTo" || name === "lineTo") primitives.push(box(name, [at(args[0], args[1])]));
    else if (name === "bezierCurveTo") primitives.push(box(name, [at(args[0], args[1]), at(args[2], args[3]), at(args[4], args[5])]));
  }
  return primitives;
}

/** 🎥️ `canvas::camera::camera_content_affine`, derived here from the fixture rather than read out of
 * the encoded list, so "the picture is at the camera's coordinates" is not asserted against itself. */
const cameraContentAffine = (camera, viewport) => [camera.zoom, 0, 0, camera.zoom, viewport.width * 0.5 - camera.x * camera.zoom, viewport.height * 0.5 - camera.y * camera.zoom];

const devicelessContext = recordingContext();
const devicelessCanvas = { width: 0, height: 0, getContext: (kind) => (kind === "2d" ? devicelessContext : null) };
const devicelessFrame = { present: "2d", width: fixture.viewport.width, height: fixture.viewport.height, dpr: 1, clear: [0, 0, 0, 0], draw: drawList };
equal(renderFlowCanvas(devicelessCanvas, devicelessFrame), FlowPresentation.replayed, "no-adapter-replays-the-list");

const painted = recordedPrimitives(devicelessContext.calls);
equal(painted.length > 0, true, "no-adapter-replay-is-not-empty");
const ink = { x0: Math.min(...painted.map((p) => p.x0)), y0: Math.min(...painted.map((p) => p.y0)), x1: Math.max(...painted.map((p) => p.x1)), y1: Math.max(...painted.map((p) => p.y1)) };
equal(ink.x1 - ink.x0 > fixture.viewport.width * 0.5 && ink.y1 - ink.y0 > fixture.viewport.height * 0.5, true, `no-adapter-replay-covers-the-viewport:${JSON.stringify(ink)}`);

const affine = cameraContentAffine(fixture.camera, fixture.viewport);
const screenPoint = (x, y) => [affine[0] * x + affine[2] * y + affine[4], affine[1] * x + affine[3] * y + affine[5]];
for (const node of fixture.nodes) {
  const [x0, y0] = screenPoint(node.x - node.width / 2, node.y - node.height / 2);
  const [x1, y1] = screenPoint(node.x + node.width / 2, node.y + node.height / 2);
  const covered = painted.some((p) => p.name === "roundRect" && Math.abs(p.x0 - x0) < 1e-3 && Math.abs(p.y0 - y0) < 1e-3 && Math.abs(p.x1 - x1) < 1e-3 && Math.abs(p.y1 - y1) < 1e-3);
  equal(covered, true, `no-adapter-replay-covers-node:${node.id}`);
}

// 🚨️ …and the two verdicts the host acts on. `gpu` never touches the element (a canvas admits one
// context kind for its whole life); `unpresentable` is the frame that wants the replay on a canvas
// which refuses a 2D context — the WebGPU-poisoned element that must be REPLACED, not retried.
let askedForAContext = false;
equal(renderFlowCanvas({ width: 0, height: 0, getContext: () => { askedForAContext = true; return null; } }, { present: "gpu", width: 10, height: 10, dpr: 1 }), FlowPresentation.gpu, "gpu-frames-report-gpu");
equal(askedForAContext, false, "gpu-frames-never-ask-a-presented-canvas-for-a-context");
equal(renderFlowCanvas({ width: 0, height: 0, getContext: () => null }, devicelessFrame), FlowPresentation.unpresentable, "a-canvas-that-refuses-2d-under-a-2d-frame-is-unpresentable");
equal(renderFlowCanvas(null, devicelessFrame), FlowPresentation.none, "no-canvas-presents-nothing");
console.log("[DEBUG] Flow deviceless frame replayed a non-empty list over every node of the shared fixture and named its own presentation verdict");
//#endregion 🙈️DevicelessReplay
