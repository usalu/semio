/** 🪟️ The mounted-window fetch law: a `refresh-ui` pass asks the guest only for the windows the mode
 * layout actually mounts. Before this, every pass asked for every window the app DECLARES — in the
 * generation3d editor that meant re-rendering the three generate-mode windows, the generate preview's
 * mesh payload included, on every `flowEvalTick` hop while the user sat in edit mode
 * (`📓️react-hop-latency-2026-09-14.md` §2).
 *
 * 🧫️ Read from `🛠️ShellHelpers/🧫️fixtures/🪟️mounted-window-fetch.json`, the neutral declaration the
 * wgpu shell's own `refresh_ui` window walk is pinned against too. The rule itself lives in
 * `🔨️modules/🎠️kernel/🟦️.ts`, beside the other `UiDirtyScope` predicates both shells share.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🪟️mounted-window-fetch.json" with { type: "json" };
import { partitionRefreshWindowInstancesV1, windowLayoutWindowIdsV1 } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";

describe("🪟️ mounted-window refresh fetch", () => {
  it("declares the contract both shells answer to", () => {
    expect(fixture.contract.unknownLayoutFetchesEverything).toBe(true);
    expect(fixture.contract.skippedWindowCacheIsDropped).toBe(true);
    expect(fixture.contract.tabsCountAsMounted).toBe(true);
  });

  for (const scenario of fixture.layouts) {
    it(scenario.name, () => {
      expect([...windowLayoutWindowIdsV1(scenario.layout)]).toEqual(scenario.windowIds);
    });
  }

  for (const scenario of fixture.cases) {
    it(scenario.name, () => {
      const { fetched, skipped } = partitionRefreshWindowInstancesV1(scenario.declared, new Set(scenario.mounted));
      expect(fetched.map((instance) => instance.id)).toEqual(scenario.expect.fetched);
      expect(skipped.map((instance) => instance.id)).toEqual(scenario.expect.skipped);
    });
  }

  it("drops the cached body of every skipped window, so mounting it later asks with no hash", () => {
    const scenario = fixture.cases[0]!;
    const cache = new Map(scenario.declared.map((instance) => [`window:${instance.id}`, { hash: `h-${instance.id}` }] as const));
    const { fetched, skipped } = partitionRefreshWindowInstancesV1(scenario.declared, new Set(scenario.mounted));
    for (const instance of skipped) cache.delete(`window:${instance.id}`);
    expect(fetched.map((instance) => cache.get(`window:${instance.id}`)?.hash)).toEqual(scenario.expect.fetched.map((id) => `h-${id}`));
    // 🪟️ After a mode switch every declared window is mounted again; the ones this pass skipped own no
    // hash, which is what makes the guest answer with a whole body instead of "unchanged".
    const remounted = partitionRefreshWindowInstancesV1(scenario.declared, new Set(scenario.declared.map((instance) => instance.id)));
    expect(remounted.skipped).toEqual([]);
    expect(remounted.fetched.filter((instance) => cache.get(`window:${instance.id}`) === undefined).map((instance) => instance.id)).toEqual(scenario.expect.skipped);
  });
});
