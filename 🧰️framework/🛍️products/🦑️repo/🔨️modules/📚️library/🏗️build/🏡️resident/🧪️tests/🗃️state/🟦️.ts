import {afterAll,describe,expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,readdirSync,renameSync,rmSync,writeFileSync,appendFileSync} from "node:fs";
import {POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {createFilterV1,createOwnerIndexV1,createWorkspaceStateV1,outputSignatureV1} from "../../🗃️state/🟨️.mjs";
import {cleanup,pick,pythonDigest,rng,sha,snapshot,tempRoot,writeTree} from "../🟦️.ts";

const names=["o0","o1","o2","o3","o4","inner"];
const graph={
 owners:[
  {name:"o0",root:"pkgs/o0",deps:[],tasks:[{name:"build",command:["b","o0"],outputs:[".out/o0"]}]},
  {name:"o1",root:"pkgs/o1",deps:["o0"],tasks:[{name:"build",command:["b","o1"],dependsOn:["o0:build"],outputs:[".out/o1"]},{name:"test",command:["t","o1"],dependsOn:["o1:build"],outputs:[".out/o1t"]}]},
  {name:"o2",root:"pkgs/o2",deps:["o0","o1"],tasks:[{name:"build",command:["b","o2"],dependsOn:["o0:build","o1:build"],outputs:[".out/o2"]}]},
  {name:"o3",root:"pkgs/o3",deps:["o2","inner"],tasks:[{name:"build",command:["b","o3"],dependsOn:["o2:build","inner:build"],outputs:[".out/o3"]}]},
  {name:"o4",root:"pkgs/o4",deps:[],tasks:[{name:"build",command:["b","o4"],outputs:[".out/o4"]}]},
  {name:"inner",root:"pkgs/o2/inner",deps:[],tasks:[{name:"build",command:["b","inner"],outputs:[".out/inner"]}]},
 ],
 env:{paths:["package.json","tsconfig.base.json"],prefixes:[]},
};
const policy={...POLICY_V1,racyWindowMilliseconds:2000};
const filter=createFilterV1({rooted:[".git"],names:["node_modules"]});
const isOutput=(rel:string)=>/^\.out\/(o0|o1|o1t|o2|o3|o4|inner)(\/|$)/u.test(rel);

function setup(prefix:string,overrides:Record<string,unknown>={}){
 const root=tempRoot(prefix);
 writeTree(root,{"package.json":"{}","tsconfig.base.json":"{}",".git/HEAD":"ref","pkgs/o0/src/a.ts":"a","pkgs/o1/src/b.ts":"b","pkgs/o2/src/c.ts":"c","pkgs/o2/inner/i.ts":"i","pkgs/o3/src/d.ts":"d","pkgs/o4/src/e.ts":"e","pkgs/o1/node_modules/x/y.js":"y","README.md":"r",".out/.keep":"k"});
 const state=createWorkspaceStateV1({root,instance:"i1",policy:{...policy,...overrides},filter,fence:".semio-fence",now:()=>Date.now()});
 state.configure(graph);
 const excluded=(rel:string)=>filter.excluded(rel)||rel===".semio-fence"||rel.startsWith(".semio-fence/")||isOutput(rel);
 const listing=()=>{const s=snapshot(root,excluded);return{directories:[...s.directories].map(([p,[m,i]])=>[p,m,i] as [string,number,number]),members:[...s.files]};};
 state.load(listing());
 return{root,state,excluded,snap:()=>snapshot(root,excluded)};
}

function naiveDigest(root:string,files:Map<string,number>,owner:{root:string},ownerRoots:string[]){
 const mine=[...files.keys()].filter(path=>{
  if(!(path===owner.root||path.startsWith(owner.root+"/")))return false;
  return!ownerRoots.some(other=>other.length>owner.root.length&&other.startsWith(owner.root+"/")&&(path===other||path.startsWith(other+"/")));
 }).filter(path=>path!=="package.json"&&path!=="tsconfig.base.json").sort();
 return{files:mine,digest:sha(mine.map(path=>path+"\0"+sha(require("node:fs").readFileSync(root+"/"+path))+"\n").join(""))};
}

function fingerprints(root:string,files:Map<string,number>){
 const roots=graph.owners.map(o=>o.root),digests=new Map(graph.owners.map(o=>[o.name,naiveDigest(root,files,o,roots).digest]));
 const env=sha(require("node:fs").readFileSync(root+"/package.json")+""+require("node:fs").readFileSync(root+"/tsconfig.base.json"));
 const closure=(name:string)=>{const seen=new Set([name]),order=[name];for(let i=0;i<order.length;i++)for(const d of graph.owners.find(o=>o.name===order[i])!.deps)if(!seen.has(d)){seen.add(d);order.push(d);}return order.sort();};
 const out=new Map<string,string>();
 for(const owner of graph.owners)for(const task of owner.tasks)out.set(owner.name+":"+task.name,sha(env+"|"+closure(owner.name).map(n=>digests.get(n)).join(",")+"|"+task.command.join(" ")+"|"+(task.name==="test"?"t":"")));
 return out;
}

const roots:string[]=[];
afterAll(()=>roots.forEach(cleanup));

describe("owner index",()=>{
 test("longest prefix wins and nested owners carve out their subtree",()=>{
  const index=createOwnerIndexV1([["pkgs/a","a"],["pkgs/a/inner","inner"],["","root"]]);
  expect(index.ownerOf("pkgs/a/x.ts")).toBe("a");expect(index.ownerOf("pkgs/a/inner/x.ts")).toBe("inner");expect(index.ownerOf("pkgs/ab/x")).toBe("root");expect(index.ownerOf("other")).toBe("root");
  expect(index.ownersUnder("pkgs/a").sort()).toEqual(["a","inner"]);
  expect(createOwnerIndexV1([["pkgs/a","a"]]).ownerOf("pkgs")).toBeUndefined();
 });
 test("filter honours rooted prefixes with a path boundary and component names",()=>{
  const f=createFilterV1({rooted:[".git","a/b"],names:["node_modules"]});
  expect(f.excluded(".git")).toBe(true);expect(f.excluded(".git/x")).toBe(true);expect(f.excluded(".github/x")).toBe(false);expect(f.excluded("a/b/c")).toBe(true);expect(f.excluded("a/bc")).toBe(false);
  expect(f.excluded("x/node_modules/y")).toBe(true);expect(f.excluded("x/node_modules2")).toBe(false);expect(f.excluded("")).toBe(false);
 });
});

describe("state against naive oracles under random edits",()=>{
 for(const seed of[1,2,3,4])test(`seed ${seed}: membership, net changes, owner digests and task keys equal the oracles after every step`,async()=>{
  const ctx=setup("state-"+seed+"-");roots.push(ctx.root);
  const{root,state}=ctx,random=rng(seed*7919),start=ctx.snap(),cursor0=state.cursor();
  let counter=0;
  const keys0=new Map<string,string>();for(const id of state.tasks.keys())keys0.set(id,await state.taskKey(id));
  const ownerDirs=["pkgs/o0","pkgs/o1","pkgs/o2","pkgs/o3","pkgs/o4","pkgs/o2/inner"];
  const listFiles=()=>[...ctx.snap().files.keys()].filter(p=>p.startsWith("pkgs/")&&!p.includes("node_modules"));
  let previousFingerprint=fingerprints(root,ctx.snap().files),previousKeys=keys0;
  for(let step=0;step<70;step++){
   const rows:[string,number,string][]=[];
   const op=pick(random,["create","modify","delete","rename","mkdir","rmdir","env","excluded","output","chmod","noop","atomic"] as const);
   const dir=pick(random,ownerDirs),files=listFiles(),file=files.length?pick(random,files):undefined;
   if(op==="create"){const p=dir+"/src/n"+(counter++)+".ts";mkdirSync(root+"/"+dir+"/src",{recursive:true});writeFileSync(root+"/"+p,"new"+counter);rows.push(["a",0,p]);}
   else if(op==="modify"&&file){appendFileSync(root+"/"+file,"m"+step);rows.push(["m",0,file]);}
   else if(op==="delete"&&file){rmSync(root+"/"+file);rows.push(["r",0,file]);}
   else if(op==="rename"&&file){const target=pick(random,ownerDirs)+"/src/r"+(counter++)+".ts";mkdirSync(root+"/"+target.slice(0,target.lastIndexOf("/")),{recursive:true});renameSync(root+"/"+file,root+"/"+target);rows.push(["n",0,file],["n",0,target]);}
   else if(op==="mkdir"){const p=dir+"/gen"+(counter++);mkdirSync(root+"/"+p+"/deep",{recursive:true});writeFileSync(root+"/"+p+"/deep/f.ts","g");writeFileSync(root+"/"+p+"/h.ts","h");rows.push(["a",1,p]);}
   else if(op==="rmdir"){const dirs=[...ctx.snap().directories.keys()].filter(p=>/^pkgs\/o\d\/(src|gen\d+)$/u.test(p));if(dirs.length){const p=pick(random,dirs);rmSync(root+"/"+p,{recursive:true});rows.push(["r",1,p]);}}
   else if(op==="env"){appendFileSync(root+"/package.json"," ");rows.push(["m",0,"package.json"]);}
   else if(op==="excluded"){mkdirSync(root+"/pkgs/o1/node_modules/x",{recursive:true});writeFileSync(root+"/pkgs/o1/node_modules/x/z"+(counter++)+".js","z");rows.push(["a",0,"pkgs/o1/node_modules/x/z"+counter+".js"]);}
   else if(op==="output"){mkdirSync(root+"/.out/o1",{recursive:true});writeFileSync(root+"/.out/o1/out"+(counter++)+".txt","o");rows.push(["a",0,".out/o1/out"+counter+".txt"]);}
   else if(op==="chmod"&&file){chmodSync(root+"/"+file,0o755);rows.push(["m",0,file]);}
   else if(op==="atomic"&&file){writeFileSync(root+"/"+file+".tmp","atomic"+step);renameSync(root+"/"+file+".tmp",root+"/"+file);rows.push(["a",0,file+".tmp"],["r",0,file+".tmp"],["n",0,file]);}
   if(random()<0.3&&file)rows.push(["m",0,file]);
   if(random()<0.2)rows.push(["m",0,"pkgs/o0/src/a.ts"]);
   if(random()<0.5)rows.reverse();
   await state.apply(rows,Date.now());
   const snap=ctx.snap();
   expect([...state.members.keys()].sort()).toEqual([...snap.files.keys()].sort());
   for(const[path,mode]of snap.files)expect(state.members.get(path)).toBe(mode);
   expect([...state.directories.keys()].sort()).toEqual([...snap.directories.keys()].sort());
   const fingerprint=fingerprints(root,snap.files),keys=new Map<string,string>();
   for(const id of state.tasks.keys())keys.set(id,await state.taskKey(id));
   for(const id of state.tasks.keys())expect(keys.get(id)===previousKeys.get(id)).toBe(fingerprint.get(id)===previousFingerprint.get(id));
   previousFingerprint=fingerprint;previousKeys=keys;
  }
  const net=state.since(cursor0);
  expect(net.complete).toBe(true);
  const expected:string[]=[];
  for(const[path,mode]of ctx.snap().files){const was=start.files.get(path);if(was===undefined)expected.push("added file "+path);else if(was!==mode||false)expected.push("changed file "+path);}
  for(const path of start.files.keys())if(!ctx.snap().files.has(path))expected.push("removed file "+path);
  for(const[path,stamp]of ctx.snap().directories){const was=start.directories.get(path);if(!was)expected.push("added dir "+path);else if(was[0]!==stamp[0]||was[1]!==stamp[1])expected.push("changed dir "+path);}
  for(const path of start.directories.keys())if(!ctx.snap().directories.has(path))expected.push("removed dir "+path);
  const got=(net as any).changes.map((c:any)=>c.kind+(c.directory?" dir ":" file ")+c.path);
  const modeOnly=(list:string[])=>list.filter(x=>!x.startsWith("changed file")&&x!=="changed dir .out");
  expect(modeOnly(got).sort()).toEqual(modeOnly(expected).sort());
  for(const line of expected.filter(x=>x.startsWith("changed file")))expect(got).toContain(line);
  const owner=graph.owners[1]!,naive=naiveDigest(root,ctx.snap().files,owner,graph.owners.map(o=>o.root));
  expect(await state.refreshOwner("o1")).toBe(naive.digest);
  expect(pythonDigest(root,naive.files)).toBe(naive.digest);
 },60000);

 test("a gap in the events is repaired by reconcile and the journal refuses to vouch across it",async()=>{
  const ctx=setup("state-lost-");roots.push(ctx.root);const{root,state}=ctx,cursor=state.cursor();
  const before=await state.taskKey("o3:build");
  writeFileSync(root+"/pkgs/o0/src/a.ts","changed without any event");writeFileSync(root+"/pkgs/o4/src/new.ts","n");rmSync(root+"/pkgs/o4/src/e.ts");
  state.lost("","overflow");
  expect(state.since(cursor)).toMatchObject({complete:false,reason:"lost"});
  const rows=await state.reconcile("");
  expect(rows).toBeGreaterThan(0);
  expect(await state.taskKey("o3:build")).not.toBe(before);
  expect([...state.members.keys()].sort()).toEqual([...ctx.snap().files.keys()].sort());
  expect(await state.refreshOwner("o0")).toBe(naiveDigest(root,ctx.snap().files,graph.owners[0]!,graph.owners.map(o=>o.root)).digest);
 });

 test("verify mode proves by stat: a silent content change is found, an unchanged owner is not rehashed",async()=>{
  const ctx=setup("state-verify-",{racyWindowMilliseconds:0});roots.push(ctx.root);const{root,state}=ctx;
  state.setTrust("verify");
  const key=await state.taskKey("o1:build"),hashed=state.stats.hashCalls;
  expect(await state.taskKey("o1:build")).toBe(key);
  expect(state.stats.hashCalls).toBe(hashed);
  await Bun.sleep(20);writeFileSync(root+"/pkgs/o0/src/a.ts","silent edit longer");
  expect(await state.taskKey("o1:build")).not.toBe(key);
 });

 test("exact mode never touches the disk for a clean task",async()=>{
  const ctx=setup("state-clean-");roots.push(ctx.root);const{state}=ctx;
  await state.taskKey("o3:build");const stats={...state.stats};
  for(let i=0;i<100;i++)await state.taskKey("o3:build");
  expect(state.stats.statCalls).toBe(stats.statCalls);expect(state.stats.hashCalls).toBe(stats.hashCalls);expect(state.stats.ownerRefreshes).toBe(stats.ownerRefreshes);
 });

 test("an edit marks exactly the reverse closure stale",async()=>{
  const ctx=setup("state-closure-");roots.push(ctx.root);const{root,state}=ctx;
  for(const id of state.tasks.keys())state.memo.set(id,{id,key:await state.taskKey(id),exit:0,finishedAt:1,replay:{stdout:"",stderr:""},outputs:[]});
  expect(await state.stalePlan("o3:build")).toEqual([]);
  appendFileSync(root+"/pkgs/o1/src/b.ts","x");await state.apply([["m",0,"pkgs/o1/src/b.ts"]]);
  expect(await state.stalePlan("o3:build")).toEqual(["o1:build","o2:build","o3:build"]);
  expect(await state.stalePlan("o4:build")).toEqual([]);expect(await state.stalePlan("o1:test")).toEqual(["o1:build","o1:test"]);
  appendFileSync(root+"/pkgs/o2/inner/i.ts","x");await state.apply([["m",0,"pkgs/o2/inner/i.ts"]]);
  expect((await state.stalePlan("o3:build")).includes("inner:build")).toBe(true);expect((await state.stalePlan("o2:build")).includes("inner:build")).toBe(false);
  appendFileSync(root+"/package.json"," ");await state.apply([["m",0,"package.json"]]);
  for(const id of state.tasks.keys())expect((await state.stalePlan(id)).includes(id)).toBe(true);
 });

 test("output events invalidate the producing task unless it is running",async()=>{
  const ctx=setup("state-outputs-");roots.push(ctx.root);const{root,state}=ctx;
  mkdirSync(root+"/.out/o1",{recursive:true});writeFileSync(root+"/.out/o1/x","1");
  state.beginTask("o1:build");await state.apply([["a",0,".out/o1/x"]]);state.endTask("o1:build");
  expect(state.outputsInvalid("o1:build")).toBe(false);
  await state.apply([["m",0,".out/o1/x"]]);expect(state.outputsInvalid("o1:build")).toBe(true);expect(state.outputsInvalid("o0:build")).toBe(false);
  expect(state.members.has(".out/o1/x")).toBe(false);
  state.endTask("o1:build");expect(state.outputsInvalid("o1:build")).toBe(false);
  await state.apply([["m",0,".out/o1t/y"]]);expect(state.outputsInvalid("o1:test")).toBe(true);expect(state.outputsInvalid("o1:build")).toBe(false);
 });

 test("the journal is bounded and a foreign or too old cursor proves nothing",async()=>{
  const ctx=setup("state-journal-",{journalMaximumEvents:5});roots.push(ctx.root);const{root,state}=ctx,old=state.cursor();
  for(let i=0;i<8;i++){writeFileSync(root+"/pkgs/o4/src/j"+i+".ts","j");await state.apply([["a",0,"pkgs/o4/src/j"+i+".ts"]]);}
  expect(state.since(old)).toMatchObject({complete:false,reason:"cursor"});
  expect(state.since({instance:"other",seq:state.seq})).toMatchObject({complete:false,reason:"cursor"});
  expect(state.since(state.cursor())).toMatchObject({complete:true,changes:[]});
 });

 test("output signature by bounded stat changes with content and refuses large trees",async()=>{
  const root=tempRoot("state-sig-");roots.push(root);writeTree(root,{"out/a/x":"1","out/b":"2"});
  const first=await outputSignatureV1(root,["out/a","out/b"],100);
  expect(first).toBeDefined();expect(await outputSignatureV1(root,["out/a","out/b"],100)).toBe(first);
  await Bun.sleep(5);writeFileSync(root+"/out/a/x","changed longer");expect(await outputSignatureV1(root,["out/a","out/b"],100)).not.toBe(first);
  expect(await outputSignatureV1(root,["out"],2)).toBeUndefined();
  rmSync(root+"/out/b");expect(await outputSignatureV1(root,["out/b"],100)).toBeDefined();
 });
});
