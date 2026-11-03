/** 🔭️ Viewpoints: the selector, the synchronized three-view sheet, unavailability in words, the address, compare, playground and custom element, and the caches and file names.
 * @see ./🥒️.feature — the scenarios */
import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { useState, type ReactElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { VIEWPOINTS, mmPerMetre } from "@teaching/staffage";
import type { Asset, Language, Viewpoint } from "@teaching/staffage";
import { defineStaffageElement, exportFilename, StaffageClock, StaffageFigure } from "@teaching/staffage-react";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { Frieze } from "../../🔨️modules/🏠️overview/🟦️.tsx";
import { STORAGE_KEY, STORAGE_VERSION, ArchiveStore, INITIAL_STATE, memoryStorage, reduce, sanitize } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { depictionOptions, evaluateState, figureFrame, sceneOf } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { PlacedFigure } from "../../🔨️modules/📍️placed/🟦️.tsx";
import { ACROSS_VIEWS, comparisonOf, sameAssetAcrossViewpoints } from "../../🔨️modules/⚖️compare/🧮️panes/🟦️.ts";
import { DEFAULT_VIEWER, addressProblems, compareFromParams, compareToParams, playgroundFromParams, playgroundToParams, viewerFromParams, viewerToParams, type ViewerState } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { SHEET_ORDER, VIEWPOINT_NAMES, resolveViewpoint, sheetOf, threeViewLayout } from "../../🔨️modules/🔭️viewpoints/🟦️.ts";
import { ViewpointSelector } from "../../🔨️modules/🔭️viewpoints/🎚️selector/🟦️.tsx";
import { ThreeViewSheet } from "../../🔨️modules/🔭️viewpoints/🪟️sheet/🟦️.tsx";
import { Viewer } from "../../🔨️modules/🎛️viewer/🟦️.tsx";
import { walker } from "../🧰️support/🧰️.ts";

defineStaffageElement([walker]);

const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
const wrap = (node: ReactElement, language: Language = "en"): ReactElement => (
  <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>
    {node}
  </ArchiveProvider>
);
const flush = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));
const pose = { kind: "pose", pose: walker.presentation.defaultPose } as const;

const withoutPlan: Asset = { ...walker, representations: Object.fromEntries(Object.entries(walker.representations).map(([key, rep]) => [key, rep.status === "authored" ? { ...rep, geometry: { ...rep.geometry, plan: [] } } : rep])) } as unknown as Asset;

afterEach(() => {
  cleanup();
  document.body.replaceChildren();
});

describe("🎚️ selector", () => {
  function Harness(props: { readonly language: Language; readonly seen: string[] }): ReactElement {
    const [value, setValue] = useState<"elevation" | "front" | "plan" | "all">("elevation");
    return wrap(<ViewpointSelector value={value} language={props.language} onChange={(choice) => { props.seen.push(choice); setValue(choice); }} />, props.language);
  }

  it("is one radio group with the approved names in English and German", () => {
    const { unmount } = render(<Harness language="en" seen={[]} />);
    expect(screen.getByRole("radiogroup", { name: "Viewpoint" })).toBeTruthy();
    expect(screen.getAllByRole("radio").map((radio) => radio.getAttribute("aria-label"))).toEqual(["Side elevation", "Front elevation", "Plan (top view)", "Three views"]);
    unmount();
    render(<Harness language="de" seen={[]} />);
    expect(screen.getByRole("radiogroup", { name: "Blickrichtung" })).toBeTruthy();
    expect(screen.getAllByRole("radio").map((radio) => radio.getAttribute("aria-label"))).toEqual(["Seitenansicht", "Vorderansicht", "Draufsicht", "Drei Ansichten"]);
  });

  it("moves and selects with the arrow keys, one tab stop, and announces the change once", () => {
    const seen: string[] = [];
    render(<Harness language="en" seen={seen} />);
    const radios = screen.getAllByRole("radio");
    expect(radios.filter((radio) => radio.getAttribute("tabindex") === "0")).toHaveLength(1);
    expect(radios[0]!.getAttribute("aria-checked")).toBe("true");
    fireEvent.keyDown(radios[0]!, { key: "ArrowRight" });
    expect(seen).toEqual(["front"]);
    fireEvent.keyDown(screen.getAllByRole("radio")[1]!, { key: "ArrowRight" });
    expect(seen).toEqual(["front", "plan"]);
    const regions = document.querySelectorAll("[data-viewpoint-announcement]");
    expect(regions).toHaveLength(1);
    expect(regions[0]!.getAttribute("role")).toBe("status");
    expect(regions[0]!.textContent).toBe("Showing the plan (top view).");
    fireEvent.click(screen.getByRole("radio", { name: "Three views" }));
    expect(regions[0]!.textContent).toBe("Showing three views, focused pane: side elevation.");
  });

  it("marks an unavailable viewpoint with its reason in the name and keeps it selectable", () => {
    const resolutions = { elevation: resolveViewpoint(withoutPlan, "elevation", 100), front: resolveViewpoint(withoutPlan, "front", 100), plan: resolveViewpoint(withoutPlan, "plan", 100) };
    const seen: string[] = [];
    render(wrap(<ViewpointSelector value="elevation" language="en" resolutions={resolutions} onChange={(choice) => seen.push(choice)} />));
    const plan = screen.getByRole("radio", { name: /Plan \(top view\) \(unavailable\): This drawing has no plan/u });
    expect(plan.getAttribute("title")).toMatch(/no plan/u);
    fireEvent.click(plan);
    expect(seen).toEqual(["plan"]);
  });
});

