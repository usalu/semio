import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import Ajv from "ajv";
import ts from "typescript";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { readBuildMode, buildModePolicy, buildModeEnvironment } from "../🟦️.ts";
import * as api from "../🟦️.ts";


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
  const sourceUrl = new URL("../🟦️.ts", import.meta.url), source = readFileSync(sourceUrl, "utf8");
  const compiled = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022, esModuleInterop: true } });
  const exports: Record<string, (...args: any[]) => any> = {};
  new Function("exports", "require", "moduleUrl", compiled.outputText.replaceAll("import.meta.url", "moduleUrl"))(exports, createRequire(sourceUrl), sourceUrl.href);
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

test("declared build modes expose the complete immutable portable schema enumeration", () => {
  const modes = (api as unknown as { buildModesV1(): readonly string[] }).buildModesV1();
  const oracle = new Ajv({ strict: true }).compile(schema);
  expect(modes).toEqual(fixture.modes.map(row => row.mode));
  expect(modes).toEqual(schema.enum);
  expect(Object.isFrozen(modes)).toBe(true);
  for (const mode of modes) expect(oracle(mode)).toBe(true);
  expect(() => (modes as string[]).push("foreign")).toThrow();
  const sourceUrl = new URL("../🟦️.ts", import.meta.url), source = readFileSync(sourceUrl, "utf8");
  const compiled = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022, esModuleInterop: true } });
  const exports: Record<string, (...args: any[]) => any> = {};
  new Function("exports", "require", "moduleUrl", compiled.outputText.replaceAll("import.meta.url", "moduleUrl"))(exports, createRequire(sourceUrl), sourceUrl.href);
  expect(exports.buildModesV1!()).toEqual(modes);
  expect(Object.isFrozen(exports.buildModesV1!())).toBe(true);
});


test("current owned build vocabulary revisions preserve immutable semantic cache authority", () => {
  const sourceUrl=new URL("../🟦️.ts",import.meta.url),source=readFileSync(sourceUrl,"utf8"),actualRequire=createRequire(sourceUrl);
  const compiled=ts.transpileModule(source,{compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2022,esModuleInterop:true}});
  let current={...schema,enum:[...fixture.enumRevisions[0]!]};
  const exports:Record<string,(...args:any[])=>any>={};
  const requirePort=(name:string)=>name==="./🧬️schema/🔣️.json"?current:name==="node:fs"?{...actualRequire(name),readFileSync:(...args:unknown[])=>JSON.stringify(current)}:actualRequire(name);
  new Function("exports","require","moduleUrl",compiled.outputText.replaceAll("import.meta.url","moduleUrl"))(exports,requirePort,sourceUrl.href);
  const revisions:readonly string[][]=[];
  for(const modes of fixture.enumRevisions){
    current.enum.splice(0,current.enum.length,...modes);
    const value=exports.buildModesV1!(),oracle=new Ajv({strict:true}).compile(current);
    expect(value).toEqual(modes);expect(value).toEqual(JSON.parse(JSON.stringify(current)).enum);expect(Object.isFrozen(value)).toBe(true);
    for(const mode of value)expect(oracle(mode)).toBe(true);
    (revisions as string[][]).push(value);
  }
  expect(revisions[0]).toEqual(fixture.enumRevisions[0]);expect(revisions[1]).toEqual(fixture.enumRevisions[1]);
  console.log("[DEBUG] Actual TypeScript-transpiled producer observes current owned schema-service revisions; independent Node JSON/Ajv agree; earlier immutable value retained");
});
