import { build } from "esbuild";
import { fileURLToPath, pathToFileURL } from "node:url";
import { test, expect } from "bun:test";
import { readFileSync, writeFileSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import { readWasmBuildPolicyV1 } from "../🟦️.ts";
const fixture=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
test("WASM compiler policy is explicit and agrees with independent schema admission",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture.policy)).toBe(true);expect(readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(fixture.policy)},fixture.policy.cwd)).toEqual(fixture.policy);
 for(const row of [{...fixture.policy,mode:"guess"},{...fixture.policy,mode:"dev"},{...fixture.policy,mode:"ship"},{...fixture.policy,budgetMs:-1},{...fixture.policy,extra:true},{...fixture.policy,artifactDirectory:"relative"}]){expect(validate(row)).toBe(false);expect(()=>readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(row)},fixture.policy.cwd)).toThrow();}
 expect(()=>readWasmBuildPolicyV1({},fixture.policy.cwd)).toThrow();expect(()=>readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(fixture.policy)},"/foreign")).toThrow();
});
for(const owner of [
 {name:"wasm",source:new URL("../🟦️.ts",import.meta.url),expression:`readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(${JSON.stringify(fixture.policy)})},${JSON.stringify(fixture.policy.cwd)}).packageName`,expected:fixture.policy.packageName},
 {name:"native",source:new URL("../../🏗️native-build/🟦️.ts",import.meta.url),expression:'readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify({version:1,mode:"prototype",cwd:"/owner",buildDirectory:"/compiler",leaseDirectory:"/leases",captureDirectory:"/captures",budgetMs:0})},"/owner").budgetMs',expected:0},
 {name:"exact",source:new URL("../../../🧪️testing/🦀️cargo/🎯️exact/🟦️.ts",import.meta.url),expression:'exactExecutableFingerprint(process.argv[1]).sha256',expected:null},
])test(`neutral compiler owner executes in Node with all product imports refused: ${owner.name}`,async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"node-compiler-owner-")), executable=join(root,"fixture");writeFileSync(executable,Buffer.from(fixture.wasmHex,"hex"),{mode:0o700});
 const compiled=await build({entryPoints:[fileURLToPath(owner.source)],bundle:true,write:false,format:"esm",platform:"node",metafile:true,define:{"import.meta.url":JSON.stringify(pathToFileURL(fileURLToPath(owner.source)).href)},plugins:[{name:"refuse-product-imports",setup(build){build.onResolve({filter:/.*/},args=>/🛍️products|✏️s|🌎️hub/u.test(args.path)?{errors:[{text:`Product dependency refused: ${args.path}`}]}:undefined);}}]});
 expect(Object.keys(compiled.metafile!.inputs).every(path=>!/[🛍]|✏️s|🌎️hub/u.test(path))).toBe(true);
 const result=spawnSync("node",["--input-type=module","-"],{encoding:"utf8",timeout:30000,input:compiled.outputFiles![0]!.text+`\nprocess.argv[1]=${JSON.stringify(executable)};process.stdout.write(JSON.stringify(${owner.expression}));`});expect(result.status).toBe(0);expect(result.stderr).toBe("");
 const expected=owner.expected??Buffer.from(await crypto.subtle.digest("SHA-256",Buffer.from(fixture.wasmHex,"hex"))).toString("hex");expect(JSON.parse(result.stdout)).toBe(expected);
});


/** ⏱️ The deadline of a long wasm build is the renewable progress window of the native-child class, not a fixed wall clock: a 25-50 min build on a loaded host that keeps compiling must not die on `Original resource loan deadline exhausted`. */
test("the wasm build derives its deadline from the native-child progress policy, renewed by child progress and bounded by the declared budget",async()=>{
 const{progressDeadlinePolicyV1}=await import("../../../⏱️budget/🟦️.ts"),source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8");
 expect(source).toContain("renewableCompilerAdmissionControlV1(opts.control, progressDeadlinePolicyV1(\"native-child\"");expect(source).not.toMatch(/requestedCompilerAdmissionControlV1\(opts\.control, policy\.budgetMs\)/u);
 const policy=progressDeadlinePolicyV1("native-child",0,{},{maximumMilliseconds:fixture.policy.budgetMs>0?fixture.policy.budgetMs:21600000});
 expect(policy.idleMilliseconds).toBeGreaterThan(60*60000-1);expect(policy.maximumMilliseconds).toBeLessThanOrEqual(fixture.policy.budgetMs>0?fixture.policy.budgetMs:21600000);expect(policy.maximumMilliseconds).toBeGreaterThan(60*60000);
});

/** 🔒️ The wasm build takes its leases in ascending class rank: configuration view (toolchain 1), then the cargo build lease (cargo-coordination 2), then the compiler writer (image 7) around the compile only, and the artifact capture (artifact 6) AFTER the writer is released; the old order held the build lease before the view and the writer while it captured, which the lease-order rule reports and which can deadlock against an exclusive writer. */
test("the wasm build acquires configuration view, build lease, writer and artifact capture in an order the lease-order policy accepts",async()=>{
 const order=await import("../../../🔒️leases/🧭️order/🟦️.ts"),policy=order.leaseOrderPolicyV1(),rank=(resource:string)=>order.classifyLeaseResourceV1(policy,resource).rank;
 const view=rank("cargo:configuration-view:7"),build=rank('["cargo-target-recipe-v1","/x/u/l/p1/t/f"]'),writer=rank("repo:compiler-writer:/x/fixed/m"),capture=rank("artifact-capture:/x/browser/prototype/wasm-packages/abc");
 expect([view,build,writer,capture]).toEqual([1,2,7,6]);expect(view<build&&build<writer&&capture<writer).toBe(true);
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),at=(needle:string)=>{const index=source.indexOf(needle);expect(index,needle).toBeGreaterThan(0);return index;};
 const inputs=at("await withCargoCompilerInputsV1("),lease=at("leased=await acquireCargoBuildLeaseV1("),writerEntry=at("await actual.withWriter(async()=>{"),consume=at("await consumeProducedWasmPackageV1(produced.files");
 expect(inputs<lease&&lease<writerEntry&&writerEntry<consume).toBe(true);
 expect(source.slice(writerEntry,consume)).not.toContain("consumeProducedWasmPackageV1(");
});
