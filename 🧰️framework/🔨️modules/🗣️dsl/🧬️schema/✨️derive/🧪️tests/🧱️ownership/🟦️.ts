/** 🧩️ Admits generic derivation and explicit higher composition through one owned compiler. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

import TOML from "@iarna/toml";

type Case = Readonly<{id: string; mode: string; names?: Readonly<{spec: string; to: string; from: string}>; members: readonly string[]; traits: readonly string[]}>;
const owner = resolve(import.meta.dir, "../.."), read = (path: string): string => readFileSync(resolve(owner, path), "utf8");

test("language-neutral emission modes require explicit projection names and exclude product fronts", () => {
  const fixture = JSON.parse(read("🧫️fixtures/🧩️composition/🔣️.json")) as {cases: Case[]; ownedFronts: string[]; higherFronts: string[]};
  expect(new Set(fixture.ownedFronts).intersection(new Set(fixture.higherFronts)).size).toBe(0);
  const projection = fixture.cases.find(row => row.mode === "Projection")!;
  expect(Object.values(projection.names!)).toEqual([...projection.members]);
  expect(projection.traits).toEqual(["BorrowedDslRecord"]);
  expect(fixture.cases.find(row => row.mode === "Record")!.members).toContain("__dsl_from_record_controlled");
});

test("the generic emitter is an actual proc macro owner with private third-party syntax types", () => {
  const path = "📦️packages/🦀️rust/Cargo.toml";
  expect(existsSync(resolve(owner, path)), "actual neutral emitter package").toBe(true);
  const source = read(path), native = Bun.TOML.parse(source) as {package: {name: string}; lib: {name: string; path: string; "proc-macro": boolean}; dependencies: Record<string, unknown>};
  expect(native).toEqual(TOML.parse(source) as typeof native);
  expect(native.package.name).toBe("semio-framework-dsl-record-derive");
  expect(native.lib).toEqual({name: "semio_framework_dsl_record_derive", path: "../../🦀️.rs", "proc-macro": true});
  expect(Object.keys(native.dependencies).some(name => name.includes("os") || name.includes("protocol"))).toBe(false);
});

test("every public compiler front uses the owned system token boundary", () => {
  expect(existsSync(resolve(owner, "🦀️.rs")), "actual generic emitter definitions").toBe(true);
  const source = read("🦀️.rs");
  for (const name of ["derive_record", "derive_scalar", "derive_enum", "record_binding", "record_projection", "variant_binding"]) expect(source).toContain(`pub fn ${name}(input: TokenStream) -> TokenStream`);
  for (const spelling of ["::semio_framework_os_kernel", "::store::", "DslArtifact", "DslDiff", "DslOps", "pub use syn", "pub use quote"]) expect(source).not.toContain(spelling);
});


test("a required inline tagged owner has exactly one native variant without boxing", () => {
  const fixture = JSON.parse(read("🧫️fixtures/🏷️required-inline/🔣️.json")) as {valid: unknown[]; invalid: unknown[]; fieldKinds: {type: string; role: string}[]};
  for (const row of [...fixture.valid, ...fixture.invalid]) expect(JSON.parse(JSON.stringify(row))).toEqual(row);
  const native = read("../🧪️tests/🏷️required-inline/🦀️.rs");
  expect(native).toContain("serde_json::from_value::<InlineStatement>");
  expect(native).toContain('corpus["invalid"].as_array().unwrap()');
  const source = read("🦀️.rs");
  expect(source).toContain("RequiredInlineStatements(Box<Type>)");
  expect(source).toContain("FieldKind::RequiredInlineStatements");
  expect(fixture.fieldKinds.map(row => row.role)).toEqual(["RequiredInlineStatements", "RequiredStatements", "OptionStatements", "VecStatements"]);
});
