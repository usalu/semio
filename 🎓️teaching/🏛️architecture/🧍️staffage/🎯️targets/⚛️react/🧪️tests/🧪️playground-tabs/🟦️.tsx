/** 🧪️ The three playgrounds share one scene: tabs with addresses, one placement in every tab, per-figure and global playback, the timeline's keyboard sliders, the 3D scene's progress, cancel and readiness, and a catalog that changes under the page.
 * The 3D viewer runs on its software rasteriser here (jsdom has no WebGL2); the WebGL2 path is checked in a real browser.
 * @see ./🥒️.feature — the scenarios */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { VIEWS } from "@teaching/staffage";
import type { Asset, Manifest } from "@teaching/staffage";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { CatalogStore, type CategoryDocument } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { PlaygroundPage } from "../../🔨️modules/🧪️playground/🟦️.tsx";
import { sampleFigure } from "../🧰️support/🧰️.ts";

vi.mock("@semio-tech/ui-react/chrome", async (original) => ({ ...(await original<Record<string, unknown>>()), Icon: () => null }));

const entryOf = (asset: Asset) => ({ id: asset.id, category: "humans", title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM, powerW: 0, poses: [], clips: [], representations: { "500": "authored", "200": "authored", "100": "authored", "50": "authored" }, symbols: [], analytics: ["thermal", "structural"], exports: [], distinction: asset.summary, completion: "authored" });
const manifest = { $schema: "", version: "semio.staffage/v2", categories: [{ id: "humans", emoji: "", title: { en: "Humans", de: "Menschen" }, description: { en: "h", de: "h" }, folder: "humans" }], collections: [], entries: [entryOf(sampleFigure)] } as unknown as Manifest;

let current: Asset = sampleFigure;
const catalog = (): CatalogStore => new CatalogStore({ manifest: async () => manifest, category: async () => ({ assets: [current], revisions: {} }) satisfies CategoryDocument });

beforeEach(() => {
  current = sampleFigure;
  window.location.hash = "";
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ x: 0, y: 0, left: 0, top: 0, right: 640, bottom: 400, width: 640, height: 400, toJSON: () => ({}) });
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
});
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  window.location.hash = "";
});

async function setup(hash = "#/playground", language: "en" | "de" = "en") {
  window.location.hash = hash;
  const source = catalog();
  await source.loadManifest();
  const store = new ArchiveStore(memoryStorage(), VIEWS);
  store.dispatch({ type: "preferences-changed", patch: { motion: "reduced", language } });
  render(
    <ArchiveProvider language={language} t={translator(language)} catalog={source} store={store}>
      <PlaygroundPage view={{ opened: true, revealed: false }} />
    </ArchiveProvider>,
  );
  return { store, source };
}

const pointer = (element: Element, type: "pointerdown" | "pointermove" | "pointerup", x: number, y: number): void => {
  act(() => {
    element.dispatchEvent(new MouseEvent(type, { clientX: x, clientY: y, bubbles: true, cancelable: true }));
  });
};
const placed = (store: ArchiveStore) => store.getSnapshot().playground.placements;
const place = async (title = /Place Sample figure/u) => fireEvent.click(await screen.findByRole("button", { name: title }, { timeout: 30000 }));

