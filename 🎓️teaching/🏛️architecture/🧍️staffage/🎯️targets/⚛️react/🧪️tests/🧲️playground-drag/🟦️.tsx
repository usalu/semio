/** 🧲️ The playground page moves, duplicates, removes and undoes placements by keyboard, persists them, and offers the asset list of the loaded catalog.
 * The evaluator's drawing is replaced by a fixed scene so the case tests interaction only; real drawing is covered by the figure cases.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { VIEWS } from "@teaching/staffage";
import type { CategoryDocument, Manifest } from "@teaching/staffage";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { PlaygroundPage } from "../../🔨️modules/🧪️playground/🟦️.tsx";
import { keyAction } from "../../🔨️modules/🧪️playground/🧲️keys/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset } from "@teaching/staffage";

const walker: Asset = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../../../🧫️fixtures/🚶️reference-walker/🔣️.json"), "utf8"));

vi.mock("../../🔨️modules/🧮️pipeline/🟦️.ts", async (original) => {
  const real = await original<typeof import("../../🔨️modules/🧮️pipeline/🟦️.ts")>();
  const scene = { widthMm: 20, heightMm: 40, background: null, layers: [], legends: [], metadata: { asset: "reference-walker", title: { en: "Reference walker", de: "Referenz-Fußgänger" }, query: { kind: "pose", pose: "stand" }, timeS: 0, view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, paperUnits: "mm", modelUnits: "m", mmPerMetre: 10, origin: { xMm: 10, yMm: 36 } } };
  return { ...real, frameOf: () => ({ scene }), inlineSvg: () => "<svg></svg>" };
});

vi.mock("@semio-tech/ui-react/chrome", async (original) => ({ ...(await original<Record<string, unknown>>()), Icon: () => null }));

afterEach(cleanup);

const manifest = { $schema: "", version: "semio.staffage/v1", categories: [{ id: "humans", emoji: "", title: { en: "Humans", de: "Menschen" }, description: { en: "h", de: "h" }, file: "h/🔣️.json" }], collections: [], entries: [{ id: walker.id, category: "humans", title: walker.title, heightM: walker.dimensions.heightM, widthM: walker.dimensions.widthM, poses: [], clips: [], analytics: ["thermal", "structural"], viewpoints: ["elevation"], exports: [], distinction: walker.summary, completion: "authored" }] } as unknown as Manifest;
const catalog = () => new CatalogStore({ manifest: async () => manifest, category: async () => ({ assets: [walker] }) as unknown as CategoryDocument });

async function setup() {
  const source = catalog();
  await source.loadManifest();
  const store = new ArchiveStore(memoryStorage(), VIEWS);
  store.dispatch({ type: "preferences-changed", patch: { motion: "reduced", language: "en" } });
  render(
    <ArchiveProvider language="en" t={translator("en")} catalog={source} store={store}>
      <PlaygroundPage view={{ opened: true, revealed: false }} />
    </ArchiveProvider>,
  );
  return store;
}

const placed = (store: ArchiveStore) => store.getSnapshot().playground.placements;

describe("🧲️ keyboard model", () => {
  it("maps keys to actions with the stated step sizes", () => {
    const base = { shiftKey: false, altKey: false };
    expect(keyAction({ key: "ArrowRight", ...base })).toEqual({ kind: "move", deltaM: 0.1 });
    expect(keyAction({ key: "ArrowLeft", ...base, shiftKey: true })).toEqual({ kind: "move", deltaM: -1 });
    expect(keyAction({ key: "ArrowRight", ...base, altKey: true })).toEqual({ kind: "move", deltaM: 0.01 });
    expect(keyAction({ key: "Delete", ...base })?.kind).toBe("remove");
    expect(keyAction({ key: "d", ...base })?.kind).toBe("duplicate");
    expect(keyAction({ key: "]", ...base })).toEqual({ kind: "select", step: 1 });
    expect(keyAction({ key: "x", ...base })).toBeNull();
  });
});

describe("🧲️ playground page", () => {
  it("shows the empty state, places an asset from the list, moves, duplicates, removes and undoes by keyboard", async () => {
    const store = await setup();
    expect(screen.getByText("Nothing placed yet")).toBeTruthy();
    fireEvent.click(await screen.findByRole("button", { name: /Place Reference walker on the ground line/u }));
    expect(placed(store)).toHaveLength(1);
    const start = placed(store)[0]!.xM;
    const figure = () => document.querySelector<HTMLElement>("[data-placement]")!;
    fireEvent.keyDown(figure(), { key: "ArrowRight" });
    expect(placed(store)[0]!.xM).toBeCloseTo(start + 0.1, 9);
    fireEvent.keyDown(figure(), { key: "ArrowRight", shiftKey: true });
    expect(placed(store)[0]!.xM).toBeCloseTo(start + 1.1, 9);
    fireEvent.keyDown(figure(), { key: "ArrowLeft", altKey: true });
    expect(placed(store)[0]!.xM).toBeCloseTo(start + 1.09, 9);
    fireEvent.keyDown(figure(), { key: "Home" });
    expect(placed(store)[0]!.xM).toBe(0);
    fireEvent.keyDown(figure(), { key: "d" });
    expect(placed(store)).toHaveLength(2);
    fireEvent.keyDown(document.querySelectorAll<HTMLElement>("[data-placement]")[1]!, { key: "Delete" });
    expect(placed(store)).toHaveLength(1);
    fireEvent.click(screen.getByRole("button", { name: "Undo" }));
    expect(placed(store)).toHaveLength(2);
    fireEvent.click(screen.getByRole("button", { name: "Redo" }));
    expect(placed(store)).toHaveLength(1);
  });

  it("announces actions politely and keeps the composition after a remount", async () => {
    const store = await setup();
    fireEvent.click(await screen.findByRole("button", { name: /Place Reference walker/u }));
    await waitFor(() => expect(document.querySelector("[aria-live=polite]")!.textContent).toMatch(/Placed Reference walker/u));
    cleanup();
    const source = catalog();
    await source.loadManifest();
    render(
      <ArchiveProvider language="de" t={translator("de")} catalog={source} store={store}>
        <PlaygroundPage view={{ opened: true, revealed: false }} />
      </ArchiveProvider>,
    );
    expect(within(document.body).getAllByText(/Standlinie/u).length).toBeGreaterThan(0);
    expect(placed(store)).toHaveLength(1);
  });
});
