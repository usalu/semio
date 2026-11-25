import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtemp,readFile,rm,utimes,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import cases from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {forgetPersistedPathsV1,readForgottenPathsV1,rowsForgottenV1} from "../🟦️.ts";
import {openOwnerCheckpointV1} from "../../💾️owner-checkpoint/🟦️.ts";
import {observeOwnerFactsMemoizedV1} from "../../🟦️.ts";

const fixture=async(body:(directory:string)=>Promise<void>)=>{
 const directory=await mkdtemp(join(tmpdir(),"forgotten-paths-"));
 try{await body(directory);}finally{await rm(directory,{recursive:true,force:true});}
};
const written=async(directory:string)=>{
 const file=join(directory,"forgotten.json");
 await writeFile(file,JSON.stringify({version:1,paths:cases.forgotten}));
 return file;
};

test("a stored row naming a forgotten path or its listing is refused exactly when it was stored before the proof",()=>fixture(async directory=>{
 const forgotten=readForgottenPathsV1(await written(directory),cases.now);
 for(const row of cases.rows)expect([row.name,rowsForgottenV1(forgotten,row.rows,row.storedAt)]).toEqual([row.name,row.refused]);
}));

test("the refusal agrees with an independent Python implementation",()=>fixture(async directory=>{
 const forgotten=readForgottenPathsV1(await written(directory),cases.now);
 const script=[
  "import json,sys",
  "d=json.load(sys.stdin);f={e['path']:e['at'] for e in d['forgotten']};l={}",
  "for p,a in f.items():",
  " q=p[:max(p.rfind('/'),p.rfind(chr(92)))];l[q]=max(a,l.get(q,0))",
  "out=[]",
  "for c in d['rows']:",
  " s=int(c['storedAt']);r=False",
  " for w in c['rows']:",
  "  p=w.get('path')",
  "  if not isinstance(p,str):continue",
  "  if p in f and s<=f[p]:r=True",
  "  if w.get('kind') in ('readdir','readdirNames') and p in l and s<=l[p]:r=True",
  " out.append(r)",
  "print(json.dumps(out))",
 ].join("\n");
 const run=spawnSync("python3",["-I","-c",script],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(run.status).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(cases.rows.map(row=>rowsForgottenV1(forgotten,row.rows,row.storedAt)));
}));

test("forgetting merges, bounds and expires entries, and a corrupt or foreign file forgets nothing",()=>fixture(async directory=>{
 const file=join(directory,"forgotten.json"),limits=schema.$defs.Limits.const;
 expect(readForgottenPathsV1(file).paths.size).toBe(0);
 expect(await forgetPersistedPathsV1(file,["/w/a","/w/b"],1000)).toBe(true);
 expect(await forgetPersistedPathsV1(file,["/w/b","/w/c"],2000)).toBe(true);
 expect([...readForgottenPathsV1(file,3000).paths].sort()).toEqual([["/w/a",1000],["/w/b",2000],["/w/c",2000]]);
 expect(readForgottenPathsV1(file,1000+limits.maximumAgeMilliseconds).paths.has("/w/a")).toBe(false);
 expect(readForgottenPathsV1(file,1000+limits.maximumAgeMilliseconds).paths.has("/w/b")).toBe(true);
 const many=Array.from({length:limits.maximumPaths+10},(_,at)=>"/w/many/"+at);
 await forgetPersistedPathsV1(file,many,5000);
 const bounded=JSON.parse(await readFile(file,"utf8"));
 expect(bounded.paths.length).toBe(limits.maximumPaths);
 expect(bounded.paths.every((entry:{at:number})=>entry.at===5000)).toBe(true);
 await writeFile(file,"{not json");
 expect(readForgottenPathsV1(file,5000).paths.size).toBe(0);
 await writeFile(file,JSON.stringify({version:2,paths:[]}));
 expect(readForgottenPathsV1(file,5000).paths.size).toBe(0);
 await writeFile(file,JSON.stringify({version:1,paths:[{path:"/w/x",at:-1}]}));
 expect(readForgottenPathsV1(file,5000).paths.size).toBe(0);
 expect(await forgetPersistedPathsV1(join(directory,"no","such","\0"),["/w/a"],1)).toBe(false);
}));

const scope="a".repeat(64),format="b".repeat(64),suffix="owner\0"+JSON.stringify([["dev"],[]]),key=scope+"\0"+format+"\0"+suffix;
const globalPath=JSON.stringify(["closure-graph","/w",{roots:["{workspaceRoot}/a.rs"]}]);
const stored={facts:{project:{name:"owner",targets:{}},files:["old"],dependencies:[]},rows:[{kind:"exists",path:"/w/x",fact:true},{kind:"global",path:globalPath,fact:"{\"digest\":\"old\"}"}]};
const fresh={facts:{project:{name:"owner",targets:{}},files:["new"],dependencies:[]},rows:[{kind:"exists",path:"/w/x",fact:true},{kind:"global",path:globalPath,fact:"{\"digest\":\"new\"}"}]};
const port=(forgotten?:string)=>({
 check:()=>undefined,
 yieldContinuation:async()=>undefined,
 publish:(_paths:readonly string[],_plan:unknown,write:()=>void)=>{write();return true;},
 ...forgotten===undefined?{}:{refused:(rows:readonly unknown[],storedAt:number)=>rowsForgottenV1(readForgottenPathsV1(forgotten),rows,storedAt)},
});
let workers=0;
const worker=async(directory:string,forgotten:string|undefined,facts:{facts:unknown;rows:readonly unknown[]})=>{
 const checkpoint=await openOwnerCheckpointV1(directory,scope,format,port(forgotten)),discovered:string[]=[];
 const answer=await observeOwnerFactsMemoizedV1({
  key:key+"\0"+(workers++),recover:async()=>checkpoint.read(suffix),persist:entry=>checkpoint.save(suffix,entry),
  hold:async()=>true,
  observation:{captureObservedSourceInputsV1:async observe=>({value:await observe(),observations:facts.rows as never}),retainObservedRowsV1:()=>undefined},
  observe:async()=>{discovered.push("discovery");return facts.facts;},
 });
 checkpoint.flush();
 return{answer,discovered};
};

test("a stale row the in-round judge keeps holding is served by every fresh worker from the checkpoint until its path is forgotten",()=>fixture(async directory=>{
 const seed=await openOwnerCheckpointV1(directory,scope,format,port());
 seed.save(suffix,stored);seed.flush();
 const first=await worker(directory,undefined,fresh),second=await worker(directory,undefined,fresh),third=await worker(directory,undefined,fresh);
 expect([first,second,third].map(run=>(run.answer as typeof stored.facts).files)).toEqual([["old"],["old"],["old"]]);
 expect([first,second,third].map(run=>run.discovered.length)).toEqual([0,0,0]);
}));

test("after the seal forgot the stale path no later worker adopts the old row, and the row it observed afterwards is kept",()=>fixture(async directory=>{
 const file=join(directory,"forgotten.json"),seed=await openOwnerCheckpointV1(directory,scope,format,port());
 seed.save(suffix,stored);seed.flush();
 const old=new Date(Date.now()-60000);
 for(const name of await (await import("node:fs/promises")).readdir(directory))if(name.endsWith(".pack"))await utimes(join(directory,name),old,old);
 expect(await forgetPersistedPathsV1(file,[globalPath],Date.now()-1000)).toBe(true);
 const first=await worker(directory,file,fresh),second=await worker(directory,file,fresh),third=await worker(directory,file,fresh);
 expect([first,second,third].map(run=>(run.answer as typeof stored.facts).files)).toEqual([["new"],["new"],["new"]]);
 expect(first.discovered).toEqual(["discovery"]);
 expect(second.discovered).toEqual([]);
 expect(third.discovered).toEqual([]);
}));