describe("🪟️ the viewer", () => {
  function Stateful(props: { readonly initial: ViewerState; readonly patches: Partial<ViewerState>[] }): ReactElement {
    const [viewer, setViewer] = useState(props.initial);
    return wrap(<Viewer asset={walker} viewer={viewer} onChange={(patch) => { props.patches.push(patch); setViewer((current) => ({ ...current, ...patch })); }} />);
  }
  const initial: ViewerState = { ...DEFAULT_VIEWER, scale: 50, selection: { kind: "clip", clip: "walk" }, timeS: 0.5, rate: 0.5, view: "thermal", range: { mode: "span", spanK: 15 }, environment: { airTempC: 5 } };

  it("changes only the camera: a switch writes viewpoint and layout and nothing else", () => {
    const patches: Partial<ViewerState>[] = [];
    render(<Stateful initial={initial} patches={patches} />);
    fireEvent.click(screen.getByRole("radio", { name: "Plan (top view)" }));
    fireEvent.click(screen.getByRole("radio", { name: "Three views" }));
    fireEvent.click(screen.getByRole("radio", { name: "Front elevation" }));
    expect(patches).toEqual([{ viewpoint: "plan", layout: "single" }, { layout: "all" }, { viewpoint: "front", layout: "single" }]);
  });

  it("keeps pose, time, playback, scale and environment in the figure across the switch", () => {
    const patches: Partial<ViewerState>[] = [];
    const { container } = render(<Stateful initial={initial} patches={patches} />);
    const read = () => {
      const figure = container.querySelector<HTMLElement>("[data-staffage-figure]")!;
      return { time: figure.getAttribute("data-time"), label: figure.getAttribute("aria-label")!.replace(/, (side elevation|front elevation|plan|top view)[^,]*,/iu, ","), viewpoint: figure.getAttribute("data-viewpoint") };
    };
    const before = read();
    expect(before.viewpoint).toBe("elevation");
    fireEvent.click(screen.getByRole("radio", { name: "Plan (top view)" }));
    const after = read();
    expect(after.viewpoint).toBe("plan");
    expect(after.time).toBe(before.time);
    expect(Number(after.time)).toBeCloseTo(0.5, 6);
  });

  it("shows the three views on one sheet with plan above, front beside, and names the focused pane", () => {
    const { container } = render(<Stateful initial={{ ...initial, layout: "all", viewpoint: "front" }} patches={[]} />);
    const sheet = container.querySelector("[data-three-views]")!;
    expect(sheet.getAttribute("role")).toBe("group");
    expect([...sheet.querySelectorAll("figure[data-viewpoint]")].map((cell) => cell.getAttribute("data-viewpoint"))).toEqual([...SHEET_ORDER]);
    const focus = [...sheet.querySelectorAll("figure[data-focus]")].map((cell) => cell.getAttribute("data-viewpoint"));
    expect(focus).toEqual(["front"]);
    expect(within(sheet as HTMLElement).getByRole("button", { name: "Front elevation, focused pane" }).getAttribute("aria-pressed")).toBe("true");
  });
});