describe("🧭️ the tabs", () => {
  it("offers the section, the floor plan and the 3D playground at their own addresses and keeps the scene when switching", async () => {
    const { store } = await setup();
    const links = within(screen.getByRole("navigation", { name: "Playgrounds" })).getAllByRole("link");
    expect(links.map((link) => [link.textContent, link.getAttribute("href"), link.getAttribute("aria-current")])).toEqual([["Section", "#/playground", "page"], ["Floor plan", "#/playground/plan", null], ["3D", "#/playground/3d", null]]);
    await place();
    const id = placed(store)[0]!.id;
    expect(document.querySelector("[data-stage]")!.getAttribute("data-stage")).toBe("section");
    window.location.hash = "#/playground/plan";
    await waitFor(() => expect(document.querySelector("[data-stage='plan']")).not.toBeNull());
    expect(document.querySelector(`[data-placement="${id}"]`)).not.toBeNull();
    expect(document.querySelector("[data-viewpoint='plan'][role='group']")!.getAttribute("aria-label")).toMatch(/Floor plan with 1 placed figures at scale 1:100/u);
    expect(screen.getByRole("link", { name: "Floor plan" }).getAttribute("aria-current")).toBe("page");
    window.location.hash = "#/playground";
    await waitFor(() => expect(document.querySelector("[data-stage='section']")).not.toBeNull());
    expect(placed(store).map((placement) => placement.id)).toEqual([id]);
  });

  it("chooses between the side and the front elevation in the section only", async () => {
    const { store } = await setup();
    const group = await screen.findByRole("radiogroup", { name: "Viewpoint" });
    expect(within(group).getAllByRole("radio").map((radio) => radio.getAttribute("aria-label"))).toEqual(["Side elevation", "Front elevation"]);
    fireEvent.click(within(group).getByRole("radio", { name: "Front elevation" }));
    expect(store.getSnapshot().playground.viewpoint).toBe("front");
    window.location.hash = "#/playground/plan";
    await waitFor(() => expect(document.querySelector("[data-stage='plan']")).not.toBeNull());
    expect(screen.queryByRole("radiogroup", { name: "Viewpoint" })).toBeNull();
    expect(document.querySelector("[data-viewpoint='plan']")).not.toBeNull();
  });

  it("drags a figure in both directions in the plan, up being the asset's left", async () => {
    const { store } = await setup("#/playground/plan");
    await place();
    const before = placed(store)[0]!;
    const metrePx = Number(document.querySelector("[data-metre-px]")!.getAttribute("data-metre-px"));
    const hit = document.querySelector<HTMLElement>(`[data-placement="${before.id}"]`)!;
    pointer(hit, "pointerdown", 100, 100);
    pointer(hit, "pointermove", 100 + metrePx, 100 - metrePx / 2);
    pointer(hit, "pointerup", 100 + metrePx, 100 - metrePx / 2);
    const after = placed(store)[0]!;
    expect(after.xM).toBeCloseTo(before.xM + 1, 6);
    expect(after.zM).toBeCloseTo(0.5, 6);
    await waitFor(() => expect(document.querySelector("[aria-live=polite]")!.textContent).toMatch(/Dropped Sample figure at x .* m, z .* m/u));
  });
});

