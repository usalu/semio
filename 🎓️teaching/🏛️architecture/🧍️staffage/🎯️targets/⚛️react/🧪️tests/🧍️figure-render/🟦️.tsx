/** 🧍️ `StaffageFigure` renders true paper size, fits its parent without changing the scene, and is accessible.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { StaffageFigure, frameOf, figureFrame } from "@teaching/staffage-react";
import { defaultDepictionOptions, depict, evaluate, mmPerMetre } from "@teaching/staffage";
import { walker } from "../🧰️support/🧰️.ts";

afterEach(cleanup);

const base = { asset: walker, scale: { denominator: 100 }, language: "en" as const };
const stand = { kind: "pose", pose: "stand" } as const;

describe("🧍️ figure", () => {
  it("is as wide in CSS millimetres as its paper scene at zoom 1 and scales with zoom", () => {
    const options = defaultDepictionOptions("dark-line", "en", 100);
    const expected = depict(evaluate(walker, stand, options.representation), options);
    const { container, rerender } = render(<StaffageFigure {...base} view="dark-line" query={stand} />);
    const figure = container.querySelector<HTMLElement>("[data-staffage-figure]")!;
    expect(figure.style.width).toBe(`${expected.widthMm}mm`);
    expect(figure.style.height).toBe(`${expected.heightMm}mm`);
    rerender(<StaffageFigure {...base} view="dark-line" query={stand} zoom={2} />);
    expect(container.querySelector<HTMLElement>("[data-staffage-figure]")!.style.width).toBe(`${expected.widthMm * 2}mm`);
    expect(1.8 * mmPerMetre({ denominator: 100 })).toBeCloseTo(18, 9);
  });

  it("fills the parent in contain mode without taking its size from the scene", () => {
    const { container } = render(<StaffageFigure {...base} view="light-line" fit="contain" query={stand} />);
    const figure = container.querySelector<HTMLElement>("[data-staffage-figure]")!;
    const w = Number(figure.getAttribute("data-width-mm"));
    const h = Number(figure.getAttribute("data-height-mm"));
    expect(figure.style.position).toBe("absolute");
    expect(figure.style.width).toBe("100%");
    expect(figure.style.height).toBe("100%");
    expect(figure.style.aspectRatio).toBe("");
    expect(figure.style.overflow).toBe("hidden");
    const svg = figure.querySelector("svg")!;
    expect(svg.getAttribute("viewBox")).toBe(`0 0 ${w} ${h}`);
    expect(svg.getAttribute("preserveAspectRatio")).toBe("xMidYMid meet");
  });

  it("is a focusable image with a bilingual summary label", () => {
    const { container, rerender } = render(<StaffageFigure {...base} view="thermal" query={stand} />);
    const figure = container.querySelector<HTMLElement>("[data-staffage-figure]")!;
    expect(figure.getAttribute("role")).toBe("img");
    expect(figure.tabIndex).toBe(0);
    expect(figure.getAttribute("aria-label")).toMatch(/pose stand.*thermal view.*scale 1:100/u);
    rerender(<StaffageFigure {...base} language="de" view="thermal" query={stand} />);
    expect(container.querySelector("[data-staffage-figure]")!.getAttribute("aria-label")).toMatch(/Pose stand.*Thermikansicht.*Maßstab 1:100/u);
  });

  it("offers the analytic values as a table alternative to colour and reports evaluation", () => {
    const seen: number[] = [];
    const { container } = render(<StaffageFigure {...base} view="thermal" query={stand} onEvaluated={(frame) => seen.push(Object.keys(frame.thermal.regions).length)} />);
    expect(seen).toHaveLength(1);
    expect(seen[0]).toBeGreaterThan(0);
    const table = container.querySelector("table");
    expect(table).not.toBeNull();
    expect(table!.querySelectorAll("tbody tr").length).toBeGreaterThan(0);
  });

  it("shares one frame around the origin across views and passes the theme", () => {
    const scenes = (["dark-line", "thermal"] as const).map((view) => figureFrame(walker, stand, { ...base, view }).scene);
    expect(scenes[0]!.widthMm === scenes[1]!.widthMm && scenes[0]!.heightMm === scenes[1]!.heightMm).toBe(false);
    const frame = frameOf(scenes);
    const { container } = render(
      <>
        <StaffageFigure {...base} view="dark-line" query={stand} frame={frame} />
        <StaffageFigure {...base} view="thermal" query={stand} frame={frame} theme="dark" />
      </>,
    );
    const figures = [...container.querySelectorAll<HTMLElement>("[data-staffage-figure]")];
    expect(figures[0]!.style.width).toBe(figures[1]!.style.width);
    expect(figures[0]!.style.height).toBe(figures[1]!.style.height);
    const [a, b] = figures.map((figure, i) => figure.querySelector("svg")!.getAttribute("viewBox")!.split(" ").map(Number).map((v, k) => v - (k < 2 ? [scenes[i]!.metadata.origin.xMm, scenes[i]!.metadata.origin.yMm][k]! : 0)));
    expect(a!.map((v) => +v.toFixed(6))).toEqual(b!.map((v) => +v.toFixed(6)));
  });
});
