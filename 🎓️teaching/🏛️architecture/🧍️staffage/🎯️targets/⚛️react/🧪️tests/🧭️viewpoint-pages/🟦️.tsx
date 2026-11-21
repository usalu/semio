/** 🧭️ The compare page, the playground and the export panel carry the viewpoint: compare panes and their address, the playground selector with sideways position and yaw, depth order in the front elevation, and the embed snippet.
 * @see ./🥒️.feature — the scenarios */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { VIEWS, VIEWPOINT_CAPTIONS } from "@teaching/staffage";
import type { Manifest } from "@teaching/staffage";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { ComparePage } from "../../🔨️modules/⚖️compare/🟦️.tsx";
import { CategoryPage } from "../../🔨️modules/🗃️browse/🟦️.tsx";
import { ExportPanel } from "../../🔨️modules/🔬️detail/📤️export-panel/🟦️.tsx";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { CatalogStore, type CategoryDocument } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { PlaygroundPage } from "../../🔨️modules/🧪️playground/🟦️.tsx";
import { linePosition } from "../../🔨️modules/🧪️playground/🧱️compose/🟦️.ts";
import { keyAction } from "../../🔨️modules/🧪️playground/🧲️keys/🟦️.ts";
import { playgroundFromParams, playgroundToParams } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { VIEWPOINT_NAMES } from "../../🔨️modules/🔭️viewpoints/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

const manifest = {
  $schema: "",
  version: "semio.staffage/v2",
  categories: [{ id: "humans", emoji: "", title: { en: "Humans", de: "Menschen" }, description: { en: "h", de: "h" }, folder: "humans" }],
  collections: [],
  entries: [{ id: walker.id, category: "humans", title: walker.title, heightM: walker.dimensions.heightM, widthM: walker.dimensions.widthM, powerW: 0, poses: [], clips: [], representations: { "500": "unavailable", "200": "authored", "100": "authored", "50": "authored" }, symbols: [{ id: "position-dot", semantics: "true-position" }, { id: "figure-block", semantics: "true-extent" }], analytics: ["thermal", "structural"], exports: [], distinction: walker.summary, completion: "authored" }],
} as unknown as Manifest;
const source = () => new CatalogStore({ manifest: async () => manifest, category: async () => ({ assets: [walker], revisions: {} }) satisfies CategoryDocument });

async function open(page: "compare" | "playground", hash = ""): Promise<ArchiveStore> {
  window.location.hash = `#/${page}${hash}`;
  const catalog = source();
  await catalog.loadManifest();
  const store = new ArchiveStore(memoryStorage(), VIEWS);
  store.dispatch({ type: "preferences-changed", patch: { motion: "reduced", language: "en" } });
  render(
    <ArchiveProvider language="en" t={translator("en")} catalog={catalog} store={store}>
      {page === "compare" ? <ComparePage view={{ opened: true, revealed: false }} /> : <PlaygroundPage view={{ opened: true, revealed: false }} />}
    </ArchiveProvider>,
  );
  return store;
}

beforeEach(() => {
  window.location.hash = "";
});
afterEach(() => {
  cleanup();
  window.location.hash = "";
});

describe("⚖️ compare page", () => {
  it("keeps the two closed comparison drawings without the transport or an editor", async () => {
    const catalog = source();
    await catalog.loadManifest();
    await catalog.loadCategory("humans");
    const store = new ArchiveStore(memoryStorage(), VIEWS);
    const { container } = render(
      <ArchiveProvider language="en" t={translator("en")} catalog={catalog} store={store}>
        <ComparePage view={{ opened: false, revealed: false }} />
      </ArchiveProvider>,
    );
    expect(container.querySelectorAll("[data-compare-figure] [data-staffage-figure]")).toHaveLength(2);
    expect(container.querySelector("input")).toBeNull();
  });

  it("changes the viewpoint of a pane and writes it into the address", async () => {
    await open("compare");
    const panes = await screen.findAllByRole("region", { hidden: true }).catch(() => []);
    void panes;
    const selects = await screen.findAllByLabelText("Viewpoint");
    expect(selects).toHaveLength(2);
    fireEvent.change(selects[1]!, { target: { value: "plan" } });
    await waitFor(() => expect(decodeURIComponent(window.location.hash)).toContain("~~plan"));
    const figures = [...document.querySelectorAll("[data-compare-figure] [data-staffage-figure]")].map((figure) => figure.getAttribute("data-viewpoint"));
    expect(figures).toEqual(["elevation", "plan"]);
  });

  it("shows the same asset in three viewpoints on one clock from one button", async () => {
    await open("compare");
    fireEvent.click(await screen.findByRole("button", { name: ENGLISH["compare.threeViews"] }));
    await waitFor(() => expect(document.querySelectorAll("[data-compare-figure] [data-staffage-figure]").length).toBe(3));
    expect([...document.querySelectorAll("[data-compare-figure] [data-staffage-figure]")].map((figure) => figure.getAttribute("data-viewpoint"))).toEqual(["elevation", "front", "plan"]);
    expect(new Set([...document.querySelectorAll("[data-compare-figure] [data-staffage-figure]")].map((figure) => figure.getAttribute("data-time"))).size).toBe(1);
    expect(decodeURIComponent(window.location.hash)).toMatch(/~~front.*~~plan/u);
  });

  it("reports a wrong viewpoint in the link in words and still draws", async () => {
    await open("compare", "?p=reference-walker~dark-line~~sideways&p=reference-walker~thermal");
    expect(await screen.findByText(/unknown viewpoint "sideways"/u)).toBeTruthy();
    expect(document.querySelectorAll("[data-compare-figure] [data-staffage-figure]").length).toBe(2);
  });
});

