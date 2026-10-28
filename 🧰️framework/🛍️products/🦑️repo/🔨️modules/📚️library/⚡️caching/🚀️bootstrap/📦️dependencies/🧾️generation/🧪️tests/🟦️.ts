import {test,expect} from "bun:test";
import {javascriptInstallSchemaV1 as materializedInstallSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {withCurrentBunWorkspaceOwnershipV1,readBunWorkspaceListingProjectionV1} from "../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,readlinkSync,realpathSync,renameSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {dirname,join,relative} from "node:path";
import {captureGenerationReceiptV1,describeGenerationMismatchV1,generationAnchorV1,generationCanonicalJsonV1,generationProjectionDigestV1,generationProtocolV1 as protocol,generationSha256V1,generationStampV1,parseGenerationReceiptV1,sealGenerationReceiptV1,verifyGenerationReceiptV1,type GenerationCaptureInputV1,type GenerationControlV1,type GenerationObservationV1,type GenerationReceiptV1,type GenerationRuntimeV1,type GenerationVerifyInputV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🔣️.json" with {type:"json"};
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../🏗️build/📥️caller/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as issued from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as storage from "../../💾️storage/🟦️.ts";
import * as readContext from "../../💾️storage/📥️read-context/🟦️.ts";
import * as glue from "../💾️storage/🟦️.ts";
import {startChangeJournalServiceV1} from "../../../../../🏗️build/🏡️resident/📜️journal/🟦️.ts";
import {readChangeStateV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {openVouchV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";

const runtime:GenerationRuntimeV1={manager:"0.0.0",revision:"fixture",node:"0.0.0",platform:"fixture",arch:"fixture",sha256:"a".repeat(64)};

type Mutation={op:string;path:string;text?:string;target?:string};
type Case={name:string;mutations:Mutation[];expect:string;tamper?:string;port?:string;relist?:number;reseal?:string;reason?:{path?:string;detail?:string};oracle?:boolean};
type Ambient={env:Record<string,string|undefined>;home:string;config:string};
type Subject=glue.JavascriptDependencyGenerationSubjectV1;
const withOriginal=<T>(body:(original:ScriptInvocation)=>Promise<T>):Promise<T>=>issued.withScriptProcessEnvelope(issued.createScriptProcessEnvelope({version:1,owner:"generation-receipt",maximumElapsedMilliseconds:600000,},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),body);
const subjectOf=(root:string,env:Record<string,string|undefined>={}):Subject=>Object.freeze({ambient:Object.freeze({env,home:join(root,".ambient/home"),config:join(root,".ambient/xdg")})});

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

async function capture(root:string,store:string,startedAtMs:number,subject:Subject=subjectOf(root)){
 const walked=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed()),
  source=(path:string)=>{const f=lstatSync(path);return{path,kind:"file"as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(path)};},
  absent=(path:string)=>({path,kind:"missing"as const,dev:"",ino:"",mode:0,bytes:Buffer.alloc(0)}),
  directory=(path:string)=>{const f=lstatSync(path);return{path,kind:"directory"as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:Buffer.alloc(0)};},
  pairs=[{sourceDirectory:join(root,"pkgs/a/node_modules"),targetDirectory:join(store,"j/g1/node_modules")}],
  observed=fixtures.workspace.observed,observations=[...observed.full.map(path=>row(join(root,path),true)),...observed.anchors.map(path=>row(join(root,path),false)),...observed.links.map(path=>row(join(root,path),true)),...observed.missing.map(path=>row(join(root,path),false))],
  input:GenerationCaptureInputV1={
   workspace:root,storeRoot:store,scope:".",startedAtMs,runtime,producer:glue.observeJavascriptDependencyGenerationProducerV1(root,store,["."],subject.ambient),recipeSha256:"c".repeat(64),ownershipSha256:"d".repeat(64),required:["."],
   inputs:[{scope:".",sources:[directory(root),source(join(root,"package.json")),source(join(root,"bun.lock")),absent(join(root,"bunfig.toml"))]}],
   membership:walked,bindings:[{scope:".",pairs,observation:bindingText(pairs)}],ready:[{scope:".",generation:"e".repeat(64),path:join(store,"j/g1/bindings.json"),fingerprint:readyFingerprint(join(store,"j/g1/bindings.json"))}],
   execution:{scopes:[{scope:".",sha256:"f".repeat(64)}],runtimeSha256:runtime.sha256,observations},scopeReadSha256:"0".repeat(64),control:control({ticks:0,checks:0}),
  };
 const receiptsAndBindings=[{scope:".",generation:"e".repeat(64),fingerprint:input.ready[0]!.fingerprint}],scopeReadSha256=generationSha256V1(JSON.stringify([receiptsAndBindings,[[".",input.bindings[0]!.observation]],input.execution.scopes])),sealed={...input,scopeReadSha256};
 return{input:sealed,result:await captureGenerationReceiptV1(sealed),walked};
}
function verifyInput(root:string,store:string,receipt:GenerationReceiptV1,files:Map<string,Uint8Array>,counter:{ticks:number;checks:number},abort?:AbortController,port?:string,subject:Subject=subjectOf(root)):GenerationVerifyInputV1{
 return{
  workspace:root,storeRoot:store,receipt,readEvidence:name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},
  producer:()=>glue.observeJavascriptDependencyGenerationProducerV1(root,store,receipt.required,subject.ambient),
  runtime:()=>port==="runtime"?{...runtime,manager:"9.9.9"}:port==="revision"?{...runtime,revision:"other"}:runtime,conditionsClosed:()=>port!=="conditions",
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
const keyingOracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url)),python=()=>testPythonV1()??"python3";
const oracleVerify=(root:string,store:string,receipt:GenerationReceiptV1)=>{
 const file=root+".receipt.json";writeFileSync(file,JSON.stringify(receipt));
 const run=spawnSync(python(),["-I",keyingOracle,"verify",file,root,store,JSON.stringify(runtime),schemaPath],{encoding:"utf8"});rmSync(file,{force:true});
 if(run.status!==0)throw Error("keying oracle failed: "+run.stderr);return JSON.parse(run.stdout)as{status:string;class?:string;path?:string;detail?:string};
};

test("generation receipt contract: schema closes the mismatch classes the fixtures drive and a third-party validator accepts the sealed receipt",async()=>{
 const{root,store}=materialize();
 try{
  const{result}=await capture(root,store,Date.now()+10000);expect(result.status).toBe("captured");if(result.status!=="captured")return;
  const ajv=new Ajv({strict:false}).addSchema(schema),validate=ajv.getSchema(schema.$id+"#/$defs/Receipt")!;
  expect(validate(JSON.parse(JSON.stringify(result.receipt))),JSON.stringify(validate.errors)).toBe(true);
  const reached=new Set(fixtures.cases.map(row=>row.expect.split(":")[1]).filter(Boolean));
  for(const name of protocol.mismatchClasses)if(!["receipt-invalid","unreadable"].includes(name))expect(reached.has(name),name).toBe(true);
  expect(new Set(protocol.mismatchClasses).size).toBe(protocol.mismatchClasses.length);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("generation receipt: every fixture case verifies exactly as declared (match or typed fallback)",async()=>{
 for(const row of fixtures.cases as Case[]){
  const{root,store,place}=materialize(),env:Record<string,string|undefined>={...fixtures.environment},subject=subjectOf(root,env);
  try{
   const{result}=await capture(root,store,Date.now()+10000,subject);expect(result.status,row.name).toBe("captured");if(result.status!=="captured")continue;
   let receipt=result.receipt;const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes]));
   if(row.tamper==="frontier"||row.tamper==="execution"){const name=row.tamper==="frontier"?receipt.membership.frontier.name:receipt.execution.evidence.name;files.set(name,Buffer.concat([files.get(name)!,Buffer.from("\n")]));}
   if(row.reseal==="scopeReadSha256"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:"1".repeat(64)});}
   if(row.reseal==="walkStamp"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,walkStamp:protocol.walkStamp.replace(/\d+$/u,"0")});}
   const before=protocol.maximumRelist;
   try{
    if(row.relist)(protocol as {maximumRelist:number}).maximumRelist=row.relist;
    for(const mutation of row.mutations)mutate(root,place,mutation,env);
    const counter={ticks:0,checks:0},verdict=await verifyGenerationReceiptV1(verifyInput(root,store,receipt,files,counter,undefined,row.port,subject));
    expect(outcome(verdict),row.name).toBe(row.expect);
    if(row.reason)expect(verdict,row.name).toMatchObject(row.reason);
    if(row.oracle){const python=oracleVerify(root,store,receipt);expect(python.status==='match'?'match':'mismatch:'+python.class,row.name).toBe(row.expect);if(row.reason)expect(python,row.name).toMatchObject(row.reason);}
    if(verdict.status==="match"){expect(verdict.required).toEqual(["."]);expect(verdict.receipts[0]!.fingerprint).toBe(readyFingerprint(join(store,"j/g1/bindings.json")));}
   }finally{(protocol as {maximumRelist:number}).maximumRelist=before;}
  }finally{rmSync(root,{recursive:true,force:true});}
 }
},60000);

