/** 🔒️ Contract: implementation files are facts of the process, not of the tree. A pinned process holds a fact about an implementation file it loaded exactly when the fact names the digest it loaded, never by a read of the file on disk: in a model world where the tooling file changes every 5 ms for the whole read, the facts hold at every round, the work is bounded, the read can seal; a fact of another digest or about a file that is not an implementation file is still judged on disk; a new process loads the new digest and judges by that. */
import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import * as observation from "../../🟨️.mjs";
import {withImplementationPinV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";

const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"loaded-implementation-")),sha=(text:string)=>createHash("sha256").update(text).digest("hex");
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const tool=join(base,"tooling.mjs"),other=join(base,"other.txt");
mkdirSync(base,{recursive:true});writeFileSync(tool,"export const v=0;\n");writeFileSync(other,"unrelated\n");
const loadedSha=sha("export const v=0;\n"),digestRow=(path:string,hash:string)=>({kind:"digest" as const,path,fact:{sha256:hash,size:statSync(path).size,mtimeMs:statSync(path).mtimeMs}});
const fileRow=(path:string,hash:string)=>{const status=statSync(path);return{kind:"file" as const,path,fact:{sha256:hash,size:status.size,mtimeMs:status.mtimeMs,ctimeMs:status.ctimeMs,ino:status.ino}};};
observation.registerLoadedImplementationRowsV1([digestRow(tool,loadedSha)]);

test("under churn of the tooling file every round of a pinned read holds, the work is bounded and nothing ends the read",async()=>{
 const rows=[digestRow(tool,loadedSha),fileRow(tool,loadedSha)];let rounds=0,reads=0;
 await withImplementationPinV1(async()=>{
  for(let round=1;round<=40;round++){writeFileSync(tool,`export const v=${round};\n`);rounds++;reads++;expect(await observation.observationsHold(rows),"round "+round).toBe(true);}
 });
 expect(rounds).toBe(40);expect(reads).toBe(40);
});

test("a deleted tooling file does not make a fact of the loaded digest stale",async()=>{
 const gone=join(base,"gone.mjs");writeFileSync(gone,"x\n");observation.registerLoadedImplementationRowsV1([digestRow(gone,sha("x\n"))]);const row=digestRow(gone,sha("x\n"));rmSync(gone);
 expect(await withImplementationPinV1(()=>observation.observationsHold([row]))).toBe(true);
});

test("a fact that names another digest than the loaded one, and a fact outside the loaded implementation, are still judged on disk",async()=>{
 expect(await withImplementationPinV1(()=>observation.observationsHold([digestRow(tool,sha("recorded by other code"))]))).toBe(false);
 const stale=fileRow(other,sha("unrelated\n"));writeFileSync(other,"changed after the fact\n");
 const seen:string[]=[];expect(await withImplementationPinV1(()=>observation.observationsHold([stale],64,new Set(),new Set(),undefined,row=>seen.push(row.path)))).toBe(false);expect(seen).toEqual([other]);
});

test("outside a pin the loaded digest is not an answer: the tree decides, as before",async()=>{
 expect(await observation.observationsHold([digestRow(tool,loadedSha)])).toBe(false);
});

test("the first registration of a path wins and a later load never replaces it",async()=>{
 const path=join(base,"first.mjs");writeFileSync(path,"a\n");observation.registerLoadedImplementationRowsV1([digestRow(path,sha("a\n"))]);observation.registerLoadedImplementationRowsV1([digestRow(path,sha("b\n"))]);
 expect(await withImplementationPinV1(()=>observation.observationsHold([digestRow(path,sha("a\n"))]))).toBe(true);
 expect(await withImplementationPinV1(()=>observation.observationsHold([digestRow(path,sha("b\n"))]))).toBe(false);
});

