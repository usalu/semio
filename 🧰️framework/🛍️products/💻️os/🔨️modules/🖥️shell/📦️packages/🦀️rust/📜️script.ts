#!/usr/bin/env bun
import {type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🖥️ `@semio-tech/framework-os-shell-rs` task router: `bun ./📜️script.ts <check|test|typegen|schema-check|preview-generated>`. */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import {runRepositoryCargoCommandV1,runRepositoryCargoTests} from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import {launchRepositoryCargoV1,readRepositoryCargoLaunchV1,readRepositoryCargoGeneratedCapturePolicyV1} from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts";
import {readBuildMode} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {OwnedCommandFailure,writeOwnedCommandOutputV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await runRepositoryCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check","--manifest-path","Cargo.toml"],this.root,process.env,buildBudgetMs());
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-os-shell"], this.repoRoot, this.invocation, rest);
  }
}

//#region 🔖️Typegen
/** 🧬️ The `🧬️schema` module is the ONLY source of truth for the mirror: `🔣️.json` is the
 * language-neutral authority and `🧬️schema/🦀️.rs`'s `schema_registry` is its Rust registry. This
 * test renders that registry; nothing in ordinary module code feeds it. */
const TYPEGEN_TEST_FILTER = "schema::tests::exports_typescript_bindings";

/** 🧬️ The `$defs`/registry agreement gate, in the schema module's own tests. */
const SCHEMA_GATE_TEST_FILTER = "schema::tests::owned_json_schema_defs_match_registry";

/** 🧬️ The owner module root — `<owner>/📦️packages/🦀️rust` is where this script runs. */
function ownerRoot(root: string): string {
  return join(root, "..", "..");
}

/** 🧬️ The scope this generator reads from: `<owner>/🧬️schema`. */
function schemaModulePath(root: string, ...segments: string[]): string {
  return join(ownerRoot(root), "🧬️schema", ...segments);
}

/** 🎯️ The mirror this generator writes: `🧰️framework/🔨️modules/🐚️shell/🤖️generated/🟦️.ts`, beside the reducer that consumes it (the renderer reaches both through the framework SDK). */
function generatedBindingsPath(root: string): string {
  return join(ownerRoot(root), "..", "..", "..", "..", "🔨️modules", "🐚️shell", "🤖️generated", "🟦️.ts");
}

/** 🪪️ The `$id` `🧬️schema/🔣️.json` must declare, mirrored from `schema_registry::OWNED_JSON_SCHEMA_ID`. */
const OWNED_JSON_SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/os/shell/component.json";
const OWNED_JSON_SCHEMA_DIALECT = "http://json-schema.org/draft-07/schema#";

async function runTypegenExportTest(root:string,outPath:string,workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
  await runRepositoryCargoCommandV1(invocation,phaseControl,workspace,["test","--features","typegen",TYPEGEN_TEST_FILTER],root,{...process.env,SEMIO_TYPEGEN_OUT:outPath},buildBudgetMs());
}

/** 🧬️ Renders `🧰️framework/🔨️modules/🐚️shell/🤖️generated/🟦️.ts` from `<owner>/🧬️schema` — the schema module's Rust
 * registry, gated against its own `🔣️.json` before a single byte is written. */
