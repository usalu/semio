import Ajv from "ajv";
/** 🧾️ Normal MCP producer and warm-consumer publication identity law. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import ts from "typescript";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { posix, win32 } from "node:path";
import vectors from "./🧫️fixtures/🔣️.json";

const mcp = readFileSync(new URL("../../../../🟦️.ts", import.meta.url), "utf8");
const binary = readFileSync(new URL("../../../../📦️binary/🟦️.ts", import.meta.url), "utf8");
const producer = readFileSync(new URL("../../../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), "utf8");
const projection = readFileSync(new URL("../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📤️published/🟦️.ts", import.meta.url), "utf8");
const ast = ts.createSourceFile("mcp.ts", mcp, ts.ScriptTarget.Latest, true);
const buildAst = ts.createSourceFile("build.ts", producer, ts.ScriptTarget.Latest, true);
const declarations = (names: readonly string[]) => ast.statements.filter(node => ts.isFunctionDeclaration(node) ? names.includes(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some(value => ts.isIdentifier(value.name) && names.includes(value.name.text)));
const compile = (text: string) => ts.transpileModule(text.replace(/^export /gmu, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
const binaryAst = ts.createSourceFile("binary.ts", binary, ts.ScriptTarget.Latest, true);
const binaryCode = compile(binaryAst.statements.filter(node => ts.isFunctionDeclaration(node) ? ["pathApi", "resolveMcpBinaryPath"].includes(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some(value => ts.isIdentifier(value.name) && ["MCP_ARTIFACT_REL", "MCP_BINARY_NAME"].includes(value.name.text))).map(node => node.getText(binaryAst)).join("\n"));
const publicationAst = ts.createSourceFile("published.ts", projection, ts.ScriptTarget.Latest, true);
const publication = publicationAst.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "publishedCargoArtifactDirectoryV1");
if (!publication) throw Error("Actual publication projection required");
const build = buildAst.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "BuildScript");
if (!build || !ts.isClassDeclaration(build)) throw Error("Actual normal MCP BuildScript required");
const run = build.members.find(node => ts.isMethodDeclaration(node) && node.name.getText(buildAst) === "run");
if (!run) throw Error("Actual MCP build method required");
const nativeProducerAst=ts.createSourceFile("producer.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
const nativeProducer=nativeProducerAst.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="buildMcpArtifactV1");if(!nativeProducer)throw Error("Actual native producer required");
const calls: ts.CallExpression[] = [];
const collect = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.getText(nativeProducerAst) === "buildAdmittedCargoArtifactsV1") calls.push(node); ts.forEachChild(node, collect); };
collect(nativeProducer);

test("closed publication vectors agree with the independent SQLite path oracle", () => {
  expect(Object.keys(vectors).sort()).toEqual(["cases", "version"]);
  expect(vectors.version).toBe(1);
  expect(vectors.cases.map(row => row.id)).toEqual(["linux-prototype", "linux-release", "darwin-prototype", "darwin-release", "win32-prototype", "win32-release"]);
  for (const row of vectors.cases) {
    expect(Object.keys(row).sort()).toEqual(["arguments", "backingRoot", "binary", "id", "manifest", "manifestKey", "mode", "platform", "publication", "repoRoot", "target"]);
    expect(row.id).toBe(row.platform + "-" + row.mode);
    expect(row.manifestKey).toMatch(/^[a-f0-9]{20}$/u);
  }
  expect(calls).toHaveLength(1);
  const options = calls[0]!.arguments[3];
  expect(options && ts.isObjectLiteralExpression(options)).toBe(true);
  if (!options || !ts.isObjectLiteralExpression(options)) throw Error("Actual producer options required");
  expect(options.properties.some(node => node.name?.getText(nativeProducerAst) === "output")).toBe(false);
  expect(options.properties.some(node => node.name?.getText(nativeProducerAst) === "sourcesRecord")).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of vectors.cases) {
      const oracle = db.query("SELECT ?1 || ?2 || 'published' || ?2 || ?3 || ?2 || ?4 publication, json_array('run',?5,'--output=stream') arguments").get(row.backingRoot, row.platform === "win32" ? "\\" : "/", row.manifestKey, row.mode, row.target) as { publication: string; arguments: string };
      expect(row.publication).toBe(oracle.publication);
      expect(row.arguments).toEqual(JSON.parse(oracle.arguments));
      const api = row.platform === "win32" ? win32 : posix;
      const actual = new Function("join", "resolve", "isAbsolute", "createHash", "readBuildMode", compile(publication.getText(publicationAst)) + ";return publishedCargoArtifactDirectoryV1;")(api.join, api.resolve, api.isAbsolute, createHash, (mode: string) => { if (!["prototype", "release"].includes(mode)) throw Error("Fixture mode required"); return mode; });
      expect(actual(row.backingRoot, row.manifest, row.mode)).toBe(oracle.publication);
    }
  } finally { db.close(); }
});

for (const row of vectors.cases) test("normal MCP consumer selects its producer publication on " + row.id, () => {
 const selected = binaryAst.statements.filter(node => ts.isFunctionDeclaration(node) ? ["pathApi", "resolvePublishedMcpBinaryPathV1"].includes(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some(value => ts.isIdentifier(value.name) && value.name.text === "MCP_BINARY_NAME"));
 expect(selected).toHaveLength(3);
 const resolveConsumer = new Function("posix", "win32", "process", compile(selected.map(node => node.getText(binaryAst)).join("\n")) + ";return resolvePublishedMcpBinaryPathV1;")(posix, win32, {platform:row.platform});
 const api = row.platform === "win32" ? win32 : posix;
 expect(resolveConsumer(row.publication, row.platform)).toBe(api.join(row.publication, row.binary));
});

for(const row of vectors.cases)test("actual launcher builds before acquiring its exact reader on "+row.id,async()=>{
 const owner=declarations(["servePublishedMcpStdioV1"])[0];if(!owner)throw Error("Actual publication owner required");
 const source=owner.getText(ast).replace(/^\s*const \{[^\n]+await import\([^\n]+$/gmu,""),trace:string[]=[],original={},control={},controller=new AbortController(),path=row.platform==="win32"?win32:posix;
 const reader=async(manifest:string,name:string,root:string,mode:string,invocation:unknown,phase:{signal:AbortSignal},consume:(artifact:{path:string})=>Promise<number>)=>{
  expect(trace).toEqual(["build"]);expect(path.resolve(root,manifest)).toBe(row.manifest);expect(name).toBe(row.binary);expect(mode).toBe(row.mode);expect(invocation).toBe(original);trace.push("reader-enter");try{return await consume({path:path.join(row.publication,row.binary)});}finally{expect(phase.signal.aborted).toBe(false);trace.push("reader-leave");}
 };
 const gateway=async(launch:{stage:(progress:(line:string)=>void,signal:AbortSignal)=>Promise<number>;command:()=>string;onStop:()=>void})=>{expect(await launch.stage(()=>{},controller.signal)).toBe(0);trace.push("serve");expect(launch.command()).toBe(path.join(row.publication,row.binary));trace.push("served");launch.onStop();return 0;};
 const actual=new Function("withAdmittedPublishedCargoExecutableV1","readBuildMode","createRequestedIssuedResourceControlV1","serveWarmMcpGateway","withMcpHostDriverPolicy","MCP_MANIFEST_REL","MCP_BINARY_NAME","MCP_BINARY_TARGET","MCP_STDIO_BUILD_MODE_V1","process",compile(source)+";return servePublishedMcpStdioV1;")(reader,(mode:string)=>mode,(_parent:unknown,_time:number,signal:AbortSignal)=>({signal}),gateway,(args:string[])=>args,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml","semio-os-mcp",row.target,"prototype",{platform:row.platform,stdin:undefined,stdout:undefined});
 expect(await actual(row.repoRoot,[],original,control,row.mode==="prototype"?{}:{SEMIO_BUILD_MODE:row.mode},undefined,async(target:string)=>{expect(target).toBe(row.target);trace.push("build");return 0;})).toBe(0);expect(trace).toEqual(["build","reader-enter","serve","served","reader-leave"]);
});

test("warm stdio begins serving before compiler storage is ready and stages catalog after binary", async () => {
 const owner=declarations(["serveWarmMcpStdio"])[0];if(!owner)throw Error("Actual warm launch required");
 let entered=false,release!: (binary:string)=>void;const prepared=new Promise<string>(resolve=>{release=resolve;}),controller=new AbortController(),targets:string[]=[];
 const port=async(options:{stage:(progress:(line:string)=>void,signal:AbortSignal)=>Promise<number>;command:()=>string})=>{entered=true;expect(await options.stage(()=>{},controller.signal)).toBe(0);expect(options.command()).toBe("published/semio-os-mcp");return 0;};
 const actual=new Function("serveWarmMcpGateway","mcpBinaryFreshness","withMcpHostDriverPolicy","statSync","accessSync","fsConstants","MCP_BINARY_TARGET","process",compile([...declarations(["waitForMcpPublicationV1"]),owner].map(node=>node.getText(ast)).join("\n"))+";return serveWarmMcpStdio;")(port,()=>({fresh:false}),(args:string[])=>args,()=>({isFile:()=>true}),()=>{}, {X_OK:1},"@semio-tech/framework-os-mcp-rs:build",{platform:"linux",stdin:undefined,stdout:undefined});
 const result=actual("repo",[],{},"linux",prepared,"@semio-tech/s-dev-composition:generate",async(target:string,_progress:unknown,signal:AbortSignal)=>{expect(signal).toBe(controller.signal);targets.push(target);return 0;});
 expect(entered).toBe(true);expect(targets).toEqual([]);release("published/semio-os-mcp");expect(await result).toBe(0);expect(targets).toEqual(["@semio-tech/framework-os-mcp-rs:build","@semio-tech/s-dev-composition:generate"]);
});

test("client EOF cancels blocked publication admission and drains its owner",async()=>{
 const vector=JSON.parse(readFileSync(new URL("../🧫️fixtures/lifetime.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:false}).validate(schema.definitions.Lifetime,vector)).toBe(true);
 const db=new Database(":memory:");try{expect(db.query("select 0 exitCode,0 ready,1 cancelled").get()).toEqual({exitCode:vector.exitCode,ready:Number(vector.ready),cancelled:Number(vector.cancelled)});}finally{db.close();}
 const owner=declarations(["servePublishedMcpStdioV1"])[0];if(!owner)throw Error("Actual publication owner required");const source=owner.getText(ast).replace(/^\s*const \{[^\n]+await import\([^\n]+$/gmu,"");
 let cancelled=false,drained=false,entered!:()=>void;const admitted=new Promise<void>(resolve=>{entered=resolve;});
 const reader=async(_manifest:unknown,_name:unknown,_root:unknown,_mode:unknown,_original:unknown,control:{signal:AbortSignal})=>{entered();try{await new Promise((_,reject)=>control.signal.addEventListener("abort",()=>{cancelled=true;reject(control.signal.reason);},{once:true}));}finally{drained=true;}};
 const gateway=async(launch:{stage:(progress:(line:string)=>void,signal:AbortSignal)=>Promise<number>;onStop:()=>void})=>{const pending=launch.stage(()=>{},new AbortController().signal);await admitted;launch.onStop();await pending.catch(()=>{});return vector.exitCode;};
 const actual=new Function("withAdmittedPublishedCargoExecutableV1","readBuildMode","createRequestedIssuedResourceControlV1","serveWarmMcpGateway","withMcpHostDriverPolicy","MCP_MANIFEST_REL","MCP_BINARY_NAME","MCP_BINARY_TARGET","MCP_STDIO_BUILD_MODE_V1","process",compile(source)+";return servePublishedMcpStdioV1;")(reader,(mode:string)=>mode,(_parent:unknown,_time:number,signal:AbortSignal)=>({signal}),gateway,(args:string[])=>args,"owner/Cargo.toml","semio-os-mcp","mcp:build","prototype",{platform:"linux",stdin:undefined,stdout:undefined});
 expect(await actual("repo",[],{},{},{},undefined,async()=>0)).toBe(vector.exitCode);expect(cancelled).toBe(vector.cancelled);expect(drained).toBe(true);
});

test("publication wait stops immediately when cancelled before preparation settles", async()=>{
 const owner=declarations(["waitForMcpPublicationV1"])[0];if(!owner)throw Error("Actual cancellable preparation wait required");
 const actual=new Function(compile(owner.getText(ast))+";return waitForMcpPublicationV1;")(),controller=new AbortController(),reason=Error("client closed");
 const pending=actual(new Promise<string>(()=>{}),controller.signal);controller.abort(reason);await expect(pending).rejects.toBe(reason);
});

test("actual TypeScript test task holds the shared publication reader throughout Vitest",async()=>{
 const text=readFileSync(new URL("../../../../📦️packages/🟦️typescript/📜️script.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("test-task.ts",text,ts.ScriptTarget.Latest,true),owner=tree.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text==="TestScript");if(!owner)throw Error("Actual TypeScript test owner required");
 const source=owner.getText(tree).replace(/^\s*const \{[^\n]+await import\([^\n]+$/gmu,""),trace:string[]=[],original={},environment:Record<string,string|undefined>={},subject="publication/semio-os-mcp";
 class Bundle{repoRoot="repo";root="package";invocation=original;}
 const reader=async(_manifest:unknown,_name:unknown,_workspace:unknown,_mode:unknown,invocation:unknown,_control:unknown,consume:(artifact:{path:string})=>Promise<void>)=>{expect(invocation).toBe(original);trace.push("reader-enter");await consume({path:subject});expect(environment.SEMIO_OS_MCP_BIN).toBeUndefined();trace.push("reader-leave");};
 const actual=new Function("BundleScript","resolveTestLevel","withAdmittedPublishedCargoExecutableV1","readBuildMode","MCP_MANIFEST_REL","MCP_BINARY_NAME","MCP_STDIO_BUILD_MODE_V1","withIssuedResourceControlV1","requireMcpBinary","runVitest","process","console",compile(source)+";return new TestScript();")(Bundle,()=>({rest:[]}),reader,(mode:string)=>mode,"owner/Cargo.toml","semio-os-mcp","prototype",(_original:unknown,_name:unknown,_time:unknown,consume:(control:unknown)=>Promise<void>)=>consume({}),()=>environment.SEMIO_OS_MCP_BIN,async()=>{expect(environment.SEMIO_OS_MCP_BIN).toBe(subject);expect(trace).toEqual(["reader-enter"]);trace.push("tests");},{env:environment,platform:"linux"},{log:()=>{}});
 await actual.run([]);expect(trace).toEqual(["reader-enter","tests","reader-leave"]);
});

test("actual native dev transports hold publication custody and inject authored host policy",async()=>{
 const vectors=JSON.parse(readFileSync(new URL("../🧫️fixtures/transports.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:false}).validate(schema.definitions.Transports,vectors)).toBe(true);
 const owner=buildAst.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text==="DevScript");if(!owner)throw Error("Actual native DevScript required");const helper=buildAst.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text==="PublishedMcpScript");if(!helper)throw Error("Shared native publication consumer required");const source=[helper,owner].map(node=>node.getText(buildAst)).join("\n").replace(/^\s*const \{[^\n]+await import\([^\n]+$/gmu,"");
 const policy=declarations(["withMcpHostDriverPolicy"])[0];if(!policy)throw Error("Authored policy projection required");const hostPolicy=new Function("MCP_HOST_DRIVER_POLICY_PATH","MCP_HOST_OPERATION_POLICY_PATH",compile(policy.getText(ast))+";return withMcpHostDriverPolicy;")("authored-policy","authored-inference-policy");
 for(const row of vectors.cases){
  const trace:string[]=[],original={capabilities:{buildMode:"prototype"}},controller=new AbortController(),environment={SENTINEL:"caller",SEMIO_BUILD_MODE:"prototype"};class Bundle{repoRoot="workspace";root="crate";invocation=original;}
  const reader=async(_manifest:unknown,_name:unknown,root:string,_mode:unknown,invocation:unknown,_control:unknown,consume:(artifact:{path:string;environment:object},control:unknown)=>Promise<void>)=>{expect(root).toBe("workspace");expect(invocation).toBe(original);trace.push("reader-enter");await consume({path:"published/semio-os-mcp",environment:{ARTIFACT:"owned"}},{signal:controller.signal});trace.push("reader-leave");};
  const run=async(command:string,args:string[],cwd:string,_label:unknown,_budget:unknown,options:{env:object;signal:AbortSignal;stdout:string})=>{expect(command).toBe("published/semio-os-mcp");expect(args).toEqual(row.expected);expect(cwd).toBe("crate");expect(options.env).toEqual({...environment,ARTIFACT:"owned",ORIGINAL:original});expect(options.signal).toBe(controller.signal);expect(options.stdout).toBe("inherit");expect(trace).toEqual(["reader-enter"]);trace.push("run");};
  const actual=new Function("BundleScript","withAdmittedPublishedCargoExecutableV1","readBuildMode","MCP_MANIFEST_REL","MCP_BINARY_NAME","MCP_STDIO_BUILD_MODE_V1","withIssuedResourceControlV1","withMcpHostDriverPolicy","runOwnedCommand","scriptInvocationBudget","scriptInvocationEnvironment","process",compile(source)+";return new DevScript();")(Bundle,reader,(mode:string)=>mode,"owner/Cargo.toml","semio-os-mcp","prototype",(_inv:unknown,_label:unknown,_time:unknown,consume:(control:unknown)=>Promise<void>)=>consume({}),hostPolicy,run,()=>({maximumElapsedMilliseconds:0}),(invocation:unknown,env:object)=>({...env,ORIGINAL:invocation}),{env:environment,platform:"linux"});
  const db=new Database(":memory:");try{const oracle=db.query("with recursive policy(n,flag,path) as (select 0,?4,?5 union all select 1,'--operation-policy','authored-inference-policy'), steps(n,args) as (select 0,case when json_array_length(?1)=0 then json_array(?2) else ?1 end union all select steps.n+1,case when exists(select 1 from json_each(steps.args) where value=policy.flag) then steps.args else json_insert(steps.args,?3,policy.flag,?3,policy.path) end from steps join policy on policy.n=steps.n) select args arguments from steps where n=2").get(JSON.stringify(row.args),"stdio","$[#]","--host-driver-policy","authored-policy")as{arguments:string};expect(row.expected).toEqual(JSON.parse(oracle.arguments));}finally{db.close();}
  await actual.run(row.args);expect(trace).toEqual(["reader-enter","run","reader-leave"]);
 }
});

test("native audit and schema capture retain full output, exact status and reader custody",async()=>{
 const vectors=JSON.parse(readFileSync(new URL("../🧫️fixtures/captures.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:false}).validate(schema.definitions.Captures,vectors)).toBe(true);
 const owner=buildAst.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text==="PublishedMcpScript");if(!owner)throw Error("Shared native consumer required");const source=owner.getText(buildAst).replace(/^\s*const \{[^\n]+await import\([^\n]+$/gmu,"");
 for(const row of vectors.cases){const trace:string[]=[],original={capabilities:{buildMode:"prototype"}},controller=new AbortController(),expected=row.character.repeat(row.bytes);class Bundle{repoRoot="workspace";root="crate";invocation=original;}class Failure extends Error{outcome={status:row.status,signal:null};}
  const reader=async(_manifest:unknown,_name:unknown,_root:unknown,_mode:unknown,_invocation:unknown,_control:unknown,consume:(artifact:{path:string;environment:object},control:unknown)=>Promise<unknown>)=>{trace.push("reader-enter");const value=await consume({path:"publication",environment:{}},{signal:controller.signal});trace.push("reader-leave");return value;};
  const run=async(_binary:unknown,args:string[],_cwd:unknown,_label:unknown,_budget:unknown,options:{signal:AbortSignal;stdout:string;outputConsumer:(bytes:Uint8Array,stream:string)=>void})=>{expect(args).toEqual(row.arguments);expect(options.stdout).toBe("ignore");expect(options.signal).toBe(controller.signal);const chunk=Buffer.from(expected);options.outputConsumer(chunk,"stdout");chunk.fill(0);options.outputConsumer(Buffer.from("diagnostic"),"stderr");trace.push("run");if(row.status)throw new Failure();};
  const actual=new Function("BundleScript","withAdmittedPublishedCargoExecutableV1","readBuildMode","MCP_MANIFEST_REL","MCP_BINARY_NAME","MCP_STDIO_BUILD_MODE_V1","withIssuedResourceControlV1","runOwnedCommand","OwnedCommandFailure","scriptInvocationBudget","scriptInvocationEnvironment","process",compile(source)+";return new(class extends PublishedMcpScript{run(args){return this.capturePublished(args,60000);}})();")(Bundle,reader,(mode:string)=>mode,"owner/Cargo.toml","semio-os-mcp","prototype",(_inv:unknown,_label:unknown,_time:unknown,consume:(control:unknown)=>Promise<unknown>)=>consume({}),run,Failure,()=>60000,(_invocation:unknown,env:object)=>env,{env:{},platform:"linux"});
  const result=await actual.run(row.arguments);expect(result).toEqual({stdout:expected,stderr:"diagnostic",status:row.status});expect(trace).toEqual(["reader-enter","run","reader-leave"]);const db=new Database(":memory:");try{expect(db.query("select length(?) bytes").get(result.stdout)).toEqual({bytes:row.bytes});}finally{db.close();}
 }
});
