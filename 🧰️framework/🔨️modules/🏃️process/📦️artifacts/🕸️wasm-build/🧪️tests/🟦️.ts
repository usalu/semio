import {registeredToolExecutableV1} from "../../../🧰️tools/🏃️run/🟦️.ts";
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

test("the retained WASM consumer retries only transient observation changes against the original identity",async()=>{
 const {lstat,mkdir,mkdtemp,readFile,rm}=await import("node:fs/promises");
 const {createHash}=await import("node:crypto");
 const {ScriptTarget,transpileModule}=await import("typescript");
 const {FileObservationErrorV1}=await import("../../../../📁️filesystem/🧾️observation/🟦️.ts");
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),memberCode=source.slice(source.indexOf("declare const wasmPackageBrand:")).replace(/^export /gm,"");
 const code=transpileModule(memberCode,{compilerOptions:{target:ScriptTarget.ES2022}}).outputText;
 const root=process.env.SEMIO_TEST_ARTIFACT_DIR!;
 await mkdir(root,{recursive:true});
 const directory=await mkdtemp(join(root,"wr-"));
 try{
  for(const row of fixture.packageRecheck){
   const path=join(directory,row.name),expectedBytes=Buffer.from(fixture.wasmHex,"hex");
   writeFileSync(path,expectedBytes);
   const expectedHash=new Bun.CryptoHasher("sha256").update(expectedBytes).digest("hex");
   let probing=false,checks=0,observations=0;
   const stat=async(path:string)=>{
    const native=await lstat(path,{bigint:true});
    if(!probing)return native;
    checks++;
    if(row.change==="persistent-timestamp"||row.change==="transient-timestamp"&&checks===1)return Object.assign(native,{mtimeNs:native.mtimeNs+1n});
    if(row.change==="transient-inode"&&checks===1)return Object.assign(native,{ino:native.ino+1n});
    return native;
   };
   const observe=async(path:string)=>{
    let input=await readFile(path);
    if(probing){
     observations++;
     if(row.change==="persistent-observation"||row.change==="transient-observation"&&observations===1)throw new FileObservationErrorV1("changed",path);
     if(row.change==="io")throw new FileObservationErrorV1("io",path);
     if(row.change==="different-digest"){input=Buffer.from(input);input[0]^=1;}
    }
    const sha256=createHash("sha256").update(input).digest("hex");
    if(!probing||row.change!=="different-digest")expect(sha256).toBe(expectedHash);
    return{path,sha256,byteLength:input.length};
   };
   const noOp=()=>undefined,run=(_:unknown,body:()=>Promise<unknown>)=>body();
   const dependencies={lstat:stat,observePhysicalFileV1:observe,FileObservationErrorV1,withArtifactDescriptorSlotV1:(body:()=>Promise<unknown>)=>body(),requireIssuedResourceControlV1:noOp,requireNativeStorageReservationV1:noOp,requireNativeStoragePathV1:noOp,validateJsonSchemaSubset:()=>[],checkResourceLeaseControl:noOp,withIssuedResourceOperationV1:run,sourceSchema:{properties:{maximumFiles:{const:1},maximumBytes:{const:expectedBytes.length},chunkBytes:{const:1},maximumPendingOperations:{const:1}},definitions:{}}};
   const {consumeProducedWasmPackageV1:consume,withProducedWasmPackageTaskV1:task}=new Function(...Object.keys(dependencies),code+";return {consumeProducedWasmPackageV1,withProducedWasmPackageTaskV1};")(...Object.values(dependencies));
   const attempt=consume(new Map([[row.name,path]]),{},{
    storage:{backing:{root:directory},recheck:noOp},control:{signal:new AbortController().signal},
    consume:(token:unknown)=>task(token,async(view:{recheck():Promise<void>;sources:unknown})=>{expect(view.sources).toEqual(fixture.provenance);probing=true;try{await view.recheck();}finally{probing=false;}}),
   },noOp,fixture.provenance);
   if(row.expected.length){
    const failure=await attempt.catch((error:Error)=>error);
    expect(failure).toBeInstanceOf(Error);
    expect(failure.message).toContain(row.name);
    for(const field of row.expected)expect(failure.message).toContain(field);
   }else await attempt;
   expect({observations,checks},row.change).toEqual({observations:row.observations,checks:row.checks});
  }
 }finally{await rm(directory,{recursive:true,force:true});}
});
for(const owner of [
 {name:"wasm",source:new URL("../🟦️.ts",import.meta.url),expression:`readWasmBuildPolicyV1({SEMIO_WASM_BUILD_POLICY:JSON.stringify(${JSON.stringify(fixture.policy)})},${JSON.stringify(fixture.policy.cwd)}).packageName`,expected:fixture.policy.packageName},
 {name:"native",source:new URL("../../🏗️native-build/🟦️.ts",import.meta.url),expression:'readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify({version:1,mode:"prototype",cwd:"/owner",buildDirectory:"/compiler",leaseDirectory:"/leases",captureDirectory:"/captures",budgetMs:0})},"/owner").budgetMs',expected:0},
 {name:"exact",source:new URL("../../../🧪️testing/🦀️cargo/🎯️exact/🟦️.ts",import.meta.url),expression:'exactExecutableFingerprint(process.argv[1]).sha256',expected:null},
])test(`neutral compiler owner executes in Node with all product imports refused: ${owner.name}`,async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"node-compiler-owner-")), executable=join(root,"fixture");writeFileSync(executable,Buffer.from(fixture.wasmHex,"hex"),{mode:0o700});
 const compiled=await build({entryPoints:[fileURLToPath(owner.source)],bundle:true,write:false,format:"esm",platform:"node",metafile:true,define:{"import.meta.url":JSON.stringify(pathToFileURL(fileURLToPath(owner.source)).href)},plugins:[{name:"refuse-product-imports",setup(build){build.onResolve({filter:/.*/},args=>/🛍️products|✏️s|🌎️hub/u.test(args.path)?{errors:[{text:`Product dependency refused: ${args.path}`}]}:undefined);}}]});
 expect(Object.keys(compiled.metafile!.inputs).every(path=>!/[🛍]|✏️s|🌎️hub/u.test(path))).toBe(true);
 const result=spawnSync(registeredToolExecutableV1("node"),["--input-type=module","-"],{encoding:"utf8",timeout:30000,input:compiled.outputFiles![0]!.text+`\nprocess.argv[1]=${JSON.stringify(executable)};process.stdout.write(JSON.stringify(${owner.expression}));`});expect(result.status).toBe(0);expect(result.stderr).toBe("");
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
 const inputs=at("await withCargoCompilerInputsV1("),lease=at("leased=await acquireCargoBuildLeaseV1("),writerEntry=at("await actual.withWriter("),admission=at("await port.admit("),consume=at("await consumeProducedWasmPackageV1(produced.files");
 expect(inputs<lease&&lease<writerEntry&&writerEntry<admission&&admission<consume).toBe(true);
 expect(source.slice(writerEntry,consume)).not.toContain("consumeProducedWasmPackageV1(");
});

