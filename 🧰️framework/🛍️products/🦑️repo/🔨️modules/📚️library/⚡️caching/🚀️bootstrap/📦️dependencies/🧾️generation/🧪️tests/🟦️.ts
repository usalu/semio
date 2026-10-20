import {test,expect} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {build as esbuild} from "esbuild";
import {withCurrentBunWorkspaceOwnershipV1,readBunWorkspaceListingProjectionV1} from "../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,readlinkSync,realpathSync,renameSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import {captureGenerationReceiptV1,generationAnchorV1,generationCanonicalJsonV1,generationProjectionDigestV1,generationProtocolV1 as protocol,generationSha256V1,generationStampV1,parseGenerationReceiptV1,sealGenerationReceiptV1,verifyGenerationReceiptV1,type GenerationCaptureInputV1,type GenerationControlV1,type GenerationObservationV1,type GenerationReceiptV1,type GenerationRuntimeV1,type GenerationVerifyInputV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🔣️.json" with {type:"json"};
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../🏗️build/📥️caller/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as issued from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as storage from "../../💾️storage/🟦️.ts";
import * as glue from "../💾️storage/🟦️.ts";

const runtime:GenerationRuntimeV1={bun:"0.0.0",revision:"fixture",node:"0.0.0",platform:"fixture",arch:"fixture",sha256:"a".repeat(64)};

type Mutation={op:string;path:string;text?:string;target?:string};
type Case={name:string;mutations:Mutation[];expect:string;tamper?:string;port?:string;relist?:number;reseal?:string};
type Ambient={env:Record<string,string|undefined>;home:string;config:string};
type Subject=glue.JavascriptDependencyGenerationSubjectV1;
const withOriginal=<T>(body:(original:ScriptInvocation)=>Promise<T>):Promise<T>=>issued.withScriptProcessEnvelope(issued.createScriptProcessEnvelope({version:1,owner:"generation-receipt",maximumElapsedMilliseconds:600000,},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),body);
const subjectOf=(root:string,loadedAtMs:number,env:Record<string,string|undefined>={}):Subject=>Object.freeze({root,entries:fixtures.implementation.entries,loadedAtMs,ambient:Object.freeze({env,home:join(root,".ambient/home"),config:join(root,".ambient/xdg")})});

const control=(counter:{ticks:number;checks:number},abort=new AbortController()):GenerationControlV1=>({signal:abort.signal,check:()=>{counter.checks++;abort.signal.throwIfAborted();},tick:async()=>{counter.ticks++;abort.signal.throwIfAborted();}});