describe("📐️ the three-view layout", () => {
  const frames = { elevation: { minX: -10, maxX: 30, minY: -50, maxY: 5 }, front: { minX: -8, maxX: 8, minY: -48, maxY: 3 }, plan: { minX: -12, maxX: 28, minY: -9, maxY: 9 } };

  it("shares the forward span of side and plan and the height span of side and front", () => {
    const layout = threeViewLayout(frames);
    expect(layout.cells.plan.frame).toMatchObject({ minX: -12, maxX: 30 });
    expect(layout.cells.elevation.frame).toMatchObject({ minX: -12, maxX: 30, minY: -50, maxY: 5 });
    expect(layout.cells.front.frame).toMatchObject({ minY: -50, maxY: 5, minX: -8, maxX: 8 });
    expect(layout.cells.plan.widthMm).toBe(layout.cells.elevation.widthMm);
    expect(layout.cells.front.heightMm).toBe(layout.cells.elevation.heightMm);
    expect([layout.cells.plan, layout.cells.elevation, layout.cells.front].map((cell) => [cell.column, cell.row])).toEqual([[1, 1], [1, 2], [2, 2]]);
    expect(layout.widthMm).toBe(layout.cells.elevation.widthMm + layout.gapMm + layout.cells.front.widthMm);
  });

  it("keeps the cell of an unavailable viewpoint so the others do not move", () => {
    const layout = threeViewLayout({ ...frames, plan: null });
    expect(layout.cells.plan.frame).toMatchObject({ minX: frames.elevation.minX, maxX: frames.elevation.maxX });
    expect(layout.cells.elevation.frame).toEqual({ minX: -10, maxX: 30, minY: -50, maxY: 5 });
  });

  it("draws the real walker with one millimetres-per-metre in all three scenes", () => {
    const k = mmPerMetre({ denominator: 50 });
    const scenes = VIEWPOINTS.map((viewpoint) => figureFrame(walker, pose, { view: "dark-line", viewpoint, scale: { denominator: 50 }, language: "en" }).scene);
    expect(scenes.map((scene) => scene.metadata.mmPerMetre)).toEqual([k, k, k]);
    expect(scenes.map((scene) => scene.metadata.viewpoint)).toEqual([...VIEWPOINTS]);
  });

  it("projects the three viewpoints of one instant from one evaluated state", () => {
    const state = evaluateState(walker, { kind: "clip", clip: "walk", timeS: 0.4 });
    const scenes = VIEWPOINTS.map((viewpoint) => sceneOf(state, depictionOptions({ view: "dark-line", viewpoint, scale: { denominator: 100 }, language: "en" })));
    expect(new Set(scenes.map((scene) => scene.metadata.timeS)).size).toBe(1);
    expect(new Set(scenes.map((scene) => JSON.stringify(scene.metadata.query))).size).toBe(1);
  });
});

