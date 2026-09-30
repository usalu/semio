import { describe, it, expect } from "vitest";
import Ajv from "ajv";
import { readFileSync, mkdirSync, writeFileSync, mkdtempSync, symlinkSync, rmSync } from "node:fs";
import { resolve } from "node:path";
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
