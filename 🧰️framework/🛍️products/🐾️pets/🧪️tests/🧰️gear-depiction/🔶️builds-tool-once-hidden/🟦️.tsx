/** 🔶️ The builds tool once hidden tests of the gear depiction suite. */
import { mat2d, vec2 } from "gl-matrix";
import { expect, it } from "vitest";
import type { ActorFrame, Species } from "@semio-tech/pets";
import { CHUTE_DOME, CHUTE_RISE, CHUTE_SPAN, canopyRim, depict, equip, paint, tiltDegrees, tiltedPlacement } from "@semio-tech/pets-react";
import { FEET, GRIP, HEIGHT, NEAR, SCRUFF, SPECIMEN, WIDTH, apart, bearing, carried, css, equipped, meant, numbersOf, placed, shown, through } from "../🟢️harness/🟦️.tsx";

/** 🔷 Registers the builds tool once hidden tests of the gear depiction suite. */
export function registerBuildsToolOnceHiddenTests(): void {
  it("builds every tool once and hidden: the parachute and the carried ladder for the back, rope, hook and gun for the front", () => {
    const equipment = equip(SPECIMEN, document);
    const tags = (element: Element): string[] => [...element.children].map((child) => `${child.tagName}${child.getAttribute("data-pet-tool") === null ? "" : `:${child.getAttribute("data-pet-tool")}`}`);
    expect([equipment.back.tagName, equipment.back.getAttribute("class"), equipment.front.getAttribute("class")]).toEqual(["g", "pet-gear", "pet-gear"]);
    expect(equipment.back.namespaceURI).toBe("http://www.w3.org/2000/svg");
    expect(tags(equipment.back)).toEqual(["g:chute", "g:ladder"]);
    expect(tags(equipment.front)).toEqual(["path:rope", "g:hook", "g:gun"]);
    expect(tags(equipment.chute)).toEqual(["path", "g"]);
    expect(tags(equipment.canopy)).toEqual(["path", "path"]);
    expect(tags(equipment.ladder.group)).toEqual(["path", "path"]);
    expect(tags(equipment.hook)).toEqual(["line", "path"]);
    expect(tags(equipment.gun)).toEqual(["line", "rect", "rect"]);
    const tools = [...equipment.back.querySelectorAll("[data-pet-tool]"), ...equipment.front.querySelectorAll("[data-pet-tool]")];
    expect(tools.map((tool) => tool.getAttribute("visibility"))).toEqual(["hidden", "hidden", "hidden", "hidden", "hidden"]);
    expect([equipment.back.hasAttribute("transform"), equipment.front.hasAttribute("transform")]).toEqual([false, false]);
    for (const root of [equipment.back, equipment.front]) {
      expect(root.querySelector("[id], [style], defs, use, symbol, clipPath, marker, style, script, title, desc, a, text, [tabindex]")).toBeNull();
      for (const shape of root.querySelectorAll("path, line, rect, ellipse")) {
        const classes = (shape.getAttribute("class") ?? "").split(" ");
        expect(classes.length).toBe(2);
        for (const name of classes) expect(css).toContain(`.${name} {`);
      }
    }
    expect([equipment.cords.getAttribute("class"), equipment.rope.getAttribute("class")]).toEqual(["pet-fill-none pet-stroke-ink", "pet-fill-none pet-stroke-ink"]);
    expect([...equipment.canopy.children].map((piece) => piece.getAttribute("class"))).toEqual(["pet-fill-accent pet-stroke-ink", "pet-fill-paper pet-stroke-ink"]);
    expect(equipment.gun.children[1]!.getAttribute("class")).toBe("pet-fill-accent pet-stroke-ink");
  });
  it("tilts the whole drawing about its pivot as gl-matrix composes it, the same way on screen whichever way the actor faces", () => {
    const points: [number, number][] = [[0, 0], [12, -30], [-9, -4], [0, -GRIP]];
    for (const facing of [1, -1] as const) {
      for (const tilt of [20 / 360, -20 / 360, 0.3, -0.731]) {
        for (const pivot of [SCRUFF, { x: 4.25, y: -20.5 }, FEET]) {
          for (const size of [1, 0.8]) {
            const bearer = bearing([], { x: 123.25, y: 77.5, facing, tilt, pivot });
            const written = meant(tiltedPlacement(bearer.x * size, bearer.y * size, facing * size, size, tilt, pivot));
            const stage = placed(bearer, size);
            for (const [x, y] of points) expect(apart(through(written, x, y), through(stage, x, y))).toBeLessThan(NEAR);
            const still = placed({ ...bearer, tilt: 0 }, size);
            expect(apart(through(written, pivot.x, pivot.y), through(still, pivot.x, pivot.y))).toBeLessThan(NEAR);
          }
        }
      }
      const hung = meant(tiltedPlacement(50, 90, facing, 1, 0.05, SCRUFF));
      expect(through(hung, 0, 0)[0]).toBeLessThan(50 - 5);
      expect(through(hung, 0, 0)[1]).toBeLessThan(90);
    }
    expect([tiltDegrees(1, 0.05), tiltDegrees(-1, 0.05), tiltDegrees(-0.8, -0.25), tiltDegrees(1, 0)]).toEqual([18, -18, 90, 0]);
    expect(tiltedPlacement(12, 300, -1.5, 1.5, 0.05, { x: 2, y: -27 })).toBe("translate(12px, 300px) scale(-1.5, 1.5) translate(2px, -27px) rotate(-18deg) translate(-2px, 27px)");
  });
  it("places an actor that is not tilted exactly as the depiction places it", () => {
    const frame: ActorFrame = { species: "specimen", x: 120.504, y: 80.25, facing: -1, activity: "idle", opacity: 1, bones: [1, 0, 0, 1, 0, 0], eyes: [], footing: "perch", state: "resting", mood: "content", intensity: 0, spirits: 0, tilt: 0, pivot: SCRUFF, tools: [], body: { x: 100, y: 30, width: 40, height: 50 } };
    const depiction = depict(SPECIMEN, document);
    paint(depiction, frame, 1.5);
    expect(tiltedPlacement(120.5, 80.25, -1.5, 1.5, 0, SCRUFF)).toBe(depiction.element.style.transform);
    expect(tiltedPlacement(120.5, 80.25, -1.5, 1.5, 0.004 / 360, SCRUFF)).toBe("translate(120.5px, 80.25px) scale(-1.5, 1.5)");
    expect(tiltedPlacement(120.5, 80.25, -1.5, 1.5, 1, SCRUFF)).not.toBe(depiction.element.style.transform);
  });
  it("lays the tools out in the axes of the stage around the feet: its groups undo the mirror and the tilt of the drawing", () => {
    for (const facing of [1, -1] as const) {
      for (const tilt of [0, 20 / 360, -0.2, 0.31]) {
        for (const pivot of [SCRUFF, { x: 4.25, y: -20.5 }]) {
          const bearer = bearing([{ kind: "gun", aim: 0 }], { facing, tilt, pivot });
          const equipment = equipped(bearer);
          expect(equipment.front.getAttribute("transform")).toBe(equipment.back.getAttribute("transform"));
          for (const size of [1, 0.8]) {
            const drawing = meant(tiltedPlacement(bearer.x * size, bearer.y * size, facing * size, size, tilt, pivot));
            const inside = mat2d.multiply(mat2d.create(), drawing, meant(equipment.back.getAttribute("transform")));
            const stage = mat2d.fromValues(size, 0, 0, size, bearer.x * size, bearer.y * size);
            for (const [x, y] of [[0, 0], [46, -70], [-30, 12]] as const) expect(apart(through(inside, x, y), through(stage, x, y))).toBeLessThan(NEAR);
          }
        }
      }
    }
    expect(equipped(bearing([{ kind: "gun", aim: 0 }])).back.getAttribute("transform")).toBe("matrix(1 0 0 1 0 0)");
    expect(equipped(bearing([{ kind: "gun", aim: 0 }], { facing: -1 })).back.getAttribute("transform")).toBe("matrix(-1 0 0 1 0 0)");
  });
  it("hangs the parachute from the grip, its cords ending on the rim of the canopy", () => {
    const span = CHUTE_SPAN * WIDTH;
    const rim: [number, number][] = [[-span, 0], [-span / 3, 0], [span / 3, 0], [span, 0]];
    for (const open of [0.15, 0.4, 0.7, 1, 1.25]) {
      for (const [facing, tilt, sway] of [[1, 0, 0], [1, -15 / 360, 0.04], [-1, 15 / 360, -0.04]] as const) {
        const bearer = bearing([{ kind: "chute", open, sway }], { facing, tilt });
        const equipment = equipped(bearer);
        expect(shown(equipment.chute)).toBe(true);
        const hung = meant(equipment.chute.getAttribute("transform"));
        expect(apart(through(hung, 0, 0), carried(bearer, 0, -GRIP))).toBeLessThan(NEAR);
        const lean = mat2d.rotate(mat2d.create(), mat2d.fromTranslation(mat2d.create(), carried(bearer, 0, -GRIP)), sway * 2 * Math.PI);
        expect(apart(through(hung, 0, -20), through(lean, 0, -20))).toBeLessThan(NEAR);
        const canopy = meant(equipment.canopy.getAttribute("transform"));
        const cords = numbersOf(equipment.cords.getAttribute("d"));
        expect(cords.length).toBe(16);
        rim.forEach(([x, y], cord) => {
          expect(cords.slice(cord * 4, cord * 4 + 2)).toEqual([0, 0]);
          expect(apart(through(canopy, x, y), [cords[cord * 4 + 2]!, cords[cord * 4 + 3]!])).toBeLessThan(NEAR);
        });
      }
    }
    const dome = numbersOf(equip(SPECIMEN, document).canopy.children[0]!.getAttribute("d"));
    expect([dome.slice(0, 2), dome.slice(6, 8), dome.slice(10, 12), dome.slice(14, 16), dome.slice(18, 20)]).toEqual([[-span, 0], [span, 0], [span / 3, 0], [-span / 3, 0], [-span, 0]]);
    expect(Math.min(...dome.filter((_, index) => index % 2 === 1))).toBeCloseTo((-CHUTE_DOME * HEIGHT * 4) / 3, 2);
    const gore = numbersOf(equip(SPECIMEN, document).canopy.children[1]!.getAttribute("d"));
    expect([gore.slice(0, 2), gore.slice(4, 6), gore.slice(8, 10)]).toEqual([[0, -CHUTE_DOME * HEIGHT], [span / 3, 0], [-span / 3, 0]]);
  });
  it("opens the canopy from a narrow streamer close to the pet to its full span, squashed as it overshoots, and hides a packed one", () => {
    const canopy = (open: number): string | null => equipped(bearing([{ kind: "chute", open, sway: 0 }])).canopy.getAttribute("transform");
    expect(CHUTE_RISE * HEIGHT).toBeCloseTo(21, 9);
    expect(canopy(1)).toBe("translate(0 -21) scale(1 1)");
    expect(canopy(1.25)).toBe("translate(0 -21) scale(0.938 1.25)");
    expect(canopy(0.5)).toBe("translate(0 -14.7) scale(0.65 0.5)");
    expect(canopy(0.15)).toBe("translate(0 -10.29) scale(0.405 0.15)");
    expect(equipped(bearing([{ kind: "chute", open: 1, sway: 0.04 }])).chute.getAttribute("transform")).toBe("translate(0 -27) rotate(14.4)");
    for (const open of [0, 0.0004, -1]) expect(shown(equipped(bearing([{ kind: "chute", open, sway: 0 }])).chute)).toBe(false);
    const own = equipped(bearing([{ kind: "chute", open: 1, sway: 0 }]), { size: SPECIMEN.size, grip: GRIP, canopy: { kind: "ellipse", cx: 0, cy: -8, rx: 30, ry: 8 } });
    expect(own.canopy.children.length).toBe(1);
    const flown = own.canopy.children[0]!;
    expect([flown.tagName, flown.getAttribute("class"), flown.getAttribute("stroke-width"), flown.getAttribute("rx"), flown.getAttribute("cy")]).toEqual(["ellipse", "pet-fill-accent pet-stroke-ink", "2", "30", "-8"]);
    expect(numbersOf(own.cords.getAttribute("d")).slice(2, 4)).toEqual([-30, -29]);
  });
  it("runs the cords of a canopy of its own to its rim: the ends of what the shape spans, wherever it was drawn", () => {
    const rim = (canopy: Species["canopy"]): number[] => canopyRim({ size: SPECIMEN.size, grip: GRIP, canopy }).flatMap((end) => [end.x, end.y]);
    expect(rim(undefined)).toEqual([-CHUTE_SPAN * WIDTH, 0, CHUTE_SPAN * WIDTH, 0]);
    expect(rim({ kind: "path", d: "M -16 0 Q 0 -18 16 0 Z" })).toEqual([-16, 0, 16, 0]);
    expect(rim({ kind: "path", d: "M-16,0Q0-18,16,0Z" })).toEqual([-16, 0, 16, 0]);
    expect(rim({ kind: "path", d: "m -12 2 q 12 -20 24 0 z" })).toEqual([-12, 2, 12, 2]);
    expect(rim({ kind: "path", d: "M -10 0 V -8 H 10 v 8 Z" })).toEqual([-10, 0, 10, 0]);
    expect(rim({ kind: "path", d: "m -12 0 l 12 -10 l 12 10 z l -4 3" })).toEqual([-16, 3, 12, 0]);
    expect(rim({ kind: "path", d: "M 0 -20 C 30 -20 22 -4 18 0 L 6 -3 l -12 0 L -18 0 C -22 -4 -30 -20 0 -20 Z" })).toEqual([-18, 0, 18, 0]);
    expect(rim({ kind: "path", d: "M 14 1 A 14 9 0 0 0 -14 1.5 Z" })).toEqual([-14, 1.5, 14, 1]);
    expect(rim({ kind: "ellipse", cx: 2, cy: -8, rx: 30, ry: 8 })).toEqual([-28, -8, 32, -8]);
    expect(rim({ kind: "rect", x: -14, y: -10, width: 28, height: 10, radius: 3 })).toEqual([-14, 0, 14, 0]);
    expect(rim({ kind: "line", x1: 9, y1: -2, x2: -9, y2: -4 })).toEqual([-9, -4, 9, -2]);
    for (const d of ["", "nonsense", "M 5 5", "M 5 5 L 5 -9", "M 1e999 0 L 3 0", "L"]) expect(rim({ kind: "path", d })).toEqual([-30, 0, 30, 0]);
    for (const canopy of [{ kind: "path", d: "M -16 0 Q 0 -18 16 0 Z" }, { kind: "line", x1: 9, y1: -2, x2: -9, y2: -4 }, { kind: "ellipse", cx: 2, cy: -8, rx: 30, ry: 8 }] as const) {
      for (const open of [0.5, 1, 1.2]) {
        const species = { size: SPECIMEN.size, grip: GRIP, canopy };
        const equipment = equipped(bearing([{ kind: "chute", open, sway: 0.03 }], { tilt: 0.02, facing: -1 }), species);
        const [left, right] = canopyRim(species);
        expect(equipment.rim).toEqual([left, right]);
        const flown = meant(equipment.canopy.getAttribute("transform"));
        const cords = numbersOf(equipment.cords.getAttribute("d"));
        for (let cord = 0; cord < 4; cord++) {
          const end = vec2.lerp(vec2.create(), [left.x, left.y], [right.x, right.y], cord / 3);
          expect(apart(through(flown, end[0], end[1]), [cords[cord * 4 + 2]!, cords[cord * 4 + 3]!])).toBeLessThan(NEAR);
        }
      }
    }
  });
}