describe("🎞️ the sheet follows one clock", () => {
  it("shows the same instant in all three views while scrubbing and keeps the zoom", () => {
    const clock = new StaffageClock({ fps: 30, durationS: 1, playing: false, reducedMotion: false });
    const sheet = sheetOf(walker, [{ kind: "clip", clip: "walk", timeS: 0 }], { view: "dark-line", denominator: 100, language: "en" });
    const { container } = render(wrap(<ThreeViewSheet asset={walker} clock={clock} view="dark-line" scale={{ denominator: 100 }} language="en" sheet={sheet} zoom={1} focus="elevation" />));
    const times = () => [...container.querySelectorAll("[data-staffage-figure]")].map((figure) => figure.getAttribute("data-time"));
    const zoom = container.querySelector("[data-three-views]")!.getAttribute("data-zoom");
    expect(times()).toHaveLength(3);
    for (const frame of [0, 7, 19, 3]) {
      act(() => clock.seek(frame / 30));
      expect(new Set(times()).size).toBe(1);
      expect(Number(times()[0])).toBeCloseTo(frame / 30, 9);
    }
    expect(container.querySelector("[data-three-views]")!.getAttribute("data-zoom")).toBe(zoom);
  });

  it("orders the cells plan, side elevation, front elevation so the tab order follows the grid", () => {
    const sheet = sheetOf(walker, [pose], { view: "dark-line", denominator: 100, language: "en" });
    const { container } = render(wrap(<ThreeViewSheet asset={walker} view="dark-line" scale={{ denominator: 100 }} language="en" sheet={sheet} zoom={1} focus="elevation" />));
    const cells = [...container.querySelectorAll<HTMLElement>("figure[data-viewpoint]")];
    expect(cells.map((cell) => cell.getAttribute("data-viewpoint"))).toEqual(["plan", "elevation", "front"]);
    expect(cells.map((cell) => [cell.style.gridColumn, cell.style.gridRow])).toEqual([["1", "1"], ["1", "2"], ["2", "2"]]);
    expect(cells.map((cell) => cell.getAttribute("aria-label"))).toEqual(["Plan (top view)", "Side elevation", "Front elevation"]);
  });
});

describe("🚫️ unavailability is explicit", () => {
  it("names a missing viewpoint and keeps it empty, never another view", () => {
    const sheet = sheetOf(withoutPlan, [pose], { view: "dark-line", denominator: 100, language: "en" });
    const { container } = render(wrap(<ThreeViewSheet asset={withoutPlan} view="dark-line" scale={{ denominator: 100 }} language="en" sheet={sheet} zoom={1} focus="elevation" />));
    const plan = container.querySelector("figure[data-viewpoint=plan]")!;
    expect(plan.getAttribute("data-available")).toBe("false");
    expect(plan.querySelector("svg")).toBeNull();
    expect(plan.textContent).toContain("The plan (top view) is not available");
    expect(plan.textContent).toContain("This drawing has no plan (top view).");
    expect(plan.textContent).toContain("No other view is shown in its place.");
    expect(container.querySelector("figure[data-viewpoint=elevation] svg")).not.toBeNull();
    expect(container.querySelector("figure[data-viewpoint=front] svg")).not.toBeNull();
    expect(container.querySelectorAll("[data-three-views] > [role=status]")).toHaveLength(1);
    expect(plan.querySelector("[role=status]")).toBeNull();
  });

  it("says why every viewpoint is unavailable at a scale the asset does not draw", () => {
    for (const viewpoint of VIEWPOINTS) {
      const resolution = resolveViewpoint(walker, viewpoint, 500, "de");
      expect(resolution.available).toBe(false);
      if (!resolution.available) {
        expect(resolution.reason).toBe("scale-unavailable");
        expect(resolution.message.en.length).toBeGreaterThan(10);
        expect(resolution.message.de.length).toBeGreaterThan(10);
      }
    }
  });

  it("shows the viewer notice in place of the stage and keeps the clock controls", () => {
    const { container } = render(wrap(<Viewer asset={withoutPlan} viewer={{ ...DEFAULT_VIEWER, viewpoint: "plan", selection: { kind: "clip", clip: "walk" } }} onChange={() => undefined} />));
    expect(container.querySelector("[data-stage]")).toBeNull();
    expect(container.querySelector("[data-notice=unavailable]")!.textContent).toContain("This drawing has no plan (top view).");
  });
});

