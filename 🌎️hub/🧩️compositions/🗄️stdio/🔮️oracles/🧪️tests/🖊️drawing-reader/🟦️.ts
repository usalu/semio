import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve, relative } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { inspectRustCompileReferences, rustTokens, rustTokenPairs } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import contract from "../../🧫️fixtures/🖊️drawing-reader/🔣️.json";
import { extractedDxfMutationSource } from "./🧩️preservation/🟦️.ts";
import schema from "../../🧬️schema/🖊️drawing-reader/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");
const familyManifest = `${contract.owner}/📦️packages/🦀️rust/Cargo.toml`;

test("owned schema and independent Ajv close exact reader authorities and hostile cases", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const [value, expected] of [[contract, true], [{ ...contract, unknown: true }, false], [{ ...contract, owner: "🌎️hub" }, false], [{ ...contract, externalDependency: { version: "0.6", optional: false } }, false], [{ ...contract, cases: [{ id: "silent", input: "x" }] }, false]] as const) {
    expect(validateJsonSchemaSubset(schema, value).length === 0).toBe(expected);
    expect(validate(value)).toBe(expected);
  }
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
  expect(cargo.features.oracles).toEqual(["dep:dxf"]);
});

test("both actual providers select the lower reader without sibling or higher assembly edges", () => {
  for (const original of contract.originals.filter(({ path }) => path.endsWith("Cargo.toml"))) {
    const raw = read(original.path), cargo = toml.parse(raw) as any;
    expect(Bun.TOML.parse(raw)).toEqual(cargo);
    const dependency = cargo.dependencies[contract.package];
    expect(dependency, original.path).toBeDefined();
    expect(resolve(root, original.path, "..", dependency.path)).toBe(resolve(root, familyManifest, ".."));
    expect(cargo.features.oracles).toContain(`${contract.package}/oracles`);
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
  const support = relative(resolve(root, dxf.path, ".."), resolve(root, contract.owner, "🧰️support/🦀️.rs")).replaceAll("\\", "/");
  const mount = `\n\n#[cfg(feature = "oracles")]\n#[path = ${JSON.stringify(support)}]\nmod reference_support;`;
  const current = read(dxf.path).replace(mount, "").replace("\n    use super::reference_support::{load, point_json, obj};", "");
  expect(current).toBe(extractedDxfMutationSource(dxf.source));
  expect(read(dxf.path)).not.toContain("pub fn project_dxf_r12");
  for (const row of contract.originals.filter(({ path }) => path.endsWith(".rs") && path !== dxf.path)) {
    let expected = row.source;
    if (row.path.includes("/🗒️note/")) expected = expected.replaceAll("crate::artifacts::dxf::standards::v_r12::subsets::header::project_dxf_r12", `${contract.library}::project_dxf_r12`);
    else if (row.path.includes("/🧪️tests/")) {
      expected = expected.replaceAll(", project_dxf_r12}", "}").replaceAll("use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;", `use ${contract.library}::project_dxf_r12;`);
      if (!expected.includes(`use ${contract.library}::project_dxf_r12;`)) expected = `use ${contract.library}::project_dxf_r12;\n${expected}`;
    }
    expect(read(row.path), row.path).toBe(expected);
  }
  expect(JSON.parse(read(`${contract.owner}/🧫️fixtures/🖊️semantic/🔣️.json`))).toEqual(contract.cases);
});

test("the actual private shared helpers and public byte boundary carry no foreign types", () => {
  const support = read(`${contract.owner}/🧰️support/🦀️.rs`);
  for (const name of ["load", "point_json", "obj"]) expect(support).toContain(`pub(super) fn ${name}`);
  expect(support).not.toMatch(/pub (?:fn|struct|enum|type|use)/u);
  const source = read(`${contract.owner}/🦀️.rs`);
  expect(source).toContain("pub use reader::project_dxf_r12;");
  for (const parameter of ["bytes: &[u8]", "_bytes: &[u8]"]) expect(source).toContain(`pub fn project_dxf_r12(${parameter}) -> Result<Json, String>`);
  const original = contract.originals.find(({ path }) => path === contract.functions[0]!.source)!.source;
  const refusal = /pub fn project_dxf_r12\(_bytes: &\[u8\]\) -> Result<Json, String> (\{[^}]*\})/u.exec(original)![1]!;
  expect(source).toContain(refusal);
  const mounts = contract.originals.filter(({ path }) => path === contract.functions[0]!.source).flatMap(({ path }) => inspectRustCompileReferences(read(path)).filter((row) => row.path.includes("🧰️support")).map((row) => resolve(root, path, "..", row.path)));
  expect(mounts).toEqual([resolve(root, contract.owner, "🧰️support/🦀️.rs")]);
  expect(inspectRustCompileReferences(source).filter(({ path }) => path.includes("🧰️support")).map(({ path }) => resolve(root, contract.owner, path))).toEqual(mounts);
});
