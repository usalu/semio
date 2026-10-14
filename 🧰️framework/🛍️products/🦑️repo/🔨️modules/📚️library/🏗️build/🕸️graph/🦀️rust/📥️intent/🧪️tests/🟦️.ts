import {test,expect} from "bun:test";
import Ajv from "ajv";
import {build} from "esbuild";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fallbackSchema from "../🧬️schema/📥️fallback.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {readRustBuildIntentV1,rustBuildIntentArgumentsV1,qualifyRustBuildFallbackV1,readRustBuildFallbackDecisionV1} from "../🟦️.ts";
import {declaredRustBuildArgumentsV1,declaredRustBuildFallbackV1} from "../🏃️routing/🟦️.ts";
import {selectedCargoBuildOwnerV1,selectedRustAggregateTargetV1} from "../../🟦️.ts";

test("ordinary Rust build entry Source excludes the catalog barrel while retaining finite test ownership",async()=>{
 for(const row of fixture.routes){const declaration=JSON.parse(readFileSync(row.declaration,"utf8")),script=readFileSync(resolve(row.declaration,"../📜️script.ts"),"utf8");expect(script).not.toContain("📚️library/📦️packages/🟦️typescript/🟦️.ts");}
 const yaml=fixture.routes.find(row=>row.project==="@semio-tech/repo-yaml-rs")!,oracle=await build({entryPoints:[resolve(yaml.declaration,"../📜️script.ts")],bundle:true,metafile:true,write:false,platform:"node",format:"esm",packages:"external",logLevel:"silent"}),inputs=Object.keys(oracle.metafile!.inputs);expect(inputs.some(path=>path.endsWith("📚️library/🟦️.ts"))).toBe(false);expect(inputs.some(path=>path.includes("📇️registry/🔍️discovery"))).toBe(false);expect(inputs.some(path=>path.includes("🦀️cargo/🎟️admission/🟦️.ts"))).toBe(true);console.log("[DEBUG] Independent esbuild ordinary YAML Source excludes catalog barrel and retains actual bounded Cargo test owner "+inputs.length);
});

test("closed selected Cargo build intent matches portable schema and retains package scope",()=>{
 const oracle=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.accepted){expect(oracle(row.intent)).toBe(true);expect(rustBuildIntentArgumentsV1(readRustBuildIntentV1(row.intent))).toEqual(row.arguments);}
 for(const row of fixture.rejected){expect(oracle(row)).toBe(false);expect(()=>readRustBuildIntentV1(row)).toThrow("Cargo build intent");}
 console.log("[DEBUG] Selected Cargo build intent matches third-party schema admission and preserves package/library/binary/features/target declaration facts");
});

test("ordinary CLI and dashboard compile declarations preserve their exact binary deliverables and installation barrier",()=>{
 for(const row of fixture.appRoutes){const declaration=JSON.parse(readFileSync(row.declaration,"utf8")),target=declaration.targets[row.target],intent=readRustBuildIntentV1(target?.metadata?.semio?.rustBuildIntent);expect(intent.scope).toBe(row.scope);expect(intent.binary).toBe("binary" in row?row.binary:undefined);expect(target.cache).toBe(false);expect(selectedCargoBuildOwnerV1({execution:{kind:"command",command:target.options.command},metadata:target.metadata},process.cwd())).toBe(resolve(intent.manifest));}
 const dashboard=JSON.parse(readFileSync(fixture.appRoutes[2]!.declaration,"utf8"));expect(dashboard.targets.build.dependsOn).toBeUndefined();expect(dashboard.targets.install.dependsOn).toEqual(["build"]);expect(dashboard.targets.build.cache).toBe(false);
 console.log("[DEBUG] CLI default package and semio-repo-only startup scopes remain exact; dashboard binary compilation precedes its uncached installation barrier");
});

