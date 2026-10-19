import { expect, test } from "vitest";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🕹️interaction.json";
import { buildFem3dSceneNode, fem3dStructuralInstances } from "../../🎞️scenes/🟦️.ts";
import { fem3dStoryStateFor, reduceFem3dStoryAction } from "../../🔁️story-reducers/🟦️.ts";

test("story interaction echoes native entity targets through hover and mixed selection", () => {
  let state = fem3dStoryStateFor("demo", "de-DE");
  const instances = fem3dStructuralInstances(state.snapshot);
  expect(instances.find((entry) => entry.label === "n00_g")).toMatchObject({ id: "scope:1:4:node5:n00_g", interactionGranularityId: "node" });
  expect(instances.find((entry) => entry.label === "e1")).toMatchObject({ id: "scope:1:7:element2:e1", interactionGranularityId: "element" });
  for (const event of fixture.events) {
    const next = reduceFem3dStoryAction(state, event.action, event.args);
    const independent = applyPatch(structuredClone(state), [
      { op: "replace", path: "/selection", value: { ...next.selection, ids: event.selection } },
      { op: "replace", path: "/hover", value: { ...next.hover, ids: event.hover } },
    ], true).newDocument;
    expect(next).toEqual(independent);
    const scene = buildFem3dSceneNode(instances, next.camera, "model", "fem3d-play", next.selection, next.hover);
    expect(scene.world3d.domainId).toBe(fixture.domain);
    expect(scene.world3d.domainGranularityId).toBe("node");
    expect(JSON.parse(scene.world3d.selectionJson)).toMatchObject({ ids: event.selection, hoveredId: event.hover[0] ?? null });
    state = next;
  }
});

test("story interaction clears document targets when replacing the example", () => {
  const initial = fem3dStoryStateFor("demo", "en-US");
  const hovered = reduceFem3dStoryAction(initial, fixture.events[0]!.action, fixture.events[0]!.args);
  const selected = reduceFem3dStoryAction(hovered, fixture.events[1]!.action, fixture.events[1]!.args);
  const cleared = reduceFem3dStoryAction(selected, "setActiveExample", { exampleId: "cleared" });
  expect(cleared.selection.ids).toEqual([]);
  expect(cleared.hover.ids).toEqual([]);
  expect(cleared.camera).toBe(selected.camera);
});