describe("🎛️ playback of one figure and of the timeline", () => {
  it("pauses, scrubs, loops, speeds up and moves one figure without touching the others", async () => {
    const { store } = await setup();
    await place();
    await place();
    const [first, second] = placed(store);
    fireEvent.focus(document.querySelector<HTMLElement>(`[data-placement="${first!.id}"]`)!);
    const panel = await waitFor(() => {
      const found = document.querySelector<HTMLElement>(`[data-asset-panel="${first!.id}"]`);
      expect(found).not.toBeNull();
      return within(found!);
    });
    fireEvent.click(panel.getByRole("button", { name: "Pause Sample figure" }));
    expect(placed(store)[0]!.holdS).not.toBeNull();
    expect(placed(store)[1]!.holdS).toBeNull();
    fireEvent.change(panel.getByRole("slider", { name: "Clip time" }), { target: { value: "0.5" } });
    expect(placed(store)[0]!.holdS).toBeCloseTo(0.5, 6);
    fireEvent.click(panel.getByRole("button", { name: "Play Sample figure" }));
    expect(placed(store)[0]!.holdS).toBeNull();
    fireEvent.click(panel.getByRole("checkbox", { name: "Loop" }));
    expect(placed(store)[0]!.loop).toBe(false);
    fireEvent.change(panel.getByRole("combobox", { name: "Speed" }), { target: { value: "2" } });
    expect(placed(store)[0]!.speed).toBe(2);
    fireEvent.change(panel.getByRole("combobox", { name: "Pose or clip" }), { target: { value: "clip:idle-breathing" } });
    expect(placed(store)[0]!.pick).toEqual({ kind: "clip", clip: "idle-breathing" });
    expect(placed(store)[1]).toMatchObject({ pick: null, speed: 1, loop: true, id: second!.id });
  });

  it("moves and resizes a window on the timeline by keyboard, with the stated steps, and announces it", async () => {
    const { store } = await setup();
    await place();
    const id = placed(store)[0]!.id;
    const bar = await waitFor(() => {
      const found = document.querySelector<HTMLElement>(`[data-bar="${id}"]`);
      expect(found).not.toBeNull();
      return found!;
    });
    expect(bar.getAttribute("role")).toBe("slider");
    expect(bar.getAttribute("aria-label")).toBe("Start of Sample figure on the timeline");
    fireEvent.keyDown(bar, { key: "ArrowRight" });
    expect(placed(store)[0]!.startS).toBeCloseTo(0.1, 6);
    fireEvent.keyDown(document.querySelector(`[data-bar="${id}"]`)!, { key: "ArrowRight", shiftKey: true });
    expect(placed(store)[0]!.startS).toBeCloseTo(1.1, 6);
    fireEvent.keyDown(document.querySelector(`[data-bar="${id}"]`)!, { key: "Home" });
    expect(placed(store)[0]!.startS).toBe(0);
    const end = document.querySelector<HTMLElement>(`[data-bar-end="${id}"]`)!;
    fireEvent.keyDown(end, { key: "ArrowLeft", shiftKey: true });
    expect(placed(store)[0]!.durationS).not.toBeNull();
    const length = placed(store)[0]!.durationS!;
    fireEvent.keyDown(document.querySelector(`[data-bar-end="${id}"]`)!, { key: "ArrowRight", altKey: true });
    expect(placed(store)[0]!.durationS).toBeCloseTo(length + 0.01, 6);
    fireEvent.keyDown(document.querySelector(`[data-bar-end="${id}"]`)!, { key: "End" });
    expect(placed(store)[0]!.durationS).toBeNull();
    await waitFor(() => expect(document.querySelector("[aria-live=polite]")!.textContent).toMatch(/plays until the end of the timeline/u));
  });

  it("has one transport for the whole timeline and a length that follows the longest window unless set", async () => {
    const { store } = await setup();
    await place();
    const timeline = within(screen.getByRole("region", { name: "Timeline" }));
    expect(timeline.getByRole("group", { name: "Timeline playback" })).toBeTruthy();
    expect(timeline.getByRole("button", { name: "Play" })).toBeTruthy();
    fireEvent.click(timeline.getByRole("checkbox", { name: "Length follows the longest window" }));
    expect(store.getSnapshot().playground.timelineS).not.toBeNull();
    fireEvent.click(timeline.getByRole("checkbox", { name: "Length follows the longest window" }));
    expect(store.getSnapshot().playground.timelineS).toBeNull();
  });
});

describe("🔄️ a catalog that changes under the page", () => {
  it("offers the clips of the current revision and names a pick the new revision no longer has", async () => {
    const { store, source } = await setup();
    await place();
    store.dispatch({ type: "placement-changed", id: placed(store)[0]!.id, patch: { pick: { kind: "clip", clip: "bag-swing" } } });
    fireEvent.focus(document.querySelector<HTMLElement>(`[data-placement="${placed(store)[0]!.id}"]`)!);
    const panel = await waitFor(() => {
      const found = document.querySelector<HTMLElement>("[data-asset-panel]");
      expect(found).not.toBeNull();
      return within(found!);
    });
    expect(panel.getByRole("combobox", { name: "Pose or clip" }).textContent).toContain("Clip: ");
    expect(panel.getAllByRole("option").some((option) => option.getAttribute("value") === "clip:bag-swing")).toBe(true);
    current = { ...sampleFigure, clips: sampleFigure.clips.filter((clip) => clip.id !== "bag-swing") };
    await source.retryCategory("humans");
    await waitFor(() => expect(within(document.querySelector<HTMLElement>("[data-asset-panel]")!).getAllByRole("option").some((option) => option.getAttribute("value") === "clip:bag-swing")).toBe(false));
    expect(document.querySelector("[data-asset-panel]")!.textContent).toContain('no longer has "bag-swing"');
    expect(placed(store)[0]!.pick).toEqual({ kind: "clip", clip: "bag-swing" });
  });
});