function materialize(){
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"gr-"))),store=join(root,"store"),place=(value:string)=>value.replaceAll("@STORE@",store);
 for(const[path,text]of Object.entries(fixtures.workspace.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 for(const[path,target]of Object.entries(fixtures.workspace.links)){mkdirSync(dirname(join(root,path)),{recursive:true});symlinkSync(place(target),join(root,path));}
 return{root,store,place};
}
const facts=(path:string)=>lstatSync(path,{bigint:true,throwIfNoEntry:false});
const row=(path:string,full:boolean):GenerationObservationV1=>{const f=facts(path);const kind=!f?"missing":f.isSymbolicLink()?"link":f.isDirectory()?"directory":f.isFile()?"file":"foreign";return{path,kind,anchor:f?generationAnchorV1(f):null,stamp:f?generationStampV1(f):null,target:kind==="link"?readlinkSync(path):null,full:full||kind==="link"};};
const bindingText=(pairs:readonly {sourceDirectory:string;targetDirectory:string}[])=>JSON.stringify(pairs.map(pair=>{const source=lstatSync(pair.sourceDirectory,{bigint:true});if(!source.isSymbolicLink()||realpathSync(pair.sourceDirectory)!==pair.targetDirectory)throw Error("Current expected JavaScript dependency binding required");const target=lstatSync(pair.targetDirectory,{bigint:true});return[pair.sourceDirectory,String(source.dev),String(source.ino),String(target.dev),String(target.ino)];}));
const readyFingerprint=(path:string)=>{const f=lstatSync(path,{bigint:true}),bytes=readFileSync(path);return[f.size,f.ino,generationSha256V1(bytes)].join(":");};

async function capture(original:ScriptInvocation,root:string,store:string,startedAtMs:number,subject:Subject=subjectOf(root,startedAtMs),implementation?:Awaited<ReturnType<typeof glue.captureJavascriptDependencyGenerationImplementationV1>>){
 const walked=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed()),
  source=(path:string)=>{const f=lstatSync(path);return{path,kind:"file"as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(path)};},
  absent=(path:string)=>({path,kind:"missing"as const,dev:"",ino:"",mode:0,bytes:Buffer.alloc(0)}),
  directory=(path:string)=>{const f=lstatSync(path);return{path,kind:"directory"as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:Buffer.alloc(0)};},
  pairs=[{sourceDirectory:join(root,"pkgs/a/node_modules"),targetDirectory:join(store,"j/g1/node_modules")}],
  observed=fixtures.workspace.observed,observations=[...observed.full.map(path=>row(join(root,path),true)),...observed.anchors.map(path=>row(join(root,path),false)),...observed.links.map(path=>row(join(root,path),true)),...observed.missing.map(path=>row(join(root,path),false))],
  input:GenerationCaptureInputV1={
   workspace:root,storeRoot:store,scope:".",startedAtMs,implementation:implementation??await glue.captureJavascriptDependencyGenerationImplementationV1(original,subject),runtime,producer:glue.observeJavascriptDependencyGenerationProducerV1(root,store,["."],subject.ambient),recipeSha256:"c".repeat(64),ownershipSha256:"d".repeat(64),required:["."],
   inputs:[{scope:".",sources:[directory(root),source(join(root,"package.json")),source(join(root,"bun.lock")),absent(join(root,"bunfig.toml"))]}],
   membership:walked,bindings:[{scope:".",pairs,observation:bindingText(pairs)}],ready:[{scope:".",generation:"e".repeat(64),path:join(store,"j/g1/bindings.json"),fingerprint:readyFingerprint(join(store,"j/g1/bindings.json"))}],
   execution:{scopes:[{scope:".",sha256:"f".repeat(64)}],runtimeSha256:runtime.sha256,observations},scopeReadSha256:"0".repeat(64),control:control({ticks:0,checks:0}),
  };
 const receiptsAndBindings=[{scope:".",generation:"e".repeat(64),fingerprint:input.ready[0]!.fingerprint}],scopeReadSha256=generationSha256V1(JSON.stringify([receiptsAndBindings,[[".",input.bindings[0]!.observation]],input.execution.scopes])),sealed={...input,scopeReadSha256};
 return{input:sealed,result:await captureGenerationReceiptV1(sealed),walked};
}
function verifyInput(root:string,store:string,receipt:GenerationReceiptV1,files:Map<string,Uint8Array>,counter:{ticks:number;checks:number},abort?:AbortController,port?:string,subject:Subject=subjectOf(root,0),implementationRoot:string=root):GenerationVerifyInputV1{
 return{
  workspace:root,storeRoot:store,receipt,readEvidence:name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},
  implementationEntries:port==="entries"?[...fixtures.implementation.entries,"impl/other.ts"]:subject.entries,implementationFile:glue.javascriptDependencyGenerationImplementationFileV1(implementationRoot),producer:()=>glue.observeJavascriptDependencyGenerationProducerV1(root,store,receipt.required,subject.ambient),
  runtime:()=>port==="runtime"?{...runtime,bun:"9.9.9"}:port==="revision"?{...runtime,revision:"other"}:runtime,conditionsClosed:()=>port!=="conditions",
  project:path=>readBunWorkspaceListingProjectionV1(root,path),observeBindings:(_scope,pairs)=>bindingText(pairs),readReady:readyFingerprint,control:control(counter,abort),
 };
}
function mutate(root:string,place:(value:string)=>string,mutation:Mutation,env:Record<string,string|undefined>){
 const path=join(root,mutation.path);
 if(mutation.op==="env")env[mutation.path]=mutation.text;
 else if(mutation.op==="write"){mkdirSync(dirname(path),{recursive:true});writeFileSync(path,mutation.text!);}
 else if(mutation.op==="mkdir")mkdirSync(path,{recursive:true});
 else if(mutation.op==="remove")rmSync(path,{recursive:true,force:true});
 else if(mutation.op==="symlink")symlinkSync(place(mutation.target!),path);
 else if(mutation.op==="chmod")chmodSync(path,parseInt(mutation.text!,8));
 else if(mutation.op==="touch"){const now=new Date(Date.now()+5000);utimesSync(path,now,now);}
 else if(mutation.op==="replace"){const bytes=readFileSync(path);writeFileSync(path+".tmp",bytes);renameSync(path+".tmp",path);}
 else throw Error("Closed mutation required");
}
const outcome=(result:{status:string;class?:string})=>result.status==="match"?"match":"mismatch:"+result.class;

