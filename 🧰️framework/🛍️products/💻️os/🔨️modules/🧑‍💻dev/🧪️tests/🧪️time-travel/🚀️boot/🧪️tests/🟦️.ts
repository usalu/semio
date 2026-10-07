/** 🧪️ Universal boot selection agrees with catalog authority and the independent URL parser. */
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { playgroundCatalog } from "../../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { timeTravelServeVariantV1, type TimeTravelServeSelectionV1 } from "../🟦️.ts";
import { playgroundActivationTargetV1 } from "../../../../♻️activation/🏃️execution/🟦️.ts";
import { ensureDevServe } from "../../../../🚀️local-hub/🏃️execution/🟦️.ts";
const { JSDOM } = createRequire(import.meta.url)("jsdom");
const read = (relative: string) => JSON.parse(readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8"));
const fixture = read("../🧫️fixtures/🔣️.json");

describe("universal history serve boot", () => {
  for (const law of fixture.cases) it(law.name, () => {
    const selection = law as TimeTravelServeSelectionV1;
    if (law.error) expect(() => timeTravelServeVariantV1(selection, playgroundCatalog)).toThrow();
    else {
      const variant = timeTravelServeVariantV1(selection, playgroundCatalog);
      expect(variant).toBe(law.expected);
      expect(playgroundCatalog.some(row => row.variant === variant)).toBe(true);
      const dom = new JSDOM();
      const url = new dom.window.URL(law.url);
      const requested = law.explicit ?? url.searchParams.get("plugin");
      const oracle = !law.universal ? "puzzle2d" : requested ? playgroundCatalog.find(row => row.variant === requested || row.aliases.includes(requested))?.variant : playgroundCatalog.find(row => row.ports[law.renderer as "react" | "wgpu"] === Number(url.port))?.variant;
      expect(variant).toBe(oracle);
      dom.window.close();
    }
  });
});


describe("owned history journey startup", () => {
  for (const law of fixture.startups) it(law.name, async () => {
    const controller = new AbortController();
    if (law.cancel === "before") controller.abort();
    const trace: string[] = [];
    let spawned = false;
    let activated = false;
    const variant = timeTravelServeVariantV1({ universal: true, url: `http://127.0.0.1:${law.port}/?plugin=${law.variant}`, renderer: law.renderer }, playgroundCatalog);
    const result = await ensureDevServe({
      repoRoot: "fixture", port: law.port, variant, renderer: law.renderer, signal: controller.signal,
      beforeSpawn: async () => {
        trace.push(playgroundActivationTargetV1(variant, "dev", law.renderer));
        activated = true;
        if (law.activationFails) throw Error("activation refused");
        if (law.cancel === "during") controller.abort();
      },
      world: {
        answers: async () => law.answers || Boolean(activated && law.afterActivationAnswers) || spawned,
        portInUse: async () => law.occupied || Boolean(activated && law.afterActivationOccupied) || spawned,
        spawnServe: request => { expect(request.variant).toBe(variant); trace.push("spawn"); spawned = true; return { pid: 17, exited: () => false }; },
        terminate: () => { trace.push("stop"); spawned = false; },
        now: () => 0, sleep: async () => undefined,
      },
    }).catch(() => null);
    expect(result === null ? "error" : result.reused ? "reused" : "ready").toBe(law.outcome);
    await result?.stop();
    expect(JSON.parse(JSON.stringify(trace))).toEqual(law.trace);
  });
});