describe("🔗️ the address", () => {
  it("round-trips vp and vl with every other parameter in a stable order", () => {
    const state: ViewerState = { ...DEFAULT_VIEWER, viewpoint: "plan", layout: "all", scale: 50, view: "thermal", selection: { kind: "clip", clip: "walk" }, timeS: 1.25 };
    const params = viewerToParams(state);
    expect(params.toString()).toBe("view=thermal&vp=plan&vl=all&scale=50&clip=walk&t=1.25");
    expect(viewerFromParams(new URLSearchParams(params.toString()))).toEqual(state);
    expect(viewerToParams(DEFAULT_VIEWER).toString()).toBe("");
  });

  it("reports an unknown viewpoint, layout or representation in words in both languages", () => {
    const params = new URLSearchParams("vp=side&vl=triple&view=infrared");
    expect(viewerFromParams(params).viewpoint).toBe("elevation");
    const problems = addressProblems(params, "asset");
    expect(problems).toHaveLength(3);
    expect(problems[0]!.en).toMatch(/unknown viewpoint "side"/u);
    expect(problems[0]!.de).toMatch(/unbekannte Blickrichtung "side"/u);
    expect(addressProblems(new URLSearchParams("vp=plan&vl=all&view=thermal"), "asset")).toEqual([]);
  });

  it("carries a viewpoint per compare pane and in the playground, and reports a wrong one", () => {
    const compare = { panes: sameAssetAcrossViewpoints("a-one", "thermal"), scale: 50, timeS: null };
    const text = compareToParams(compare).toString();
    expect(decodeURIComponent(text)).toBe("p=a-one~thermal&p=a-one~thermal~~front&p=a-one~thermal~~plan&scale=50");
    expect(compareFromParams(new URLSearchParams(text))).toEqual(compare);
    expect(addressProblems(new URLSearchParams("p=a-one~thermal~~sideways"), "compare")).toHaveLength(1);
    const playground = { scale: 50, view: "light-solid" as const, viewpoint: "plan" as const, background: "dark" as const, lengthM: 30, placements: [] };
    const params = playgroundToParams(playground);
    expect(params.get("vp")).toBe("plan");
    expect(playgroundFromParams(new URLSearchParams(params.toString()))).toEqual(playground);
    expect(addressProblems(new URLSearchParams("vp=sideways"), "playground")).toHaveLength(1);
  });

  it("offers a same-asset comparison across the three viewpoints in one representation", () => {
    const panes = sameAssetAcrossViewpoints("a-one", "forces", { kind: "pose", pose: "stand" });
    expect(panes.map((pane) => pane.viewpoint)).toEqual([...VIEWPOINTS]);
    expect(new Set(panes.map((pane) => pane.view)).size).toBe(1);
    expect(ACROSS_VIEWS.length).toBeGreaterThan(0);
  });
});

describe("🧪️ playground", () => {
  it("stores the viewpoint, sanitizes a stored wrong one and exports a file name with it", () => {
    const state = reduce(INITIAL_STATE, { type: "playground-configured", patch: { viewpoint: "front" } });
    expect(state.playground.viewpoint).toBe("front");
    expect(sanitize({ version: 1, state: { playground: { viewpoint: "sideways" }, preferences: { previewViewpoint: "plan" } } }, ["dark-line"]).playground.viewpoint).toBe("elevation");
    expect(sanitize({ version: 1, state: { preferences: { previewViewpoint: "plan" } } }, ["dark-line"]).preferences.previewViewpoint).toBe("plan");
  });

  it("mirrors a placed figure only in the side elevation", () => {
    const place = (viewpoint: Viewpoint) => {
      const clock = new StaffageClock({ fps: 12, playing: false, reducedMotion: true });
      const { container, unmount } = render(<PlacedFigure asset={walker} selection={{ kind: "pose", pose: walker.presentation.defaultPose }} clock={clock} view="dark-line" viewpoint={viewpoint} scale={100} language="en" pxPerMm={3} xPx={50} groundPx={100} mirrored />);
      const transform = container.querySelector<HTMLElement>("[data-placed]")!.style.transform;
      unmount();
      return transform;
    };
    expect(place("elevation")).toBe("scaleX(-1)");
    expect(place("front")).toBe("");
    expect(place("plan")).toBe("");
  });

  it("marks a placed figure whose viewpoint is unavailable instead of dropping it", () => {
    const clock = new StaffageClock({ fps: 12, playing: false, reducedMotion: true });
    const { container } = render(<PlacedFigure asset={withoutPlan} selection={null} clock={clock} view="dark-line" viewpoint="plan" scale={100} language="en" pxPerMm={3} xPx={50} groundPx={100} />);
    const marker = container.querySelector("[data-placed-unavailable]")!;
    expect(marker.getAttribute("data-placed-unavailable")).toBe("view-missing");
    expect(marker.getAttribute("aria-label")).toMatch(/The plan \(top view\) is not available/u);
  });
});

