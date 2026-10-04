import { inspectRustCompileReferences, type RustCompileReference } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { expect, test } from "bun:test";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { normalize } from "pathe";
import {inspectRustModuleGraph,inspectRustModuleGraphFacts,rustModuleScopeProof} from "../../../🔍️discovery/🟦️.ts";
import { rustSourceTargets } from "../../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
import { inspectRustSourceInputs } from "../../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";

type Case = Readonly<{ id: string; source: string; files: Readonly<Record<string, string>>; nativeInputs: readonly string[]; result: Readonly<{ state: "resolved"; references: readonly RustCompileReference[] } | { state: "unsupported-expression" }>; removal: Readonly<{ paths: readonly string[]; native: boolean }> | null }>;
const library = resolve(import.meta.dir, "../../.."), read = (path: string): unknown => JSON.parse(readFileSync(join(library, path), "utf8"));
const corpus = read("🧫️fixtures/🧱️rust-source-direction/🧾️attributes/🔣️.json") as { readonly schemaVersion: 1; readonly featureCases:readonly Readonly<{id:string;source:string;modulePath:readonly string[];resolved:boolean;files:Readonly<Record<string,string>>;mountPath:readonly string[];mountResolved:boolean;native:boolean;diagnostic:string}>[]; readonly cases: readonly Case[] };

test("attribute input corpus follows its closed portable schema", () => {
  const validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🧾️attributes/🔣️.json") as object);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  const graphSchema=read("🧬️schema/🧱️rust-source-direction/🔣️.json") as {$schema:string;$defs:Record<string,unknown>};
  const scopeValid=new Ajv({strict:true}).compile({$schema:graphSchema.$schema,$defs:graphSchema.$defs,$ref:"#/$defs/moduleScopeFact"});
  expect(scopeValid({kind:"root",modulePath:[],bodyStartOffset:0,bodyEndOffset:1,crateCapabilities:["unknown"]})).toBe(false);
  expect(scopeValid({kind:"inline",modulePath:["subject"],bodyStartOffset:0,bodyEndOffset:1,crateCapabilities:["associated_type_defaults"]})).toBe(false);
  expect(new Set(corpus.featureCases.map(row=>row.id)).size).toBe(corpus.featureCases.length);
  for(const row of corpus.featureCases){
   for(const scope of inspectRustModuleGraphFacts(row.source).scopes)expect(scopeValid(scope),JSON.stringify(scopeValid.errors)).toBe(true);
   expect(rustModuleScopeProof(inspectRustModuleGraphFacts(row.source),row.modulePath).state==="resolved",row.id).toBe(row.resolved);
   const sources=new Map([["Cargo.toml",'[package]\nname="feature_oracle"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="source.rs"'],["source.rs",row.source],...Object.entries(row.files)]);
   const graph=inspectRustModuleGraph([...sources.keys()],path=>sources.get(path),{strictManifests:true});
   expect([...graph.contexts.values()].flat().some(context=>context.modulePath.join("::")===row.mountPath.join("::")),row.id).toBe(row.mountResolved);
  }
  for (const row of corpus.cases) expect(row.removal?.paths.every((path) => Object.hasOwn(row.files, path)) ?? true, row.id).toBe(true);
});

