/** 🧊️ The 3D model in the archive's viewer and address: the viewpoint selector offers the axonometric view and the 3D model, a link carries the viewpoint and the camera preset, the viewer shows the model with the shared transport and writes the camera back, and the pages that draw planar viewpoints only say so.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useState, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { Language } from "@teaching/staffage";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { Viewer } from "../../🔨️modules/🎛️viewer/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { comparisonOf } from "../../🔨️modules/⚖️compare/🧮️panes/🟦️.ts";
import { DEFAULT_VIEWER, addressProblems, formatAddress, parseAddress, viewerFromParams, viewerToParams, type ViewerState } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { FIGURE_CHOICES, VIEWPOINT_NAMES, resolveFigure } from "../../🔨️modules/🔭️viewpoints/🟦️.ts";
import { sampleFigure } from "../🧰️support/🧰️.ts";

const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
const wrap = (node: ReactElement, language: Language = "en"): ReactElement => (
  <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>
    {node}
  </ArchiveProvider>
);
const canonical = (hash: string): string => {
  const { route, params } = parseAddress(hash);
  return formatAddress(route, viewerToParams(viewerFromParams(params)));
};

beforeEach(() => {
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ x: 0, y: 0, left: 0, top: 0, right: 160, bottom: 120, width: 160, height: 120, toJSON: () => ({}) });
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
});
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe("🔗️ the address carries the viewpoint and the camera", () => {
  it("reads vp=model and vp=axonometric and the camera preset, writes them canonically and drops the defaults", () => {
    expect(viewerFromParams(new URLSearchParams("vp=model&cam=front")).viewpoint).toBe("model");
    expect(viewerFromParams(new URLSearchParams("vp=model&cam=front")).camera).toBe("front");
    expect(viewerFromParams(new URLSearchParams("vp=axonometric")).viewpoint).toBe("axonometric");
    expect(viewerFromParams(new URLSearchParams("")).camera).toBe("perspective");
    for (const hash of ["#/humans/standing-adult?vp=model", "#/humans/standing-adult?vp=model&cam=plan", "#/vehicles/city-car?view=thermal&vp=model&cam=elevation&scale=50&clip=drive-by", "#/humans/standing-adult?vp=axonometric"]) expect(canonical(hash)).toBe(hash);
    expect(canonical("#/humans/standing-adult?cam=perspective&vp=model")).toBe("#/humans/standing-adult?vp=model");
    expect(viewerToParams({ ...DEFAULT_VIEWER, viewpoint: "model", camera: "axonometric", scale: 50 }).toString()).toBe("vp=model&cam=axonometric&scale=50");
  });

  it("reports an unknown viewpoint or camera in words instead of replacing them silently, and names the pages that draw planar viewpoints only", () => {
    expect(addressProblems(new URLSearchParams("vp=sideways"), "asset").map((problem) => problem.en)).toEqual([expect.stringContaining('unknown viewpoint "sideways"')]);
    expect(addressProblems(new URLSearchParams("vp=model&cam=sideways"), "asset").map((problem) => problem.en)).toEqual([expect.stringContaining('unknown camera "sideways"')]);
    expect(addressProblems(new URLSearchParams("vp=model&cam=front"), "asset")).toEqual([]);
    expect(addressProblems(new URLSearchParams("vp=model"), "compare")[0]!.de).toMatch(/Blickrichtung "model".*nicht zeigen/u);
    expect(addressProblems(new URLSearchParams("p=a~dark-line~~axonometric"), "compare")[0]!.en).toMatch(/viewpoint "axonometric", which this page cannot show/u);
    expect(viewerFromParams(new URLSearchParams("vp=sideways")).viewpoint).toBe("elevation");
  });

  it("compares in the side elevation when the viewer shows the axonometric drawing or the model", () => {
    for (const viewpoint of ["model", "axonometric"] as const) expect(comparisonOf("sample-figure", { viewpoint, selection: null, timeS: null, scale: 100 }).panes.map((pane) => pane.viewpoint)).toEqual(["elevation", "elevation"]);
    expect(comparisonOf("sample-figure", { viewpoint: "plan", selection: null, timeS: null, scale: 100 }).panes[0]!.viewpoint).toBe("plan");
  });
});

describe("🎚️ the viewer", () => {
  function Stateful(props: { readonly initial: ViewerState; readonly patches: Partial<ViewerState>[]; readonly language?: Language }): ReactElement {
    const [viewer, setViewer] = useState(props.initial);
    return wrap(<Viewer asset={sampleFigure} viewer={viewer} onChange={(patch) => { props.patches.push(patch); setViewer((current) => ({ ...current, ...patch })); }} />, props.language);
  }
  const clip: ViewerState = { ...DEFAULT_VIEWER, selection: { kind: "clip", clip: "walk" }, timeS: 0.5 };

  it("offers the three viewpoints, the sheet, the axonometric view and the 3D model as one radio group with names in both languages", () => {
    const patches: Partial<ViewerState>[] = [];
    const { unmount } = render(<Stateful initial={clip} patches={patches} />);
    const group = screen.getByRole("radiogroup", { name: "Viewpoint" });
    expect(within(group).getAllByRole("radio").map((radio) => radio.getAttribute("aria-label"))).toEqual(["Side elevation", "Front elevation", "Plan (top view)", "Three views", "Axonometric view", "3D model"]);
    expect(FIGURE_CHOICES).toEqual(["elevation", "front", "plan", "all", "axonometric", "model"]);
    unmount();
    render(<Stateful initial={clip} patches={patches} language="de" />);
    expect(within(screen.getByRole("radiogroup", { name: "Blickrichtung" })).getAllByRole("radio").map((radio) => radio.getAttribute("aria-label"))).toEqual(["Seitenansicht", "Vorderansicht", "Draufsicht", "Drei Ansichten", "Axonometrie", "3D-Modell"]);
    expect(VIEWPOINT_NAMES.model).toEqual({ en: "3D model", de: "3D-Modell" });
  });

  it("switches to the 3D model with one patch, keeps pose, time and scale, shows the stage with the shared transport and writes the camera back", async () => {
    const patches: Partial<ViewerState>[] = [];
    const { container } = render(<Stateful initial={clip} patches={patches} />);
    expect(container.querySelector("svg")).not.toBeNull();
    fireEvent.click(screen.getByRole("radio", { name: "3D model" }));
    expect(patches).toEqual([{ viewpoint: "model", layout: "single" }]);
    await waitFor(() => expect(container.querySelector("[data-model-stage]")).not.toBeNull());
    await waitFor(() => expect(container.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 60000 });
    expect(container.querySelector("[data-model]")!.getAttribute("aria-label")).toBe("3D model of Sample figure");
    expect(screen.getByRole("group", { name: "Time of the clip" })).toBeTruthy();
    expect(container.querySelector("[data-stage]:not([data-model])")).toBeNull();
    expect(screen.queryByRole("button", { name: ENGLISH["viewer.printSize"] })).toBeNull();
    expect(screen.getByRole("button", { name: ENGLISH["model.zoomIn"] })).toBeTruthy();
    expect(container.querySelector("[data-model-stage]")!.getAttribute("data-view")).toBe("dark-line");
    fireEvent.click(container.querySelector("[data-preset='front']")!);
    expect(patches.at(-1)).toEqual({ camera: "front" });
    fireEvent.click(screen.getByRole("radio", { name: "light solid" }));
    await waitFor(() => expect(container.querySelector("[data-model-stage]")!.getAttribute("data-view")).toBe("light-solid"));
    expect(container.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready");
    fireEvent.click(screen.getByRole("radio", { name: "Plan (top view)" }));
    await waitFor(() => expect(container.querySelector("[data-model-stage]")).toBeNull());
    expect(container.querySelector("svg")).not.toBeNull();
  });

  it("says in words that an asset without a volume has no axonometric view and no 3D model, and keeps both choices in place", () => {
    const bare = { ...sampleFigure, volume: undefined } as never;
    const resolutions = resolveFigure(bare, 100, "en");
    for (const viewpoint of ["axonometric", "model"] as const) {
      const resolution = resolutions[viewpoint];
      expect(resolution.available).toBe(false);
      if (!resolution.available) {
        expect(resolution.reason).toBe("no-volume");
        expect(resolution.message.en).toMatch(/no authored 3D volume/u);
        expect(resolution.message.de).toMatch(/kein erstelltes 3D-Volumen/u);
      }
    }
    expect(resolveFigure(sampleFigure, 100, "en").model.available).toBe(true);
  });
});