test("generation receipt contract: schema closes the mismatch classes the fixtures drive and a third-party validator accepts the sealed receipt",()=>withOriginal(async original=>{
 const{root,store}=materialize();
 try{
  const{result}=await capture(original,root,store,Date.now()+10000);expect(result.status).toBe("captured");if(result.status!=="captured")return;
  const ajv=new Ajv({strict:false}).addSchema(schema),validate=ajv.getSchema(schema.$id+"#/$defs/Receipt")!;
  expect(validate(JSON.parse(JSON.stringify(result.receipt))),JSON.stringify(validate.errors)).toBe(true);
  const reached=new Set(fixtures.cases.map(row=>row.expect.split(":")[1]).filter(Boolean));
  for(const name of protocol.mismatchClasses)if(!["receipt-invalid","unreadable"].includes(name))expect(reached.has(name),name).toBe(true);
  expect(new Set(protocol.mismatchClasses).size).toBe(protocol.mismatchClasses.length);
 }finally{rmSync(root,{recursive:true,force:true});}
}));

test("generation receipt: every fixture case verifies exactly as declared (match or typed fallback)",()=>withOriginal(async original=>{
 for(const row of fixtures.cases as Case[]){
  const{root,store,place}=materialize(),env:Record<string,string|undefined>={...fixtures.environment},subject=subjectOf(root,Date.now()+10000,env);
  try{
   const{result}=await capture(original,root,store,Date.now()+10000,subject);expect(result.status,row.name).toBe("captured");if(result.status!=="captured")continue;
   let receipt=result.receipt;const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes]));
   if(row.tamper==="frontier"||row.tamper==="execution"||row.tamper==="closure"){const name=row.tamper==="frontier"?receipt.membership.frontier.name:row.tamper==="execution"?receipt.execution.evidence.name:receipt.implementation.evidence.name;files.set(name,Buffer.concat([files.get(name)!,Buffer.from("\n")]));}
   if(row.reseal==="scopeReadSha256"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:"1".repeat(64)});}
   const before=protocol.maximumRelist;
   try{
    if(row.relist)(protocol as {maximumRelist:number}).maximumRelist=row.relist;
    for(const mutation of row.mutations)mutate(root,place,mutation,env);
    const counter={ticks:0,checks:0},verdict=await verifyGenerationReceiptV1(verifyInput(root,store,receipt,files,counter,undefined,row.port,subject));
    expect(outcome(verdict),row.name).toBe(row.expect);
    if(verdict.status==="match"){expect(verdict.required).toEqual(["."]);expect(verdict.receipts[0]!.fingerprint).toBe(readyFingerprint(join(store,"j/g1/bindings.json")));}
   }finally{(protocol as {maximumRelist:number}).maximumRelist=before;}
  }finally{rmSync(root,{recursive:true,force:true});}
 }
}),60000);

test("generation receipt: a tampered or schema-invalid receipt is refused before any filesystem read",()=>withOriginal(async original=>{
 const{root,store}=materialize();
 try{
  const{result}=await capture(original,root,store,Date.now()+10000);if(result.status!=="captured")throw Error("captured expected");
  const text=JSON.stringify(result.receipt),bytes=Buffer.from(text);expect(parseGenerationReceiptV1(bytes).sha256).toBe(result.receipt.sha256);
  expect(()=>parseGenerationReceiptV1(Buffer.from(text.replace('"fixture"','"fixturf"')))).toThrow("Sealed dependency generation receipt");
  const{sha256:_,...body}=result.receipt;
  expect(()=>parseGenerationReceiptV1(Buffer.from(JSON.stringify({...body,version:2,sha256:generationSha256V1(generationCanonicalJsonV1({...body,version:2}))})))).toThrow("Closed dependency generation receipt");
  expect(()=>parseGenerationReceiptV1(Buffer.from(JSON.stringify({...result.receipt,extra:true})))).toThrow("Closed dependency generation receipt");
  expect(()=>parseGenerationReceiptV1(Buffer.alloc(protocol.maximumReceiptBytes+1))).toThrow("Bounded dependency generation receipt");
 }finally{rmSync(root,{recursive:true,force:true});}
}));

test("generation receipt: capture defers rather than seal an unsettled timestamp",()=>withOriginal(async original=>{
 const{root,store}=materialize();
 try{
  const settledSubject=subjectOf(root,Date.now()+10000),young=await capture(original,root,store,Date.now()-10000,settledSubject);expect(young.result.status).toBe("deferred");
  const unloaded=await capture(original,root,store,Date.now()+10000,subjectOf(root,Date.now()-10000));expect(unloaded.result).toMatchObject({status:"deferred",reason:"implementation-unsettled"});
  const settled=await capture(original,root,store,Date.now()+10000);expect(settled.result.status).toBe("captured");
  const volatile=settled.result.status==="captured"?new TextDecoder().decode(settled.result.files[0]!.bytes).split("\n").filter(Boolean).map(line=>JSON.parse(line)as unknown[]).filter(line=>line[0]==="d"&&line[7]===0):[];expect(volatile).toEqual([]);
  const mixed=await capture(original,root,store,Date.now(),settledSubject);
  if(mixed.result.status==="captured"){const rows=new TextDecoder().decode(mixed.result.files[0]!.bytes).split("\n").filter(Boolean).map(line=>JSON.parse(line)as unknown[]);expect(rows.some(line=>line[0]==="d"&&line[7]===0)).toBe(true);}
 }finally{rmSync(root,{recursive:true,force:true});}
}));