test("generation receipt recheck: every fixture case answers as the complete verification except that the provider observations are not replayed",async()=>{
 let provider=0,same=0;
 for(const row of fixtures.cases as Case[]){
  const{root,store,place}=materialize(),env:Record<string,string|undefined>={...fixtures.environment},subject=subjectOf(root,env);
  try{
   const{result}=await capture(root,store,Date.now()+10000,subject);expect(result.status,row.name).toBe("captured");if(result.status!=="captured")continue;
   let receipt=result.receipt;const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes]));
   if(row.tamper==="frontier"||row.tamper==="execution"){const name=row.tamper==="frontier"?receipt.membership.frontier.name:receipt.execution.evidence.name;files.set(name,Buffer.concat([files.get(name)!,Buffer.from("\n")]));}
   if(row.reseal==="scopeReadSha256"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:"1".repeat(64)});}
   if(row.reseal==="walkStamp"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,walkStamp:protocol.walkStamp.replace(/\d+$/u,"0")});}
   const before=protocol.maximumRelist;
   try{
    if(row.relist)(protocol as {maximumRelist:number}).maximumRelist=row.relist;
    for(const mutation of row.mutations)mutate(root,place,mutation,env);
    const full=await verifyGenerationReceiptV1(verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject)),again=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),recheck:true});
    const skipped=outcome(full)==="mismatch:provider-changed"||row.tamper==="execution"&&outcome(full)==="mismatch:evidence-invalid";
    expect(outcome(again),row.name).toBe(skipped?"match":outcome(full));
    if(skipped)provider++;else{same++;if(full.status==="mismatch"&&again.status==="mismatch"){expect(again.path,row.name).toBe(full.path);expect(again.detail,row.name).toBe(full.detail);}}
    if(again.status==="match"&&full.status==="match")expect(again.sha256,row.name).toBe(full.sha256);
   }finally{(protocol as {maximumRelist:number}).maximumRelist=before;}
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 expect(provider).toBeGreaterThan(0);expect(same).toBeGreaterThan(provider);
},120000);