class TypegenScript extends BundleScript {
  async run(): Promise<void> {
    assertSchemaModulePresent(this.root);
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    await runTypegenExportTest(this.root,outPath,this.repoRoot,this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:typegen",0));
    for (const name of readdirSync(dirname(outPath))) if (name !== basename(outPath)) rmSync(join(dirname(outPath), name), { recursive: true, force: true });
    console.log(`framework-os-shell typescript mirror refreshed from 🧬️schema -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact exporter in issued Rust capture backing and emits only canonical JSON. */
class PreviewGeneratedScript extends BundleScript {
  async run():Promise<void>{
    const targetPath=generatedBindingsPath(this.root),mode=readBuildMode(process.env.SEMIO_BUILD_MODE),request=readRepositoryCargoLaunchV1(["--store=shared","--configuration="+mode,"--","test","--locked","--features","typegen",TYPEGEN_TEST_FILTER],{...process.env,SEMIO_BUILD_MODE:mode});
    await launchRepositoryCargoV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:typegen-preview",0),this.repoRoot,request,this.root,process.env,{maximumMilliseconds:buildBudgetMs(),output:"capture",limits:{maximumBytes:16777216,maximumLines:65536},generated:readRepositoryCargoGeneratedCapturePolicyV1({version:1,maximumBytes:1048576}),consume:async(result,context)=>{
      if(result.status!==0||result.signal!==null){if(result.stderr)await writeOwnedCommandOutputV1(process.stderr,result.stderr,context.control);throw new OwnedCommandFailure("shell:preview-generated",{version:1,status:result.status,signal:result.signal});}
      if(!context.generated)throw Error("Actual shell generated output absent");
      const rootPath=relative(this.repoRoot,dirname(targetPath)).replaceAll("\\","/").normalize("NFC"),nodes=[{bytesBase64:"",mode:0o755,nodeKind:"directory",path:rootPath},{bytesBase64:Buffer.from(context.generated).toString("base64"),mode:0o644,nodeKind:"file",path:rootPath+"/"+basename(targetPath).normalize("NFC")}].sort((a,b)=>Buffer.from(a.path).compare(Buffer.from(b.path))),staleRemovals=(existsSync(dirname(targetPath))?readdirSync(dirname(targetPath)):[]).filter(name=>name!==basename(targetPath)).map(name=>rootPath+"/"+name.normalize("NFC")).sort((a,b)=>Buffer.from(a).compare(Buffer.from(b)));
      await writeOwnedCommandOutputV1(process.stdout,JSON.stringify({contractId:"shell-typegen",nodes,schemaVersion:1,staleRemovals})+"\n",context.control);
    }});
  }
}
//#endregion 🔖️Typegen

//#region 🧬️SchemaCheck
/** 🧬️ Reads `🔣️.json` and fails loudly if the schema module is absent or off-dialect. */
function readOwnedJsonSchema(root: string): { $schema?: string; $id?: string; $defs?: Record<string, unknown> } {
  const path = schemaModulePath(root, "🔣️.json");
  if (!existsSync(path)) throw new Error(`framework-os-shell: the 🧬️schema module is missing its authority document at ${path}`);
  return JSON.parse(readFileSync(path, "utf8")) as { $schema?: string; $id?: string; $defs?: Record<string, unknown> };
}

function assertSchemaModulePresent(root: string): void {
  for (const name of ["🔣️.json", "🦀️.rs", "🟦️.ts"]) {
    const path = schemaModulePath(root, name);
    if (!existsSync(path)) throw new Error(`framework-os-shell: the 🧬️schema module is missing ${name} at ${path}`);
  }
}

/** 🧾️ Every `SchemaExport { name: "…" }` row the schema module's Rust registry declares. */
function rustRegistryExportIds(root: string): string[] {
  const source = readFileSync(schemaModulePath(root, "🦀️.rs"), "utf8");
  return [...source.matchAll(/SchemaExport \{\s*\n?\s*name: "([A-Za-z0-9_]+)"/gu)].map((match) => match[1]!);
}

/** 🟦️ Every `export type X` the rendered mirror declares. */
function renderedMirrorExportIds(root: string): string[] {
  const path = generatedBindingsPath(root);
  if (!existsSync(path)) throw new Error(`framework-os-shell: the rendered mirror is missing at ${path} — run \`typegen\` first.`);
  return [...readFileSync(path, "utf8").matchAll(/^export type ([A-Za-z0-9_]+)/gmu)].map((match) => match[1]!);
}

/** 🧬️ Every parser covering an owned shell export; canonical config facade reexports stay external. */
function typescriptParserExportIds(root: string, owned: readonly string[]): string[] {
  const source = readFileSync(schemaModulePath(root, "🟦️.ts"), "utf8");
  const local = [...source.matchAll(/^export const parse([A-Za-z0-9_]+) = defineParser</gmu)].map((match) => match[1]!);
  const reexported = [...source.matchAll(/^export \{([^}]+)\} from /gmu)].flatMap((match) => match[1]!.split(",").map((name) => name.trim()).filter((name) => /^parse[A-Za-z0-9_]+$/u.test(name)).map((name) => name.slice("parse".length))).filter((name) => owned.includes(name));
  return [...new Set([...local, ...reexported])];
}

function reportSetDifference(label: string, expected: string[], actual: string[]): string[] {
  const missing = expected.filter((name) => !actual.includes(name));
  const extra = actual.filter((name) => !expected.includes(name));
  const problems: string[] = [];
  if (missing.length > 0) problems.push(`${label} is missing ${JSON.stringify(missing)}`);
  if (extra.length > 0) problems.push(`${label} declares unknown ${JSON.stringify(extra)}`);
  return problems;
}

/** 🧬️ `🔣️.json`'s `$defs` key set and the Rust registry's exported type set MUST agree — checked
 * here across all three projections (authority, Rust registry, rendered mirror, TypeScript face)
 * and then again inside Rust itself, where the registry is the live value rather than source text. */
class SchemaCheckScript extends BundleScript {
  async run(): Promise<void> {
    assertSchemaModulePresent(this.root);
    const document = readOwnedJsonSchema(this.root);
    const problems: string[] = [];
    if (document.$schema !== OWNED_JSON_SCHEMA_DIALECT) problems.push(`🔣️.json declares dialect '${document.$schema}', expected '${OWNED_JSON_SCHEMA_DIALECT}'`);
    if (document.$id !== OWNED_JSON_SCHEMA_ID) problems.push(`🔣️.json declares $id '${document.$id}', expected '${OWNED_JSON_SCHEMA_ID}'`);
    const declared = Object.keys(document.$defs ?? {}).sort();
    if (declared.length === 0) problems.push("🔣️.json declares no `$defs` exports");
    problems.push(...reportSetDifference("the Rust registry in 🧬️schema/🦀️.rs", declared, rustRegistryExportIds(this.root).sort()));
    problems.push(...reportSetDifference("the rendered mirror 🤖️generated/🟦️.ts", declared, renderedMirrorExportIds(this.root).sort()));
    problems.push(...reportSetDifference("the parsers in 🧬️schema/🟦️.ts", declared, typescriptParserExportIds(this.root, declared).sort()));
    if (problems.length > 0) {
      for (const problem of problems) console.error(`framework-os-shell schema-check: ${problem}`);
      throw Error("framework-os-shell schema ownership disagreement");
    }
    await runRepositoryCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["test","--features","typegen",SCHEMA_GATE_TEST_FILTER],this.root,process.env,buildBudgetMs());
    console.log(`framework-os-shell schema-check: ${declared.length} 🔣️.json \`$defs\` exports agree with the Rust registry, the rendered mirror and 🧬️schema/🟦️.ts.`);
  }
}
//#endregion 🧬️SchemaCheck

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("typegen", TypegenScript).register("schema-check", SchemaCheckScript).register("preview-generated", PreviewGeneratedScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
