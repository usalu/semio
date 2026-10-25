/** 🧰️ The gear draws what an actor gets around with — a parachute, a rope with its hook, a grappling gun, a carried
 * ladder, the ladders that stand on the stage — and tilts a whole drawing about its pivot. Every transform it writes is
 * held to gl-matrix's `mat2d`, which composes the same placement the way the stage means it (feet, a turn about the
 * pivot on screen, the mirror, the size) and carries the same points; the geometry is held to what the tools are: cords
 * that end on the rim of the canopy, a rope whose middle hangs by its slack, rungs perpendicular to their rails and
 * evenly spaced. A frame that changes nothing writes nothing, and nothing is ever written in a way a
 * Content-Security-Policy without inline styles would block.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🧰️gear/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://glmatrix.net/docs/module-mat2d.html — the third-party oracle
 */
import { glMatrix, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { Species } from "@semio-tech/pets";
import { NEAR, bearing, meant, numbersOf, through, apart, equipped } from "./🟢️harness/🟦️.tsx";
import { registerBuildsToolOnceHiddenTests } from "./🔶️builds-tool-once-hidden/🟦️.tsx";
import { registerRunsRopePivotActorTests } from "./🔸️runs-rope-pivot-actor/🟦️.tsx";
import { registerNeverWritesWayPolicyTests } from "./🟠️never-writes-way-policy/🟦️.tsx";

glMatrix.setMatrixArrayType(Array);

describe("gear depiction", () => {
  registerBuildsToolOnceHiddenTests();
  registerRunsRopePivotActorTests();
  registerNeverWritesWayPolicyTests();
});

/** 🎨️ Real architecture canopy outlines and their four cords agree with the fixture and gl-matrix. */
describe("architecture custom parachutes", () => {
  const architecture = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../🎓️teaching/🏛️architecture");
  const fixture = JSON.parse(readFileSync(resolve(architecture, "📚️course/🧫️fixtures/🪂️canopies/🔣️.json"), "utf8")) as { species: { id: string; shape: NonNullable<Species["canopy"]>; rim: number[] }[] };
  const ensemble = JSON.parse(readFileSync(resolve(architecture, "🐾️pets/🔣️.json"), "utf8")) as { species: string[] };
  const species = ensemble.species.map((path) => JSON.parse(readFileSync(resolve(architecture, "🐾️pets", path), "utf8")) as Species);

  for (const expected of fixture.species) it("renders " + expected.id + " with its custom outline and attached cords", () => {
    const kind = species.find(({ id }) => id === expected.id)!;
    const equipment = equipped(bearing([{ kind: "chute", open: 1, sway: 0.03 }], { pivot: { x: 0, y: -kind.grip }, tilt: 0.02, facing: -1 }), kind);
    expect(equipment.canopy.children).toHaveLength(1);
    expect(equipment.canopy.children[0]!.getAttribute("d")).toBe(expected.shape.kind === "path" ? expected.shape.d : undefined);
    expect(equipment.rim.flatMap(({ x, y }) => [x, y])).toEqual(expected.rim);
    expect(equipment.chute.getAttribute("visibility")).toBe("visible");
    const [left, right] = equipment.rim;
    const flown = meant(equipment.canopy.getAttribute("transform"));
    const cords = numbersOf(equipment.cords.getAttribute("d"));
    for (let cord = 0; cord < 4; cord++) {
      const end = vec2.lerp(vec2.create(), [left.x, left.y], [right.x, right.y], cord / 3);
      expect(apart(through(flown, end[0], end[1]), [cords[cord * 4 + 2]!, cords[cord * 4 + 3]!])).toBeLessThan(NEAR);
    }
  });
});
