/** 🧩️ `<staffage-figure>` renders without React, updates with its attributes and announces evaluation.
 * @see ./🥒️.feature — the scenarios */
import { afterEach, describe, expect, it } from "vitest";
import { defineStaffageElement } from "@teaching/staffage-react";
import { walker } from "../🧰️support/🧰️.ts";

defineStaffageElement([walker]);

const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0));
type Figure = HTMLElement & { state: { query: { timeS?: number }; frame: { scene: { widthMm: number; heightMm: number } } } | null };

afterEach(() => document.body.replaceChildren());

function make(attributes: Record<string, string>): Figure {
  const element = document.createElement("staffage-figure") as Figure;
  for (const [key, value] of Object.entries(attributes)) element.setAttribute(key, value);
  return element;
}

const stand = { asset: "reference-walker", view: "dark-line", scale: "100", lang: "en", pose: "stand" };

describe("🧩️ custom element", () => {
  it("renders inline SVG at paper size with an accessible label and emits staffage-evaluated", async () => {
    const element = make(stand);
    let evaluated = 0;
    element.addEventListener("staffage-evaluated", () => evaluated++);
    document.body.append(element);
    await flush();
    const figure = element.shadowRoot!.querySelector<HTMLElement>("[role=img]")!;
    expect(element.shadowRoot!.querySelector("svg")).not.toBeNull();
    expect(figure.getAttribute("aria-label")).toMatch(/pose stand.*1:100/u);
    expect(figure.style.width).toBe(`${element.state!.frame.scene.widthMm}mm`);
    expect(evaluated).toBe(1);
  });

  it("re-renders when an observed attribute changes", async () => {
    const element = make(stand);
    document.body.append(element);
    await flush();
    const before = element.state!.frame.scene.heightMm;
    element.setAttribute("scale", "50");
    await flush();
    expect(element.state!.frame.scene.heightMm).toBeGreaterThan(before + 15);
    element.setAttribute("lang", "de");
    await flush();
    expect(element.shadowRoot!.querySelector("[role=img]")!.getAttribute("aria-label")).toMatch(/Maßstab 1:50/u);
  });

  it("renders nothing without a language and evaluates a clip at the given time", async () => {
    const silent = make({ ...stand, lang: "" });
    silent.removeAttribute("lang");
    document.body.append(silent);
    await flush();
    expect(silent.state).toBeNull();
    const clipped = make({ asset: "reference-walker", view: "dark-line", scale: "100", lang: "en", clip: "walk", time: "0.5" });
    document.body.append(clipped);
    await flush();
    expect(clipped.state!.query.timeS).toBeCloseTo(0.5, 9);
  });
});