test("bounded Cargo fallback requires every typed unsupported adapter and rejects changed intent identity",()=>{
 const intent=readRustBuildIntentV1(fixture.fallback.intent),decision=qualifyRustBuildFallbackV1(intent,fixture.fallback.unsupported)!;
 expect(new Ajv({strict:true}).compile(fallbackSchema)(decision)).toBe(true);
 expect(decision.adapters).toEqual(fixture.fallback.expected);expect(readRustBuildFallbackDecisionV1(decision,intent)).toEqual(decision);expect(qualifyRustBuildFallbackV1(intent,[])).toBeUndefined();
 for(const unsupported of fixture.fallback.rejected)expect(()=>qualifyRustBuildFallbackV1(intent,unsupported)).toThrow("fallback");
 expect(()=>qualifyRustBuildFallbackV1({...intent,fallback:undefined},fixture.fallback.unsupported)).toThrow("fallback");expect(()=>readRustBuildFallbackDecisionV1(decision,{...intent,package:"different"})).toThrow("fallback");
 const project="@semio-tech/repo-yaml-rs",target="build",environment={NX_TASK_TARGET_PROJECT:project,NX_TASK_TARGET_TARGET:target,SEMIO_RUST_BUILD_FALLBACK:JSON.stringify({...decision,project,target})};expect(declaredRustBuildFallbackV1(project,target,intent,environment)).toEqual(decision);expect(()=>declaredRustBuildFallbackV1(project,target,intent,{...environment,NX_TASK_TARGET_PROJECT:"other"})).toThrow("fallback");expect(()=>declaredRustBuildFallbackV1(project,"build-cli",intent,environment)).toThrow("fallback");
 console.log("[DEBUG] Bounded Cargo fallback permits exactly declared unavailable adapter categories, keeps supported units selected, and rejects stale intent identity");
});

test("every ordinary Repo Rust package build declares its actual package and redirects without recursive Cargo dispatch",()=>{
 expect(fixture.routes.length).toBe(21);
 for(const route of fixture.routes){
  const declaration=JSON.parse(readFileSync(route.declaration,"utf8")),build=declaration.targets.build,intent=readRustBuildIntentV1(build.metadata?.semio?.rustBuildIntent),manifest=readFileSync(route.manifest,"utf8"),script=readFileSync(route.manifest.replace("Cargo.toml","📜️script.ts"),"utf8");
  expect(declaration.name).toBe(route.project);expect(intent.manifest).toBe(route.manifest);expect(intent.package).toBe(manifest.match(/^name = "([^"]+)"/mu)?.[1]);expect(intent.scope).toBe("package");expect(script).toContain('await runDeclaredRustBuildV1(this.repoRoot, '+JSON.stringify(route.project));expect(build.cache).toBe(false);expect(build.options.command).toBe("bun ./📜️script.ts build");expect(declaredRustBuildArgumentsV1(route.project,[],{})).toEqual(["run",route.project+":build"]);expect(()=>declaredRustBuildArgumentsV1(route.project,[],{NX_TASK_TARGET_PROJECT:route.project,NX_TASK_TARGET_TARGET:"build"})).toThrow("selected graph");
  const original={execution:{kind:"command" as const,command:build.options.command,cwd:build.options.cwd},metadata:build.metadata},owner=selectedCargoBuildOwnerV1(original,process.cwd());expect(owner).toBe(resolve(route.manifest));for(const mode of ["prototype","release"]){const aggregate=selectedRustAggregateTargetV1(original,owner!,mode,["rust-unit-declaration"]);expect(aggregate.execution).toEqual({kind:"noop"});expect(aggregate.cache).toBe(false);expect(aggregate.dependsOn).toEqual(["rust-unit-declaration:compile"]);expect(selectedCargoBuildOwnerV1(aggregate,process.cwd())).toBe(owner);expect((aggregate.metadata as any).semio.rustBuildIntent).toEqual(intent);}
 }
 console.log("[DEBUG] All 21 actual Repo package build declarations retain Cargo default deliverables and canonical Nx redirect refuses failed graph consumption before Cargo writes");
});