test("generation receipt: verification honors cancellation and publishes bounded progress",()=>withOriginal(async original=>{
 const{root,store}=materialize();
 try{
  for(let index=0;index<fixtures.progress.directories;index++)mkdirSync(join(root,"pkgs/a/src/d"+index),{recursive:true});
  const{result}=await capture(original,root,store,Date.now()+10000);if(result.status!=="captured")throw Error("captured expected");
  const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes])),counter={ticks:0,checks:0},verdict=await verifyGenerationReceiptV1(verifyInput(root,store,result.receipt,files,counter));
  expect(verdict.status).toBe("match");expect(counter.ticks).toBeGreaterThan(0);expect(counter.checks).toBeGreaterThan(counter.ticks);
  const abort=new AbortController(),stopper={ticks:0,checks:0},input=verifyInput(root,store,result.receipt,files,stopper,abort);
  const cancelling:GenerationVerifyInputV1={...input,control:{signal:abort.signal,check:()=>abort.signal.throwIfAborted(),tick:async()=>{abort.abort(Object.freeze({cancel:"fixture"}));abort.signal.throwIfAborted();}}};
  await expect(verifyGenerationReceiptV1(cancelling)).rejects.toMatchObject({cancel:"fixture"});
  const failing:GenerationVerifyInputV1={...input,control:{signal:new AbortController().signal,check:()=>{},tick:async()=>{}},project:()=>{throw Error("directory unreadable");}};
  writeFileSync(join(root,"pkgs/a/touched.txt"),"x");const stale=new Date(Date.now()+60000);utimesSync(join(root,"pkgs/a"),stale,stale);
  expect(outcome(await verifyGenerationReceiptV1(failing))).toBe("mismatch:unreadable");
 }finally{rmSync(root,{recursive:true,force:true});}
}),60000);

const oracle=`
import hashlib, json, os, re, sys
root, rules = sys.argv[1], json.loads(sys.argv[2])
opaque = set(rules["opaque"]); patterns = [re.compile(p) for p in rules["opaquePatterns"]]
is_opaque = lambda name: name in opaque or any(p.search(name) for p in patterns)
def kind(entry):
    if entry.is_symlink(): return "symlink"
    if entry.is_dir(follow_symlinks=False): return "directory"
    if entry.is_file(follow_symlinks=False): return "file"
    return "other"
out = {}
def walk(directory):
    entries = sorted(os.scandir(directory), key=lambda e: e.name.encode("utf-8"))
    rows = [{"name": e.name, "kind": kind(e)} for e in entries if e.name == "package.json" or (not e.name.startswith(".") and not is_opaque(e.name) and kind(e) in ("symlink", "directory"))]
    out[os.path.relpath(directory, root)] = hashlib.sha256(json.dumps(rows, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()[:32]
    for e in entries:
        if e.name.startswith(".") or is_opaque(e.name): continue
        if e.is_dir(follow_symlinks=False): walk(os.path.join(directory, e.name))
for start in rules["walkRoots"]: walk(os.path.join(root, start))
seal = json.dumps(json.loads(sys.argv[3]), sort_keys=True, separators=(",", ":"), ensure_ascii=False)
print(json.dumps({"directories": out, "seal": hashlib.sha256(seal.encode("utf-8")).hexdigest()}, ensure_ascii=False))
`;