for (const row of corpus.cases) test(row.id, async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust attribute oracle requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-attribute-")));
  const filename = process.platform === "win32" ? "oracle.exe" : "oracle", binary = join(root, filename);
  const compile = async (): Promise<{ status: number; stdout: string; stderr: string }> => {
    const child = Bun.spawn(["rustc", "--edition=2021", "--crate-name", "attribute_oracle", "--emit=dep-info=dependencies.d,link", "-o", filename, "source.rs"], { cwd: root, stdout: "pipe", stderr: "pipe" });
    const [status, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
    return { status, stdout, stderr };
  };
  try {
    writeFileSync(join(root, "source.rs"), row.source);
    for (const [path, content] of Object.entries(row.files)) writeFileSync(join(root, path), content);
    for (const [path, content] of Object.entries(row.files)) expect(readFileSync(join(root, path), "utf8"), row.id).toBe(content);
    const compiled = await compile();
    expect(compiled.status, row.id + ": " + compiled.stdout + compiled.stderr).toBe(0);
    const dependencies = readFileSync(join(root, "dependencies.d"), "utf8").split(/\r?\n/u).find((line) => line.startsWith(filename + ":"));
    expect(dependencies, row.id).toBeDefined();
    expect(dependencies!.slice(filename.length + 1).trim().split(/\s+/u).map(normalize).sort(), row.id).toEqual([...row.nativeInputs].sort());
    const runtime = Bun.spawn([binary], { cwd: root, stdout: "pipe", stderr: "pipe" });
    const [runtimeOut, runtimeErr, runtimeStatus] = await Promise.all([new Response(runtime.stdout).text(), new Response(runtime.stderr).text(), runtime.exited]);
    expect(runtimeStatus, runtimeErr).toBe(0);
    expect(runtimeOut.replaceAll("\r\n", "\n")).toBe("[DEBUG] attribute oracle\n");
    if (row.result.state === "unsupported-expression") {
      expect(() => inspectRustCompileReferences(row.source), row.id).toThrow("Unsupported Rust compile attribute expression");
    } else {
      const references = inspectRustCompileReferences(row.source), targets = rustSourceTargets("source.rs", references), sources = new Set(["source.rs"]);
      expect(references, row.id).toEqual(row.result.references);
      expect(await inspectRustSourceInputs(root, targets, sources), row.id).toEqual([]);
      if (row.removal) {
        for (const path of row.removal.paths) { rmSync(join(root, path)); expect(() => lstatSync(join(root, path)), row.id).toThrow(); }
        const after = await compile();
        expect(after.status === 0, row.id + ": " + after.stdout + after.stderr).toBe(row.removal.native);
        if (!row.removal.native) expect(after.stderr).toContain("couldn't read");
        expect(readFileSync(join(root, "source.rs"), "utf8"), row.id).toBe(row.source);
        expect(inspectRustCompileReferences(row.source), row.id).toEqual(row.result.references);
        expect(await inspectRustSourceInputs(root, targets, sources), row.id).toEqual(row.result.references.filter((reference) => row.removal!.paths.includes(reference.path)).map((reference) => ({ code: "missing-input", to: reference.path, kind: reference.kind, line: reference.line })));
      }
    }
    console.log("[DEBUG] attribute input oracle; id=" + row.id + "; native=" + compiled.status + "; scanner=" + row.result.state);
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

for(const row of corpus.featureCases)test("compiler feature scope "+row.id,()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Feature oracle requires caller-owned output");mkdirSync(output,{recursive:true});
 const root=mkdtempSync(join(output,"rust-feature-"));
 try{
  writeFileSync(join(root,"source.rs"),row.source);
  for(const [path,source]of Object.entries(row.files))writeFileSync(join(root,path),source);
  const result=Bun.spawnSync(["rustc","--edition=2021","--crate-name=feature_oracle","--emit=metadata","source.rs","-o","oracle.rmeta"],{cwd:root,stdout:"pipe",stderr:"pipe",timeout:4_000});
  if(result.stderr===undefined)throw Error("Feature oracle requires compiler diagnostics");
  expect(result.exitCode===0,result.stderr.toString()).toBe(row.native);if(!row.native)expect(result.stderr.toString()).toContain("error["+row.diagnostic+"]");
  expect(rustModuleScopeProof(inspectRustModuleGraphFacts(row.source),row.modulePath).state==="resolved").toBe(row.resolved);
  console.log("[DEBUG] feature scope oracle; id="+row.id+"; native="+result.exitCode+"; resolved="+row.resolved);
 }finally{rmSync(root,{recursive:true,force:true});}
},45_000);