describe("🧩️ custom element", () => {
  const make = (attributes: Record<string, string>): HTMLElement => {
    const element = document.createElement("staffage-figure");
    for (const [key, value] of Object.entries({ asset: "reference-walker", view: "dark-line", scale: "100", lang: "en", pose: "stand", ...attributes })) element.setAttribute(key, value);
    return element;
  };

  it("draws the viewpoint named by the attribute", async () => {
    for (const viewpoint of VIEWPOINTS) {
      const element = make({ viewpoint });
      document.body.append(element);
      await flush();
      expect(element.shadowRoot!.querySelector("[role=img]")!.getAttribute("data-viewpoint")).toBe(viewpoint);
      element.remove();
    }
  });

  it("shows an explicit problem for a missing or unknown viewpoint, in the element's language", async () => {
    const cases: readonly (readonly [Record<string, string>, RegExp])[] = [[{ viewpoint: "" }, /viewpoint is required/u], [{ viewpoint: "sideways" }, /sideways/u], [{ viewpoint: "sideways", lang: "de" }, /ist erforderlich/u]];
    for (const [attributes, expected] of cases) {
      const element = make(attributes);
      document.body.append(element);
      await flush();
      const alert = element.shadowRoot!.querySelector("[role=alert]");
      expect(alert, JSON.stringify(attributes)).not.toBeNull();
      expect(alert!.textContent).toMatch(expected);
      expect(element.shadowRoot!.querySelector("svg")).toBeNull();
      element.remove();
    }
  });

  it("draws the aligned three-view sheet for viewpoint all", async () => {
    const element = make({ viewpoint: "all" });
    document.body.append(element);
    await flush();
    const cells = [...element.shadowRoot!.querySelectorAll("figure[data-viewpoint]")];
    expect(cells.map((cell) => cell.getAttribute("data-viewpoint"))).toEqual([...SHEET_ORDER]);
    expect(cells.every((cell) => cell.querySelector("svg") !== null)).toBe(true);
  });
});

describe("🗃️ caches and file names", () => {
  it("keys scenes by viewpoint, representation, time and scale, and reuses an identical request", () => {
    const state = evaluateState(walker, { kind: "clip", clip: "walk", timeS: 0.2 });
    const options = (patch: Partial<{ view: "dark-line" | "thermal"; viewpoint: Viewpoint; denominator: number }>) => depictionOptions({ view: patch.view ?? "dark-line", viewpoint: patch.viewpoint ?? "elevation", scale: { denominator: patch.denominator ?? 100 }, language: "en" });
    const base = sceneOf(state, options({}));
    expect(sceneOf(state, options({}))).toBe(base);
    expect(sceneOf(state, options({ viewpoint: "plan" }))).not.toBe(base);
    expect(sceneOf(state, options({ viewpoint: "front" }))).not.toBe(sceneOf(state, options({ viewpoint: "plan" })));
    expect(sceneOf(state, options({ view: "thermal" }))).not.toBe(base);
    expect(sceneOf(state, options({ denominator: 50 }))).not.toBe(base);
    const later = evaluateState(walker, { kind: "clip", clip: "walk", timeS: 0.6 });
    expect(sceneOf(later, options({}))).not.toBe(base);
  });

  it("puts the viewpoint into every file name so the three outputs never collide", () => {
    const names = VIEWPOINTS.flatMap((viewpoint) => (["svg", "pdf", "dxf", "png-sequence"] as const).map((format) => exportFilename(walker, pose, { view: "dark-line", viewpoint, scale: { denominator: 100 }, language: "en" }, format)));
    expect(new Set(names).size).toBe(names.length);
    for (const viewpoint of VIEWPOINTS) expect(names.filter((name) => name.includes(`_${viewpoint}_`))).toHaveLength(4);
  });

  it("names the viewpoints with the approved terms", () => {
    expect(VIEWPOINTS.map((viewpoint) => VIEWPOINT_NAMES[viewpoint].en)).toEqual(["Side elevation", "Front elevation", "Plan (top view)"]);
    expect(VIEWPOINTS.map((viewpoint) => VIEWPOINT_NAMES[viewpoint].de)).toEqual(["Seitenansicht", "Vorderansicht", "Draufsicht"]);
    expect(VIEWPOINT_NAMES.all).toEqual({ en: "Three views", de: "Drei Ansichten" });
  });
});