describe("🗂️ category page", () => {
  it("leaves the closed category load to the overview card and loads on reveal", async () => {
    const catalog = source();
    await catalog.loadManifest();
    const store = new ArchiveStore(memoryStorage(), VIEWS);
    const page = (revealed: boolean) => (
      <ArchiveProvider language="en" t={translator("en")} catalog={catalog} store={store}>
        <CategoryPage category="humans" view={{ opened: false, revealed }} />
      </ArchiveProvider>
    );
    const { rerender } = render(page(false));
    await act(async () => undefined);
    expect(catalog.getSnapshot().categories.humans.state).toBe("idle");
    rerender(page(true));
    await waitFor(() => expect(catalog.getSnapshot().categories.humans.state).toBe("ready"));
  });
});

describe("🧪️ playground selector, position and yaw", () => {
  it("is one radio group of the two section viewpoints and stores the choice", async () => {
    const store = await open("playground");
    const group = await screen.findByRole("radiogroup", { name: "Viewpoint" });
    expect(within(group).getAllByRole("radio").map((radio) => radio.getAttribute("aria-label"))).toEqual(["Side elevation", "Front elevation"]);
    fireEvent.click(within(group).getByRole("radio", { name: "Front elevation" }));
    expect(store.getSnapshot().playground.viewpoint).toBe("front");
    expect(playgroundToParams(store.getSnapshot().playground).get("vp")).toBe("front");
  });

  it("sets sideways position and yaw by field and key, keeps them in the address and says yaw is drawn in 3D only", async () => {
    const store = await open("playground");
    store.dispatch({ type: "placement-added", placement: { id: "w", asset: walker.id, xM: 2, zM: 0, yawDeg: 0, pick: null, offsetS: 0, mirrored: false, startS: 0, durationS: null, speed: 1, loop: true, holdS: null } });
    const hit = await waitFor(() => {
      const node = document.querySelector<HTMLElement>('[data-placement="w"]');
      expect(node).not.toBeNull();
      return node!;
    });
    fireEvent.click(hit);
    fireEvent.keyDown(hit, { key: "ArrowUp" });
    expect(store.getSnapshot().playground.placements[0]!.zM).toBeCloseTo(0.1, 9);
    fireEvent.keyDown(document.querySelector<HTMLElement>('[data-placement="w"]')!, { key: "PageUp" });
    expect(store.getSnapshot().playground.placements[0]!.yawDeg).toBe(5);
    const z = await screen.findByLabelText(ENGLISH["playground.z"], { exact: false });
    fireEvent.change(z, { target: { value: "-0.5" } });
    expect(store.getSnapshot().playground.placements[0]!.zM).toBe(-0.5);
    expect(document.querySelector("[data-yaw-note]")!.textContent).toMatch(/yaw of 5°/u);
    const params = playgroundToParams(store.getSnapshot().playground);
    expect(decodeURIComponent(params.get("p")!)).toBe(`${walker.id}~2~~~~-0.5~5`);
    expect(playgroundFromParams(new URLSearchParams(params.toString()))!.placements[0]).toMatchObject({ xM: 2, zM: -0.5, yawDeg: 5 });
  });

  it("orders figures in the front elevation by depth along the line and keeps them at their sideways position", async () => {
    const store = await open("playground");
    store.dispatch({ type: "playground-configured", patch: { viewpoint: "front" } });
    store.dispatch({ type: "placement-added", placement: { id: "far", asset: walker.id, xM: 2, zM: 0.2, yawDeg: 0, pick: null, offsetS: 0, mirrored: false, startS: 0, durationS: null, speed: 1, loop: true, holdS: null } });
    store.dispatch({ type: "placement-added", placement: { id: "near", asset: walker.id, xM: 6, zM: 0.2, yawDeg: 0, pick: null, offsetS: 0, mirrored: false, startS: 0, durationS: null, speed: 1, loop: true, holdS: null } });
    await waitFor(() => expect(document.querySelectorAll("[data-near]").length).toBe(2));
    const order = (id: string): number => Number(document.querySelector(`[data-placement="${id}"]`)!.getAttribute("data-z-m") === "0.2" ? document.querySelector<HTMLElement>(`[data-placement="${id}"]`)!.style.zIndex : -1);
    expect(order("near")).toBeGreaterThan(order("far"));
    const placement = (xM: number, zM: number) => linePosition("front", { xM, zM }, { lengthM: 24, depthM: 12 });
    expect(placement(2, 0.2).horizontalM).toBe(placement(6, 0.2).horizontalM);
    expect(placement(6, 0.2).nearness).toBeGreaterThan(placement(2, 0.2).nearness);
  });

  it("projects one position by the camera: elevation keeps x, front keeps z, plan lifts by z", () => {
    const ground = { lengthM: 24, depthM: 12 };
    expect(linePosition("elevation", { xM: 3, zM: 1 }, ground)).toEqual({ horizontalM: 3, liftM: 0, nearness: -1 });
    expect(linePosition("front", { xM: 3, zM: 1 }, ground)).toEqual({ horizontalM: 7, liftM: 0, nearness: 3 });
    expect(linePosition("plan", { xM: 3, zM: 1 }, ground)).toEqual({ horizontalM: 3, liftM: 1, nearness: 0 });
  });

  it("maps Up, Down, Page Up and Page Down to sideways steps and turns", () => {
    const key = (name: string, extra: object = {}) => keyAction({ key: name, shiftKey: false, altKey: false, ...extra });
    expect(key("ArrowUp")).toEqual({ kind: "lateral", deltaM: 0.1 });
    expect(key("ArrowDown", { shiftKey: true })).toEqual({ kind: "lateral", deltaM: -1 });
    expect(key("PageUp")).toEqual({ kind: "yaw", deltaDeg: 5 });
    expect(key("PageDown", { shiftKey: true })).toEqual({ kind: "yaw", deltaDeg: -45 });
  });
});

