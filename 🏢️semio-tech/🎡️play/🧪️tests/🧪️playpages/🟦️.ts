export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: {
    assignPlayPages: (entries: readonly { name: string; bytes: number }[], apex: string, budget?: number) => { pages: readonly { name: string; bytes: number; host: string }[]; origins: Readonly<Record<string, string>> };
    playPageOrigins: (apex: string) => Record<string, string>;
    PLAY_PAGE_BUDGET_BYTES: number;
  },
): Promise<void> {
  const { assignPlayPages, playPageOrigins, PLAY_PAGE_BUDGET_BYTES } = dependencies;
  const { describe, expect, it } = vitest;
  const { publishedPageUrl } = await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts");

  describe("play CDN pages", () => {
    const entries = [
      { name: "index.html", bytes: 4_000 },
      { name: "assets", bytes: 180_000_000 },
      { name: "🖼️assets", bytes: 175_000_000 },
      { name: "🔌️plugin-modules", bytes: 200_000_000 },
      { name: "🧩️extension-modules", bytes: 20_000_000 },
      { name: "osm", bytes: 277_000_000 },
      { name: "vt", bytes: 329_000_000 },
      { name: "dem", bytes: 2_000_000 },
    ];

    it("keeps every page under 1GB", () => {
      const { pages } = assignPlayPages(entries, "play.semio-tech.com");
      expect(pages.map((page) => page.name)).toEqual(["play", "map"]);
      for (const page of pages) expect(page.bytes).toBeLessThan(PLAY_PAGE_BUDGET_BYTES);
      expect(pages.find((page) => page.name === "map")?.host).toBe("map.assets.semio-tech.com");
    });

    it("refuses a page that reaches 1GB", () => {
      expect(() => assignPlayPages([{ name: "assets", bytes: PLAY_PAGE_BUDGET_BYTES }], "play.semio-tech.com")).toThrow(/CDN page limit/);
    });

    it("prefixes satellite routes and leaves the app origin relative", () => {
      const origins = playPageOrigins("play.semio-tech.com");
      expect(publishedPageUrl("/osm/{z}/{x}/{y}.png", origins)).toBe("https://map.assets.semio-tech.com/osm/{z}/{x}/{y}.png");
      expect(publishedPageUrl("/🔌️plugin-modules/cad/bridge.js", origins)).toBe("/🔌️plugin-modules/cad/bridge.js");
      expect(publishedPageUrl("./assets/app.js", origins)).toBe("./assets/app.js");
      expect(publishedPageUrl("https://cdn.example/osm/0/0/0.png", origins)).toBe("https://cdn.example/osm/0/0/0.png");
    });
  });
}