const identityOf=(path:string):string=>{
 try{const f=lstatSync(path,{bigint:true});return[f.isSymbolicLink()?"l:"+readlinkSync(path):f.isDirectory()?"d:"+readdirSync(path).sort().join(","):"f",f.ino,f.mode,f.size,f.mtimeNs,f.ctimeNs].join("|");}catch{return"absent";}
};
const treeIdentity=(root:string):Map<string,string>=>{
 const rows=new Map<string,string>(),walk=(directory:string)=>{for(const name of readdirSync(directory)){const path=join(directory,name);rows.set(relative(root,path).split("\\").join("/"),identityOf(path));if(lstatSync(path).isDirectory())walk(path);}};
 walk(root);return rows;
};
test("generation receipt with a vouch: every fixture case answers as the complete verification when the vouch proves exactly the unchanged paths, a vouch that lies is caught, and the vouched look skips rows",async()=>{
 let skipped=0,caught=0,compared=0;
 for(const row of fixtures.cases as Case[]){
  const{root,store,place}=materialize(),env:Record<string,string|undefined>={...fixtures.environment},subject=subjectOf(root,env);
  try{
   const{result}=await capture(root,store,Date.now()+10000,subject);expect(result.status,row.name).toBe("captured");if(result.status!=="captured")continue;
   let receipt=result.receipt;const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes]));
   if(row.tamper==="frontier"||row.tamper==="execution"){const name=row.tamper==="frontier"?receipt.membership.frontier.name:receipt.execution.evidence.name;files.set(name,Buffer.concat([files.get(name)!,Buffer.from("\n")]));}
   if(row.reseal==="scopeReadSha256"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:"1".repeat(64)});}
   if(row.reseal==="walkStamp"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,walkStamp:protocol.walkStamp.replace(/\d+$/u,"0")});}
   const before=protocol.maximumRelist,identities=treeIdentity(root);
   try{
    if(row.relist)(protocol as {maximumRelist:number}).maximumRelist=row.relist;
    for(const mutation of row.mutations)mutate(root,place,mutation,env);
    const full=await verifyGenerationReceiptV1(verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject));
    const honest=(paths:readonly string[])=>new Set(paths.filter(path=>identityOf(join(root,path))===(identities.get(path)??"absent")));
    const liar=(paths:readonly string[])=>new Set(paths);
    const asked:number[]=[],withVouch=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),vouched:paths=>{const proven=honest(paths);asked.push(proven.size);return proven;}});
    expect(outcome(withVouch),row.name).toBe(outcome(full));
    if(full.status==="mismatch"&&withVouch.status==="mismatch"){expect(withVouch.path,row.name).toBe(full.path);expect(withVouch.detail,row.name).toBe(full.detail);}
    if(full.status==="match"&&withVouch.status==="match")expect(withVouch.sha256,row.name).toBe(full.sha256);
    const again=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),recheck:true,vouched:honest}),plain=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),recheck:true});
    expect(outcome(again),row.name).toBe(outcome(plain));
    skipped+=asked.reduce((sum,count)=>sum+count,0);compared++;
    const lied=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),vouched:liar});
    if(outcome(lied)!==outcome(full))caught++;
   }finally{(protocol as {maximumRelist:number}).maximumRelist=before;}
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 expect(compared).toBeGreaterThan(10);expect(skipped).toBeGreaterThan(0);expect(caught).toBeGreaterThan(0);
},180000);