describe("📤️ export snippet and vocabulary", () => {
  const capabilities = { svg: { available: true }, pdf: { available: true }, dxf: { available: true }, dwg: { available: false }, "png-sequence": { available: false }, mp4: { available: false }, webm: { available: false }, glb: { available: true }, fbx: { available: true }, model: { available: true }, "model-zip": { available: true } } as never;
  it("puts representation, viewpoint and scale in the embed snippet and the lede", async () => {
    const store = new ArchiveStore(memoryStorage(), VIEWS);
    const catalog = source();
    render(
      <ArchiveProvider language="en" t={translator("en")} catalog={catalog} store={store}>
        <ExportPanel asset={walker} query={{ kind: "pose", pose: walker.presentation.defaultPose }} request={{ view: "thermal", viewpoint: "plan", scale: { denominator: 50 }, language: "en" }} language="en" capabilities={capabilities} />
      </ArchiveProvider>,
    );
    expect(document.body.textContent).toContain('<staffage-figure asset="reference-walker" view="thermal" viewpoint="plan" scale="50" lang="en">');
    const lede = document.querySelector("[data-section=export] p")!.textContent!;
    expect(lede).toContain("thermal view representation");
    expect(lede).toContain("plan (top view) viewpoint");
    expect(lede).toContain("1:50");
    expect(lede).not.toMatch(/\{\w+\}/u);
    await act(async () => undefined);
  });

  it("reads one vocabulary: the app bundle names are the library captions", () => {
    for (const viewpoint of ["elevation", "front", "plan"] as const) {
      expect(VIEWPOINT_NAMES[viewpoint]).toEqual(VIEWPOINT_CAPTIONS[viewpoint]);
    }
    expect(ENGLISH["viewer.viewpoint.elevation"]).toBe(VIEWPOINT_CAPTIONS.elevation.en);
    expect(ENGLISH["viewer.viewpoint.front"]).toBe(VIEWPOINT_CAPTIONS.front.en);
    expect(ENGLISH["viewer.viewpoint.plan"]).toBe(VIEWPOINT_CAPTIONS.plan.en);
    expect(GERMAN["viewer.viewpoint.elevation"]).toBe(VIEWPOINT_CAPTIONS.elevation.de);
    expect(GERMAN["viewer.viewpoint.front"]).toBe(VIEWPOINT_CAPTIONS.front.de);
    expect(GERMAN["viewer.viewpoint.plan"]).toBe(VIEWPOINT_CAPTIONS.plan.de);
  });
});