test("retained WASM package reports the file and changed identity facts",async()=>{
 const {lstat,link,mkdir,mkdtemp,rename,rm,utimes,writeFile}=await import("node:fs/promises");
 const {createSourceFile,isClassDeclaration,isFunctionDeclaration,ScriptTarget,transpileModule}=await import("typescript");
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=createSourceFile("wasm-build.ts",source,ScriptTarget.Latest,true);
 const owner=tree.statements.filter(statement=>(isClassDeclaration(statement)||isFunctionDeclaration(statement))&&["WasmPackageIdentityErrorV1","readProducedWasmIdentityV1"].includes(statement.name?.text??""));
 expect(owner).toHaveLength(2);
 const code=transpileModule(owner.map(statement=>statement.getText(tree)).join("\n"),{compilerOptions:{target:ScriptTarget.ES2022}}).outputText;
 const observe=new Function("lstat",code+";return readProducedWasmIdentityV1")(lstat) as (name:string,path:string,before?:unknown)=>Promise<unknown>;
 const root=process.env.SEMIO_TEST_ARTIFACT_DIR!;
 await mkdir(root,{recursive:true});
 const directory=await mkdtemp(join(root,"wi-"));
 try{
  for(const row of fixture.packageIdentity){
   const path=join(directory,row.name),replacement=path+".new",knownBytes=Buffer.from(fixture.wasmHex,"hex");
   await writeFile(path,knownBytes);
   const before=await observe(row.name,path);
   if(row.change==="retime")await utimes(path,new Date(0),new Date(0));
   if(row.change==="replace"){await writeFile(replacement,knownBytes);await rename(replacement,path);}
   if(row.change==="hardlink")await link(path,replacement);
   if(row.change==="remove")await rm(path);
   const native=await lstat(path,{bigint:true}).catch(()=>undefined);
   if(native)expect(Buffer.from(await Bun.file(path).arrayBuffer())).toEqual(knownBytes);
   if(!row.expected.length){expect(await observe(row.name,path,before)).toBeDefined();continue;}
   const failure=await observe(row.name,path,before).catch((error:Error)=>error);
   expect(failure).toBeInstanceOf(Error);
   expect((failure as Error).message).toContain(row.name);
   for(const field of row.expected)expect((failure as Error).message).toContain(field);
  }
 }finally{await rm(directory,{recursive:true,force:true});}
});
