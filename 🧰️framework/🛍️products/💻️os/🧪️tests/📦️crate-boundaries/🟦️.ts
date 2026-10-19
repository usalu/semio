import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import { readRustSourceClosureV1, selectRustSourceV1 } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { rustTokens } from "../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { readModuleTreeSource, sliceModuleTreeSource } from "../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";

const os = resolve(import.meta.dir, "../.."), workspace = resolve(os, "../../.."), read = (path: string) => readFileSync(resolve(os, path), "utf8");
const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
type Manifest = { package: { name: string }; lib: { path: string }; dependencies: Record<string, { package?: string }> };
const corpus = JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json")) as { crates: { package: string; manifest: string; entry: string; maximumActiveLines: number; forbiddenDependencies: string[] }[] };
test("language-neutral OS ownership corpus validates with the independent schema oracle", () => {
  const validate = new Ajv().compile(JSON.parse(read("🧬️schema/📦️crate-boundaries/🔣️.json")));
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, hidden: true })).toBe(false);
});
for (const row of corpus.crates) test(row.package + " has independent production ownership and an authored source ceiling", async () => {
  const source = read(row.manifest), manifest = Bun.TOML.parse(source) as unknown as Manifest;
  expect(manifest).toEqual(TOML.parse(source) as unknown as Manifest);
  expect(manifest.package.name).toBe(row.package);
  const project = JSON.parse(read(row.manifest.replace("Cargo.toml", "📋️project.json"))) as { name: string };
  const owners = JSON.parse(readFileSync(resolve(process.cwd(), "📋️build-owners.json"), "utf8")).owners as { name: string; manifest: string }[];
  const owner = owners.find(owner => owner.name === project.name);
  expect(owner, project.name).toBeDefined();
  expect(resolve(process.cwd(), owner!.manifest)).toBe(resolve(os, row.manifest.replace("Cargo.toml", "📋️project.json")));
  expect(resolve(os, row.manifest, "..", manifest.lib.path)).toBe(resolve(os, row.entry));
  const dependencies = Object.entries(manifest.dependencies).map(([alias, edge]) => edge.package ?? alias);
  for (const forbidden of row.forbiddenDependencies) expect(dependencies).not.toContain(forbidden);
  if (row.package === "semio-framework-os-store-cursor" || row.package === "semio-framework-os-store-projection") {
    const consumer = Bun.TOML.parse(read("📦️packages/🦀️rust/Cargo.toml")) as unknown as Manifest;
    expect(consumer.dependencies[row.package]).toBeDefined();
    const schema = read("🔨️modules/🏪️store/🧬️schema/🦀️.rs");
    if (row.package === "semio-framework-os-store-cursor") {
      expect(schema).toContain("pub use semio_framework_os_store_cursor::{ArtifactCursor, ArtifactCursorOwners};");
      expect(schema).not.toContain("pub struct ArtifactCursor {");
      expect(read("🔨️modules/🏪️store/♻️retirement/🦀️.rs")).not.toContain("impl RetireOwned for super::ArtifactCursor");
      expect(read(row.entry)).toContain("impl RetireOwned for ArtifactCursor");
      expect(read(row.entry)).not.toContain("pub(crate) group:");
      expect(read(row.entry)).not.toContain("pub group:");
    } else {
      expect(schema).toContain("pub use semio_framework_os_store_projection::*;");
      expect(schema).not.toContain("pub struct ArtifactProjectionStamp {");
    }
  }
  const rows = await readRustSourceClosureV1([resolve(os, row.entry)], workspace, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
  expect(rows.reduce((sum, row) => sum + row.activeLines, 0)).toBeLessThanOrEqual(row.maximumActiveLines);
  {
    expect(rows.reduce((sum, input) => sum + input.bytes, 0)).toBeLessThanOrEqual(262144);
    expect(rows.length).toBeLessThanOrEqual(32);
    expect(rows.reduce((sum, input) => sum + input.activeLines, 0)).toBeLessThanOrEqual(4000);
    if (row.package.startsWith("semio-framework-os-history")) for (const forbidden of ["semio-framework-os-directory", "semio-framework-os-vcs", "semio-framework-os-command"]) expect(dependencies).not.toContain(forbidden);
  }
  for (const input of rows.filter(row => row.kind === "source")) {
    const tokens = rustTokens(selectRustSourceV1(readFileSync(resolve(workspace, input.path), "utf8"), cfg));
    for (const forbidden of ["semio_framework_os_kernel", "semio_framework_plugin", "os_store", "os_spr", "os_directory"]) expect(tokens.some(token => token.text === forbidden)).toBe(false);
  }
}, 120000);

const cuts = JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json")) as {version: number; cuts: {producer: string; consumer: string; canonical: string; definition: string}[]; mutationOwners: string[]; forbiddenMutationOwners: string[]};
test("neutral kernel cuts retain exactly one authored owner and producer-facing consumption", () => {
  const validate = new Ajv().compile({type:"object",additionalProperties:false,required:["version","cuts","mutationOwners","forbiddenMutationOwners"],properties:{version:{const:1},cuts:{type:"array",minItems:4,items:{type:"object",additionalProperties:false,required:["producer","consumer","canonical","definition"],properties:{...Object.fromEntries(["producer","consumer","canonical","definition"].map(name=>[name,{type:"string",minLength:1}]))}}},mutationOwners:{type:"array",minItems:3,uniqueItems:true,items:{type:"string"}},forbiddenMutationOwners:{type:"array",minItems:1,uniqueItems:true,items:{type:"string"}}}});
  expect(validate(cuts), JSON.stringify(validate.errors)).toBe(true);
  for (const law of cuts.cuts) {
    const producer = corpus.crates.find(row=>row.package===law.producer)!;
    expect(producer).toBeDefined();
    expect(readModuleTreeSource(resolve(os, producer.entry))).toContain(law.definition);
    expect(read(law.consumer)).toContain(law.canonical);
    expect(read(law.consumer)).not.toContain(law.definition);
  }
  const emission = sliceModuleTreeSource(resolve(os, "🔨️modules/🗣️dsl/✨️derive/🦀️.rs"), "fn expand_mutations(", "fn mutation_relative_path(");
  const owners = rustTokens(emission).filter(token=>token.kind==="identifier").map(token=>token.text);
  for (const owner of cuts.mutationOwners) expect(owners).toContain(owner);
  for (const owner of cuts.forbiddenMutationOwners) expect(owners).not.toContain(owner);
  expect(read("../../🔨️modules/🌱️value/🧪️testing/🧾️wire/🦀️.rs")).toContain("serde_json::Value::from(operation.to_value()), committed");
});
for (const mode of [cfg,["target_arch=x86_64","target_os=linux","target_env=gnu","target_pointer_width=64","unix"],["target_arch=x86_64","target_os=windows","target_env=msvc","target_pointer_width=64","windows"],[...cfg,"test"]]) test("all new kernel owners enforce active source caps for "+mode.join(","),async()=>{
  for (const law of cuts.cuts) {const producer=corpus.crates.find(row=>row.package===law.producer)!;const inputs=await readRustSourceClosureV1([resolve(os,producer.entry)],workspace,mode,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));const caps={maxSourceFiles:32,maxSourceBytes:262144,maxActiveLines:4000};expect(inputs.length).toBeLessThanOrEqual(caps.maxSourceFiles);expect(inputs.reduce((sum,row)=>sum+row.bytes,0)).toBeLessThanOrEqual(caps.maxSourceBytes);expect(inputs.reduce((sum,row)=>sum+row.activeLines,0)).toBeLessThanOrEqual(caps.maxActiveLines);}
},120000);

