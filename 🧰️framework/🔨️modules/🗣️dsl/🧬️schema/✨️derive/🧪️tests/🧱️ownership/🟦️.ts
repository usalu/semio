/** 🧩️ Admits generic derivation and explicit higher composition through one owned compiler. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { build } from "esbuild";
import Ajv2020 from "ajv/dist/2020";
import empty from "../🕳️empty/🔣️.json" with { type: "json" };
import emptySchema from "../🕳️empty/🧬️.json" with { type: "json" };
import absence from "../🚮️absence/🔣️.json" with { type: "json" };
import absenceSchema from "../🚮️absence/🧬️.json" with { type: "json" };

import TOML from "@iarna/toml";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

type Case = Readonly<{id: string; mode: string; names?: Readonly<{spec: string; to: string; from: string}>; members: readonly string[]; traits: readonly string[]}>;
const owner = resolve(import.meta.dir, "../.."), read = (path: string): string => readFileSync(resolve(owner, path), "utf8");
const readTree = (path: string): string => readModuleTreeSource(resolve(owner, path));

/** 🚮️ Actual generic native receivers survive removal of product sources and preserve owned law selection. */
test("generic derivation native commands survive product removal", async () => {
  expect(new Ajv2020({ strict: true }).validate(absenceSchema, absence)).toBe(true);
  const workspace = resolve(owner, "../../../../.."), entry = resolve(owner, absence.entry);
  const bundled = await build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", plugins: [{ name: "specific-removal", setup(builder) {
    builder.onResolve({ filter: /./ }, args => { if (absence.removedPackages.some(name => args.path === name || args.path.startsWith(name + "/") || args.path.startsWith(name + "-"))) throw Error("removed product package: " + args.path); return undefined; });
    builder.onLoad({ filter: /./ }, args => { const path = relative(workspace, args.path).split("\\").join("/"); if (absence.removedRoots.some(root => path === root || path.startsWith(root + "/"))) throw Error("removed product source: " + path); return undefined; });
  } }] });
  expect(Object.keys(bundled.metafile!.inputs).length).toBeGreaterThan(1);
  expect(Object.keys(bundled.metafile!.inputs).every(path => ["🧰️framework/🔨️modules/","🧰️framework/📦️packages/"].some(root => path.startsWith(root)))).toBe(true);
  const source = read(absence.entry);
  for (const name of [...absence.packages, ...absence.laws]) expect(source).toContain(name);
});

test("language-neutral emission modes require explicit projection names and exclude product fronts", () => {
  const fixture = JSON.parse(read("../../🧪️tests/🧫️fixtures/✨️derive/🧩️composition/🔣️.json")) as {cases: Case[]; ownedFronts: string[]; higherFronts: string[]};
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
  const source = readTree("🦀️.rs");
  for (const name of ["derive_record", "derive_scalar", "derive_enum", "record_binding", "record_projection", "variant_binding"]) expect(source).toContain(`pub fn ${name}(input: TokenStream) -> TokenStream`);
  for (const spelling of ["::semio_framework_os_kernel", "::store::", "DslArtifact", "DslDiff", "DslOps", "pub use syn", "pub use quote"]) expect(source).not.toContain(spelling);
});


test("a required inline tagged owner has exactly one native variant without boxing", () => {
  const fixture = JSON.parse(read("../../🧪️tests/🧫️fixtures/✨️derive/🏷️required-inline/🔣️.json")) as {valid: unknown[]; invalid: unknown[]; fieldKinds: {type: string; role: string}[]};
  for (const row of [...fixture.valid, ...fixture.invalid]) expect(JSON.parse(JSON.stringify(row))).toEqual(row);
  const native = read("../🧪️tests/🏷️required-inline/🦀️.rs");
  expect(native).toContain("serde_json::from_value::<InlineStatement>");
  expect(native).toContain('corpus["invalid"].as_array().unwrap()');
  const source = readTree("🦀️.rs");
  expect(source).toContain("RequiredInlineStatements(Box<Type>)");
  expect(source).toContain("FieldKind::RequiredInlineStatements");
  expect(fixture.fieldKinds.map(row => row.role)).toEqual(["RequiredInlineStatements", "RequiredStatements", "OptionStatements", "VecStatements"]);
});

/** 🫴️ Declares original optional custody independently of public wire fields. */
test("original optional custody stays native while wire omits its source",()=>{
 const fixture=JSON.parse(read("../../🧪️tests/🧫️fixtures/✨️derive/🫴️custody/🔣️.json")),source=readTree("🦀️.rs");expect(fixture.wireKeys).toEqual(["visible","count"]);expect(fixture.nativeFields).toEqual(["visible","source","count"]);expect(fixture.absentDecodedCustody).toBeNull();
 for(const row of fixture.cases){const {source:_,wire:expectedWire,...wire}=row;expect(wire).toEqual(row.wire);expect(JSON.parse(JSON.stringify(row.wire))).toEqual(row.wire);}
 expect(source).toContain('meta.path.is_ident("skip")');expect(source).toContain("if plan.skip");expect(source).toContain("filter(|plan|!plan.skip)");expect(read("🧪️tests/🦀️.rs")).toContain("original_optional_custody_is_excluded_from_wire_but_retired_as_owned_native_field");
});

test("empty native enums preserve zero variants and borrowed uninhabited match selection", () => {
 expect(new Ajv2020({strict:true}).validate(emptySchema,empty)).toBe(true);
 expect(JSON.parse(JSON.stringify([]))).toHaveLength(empty.variantCount);
 expect(empty.cases.map(row=>row.borrowedMatches)).toEqual([6,7,3]);
});

/** 🕳️ The neutral empty and inhabited owner corpus admits exactly six generic emitter roles. */
test("empty generic variant contracts retain zero and inhabited owner roles",()=>{
 const corpus=JSON.parse(read("../../🧪️tests/🧫️fixtures/✨️derive/🕳️empty/🔣️.json")),schema=JSON.parse(read("../../🧪️tests/🧫️fixtures/✨️derive/🕳️empty/🧬️.json"));
 expect(new Ajv2020({strict:true}).validate(schema,corpus)).toBe(true);expect(corpus.cases.length).toBe(6);
 expect(corpus.cases.filter((row:{borrowedEmptyMatches:number})=>row.borrowedEmptyMatches>0).map((row:{mode:string})=>row.mode)).toEqual(["Variants","Scalar","Receiving"]);
 expect(read("🧪️tests/🦀️.rs")).toContain("empty_enum_borrowed_and_receiving_emission_matches_independent_syntax");
});
