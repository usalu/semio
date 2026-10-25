/** 🔸️ The runs rope pivot actor tests of the gear depiction suite. */
import { mat2d, vec2 } from "gl-matrix";
import { expect, it, vi } from "vitest";
import { GUN_HEIGHT, GUN_MUZZLE, GUN_REACH, LADDER_CARRY, LADDER_RUNG, equip, paintLadders, paintTools, rackLadders, type GearLadder, type GearTool } from "@semio-tech/pets-react";
import { FEET, GRIP, HEIGHT, NEAR, SPECIMEN, WIDTH, apart, bearing, carried, equipped, expectLadder, ladderGeometry, meant, numbersOf, shown, through, writes } from "../🟢️harness/🟦️.tsx";

/** 🔹 Registers the runs rope pivot actor tests of the gear depiction suite. */
export function registerRunsRopePivotActorTests(): void {
  it("runs the rope from the pivot the actor hangs from to its point of the stage, taut or hanging by its slack, with the hook pointing the way it arrives", () => {
    for (const facing of [1, -1] as const) {
      for (const tilt of [0, 20 / 360, -20 / 360]) {
        for (const slack of [0, 12, -8]) {
          const bearer = bearing([{ kind: "rope", x: 246, y: 50, slack }, { kind: "hook", x: 246, y: 50 }], { facing, tilt, pivot: { x: 3, y: -GRIP } });
          const equipment = equipped(bearer);
          const text = equipment.rope.getAttribute("d")!;
          const rope = numbersOf(text);
          const near: [number, number] = [rope[0]!, rope[1]!];
          const far: [number, number] = [rope.at(-2)!, rope.at(-1)!];
          expect(apart(carried(bearer, 3, -GRIP), near)).toBeLessThan(NEAR);
          expect(near).toEqual([facing * 3, -GRIP]);
          expect(far).toEqual([46, -70]);
          expect(text).toMatch(slack === 0 ? /^M \S+ \S+ L \S+ \S+$/u : /^M \S+ \S+ Q \S+ \S+ \S+ \S+$/u);
          const control: [number, number] = slack === 0 ? near : [rope[2]!, rope[3]!];
          if (slack !== 0) {
            const halfway = vec2.lerp(vec2.create(), vec2.lerp(vec2.create(), near, control, 0.5), vec2.lerp(vec2.create(), control, far, 0.5), 0.5);
            expect(halfway[1] - (near[1] + far[1]) / 2).toBeCloseTo(slack, 2);
            expect(halfway[0]).toBeCloseTo((near[0] + far[0]) / 2, 2);
          }
          const hook = meant(equipment.hook.getAttribute("transform"));
          expect(apart(through(hook, 0, 0), far)).toBeLessThan(NEAR);
          const arriving = vec2.normalize(vec2.create(), [far[0] - control[0], far[1] - control[1]]);
          const pointing = vec2.subtract(vec2.create(), through(hook, 1, 0), through(hook, 0, 0));
          expect(apart(pointing, arriving)).toBeLessThan(1e-4);
          expect(mat2d.determinant(hook)).toBeCloseTo(1, 4);
        }
      }
    }
    const alone = equipped(bearing([{ kind: "hook", x: 230, y: 70 }]));
    expect(alone.hook.getAttribute("transform")).toBe("matrix(0 -1 1 0 30 -50)");
    expect([shown(alone.hook), shown(alone.rope)]).toEqual([true, false]);
    const reeled = equipped(bearing([{ kind: "rope", x: 200, y: 120 - GRIP, slack: 0 }, { kind: "hook", x: 200, y: 120 - GRIP }]));
    expect([reeled.rope.getAttribute("d"), reeled.hook.getAttribute("transform")]).toEqual(["M 0 -27 L 0 -27", "matrix(0 -1 1 0 0 -27)"]);
  });
  it("holds the gun at the actor's side, turned to its aim with the handle down, and lets the rope leave its muzzle", () => {
    for (const facing of [1, -1] as const) {
      for (const tilt of [0, -20 / 360]) {
        for (const aim of [0, -0.125, -0.25, 0.08, 0.5, 0.625, -0.3]) {
          const bearer = bearing([{ kind: "gun", aim }, { kind: "rope", x: 246, y: 50, slack: 0 }], { facing, tilt });
          const equipment = equipped(bearer);
          const gun = meant(equipment.gun.getAttribute("transform"));
          const hand = carried(bearer, GUN_REACH * WIDTH, -GUN_HEIGHT * HEIGHT);
          expect(apart(through(gun, 0, 0), hand)).toBeLessThan(NEAR);
          expect(Math.sign(hand[0])).toBe(facing);
          const barrel = mat2d.rotate(mat2d.create(), mat2d.fromTranslation(mat2d.create(), hand), aim * 2 * Math.PI);
          expect(apart(through(gun, GUN_MUZZLE, 0), through(barrel, GUN_MUZZLE, 0))).toBeLessThan(NEAR);
          const handle = vec2.subtract(vec2.create(), through(gun, 0, 5), through(gun, 0, 0));
          const below = vec2.subtract(vec2.create(), through(barrel, 0, Math.cos(aim * 2 * Math.PI) < 0 ? -5 : 5), through(barrel, 0, 0));
          expect(apart(handle, below)).toBeLessThan(1e-3);
          if (Math.abs(Math.cos(aim * 2 * Math.PI)) > 0.1) expect(handle[1]).toBeGreaterThan(0);
          const rope = numbersOf(equipment.rope.getAttribute("d"));
          expect(apart(through(barrel, GUN_MUZZLE, 0), [rope[0]!, rope[1]!])).toBeLessThan(NEAR);
        }
      }
    }
    expect([GUN_REACH * WIDTH, GUN_HEIGHT * HEIGHT].map((value) => Math.round(value * 100) / 100)).toEqual([16.8, 15]);
    expect(equipped(bearing([{ kind: "gun", aim: -0.125 }])).gun.getAttribute("transform")).toBe("translate(16.8 -15) rotate(-45) scale(1 1)");
    expect(equipped(bearing([{ kind: "gun", aim: 0.625 }], { facing: -1 })).gun.getAttribute("transform")).toBe("translate(-16.8 -15) rotate(225) scale(1 -1)");
  });
  it("carries a ladder at its middle on the back: two rails, and rungs perpendicular to them and evenly spaced; raised, its foot stands at the feet", () => {
    const hold = LADDER_CARRY * HEIGHT;
    for (const facing of [1, -1] as const) {
      for (const [lean, length] of [[0.25, 60], [0.22, 110], [-0.22, 110], [0.23, 37]] as const) {
        const bearer = bearing([{ kind: "ladder", lean, length }], { facing, pivot: FEET });
        const equipment = equipped(bearer);
        expect(shown(equipment.ladder.group)).toBe(true);
        const axis = through(mat2d.fromRotation(mat2d.create(), lean * 2 * Math.PI), 0, -length / 2);
        expectLadder(ladderGeometry(equipment.ladder.group), [-axis[0], -hold - axis[1]], [axis[0], -hold + axis[1]], Math.floor(length / LADDER_RUNG));
      }
    }
    for (const lean of [0.04, -0.1, 0]) {
      const geometry = ladderGeometry(equipped(bearing([{ kind: "ladder", lean, length: 110 }], { pivot: FEET })).ladder.group);
      const axis = through(mat2d.fromRotation(mat2d.create(), lean * 2 * Math.PI), 0, -1);
      const foot: [number, number] = [(hold * axis[0]) / axis[1], 0];
      expectLadder(geometry, foot, [foot[0] + 110 * axis[0], 110 * axis[1]], 10);
      const held = vec2.cross([0, 0, 0], vec2.fromValues(axis[0], axis[1]), vec2.fromValues(0 - foot[0], -hold - foot[1]));
      expect(Math.abs(held[2]!)).toBeLessThan(NEAR);
    }
    expect(shown(equipped(bearing([{ kind: "ladder", lean: 0.25, length: 0 }])).ladder.group)).toBe(false);
    expect(ladderGeometry(equipped(bearing([{ kind: "ladder", lean: 0.25, length: 8 }])).ladder.group).rungs.length).toBe(1);
    expect(equipped(bearing([{ kind: "ladder", lean: 0.25, length: 60 }], { pivot: FEET })).ladder.rails.getAttribute("d")).toBe("M 0 -5 H 60 M 0 5 H 60");
  });
  it("stands the ladders of the stage between foot and top at the size pets are drawn at, builds each once and hides the ones that left", () => {
    const rack = rackLadders(document);
    expect([rack.element.tagName, rack.element.getAttribute("class"), rack.element.getAttribute("focusable"), rack.element.getAttribute("overflow"), rack.element.childElementCount]).toEqual(["svg", "pet-ladders", "false", "visible", 0]);
    expect(writes(rack.element, () => paintLadders(rack, [], 0.8))).toEqual([]);
    expect(rack.element.getAttribute("style")).toBeNull();
    const ladders: GearLadder[] = [
      { x0: 240, y0: 300, x1: 257.5, y1: 230, rungs: 6, opacity: 1 },
      { x0: 90, y0: 300, x1: 60, y1: 180, rungs: 11, opacity: 0.5 },
    ];
    paintLadders(rack, ladders, 0.8);
    expect(rack.element.style.transform).toBe("scale(0.8)");
    expect(rack.element.childElementCount).toBe(2);
    const size = meant(rack.element.style.transform);
    ladders.forEach((ladder, index) => {
      const group = rack.element.children[index]!;
      expect([group.getAttribute("data-pet-tool"), group.getAttribute("visibility"), group.getAttribute("opacity")]).toEqual(["ladder", "visible", String(ladder.opacity)]);
      expectLadder(ladderGeometry(group), [ladder.x0, ladder.y0], [ladder.x1, ladder.y1], ladder.rungs);
      const drawn = ladderGeometry(group, size).rails[0]!;
      expect(vec2.distance(drawn[0], drawn[1])).toBeCloseTo(0.8 * Math.hypot(ladder.x1 - ladder.x0, ladder.y1 - ladder.y0), 1);
    });
    expect(writes(rack.element, () => paintLadders(rack, ladders, 0.8))).toEqual([]);
    expect(writes(rack.element, () => paintLadders(rack, [{ ...ladders[0]!, opacity: 0.7 }, ladders[1]!], 0.8))).toEqual(["attributes:g.opacity"]);
    expect(writes(rack.element, () => paintLadders(rack, [{ ...ladders[0]!, opacity: 0.7, x1: 300, y1: 270 }, ladders[1]!], 0.8))).toEqual(["attributes:g.transform", "attributes:path.d", "attributes:path.d"]);
    expect(writes(rack.element, () => paintLadders(rack, [ladders[1]!], 0.8)).sort()).toEqual(["attributes:g.opacity", "attributes:g.transform", "attributes:g.visibility", "attributes:path.d", "attributes:path.d"]);
    expect([...rack.element.children].map((group) => group.getAttribute("visibility"))).toEqual(["visible", "hidden"]);
    expect(writes(rack.element, () => paintLadders(rack, [], 0.8))).toEqual(["attributes:g.visibility"]);
    expect(writes(rack.element, () => paintLadders(rack, [], 0.8))).toEqual([]);
    expect(writes(rack.element, () => paintLadders(rack, [ladders[1]!, ladders[0]!, ladders[0]!], 1)).filter((record) => record.startsWith("childList"))).toEqual(["childList:svg.null"]);
    expect(rack.element.childElementCount).toBe(3);
    expect(rack.element.style.transform).toBe("scale(1)");
    expect(shown(rack.element.children[0]!)).toBe(true);
    paintLadders(rack, [{ ...ladders[0]!, opacity: 0 }, { ...ladders[1]!, rungs: 0 }], 1);
    expect([...rack.element.children].map(shown)).toEqual([false, false, false]);
    expect(rack.element.querySelector("[id], defs, use, style")).toBeNull();
  });
  it("writes nothing when a frame changes nothing, and only what a frame changes", () => {
    const equipment = equip(SPECIMEN, document);
    const drawing = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    drawing.append(equipment.back, equipment.front);
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    paintTools(equipment, bearing([], { tilt: 0.1, facing: -1 }));
    expect(setAttribute).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    const chute: GearTool = { kind: "chute", open: 1, sway: 0 };
    expect(writes(drawing, () => paintTools(equipment, bearing([chute])))).toEqual(["attributes:g.transform", "attributes:g.transform", "attributes:g.visibility", "attributes:g.transform", "attributes:g.transform", "attributes:path.d"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([chute])))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([chute], { x: 205.5, y: 131 })))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.02 }])))).toEqual(["attributes:g.transform"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.020001, open: 1.0004 }])))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.02, open: 0.9 }])))).toEqual(["attributes:g.transform", "attributes:path.d"]);
    const leaning = { tilt: 0.05 };
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.02, open: 0.9 }], leaning)))).toEqual(["attributes:g.transform", "attributes:g.transform"]);
    const line: GearTool[] = [{ ...chute, sway: 0.02, open: 0.9 }, { kind: "rope", x: 246, y: 50, slack: 0 }, { kind: "hook", x: 246, y: 50 }];
    expect(writes(drawing, () => paintTools(equipment, bearing(line, leaning)))).toEqual(["attributes:path.visibility", "attributes:path.d", "attributes:g.visibility", "attributes:g.transform"]);
    expect(writes(drawing, () => paintTools(equipment, bearing(line, leaning)))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing(line, { ...leaning, x: 201 })))).toEqual(["attributes:path.d", "attributes:g.transform"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([line[1]!, { kind: "rope", x: 0, y: 0, slack: 9 }, line[2]!], { ...leaning, x: 201 })))).toEqual(["attributes:g.visibility"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([], { ...leaning, x: 201 })))).toEqual(["attributes:path.visibility", "attributes:g.visibility"]);
    expect([equipment.chute, equipment.ladder.group, equipment.rope, equipment.hook, equipment.gun].map(shown)).toEqual([false, false, false, false, false]);
    expect(writes(drawing, () => paintTools(equipment, bearing([], { tilt: -0.2, x: 0 })))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ kind: "gun", aim: 0 }, { kind: "ladder", lean: 0.25, length: 60 }], leaning))).sort()).toEqual(["attributes:g.transform", "attributes:g.transform", "attributes:g.visibility", "attributes:g.visibility", "attributes:path.d", "attributes:path.d"]);
  });
}