const enclosing=JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/📦️enclosing.json")) as {version:number;owners:{package:string;manifest:string;entry:string}[]};
test("original enclosing Geometry and DSL owners enforce production and native Source caps",async()=>{
 const validate=new Ajv().compile({type:"object",additionalProperties:false,required:["version","owners"],properties:{version:{const:1},owners:{type:"array",minItems:3,items:{type:"object",additionalProperties:false,required:["package","manifest","entry"],properties:Object.fromEntries(["package","manifest","entry"].map(name=>[name,{type:"string",minLength:1}]))}}}});expect(validate(enclosing)).toBe(true);
 for(const row of enclosing.owners){const source=read(row.manifest);expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));const manifest=Bun.TOML.parse(source) as any;expect(manifest.package.name).toBe(row.package);expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual({maxSourceBytes:262144,maxSourceFiles:32,maxActiveLines:4000});for(const mode of [cfg,[...cfg,"test"],["target_arch=x86_64","target_os=linux","target_env=gnu","target_pointer_width=64","unix","test"],["target_arch=x86_64","target_os=windows","target_env=msvc","target_pointer_width=64","windows","test"]]){const rows=await readRustSourceClosureV1([resolve(os,row.entry)],workspace,mode,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));expect(rows.length).toBeLessThanOrEqual(32);expect(rows.reduce((sum,row)=>sum+row.bytes,0)).toBeLessThanOrEqual(262144);expect(rows.reduce((sum,row)=>sum+row.activeLines,0)).toBeLessThanOrEqual(4000);}}
}, 120000);

const production=JSON.parse(read("🧪️tests/🧫️fixtures/📦️crate-boundaries/📦️production.json")) as {version:number;owners:{package:string;manifest:string;entry:string}[]};
test("original enclosing Value production owner enforces exact Source caps across native targets",async()=>{
 const validate=new Ajv().compile({type:"object",additionalProperties:false,required:["version","owners"],properties:{version:{const:1},owners:{type:"array",minItems:1,items:{type:"object",additionalProperties:false,required:["package","manifest","entry"],properties:Object.fromEntries(["package","manifest","entry"].map(name=>[name,{type:"string",minLength:1}]))}}}});expect(validate(production)).toBe(true);
 for(const row of production.owners){const source=read(row.manifest);expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));const manifest=Bun.TOML.parse(source) as any;expect(manifest.package.name).toBe(row.package);expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual({maxSourceBytes:262144,maxSourceFiles:32,maxActiveLines:4000});for(const mode of [cfg,["target_arch=x86_64","target_os=linux","target_env=gnu","target_pointer_width=64","unix"],["target_arch=x86_64","target_os=windows","target_env=msvc","target_pointer_width=64","windows"]]){const rows=await readRustSourceClosureV1([resolve(os,row.entry)],workspace,mode,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));expect(rows.length).toBeLessThanOrEqual(32);expect(rows.reduce((sum,row)=>sum+row.bytes,0)).toBeLessThanOrEqual(262144);expect(rows.reduce((sum,row)=>sum+row.activeLines,0)).toBeLessThanOrEqual(4000);}}
}, 120000);
