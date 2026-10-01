import { describe, it, expect } from "vitest";
import Ajv from "ajv";
import { readFileSync, mkdirSync, writeFileSync, mkdtempSync, symlinkSync, rmSync } from "node:fs";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";
import { execFileSync } from "node:child_process";
import { linkedSessionEngines } from "../../⚙️engine/🧭️selection/🟦️.ts";
import { declaredBrowserSessionEnginesV1 } from "../../⚙️engine/🧭️selection/🟨️.mjs";
import schema from "../../🧩️contribution/🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🧩️contribution/🔣️.json";
import { ownedDevPath, parseDevContribution } from "../../🧩️contribution/🧬️schema/🟦️.ts";
import { loadDevContribution, resolveDevContributionFile } from "../../🧩️contribution/📥️loading/🟦️.ts";
import { resolveShellBrandById } from "../../🏷️brand/🟦️.ts";

import localHubSchema from "../../🚀️local-hub/🧬️schema/🔣️.json";
import brokerSchema from "../../../📇️directory/🎫️local-session/🗄️broker/🧬️schema/🔣️.json";
const ajv = new Ajv({ strict: true }).addKeyword("x-semio-formats").addSchema(brokerSchema).addSchema(localHubSchema);
const validate = ajv.compile(schema), pathOracle = ajv.compile(schema.$defs.DevContributionPathV1);

describe("owned development contributions", () => {
  it("admits only selected-owner factory engine contributions across Bun, Node and Ajv", () => {
    const corpus = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/🔗️linked-session-engines.json"), "utf8"));
    const oracle = ajv.compile(schema.properties.browserSessionFactories);
    const root = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!root) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
    mkdirSync(root, { recursive: true });
    const temporary = mkdtempSync(join(root, "selected-browser-factories-"));
    try {
      expect(declaredBrowserSessionEnginesV1(temporary, undefined)).toEqual([]);
      for (const vector of corpus.valid) {
        expect(oracle(vector.declarations)).toBe(true);
        expect(linkedSessionEngines(vector.declarations)).toEqual(vector.engines);
        writeFileSync(join(temporary, "contribution.json"), JSON.stringify({ ...fixture.valid, browserSessionFactories: vector.declarations }));
        expect(declaredBrowserSessionEnginesV1(temporary, "contribution.json")).toEqual(vector.engines);
        const node = "const {declaredBrowserSessionEnginesV1:f}=await import(process.argv[1]);process.stdout.write(JSON.stringify(f(process.argv[2],'contribution.json')));";
        expect(JSON.parse(execFileSync("node", ["--input-type=module", "-e", node, pathToFileURL(resolve(import.meta.dirname, "../../⚙️engine/🧭️selection/🟨️.mjs")).href, temporary], { encoding: "utf8" }))).toEqual(vector.engines);
      }
      for (const value of corpus.invalid) {
        expect(oracle(value)).toBe(false);
        expect(() => linkedSessionEngines(value)).toThrow();
        writeFileSync(join(temporary, "contribution.json"), JSON.stringify({ ...fixture.valid, browserSessionFactories: value }));
        expect(() => declaredBrowserSessionEnginesV1(temporary, "contribution.json")).toThrow();
      }
      console.log(`Selected browser factories: ${corpus.valid.length + corpus.invalid.length} portable vectors with independent Ajv and real Node process`);
    } finally { rmSync(temporary, { recursive: true, force: true }); }
  });

  it("agrees with the independent JSON Schema oracle on neutral fixtures", () => {
    expect(validate(fixture.valid)).toBe(true);
    expect(parseDevContribution(fixture.valid)).toEqual(fixture.valid);
    for (const value of fixture.invalid) { expect(validate(value)).toBe(false); expect(() => parseDevContribution(value)).toThrow(); }
    for (const path of fixture.validPaths) { expect(pathOracle(path)).toBe(true); expect(ownedDevPath(path)).toBe(path); }
    for (const path of fixture.invalidPaths) { expect(pathOracle(path)).toBe(false); expect(() => ownedDevPath(path)).toThrow(); }
    expect(() => parseDevContribution({ ...fixture.valid, browserEntry: "other/app.ts" })).toThrow();
    console.log(`Development contribution oracle cases=${fixture.invalid.length + fixture.validPaths.length + fixture.invalidPaths.length + 2}`);
  });
  it("selects only an explicitly contributed brand and supports unbranded owners", () => {
    const brand = { id: "fixture", windowTitle: "Fixture" };
    expect(resolveShellBrandById([brand], "fixture")).toBe(brand);
    expect(resolveShellBrandById([], undefined)).toBeUndefined();
    expect(() => resolveShellBrandById([brand], "missing")).toThrow();
    expect(loadDevContribution(process.cwd(), {})).toBeUndefined();
    expect(() => loadDevContribution(process.cwd(), { brand: "fixture" })).toThrow();
  });
  it("rejects a real symlink crossing its owner even when the path looks owned", () => {
    const workspace = resolve(import.meta.dirname, "../../../../../../.."), generated = resolve(workspace, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated");
    mkdirSync(generated, { recursive: true });
    const ticket = mkdtempSync(resolve(generated, "dev-contribution-boundary-"));
    mkdirSync(resolve(ticket, "owner"), { recursive: true });
    mkdirSync(resolve(ticket, "outside"));
    writeFileSync(resolve(ticket, "outside/file.ts"), "export {};");
    symlinkSync(resolve(ticket, "outside"), resolve(ticket, "owner/link"), "junction");
    try { expect(() => resolveDevContributionFile(ticket, "owner/link/file.ts", "owner")).toThrow(); }
    finally { rmSync(ticket, { recursive: true, force: true }); }
  });
});