test("generation receipt: the frontier and the seal equal an independent Python implementation",()=>withOriginal(async original=>{
 const{root,store}=materialize();
 try{
  const{result,walked}=await capture(original,root,store,Date.now()+10000);if(result.status!=="captured")throw Error("captured expected");
  const{sha256:_,...body}=result.receipt,run=spawnSync(testPythonV1()??"python3",["-I","-c",oracle,root,JSON.stringify(fixtures.rules),JSON.stringify(body)],{encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);
  const python=JSON.parse(run.stdout)as{directories:Record<string,string>;seal:string},ours=Object.fromEntries(walked.paths.filter(path=>path.list).map(path=>[relative(root,path.path),generationProjectionDigestV1(path.list!)]));
  expect(ours).toEqual(python.directories);expect(Object.keys(ours).length).toBeGreaterThan(3);expect(python.seal).toBe(result.receipt.sha256);
  const frontier=new TextDecoder().decode(result.files[0]!.bytes).split("\n").filter(Boolean).map(line=>JSON.parse(line)as unknown[]);
  expect(Object.fromEntries(frontier.filter(line=>line[0]==="d").map(line=>[line[1],line[8]]))).toEqual(python.directories);
  expect(walked.texts.map(text=>text.path).sort()).toEqual(["package.json","pkgs/a/package.json","pkgs/b/package.json","pkgs/\u{1F9E9}️widget/package.json"].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
}));

test("generation receipt: stamp and anchor equal the provider walk's own formulas and the storage laws stay pinned",()=>{
 const storage=readFileSync(new URL("../../💾️storage/🟦️.ts",import.meta.url),"utf8"),install=JSON.parse(readFileSync(new URL("../../💾️storage/🧬️schema/📥️install.json",import.meta.url),"utf8"));
 expect(storage).toContain('keys=["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"]as const');
 expect(storage).toContain("stamp=(row:BigIntStats)=>JSON.stringify(keys.map(key=>String(row[key])))");
 expect(storage).toContain("anchor=(row:BigIntStats)=>JSON.stringify(protocol.ancestorFields.map(key=>String(row[key as keyof BigIntStats])))");
 expect(storage).toContain("if(process.env.NODE_OPTIONS||process.env.BUN_OPTIONS||process.execArgv.some(value=>/^--(?:conditions|preserve-symlinks)(?:=|$)/u.test(value)))throw Error(\"Closed declared execution conditions required\")");
 expect(install.$defs.SourceOwnerExecutionProtocol.const.ancestorFields).toEqual(protocol.anchorFields);
 expect(protocol.stampFields).toEqual(["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"]);
 const f=lstatSync(import.meta.path,{bigint:true}),keys=protocol.stampFields as readonly string[];
 expect(JSON.stringify(generationStampV1(f).split(":"))).toBe(JSON.stringify(keys.map(key=>String(f[key as keyof typeof f]))));
 expect(JSON.stringify(generationAnchorV1(f).split(":"))).toBe(JSON.stringify(install.$defs.SourceOwnerExecutionProtocol.const.ancestorFields.map((key:string)=>String(f[key as keyof typeof f]))));
 const walk=readFileSync(new URL("../../../../../🗂️workspaces/🟦️bun/🟨️.cjs",import.meta.url),"utf8");
 expect(walk.match(/projectListing\(/gu)!.length).toBe(2);
 expect(walk.match(/entry\.name === "package\.json"/gu)!.length).toBe(2);
});

const closureOracle=String.raw`
import json, os, re, sys
root, entries = sys.argv[1], json.loads(sys.argv[2])
literal = re.compile(r"""(["'])(\.{1,2}/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1""")
statement = re.compile(r"""(?:from\s*|import\s*\(\s*|require\(\s*|import\s+)["']([^"']+)["']""")
seen, pending = set(), list(entries)
while pending:
    path = pending.pop()
    if path in seen or not os.path.isfile(os.path.join(root, path)): continue
    seen.add(path)
    directory = os.path.dirname(path)
    schema_directory = os.path.join(root, directory, "\U0001F9EC\ufe0fschema")
    if os.path.isdir(schema_directory):
        for base, _names, filenames in os.walk(schema_directory):
            pending.extend(os.path.relpath(os.path.join(base, name), root) for name in filenames if name.endswith(".json"))
    if path.endswith(".json"): continue
    text = open(os.path.join(root, path), encoding="utf-8", errors="replace").read()
    for match in literal.finditer(text): pending.append(os.path.normpath(os.path.join(directory, match.group(2))))
    for specifier in statement.findall(text):
        if specifier.startswith(".") and re.search(r"\.(?:[cm]?[jt]sx?|json)$", specifier): pending.append(os.path.normpath(os.path.join(directory, specifier)))
print(json.dumps(sorted(seen)))
`;

test("generation receipt: the sealed implementation is the actual runtime import closure, a superset of what esbuild and an independent Python reader find, and covers every named fence",()=>withOriginal(async original=>{
 const root=resolve(import.meta.dir,"../../../../../../../../../.."),subject:Subject={root,entries:protocol.implementation.entries,loadedAtMs:Date.now(),ambient:glue.javascriptDependencyGenerationAmbientV1()};
 const implementation=await glue.captureJavascriptDependencyGenerationImplementationV1(original,subject),sealed=new Set(implementation.files.map(file=>file.path));
 expect(implementation.entries).toEqual(protocol.implementation.entries);expect(sealed.size).toBe(implementation.files.length);expect(sealed.size).toBeGreaterThan(100);
 for(const entry of protocol.implementation.entries){expect(sealed.has(entry),entry).toBe(true);expect(lstatSync(join(root,entry)).isFile(),entry).toBe(true);}
 for(const file of implementation.files){const bytes=readFileSync(join(root,file.path));expect(file.sha256,file.path).toBe(generationSha256V1(bytes));expect(file.bytes,file.path).toBe(bytes.length);}
 const bundled=new Set<string>();
 for(const entry of protocol.implementation.entries){
  const bundle=await esbuild({absWorkingDir:root,entryPoints:[entry],bundle:true,write:false,metafile:true,platform:"node",logLevel:"silent",external:["bun:*","bun"],packages:"external"});
  for(const name of Object.keys(bundle.metafile.inputs)){const input=name.replace(/ with \{.*\}$/u,"");if(!input.startsWith("node_modules/")&&!input.includes("/node_modules/"))bundled.add(input);}
 }
 expect(bundled.size).toBeGreaterThan(50);
 expect([...bundled].filter(path=>!sealed.has(path)),"esbuild bundle inputs missing from the sealed closure").toEqual([]);
 const python=spawnSync(testPythonV1()??"python3",["-I","-c",closureOracle,root,JSON.stringify(protocol.implementation.entries)],{encoding:"utf8",maxBuffer:67108864});expect(python.status,python.stderr).toBe(0);
 const independent=new Set<string>(JSON.parse(python.stdout));expect([...independent].filter(path=>!sealed.has(path)),"python closure inputs missing from the sealed closure").toEqual([]);
 const library=resolve(import.meta.dir,"../../../../..");
 for(const file of fixtures.fences)expect(sealed.has(file),file).toBe(true);
 for(const file of fixtures.legacyImplementationFiles)expect(sealed.has(relative(root,join(library,file))),file).toBe(true);
 expect([...sealed].some(path=>path.endsWith("/🧬️schema/🔣️.json"))).toBe(true);
}),120000);

test("generation receipt: editing any file of the real closure, a fence in an unlisted-by-hand module or an installer configuration forces the complete walk",()=>withOriginal(async original=>{
 const real=resolve(import.meta.dir,"../../../../../../../../../.."),{root,store}=materialize(),copy=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"gc-")),subject:Subject={root:real,entries:protocol.implementation.entries,loadedAtMs:Date.now()+10000,ambient:subjectOf(root,0).ambient};
 try{
  const implementation=await glue.captureJavascriptDependencyGenerationImplementationV1(original,subject);
  for(const file of implementation.files){mkdirSync(dirname(join(copy,file.path)),{recursive:true});writeFileSync(join(copy,file.path),readFileSync(join(real,file.path)));}
  const{result}=await capture(original,root,store,Date.now()+10000,subjectOf(root,Date.now()+10000),implementation);expect(result.status).toBe("captured");if(result.status!=="captured")return;
  const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes])),verify=()=>verifyGenerationReceiptV1(verifyInput(root,store,result.receipt,files,{ticks:0,checks:0},undefined,undefined,subject,copy));
  expect(outcome(await verify())).toBe("match");
  const unrelated=join(copy,"unrelated-sibling.ts");writeFileSync(unrelated,"export {};\n");expect(outcome(await verify())).toBe("match");
  const touched:string[]=[];
  for(const file of implementation.files){
   const path=join(copy,file.path),before=readFileSync(path);
   writeFileSync(path,Buffer.concat([before,Buffer.from(file.path.endsWith(".json")?"\n":"\n//\n")]));
   const verdict=await verify();touched.push(file.path);
   expect(outcome(verdict),file.path).toBe("mismatch:implementation-changed");expect((verdict as {path?:string}).path,file.path).toBe(file.path);
   writeFileSync(path,before);
  }
  expect(touched.length).toBe(implementation.files.length);expect(outcome(await verify())).toBe("match");
  const fence=join(copy,fixtures.fences.find(file=>file.endsWith("📋️contract/🟦️.ts"))!);
  const text=readFileSync(fence);rmSync(fence);expect(outcome(await verify())).toBe("mismatch:implementation-changed");
  symlinkSync(join(real,relative(copy,fence)),fence);expect(outcome(await verify())).toBe("mismatch:implementation-changed");rmSync(fence);writeFileSync(fence,text);expect(outcome(await verify())).toBe("match");
 }finally{rmSync(root,{recursive:true,force:true});rmSync(copy,{recursive:true,force:true});}
}),240000);

