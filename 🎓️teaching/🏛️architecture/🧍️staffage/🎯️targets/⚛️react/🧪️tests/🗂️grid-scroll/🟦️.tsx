/** 🗂️ Desktop and tablet keep the card grid and scroll it; only a phone swipes one card at a time.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🏠️overview/🟦️.tsx — the root grid
 * @see ../../🔨️modules/🗃️browse/🟦️.tsx — a category grid
 */
import { cleanup, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { CATEGORIES, type Manifest, type ManifestEntry } from "@teaching/staffage";
import { mountArchive } from "@teaching/staffage-react/app";
import { STORAGE_KEY, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { tracks } from "../../🔨️modules/🥞️levels/🟦️.tsx";

const text = (value: string) => ({ en: value, de: value });
const human = (id: string): ManifestEntry => ({ id, category: "humans", title: text(id), heightM: 1.7, widthM: 0.5, poses: [], clips: [], representations: { "500": "unavailable", "200": "unavailable", "100": "authored", "50": "unavailable" }, symbols: [], analytics: [], viewpoints: ["elevation"], exports: ["svg"], distinction: text(id), completion: "planned" });
const manifest = { $schema: "", version: "semio.staffage/v2", categories: CATEGORIES.map((id) => ({ id, emoji: "x", title: text(id), description: text(id), folder: id })), collections: [], entries: Array.from({ length: 24 }, (_, index) => human(`human-${index + 1}`)) } as unknown as Manifest;

const view = { width: 1280, height: 400 };
const originalMatch = window.matchMedia;
const originalObserver = globalThis.ResizeObserver;

function installViewport(width: number, height: number): void {
  view.width = width;
  view.height = height;
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  Object.defineProperty(window, "innerHeight", { configurable: true, value: height });
  window.matchMedia = (query: string): MediaQueryList => {
    const max = /max-width:\s*(\d+)px/u.exec(query);
    const min = /min-width:\s*(\d+)px/u.exec(query);
    const matches = !query.includes("prefers-") && (max === null || width <= Number(max[1])) && (min === null || width >= Number(min[1]));
    return { matches, media: query, onchange: null, addListener: () => undefined, removeListener: () => undefined, addEventListener: () => undefined, removeEventListener: () => undefined, dispatchEvent: () => false };
  };
  globalThis.ResizeObserver = class {
    readonly #callback: ResizeObserverCallback;
    constructor(callback: ResizeObserverCallback) {
      this.#callback = callback;
    }
    observe(target: Element): void {
      const box = { x: 0, y: 0, width: view.width, height: view.height, top: 0, left: 0, bottom: view.height, right: view.width, toJSON: () => ({}) };
      this.#callback([{ target, contentRect: box } as ResizeObserverEntry], this);
    }
    unobserve(): void {}
    disconnect(): void {}
  } as unknown as typeof ResizeObserver;
}

function mount(): HTMLElement {
  const storage = memoryStorage();
  storage.setItem(STORAGE_KEY, JSON.stringify({ version: 1, state: { preferences: { language: "en" } } }));
  const root = document.createElement("div");
  document.body.append(root);
  window.location.hash = "#/";
  mountArchive(root, { storage, languages: ["en"], catalog: { manifest: async () => manifest, category: async () => null } });
  return root;
}

describe("track floors", () => {
  it("keeps a zero floor when no minimum is given and writes the pixel floor otherwise", () => {
    expect(tracks([1, 2])).toBe("minmax(0, 1fr) minmax(0, 2fr)");
    expect(tracks([1.5, 1, 1, 0.9], 150)).toBe("minmax(150px, 1.5fr) minmax(150px, 1fr) minmax(150px, 1fr) minmax(150px, 0.9fr)");
    expect(tracks(Array.from({ length: 6 }, () => 1), 190)).toBe(Array.from({ length: 6 }, () => "minmax(190px, 1fr)").join(" "));
  });
});

describe("the mounted archive", () => {
  afterEach(() => {
    cleanup();
    document.body.innerHTML = "";
    window.location.hash = "#/";
    window.matchMedia = originalMatch;
    globalThis.ResizeObserver = originalObserver;
  });

  it.each([[1280, 400], [1280, 640], [900, 400]] as const)("keeps the root and 24 human cards on the strip at %sx%s", async (width, height) => {
    installViewport(width, height);
    const root = mount();
    await waitFor(() => expect(root.querySelector("[data-layered-overview]")?.getAttribute("data-mode")).toBe("strip"));
    const overview = root.querySelector<HTMLElement>(".staffage-grid");
    expect(overview).not.toBeNull();
    expect(overview!.querySelectorAll(".staffage-cell").length).toBe(12);
    expect(overview!.style.getPropertyValue("--staffage-rows")).toContain("minmax(150px");
    window.location.hash = "#/humans";
    await waitFor(() => expect(root.querySelector('[data-page="category:humans"] [data-layered-overview]')?.getAttribute("data-mode")).toBe("strip"));
    const category = root.querySelector<HTMLElement>('[data-page="category:humans"] .staffage-grid');
    expect(category).not.toBeNull();
    expect(category!.querySelectorAll(".staffage-cell").length).toBe(24);
    expect(category!.querySelector(".staffage-entry")).toBeNull();
    expect(category!.style.getPropertyValue("--staffage-rows")).toContain("minmax(190px");
  });

  it("swipes one card at a time on a phone", async () => {
    installViewport(390, 800);
    const root = mount();
    await waitFor(() => expect(root.querySelector("[data-layered-overview]")?.getAttribute("data-mode")).toBe("swipe"));
    expect(root.querySelector(".staffage-cell")).toBeNull();
    expect(root.querySelector(".staffage-entry")).not.toBeNull();
    window.location.hash = "#/humans";
    await waitFor(() => expect(root.querySelector('[data-page="category:humans"] [data-layered-overview]')?.getAttribute("data-mode")).toBe("swipe"));
    expect(root.querySelector('[data-page="category:humans"] .staffage-cell')).toBeNull();
    expect(root.querySelector('[data-page="category:humans"] .staffage-entry')).not.toBeNull();
  });
});
