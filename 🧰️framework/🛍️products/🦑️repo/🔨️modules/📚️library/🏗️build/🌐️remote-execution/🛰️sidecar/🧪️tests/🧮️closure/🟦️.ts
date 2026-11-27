import {afterAll,expect,test} from "bun:test";
import {appendFileSync,copyFileSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,sep} from "node:path";
import {createSidecarClosureWatchV1,sidecarClosureIgnoredV1,sidecarImportsV1} from "../../🧮️closure/🟦️.ts";
import {remoteExecutionSidecarEntriesV1,remoteExecutionSidecarWatchV1} from "../../🟦️.ts";

const scratch:string[]=[];
const directory=():string=>{const path=mkdtempSync(join(tmpdir(),"sidecar-closure-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(root:string,name:string,text:string)=>{const path=join(root,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);return path;};
const python=async(program:string,input:string):Promise<string[]>=>{const child=Bun.spawn(["python3","-I","-c",program,input],{stdout:"pipe",stderr:"pipe"}),[out,code]=await Promise.all([new Response(child.stdout).text(),child.exited]);expect(code).toBe(0);return out.trim().split("\n").filter(Boolean);};

test("the imports of a module: static, re-exported and literal dynamic ones, relative only, type-only imports erased; python's regex scan agrees on the relative specifiers of value imports",async()=>{
 const source='import a from "./a.ts";\nexport {b} from "../b.ts";\nconst x=await import("./c.ts");\nimport type {T} from "./t.ts";\nimport j from "./j.json" with {type:"json"};\nimport fs from "node:fs";\nimport "./side.ts";\n';
 const rows=sidecarImportsV1(source);
 expect(rows.map(row=>row.specifier).sort()).toEqual(["../b.ts","./a.ts","./c.ts","./j.json","./side.ts"]);
 expect(rows.find(row=>row.specifier==="./c.ts")?.dynamic).toBe(true);expect(rows.find(row=>row.specifier==="./a.ts")?.dynamic).toBe(false);
 const oracle=await python("import re,sys\nsrc=sys.argv[1]\nfound=set(re.findall(r'(?:from|import)\\s*\\(?\\s*[\"\\'](\\.{1,2}/[^\"\\']+)[\"\\']',src))\nfound.discard('./t.ts')\nprint('\\n'.join(sorted(found)))",source);
 expect(oracle).toEqual(rows.map(row=>row.specifier).sort());
});

test("a closure changes only when a byte of a file it runs changes: touching, rewriting identically, editing an unrelated file or a file only another role imports changes nothing",()=>{
 const root=directory();
 write(root,"main.ts",'import {a} from "./a.ts";\nexport const run=async(role:string)=>role==="x"?(await import("./x/role.ts")).x():(await import("./y/role.ts")).y();\n');
 write(root,"a.ts",'import data from "./data.json" with {type:"json"};\nexport const a=data;\n');write(root,"data.json",'{"v":1}');
 const x=write(root,"x/role.ts",'import {shared} from "../shared.ts";export const x=()=>shared;'),y=write(root,"y/role.ts",'export const y=()=>2;'),shared=write(root,"shared.ts",'export const shared=1;'),unrelated=write(root,"unrelated.ts","export {};");
 const watchX=createSidecarClosureWatchV1([join(root,"main.ts"),x],[join(root,"main.ts")],root),watchY=createSidecarClosureWatchV1([join(root,"main.ts"),y],[join(root,"main.ts")],root);
 expect([...watchX.closure().files.keys()].map(path=>relative(root,path)).sort()).toEqual(["a.ts","data.json","main.ts","shared.ts","x/role.ts"].map(name=>name.split("/").join(sep)));
 expect([...watchY.closure().files.keys()].map(path=>relative(root,path)).sort()).toEqual(["a.ts","data.json","main.ts","y/role.ts"].map(name=>name.split("/").join(sep)));
 const later=new Date(Date.now()+5000);
 utimesSync(join(root,"a.ts"),later,later);expect(watchX.changed(),"touched").toBe(false);
 writeFileSync(join(root,"a.ts"),'import data from "./data.json" with {type:"json"};\nexport const a=data;\n');expect(watchX.changed(),"rewritten with the same bytes").toBe(false);
 appendFileSync(unrelated,"// edit");expect(watchX.changed(),"a file nobody imports").toBe(false);
 appendFileSync(y,"// edit");expect(watchX.changed(),"the code of the other role").toBe(false);expect(watchY.changed()).toBe(true);
 appendFileSync(shared,"// edit");expect(watchX.changed(),"a file the role imports").toBe(true);
 const fresh=createSidecarClosureWatchV1([join(root,"main.ts"),x],[join(root,"main.ts")],root);appendFileSync(join(root,"data.json")," ");expect(fresh.changed(),"a json file the role imports").toBe(true);
 const gone=createSidecarClosureWatchV1([join(root,"main.ts"),x],[join(root,"main.ts")],root);rmSync(shared);expect(gone.changed(),"a file of the closure disappeared").toBe(true);
});

test("the digest of a closure is the same on every operating system: each file is named by its path below the root with / separators, as python computes it from the bytes",async()=>{
 const root=directory();
 write(root,"main.ts",'import {a} from "./lib/a.ts";export const m=a;');write(root,"lib/a.ts",'import data from "./deep/data.json" with {type:"json"};export const a=data;');write(root,"lib/deep/data.json",'{"v":1}');
 const closure=createSidecarClosureWatchV1([join(root,"main.ts")],[],root).closure();
 const oracle=await python("import hashlib,os,sys\nroot=sys.argv[1]\nrows=[]\nfor base,_,names in os.walk(root):\n  for name in names:\n    path=os.path.join(base,name)\n    rows.append('/'+os.path.relpath(path,root).replace(os.sep,'/')+' '+hashlib.sha256(open(path,'rb').read()).hexdigest())\nprint(hashlib.sha256('\\n'.join(sorted(rows)).encode()).hexdigest())",root);
 expect(closure.files.size).toBe(3);
 expect(closure.digest).toBe(oracle[0]!);
});

test("the real roles have different closures: the agent runs no scheduler code and the cache runs no worker code, and the closure of a role is a small set",()=>{
 const names=(role:"agent"|"scheduler"|"cache")=>new Set([...remoteExecutionSidecarWatchV1(role).closure().files.keys()].map(path=>path.split("/").slice(-2).join("/")));
 const agent=names("agent"),scheduler=names("scheduler"),cache=names("cache");
 expect(agent.has("🗓️scheduler/🟦️.ts")).toBe(false);expect(agent.has("👷️agent/🟦️.ts")).toBe(true);
 expect(scheduler.has("👷️agent/🟦️.ts")).toBe(false);expect(cache.has("👷️agent/🟦️.ts")).toBe(false);
 expect(cache.has("🖥️server/🟦️.ts")).toBe(true);expect(agent.has("🖥️server/🟦️.ts")).toBe(false);
 for(const set of[agent,scheduler,cache])expect(set.size).toBeGreaterThan(20);
});

test("a real sidecar restarts for a change in the code it runs and for nothing else: a touch and an edit of another role's module leave it alone, an edit of its own closure restarts the role inside the same supervised process, with one log line",async()=>{
 const live=remoteExecutionSidecarEntriesV1("agent"),files=[...remoteExecutionSidecarWatchV1("agent").closure().files.keys()];
 const other=join(dirname(live.entries[1]!),"..","🗓️scheduler","🟦️.ts");
 const protocol=[...createSidecarClosureWatchV1([join(dirname(live.entries[0]!),"..","..","..","..","🤝️shared-worktree","🔁️replica","🧷️sidecars","🟦️.ts")],[],"/").closure().files.keys()],all=[...files,other,...protocol];
 let ancestor=dirname(all[0]!);for(const path of all)while(!path.startsWith(ancestor+sep))ancestor=dirname(ancestor);
 const root=directory(),copy=(path:string)=>join(root,relative(ancestor,path));
 for(const path of all){mkdirSync(dirname(copy(path)),{recursive:true});copyFileSync(path,copy(path));}
 const worktree=directory(),entry=copy(live.entries[0]!);
 const child=Bun.spawn([process.execPath,entry,"agent"],{stdout:"ignore",stderr:"pipe",cwd:worktree,env:{...process.env,SEMIO_FLEET_ROOT:worktree,SEMIO_WORKSPACE_ROOT:worktree,SEMIO_SIDECAR_WATCH_MILLISECONDS:"250",SEMIO_SIDECAR_QUIET_MILLISECONDS:"500",SEMIO_SIDECAR_MINIMUM_INTERVAL_MILLISECONDS:"1",SEMIO_GRAPH_SERVER:"off",SEMIO_REMOTE_CACHE_ENDPOINT:"",SEMIO_REMOTE_EXECUTION_ENDPOINT:""}});
 let stderr="";void (async()=>{for await(const chunk of child.stderr as unknown as AsyncIterable<Uint8Array>)stderr+=new TextDecoder().decode(chunk);})();
 const alive=async(milliseconds:number)=>(await Promise.race([child.exited,Bun.sleep(milliseconds).then(()=>"alive" as const)]))==="alive";
 expect(await alive(2500),"started and waiting for the fleet facts").toBe(true);
 const later=new Date(Date.now()+5000);utimesSync(copy(files[1]!),later,later);
 expect(await alive(1500),"a touch").toBe(true);
 appendFileSync(copy(other),"\n// another role\n");
 expect(await alive(1500),"the scheduler module, which the agent does not run").toBe(true);
 const starts=()=>stderr.match(/waiting for the fleet facts/gu)?.length??0,before=starts();
 appendFileSync(copy(live.entries[1]!),"\n// the agent itself\n");
 const deadline=Date.now()+15000;while(Date.now()<deadline&&!(stderr.includes("agent code changed, restarting")&&starts()>before))await Bun.sleep(100);
 expect(stderr,"the agent's own module changed").toContain("agent code changed, restarting");
 expect(starts(),"the launcher started the agent again by itself, with the new code").toBeGreaterThan(before);
 expect(await alive(1500),"the process the daemon supervises stayed up through the restart").toBe(true);
 expect(stderr.match(/code changed/gu)?.length,"one restart for one change").toBe(1);
 child.kill("SIGTERM");expect(await child.exited,"stops in order when asked").toBe(0);
},60000);

test("a text catalogue below a texts folder is words only: editing it every few seconds never changes the closure of a role, editing the schema or code beside it does",()=>{
 const root=directory();
 write(root,"main.ts",'import texts from "./📖️texts/🔣️.json" with {type:"json"};import schema from "./🧬️schema/🔣️.json" with {type:"json"};export const run=[texts,schema];');
 const text=write(root,"📖️texts/🔣️.json",'{"a":1}'),schema=write(root,"🧬️schema/🔣️.json",'{"b":1}');
 const watch=createSidecarClosureWatchV1([join(root,"main.ts")],[],root);
 for(let edit=0;edit<5;edit++){appendFileSync(text," ");expect(watch.changed(),"texts edit "+edit).toBe(false);}
 expect([...watch.closure().files.keys()].some(path=>path.includes("📖️texts")),"still part of the copy a test or a launcher runs").toBe(true);expect(watch.changes()).toEqual([]);
 appendFileSync(schema," ");expect(watch.changed(),"a schema beside it").toBe(true);
 expect(sidecarClosureIgnoredV1("/x/y/📖️texts/🔣️.json")).toBe(true);expect(sidecarClosureIgnoredV1("/x/y/📖️textsX/a.json")).toBe(false);expect(sidecarClosureIgnoredV1("C:\\x\\📖️texts\\a.json")).toBe(true);
});

test("a watch with memory hashes only what moved: the digest equals the one of a watch without memory, a stamped record is believed, a changed file is read again",()=>{
 const root=directory();
 const main=write(root,"main.ts",'import {a} from "./a.ts";\nexport const run=a;\n'),other=write(root,"a.ts","export const a=1;\n");
 let stored:Record<string,any>|undefined,writes=0;
 const memory={read:()=>stored,write:(records:Record<string,any>)=>{stored=records;writes++;}};
 const plain=createSidecarClosureWatchV1([main],[],root).baseline,first=createSidecarClosureWatchV1([main],[],root,memory);
 expect(first.baseline).toBe(plain);expect(writes).toBe(1);expect(Object.keys(stored!).sort()).toEqual([main,other].sort());
 const second=createSidecarClosureWatchV1([main],[],root,memory);
 expect(second.baseline).toBe(plain);expect(writes,"nothing moved, nothing written").toBe(1);
 const poisoned={...stored!,[other]:{...stored![other],sha256:"0".repeat(64)}};stored=poisoned;
 expect(createSidecarClosureWatchV1([main],[],root,memory).baseline,"a record with the live stamp is not hashed again").not.toBe(plain);
 appendFileSync(other,"//x\n");
 const edited=createSidecarClosureWatchV1([main],[],root,memory).baseline;
 expect(edited).toBe(createSidecarClosureWatchV1([main],[],root).baseline);expect(edited).not.toBe(plain);
});