test("generation receipt producer: the observation is sorted, digests values instead of storing them, ignores unrelated variables and follows the installer recipe",()=>{
 const{root,store}=materialize();
 try{
  const secret="https://user:hunter2@registry.example.test/",ambient={env:{NPM_CONFIG_REGISTRY:secret,https_proxy:secret,PATH:"/bin",NPM_CONFIG_USER_AGENT:"x",BUN_INSTALL_CACHE_DIR:""},home:join(root,".ambient/home"),config:join(root,".ambient/xdg")};
  const observed=glue.observeJavascriptDependencyGenerationProducerV1(root,store,[".","pkgs/a"],ambient);
  expect(JSON.stringify(observed)).not.toContain("hunter2");
  expect(observed.environment.map(row=>row.name)).toEqual(["BUN_INSTALL_CACHE_DIR","NPM_CONFIG_REGISTRY","https_proxy"]);
  expect(observed.environment[1]!.sha256).toBe(generationSha256V1(secret));expect(observed.environment[0]!.sha256).toBe(generationSha256V1(""));
  const paths=observed.files.map(row=>row.path);
  for(const needed of["w:.npmrc","w:pkgs/a/.npmrc","s:bun-home/.npmrc","s:bun-home/.bunfig.toml","s:bun-config/.bunfig.toml","w:.ambient/home/.npmrc","w:.ambient/home/.bunfig.toml","w:.ambient/xdg/.bunfig.toml","w:.ambient/home/.config/.bunfig.toml"])expect(paths,needed).toContain(needed);
  expect(new Set(paths).size).toBe(paths.length);expect(observed.files.every(row=>row.kind==="missing"&&row.sha256===null)).toBe(true);
  expect([...paths].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)))).toEqual(paths);
  const validate=new Ajv({strict:false}).addSchema(schema).getSchema(schema.$id+"#/$defs/Producer")!;expect(validate(JSON.parse(JSON.stringify(observed))),JSON.stringify(validate.errors)).toBe(true);
  writeFileSync(join(root,".npmrc"),"a=1\n");const changed=glue.observeJavascriptDependencyGenerationProducerV1(root,store,[".","pkgs/a"],ambient);
  expect(changed.files.find(row=>row.path==="w:.npmrc")).toEqual({path:"w:.npmrc",kind:"file",bytes:4,sha256:generationSha256V1("a=1\n")});
  expect(generationCanonicalJsonV1(changed)).not.toBe(generationCanonicalJsonV1(observed));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("generation receipt producer: the observed levels equal what the installer recipe isolates and refuses",()=>{
 const install=JSON.parse(readFileSync(new URL("../../💾️storage/🧬️schema/📥️install.json",import.meta.url),"utf8")),recipe=install.$defs.ProducerRecipeProtocol.const,source=readFileSync(new URL("../../💾️storage/🟦️.ts",import.meta.url),"utf8");
 expect(protocol.producer.environmentPrefixes).toEqual(recipe.clearEnvironmentPrefixes);
 expect(Object.keys(protocol.producer.storeRoleFiles)).toEqual([recipe.home,recipe.config]);
 expect(protocol.producer.storeRoleFiles[recipe.home as keyof typeof protocol.producer.storeRoleFiles]).toEqual([".npmrc",".bunfig.toml"]);
 expect(source).toContain('[[installSchema.$defs.ProducerRecipeProtocol.const.home,[".bunfig.toml",".npmrc"]],[installSchema.$defs.ProducerRecipeProtocol.const.config,[".bunfig.toml"]]]');
 expect(source).toContain("External installer configuration retained");
 expect(source).toContain("for(const candidate of[path.join(home,\".bunfig.toml\"),path.join(config,\".bunfig.toml\"),path.join(home,\".npmrc\")])");
 for(const key of recipe.clearEnvironmentKeys)expect(protocol.producer.environmentNames.includes(key),key).toBe(false);
 expect(new Set(protocol.producer.environmentIgnored.map((name:string)=>name.toUpperCase())).size).toBe(protocol.producer.environmentIgnored.length);
});

