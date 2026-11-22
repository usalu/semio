/** 🗺️ The address carries every meaningful state and survives a round trip; the path is cross-checked against the platform's URL parser.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import { DEFAULT_VIEWER, compareFromParams, compareToParams, formatAddress, openedChain, parseAddress, parentRoute, playgroundFromParams, playgroundToParams, viewerFromParams, viewerToParams, type ViewerState } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { NO_FILTERS, filtersFromParams, filtersToParams } from "../../🔨️modules/🔎️search/🟦️.ts";
import { ENVIRONMENT_PRESETS } from "../../🔨️modules/🌤️environments/🟦️.ts";

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
  const state: ViewerState = { ...DEFAULT_VIEWER, view: "thermal", scale: 50, selection: { kind: "clip", clip: "walk" }, timeS: 1.25, zoom: 2.5, pan: { xMm: 12, yMm: -4 }, background: "transparent", theme: "dark", thermalQuantity: "net-radiative-flux", range: { mode: "fixed", min: -50, max: 120 }, patterns: false, diagrams: { axial: true, shear: false, moment: false, freeBody: true }, environment: { airTempC: -5, relativeHumidity: 0.4, windMS: 6.5 } };

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

  it("round-trips the whole air: preset, every environment field, the conditions of the flow and the detail of its marks", () => {
    const air: ViewerState = { ...DEFAULT_VIEWER, view: "thermal", preset: ENVIRONMENT_PRESETS[0]!.id, environment: { airTempC: -5.5, meanRadiantTempC: 12, skyTempC: -20.5, groundTempC: 3, relativeHumidity: 0.57, windMS: 6.5, windDirectionDeg: 270, solarWM2: 640, diffuseFraction: 0.07, pressurePa: 95050 }, flow: ["streams", "pathlines"], flowSettings: { profile: "logarithmic", roughnessM: 0.25, referenceHeightM: 4.5, turbulenceIntensity: 0.33, turbulenceFloorMS: 0.05, lagrangianTimeS: 2.5, lifeS: 20, seed: 42 }, flowDetail: { spacingMm: 4.5, minStreakMm: 1.2, periodS: 0.8, exposureS: 0.25, trailS: 3.5, strokeMm: 0.45 } };
    const params = viewerToParams(air);
    expect(viewerFromParams(new URLSearchParams(params.toString()))).toEqual(air);
    expect(params.toString()).toBe(`view=thermal&fl=s%2Cp&env=${ENVIRONMENT_PRESETS[0]!.id}&air=-5.5&mrt=12&sky=-20.5&gnd=3&rh=57&wind=6.5&wdir=270&sun=640&dif=7&hpa=950.5&wp=log&z0=0.25&zr=4.5&ti=33&tf=0.05&tt=2.5&life=20&seed=42&fsp=4.5&fst=1.2&fpe=0.8&fex=0.25&ftr=3.5&fsw=0.45`);
    expect(viewerToParams(viewerFromParams(params)).toString()).toBe(params.toString());
  });

  it("writes no air for the defaults, drops a value that is its default, folds a bearing and holds every value in its range", () => {
    const parsed = viewerFromParams(new URLSearchParams("env=no-such-place&wdir=-90&sky=-500&dif=250&hpa=10&ti=10&tf=0.03&tt=1&z0=0.03&zr=10&life=14&seed=1&fsp=2.2&fst=0.9&fpe=1.5&fex=0.4&ftr=0&fsw=0.3&wp=flat"));
    expect(parsed.preset).toBeNull();
    expect(parsed.environment).toEqual({ windDirectionDeg: 270, skyTempC: -90, diffuseFraction: 1, pressurePa: 30000 });
    expect(parsed.flowSettings).toEqual({});
    expect(parsed.flowDetail).toEqual({});
    expect(viewerFromParams(new URLSearchParams("wdir=450&seed=99999&life=1&ti=abc")).environment).toEqual({ windDirectionDeg: 90 });
    expect(viewerFromParams(new URLSearchParams("wdir=360&seed=99999&life=1&ti=abc"))).toMatchObject({ environment: { windDirectionDeg: 0 }, flowSettings: { seed: 9999, lifeS: 2 } });
    expect([...viewerToParams({ ...DEFAULT_VIEWER, preset: null, environment: {}, flowSettings: {}, flowDetail: {} }).keys()]).toEqual([]);
  });

  it("keeps the order of parameters stable", () => {
    expect([...viewerToParams(state).keys()]).toEqual(["view", "scale", "clip", "t", "zoom", "pan", "bg", "th", "tq", "range", "pat", "dg", "air", "rh", "wind"]);
  });
});

describe("compare and playground", () => {
  it("round-trips four compare panes", () => {
    const compare = { panes: [{ asset: "a-one", view: "dark-line" as const, viewpoint: "elevation" as const, selection: null }, { asset: "a-one", view: "thermal" as const, viewpoint: "front" as const, selection: { kind: "pose" as const, pose: "stand" } }, { asset: "b-two", view: "forces" as const, viewpoint: "elevation" as const, selection: { kind: "clip" as const, clip: "walk" } }, { asset: "c-three", view: "moments" as const, viewpoint: "plan" as const, selection: null }], scale: 200, timeS: 0.5 };
    expect(compareFromParams(new URLSearchParams(compareToParams(compare).toString()))).toEqual(compare);
  });

  it("round-trips a playground layout and clamps positions to the ground", () => {
    const playground = { scale: 50, view: "light-solid" as const, viewpoint: "front" as const, camera: "axonometric" as const, background: "dark" as const, lengthM: 30, depthM: 8, timelineS: 12, placements: [{ id: "url-0", asset: "park-bench", xM: 3.25, zM: 0, yawDeg: 0, pick: null, offsetS: 0, mirrored: false, startS: 0, durationS: null, speed: 1, loop: true, holdS: null }, { id: "url-1", asset: "walking-commuter", xM: 12, zM: 1.5, yawDeg: 30, pick: { kind: "clip" as const, clip: "walk" }, offsetS: 0.4, mirrored: true, startS: 1.5, durationS: 6, speed: 0.5, loop: false, holdS: 0.25 }] };
    expect(playgroundFromParams(new URLSearchParams(playgroundToParams(playground).toString()))).toEqual(playground);
    expect(playgroundFromParams(new URLSearchParams("len=10&p=park-bench~99"))?.placements[0]?.xM).toBe(10);
    expect(playgroundFromParams(new URLSearchParams())).toBeNull();
  });
});

describe("filters", () => {
  it("round-trips search, filters and order", () => {
    const filters = { ...NO_FILTERS, q: "eiche tree", categories: ["vegetation" as const, "humans" as const], analytics: ["thermal" as const], heightMinM: 0.5, heightMaxM: 12, powerMinW: 100, powerMaxW: 2500, rootMotion: true, completion: ["authored" as const], sort: "mass" as const, descending: true };
    expect(filtersFromParams(new URLSearchParams(filtersToParams(filters).toString()))).toEqual(filters);
    expect(filtersToParams(NO_FILTERS).toString()).toBe("");
  });
});
