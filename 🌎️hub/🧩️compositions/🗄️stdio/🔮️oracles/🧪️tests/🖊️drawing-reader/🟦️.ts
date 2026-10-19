import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve, relative } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { inspectRustCompileReferences, rustTokens, rustTokenPairs } from "@semio-tech/compiler-syntax-rust-ts";
import contract from "../🧫️fixtures/🖊️drawing-reader/🔣️.json";
import { extractedDxfMutationSource } from "./🧩️preservation/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");
const familyManifest = `${contract.owner}/📦️packages/🦀️rust/Cargo.toml`;

test("owned schema and independent Ajv close exact reader authorities and hostile cases", () => {
  for (const original of contract.originals) {
    expect(digest(original.source), original.path).toBe(original.sha256);
    expect(Buffer.byteLength(original.source)).toBe(original.bytes);
  }
});

test("the actual artifact-free family owns every original semantic reader body exactly", () => {
  expect(existsSync(resolve(root, familyManifest)), familyManifest).toBe(true);
  for (const row of contract.functions) {
    const source = read(row.destination), tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
    const start = tokens.findIndex(({ text }, index) => text === "fn" && tokens[index + 1]?.text === row.name);
    expect(start, row.name).toBeGreaterThanOrEqual(0);
    let body = start;
    while (tokens[body]?.text !== "{") body++;
    const actual = source.slice(tokens[body]!.start, tokens[pairs.get(body)!]!.end);
    expect(digest(actual), row.name).toBe(row.sha256);
    expect(inspectRustCompileReferences(source).some(({ path }) => path.includes("🗿️artifacts")), row.destination).toBe(false);
  }
  const raw = read(familyManifest), cargo = toml.parse(raw) as any;
  expect(Bun.TOML.parse(raw)).toEqual(cargo);
  expect(cargo.dependencies.dxf).toEqual(contract.externalDependency);
  expect(Object.keys(cargo.dependencies).sort()).toEqual(["dxf", "semio-repo-test-host"]);
  expect(cargo).not.toHaveProperty("features");
});

test("both actual providers select the lower reader without sibling or higher assembly edges", () => {
  for (const original of contract.originals.filter(({ path }) => path.endsWith("Cargo.toml"))) {
    const raw = read(original.path), cargo = toml.parse(raw) as any;
    expect(Bun.TOML.parse(raw)).toEqual(cargo);
    const dependency = cargo.dependencies[contract.package];
    expect(dependency, original.path).toBeDefined();
    expect(resolve(root, original.path, "..", dependency.path)).toBe(resolve(root, familyManifest, ".."));
    expect(cargo).not.toHaveProperty("features");
    expect(Object.keys(cargo.dependencies).some((name) => name.startsWith("semio-s-artifact-") || name.startsWith("semio-hub-"))).toBe(false);
  }
  const note = contract.originals.find(({ path }) => path.includes("/🗒️note/") && path.endsWith("/🦀️.rs"))!;
  const source = read(note.path);
  expect(source).toContain(`${contract.library}::project_dxf_r12(bytes)`);
  expect(source).not.toContain("crate::artifacts::dxf");
  expect(relative(root, resolve(root, contract.owner))).not.toContain("🗿️artifacts");
});

test("the entire retained mutation source and original reader callers preserve exact fresh bytes", () => {
  const dxf = contract.originals.find(({ path }) => path === contract.functions[0]!.source)!;
  const source = read(dxf.path);
  expect(source).toContain(`use ${contract.library}::reference_support;`);
  expect(source).not.toContain("pub fn project_dxf_r12");
  expect(inspectRustCompileReferences(source).some(({ path }) => path.endsWith("🧰️support/🦀️.rs"))).toBe(false);
  expect(JSON.parse(read(`${contract.owner}/🧪️tests/🧪️tests/🧫️fixtures/🖊️semantic/🔣️.json`))).toEqual(contract.cases);

});

test("the actual private shared helpers and public byte boundary carry no foreign types", () => {
  const support = read(`${contract.owner}/🧰️support/🦀️.rs`);
  expect([...support.matchAll(/^pub (?:fn|struct|enum|type|use) (\w+)/gmu)].map((match) => match[1]).sort()).toEqual(["load", "obj", "point_json"]);
  expect(support).not.toMatch(/pub\((?:super|crate)\)/u);
  const source = read(`${contract.owner}/🦀️.rs`);
  expect(source).toContain("pub use reader::project_dxf_r12;");
  expect(source).toContain("pub fn project_dxf_r12(bytes: &[u8]) -> Result<Json, String>");
  expect(source).not.toContain("pub fn project_dxf_r12(_bytes: &[u8])");
  expect(inspectRustCompileReferences(source).filter(({ path }) => path.includes("🧰️support")).map(({ path }) => resolve(root, contract.owner, path))).toEqual([resolve(root, contract.owner, "🧰️support/🦀️.rs")]);
  expect(source).toContain("pub mod reference_support;");
});


test("additive complete lower native cohort preserves the original full oracle route", () => {
  const script = read("🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/📜️script.ts");
  const route = script.match(/class NativeOracleTestScript[\s\S]*?\n\}/u)![0];
  for (const binding of ["contract.neutralLaw.source", "contract.providers.map", "contract.families.map", "contract.grammar.package.path", "contract.package.path"]) expect(route).toContain(binding);
  expect(route).toContain('if (rest.length) throw new Error');
  expect(route).toContain('runRepositoryCargoCommandV1(this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:native-oracle",0)');
  expect(route).not.toContain('"--features"');
  const project = JSON.parse(read("🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/📋️project.json"));
  expect(project.targets["test-native-drawing-reader"]?.options.command).toBe("bun 📜️script.ts test-native-drawing-reader");
  const command = script.match(/class NativeDrawingReaderTestScript[\s\S]*?\n\}/u)?.[0];
  expect(command).toContain("contract.families.find(({ module }) => module === \"drawing\")");
  expect(command).toContain("if (rest.length) throw new Error");
  expect(command).not.toContain("\"--features\"");
  expect(command).toContain("root,process.env,900_000");
});


test("the actual native semantic law resolves the declared closed fixture input", () => {
  const source = `${contract.owner}/🧪️tests/🔬️semantic/🦀️.rs`;
  const fixtures = inspectRustCompileReferences(read(source)).filter(({ path }) => path.endsWith("🔣️.json")).map(({ path }) => resolve(root, source, "..", path));
  expect(fixtures).toEqual([resolve(root, contract.owner, "🧪️tests/🧫️fixtures/🖊️semantic/🔣️.json")]);
  expect(fixtures.every(path => existsSync(path))).toBe(true);
  expect(JSON.parse(readFileSync(fixtures[0]!, "utf8"))).toEqual(contract.cases);
});
