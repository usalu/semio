import { build } from "esbuild";
import { fileURLToPath, pathToFileURL } from "node:url";
import { test, expect } from "bun:test";
import { readFileSync, writeFileSync, mkdtempSync, mkdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import { buildWasmWebV1, readWasmBuildPolicyV1 } from "../🟦️.ts";
const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
test("portable compiler corpus is admitted by independent Ajv",()=>{ const oracle=new Ajv({strict:true}).addSchema(schema);expect(oracle.getSchema(schema.$id+"#/$defs/Fixture")!(fixture)).toBe(true); });
test("WASM compiler policy is explicit and agrees with independent schema admission",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture.policy)).toBe(true);expect(readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(fixture.policy)},fixture.policy.cwd)).toEqual(fixture.policy);
 for(const row of [{...fixture.policy,mode:"guess"},{...fixture.policy,budgetMs:-1},{...fixture.policy,extra:true},{...fixture.policy,artifactDirectory:"relative"}]){expect(validate(row)).toBe(false);expect(()=>readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(row)},fixture.policy.cwd)).toThrow();}
 expect(()=>readWasmBuildPolicyV1({},fixture.policy.cwd)).toThrow();expect(()=>readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(fixture.policy)},"/foreign")).toThrow();
});
for(const row of fixture.wasm)test(`WASM private compiler storage and package publication: ${row.name}`,async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"wasm-publication-"));let target:string|undefined;const commands:{command:string;args:readonly string[]}[]=[];
 const policy={...fixture.policy,cwd:root,mode:row.mode??"dev",artifactDirectory:join(root,"captures"),buildDirectory:join(root,"compiler"),leaseDirectory:join(root,"leases")};
 const run=async(command:string,args:readonly string[],options:{environment:Readonly<Record<string,string|undefined>>})=>{
 commands.push({command,args});target=options.environment.CARGO_TARGET_DIR;expect(target?.startsWith(policy.artifactDirectory)).toBe(true);expect(options.environment.CARGO_BUILD_BUILD_DIR).toBe(policy.buildDirectory);mkdirSync(target!,{recursive:true});
 if(row.status)throw Error(`compiler failed ${row.status}`);
 if(row.output&&(!row.threads||command==="wasm-bindgen")){mkdirSync(join(root,"pkg","snippets"),{recursive:true});writeFileSync(join(root,"pkg","fixture_bg.wasm"),Buffer.from(fixture.wasmHex,"hex"));writeFileSync(join(root,"pkg","snippets","helper.js"),"export const ready=true;");}
 };
 const invoke=()=>buildWasmWebV1({rsDir:root,logPrefix:"fixture",pkg:{name:"fixture-owner",files:["fixture.js"],main:"fixture.js",module:"fixture.js",types:"fixture.d.ts"},wasmBaseName:"fixture",threads:row.threads,cargoFeatures:["fixture-feature"],noDefaultFeatures:true,devOptimizedCrates:["fixture-layout"]},policy,{run});
 if(row.output){await invoke();const bytes=readFileSync(join(root,"pkg","fixture_bg.wasm"));expect(WebAssembly.validate(bytes)).toBe(true);const native=spawnSync("node",["-e",'const fs=require("node:fs");process.stdout.write(JSON.stringify(WebAssembly.validate(fs.readFileSync(process.argv[1]))));',join(root,"pkg","fixture_bg.wasm")],{encoding:"utf8"});expect(native.status).toBe(0);expect(JSON.parse(native.stdout)).toBe(true);expect(JSON.parse(readFileSync(join(root,"pkg","package.json"),"utf8"))).toMatchObject({name:"fixture-owner",main:"fixture.js",types:"fixture.d.ts",files:["fixture.js","snippets/helper.js"]});}else await expect(invoke()).rejects.toThrow(/failed|missing/);
 expect(target&&!existsSync(target)).toBe(true);expect(commands[0]!.args).toContain("--no-default-features");expect(commands[0]!.args).toContain("fixture-feature");if(row.mode!=="ship")expect(commands[0]!.args).toContain("profile.dev.package.fixture-layout.opt-level=3");if(row.threads){expect(commands[0]!.command).toBe("cargo");expect(commands[1]!.args[0]).toBe(join(target!,"wasm32-unknown-unknown","debug","fixture_package.wasm"));}
});

for(const owner of [
 {name:"wasm",source:new URL("../🟦️.ts",import.meta.url),expression:`readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(${JSON.stringify(fixture.policy)})},${JSON.stringify(fixture.policy.cwd)}).packageName`,expected:fixture.policy.packageName},
 {name:"native",source:new URL("../../🏗️native-build/🟦️.ts",import.meta.url),expression:'readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify({version:1,cwd:"/owner",buildDirectory:"/compiler",leaseDirectory:"/leases",captureDirectory:"/captures",budgetMs:0})},"/owner").budgetMs',expected:0},
 {name:"exact",source:new URL("../../../🧪️testing/🦀️cargo/🎯️exact/🟦️.ts",import.meta.url),expression:'exactExecutableFingerprint(process.argv[1]).sha256',expected:null},
])test(`neutral compiler owner executes in Node with all product imports refused: ${owner.name}`,async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"node-compiler-owner-")), executable=join(root,"fixture");writeFileSync(executable,Buffer.from(fixture.wasmHex,"hex"),{mode:0o700});
 const compiled=await build({entryPoints:[fileURLToPath(owner.source)],bundle:true,write:false,format:"esm",platform:"node",metafile:true,define:{"import.meta.url":JSON.stringify(pathToFileURL(fileURLToPath(owner.source)).href)},plugins:[{name:"refuse-product-imports",setup(build){build.onResolve({filter:/.*/},args=>/🛍️products|✏️s|🌎️hub/u.test(args.path)?{errors:[{text:`Product dependency refused: ${args.path}`}]}:undefined);}}]});
 expect(Object.keys(compiled.metafile!.inputs).every(path=>!/[🛍]|✏️s|🌎️hub/u.test(path))).toBe(true);
 const result=spawnSync("node",["--input-type=module","-e",compiled.outputFiles![0]!.text+`\nprocess.stdout.write(JSON.stringify(${owner.expression}));`,executable],{encoding:"utf8",timeout:5000});expect(result.status).toBe(0);expect(result.stderr).toBe("");
 const expected=owner.expected??Buffer.from(await crypto.subtle.digest("SHA-256",Buffer.from(fixture.wasmHex,"hex"))).toString("hex");expect(JSON.parse(result.stdout)).toBe(expected);
});
