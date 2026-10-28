/** 📸️ Contract: a tree is observed as ONE state: every file is consumed once, a second listing must agree to the byte of every stamp, an entry edited within the settle window is waited out, and a difference is a concurrent edit that runs the whole observation again as the churn policy declares; the values equal an independent Python hash of the tree. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,truncateSync,writeFileSync,appendFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../../🧬️schema/📸️snapshot.json";
import {ConcurrentEditErrorV1,churnProtocolV1} from "../../🟦️.ts";
import {observeStableTreeV1,stableTreeProtocolV1,type StableTreeEventV1} from "../../📸️snapshot/🟦️.ts";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const python=testPythonV1(),protocol=stableTreeProtocolV1(),churn=churnProtocolV1();
let sandbox="",saved:string|undefined;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"snapshot-")));writeFileSync(join(sandbox,"AGENTS.md"),"x");writeFileSync(join(sandbox,"package.json"),"{}");saved=process.env.SEMIO_WORKSPACE_ROOT;process.env.SEMIO_WORKSPACE_ROOT=sandbox;});
afterAll(()=>{if(saved===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=saved;rmSync(sandbox,{recursive:true,force:true});});
const made=(name:string,files:Record<string,string>):string=>{const root=join(sandbox,name);for(const[path,body]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),body);}return root;};
const reader=async(path:string):Promise<string>=>readFileSync(path,"utf8");
const options={sleep:async()=>{},sleepSync:()=>{},environment:{SEMIO_LANGUAGE:"en"},onRetry:()=>{},onSettle:()=>{}};
const aged={...options,now:()=>Date.now()+60_000,settleMilliseconds:300};

test("the protocol is closed under its schema, the settle texts exist in every language with the same placeholders",()=>{
 const ajv=new Ajv({strict:false}),text=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Text"});
 expect(Object.keys(protocol.texts).sort()).toEqual([...churn.languages].sort());
 const placeholders=(language:string)=>[...protocol.texts[language]!.settle.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const language of churn.languages){expect(text(protocol.texts[language]!.settle)).toBe(true);expect(placeholders(language)).toEqual(["age","delay","path"]);}
 expect(churn.settleMilliseconds).toBeGreaterThan(0);expect(protocol.concurrency).toBeGreaterThan(0);
});

test("a quiet tree is consumed once per file and the values equal an independent Python hash of the same tree",async()=>{
 const root=made("quiet",{"a/x.txt":"one","a/y.txt":"two","b/z.txt":"three","b/c/w.txt":"four"});
 let calls=0;
 const seen=await observeStableTreeV1([root],async path=>{calls++;return reader(path);},aged);
 expect(calls).toBe(4);expect(seen.attempts).toBe(1);expect([...seen.files.keys()]).toEqual([...seen.files.keys()].sort());
 expect(seen.directories.map(path=>relative(root,path))).toEqual(["","a","b","b/c"]);
 const manifest=Object.fromEntries([...seen.files].map(([path,body])=>[relative(root,path),body]));
 const answer=JSON.parse(Bun.spawnSync([python!,"-I","-c","import json,os,sys\nroot=sys.argv[1]\nout={}\nfor base,_,names in os.walk(root):\n    for n in names:\n        p=os.path.join(base,n)\n        out[os.path.relpath(p,root)]=open(p).read()\nprint(json.dumps(out))",root]).stdout.toString()) as Record<string,string>;
 expect(manifest).toEqual(answer);
});

test("an edit of a consumed file during the observation is a concurrent edit: the whole observation runs again and returns the new state, never a mix",async()=>{
 const root=made("edit",{"a.txt":"A0","b.txt":"B0","c.txt":"C0"});
 let attempt=0;const events:string[]=[];
 const seen=await observeStableTreeV1([root],async path=>{
  const body=await reader(path);
  if(path.endsWith("c.txt")&&attempt===0){attempt++;writeFileSync(join(root,"a.txt"),"A1");}
  return body;
 },{...aged,onRetry:event=>events.push(event.text)});
 expect(seen.attempts).toBe(2);
 expect(Object.fromEntries([...seen.files].map(([path,body])=>[relative(root,path),body]))).toEqual({"a.txt":"A1","b.txt":"B0","c.txt":"C0"});
 expect(events.length).toBe(1);
});

test("a file that is created, deleted or renamed during the observation moves the roster and the observation runs again",async()=>{
 for(const[name,change,expected]of[["created",(root:string)=>writeFileSync(join(root,"n.txt"),"N"),{"a.txt":"A","b.txt":"B","n.txt":"N"}],["renamed",(root:string)=>renameSync(join(root,"a.txt"),join(root,"r.txt")),{"b.txt":"B","r.txt":"A"}],["deleted",(root:string)=>rmSync(join(root,"a.txt")),{"b.txt":"B"}]]as const){
  const root=made("roster-"+name,{"a.txt":"A","b.txt":"B"});let fired=false;
  const seen=await observeStableTreeV1([root],async path=>{const body=await reader(path).catch(()=>"gone");if(!fired&&path.endsWith("b.txt")){fired=true;change(root);}return body;},aged);
  expect(seen.attempts,name).toBe(2);
  expect(Object.fromEntries([...seen.files].map(([path,body])=>[relative(root,path),body])),name).toEqual(expected);
 }
});

test("a raw ENOENT of a file that a peer deleted between the listing and the read is classified by the churn policy and the observation runs again",async()=>{
 const root=made("raw",{"a.txt":"A","b.txt":"B"});let fired=false;
 const seen=await observeStableTreeV1([root],async path=>{if(!fired&&path.endsWith("b.txt")){fired=true;rmSync(join(root,"a.txt"));rmSync(path);}return readFileSync(path,"utf8");},aged);
 expect(seen.attempts).toBe(2);expect([...seen.files.values()]).toEqual([]);
});

test("a writer that truncated a file and paused halfway is waited out: the snapshot is only accepted after the settle window, and a third listing agrees",async()=>{
 const root=made("torn",{"a.txt":"complete content of a","b.txt":"complete content of b"});
 truncateSync(join(root,"a.txt"),0);
 const events:StableTreeEventV1[]=[];
 setTimeout(()=>writeFileSync(join(root,"a.txt"),"rewritten a"),120);
 const seen=await observeStableTreeV1([root],reader,{environment:{SEMIO_LANGUAGE:""},onRetry:()=>{},sleepSync:()=>{},sleep:async()=>{},onSettle:event=>events.push(event)});
 expect(events.length).toBeGreaterThan(0);expect(events[0]!.phase).toBe("settle");
 expect(events[0]!.text.split("\n").length).toBe(2);expect(events[0]!.text).toContain("ms ago");expect(events[0]!.text).toContain("vor ");
 expect(Object.fromEntries([...seen.files].map(([path,body])=>[relative(root,path),body]))).toEqual({"a.txt":"rewritten a","b.txt":"complete content of b"});
});

test("an unchanged settled file keeps its value across attempts: only what changed is consumed again",async()=>{
 const root=made("memo",{"a.txt":"A","b.txt":"B","c.txt":"C"});const calls:string[]=[];let fired=false;
 const seen=await observeStableTreeV1([root],async path=>{calls.push(relative(root,path));if(!fired&&path.endsWith("c.txt")){fired=true;writeFileSync(join(root,"b.txt"),"B1");}return reader(path);},aged);
 expect(seen.attempts).toBe(2);
 expect(calls.filter(name=>name==="a.txt").length).toBe(1);expect(calls.filter(name=>name==="c.txt").length).toBe(1);expect(calls.filter(name=>name==="b.txt").length).toBe(2);
});

test("an edit that never stops fails after the attempts of the snapshot layer, naming the path, in both languages; cancellation ends the pause",async()=>{
 const root=made("busy",{"a.txt":"A"});let calls=0;
 const failure=await observeStableTreeV1([root],async path=>{calls++;appendFileSync(path,"x");return readFileSync(path,"utf8");},{...aged,environment:{SEMIO_LANGUAGE:""}}).then(()=>undefined,error=>error as ConcurrentEditErrorV1);
 expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);expect(calls).toBe(churn.profiles.snapshot.maximumAttempts);
 expect(failure!.path).toBe(join(root,"a.txt"));expect(failure!.message).toContain("kept changing in all");expect(failure!.message).toContain("änderte sich in allen");
 const controller=new AbortController(),pending=observeStableTreeV1([root],async path=>{appendFileSync(path,"y");return reader(path);},{...aged,signal:controller.signal,sleep:(_,signal)=>new Promise<void>((_accept,reject)=>signal!.addEventListener("abort",()=>reject(signal!.reason),{once:true}))});
 setTimeout(()=>controller.abort(new Error("stop")),50);
 await expect(pending).rejects.toThrow("stop");
});