describe("🏷️ fidelity", () => {
  const rep = Object.values(walker.representations).find((candidate) => candidate.status === "authored")!;
  const withSymbol = {
    ...walker,
    symbols: [{ id: "walker-sign", title: { en: "Walker sign", de: "Gehzeichen" }, description: { en: "A sign.", de: "Ein Zeichen." }, semantics: "nominal", units: "paper-mm", denominators: [100], pens: (rep as { pens: unknown }).pens, hatches: [], geometry: (rep as { geometry: unknown }).geometry, dimensionalNote: { en: "Conventional symbol, not to scale.", de: "Konventionelles Symbol, nicht maßstäblich." } }],
  } as unknown as Asset;

  it("shows a visible, described note on a conventional symbol in both languages and none on a physical drawing", () => {
    const symbol = { kind: "symbol", symbol: "walker-sign" } as const;
    for (const [language, text] of [["en", "Conventional symbol, not to scale"], ["de", "Konventionelles Symbol, nicht maßstäblich"]] as const) {
      const { container, unmount } = render(<StaffageFigure asset={withSymbol} view="dark-line" viewpoint="plan" scale={{ denominator: 100 }} language={language} representation={symbol} query={pose} />);
      const figure = container.querySelector<HTMLElement>("[data-staffage-figure]")!;
      expect(figure.getAttribute("data-fidelity")).toBe("conventional-symbol");
      const note = container.querySelector<HTMLElement>("[data-fidelity-note]")!;
      expect(note.textContent).toBe(text);
      expect(figure.getAttribute("aria-describedby")!.split(" ")).toContain(note.id);
      unmount();
    }
    const physical = render(<StaffageFigure asset={walker} view="dark-line" viewpoint="plan" scale={{ denominator: 100 }} language="en" query={pose} />);
    expect(physical.container.querySelector("[data-fidelity-note]")).toBeNull();
    expect(physical.container.querySelector("[data-staffage-figure]")!.getAttribute("data-fidelity")).toBe("physical");
  });
});

