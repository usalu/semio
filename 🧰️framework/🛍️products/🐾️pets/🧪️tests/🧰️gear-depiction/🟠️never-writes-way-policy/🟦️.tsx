/** 🟠️ The never writes way policy tests of the gear depiction suite. */
import { expect, it, vi } from "vitest";
import { equip, paintLadders, paintTools, rackLadders, type GearTool } from "@semio-tech/pets-react";
import { SPECIMEN, bearing, css, writes } from "../🟢️shared/🟦️.tsx";

/** 🟡 Registers the never writes way policy tests of the gear depiction suite. */
export function registerNeverWritesWayPolicyTests(): void {
  it("never writes in a way a policy without inline styles blocks, and never builds or removes an element for a frame", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const innerHTML = vi.spyOn(Element.prototype, "innerHTML", "set");
    const outerHTML = vi.spyOn(Element.prototype, "outerHTML", "set");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const equipment = equip(SPECIMEN, document);
    const drawing = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    drawing.append(equipment.back, equipment.front);
    const everything: GearTool[] = [{ kind: "chute", open: 1.1, sway: 0.03 }, { kind: "rope", x: 246, y: 50, slack: 5 }, { kind: "hook", x: 246, y: 50 }, { kind: "gun", aim: -0.1 }, { kind: "ladder", lean: 0.2, length: 80 }];
    const records = writes(drawing, () => {
      paintTools(equipment, bearing(everything, { tilt: 0.05, facing: -1 }));
      paintTools(equipment, bearing(everything.slice(2), { tilt: -0.05 }));
      paintTools(equipment, bearing([]));
    });
    const rack = rackLadders(document);
    paintLadders(rack, [{ x0: 0, y0: 0, x1: 20, y1: -80, rungs: 7, opacity: 1 }], 0.8);
    expect(records.length).toBeGreaterThan(20);
    expect(records.filter((record) => !record.startsWith("attributes:"))).toEqual([]);
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("style");
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("id");
    expect(innerHTML).not.toHaveBeenCalled();
    expect(outerHTML).not.toHaveBeenCalled();
    expect(cssText).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    expect(drawing.querySelector("[style], [id], defs, use, style")).toBeNull();
  });
  it("has the rules its drawings need: the roots of ladders and particles at the layer's origin, and the hand of the learner as a cursor", () => {
    const rules = css.replace(/\/\*[\s\S]*?\*\//gu, "");
    const roots = /\.pet-ladders,\s*\.pet-effects \{([^}]*)\}/u.exec(rules)?.[1] ?? "";
    for (const declaration of ["position: absolute;", "left: 0;", "top: 0;", "width: 1px;", "height: 1px;", "overflow: visible;", "transform-origin: 0 0;", "stroke-linejoin: round;", "stroke-linecap: round;"]) expect(roots).toContain(declaration);
    expect(roots).not.toContain("will-change");
    expect(rules).toMatch(/\[data-pet-held\],\s*\[data-pet-held\] \* \{\s*cursor: grabbing !important;\s*\}/u);
    expect(rules.match(/cursor:[^;]*;/gu), "grab is shown on the element under the pointer by the hand itself").toEqual(["cursor: grabbing !important;"]);
    expect(rules, "a rule naming the mark that comes and goes with every pass over a pet would restyle the page with it").not.toContain("data-pet-cursor");
    expect([...rules.matchAll(/([^{}]+)\{([^}]*)\}/gu)].filter(([, , body]) => /pointer-events:\s*(?!none)\S/u.test(body!)).map(([, selector]) => selector!.trim()), "only the touch pads take pointer events").toEqual([".pet-pad"]);
    expect(css).not.toContain("animation-name:");
    expect(rules).not.toMatch(/\.pet-gear/u);
  });
}
