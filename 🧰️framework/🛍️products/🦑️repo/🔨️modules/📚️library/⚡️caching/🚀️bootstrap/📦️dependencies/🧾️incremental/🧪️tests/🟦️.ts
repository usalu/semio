import {test,expect} from "bun:test";
import {javascriptInstallSchemaV1 as materializedInstallSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,linkSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import * as core from "../🟦️.ts";
import * as glue from "../💾️storage/🟦️.ts";
import {generationProtocolV1 as generation} from "../../🧾️generation/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {javascriptInstallSchemaV1 as installSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";

const protocol=core.incrementalProtocolV1,idle={check:()=>{},tick:async()=>{}};
const recipeBackends=()=>(installSchema.$defs.ProducerRecipeProtocol.const as unknown as {backends:Record<string,string>}).backends;
const oracle=(args:string[],input?:string,raw=false):any=>{const run=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),...args],{encoding:"utf8",input,maxBuffer:1<<28});if(run.status!==0)throw Error("oracle failed: "+run.stderr);return raw?run.stdout.trim():JSON.parse(run.stdout);};
const scratch=()=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"inc-")));
const sleep=(ms:number)=>new Promise<void>(accept=>setTimeout(accept,ms));
type Mutation={op:string;path:string;text?:string;target?:string;mode?:number};