test("the next process loads the new code and judges by the new digest",()=>{
 const script=join(base,"child.mjs"),module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),revision=fileURLToPath(new URL("../../../📥️inputs/🔏️revision/🟨️.mjs",import.meta.url)),current=sha("export const v=40;\n");
 writeFileSync(script,`import{createHash}from"node:crypto";import{readFileSync,statSync}from"node:fs";import*as observation from${JSON.stringify(module)};import{withImplementationPinV1}from${JSON.stringify(revision)};
const path=${JSON.stringify(tool)},hash=createHash("sha256").update(readFileSync(path)).digest("hex"),row=value=>({kind:"digest",path,fact:{sha256:value,size:statSync(path).size,mtimeMs:statSync(path).mtimeMs}});
observation.registerLoadedImplementationRowsV1([row(hash)]);
const fresh=await withImplementationPinV1(()=>observation.observationsHold([row(hash)])),old=await withImplementationPinV1(()=>observation.observationsHold([row(${JSON.stringify(loadedSha)})]));
console.log(JSON.stringify({hash,fresh,old}));`);
 const run=Bun.spawnSync([process.execPath,script],{cwd:base,env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});expect(run.exitCode,run.stderr.toString()).toBe(0);
 expect(JSON.parse(run.stdout.toString().trim().split("\n").at(-1)!)).toEqual({hash:current,fresh:true,old:false});
},60000);

test("a process that finishes on its loaded code records the loaded digest of a file edited after load, and the next process records the new one",()=>{
 const path=join(base,"finish.mjs"),module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url));writeFileSync(path,"export const a=1;\n");
 const script=join(base,"finish-child.mjs");
 writeFileSync(script,`import{createHash}from"node:crypto";import{readFileSync,writeFileSync,statSync}from"node:fs";import*as observation from${JSON.stringify(module)};
const path=${JSON.stringify(path)},hash=()=>createHash("sha256").update(readFileSync(path)).digest("hex"),loaded=hash(),status=()=>statSync(path),fact=()=>({size:status().size,mtimeMs:status().mtimeMs,ctimeMs:status().ctimeMs,ino:status().ino,sha256:hash()});
observation.registerLoadedImplementationRowsV1([{kind:"digest",path,fact:{sha256:loaded}}]);
const before=observation.loadedImplementationRecordedFactV1("file",path,fact()).sha256;
writeFileSync(path,"export const a=2;"+String.fromCharCode(10));
const untouched=observation.loadedImplementationRecordedFactV1("file",path,fact()).sha256;
observation.finishOnLoadedImplementationV1();
const finished=observation.loadedImplementationRecordedFactV1("file",path,fact()).sha256,other=observation.loadedImplementationRecordedFactV1("file",path+".x",{sha256:"f".repeat(64)}).sha256,error=observation.loadedImplementationRecordedFactV1("file",path,{error:"ENOENT",sha256:"e".repeat(64)}).sha256;
console.log(JSON.stringify({loaded,before,untouched,finished,current:hash(),other,error}));`);
 const run=Bun.spawnSync([process.execPath,script],{cwd:base,env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});expect(run.exitCode,run.stderr.toString()).toBe(0);
 const row=JSON.parse(run.stdout.toString().trim().split("\n").at(-1)!);
 expect(row.loaded).toBe(sha("export const a=1;\n"));expect(row.current).toBe(sha("export const a=2;\n"));
 expect(row.before).toBe(row.loaded);expect(row.untouched).toBe(row.current);expect(row.finished).toBe(row.loaded);expect(row.other).toBe("f".repeat(64));expect(row.error).toBe("e".repeat(64));
 const next=Bun.spawnSync([process.execPath,script],{cwd:base,env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});expect(next.exitCode,next.stderr.toString()).toBe(0);
 const second=JSON.parse(next.stdout.toString().trim().split("\n").at(-1)!);expect(second.loaded).toBe(sha("export const a=2;\n"));
},60000);

