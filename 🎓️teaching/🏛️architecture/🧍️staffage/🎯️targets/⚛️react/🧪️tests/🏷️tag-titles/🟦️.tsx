/** 🏷️ Tags read in the language of the page: facts, header and search take the title of a tag, never its slug.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, render, screen } from "@testing-library/react";
import type { ReactElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { Asset, Language, Tag } from "@teaching/staffage";
import walkerDocument from "../../../../🧫️fixtures/🚶️reference-walker/🔣️.json";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { DimensionsSection } from "../../🔨️modules/🔬️detail/📐️dimensions/🟦️.tsx";
import { AssetPage } from "../../🔨️modules/🔬️detail/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { NO_FILTERS, filterAndSort, type Row } from "../../🔨️modules/🔎️search/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";

const tag = (id: string, en: string, de: string): Tag => ({ id, title: { en, de } });
const walker: Asset = { ...(walkerDocument as unknown as Asset), tags: [tag("hi-vis", "hi-vis", "Warnkleidung"), tag("public-space", "public space", "öffentlicher Raum"), tag("commuter", "commuter", "Pendler")] };
const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
const row = { entry: { id: walker.id, category: "humans", title: walker.title, heightM: 1.73, widthM: 0.37, powerW: 0, poses: [], clips: [], representations: { "500": "unavailable", "200": "authored", "100": "authored", "50": "authored" }, symbols: [], analytics: [], exports: ["svg"], distinction: walker.title, completion: "authored" }, status: "ready", asset: walker } as unknown as Row;

function wrap(node: ReactElement, language: Language): ReactElement {
  return <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>{node}</ArchiveProvider>;
}

afterEach(cleanup);

describe("🏷️ tag titles", () => {
  it("shows the facts of an asset with the titles of the language of the page", () => {
    const { unmount } = render(wrap(<DimensionsSection asset={walker} row={row as never} />, "en"));
    for (const title of ["hi-vis", "public space", "commuter"]) expect(screen.getByText(title)).toBeTruthy();
    expect(screen.queryByText("Warnkleidung")).toBeNull();
    expect(screen.queryByText("public-space")).toBeNull();
    unmount();
    render(wrap(<DimensionsSection asset={walker} row={row as never} />, "de"));
    for (const title of ["Warnkleidung", "öffentlicher Raum", "Pendler"]) expect(screen.getByText(title)).toBeTruthy();
    expect(screen.queryByText("public space")).toBeNull();
    expect(screen.queryByText("hi-vis")).toBeNull();
  });

  it("shows the same titles beside the title of the page", () => {
    const url = window.location.href;
    const canvas = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    window.history.replaceState(null, "", `#/humans/${walker.id}?t=0`);
    try {
      render(wrap(<AssetPage row={row as never} view={{ opened: true, revealed: false }} />, "de"));
      for (const title of ["Warnkleidung", "öffentlicher Raum", "Pendler"]) expect(screen.getAllByText(title).length, title).toBeGreaterThan(0);
      expect(screen.queryAllByText("public space")).toHaveLength(0);
      expect(screen.queryAllByText("hi-vis")).toHaveLength(0);
    } finally {
      window.history.replaceState(null, "", url);
      canvas.mockRestore();
    }
  });

  it("finds an asset by a tag title of either language or by its slug", () => {
    for (const q of ["commuter", "pendler", "Öffentlicher Raum", "public-space", "public space"]) expect(filterAndSort([row], { ...NO_FILTERS, q }, "en").length, q).toBe(1);
    expect(filterAndSort([row], { ...NO_FILTERS, q: "zebra" }, "en")).toHaveLength(0);
  });
});