function build(root:string,tree:typeof fixture.installed.tree){
 for(const dir of tree.dirs)mkdirSync(join(root,dir),{recursive:true});
 for(const[path,text]of Object.entries(tree.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 for(const[path,target]of Object.entries(tree.links))symlinkSync(target,join(root,path));
}
function mutate(root:string,mutation:Mutation){
 const path=join(root,mutation.path);
 switch(mutation.op){
  case"write":mkdirSync(dirname(path),{recursive:true});writeFileSync(path,mutation.text!);break;
  case"touch":utimesSync(path,new Date(),new Date());break;
  case"rmtree":rmSync(path,{recursive:true});break;
  case"replace":renameSync(path,path+".old");mkdirSync(path);rmSync(path+".old",{recursive:true});break;
  case"filetolink":case"retarget":rmSync(path);symlinkSync(mutation.target!,path);break;
  case"chmod":chmodSync(path,mutation.mode!);break;
  case"dirtofile":rmSync(path,{recursive:true});writeFileSync(path,"file");break;
  default:throw Error("unknown mutation "+mutation.op);
 }
}
function collect(root:string,startMs:number){
 const sink=glue.installedLedgerSinkV1(startMs);
 const walk=(path:string,portable:string):void=>{
  const facts=lstatSync(path,{bigint:true}),names=readdirSync(path).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),kids=names.map(name=>{const state=lstatSync(join(path,name));return{name,kind:state.isSymbolicLink()?"link":state.isDirectory()?"directory":"file"};});
  for(const kid of kids)if(kid.kind==="directory")walk(join(path,kid.name),portable==="."?kid.name:portable+"/"+kid.name);
  sink.directory(portable,facts,kids);
 };
 walk(root,".");return sink.rows;
}
const receiptOf=(rows:readonly core.InstalledRowV1[],device?:string)=>{
 const counts=core.installedCountsV1(rows),encoded=core.encodeInstalledLedgerV1(rows);
 return{encoded,receipt:core.sealInstalledReceiptV1({version:1,purpose:"dependency-installed-paths-receipt",generation:"a".repeat(64),device:device??rows[0]!.stamp[0],shape:{...counts,maximumPathUnits:12,sha256:"b".repeat(64)},evidence:encoded.evidence})};
};

/** 🧬️ Pins the neutral contract: a third-party validator accepts every sealed receipt, the constants are bound to the fixture and none of the new file kinds can be mistaken for a generation receipt or evidence file. */
test("incremental contract: Ajv accepts sealed receipts, constants equal the fixture and file kinds stay disjoint",()=>{
 const ajv=new Ajv({strict:false}).addSchema(schema),validate=(name:string,value:unknown)=>{const check=ajv.getSchema(schema.$id+"#/$defs/"+name)!;expect(check(JSON.parse(JSON.stringify(value))),name+JSON.stringify(check.errors)).toBe(true);};
 const rows=collect(realpathSync(mkdtempSync(join(tmpdir(),"inc-contract-"))),Date.now()),{receipt}=receiptOf(rows);
 validate("InstalledReceipt",receipt);
 validate("SyncReceipt",core.sealSyncReceiptV1({version:1,purpose:"dependency-sync-receipt",selection:"all",runtimeSha256:"c".repeat(64),scopes:[{scope:"✏️s",generation:"a".repeat(64),read:"b".repeat(64),installed:"d".repeat(64)}]}));
 validate("OwnershipReceipt",core.sealOwnershipReceiptV1({version:1,purpose:"dependency-ownership-index",evidence:{name:"a".repeat(64)+".ownership.ndjson",sha256:"a".repeat(64),bytes:0,rows:0}}));
 validate("LockReceipt",core.sealLockReceiptV1({version:1,purpose:"dependency-lock-converged-receipt",scope:".",generation:"a".repeat(64),lockSha256:"b".repeat(64),recipeSha256:"c".repeat(64),runtimeSha256:"d".repeat(64)}));
 expect(fixture.retryPolicy).toEqual(protocol.retry);
 expect(protocol.directory).toBe(generation.directory);
 const suffixes=[protocol.installed.receiptSuffix,protocol.sync.receiptSuffix,protocol.ownership.receiptSuffix,protocol.lock.receiptSuffix,protocol.installed.evidenceSuffix,protocol.ownership.evidenceSuffix,generation.receiptSuffix,generation.frontierSuffix,generation.executionSuffix];
 for(const left of suffixes)for(const right of suffixes)if(left!==right)expect(left.endsWith(right),left+" vs "+right).toBe(false);
 expect(protocol.installed.stampFields).toEqual(generation.stampFields);
 expect(protocol.installed.settleMilliseconds).toBe(generation.settleMilliseconds);
 expect(protocol.execution.settleMilliseconds).toBe(generation.settleMilliseconds);
 expect(protocol.installed.maximumRelist).toBe(generation.maximumRelist);
});

/** 🔏️ Reproduces every seal with an independent Python canonical JSON implementation and refuses tampered, reordered or unclosed receipts. */
test("incremental seals equal the Python canonical digest and every tamper is refused",()=>{
 const rows=collect(realpathSync(mkdtempSync(join(tmpdir(),"inc-seal-"))),Date.now()),{receipt,encoded}=receiptOf(rows),{sha256,...body}=receipt;
 expect(oracle(["seal",JSON.stringify(body)],undefined,true)).toBe(sha256);
 const sync=core.sealSyncReceiptV1({version:1,purpose:"dependency-sync-receipt",selection:"all",runtimeSha256:"c".repeat(64),scopes:[{scope:"🌎️hub",generation:"a".repeat(64),read:"b".repeat(64),installed:"d".repeat(64)},{scope:".",generation:"e".repeat(64),read:"f".repeat(64),installed:"1".repeat(64)}]}),{sha256:syncSha,...syncBody}=sync;
 expect(oracle(["seal",JSON.stringify(syncBody)],undefined,true)).toBe(syncSha);
 expect(sync.scopes.map(row=>row.scope)).toEqual([".","🌎️hub"]);
 const bytes=(value:unknown)=>Buffer.from(JSON.stringify(value)+"\n");
 expect(core.parseInstalledReceiptV1(bytes(receipt)).sha256).toBe(sha256);
 expect(()=>core.parseInstalledReceiptV1(bytes({...receipt,device:"7"}))).toThrow();
 expect(()=>core.parseInstalledReceiptV1(bytes({...receipt,extra:1}))).toThrow();
 expect(()=>core.parseInstalledReceiptV1(bytes({...receipt,generation:"short"}))).toThrow();
 expect(()=>core.parseSyncReceiptV1(bytes({...sync,scopes:[sync.scopes[0],sync.scopes[0]]}))).toThrow();
 const tampered=Buffer.from(encoded.bytes);tampered[3]=tampered[3]===0x30?0x31:0x30;
 expect(()=>core.decodeInstalledLedgerV1(tampered,encoded.evidence)).toThrow();
 expect(()=>core.decodeInstalledLedgerV1(encoded.bytes.subarray(0,encoded.bytes.length-1),encoded.evidence)).toThrow();
});

/** 📚️ Reproduces the ledger rows of a real tree with an independent Python walk: stamps, settle flag, entry digest and per-kind counts. */
test("incremental ledger rows equal the Python walk for settled and unsettled trees",()=>{
 for(const offset of[fixture.installed.futureOffsetMs,fixture.installed.pastOffsetMs]){
  const root=scratch();try{
   build(root,fixture.installed.tree);const start=Date.now()+offset,rows=collect(root,start),expected=oracle(["ledger",root,String(start)]);
   expect(rows.map(row=>[row.path,...row.stamp,row.settled?1:0,row.children,row.directories,row.files,row.links]).sort((left,right)=>Buffer.compare(Buffer.from(left[0] as string),Buffer.from(right[0] as string)))).toEqual(expected.rows);
   expect(core.installedCountsV1(rows)).toEqual({entries:expected.entries,directories:expected.directories,files:expected.files,links:expected.links});
   expect(rows.every(row=>row.settled)).toBe(offset>0);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

/** 🔬️ Executes the closed mutation table against a sealed ledger: the verdict equals the fixture and an independent Python digest comparison, and a settled unchanged directory costs one lstat and no listing. */
test("incremental ledger verification: every mutation is classified like the Python oracle",async()=>{
 for(const row of fixture.installed.cases){
  const root=scratch();try{
   build(root,fixture.installed.tree);
   const offset=row.settled?fixture.installed.futureOffsetMs:fixture.installed.pastOffsetMs,rows=core.decodeInstalledLedgerV1(...(()=>{const encoded=core.encodeInstalledLedgerV1(collect(root,Date.now()+offset));return[encoded.bytes,encoded.evidence]as const;})()),{receipt}=receiptOf(rows,(row as {device?:string}).device);
   for(const mutation of row.mutations)mutate(root,mutation as Mutation);
   let listings=0;const ports=glue.installedPortsV1(root,idle),counted={...ports,children:(path:string)=>{listings++;return ports.children(path);}};
   const result=await core.verifyInstalledLedgerV1(receipt,rows,counted),independent=oracle(["verify"],JSON.stringify({root,device:receipt.device,rows:rows.map(value=>({path:value.path,stamp:value.stamp,children:value.children}))}));
   expect(result.status as string,row.id).toBe(row.expect.status);
   if(result.status==="mismatch"){expect(result.class as string,row.id).toBe((row.expect as {class:string}).class);expect(result.path as string,row.id).toBe((row.expect as {path:string}).path);}
   expect({status:result.status,...(result.status==="mismatch"?{class:result.class,path:result.path}:{})},row.id).toEqual(independent);
   if("relisted" in row.expect){expect(result.status==="match"?result.relisted:-1,row.id).toBe((row.expect as {relisted:number}).relisted);expect(listings,row.id).toBe((row.expect as {relisted:number}).relisted);}
  }finally{rmSync(root,{recursive:true,force:true});}
 }
},120000);

/** 🌊️ Bounds the relisting budget: more unsettled directories than the schema allows send the caller to the complete scan instead of an unbounded relist. */
test("incremental ledger verification reports churn beyond the relist budget",async()=>{
 const root=scratch();try{
  for(let index=0;index<fixture.installed.churnDirectories;index++)mkdirSync(join(root,"d"+index));
  const rows=core.decodeInstalledLedgerV1(...(()=>{const encoded=core.encodeInstalledLedgerV1(collect(root,Date.now()+fixture.installed.pastOffsetMs));return[encoded.bytes,encoded.evidence]as const;})()),{receipt}=receiptOf(rows),result=await core.verifyInstalledLedgerV1(receipt,rows,glue.installedPortsV1(root,idle));
  expect(result.status).toBe("mismatch");expect(result.status==="mismatch"?result.class:"").toBe("churn");
  const cancelling=new AbortController(),stopped={check:()=>cancelling.signal.throwIfAborted(),tick:async()=>{}};cancelling.abort(Object.freeze(Error("cancelled")));
  await expect(core.verifyInstalledLedgerV1(receipt,rows,glue.installedPortsV1(root,stopped))).rejects.toThrow("cancelled");
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

/** 🔂️ Replays recorded provider observations: every answer, the settle deferral and the link target equal the Python decision. */
test("incremental execution replay equals the Python decision table",async()=>{
 const expected=oracle(["expect",fileURLToPath(new URL("./🔣️.json",import.meta.url))]).replay;
 for(const row of fixture.replay){
  const live=row.live as unknown as Record<string,{stamp:string;anchor:string;target?:string}|null>;
  const result=await core.replayExecutionObservationsV1(row.observations as core.ExecutionObservationV1[],row.startedAtMs,{control:idle,stat:path=>live[path]??undefined,readlink:path=>live[path]!.target!});
  expect({status:result.status,...("path"in result?{path:result.path}:{})}as unknown,row.id).toEqual(row.expect);
  expect({status:result.status,...("path"in result?{path:result.path}:{})},row.id).toEqual(expected[row.id]);
 }
});

/** 🔁️ Classifies retries, converged locks and the single-link invariant exactly like the independent Python tables. */
test("incremental decisions equal the Python tables",()=>{
 const expected=oracle(["expect",fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 for(const row of fixture.retry){
  const verdict=core.retryableSyncFailureV1(row.message,{aborted:row.aborted,unresolved:row.unresolved,attempt:row.attempt});
  expect(verdict,row.id).toBe(row.expect);expect(verdict,row.id).toBe(expected.retry[row.id]);
 }
 for(const row of fixture.lock){
  const receipt=core.sealLockReceiptV1({version:1,purpose:"dependency-lock-converged-receipt",...row.receipt}),verdict=core.lockConvergedV1(receipt,row.current);
  expect(verdict,row.id).toBe(row.expect);expect(verdict,row.id).toBe(expected.lock[row.id]);
 }
 for(const row of fixture.singleLink){
  let refused=false;try{core.requireSingleLinkInputV1("a/package.json",row.nlink);}catch(error){refused=true;expect(String((error as Error).message)).toContain(protocol.singleLink.message+": a/package.json");}
  expect(refused,row.id).toBe(row.expect);expect(refused,row.id).toBe(expected.singleLink[row.id]);
 }
});

/** 🔗️ Reproduces the second name of a tracked manifest with a real hardlink and refuses it through the same invariant the synchronization applies. */
test("incremental single-link invariant sees a real second name",()=>{
 const root=scratch();try{
  const manifest=join(root,"package.json"),alias=join(root,"alias.json");writeFileSync(manifest,"{}");
  expect(Number(oracle(["nlink",manifest],undefined,true))).toBe(1);core.requireSingleLinkInputV1(manifest,lstatSync(manifest).nlink);
  linkSync(manifest,alias);expect(Number(oracle(["nlink",manifest],undefined,true))).toBe(2);
  expect(()=>core.requireSingleLinkInputV1(manifest,lstatSync(manifest).nlink)).toThrow(protocol.singleLink.message);
  rmSync(alias);core.requireSingleLinkInputV1(manifest,lstatSync(manifest).nlink);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧪️ Proves with Bun itself that no install backend gives a workspace member manifest a second name, while a directory dependency does so regardless of the backend. */
test("bun install never links workspace member manifests; only file directory dependencies gain a second name",()=>{
 for(const backend of [...new Set([...fixture.bun.probeBackends,recipeBackends()[process.platform]!])]){
  const root=scratch(),cache=join(root,"cache"),workspace=join(root,"ws"),leaf=join(root,"pk");
  try{
   for(const[path,text]of Object.entries(fixture.bun.leaf)){mkdirSync(dirname(join(leaf,path)),{recursive:true});writeFileSync(join(leaf,path),text);}
   expect(spawnSync("tar",["czf",join(root,"leaf.tgz"),"-C",leaf,"package"]).status).toBe(0);
   for(const[path,text]of Object.entries(fixture.bun.workspace)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);}
   const install=()=>spawnSync(process.execPath,["install","--linker=isolated","--backend="+backend,"--cache-dir",cache,"--ignore-scripts"],{cwd:workspace,encoding:"utf8",env:{...process.env,HOME:root}});
   const first=install();expect(first.status,first.stderr).toBe(0);
   for(const path of Object.keys(fixture.bun.workspace))expect(lstatSync(join(workspace,path)).nlink,backend+" "+path).toBe(1);
   const controlRoot=join(root,fixture.bun.control.directory);
   for(const[path,text]of Object.entries(fixture.bun.control.files)){mkdirSync(controlRoot,{recursive:true});writeFileSync(join(controlRoot,path),text);}
   const dependent=join(workspace,fixture.bun.control.dependent);writeFileSync(dependent,JSON.stringify({name:"c",version:"1.0.0",dependencies:{localpkg:"file:../../"+fixture.bun.control.directory}}));
   rmSync(join(workspace,"node_modules"),{recursive:true,force:true});rmSync(join(workspace,"bun.lock"),{force:true});
   const second=install();expect(second.status,second.stderr).toBe(0);
   expect(lstatSync(join(controlRoot,"package.json")).nlink,backend+" control").toBe(2);
   expect(lstatSync(dependent).nlink,backend+" dependent").toBe(1);
   expect(()=>core.requireSingleLinkInputV1(join(controlRoot,"package.json"),lstatSync(join(controlRoot,"package.json")).nlink)).toThrow(protocol.singleLink.message);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
},180000);

/** 🚫️ Pins the installer recipe: every platform installs by clone or copy, never by hardlink, and the recipe equals the fixture table and the platform the arguments resolve on. */
test("incremental recipe law: the producer installs by clone or copy on every platform and never by hardlink",()=>{
 expect(recipeBackends()).toEqual(fixture.recipeBackends);
 expect(Object.keys(recipeBackends()).sort()).toEqual(["darwin","linux","win32"]);
 for(const[platform,backend]of Object.entries(recipeBackends())){expect(["clonefile","copyfile"],platform).toContain(backend);expect(Object.keys(fixture.recipeBackends),platform).toContain(platform);}
 const source=readFileSync(new URL("../../💾️storage/🟦️.ts",import.meta.url),"utf8");
 expect(source).not.toContain("--backend=hardlink");expect(source).not.toContain("recipe.backend)");
 expect(source).toContain('backend==="hardlink")throw Error("Supported non-linking Bun install backend required")');
});

/** 🔗️ Walks every tracked package and project manifest of the checkout and requires a single name each, exactly like the Source revision reader, against the independent Python walk. */
test("incremental tracked manifests carry exactly one name",()=>{
 const root=spawnSync("git",["-C",dirname(fileURLToPath(import.meta.url)),"rev-parse","--show-toplevel"],{encoding:"utf8"});
 if(root.status!==0)return;
 const top=root.stdout.trim(),listing=spawnSync("git",["-C",top,"ls-files","-z","--",...fixture.manifestNames.map(name=>":(glob)**/"+name)],{encoding:"buffer",maxBuffer:1<<28});
 expect(listing.status).toBe(0);
 const offenders:[string,number][]=[];let count=0;
 for(const path of listing.stdout.toString("utf8").split("\0")){if(!path)continue;let facts;try{facts=lstatSync(join(top,path));}catch{continue;}count++;if(facts.nlink!==1)offenders.push([path,facts.nlink]);}
 const independent=oracle(["manifests",top,...fixture.manifestNames]);
 expect({count,offenders:offenders.sort()}).toEqual(independent);
 expect(offenders,"tracked manifests with a second name").toEqual([]);
});

/** 🗂️ Persists and reuses ownership listings across processes: the exported records equal a Python projection, a seeded process trusts the settled listing under its exact stamp, and a changed directory is listed again. */
test("incremental ownership index: export equals the Python projection and a seeded listing is reused until its directory changes",async()=>{
 const root=scratch(),facade=fileURLToPath(new URL("../../../../../🗂️workspaces/🟦️bun/🟦️.ts",import.meta.url));
 try{
  for(const[path,text]of Object.entries(fixture.ownership.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  await sleep(fixture.ownership.settleMilliseconds);
  const script=join(root,"..",".inc-child-"+process.pid+".ts");
  writeFileSync(script,`import {withCurrentBunWorkspaceOwnershipV1,exportBunWorkspaceOwnershipIndexV1,seedBunWorkspaceOwnershipIndexV1} from ${JSON.stringify(facade)};import {readFileSync} from "node:fs";
const [mode,root,seedFile]=process.argv.slice(2);
if(mode!=="cold")seedBunWorkspaceOwnershipIndexV1(root,JSON.parse(readFileSync(seedFile,"utf8")));
let scopes:unknown;await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>{scopes=custody.scopes;});console.log(JSON.stringify({membership:{packages:(scopes as {packages:string[]}[]).flatMap(scope=>scope.packages)},index:mode==="cold"?exportBunWorkspaceOwnershipIndexV1(root):[]}));`);
  const run=(mode:string,seed="")=>{const child=spawnSync(process.execPath,[script,mode,root,seed],{encoding:"utf8",maxBuffer:1<<28});if(child.status!==0)throw Error(child.stderr);return JSON.parse(child.stdout);};
  const cold=run("cold");expect(cold.index.length).toBeGreaterThan(0);expect(cold.membership.packages).toContain("pkg/b");
  const independent=oracle(["ownership",root]);
  for(const record of cold.index){const key=record.path===root?".":record.path.slice(root.length+1);expect(independent[key],key).toBeDefined();expect(record.stamp,key).toBe(independent[key].stamp);expect(record.entries.map((entry:{name:string;kind:string})=>[entry.name,entry.kind]),key).toEqual(independent[key].entries);}
  const encoded=core.encodeOwnershipIndexV1(root,cold.index),decoded=core.decodeOwnershipIndexV1(root,encoded.bytes,encoded.evidence);
  expect(decoded.map(record=>[record.path,record.stamp,record.count,record.entries])).toEqual([...cold.index].sort((left:any,right:any)=>Buffer.compare(Buffer.from(left.path.slice(root.length)),Buffer.from(right.path.slice(root.length)))).map((record:any)=>[record.path,record.stamp,record.count,record.entries]));
  expect(oracle(["sha",script],undefined,true).length).toBe(64);
  const seedFile=join(root,"..",".inc-seed-"+process.pid+".json"),tamperedFile=seedFile+".tampered";writeFileSync(seedFile,JSON.stringify(decoded));
  const seeded=run("seeded",seedFile);expect(seeded.membership).toEqual(cold.membership);
  const tampered=decoded.map(record=>record.path===join(root,"pkg")?{...record,entries:record.entries.filter(entry=>entry.name!=="b")}:record);expect(tampered.some((record,index)=>record!==decoded[index])).toBe(true);
  writeFileSync(tamperedFile,JSON.stringify(tampered));
  expect(()=>run("seeded",tamperedFile)).toThrow("published workspace patterns drift");
  mkdirSync(join(root,"pkg","c"));writeFileSync(join(root,"pkg","c","package.json"),JSON.stringify({name:"c"}));writeFileSync(join(root,"package.json"),JSON.stringify({...JSON.parse(fixture.ownership.files["package.json"]),workspaces:["pkg/a","pkg/b","pkg/c"]}));
  const changed=run("seeded",tamperedFile);expect(changed.membership.packages).toContain("pkg/b");expect(changed.membership.packages).toContain("pkg/c");expect(changed.membership.packages.length).toBe(cold.membership.packages.length+1);
  rmSync(tamperedFile);
  rmSync(script);rmSync(seedFile);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

/** 📝️ Writes, verifies, supersedes and retires ledgers and the synchronization receipt through the real bounded relative-create writer. */
test("incremental receipts survive the Store writer, supersede cleanly and retire with their generation",async()=>{
 const base=scratch(),store=join(base,"store"),generationId="1234567890abcdef1234".padEnd(64,"0"),root=join(store,"j",generationId.slice(0,20));
 try{
  mkdirSync(root,{recursive:true});build(root,fixture.installed.tree);
  const rows=collect(root,Date.now()+fixture.installed.futureOffsetMs),counts=core.installedCountsV1(rows),scan={...counts,maximumPathUnits:20,sha256:"c".repeat(64)},yielded=async()=>{};
  const sha=await glue.sealJavascriptDependencyInstalledReceiptV1(store,generationId,scan,rows,()=>{},yielded);
  expect(readdirSync(join(store,"generation")).filter(name=>name.endsWith(protocol.installed.receiptSuffix)).length).toBe(1);
  expect(glue.newestJavascriptDependencyInstalledReceiptV1(store,generationId)).toBe(sha);
  const proof=await glue.verifyJavascriptDependencyInstalledReceiptV1(store,root,generationId,idle);expect(proof?.sha256).toBe(sha);expect(proof?.shape.entries).toBe(counts.entries);
  expect(await glue.sealJavascriptDependencyInstalledReceiptV1(store,generationId,scan,rows,()=>{},yielded)).toBe(sha);
  writeFileSync(join(root,"a","b","added.js"),"x");
  expect(await glue.verifyJavascriptDependencyInstalledReceiptV1(store,root,generationId,idle)).toBeUndefined();
  await expect(glue.sealJavascriptDependencyInstalledReceiptV1(store,generationId,{...scan,entries:scan.entries+1},rows,()=>{},yielded)).rejects.toThrow();
  const published=await glue.publishJavascriptDependencySyncFromReadsV1(store,[{scope:".",generation:generationId,sha256:"d".repeat(64)}],()=>{},yielded);
  expect(published).toBe("published");
  expect(readdirSync(join(store,"generation")).filter(name=>name.endsWith(protocol.sync.receiptSuffix)).length).toBe(1);
  expect(await glue.publishJavascriptDependencySyncFromReadsV1(store,[{scope:".",generation:"f".repeat(64),sha256:"d".repeat(64)}],()=>{},yielded)).toBe("incomplete");
  glue.retireJavascriptDependencyInstalledReceiptsV1(store,["f".repeat(64)]);
  expect(readdirSync(join(store,"generation")).filter(name=>name.includes(".installed."))).toEqual([]);
  const lock={version:1 as const,purpose:"dependency-lock-converged-receipt" as const,scope:"🌎️hub",generation:"a".repeat(64),lockSha256:"b".repeat(64),recipeSha256:"c".repeat(64),runtimeSha256:"d".repeat(64)};
  expect(glue.javascriptDependencyLockConvergedV1(store,lock)).toBe(false);
  await glue.sealJavascriptDependencyLockConvergedV1(store,lock,()=>{},yielded);
  expect(glue.javascriptDependencyLockConvergedV1(store,lock)).toBe(true);
  expect(glue.javascriptDependencyLockConvergedV1(store,{...lock,lockSha256:"e".repeat(64)})).toBe(false);
  await glue.sealJavascriptDependencyLockConvergedV1(store,{...lock,lockSha256:"e".repeat(64)},()=>{},yielded);
  expect(glue.javascriptDependencyLockConvergedV1(store,lock)).toBe(false);
  glue.retireJavascriptDependencyLockReceiptsV1(store,[]);
  expect(readdirSync(join(store,"generation")).filter(name=>name.endsWith(protocol.lock.receiptSuffix))).toEqual([]);
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

/** 🧱️ Executes the patched installed-paths scan itself with a ledger sink and requires its rows and shape to equal the independent Python walk. */
test("incremental scan ledger: the production scan emits the rows of the Python walk",async()=>{
 const{withBuildTestInvocationV1}=await import("../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"),ts=await import("typescript"),routing=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=await import("../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts"),descriptors=await import("@semio-tech/framework-filesystem/observation/descriptor");
 const path=new URL("../../💾️storage/🟦️.ts",import.meta.url),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path.pathname,source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="scanJavascriptDependencyInstalledPathsV1");
 expect(fn).toBeDefined();
 const code=ts.transpileModule(fn!.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,scan=new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")(installSchema,createHash,lstatSync,descriptors.withArtifactReadDirectoryV1,join,routing.requireIssuedResourceControlV1,leases.checkResourceLeaseControl)as (root:string,control:unknown,check:()=>void,sink?:unknown)=>Promise<{entries:number;directories:number;files:number;links:number}>;
 const root=scratch();
 try{
  build(root,fixture.installed.tree);
  await withBuildTestInvocationV1(async original=>{
   const control=routing.createIssuedResourceControlV1(original,"incremental:scan",0),start=Date.now()+fixture.installed.futureOffsetMs,sink=glue.installedLedgerSinkV1(start),result=await scan(root,control,()=>routing.requireIssuedResourceControlV1(control),sink),expected=oracle(["ledger",root,String(start)]);
   expect(sink.rows.map(row=>[row.path,...row.stamp,row.settled?1:0,row.children,row.directories,row.files,row.links]).sort((left,right)=>Buffer.compare(Buffer.from(left[0] as string),Buffer.from(right[0] as string)))).toEqual(expected.rows);
   expect({entries:result.entries,directories:result.directories,files:result.files,links:result.links}).toEqual({entries:expected.entries,directories:expected.directories,files:expected.files,links:expected.links});
   expect(core.installedCountsV1(sink.rows)).toEqual({entries:result.entries,directories:result.directories,files:result.files,links:result.links});
   const plain=await scan(root,control,()=>routing.requireIssuedResourceControlV1(control));expect(plain).toEqual(result);
  });
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

/** ⚖️ Pins the producer wiring by source law: the gate precedes the ownership walk, the retry loop only wraps it, installed paths go through the ledger, the lock skips converged scopes and the invariant sits in the input recheck. */
test("incremental wiring laws over the storage module",async()=>{
 const ts=await import("typescript"),path=new URL("../../💾️storage/🟦️.ts",import.meta.url),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path.pathname,source,ts.ScriptTarget.Latest,true);
 const text=(name:string)=>{const node=tree.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text===name);expect(node,name).toBeDefined();return node!.getText(tree);};
 const producer=text("runJavascriptDependencyStoreProducerV1"),lock=text("runJavascriptDependencyStoreLockV1");
 expect(producer.indexOf("verifyJavascriptDependencySyncCurrentV1(")).toBeGreaterThan(0);
 expect(producer.indexOf("verifyJavascriptDependencySyncCurrentV1(")).toBeLessThan(producer.indexOf("withPublishedBunWorkspaceOwnershipV1(workspace"));
 expect(producer.indexOf("attempt=()=>withPublishedBunWorkspaceOwnershipV1(")).toBeGreaterThan(0);
 expect(producer.indexOf("retryableSyncFailureV1(")).toBeGreaterThan(producer.indexOf("attempt=()=>"));
 expect(producer.indexOf("writeJavascriptDependencyOwnershipIndexV1(")).toBeGreaterThan(producer.indexOf("retryableSyncFailureV1("));
 expect(producer).toContain("retryState.unresolved=sourceTransitionMayBeUnresolved");
 expect(producer.split("requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint)").length-1).toBe(3);
 expect(producer).not.toContain("requireJavascriptDependencyInstalledPathsReadV1(");
 expect(producer).toContain("replayJavascriptDependencyExecutionV1(context,control,execution.observations,executionStartedAt)");
 expect(producer).toContain("starts.push(Date.now())");
 expect(producer.indexOf("starts.push(Date.now())")).toBeGreaterThan(producer.indexOf("settleWaitMilliseconds"));
 expect(producer).toContain("publishJavascriptDependencySyncFromReadsV1(");
 expect(lock).toContain("javascriptDependencyLockConvergedV1(");expect(lock).toContain("sealJavascriptDependencyLockConvergedV1(");
 expect(lock.indexOf("if(!selected.length)")).toBeLessThan(lock.indexOf("material=Object.freeze("));
 const recheck=text("recheckJavascriptDependencyInputsV1");expect(recheck).toContain("requireSingleLinkInputV1(source.path,facts.nlink)");
 const inspect=text("inspectJavascriptDependencyInputsReadV1");expect(inspect.split("recheckJavascriptDependencyInputsV1(sources,control)").length-1).toBeGreaterThanOrEqual(2);
 expect(text("scanJavascriptDependencyInstalledPathsV1")).toContain("ledger?.directory(portable,facts,");
 expect(text("readJavascriptDependencySourceOwnerScopeReadV1")).toContain("replayJavascriptDependencyExecutionV1(");
 expect(installFence()).toBeLessThanOrEqual(64);
 function installFence(){return(materializedInstallSchema.$defs.InstalledPathProtocol.const as {fullFenceInterval:number}).fullFenceInterval;}
});