const watcher=process.env.SEMIO_REPO_WATCH_BINARY??"";
test.skipIf(!watcher)("generation receipt with a REAL journal: every fixture case answers as the complete verification when the vouch comes from the file watcher of the workspace, in the complete pass and in the recheck, and the unchanged rows are skipped",async()=>{
 let skipped=0,compared=0;
 for(const row of fixtures.cases as Case[]){
  const{root,store,place}=materialize(),env:Record<string,string|undefined>={...fixtures.environment},subject=subjectOf(root,env),directory=join(root+".journal"),environment={...process.env,SEMIO_REPO_WATCH_BINARY:watcher,SEMIO_REPO_CHANGES:"on"};
  try{
   const{result}=await capture(root,store,Date.now()+10000,subject);expect(result.status,row.name).toBe("captured");if(result.status!=="captured")continue;
   let receipt=result.receipt;const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes]));
   if(row.tamper==="frontier"||row.tamper==="execution"){const name=row.tamper==="frontier"?receipt.membership.frontier.name:receipt.execution.evidence.name;files.set(name,Buffer.concat([files.get(name)!,Buffer.from("\n")]));}
   if(row.reseal==="scopeReadSha256"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,scopeReadSha256:"1".repeat(64)});}
   if(row.reseal==="walkStamp"){const{sha256:_,...body}=receipt;receipt=sealGenerationReceiptV1({...body,walkStamp:protocol.walkStamp.replace(/\d+$/u,"0")});}
   const client=createChangeJournalClientV1({directory,root,environment,start:()=>startChangeJournalServiceV1({directory,root,environment})}),before=protocol.maximumRelist;
   try{
    const first=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"fixture",key:receipt.sha256});
    expect(first.mode,row.name).toBe("cursor");
    const sealing=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),vouched:paths=>first.skippable(paths)});
    if(outcome(sealing)!=="match")continue;
    await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),recheck:true,vouched:paths=>first.settled(paths)});
    await first.seal();
    if(row.relist)(protocol as {maximumRelist:number}).maximumRelist=row.relist;
    for(const mutation of row.mutations)mutate(root,place,mutation,env);
    const full=await verifyGenerationReceiptV1(verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject));
    const second=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"fixture",key:receipt.sha256});
    expect(second.mode,row.name).toBe("journal");
    const asked:number[]=[],vouched=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),vouched:paths=>{const proven=second.skippable(paths);asked.push(proven.size);return proven;}});
    expect(outcome(vouched),row.name).toBe(outcome(full));
    if(full.status==="mismatch"&&vouched.status==="mismatch"){expect(vouched.path,row.name).toBe(full.path);expect(vouched.detail,row.name).toBe(full.detail);}
    const plain=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),recheck:true}),settled=await verifyGenerationReceiptV1({...verifyInput(root,store,receipt,files,{ticks:0,checks:0},undefined,row.port,subject),recheck:true,vouched:paths=>second.settled(paths)});
    expect(outcome(settled),row.name).toBe(outcome(plain));
    skipped+=asked.reduce((sum,count)=>sum+count,0);compared++;
   }finally{(protocol as {maximumRelist:number}).maximumRelist=before;const pid=readChangeStateV1(directory)?.pid;if(pid)try{process.kill(pid,"SIGTERM");}catch{}}
  }finally{rmSync(root,{recursive:true,force:true});rmSync(root+".journal",{recursive:true,force:true});}
 }
 expect(compared).toBeGreaterThan(10);expect(skipped).toBeGreaterThan(0);
},300000);