describe("▨ hatch patterns are the default reading", () => {
  it("is on in the viewer state, drops out of the address and only an explicit off is written", () => {
    expect(DEFAULT_VIEWER.patterns).toBe(true);
    expect(viewerToParams(DEFAULT_VIEWER).has("pat")).toBe(false);
    expect(viewerToParams({ ...DEFAULT_VIEWER, patterns: false }).get("pat")).toBe("0");
    expect(viewerFromParams(new URLSearchParams("pat=0")).patterns).toBe(false);
    expect(viewerFromParams(new URLSearchParams("")).patterns).toBe(true);
  });

  it("is on in the pipeline and in the custom element unless no-patterns is set", async () => {
    expect(depictionOptions({ view: "thermal", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" }).patterns).toBe(true);
    const element = document.createElement("staffage-figure");
    for (const [key, value] of Object.entries({ asset: "reference-walker", view: "forces", viewpoint: "elevation", scale: "100", lang: "en", pose: "stand" })) element.setAttribute(key, value);
    document.body.append(element);
    await flush();
    const hatched = element.shadowRoot!.innerHTML;
    element.setAttribute("no-patterns", "");
    await flush();
    expect(element.shadowRoot!.innerHTML).not.toBe(hatched);
  });
});

describe("🚨️ nothing is replaced silently", () => {
  it("reports a stored unknown viewpoint and uses the side elevation", () => {
    const storage = memoryStorage();
    storage.setItem(STORAGE_KEY, JSON.stringify({ version: STORAGE_VERSION, state: { preferences: { previewViewpoint: "sideways" }, playground: { viewpoint: "diagonal" } } }));
    const store = new ArchiveStore(storage, ["dark-line"]);
    expect(store.getSnapshot().preferences.previewViewpoint).toBe("elevation");
    expect(store.getSnapshot().playground.viewpoint).toBe("elevation");
    expect(store.problems.map((problem) => problem.en)).toEqual([expect.stringContaining('"sideways"'), expect.stringContaining('"diagonal"')]);
    expect(store.problems[0]!.de).toContain("unbekannt");
    expect(new ArchiveStore(memoryStorage(), ["dark-line"]).problems).toEqual([]);
  });

  it("refuses to mirror outside the side elevation at input, in the store and in a link", () => {
    const placement = { id: "p", asset: "a-b", xM: 1, zM: 0, yawDeg: 0, pick: null, offsetS: 0, mirrored: false };
    const base = { ...INITIAL_STATE, playground: { ...INITIAL_STATE.playground, placements: [placement] } };
    expect(reduce(base, { type: "placement-changed", id: "p", patch: { mirrored: true } }).playground.placements[0]!.mirrored).toBe(true);
    const front = { ...base, playground: { ...base.playground, viewpoint: "front" as const } };
    expect(reduce(front, { type: "placement-changed", id: "p", patch: { mirrored: true } }).playground.placements[0]!.mirrored).toBe(false);
    const link = new URLSearchParams("vp=plan&p=a-b~1~~~m");
    expect(playgroundFromParams(link)!.placements[0]!.mirrored).toBe(false);
    expect(addressProblems(link, "playground")).toHaveLength(1);
    expect(playgroundFromParams(new URLSearchParams("p=a-b~1~~~m"))!.placements[0]!.mirrored).toBe(true);
  });

  it("keeps pose or clip and time in the link from the asset page to the comparison", () => {
    const state = comparisonOf("reference-walker", { ...DEFAULT_VIEWER, viewpoint: "plan", selection: { kind: "clip", clip: "walk" }, timeS: 0.75, scale: 50 });
    expect(state.panes.map((pane) => [pane.view, pane.viewpoint, pane.selection])).toEqual([["dark-line", "plan", { kind: "clip", clip: "walk" }], ["thermal", "plan", { kind: "clip", clip: "walk" }]]);
    expect(compareFromParams(compareToParams(state))).toEqual(state);
    expect(state.timeS).toBe(0.75);
  });
});

describe("🧍️ the frieze follows the preview viewpoint", () => {
  it("draws every figure in the reader's viewpoint, and the playground preview in its own", () => {
    vi.stubGlobal("ResizeObserver", class { constructor(private readonly callback: (entries: unknown[]) => void) {} observe(): void { this.callback([{ contentRect: { width: 800, height: 300 } }]); } disconnect(): void {} });
    const row = { entry: { id: walker.id, category: "humans", title: walker.title }, status: "ready", asset: walker } as never;
    const store = new ArchiveStore(memoryStorage(), ["dark-line"]);
    store.dispatch({ type: "preferences-changed", patch: { previewViewpoint: "plan", motion: "reduced" } });
    const view = (node: ReactElement) => render(<ArchiveProvider language="en" t={translator("en")} catalog={catalog} store={store}>{node}</ArchiveProvider>);
    const first = view(<Frieze rows={[row]} label="Frieze" />);
    expect(first.container.querySelector("[data-frieze-note]")!.textContent).toMatch(/Presentation layout.*plan \(top view\)/u);
    expect([...first.container.querySelectorAll("[data-placed]")].map((node) => node.getAttribute("data-viewpoint"))).toEqual(["plan"]);
    first.unmount();
    const second = view(<Frieze rows={[row]} label="Frieze" viewpoint="front" />);
    expect(second.container.querySelector("[data-frieze-note]")!.textContent).toMatch(/front elevation/u);
    expect([...second.container.querySelectorAll("[data-placed]")].map((node) => node.getAttribute("data-viewpoint"))).toEqual(["front"]);
    vi.unstubAllGlobals();
  });
});
