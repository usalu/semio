import { expect, test } from "bun:test";

import fixture from "../../🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json";
import { DEFAULT_WORLD3D_PRESENTATION, parseWorld3dPresentation, WORLD3D_SCENE_LANES, world3dSceneFromLanes } from "../../🟦️.ts";

test("world scene carriers and presentation match their language-neutral schema", () => {
  
  
  expect(WORLD3D_SCENE_LANES).toEqual<typeof fixture.lanes>(fixture.lanes);
  expect(parseWorld3dPresentation(undefined)).toEqual(DEFAULT_WORLD3D_PRESENTATION);
  expect(parseWorld3dPresentation(JSON.stringify(fixture.presentation.icon))).toEqual<typeof fixture.presentation.icon>(fixture.presentation.icon);
  for (const invalid of fixture.presentation.rejections) expect(parseWorld3dPresentation(JSON.stringify(invalid))).toEqual(DEFAULT_WORLD3D_PRESENTATION);
});

test("world presentation remains atomic through split and reassembly", () => {
  const spine = fixture.roundTrip.spine;
  const restored = world3dSceneFromLanes(spine, new Map(Object.entries(fixture.roundTrip.laneTexts)));
  expect({ ...restored, lanes: undefined }).toEqual({ ...fixture.roundTrip.assembled, lanes: undefined });
  expect(JSON.parse(restored.presentationJson!)).toEqual(fixture.presentation.icon);
  expect(spine).not.toHaveProperty("presentationJson");
});