test("a data file read while the modules loaded (owner registry, taxonomy) is part of the pinned snapshot: an edit during the read records the loaded fact, so nothing contradicts the sealed authority and the next process records the new one",()=>{
 const path=join(base,"registry-data.json"),unread=join(base,"unread-data.json"),module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url));writeFileSync(path,'{"owners":1}\n');writeFileSync(unread,'{"a":1}\n');
 const script=join(base,"data-child.mjs");
 writeFileSync(script,`import{createHash}from"node:crypto";import{readFileSync,writeFileSync,statSync}from"node:fs";import*as observation from${JSON.stringify(module)};
const path=${JSON.stringify(path)},unread=${JSON.stringify(unread)},hash=file=>createHash("sha256").update(readFileSync(file)).digest("hex"),fact=file=>({size:statSync(file).size,mtimeMs:statSync(file).mtimeMs,ctimeMs:statSync(file).ctimeMs,ino:statSync(file).ino,sha256:hash(file)});
const loaded=hash(path);observation.observedFs.readFileSync(path);observation.inferenceAuthorityDigestV1();
writeFileSync(path,'{"owners":2}'+String.fromCharCode(10));writeFileSync(unread,'{"a":2}'+String.fromCharCode(10));
const untouched=observation.loadedImplementationRecordedFactV1("file",path,fact(path)).sha256;
observation.finishOnLoadedImplementationV1();
const finished=observation.loadedImplementationRecordedFactV1("file",path,fact(path)).sha256,notLoaded=observation.loadedImplementationRecordedFactV1("file",unread,fact(unread)).sha256;
console.log(JSON.stringify({loaded,untouched,finished,current:hash(path),notLoaded,unreadCurrent:hash(unread)}));`);
 const run=Bun.spawnSync([process.execPath,script],{cwd:base,env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});expect(run.exitCode,run.stderr.toString()).toBe(0);
 const row=JSON.parse(run.stdout.toString().trim().split("\n").at(-1)!);
 expect(row.untouched).toBe(row.current);expect(row.finished).toBe(row.loaded);expect(row.loaded).not.toBe(row.current);expect(row.notLoaded).toBe(row.unreadCurrent);
 const next=Bun.spawnSync([process.execPath,script],{cwd:base,env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});expect(next.exitCode,next.stderr.toString()).toBe(0);
 expect(JSON.parse(next.stdout.toString().trim().split("\n").at(-1)!).loaded).toBe(sha('{"owners":2}\n'));
},60000);

test("a worker that finishes on its loaded code is not restarted by a peer's edit of a loaded file: the load in progress completes under its digest and the next load judges by the new one",()=>{
 const path=join(base,"inflight.mjs"),module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),revision=fileURLToPath(new URL("../../../📥️inputs/🔏️revision/🟨️.mjs",import.meta.url));writeFileSync(path,"export const a=1;\n");
 const script=join(base,"inflight-child.mjs");
 writeFileSync(script,`import{createHash}from"node:crypto";import{readFileSync,writeFileSync,statSync}from"node:fs";import*as observation from${JSON.stringify(module)};import{withImplementationPinV1}from${JSON.stringify(revision)};
const path=${JSON.stringify(path)},hash=()=>createHash("sha256").update(readFileSync(path)).digest("hex"),row=()=>({kind:"file",path,fact:{size:statSync(path).size,mtimeMs:statSync(path).mtimeMs,ctimeMs:statSync(path).ctimeMs,ino:statSync(path).ino,sha256:hash()}});
const loaded=hash();observation.registerLoadedImplementationRowsV1([{kind:"digest",path,fact:{sha256:loaded}}]);
const attempt=()=>{try{observation.requirePinnedImplementationFactsV1([row()]);return"ok";}catch(error){return String(error.code??error.message).slice(0,40);}};
const result={loaded};
await withImplementationPinV1(()=>{
 result.beforeEdit=attempt();
 writeFileSync(path,process.argv[2]+String.fromCharCode(10));
 result.current=hash();
 result.pinnedEdited=attempt();
 observation.finishOnLoadedImplementationV1();
 result.finishingEdited=attempt();
});
console.log(JSON.stringify(result));`);
 const run=(text:string)=>{const out=Bun.spawnSync([process.execPath,script,text],{cwd:base,env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});expect(out.exitCode,out.stderr.toString()).toBe(0);return JSON.parse(out.stdout.toString().trim().split("\n").at(-1)!);};
 const first=run("export const a=2;");
 expect(first.beforeEdit).toBe("ok");expect(first.pinnedEdited).toBe("BUILD_AUTHORITY_CHANGED");expect(first.finishingEdited).toBe("ok");expect(first.current).not.toBe(first.loaded);
 const second=run("export const a=3;");
 expect(second.loaded).toBe(first.current);expect(second.beforeEdit).toBe("ok");expect(second.finishingEdited).toBe("ok");
},60000);
