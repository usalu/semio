/** ⚖️ Compare state: the address round trip of two to four panes, unknown values, the quick actions and one clock driving every pane.
 * @see ../../🔨️modules/⚖️compare/🧮️panes/🟦️.ts
 * @see ../../🔨️modules/🗺️routing/🟦️.ts
 */
import { describe, expect, it } from "vitest";
import { StaffageClock } from "../../🔨️modules/⏱️clock/🟦️.ts";
import { clockSpan, defaultPanes, differentAssetsSameView, nextPane, sameAssetAcrossViews, selectionFromKey, selectionKey, swapFirstTwo } from "../../🔨️modules/⚖️compare/🧮️panes/🟦️.ts";
import { compareFromParams, compareToParams, formatAddress, parseAddress, type CompareState } from "../../🔨️modules/🗺️routing/🟦️.ts";

const four: CompareState = {
  panes: [
    { asset: "walking-commuter", view: "dark-line", selection: null },
    { asset: "walking-commuter", view: "thermal", selection: { kind: "clip", clip: "walk-cycle" } },
    { asset: "cyclist", view: "forces", selection: { kind: "pose", pose: "riding-upright" } },
    { asset: "park-bench", view: "moments", selection: null },
  ],
  scale: 50,
  timeS: 1.25,
};

describe("compare address", () => {
  it("round-trips four panes through the hash and a WHATWG URL", () => {
    const hash = formatAddress({ page: "compare" }, compareToParams(four));
    const url = new URL(`https://archive.test/${hash}`);
    const { route, params } = parseAddress(url.hash);
    expect(route).toEqual({ page: "compare" });
    expect(compareFromParams(params)).toEqual(four);
    expect(url.hash.split("?")[1]!.match(/p=/gu)).toHaveLength(4);
  });

  it("drops unknown views, bad slugs and panes beyond four", () => {
    const params = new URLSearchParams("p=a-b~nope&p=BAD~thermal&p=c-d~moments~clip.x-y&p=e~dark-line&p=f~dark-line&p=g~dark-line&scale=-3&t=x");
    const state = compareFromParams(params);
    expect(state.panes.map((pane) => [pane.asset, pane.view])).toEqual([["a-b", "dark-line"], ["c-d", "moments"], ["e", "dark-line"], ["f", "dark-line"]]);
    expect(state.scale).toBe(100);
    expect(state.timeS).toBeNull();
  });
});

describe("quick actions", () => {
  it("fills the panes across views, across assets, appends and swaps", () => {
    expect(sameAssetAcrossViews("oak", 3).map((pane) => pane.view)).toEqual(["dark-line", "thermal", "forces"]);
    expect(differentAssetsSameView(["a", "b", "c"], "b", "thermal", 3).map((pane) => pane.asset)).toEqual(["b", "a", "c"]);
    expect(nextPane(four.panes.slice(0, 2)).view).toBe("dark-solid");
    expect(swapFirstTwo(four.panes)[0]).toBe(four.panes[1]);
    expect(defaultPanes(["x", "y"]).map((pane) => pane.asset)).toEqual(["x", "y"]);
    expect(defaultPanes(["walking-commuter", "y"], "walking-commuter").map((pane) => pane.view)).toEqual(["dark-line", "thermal"]);
    expect(defaultPanes([])).toEqual([]);
  });

  it("round-trips selection keys", () => {
    for (const selection of [null, { kind: "pose", pose: "a-b" }, { kind: "clip", clip: "c" }] as const) expect(selectionFromKey(selectionKey(selection))).toEqual(selection);
  });

  it("spans the clock over the longest clip", () => {
    const asset = { id: "a", poses: [], clips: [{ id: "c1", durationS: 2 }, { id: "c2", durationS: 3.5 }], presentation: { defaultClip: "c1" } } as never;
    expect(clockSpan([{ asset: "a", view: "dark-line", selection: null }, { asset: "a", view: "thermal", selection: { kind: "clip", clip: "c2" } }, { asset: "a", view: "forces", selection: { kind: "pose", pose: "p" } }], () => asset)).toBe(3.5);
  });
});

describe("shared clock", () => {
  it("shows every pane the same frame after a seek and a step", () => {
    const clock = new StaffageClock({ fps: 30, durationS: 2, loop: true, playing: false, reducedMotion: true });
    const panes = [0, 1].map(() => ({ frame: -1 }));
    const unsubscribe = panes.map((pane) => clock.subscribe(() => void (pane.frame = clock.state.frame)));
    clock.seek(0.5);
    clock.step(3);
    expect(panes.map((pane) => pane.frame)).toEqual([18, 18]);
    expect(clock.timeS).toBeCloseTo(0.6, 6);
    unsubscribe.forEach((stop) => stop());
    clock.dispose();
  });
});