describe("🧊️ the 3D playground", () => {
  it("prepares the placed figures into one scene with progress and shows it", async () => {
    const { store } = await setup("#/playground/3d");
    await place();
    await place();
    store.dispatch({ type: "playground-configured", patch: { camera: "plan" } });
    const stage = await waitFor(() => {
      const found = document.querySelector<HTMLElement>("[data-model-playground]");
      expect(found).not.toBeNull();
      return found!;
    }, { timeout: 30000 });
    await waitFor(() => expect(stage.getAttribute("data-state")).toBe("ready"), { timeout: 120000 });
    expect(stage.getAttribute("data-parts")).toBe("2");
    expect(stage.querySelector("canvas")).not.toBeNull();
    expect(within(stage).getAllByRole("button").filter((button) => button.hasAttribute("data-preset")).map((button) => button.getAttribute("data-preset"))).toEqual(["elevation", "front", "plan", "axonometric", "perspective"]);
    expect(within(stage).getByRole("button", { name: "Plan (top view)" }).getAttribute("aria-pressed")).toBe("true");
  }, 180000);

  it("names the assets that have no model at the chosen scale instead of drawing another scale", async () => {
    const { store } = await setup("#/playground/3d");
    await place();
    store.dispatch({ type: "playground-configured", patch: { scale: 75 } });
    const stage = await waitFor(() => {
      const found = document.querySelector<HTMLElement>("[data-model-playground]");
      expect(found).not.toBeNull();
      return found!;
    }, { timeout: 30000 });
    await waitFor(() => expect(stage.textContent).toContain("No 3D model at 1:75: 1"));
    expect(stage.getAttribute("data-parts")).toBe("0");
  });

  it("can be cancelled while it prepares and tried again", async () => {
    current = { ...sampleFigure };
    const { store } = await setup("#/playground/3d");
    await place();
    const stage = await waitFor(() => {
      const found = document.querySelector<HTMLElement>("[data-model-playground]");
      expect(found).not.toBeNull();
      return found!;
    }, { timeout: 30000 });
    const cancel = await within(stage).findByRole("button", { name: "Cancel" }, { timeout: 30000 });
    fireEvent.click(cancel);
    await waitFor(() => expect(stage.getAttribute("data-state")).toBe("cancelled"));
    expect(placed(store)).toHaveLength(1);
    fireEvent.click(within(stage).getByRole("button", { name: "Try again" }));
    await waitFor(() => expect(stage.getAttribute("data-state")).toBe("ready"), { timeout: 120000 });
  }, 180000);
});

describe("🌍️ both languages", () => {
  it("names the tabs, the timeline and the controls in German", async () => {
    await setup("#/playground", "de");
    expect(within(screen.getByRole("navigation", { name: "Spielplätze" })).getAllByRole("link").map((link) => link.textContent)).toEqual(["Schnitt", "Grundriss", "3D"]);
    expect(screen.getByRole("region", { name: "Zeitleiste" })).toBeTruthy();
  });
});

describe("🧭️ yaw and mirror in the floor plan", () => {
  it("turns a figure about its ground point, positive yaw to the left, and flips a mirrored one across its forward axis", async () => {
    const { store } = await setup("#/playground/plan");
    await place();
    const id = placed(store)[0]!.id;
    const turn = () => document.querySelector<HTMLElement>("[data-plan-turn]")!;
    expect(turn().style.transform).toBe("rotate(0deg)");
    act(() => store.dispatch({ type: "placement-changed", id, patch: { yawDeg: 90, mirrored: true } }));
    await waitFor(() => expect(turn().getAttribute("data-yaw-deg")).toBe("90"));
    expect(turn().style.transform).toBe("rotate(-90deg) scaleY(-1)");
    expect(turn().getAttribute("data-flipped")).toBe("true");
    expect(screen.queryByText(/mirrored placements are drawn unmirrored here/u)).toBeNull();
  });
});