test("generation receipt: a tampered or schema-invalid receipt is refused before any filesystem read",async()=>{
 const{root,store}=materialize();
 try{
  const{result}=await capture(root,store,Date.now()+10000);if(result.status!=="captured")throw Error("captured expected");
  const text=JSON.stringify(result.receipt),bytes=Buffer.from(text);expect(parseGenerationReceiptV1(bytes).sha256).toBe(result.receipt.sha256);
  expect(()=>parseGenerationReceiptV1(Buffer.from(text.replace('"fixture"','"fixturf"')))).toThrow("Sealed dependency generation receipt");
  const{sha256:_,...body}=result.receipt;
  expect(()=>parseGenerationReceiptV1(Buffer.from(JSON.stringify({...body,version:2,sha256:generationSha256V1(generationCanonicalJsonV1({...body,version:2}))})))).toThrow("Closed dependency generation receipt");
  expect(()=>parseGenerationReceiptV1(Buffer.from(JSON.stringify({...result.receipt,extra:true})))).toThrow("Closed dependency generation receipt");
  expect(()=>parseGenerationReceiptV1(Buffer.alloc(protocol.maximumReceiptBytes+1))).toThrow("Bounded dependency generation receipt");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("generation receipt: capture defers rather than seal an unsettled timestamp",async()=>{
 const{root,store}=materialize();
 try{
  const settledSubject=subjectOf(root),young=await capture(root,store,Date.now()-10000,settledSubject);expect(young.result.status).toBe("deferred");
  const settled=await capture(root,store,Date.now()+10000);expect(settled.result.status).toBe("captured");
  const volatile=settled.result.status==="captured"?new TextDecoder().decode(settled.result.files[0]!.bytes).split("\n").filter(Boolean).map(line=>JSON.parse(line)as unknown[]).filter(line=>line[0]==="d"&&line[7]===0):[];expect(volatile).toEqual([]);
  const mixed=await capture(root,store,Date.now(),settledSubject);
  if(mixed.result.status==="captured"){const rows=new TextDecoder().decode(mixed.result.files[0]!.bytes).split("\n").filter(Boolean).map(line=>JSON.parse(line)as unknown[]);expect(rows.some(line=>line[0]==="d"&&line[7]===0)).toBe(true);}
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("generation receipt: verification honors cancellation and publishes bounded progress",async()=>{
 const{root,store}=materialize();
 try{
  for(let index=0;index<fixtures.progress.directories;index++)mkdirSync(join(root,"pkgs/a/src/d"+index),{recursive:true});
  const{result}=await capture(root,store,Date.now()+10000);if(result.status!=="captured")throw Error("captured expected");
  const files=new Map<string,Uint8Array>(result.files.map(file=>[file.name,file.bytes])),counter={ticks:0,checks:0},verdict=await verifyGenerationReceiptV1(verifyInput(root,store,result.receipt,files,counter));
  expect(verdict.status).toBe("match");expect(counter.ticks).toBeGreaterThan(0);expect(counter.checks).toBeGreaterThan(counter.ticks);
  const abort=new AbortController(),stopper={ticks:0,checks:0},input=verifyInput(root,store,result.receipt,files,stopper,abort);
  const cancelling:GenerationVerifyInputV1={...input,control:{signal:abort.signal,check:()=>abort.signal.throwIfAborted(),tick:async()=>{abort.abort(Object.freeze({cancel:"fixture"}));abort.signal.throwIfAborted();}}};
  await expect(verifyGenerationReceiptV1(cancelling)).rejects.toMatchObject({cancel:"fixture"});
  const failing:GenerationVerifyInputV1={...input,control:{signal:new AbortController().signal,check:()=>{},tick:async()=>{}},project:()=>{throw Error("directory unreadable");}};
  writeFileSync(join(root,"pkgs/a/touched.txt"),"x");const stale=new Date(Date.now()+60000);utimesSync(join(root,"pkgs/a"),stale,stale);
  expect(outcome(await verifyGenerationReceiptV1(failing))).toBe("mismatch:unreadable");
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

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
out, pruned, relevant = {}, {}, {}
def walk(directory):
    entries = sorted(os.scandir(directory), key=lambda e: e.name.encode("utf-8"))
    rows = [{"name": e.name, "kind": kind(e)} for e in entries if e.name == "package.json" or (not e.name.startswith(".") and not is_opaque(e.name) and kind(e) in ("symlink", "directory"))]
    digest = lambda items: hashlib.sha256(json.dumps(items, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()[:32]
    key = os.path.relpath(directory, root)
    out[key] = digest(rows)
    held = any(r["name"] == "package.json" or r["kind"] == "symlink" for r in rows)
    for e in entries:
        if e.name.startswith(".") or is_opaque(e.name): continue
        if e.is_dir(follow_symlinks=False): held = walk(os.path.join(directory, e.name)) or held
    relevant[key] = held
    pruned[key] = digest([r for r in rows if r["kind"] != "directory" or relevant[os.path.relpath(os.path.join(directory, r["name"]), root)]])
    return held
for start in rules["walkRoots"]: walk(os.path.join(root, start))
seal = json.dumps(json.loads(sys.argv[3]), sort_keys=True, separators=(",", ":"), ensure_ascii=False)
print(json.dumps({"directories": out, "pruned": pruned, "relevant": {k: int(v) for k, v in relevant.items()}, "seal": hashlib.sha256(seal.encode("utf-8")).hexdigest()}, ensure_ascii=False))
`;

test("generation receipt: the frontier and the seal equal an independent Python implementation",async()=>{
 const{root,store}=materialize();
 try{
  const{result,walked}=await capture(root,store,Date.now()+10000);if(result.status!=="captured")throw Error("captured expected");
  const{sha256:_,...body}=result.receipt,run=spawnSync(testPythonV1()??"python3",["-I","-c",oracle,root,JSON.stringify(fixtures.rules),JSON.stringify(body)],{encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);
  const python=JSON.parse(run.stdout)as{directories:Record<string,string>;pruned:Record<string,string>;relevant:Record<string,number>;seal:string},ours=Object.fromEntries(walked.paths.filter(path=>path.list).map(path=>[relative(root,path.path),generationProjectionDigestV1(path.list!)]));
  expect(ours).toEqual(python.directories);expect(Object.keys(ours).length).toBeGreaterThan(3);expect(python.seal).toBe(result.receipt.sha256);
  const frontier=new TextDecoder().decode(result.files[0]!.bytes).split("\n").filter(Boolean).map(line=>JSON.parse(line)as unknown[]);
  expect(Object.fromEntries(frontier.filter(line=>line[0]==="d").map(line=>[line[1],line[8]]))).toEqual(python.directories);
  expect(Object.fromEntries(frontier.filter(line=>line[0]==="d").map(line=>[line[1],line[9]]))).toEqual(python.pruned);
  expect(Object.fromEntries(frontier.filter(line=>line[0]==="d").map(line=>[line[1],line[10]]))).toEqual(python.relevant);
  expect(walked.texts.map(text=>text.path).sort()).toEqual(["package.json","pkgs/a/package.json","pkgs/b/package.json","pkgs/\u{1F9E9}️widget/package.json"].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("generation receipt: stamp and anchor equal the provider walk's own formulas and the storage laws stay pinned",()=>{
 const storage=readFileSync(new URL("../../💾️storage/🟦️.ts",import.meta.url),"utf8"),install=materializedInstallSchema;
 expect(storage).toContain('keys=["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"]as const');
 expect(storage).toContain("stamp=(row:BigIntStats)=>JSON.stringify(keys.map(key=>String(row[key])))");
 expect(storage).toContain("anchor=(row:BigIntStats)=>JSON.stringify(protocol.ancestorFields.map((key:string)=>String(row[key as keyof BigIntStats])))");
 expect(storage).toContain("if(process.env.NODE_OPTIONS||process.env.BUN_OPTIONS||process.execArgv.some(value=>/^--(?:conditions|preserve-symlinks)(?:=|$)/u.test(value)))throw Error(\"Closed declared execution conditions required\")");
 expect(install.$defs.SourceOwnerExecutionProtocol.const.ancestorFields).toEqual(protocol.anchorFields);
 expect(protocol.stampFields).toEqual(["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"]);
 const f=lstatSync(import.meta.path,{bigint:true}),keys=protocol.stampFields as readonly string[];
 expect(JSON.stringify(generationStampV1(f).split(":"))).toBe(JSON.stringify(keys.map(key=>String(f[key as keyof typeof f]))));
 expect(JSON.stringify(generationAnchorV1(f).split(":"))).toBe(JSON.stringify(install.$defs.SourceOwnerExecutionProtocol.const.ancestorFields.map((key:string)=>String(f[key as keyof typeof f]))));
 const walk=readFileSync(new URL("../../../../../🗂️workspaces/🟦️bun/🟨️.cjs",import.meta.url),"utf8");
 expect(walk.match(/projectListing\(/gu)!.length).toBe(6);
 expect(walk.match(/entry\.name === "package\.json"/gu)!.length).toBe(2);
});

test("generation receipt keying: the sealed receipt names the declared walk protocol and nothing about the code that performs the walk",async()=>{
 const{root,store}=materialize();
 try{
  const{result}=await capture(root,store,Date.now()+10000);expect(result.status).toBe("captured");if(result.status!=="captured")return;
  expect(result.receipt.walkStamp).toBe(protocol.walkStamp);
  expect(Object.keys(result.receipt).sort()).toEqual([...schema.$defs.Receipt.required].sort());
  expect(result.files.map(file=>file.name.split(".")[1]).sort()).toEqual(["execution","frontier"]);
  expect(JSON.stringify(schema)).not.toMatch(/closure|"implementation"/iu);
  for(const module of ["../🟦️.ts","../💾️storage/🟦️.ts"]){const source=readFileSync(new URL(module,import.meta.url),"utf8");for(const forbidden of ["captureBuildSourceClosureV1","implementationFile","implementationEntries","loadedAtMs","implementation-unsettled"])expect(source,module+" "+forbidden).not.toContain(forbidden);}
  expect(readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),"repository data is read through descriptors, never the loader's guarded file reads").not.toContain("readFileSync");
  const storage=readFileSync(new URL("../💾️storage/🟦️.ts",import.meta.url),"utf8"),reader=storage.slice(storage.indexOf("export async function verifyJavascriptDependencyGenerationReceiptV1"),storage.indexOf("const colon="));
  expect(reader.length).toBeGreaterThan(200);expect(reader).not.toMatch(/acquire\w*Lease|"exclusive"/u);expect(reader).toContain('"shared"');
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("generation receipt reasons: every mismatch class names its exact input in English and German, equal to an independent Python renderer",()=>{
 const texts=protocol.texts as Record<string,Record<string,string>>,placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(row=>row[1]).sort();
 expect(protocol.languages).toEqual(["en","de"]);expect(Object.keys(texts).sort()).toEqual([...protocol.languages].sort());
 expect(Object.keys(texts.de!).sort()).toEqual(Object.keys(texts.en!).sort());
 for(const name of Object.keys(texts.en!)){expect(placeholders(texts.de![name]!),name).toEqual(placeholders(texts.en![name]!));expect(texts.de![name],name).not.toBe(texts.en![name]);}
 for(const name of [...protocol.mismatchClasses,"absent","seal","stale"])expect(Object.keys(texts.en!),name).toContain(name);
 const rows=fixtures.reasons.map(({en:_en,de:_de,...row})=>row),run=spawnSync(python(),["-I",keyingOracle,"render",schemaPath,JSON.stringify(rows)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const independent=JSON.parse(run.stdout)as {en:string;de:string}[];
 for(const[index,row]of fixtures.reasons.entries()){
  expect(describeGenerationMismatchV1("en",row),row.class).toBe(row.en);expect(describeGenerationMismatchV1("de",row),row.class).toBe(row.de);
  expect(independent[index]).toEqual({en:row.en,de:row.de});
 }
 expect(()=>describeGenerationMismatchV1("fr",{class:"sha-changed"})).toThrow("Declared language");
});

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
 const install=materializedInstallSchema,recipe=install.$defs.ProducerRecipeProtocol.const,source=readFileSync(new URL("../../💾️storage/🟦️.ts",import.meta.url),"utf8");
 expect(protocol.producer.environmentPrefixes).toEqual(recipe.clearEnvironmentPrefixes);
 expect(Object.keys(protocol.producer.storeRoleFiles)).toEqual([recipe.home,recipe.config]);
 expect(protocol.producer.storeRoleFiles[recipe.home as keyof typeof protocol.producer.storeRoleFiles]).toEqual([".npmrc",".bunfig.toml"]);
 expect(source).toContain('[[installSchema.$defs.ProducerRecipeProtocol.const.home,installSchema.$defs.ManagerProtocol.manager.configurationNames("home")],[installSchema.$defs.ProducerRecipeProtocol.const.config,installSchema.$defs.ManagerProtocol.manager.configurationNames("config")]]');
 expect(source).toContain("External installer configuration retained");
 expect(source).toContain("for(const candidate of[...installSchema.$defs.ManagerProtocol.manager.configurationNames(\"home\").map((name:string)=>path.join(home,name)),...installSchema.$defs.ManagerProtocol.manager.configurationNames(\"config\").map((name:string)=>path.join(config,name))])");
 for(const key of recipe.clearEnvironmentKeys)expect(protocol.producer.environmentNames.includes(key),key).toBe(false);
 expect(new Set(protocol.producer.environmentIgnored.map((name:string)=>name.toUpperCase())).size).toBe(protocol.producer.environmentIgnored.length);
});

test("generation receipt glue: a captured complete walk verifies under real read custody and every storage fact falls back when it moves",()=>issued.withScriptProcessEnvelope(issued.createScriptProcessEnvelope({version:1,owner:"generation-receipt-glue",maximumElapsedMilliseconds:60000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),async original=>{
 const{root,store}=materialize();
 try{
  const generation="e".repeat(64),directory=join(store,"j",generation.slice(0,20)),bindings=join(directory,"bindings.json"),target=join(directory,"node_modules");
  mkdirSync(target,{recursive:true});writeFileSync(bindings,"{\"ready\":true}\n");mkdirSync(join(root,"pkgs/a"),{recursive:true});rmSync(join(root,"pkgs/a/node_modules"),{force:true});symlinkSync(target,join(root,"pkgs/a/node_modules"));
  const phase=issued.createIssuedResourceControlV1(original,"dependencies:generation-glue",0),context:readContext.JavascriptDependencyReadContextV1=Object.freeze({root:store,invocation:original,control:phase,check:(actual:readContext.JavascriptDependencyReadContextV1["control"])=>issued.requireIssuedResourceControlRefinementV1(phase,actual),recheck:async(actual:readContext.JavascriptDependencyReadContextV1["control"])=>{issued.requireIssuedResourceControlRefinementV1(phase,actual);}});
  const pairs=[{sourceDirectory:join(root,"pkgs/a/node_modules"),targetDirectory:target}],observation=readContext.observeJavascriptDependencyBindingsReadV1(context,pairs,phase),receipt=await readContext.readJavascriptDependencyBindingReceiptReadV1(context,bindings,phase);
  const walked=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed());
  const providers=[join(store,"j/g1/node_modules/dep/package.json"),join(store,"j/g1/node_modules/dep"),join(root,"node_modules")].map(path=>{const f=facts(path);return{path,stamp:f?JSON.stringify(protocol.stampFields.map((key:string)=>String(f[key as keyof typeof f]))):null,anchor:f?JSON.stringify(protocol.anchorFields.map((key:string)=>String(f[key as keyof typeof f]))):null,kind:f?f.isDirectory()?"directory":"file":"missing",target:null,full:path.endsWith("package.json")};});
  const scopes=[{scope:".",sha256:"f".repeat(64)}],receipts=[{scope:".",generation,fingerprint:receipt.fingerprint}],sha256=generationSha256V1(JSON.stringify([receipts,[[".",observation]],scopes]));
  const f=lstatSync(join(root,"package.json"));
  const walk={version:1 as const,required:["."],projection:{owners:{sha256:"d".repeat(64)},recipeSha256:"c".repeat(64),plans:[{scope:".",plan:{directory,generation,bindings:pairs,sources:[{path:join(root,"package.json"),kind:"file" as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(join(root,"package.json"))}]}}]},receipts,sha256,evidence:{scope:".",execution:{scopes,runtimeSha256:readContext.javascriptDependencySourceOwnerRuntimeV1(),observations:providers},bindings:[[".",observation]]},observed:()=>walked};
  const subject=subjectOf(root),generationControl={signal:phase.signal,check:()=>{},tick:async()=>{}},captured=await glue.captureJavascriptDependencyGenerationReceiptV1(root,{root:store},generationControl,walk as never,walked,Date.now()+10000,subject);
  expect(captured.status).toBe("captured");if(captured.status!=="captured")return;
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).toEqual({status:"absent"});
  const write=()=>{mkdirSync(join(store,protocol.directory),{recursive:true});for(const file of captured.files)writeFileSync(join(store,protocol.directory,file.name),file.bytes);writeFileSync(join(store,protocol.directory,generationSha256V1(".").slice(0,16)+"."+captured.receipt.sha256+protocol.receiptSuffix),JSON.stringify(captured.receipt)+"\n");};
  write();
  const match=await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject);expect(match.status).toBe("match");if(match.status==="match"){expect(match.sha256).toBe(sha256);expect(match.receipts).toEqual(receipts);}
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,"other",subject)).toEqual({status:"absent"});
  writeFileSync(join(root,"package.json"),readFileSync(join(root,"package.json"),"utf8")+" ");
  expect(await glue.verifyJavascriptDependencyGenerationReceiptV1(root,context,phase,".",subject)).toMatchObject({status:"mismatch",class:"input-changed"});
  const second=await glue.captureJavascriptDependencyGenerationReceiptV1(root,{root:store},generationControl,{...walk,projection:{...walk.projection,plans:[{...walk.projection.plans[0]!,plan:{...walk.projection.plans[0]!.plan,sources:[{...walk.projection.plans[0]!.plan.sources[0]!,bytes:readFileSync(join(root,"package.json")),ino:String(lstatSync(join(root,"package.json")).ino)}]}}]}} as never,await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed()),Date.now()+10000,subject);
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
