/** 🌤️ The reference environment of an asset: schema twin and validator, the precedence `--environment` over `thermal.referenceEnvironment` over `thermal.environment`, and the export CLI reading it.
 *
 * THE REFERENCE is the library path itself (`solveThermalAt` with an explicit environment and `depict`): the CLI output must equal it byte for byte, and the physics must make the sunlit absorber warmer than under the asset's own sunless environment.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { beforeAll, describe, expect, it } from "vitest";
import type { Asset, Environment, EnvironmentPreset } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { resolveThermalEnvironment, validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { STAFFAGE_ROOT, withScratch } from "../🧰️support/🧰️.ts";

const SCALE = 50;
let collector: Asset;
let tealight: Asset;
let presets: ReadonlyMap<string, EnvironmentPreset>;
const scratchTools = { run: async (): Promise<string> => "" };

beforeAll(async () => {
  const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
  collector = catalog.assets.find((asset) => asset.id === "solar-thermal-collector")!;
  tealight = catalog.assets.find((asset) => asset.id === "tealight")!;
  presets = new Map(((await loadEnvironments(DEFAULT_CATALOG_ROOT))?.presets ?? []).map((preset) => [preset.id, preset]));
});

const withReference = (asset: Asset, referenceEnvironment: unknown): Asset => ({ ...asset, thermal: { ...asset.thermal, referenceEnvironment } }) as Asset;
const argv = (extra: readonly string[], view = "thermal"): readonly string[] => ["--asset", "solar-thermal-collector", "--pose", "operating", "--view", view, "--scale", String(SCALE), "--format", "svg", "--viewpoint", "elevation", "--out", "unused", "--lang", "en", ...extra];

async function exported(extra: readonly string[], asset = "solar-thermal-collector", pose = "operating"): Promise<string> {
  return withScratch(async (out) => {
    const options = { ...parseArguments(argv(extra)), asset, pose, out };
    const [file] = await runExport(options, scratchTools);
    return readFileSync(file!, "utf8");
  });
}

function oracle(asset: Asset, environment: Environment | undefined, pose = "operating"): string {
  const selector = { kind: "scale", scale: { denominator: SCALE } } as const;
  const state = evaluate(asset, { kind: "pose", pose }, selector);
  return writeSvg(depict(state, { ...defaultDepictionOptions("thermal", "en", SCALE, "elevation"), transparent: false, ...(environment === undefined ? {} : { environment }) }));
}

describe("🧬️ the declaration", () => {
  it("is optional, a slug or an inline environment in the schema JSON", () => {
    const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🧬️schema/🔣️.json"), "utf8"));
    const model = schema.$defs.ThermalModel;
    expect(model.required).not.toContain("referenceEnvironment");
    expect(model.properties.referenceEnvironment.oneOf).toEqual([{ $ref: "#/$defs/Slug" }, { $ref: "#/$defs/Environment" }]);
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema, "staffage");
    const fits = (value: unknown): boolean => ajv.validate({ $ref: "staffage#/$defs/ThermalModel/properties/referenceEnvironment" }, value) as boolean;
    expect(fits("summer-heatwave-calm")).toBe(true);
    expect(fits(collector.thermal.environment)).toBe(true);
    expect(fits("Not A Slug")).toBe(false);
    expect(fits({ airTempC: 20 })).toBe(false);
  });

  it("is declared by the collector as an existing preset", () => {
    expect(collector.thermal.referenceEnvironment).toBe("summer-heatwave-calm");
    expect(presets.has("summer-heatwave-calm")).toBe(true);
    expect(tealight.thermal.referenceEnvironment).toBeUndefined();
  });

  it("validates a preset id against the environment document and accepts an inline environment", () => {
    const known = new Set(presets.keys());
    expect(validateAsset(collector, { environments: known })).toEqual([]);
    expect(validateAsset(withReference(collector, collector.thermal.environment), { environments: known })).toEqual([]);
    const issues = validateAsset(withReference(collector, "no-such-weather"), { environments: known });
    expect(issues.map((issue) => `${issue.code} ${issue.path}`)).toEqual(["unresolved-reference thermal.referenceEnvironment"]);
    expect(validateAsset(withReference(collector, "no-such-weather"))).toEqual([]);
  });
});

describe("🥇️ the precedence", () => {
  it("lets an explicit preset win over the declaration, and the declaration over nothing", () => {
    expect(resolveThermalEnvironment(collector, presets, "office-neutral")).toBe(presets.get("office-neutral")!.environment);
    expect(resolveThermalEnvironment(collector, presets)).toBe(presets.get("summer-heatwave-calm")!.environment);
    expect(resolveThermalEnvironment(withReference(collector, collector.thermal.environment), presets)).toBe(collector.thermal.environment);
  });

  it("returns nothing without either, so the solver applies thermal.environment", () => {
    expect(resolveThermalEnvironment(tealight, presets)).toBeUndefined();
    expect(solveThermalAt(tealight, { kind: "pose", pose: tealight.presentation.defaultPose }).environment).toBe(tealight.thermal.environment);
  });

  it("refuses an unknown preset naming the known ones", () => {
    expect(() => resolveThermalEnvironment(collector, presets, "nope")).toThrow(/unknown environment preset nope; presets: office-neutral/u);
  });

  it("heats the sunlit absorber above its sunless default", () => {
    const query = { kind: "pose", pose: "operating" } as const;
    const sunny = solveThermalAt(collector, query, resolveThermalEnvironment(collector, presets));
    const dull = solveThermalAt(collector, query);
    expect(sunny.environment.solarWM2).toBe(900);
    expect(dull.environment.solarWM2).toBe(0);
    expect(sunny.regions.absorber!.surfaceTempC).toBeGreaterThan(dull.regions.absorber!.surfaceTempC + 10);
  });
});

describe("🎬️ the export command line", () => {
  it("parses --environment and refuses it outside the thermal view", () => {
    expect(parseArguments(argv(["--environment", "office-neutral"])).environment).toBe("office-neutral");
    expect(parseArguments(argv([])).environment).toBeUndefined();
    expect(() => parseArguments(argv(["--environment", "office-neutral"], "dark-line"))).toThrow(/--environment applies to --view thermal only/u);
  });

  it("uses the declared reference environment by default", async () => {
    expect(await exported([])).toBe(oracle(collector, presets.get("summer-heatwave-calm")!.environment));
  });

  it("lets --environment override it", async () => {
    const svg = await exported(["--environment", "office-neutral"]);
    expect(svg).toBe(oracle(collector, presets.get("office-neutral")!.environment));
    expect(svg).not.toBe(await exported([]));
  });

  it("keeps the explicit default thermal.environment for an asset without a declaration", async () => {
    const pose = tealight.presentation.defaultPose;
    expect(await exported([], "tealight", pose)).toBe(oracle(tealight, undefined, pose));
  });

  it("names the known presets for an unknown --environment", async () => {
    await expect(exported(["--environment", "nope"])).rejects.toThrow(/unknown environment preset nope/u);
  });
});
