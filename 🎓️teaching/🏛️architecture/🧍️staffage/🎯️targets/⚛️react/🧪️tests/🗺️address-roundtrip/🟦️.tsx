/** 🗺️ The address carries every meaningful state and survives a round trip; the path is cross-checked against the platform's URL parser.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import { DEFAULT_VIEWER, compareFromParams, compareToParams, formatAddress, openedChain, parseAddress, parentRoute, playgroundFromParams, playgroundToParams, viewerFromParams, viewerToParams, type ViewerState } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { NO_FILTERS, filtersFromParams, filtersToParams } from "../../🔨️modules/🔎️search/🟦️.ts";

const canonical = (hash: string): string => {
  const { route, params } = parseAddress(hash);
  return formatAddress(route, params);
};

describe("routes", () => {
  it.each(["#/", "#/all", "#/humans", "#/humans/walking-commuter", "#/collections", "#/collections/street-life", "#/compare", "#/playground"])("round-trips %s and agrees with the URL parser", (hash) => {
    expect(canonical(hash)).toBe(hash);
    expect(new URL(`https://archive.example/${hash}`).hash).toBe(hash);
  });

  it("reads anything unknown as the overview and drops what it cannot name", () => {
    expect(parseAddress("#/nothing/here").route).toEqual({ page: "overview" });
    expect(parseAddress("#/humans/Not A Slug").route).toEqual({ page: "category", category: "humans" });
    expect(parseAddress("").route).toEqual({ page: "overview" });
  });

  it("derives the opened chain and the parent of every route", () => {
    expect(openedChain(parseAddress("#/humans/walking-commuter").route)).toEqual(["humans", "walking-commuter"]);
    expect(parentRoute(parseAddress("#/humans/walking-commuter").route)).toEqual({ page: "category", category: "humans" });
    expect(parentRoute(parseAddress("#/humans").route)).toEqual({ page: "overview" });
    expect(parentRoute({ page: "overview" })).toBeNull();
  });
});

describe("viewer state", () => {
  const state: ViewerState = { ...DEFAULT_VIEWER, view: "thermal", scale: 50, selection: { kind: "clip", clip: "walk" }, timeS: 1.25, zoom: 2.5, pan: { xMm: 12, yMm: -4 }, background: "transparent", theme: "dark", thermalQuantity: "net-flux", range: { mode: "fixed", min: -50, max: 120 }, patterns: true, diagrams: { axial: true, shear: false, moment: false, freeBody: true }, environment: { airTempC: -5, relativeHumidity: 0.4, windMS: 6.5 } };

  it("round-trips every parameter", () => {
    const params = viewerToParams(state);
    expect(viewerFromParams(new URLSearchParams(params.toString()))).toEqual(state);
  });

  it("writes nothing for the defaults and ignores unknown or out-of-range values", () => {
    expect(viewerToParams(DEFAULT_VIEWER).toString()).toBe("");
    const parsed = viewerFromParams(new URLSearchParams("view=bogus&scale=-3&zoom=nope&t=-1&rh=900&wind=99&dg=zz"));
    expect(parsed.view).toBe(DEFAULT_VIEWER.view);
    expect(parsed.scale).toBe(100);
    expect(parsed.zoom).toBe("fit");
    expect(parsed.timeS).toBeNull();
    expect(parsed.environment).toEqual({ relativeHumidity: 1, windMS: 40 });
    expect(parsed.diagrams).toEqual({ axial: false, shear: false, moment: false, freeBody: false });
  });

  it("keeps the order of parameters stable", () => {
    expect([...viewerToParams(state).keys()]).toEqual(["view", "scale", "clip", "t", "zoom", "pan", "bg", "th", "tq", "range", "pat", "dg", "air", "rh", "wind"]);
  });
});

describe("compare and playground", () => {
  it("round-trips four compare panes", () => {
    const compare = { panes: [{ asset: "a-one", view: "dark-line" as const, selection: null }, { asset: "a-one", view: "thermal" as const, selection: { kind: "pose" as const, pose: "stand" } }, { asset: "b-two", view: "forces" as const, selection: { kind: "clip" as const, clip: "walk" } }, { asset: "c-three", view: "moments" as const, selection: null }], scale: 200, timeS: 0.5 };
    expect(compareFromParams(new URLSearchParams(compareToParams(compare).toString()))).toEqual(compare);
  });

  it("round-trips a playground layout and clamps positions to the ground", () => {
    const playground = { scale: 50, view: "light-solid" as const, background: "dark" as const, lengthM: 30, placements: [{ id: "url-0", asset: "park-bench", xM: 3.25, pick: null, offsetS: 0, mirrored: false }, { id: "url-1", asset: "walking-commuter", xM: 12, pick: { kind: "clip" as const, clip: "walk" }, offsetS: 0.4, mirrored: true }] };
    expect(playgroundFromParams(new URLSearchParams(playgroundToParams(playground).toString()))).toEqual(playground);
    expect(playgroundFromParams(new URLSearchParams("len=10&p=park-bench~99"))?.placements[0]?.xM).toBe(10);
    expect(playgroundFromParams(new URLSearchParams())).toBeNull();
  });
});

describe("filters", () => {
  it("round-trips search, filters and order", () => {
    const filters = { ...NO_FILTERS, q: "eiche tree", categories: ["vegetation" as const, "humans" as const], analytics: ["thermal" as const], heightMinM: 0.5, heightMaxM: 12, rootMotion: true, completion: ["authored" as const], sort: "mass" as const, descending: true };
    expect(filtersFromParams(new URLSearchParams(filtersToParams(filters).toString()))).toEqual(filters);
    expect(filtersToParams(NO_FILTERS).toString()).toBe("");
  });
});