test("generation receipt glue: a captured complete walk verifies under real read custody and every storage fact falls back when it moves",()=>issued.withScriptProcessEnvelope(issued.createScriptProcessEnvelope({version:1,owner:"generation-receipt-glue",maximumElapsedMilliseconds:60000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),async original=>{
 const{root,store}=materialize();
 try{
  const generation="e".repeat(64),directory=join(store,"j",generation.slice(0,20)),bindings=join(directory,"bindings.json"),target=join(directory,"node_modules");
  mkdirSync(target,{recursive:true});writeFileSync(bindings,"{\"ready\":true}\n");mkdirSync(join(root,"pkgs/a"),{recursive:true});rmSync(join(root,"pkgs/a/node_modules"),{force:true});symlinkSync(target,join(root,"pkgs/a/node_modules"));
  const phase=issued.createIssuedResourceControlV1(original,"dependencies:generation-glue",0),context:storage.JavascriptDependencyReadContextV1=Object.freeze({root:store,invocation:original,control:phase,check:(actual:storage.JavascriptDependencyReadContextV1["control"])=>issued.requireIssuedResourceControlRefinementV1(phase,actual),recheck:async(actual:storage.JavascriptDependencyReadContextV1["control"])=>{issued.requireIssuedResourceControlRefinementV1(phase,actual);}});
  const pairs=[{sourceDirectory:join(root,"pkgs/a/node_modules"),targetDirectory:target}],observation=storage.observeJavascriptDependencyBindingsReadV1(context,pairs,phase),receipt=await storage.readJavascriptDependencyBindingReceiptReadV1(context,bindings,phase);
  const walked=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed());
  const providers=[join(store,"j/g1/node_modules/dep/package.json"),join(store,"j/g1/node_modules/dep"),join(root,"node_modules")].map(path=>{const f=facts(path);return{path,stamp:f?JSON.stringify(protocol.stampFields.map(key=>String(f[key as keyof typeof f]))):null,anchor:f?JSON.stringify(protocol.anchorFields.map(key=>String(f[key as keyof typeof f]))):null,kind:f?f.isDirectory()?"directory":"file":"missing",target:null,full:path.endsWith("package.json")};});
  const scopes=[{scope:".",sha256:"f".repeat(64)}],receipts=[{scope:".",generation,fingerprint:receipt.fingerprint}],sha256=generationSha256V1(JSON.stringify([receipts,[[".",observation]],scopes]));
  const f=lstatSync(join(root,"package.json"));
  const walk={version:1 as const,required:["."],projection:{owners:{sha256:"d".repeat(64)},recipeSha256:"c".repeat(64),plans:[{scope:".",plan:{directory,generation,bindings:pairs,sources:[{path:join(root,"package.json"),kind:"file" as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(join(root,"package.json"))}]}}]},receipts,sha256,evidence:{scope:".",execution:{scopes,runtimeSha256:storage.javascriptDependencySourceOwnerRuntimeV1(),observations:providers},bindings:[[".",observation]]},observed:()=>walked};
  const subject=subjectOf(root,Date.now()+10000),generationControl={signal:phase.signal,check:()=>{},tick:async()=>{}},captured=await glue.captureJavascriptDependencyGenerationReceiptV1(root,{root:store},generationControl,walk as never,walked,Date.now()+10000,original,subject);
  expect(captured.status).toBe("captured");if(captured.status!=="captured")return;
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).toEqual({status:"absent"});
  const write=()=>{mkdirSync(join(store,protocol.directory),{recursive:true});for(const file of captured.files)writeFileSync(join(store,protocol.directory,file.name),file.bytes);writeFileSync(join(store,protocol.directory,generationSha256V1(".").slice(0,16)+"."+captured.receipt.sha256+protocol.receiptSuffix),JSON.stringify(captured.receipt)+"\n");};
  write();
  const match=await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject);expect(match.status).toBe("match");if(match.status==="match"){expect(match.sha256).toBe(sha256);expect(match.receipts).toEqual(receipts);}
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,"other",subject)).toEqual({status:"absent"});
  writeFileSync(join(root,"package.json"),readFileSync(join(root,"package.json"),"utf8")+" ");
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).toMatchObject({status:"mismatch",class:"input-changed"});
  const second=await glue.captureJavascriptDependencyGenerationReceiptV1(root,{root:store},generationControl,{...walk,projection:{...walk.projection,plans:[{...walk.projection.plans[0]!,plan:{...walk.projection.plans[0]!.plan,sources:[{...walk.projection.plans[0]!.plan.sources[0]!,bytes:readFileSync(join(root,"package.json")),ino:String(lstatSync(join(root,"package.json")).ino)}]}}]}} as never,await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed()),Date.now()+10000,original,subject);
  expect(second.status).toBe("captured");if(second.status!=="captured")return;
  for(const file of second.files)writeFileSync(join(store,protocol.directory,file.name),file.bytes);writeFileSync(join(store,protocol.directory,generationSha256V1(".").slice(0,16)+"."+second.receipt.sha256+protocol.receiptSuffix),JSON.stringify(second.receipt)+"\n");
  expect((await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).status).toBe("match");
  const before=readdirSync(join(store,protocol.directory)).length;glue.retireJavascriptDependencyGenerationFilesV1(store,".");const after=readdirSync(join(store,protocol.directory));
  expect(after.filter(name=>name.endsWith(protocol.receiptSuffix)).length).toBe(1);expect(after.length).toBeLessThan(before);
  expect((await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).status).toBe("match");
  rmSync(join(store,"j/g1/node_modules/dep/package.json"));writeFileSync(join(store,"j/g1/node_modules/dep/package.json"),"{}");
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).toMatchObject({status:"mismatch",class:"provider-changed"});
  writeFileSync(join(store,protocol.directory,after.find(name=>name.endsWith(protocol.receiptSuffix))!),"{broken");
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).toMatchObject({status:"mismatch",class:"receipt-invalid"});
 }finally{rmSync(root,{recursive:true,force:true});}
}),60000);
