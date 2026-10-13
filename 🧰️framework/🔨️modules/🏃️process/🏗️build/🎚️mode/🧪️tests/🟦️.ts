import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import ts from "typescript";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { readBuildMode, buildModePolicy, buildModeEnvironment } from "../🟦️.ts";

test("build mode admission agrees with the independent schema validator", () => {
  const oracle = new Ajv({ strict: true }).compile(schema);
  for (const row of fixture.modes) {
    expect(oracle(row.mode)).toBe(true);
    expect(readBuildMode(row.mode)).toBe(row.mode);
    expect(buildModePolicy(readBuildMode(row.mode))).toEqual(row.policy);
  }
  for (const value of [...fixture.invalid, undefined]) {
    expect(oracle(value)).toBe(false);
    expect(() => readBuildMode(value)).toThrow("prototype or release");
    expect(() => buildModePolicy(value as never)).toThrow("prototype or release");
  }
});

test("portable build policy has the same result under the TypeScript compiler", () => {
  const source = readFileSync(new URL("../🟦️.ts", import.meta.url), "utf8");
  const compiled = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } });
  const exports: Record<string, (...args: any[]) => any> = {};
  new Function("exports", compiled.outputText)(exports);
  for (const row of fixture.modes) expect(exports.buildModePolicy!(row.mode)).toEqual(row.policy);
  expect(source).not.toContain("🛍️products");
});

test("build environments preserve caller configuration without mutating it", () => {
  const environment = Object.freeze({ PATH: "/tools", SEMIO_BUILD_MODE: "release", CUSTOM: "retained" });
  for (const row of fixture.modes) {
    const mode = readBuildMode(row.mode);
    expect(buildModeEnvironment(mode, environment)).toEqual({ ...environment, SEMIO_BUILD_MODE: mode });
    expect(environment.SEMIO_BUILD_MODE).toBe("release");
  }
  expect(() => buildModeEnvironment("dev" as never, environment)).toThrow("prototype or release");
});
