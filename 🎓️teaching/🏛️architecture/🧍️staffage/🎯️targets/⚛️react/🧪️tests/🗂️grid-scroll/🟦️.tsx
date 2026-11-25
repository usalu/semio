/** 🗂️ Desktop and tablet keep the card grid and scroll it; a phone uses a list.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🏠️overview/🟦️.tsx — the root grid
 * @see ../../🔨️modules/🗃️browse/🟦️.tsx — a category grid
 */
import { act, cleanup, fireEvent, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { CATEGORIES, type Asset, type Issue, type JobControl, type Manifest, type ManifestEntry } from "@teaching/staffage";
import { mountArchive } from "@teaching/staffage-react/app";
import { STORAGE_KEY, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { APP_CONFIG } from "../../🔨️modules/⚙️config/🟦️.ts";
import type { AssetCheck, CatalogSource } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";
import { tracks } from "../../🔨️modules/🥞️levels/🟦️.tsx";
import { ROOT_PAGES } from "../../🔨️modules/🏠️overview/🟦️.tsx";

const text = (value: string) => ({ en: value, de: value });
const human = (id: string): ManifestEntry => ({ id, category: "humans", title: text(id), heightM: 1.7, widthM: 0.5, powerW: 0, poses: [], clips: [], representations: { "500": "unavailable", "200": "unavailable", "100": "authored", "50": "unavailable" }, symbols: [], analytics: [], exports: ["svg"], distinction: text(id), completion: "planned" });
const manifest = { $schema: "", version: "semio.staffage/v2", categories: CATEGORIES.map((id) => ({ id, emoji: "x", title: text(id), description: text(id), folder: id })), collections: [], entries: Array.from({ length: 24 }, (_, index) => human(`human-${index + 1}`)) } as unknown as Manifest;

const view = { width: 1280, height: 400 };
const originalMatch = window.matchMedia;
const originalObserver = globalThis.ResizeObserver;

function installViewport(width: number, height: number, coarse = false): void {
  view.width = width;
  view.height = height;
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  Object.defineProperty(window, "innerHeight", { configurable: true, value: height });
  window.matchMedia = (query: string): MediaQueryList => {
    const max = /max-width:\s*(\d+)px/u.exec(query);
    const min = /min-width:\s*(\d+)px/u.exec(query);
    const maxHeight = /max-height:\s*(\d+)px/u.exec(query);
    const matches = (!query.includes("pointer: coarse") || coarse) && (maxHeight === null || height <= Number(maxHeight[1])) && !query.includes("prefers-") && (max === null || width <= Number(max[1])) && (min === null || width >= Number(min[1]));
    return { matches, media: query, onchange: null, addListener: () => undefined, removeListener: () => undefined, addEventListener: () => undefined, removeEventListener: () => undefined, dispatchEvent: () => false };
  };
  globalThis.ResizeObserver = class {
    readonly #callback: ResizeObserverCallback;
    #delivering = false;
    constructor(callback: ResizeObserverCallback) {
      this.#callback = callback;
    }
    observe(target: Element): void {
      if (this.#delivering) return;
      const box = { x: 0, y: 0, width: view.width, height: view.height, top: 0, left: 0, bottom: view.height, right: view.width, toJSON: () => ({}) };
      this.#delivering = true;
      try {
        this.#callback([{ target, contentRect: box } as ResizeObserverEntry], this);
      } finally {
        this.#delivering = false;
      }
    }
    unobserve(): void {}
    disconnect(): void {}
  } as unknown as typeof ResizeObserver;
}

let unmount: (() => void) | undefined;

function mount(catalog: CatalogSource = { manifest: async () => manifest, category: async () => null }, hash = "#/", language: "en" | "de" = "en", check?: AssetCheck): HTMLElement {
  const storage = memoryStorage();
  storage.setItem(STORAGE_KEY, JSON.stringify({ version: 1, state: { preferences: { language } } }));
  const root = document.createElement("div");
  document.body.append(root);
  window.history.replaceState(null, "", hash);
  unmount = mountArchive(root, { storage, languages: [language], catalog, ...(check === undefined ? {} : { check }) });
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
    act(() => unmount?.());
    unmount = undefined;
    cleanup();
    document.body.innerHTML = "";
    window.location.hash = "#/";
    window.matchMedia = originalMatch;
    globalThis.ResizeObserver = originalObserver;
    vi.unstubAllGlobals();
  });

  it("lets each featured category finish its first load when another finishes", async () => {
    installViewport(1280, 400);
    const archive = {
      ...manifest,
      entries: [human("featured-human"), { ...human("featured-animal"), category: "animals" }],
      collections: [{ id: APP_CONFIG.featuredCollection, title: text("Featured"), description: text("Featured figures"), assets: ["featured-human", "featured-animal"] }],
    } as unknown as Manifest;
    let finishHuman!: () => void;
    let finishAnimal!: () => void;
    const humanLoad = new Promise<void>(resolve => { finishHuman = resolve; });
    const animalLoad = new Promise<void>(resolve => { finishAnimal = resolve; });
    const category = vi.fn<CatalogSource["category"]>(async ({ id }) => {
      await (id === "humans" ? humanLoad : animalLoad);
      return null;
    });
    mount({ manifest: async () => archive, category });
    await waitFor(() => expect(category.mock.calls.map(([entry]) => entry.id)).toEqual(["humans", "animals"]));
    const animalSignal = category.mock.calls[1]![1];
    try {
      await act(async () => finishHuman());
      expect(animalSignal?.aborted).toBe(false);
      expect(category).toHaveBeenCalledTimes(2);
    } finally {
      await act(async () => finishAnimal());
    }
  });

  it("mounts intersecting cards behind the glass and every card on opening", async () => {
    installViewport(1280, 400);
    vi.stubGlobal("IntersectionObserver", class {
      constructor(readonly callback: IntersectionObserverCallback) {}
      observe(target: Element) {
        const visible = !target.hasAttribute("data-preview-asset") || target.matches(":nth-child(-n+2)");
        this.callback([{ target, isIntersecting: visible } as IntersectionObserverEntry], this as unknown as IntersectionObserver);
      }
      unobserve() {}
      disconnect() {}
    });
    const root = mount();
    const cells = () => [...root.querySelectorAll("[data-preview-asset]")];
    await waitFor(() => expect(cells().length).toBe(24), { timeout: 5000 });
    await waitFor(() => expect(cells().filter(cell => cell.querySelector("[data-overview-card]"))).toHaveLength(2));
    window.location.hash = "#/all";
    await waitFor(() => expect(cells().filter(cell => cell.querySelector("[data-overview-card]"))).toHaveLength(24));
  });

  it.each([[1280, 400], [1280, 640], [900, 400]] as const)("keeps the root and 24 human cards on the strip at %sx%s", async (width, height) => {
    installViewport(width, height);
    const root = mount();
    await waitFor(() => expect(root.querySelector("[data-layered-overview]")?.getAttribute("data-mode")).toBe("strip"));
    const overview = root.querySelector<HTMLElement>(".staffage-grid");
    expect(overview).not.toBeNull();
    expect(overview!.querySelectorAll(".staffage-cell").length).toBe(ROOT_PAGES.length);
    expect(overview!.style.getPropertyValue("--staffage-rows")).toContain("minmax(150px");
    window.location.hash = "#/humans";
    await waitFor(() => expect(root.querySelector('[data-page="category:humans"] [data-layered-overview]')?.getAttribute("data-mode")).toBe("strip"));
    const category = root.querySelector<HTMLElement>('[data-page="category:humans"] .staffage-grid');
    expect(category).not.toBeNull();
    expect(category!.querySelectorAll(".staffage-cell").length).toBe(24);
    expect(category!.querySelector(".staffage-entry")).toBeNull();
    expect(category!.style.getPropertyValue("--staffage-rows")).toContain("minmax(190px");
  });

  it("opens all 24 human links in a lightweight phone list and loads only the selected asset", async () => {
    installViewport(390, 800);
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    const asset = vi.fn(async () => null);
    const root = mount({ manifest: async () => manifest, category, asset });
    const overview = await waitFor(() => {
      const nav = within(root).getByRole("navigation", { name: "Archive overview" });
      expect(within(nav).getAllByRole("link")).toHaveLength(ROOT_PAGES.length);
      return nav;
    });
    expect(category).not.toHaveBeenCalled();
    fireEvent.click(within(overview).getByRole("link", { name: "humans" }));
    const grid = await waitFor(() => {
      const list = root.querySelector<HTMLElement>("[data-staffage-asset-list]");
      expect(list).not.toBeNull();
      return list!;
    });
    expect(within(grid).getAllByRole("link")).toHaveLength(24);
    expect(root.querySelector("[data-layered-overview]")).toBeNull();
    expect(within(root).getByRole("searchbox")).toBeDefined();
    expect(within(root).getByRole("button", { name: "Filters" }).getAttribute("aria-expanded")).toBe("false");
    expect(within(root).queryByRole("combobox", { name: "Sort by" })).toBeNull();
    expect(category).not.toHaveBeenCalled();

    const categoryPage = grid.closest<HTMLElement>("[data-page]")!;
    categoryPage.scrollTop = 720;
    fireEvent.scroll(categoryPage);
    fireEvent.click(within(grid).getByRole("link", { name: "human-24" }));
    await waitFor(() => expect(asset).toHaveBeenCalledWith(manifest.categories[0], "human-24", expect.any(AbortSignal)));
    expect(within(root).getByRole("heading", { name: "human-24" })).toBeDefined();
    expect(category).not.toHaveBeenCalled();
    expect(root.querySelector("[data-staffage-asset-list]")).toBeNull();
    fireEvent.click(within(root).getByRole("button", { name: "Up" }));
    await waitFor(() => expect(root.querySelector('[data-page="category:humans"]')?.scrollTop).toBe(720));
  });

  it("uses phone navigation in short landscape touch windows", async () => {
    installViewport(844, 390, true);
    const root = mount();
    await waitFor(() => expect(within(root).getByRole("navigation", { name: "Archive overview" })).toBeDefined());
    expect(root.querySelector("[data-layered-overview]")).toBeNull();
  });

  it("cancels and retries the selected asset without loading its category", async () => {
    installViewport(390, 800);
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    const asset = vi.fn<NonNullable<CatalogSource["asset"]>>(
      async (_entry, _id, signal) => new Promise(resolve => signal?.addEventListener("abort", () => resolve(null), { once: true })),
    );
    const root = mount({ manifest: async () => manifest, category, asset }, "#/humans/human-24");
    await waitFor(() => expect(asset).toHaveBeenCalledTimes(1));
    fireEvent.click(within(root).getByRole("button", { name: "Cancel" }));
    expect(asset.mock.calls[0]![2]?.aborted).toBe(true);
    fireEvent.click(await waitFor(() => within(root).getByRole("button", { name: "Try again" })));
    await waitFor(() => expect(asset).toHaveBeenCalledTimes(2));
    expect(asset.mock.calls[1]![2]?.aborted).toBe(false);
    expect(category).not.toHaveBeenCalled();
    window.location.hash = "#/";
    await waitFor(() => expect(asset.mock.calls[1]![2]?.aborted).toBe(true));
  });

  it("offers one retry for a failed phone asset request without loading its category", async () => {
    installViewport(390, 800);
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    let interrupt!: (error: Error) => void;
    const firstRequest = new Promise<null>((_resolve, reject) => { interrupt = reject; });
    const asset = vi.fn<NonNullable<CatalogSource["asset"]>>()
      .mockReturnValueOnce(firstRequest)
      .mockResolvedValueOnce(null);
    const root = mount({ manifest: async () => manifest, category, asset }, "#/humans/human-24");
    await waitFor(() => expect(asset).toHaveBeenCalledTimes(1));
    expect(asset.mock.calls[0]![2]?.aborted).toBe(false);
    await act(async () => { interrupt(new Error("Connection interrupted")); });
    const retry = await waitFor(() => within(root).getByRole("button", { name: "Try again" }));
    fireEvent.click(retry);
    await waitFor(() => expect(asset).toHaveBeenCalledTimes(2));
    expect(asset.mock.calls.map(([, id]) => id)).toEqual(["human-24", "human-24"]);
    expect(category).not.toHaveBeenCalled();
  });

  it("lists manifest assets on the phone before requesting data for search", async () => {
    installViewport(320, 640);
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    const root = mount({ manifest: async () => manifest, category });
    window.location.hash = "#/all";
    await waitFor(() => expect(root.querySelectorAll("[data-staffage-asset-list] li")).toHaveLength(24));
    expect(category).not.toHaveBeenCalled();
    fireEvent.change(within(root).getByRole("searchbox"), { target: { value: "human-24" } });
    await waitFor(() => expect(root.querySelectorAll("[data-staffage-asset-list] li")).toHaveLength(1));
    await waitFor(() => expect(category).toHaveBeenCalled());
  });

  it("asks on the overview for the assets its cards show and for no category document", async () => {
    installViewport(1280, 800);
    const authored = (id: string, category: ManifestEntry["category"]): ManifestEntry => ({ ...human(id), category, completion: "authored" });
    const archive = {
      ...manifest,
      entries: [...Array.from({ length: 7 }, (_, index) => authored(`human-${index + 1}`, "humans")), authored("animal-1", "animals"), human("human-planned")],
      collections: [{ id: APP_CONFIG.featuredCollection, title: text("Featured"), description: text("Featured figures"), assets: ["human-7"] }],
    } as unknown as Manifest;
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    const asset = vi.fn<NonNullable<CatalogSource["asset"]>>(async () => null);
    mount({ manifest: async () => archive, category, asset });
    const shown = new Set(["human-1", "human-2", "human-3", "human-4", "human-7", "animal-1"]);
    await waitFor(() => expect(new Set(asset.mock.calls.map(([, id]) => id))).toEqual(shown), { timeout: 400 + CATEGORIES.length * 350 + 3000 });
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 400 + CATEGORIES.length * 350 + 300)); });
    expect(new Set(asset.mock.calls.map(([, id]) => id))).toEqual(shown);
    expect(category).not.toHaveBeenCalled();
  }, 20_000);

  it("requests only the opened asset on a desktop, and its category once the reader goes up", async () => {
    installViewport(1280, 800);
    const category = vi.fn<CatalogSource["category"]>(async () => null);
    const asset = vi.fn<NonNullable<CatalogSource["asset"]>>(async () => null);
    const root = mount({ manifest: async () => manifest, category, asset }, "#/humans/human-24");
    await waitFor(() => expect(asset).toHaveBeenCalledWith(manifest.categories[0], "human-24", expect.any(AbortSignal)));
    await waitFor(() => expect(root.querySelector('[data-page="asset:human-24"]')).not.toBeNull());
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 400 + CATEGORIES.length * 350 + 300)); });
    expect(category).not.toHaveBeenCalled();
    expect(new Set(asset.mock.calls.map(([, id]) => id))).toEqual(new Set(["human-24"]));
    window.location.hash = "#/humans";
    await waitFor(() => expect(category.mock.calls.map(([entry]) => entry.id)).toContain("humans"));
  }, 20_000);

  it.each([["en", "Loading human-24…", "Planned, not yet authored"], ["de", "human-24 wird geladen …", "Geplant, noch nicht erstellt"]] as const)("says in %s that the asset is loading and calls it planned only once no document exists", async (language, loading, planned) => {
    installViewport(1280, 800);
    let finish!: (document: null) => void;
    const asset = vi.fn<NonNullable<CatalogSource["asset"]>>(() => new Promise((resolve) => { finish = resolve; }));
    const root = mount({ manifest: async () => manifest, category: async () => null, asset }, "#/humans/human-24", language);
    const page = await waitFor(() => {
      const found = root.querySelector<HTMLElement>('[data-page="asset:human-24"]');
      expect(found?.querySelector('[data-asset-state="loading"]') ?? null).not.toBeNull();
      return found!;
    });
    const notice = page.querySelector<HTMLElement>('[data-asset-state="loading"]')!;
    expect(notice.getAttribute("role")).toBe("status");
    expect(notice.textContent).toContain(loading);
    expect(page.textContent).not.toContain(planned);
    await act(async () => finish(null));
    await waitFor(() => expect(page.textContent).toContain(planned));
    expect(page.querySelector('[data-asset-state="loading"]')).toBeNull();
  });

  it("shows an authored asset while its findings are computed and keeps showing it beside the findings that arrive", async () => {
    installViewport(1280, 800);
    const authored = { ...walker, id: "human-24", category: "humans" } as Asset;
    const asset = vi.fn<NonNullable<CatalogSource["asset"]>>(async () => ({ asset: authored }));
    let hear!: NonNullable<JobControl["progress"]>;
    let answer!: (issues: readonly Issue[]) => void;
    const check: AssetCheck = (_asset, control) => new Promise((resolve) => {
      hear = control!.progress!;
      answer = resolve;
    });
    const root = mount({ manifest: async () => manifest, category: async () => null, asset }, "#/humans/human-24", "en", check);
    const page = await waitFor(() => {
      const found = root.querySelector<HTMLElement>('[data-page="asset:human-24"]');
      expect(found?.querySelector("[data-viewer]") ?? null).not.toBeNull();
      return found!;
    });
    const notice = page.querySelector<HTMLElement>('[data-asset-state="checking"]')!;
    expect(notice.getAttribute("role")).toBe("status");
    expect(notice.textContent).toContain("Checking this asset…");
    await act(async () => hear(2, 6, { en: "Drawings checked", de: "Zeichnungen geprüft" }));
    expect(page.querySelector('[data-asset-state="checking"]')!.textContent).toContain("Part 2 of 6: Drawings checked.");
    await act(async () => answer([{ path: "$.rig", code: "broken", message: { en: "Broken rig", de: "Defektes Rig" } }]));
    const findings = await waitFor(() => {
      const found = page.querySelector<HTMLElement>('[data-asset-state="findings"]');
      expect(found).not.toBeNull();
      return found!;
    });
    expect(findings.getAttribute("role")).toBe("alert");
    expect(findings.textContent).toContain("Findings of the check: 1");
    expect(findings.textContent).toContain("Broken rig");
    expect(page.querySelector("[data-viewer]")).not.toBeNull();
    expect(page.textContent).not.toContain("Planned, not yet authored");
    expect(page.querySelector('[data-asset-state="checking"]')).toBeNull();
  });
});
