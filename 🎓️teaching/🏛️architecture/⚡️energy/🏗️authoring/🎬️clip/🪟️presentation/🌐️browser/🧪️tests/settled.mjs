/** 🪟 Verify clip readiness against portable compositor and geometry cases. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {presentationBordersSettled, presentationWindowSettled, requestCameraRemeasure} from "../🟦️settled.ts";

const cases = JSON.parse(await readFile(new URL("./settled.cases.json", import.meta.url), "utf8"));
globalThis.document = {createElement: () => ({style: {clipPath: ""}})};
const expected = "polygon(0px 0px, 100px 0px, 100px 100px)";
for (const sample of cases) {
  const border = {paths: sample.paths, hasAttribute: name => name === "data-pending" && sample.pending};
  const content = {style: {clipPath: sample.clip}};
  const stack = {querySelector: selector => selector === "[data-window-silhouette-border]" ? border : content};
  assert.equal(presentationWindowSettled(stack, expected), sample.ready, sample.id);
  console.log("PASS " + sample.id);
}
assert.equal(presentationWindowSettled({querySelector: () => null}, expected), false, "missing border");
for (const sample of JSON.parse(await readFile(new URL("./settled-roots.cases.json", import.meta.url), "utf8"))) {
  const document = {querySelectorAll: () => sample.pending.map(pending => ({hasAttribute: () => pending}))};
  assert.equal(presentationBordersSettled(document), sample.ready, sample.id);
  console.log("PASS " + sample.id);
}

for (const sample of JSON.parse(await readFile(new URL("./camera-remeasure.cases.json", import.meta.url), "utf8"))) {
  const nodes = sample.initial.map(value => ({value, getAttribute() { return this.value; }, setAttribute(_name, next) { this.value = next; }}));
  requestCameraRemeasure({querySelectorAll: () => nodes});
  assert.deepEqual(nodes.map(node => node.value), sample.expected, sample.id);
  console.log("PASS " + sample.id);
}

await import("./paused-transitions.mjs");
await import("./paint-barrier.mjs");
await import("./transition-policy.mjs");
