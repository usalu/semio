/** 🧮️ Contract: a node of the derived DAG is the pure answer of a rule over raw reads and other nodes; one edit re-runs only the nodes whose raw read or read node changed, and stops at the first node whose value digest is equal (early cutoff). The values and the exact set of recomputed nodes equal a naive recompute-everything model written independently in Python (full `tomllib`, `json`), the glob matcher equals Bun.Glob and the restricted TOML reader equals Bun.TOML. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{DERIVED_NODES_POLICY_V1,createDerivedGraphV1,keyIdV1}from"../🟨️.mjs";
import{KIND_RULES_V1,cargoDependenciesV1,globMatcherV1,parseTomlSubsetV1,verifyObservedV1}from"../🧩️kinds/🟨️.mjs";
import{createNodeStoreV1}from"../💾️store/🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

type Step={name:string;edit:Record<string,string|null>;touch?:string[];expect?:{recomputedByKind?:Record<string,number>;changedByKind?:Record<string,number>}};
type Scenario={id:string;files:Record<string,string>;projects:Record<string,{manifests:string[]}>;actions?:Record<string,{env?:Record<string,string>}>;closures?:Record<string,{members:string[]}>;requests:string[];steps:Step[]};
const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const scenarios=(()=>{if(!new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture))throw Error("Closed derived-nodes fixture required");return(fixture as {scenarios:Scenario[]}).scenarios;})();
const model=(scenario:Scenario)=>{const run=Bun.spawnSync([python!,"-I",oracle,"run"],{stdin:Buffer.from(JSON.stringify(scenario))});expect(run.stderr.toString()).toBe("");expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).steps as {name:string;values:Record<string,unknown>;recomputed:string[];changed:string[];created:string[]}[];};
const labelOf=(id:string)=>id.split("\0").slice(0,2).join("|");
const kindOf=(label:string)=>label.slice(0,label.indexOf("|"));
const count=(labels:readonly string[])=>{const out:Record<string,number>={};for(const label of labels)out[kindOf(label)]=(out[kindOf(label)]??0)+1;return out;};

const environmentOf=(files:Map<string,string>)=>({listProjectFiles:async()=>[...files.keys()].filter(path=>/(^|\/)(📋️project\.json|Cargo\.toml)$/u.test(path)).sort(),readdir:async(dir:string)=>{const prefix=dir===""?"":dir+"/";return[...new Set([...files.keys()].filter(path=>path.startsWith(prefix)).map(path=>path.slice(prefix.length).split("/")[0]!))];}});

function worldOf(files:Map<string,string>,reads?:{count:number}){
 return{read:async(path:string)=>{if(reads)reads.count++;const text=files.get(path);return text===undefined?undefined:Buffer.from(text);},files:async()=>[...files.keys()]};
}

function keyOf(graph:ReturnType<typeof createDerivedGraphV1>,scenario:Scenario,label:string):any{
 const kind=label.slice(0,label.indexOf("|")),subject=label.slice(label.indexOf("|")+1),project=(root:string)=>({root,manifests:scenario.projects[root]!.manifests});
 if(kind==="manifest"||kind==="directory-names"||kind==="project-config"||kind==="project-set")return graph.key(kind,subject,{});
 if(kind==="task-key"){const cut=subject.lastIndexOf(":"),root=subject.slice(0,cut),target=subject.slice(cut+1);return graph.key(kind,subject,{root,target,env:scenario.actions?.[subject]?.env??{}});}
 if(kind==="project")return graph.key(kind,subject,project(subject));
 if(kind==="named-input"){const[root,name]=subject.split("#");return graph.key(kind,subject,{root,name,project:project(root!)});}
 if(kind==="target-inputs"||kind==="action-key"){const cut=subject.lastIndexOf(":"),root=subject.slice(0,cut),target=subject.slice(cut+1);return graph.key(kind,subject,kind==="action-key"?{root,target,project:project(root),env:scenario.actions?.[subject]?.env??{}}:{root,target,project:project(root)});}
 const members=scenario.closures![subject]!.members.map(member=>{const key=keyOf(graph,scenario,member);const at=member.indexOf("|"),k=member.slice(0,at),s=member.slice(at+1);void key;return{kind:k,subject:s,params:paramsOf(scenario,member)};});
 return graph.key("closure",subject,{members});
}

function paramsOf(scenario:Scenario,label:string):unknown{
 const kind=label.slice(0,label.indexOf("|")),subject=label.slice(label.indexOf("|")+1),project=(root:string)=>({root,manifests:scenario.projects[root]!.manifests});
 if(kind==="manifest"||kind==="directory-names"||kind==="project-config"||kind==="project-set")return{};
 if(kind==="task-key"){const cut=subject.lastIndexOf(":"),root=subject.slice(0,cut),target=subject.slice(cut+1);return{root,target,env:scenario.actions?.[subject]?.env??{}};}
 if(kind==="project")return project(subject);
 if(kind==="named-input"){const[root,name]=subject.split("#");return{root,name,project:project(root!)};}
 const cut=subject.lastIndexOf(":"),root=subject.slice(0,cut),target=subject.slice(cut+1);
 return kind==="action-key"?{root,target,project:project(root),env:scenario.actions?.[subject]?.env??{}}:{root,target,project:project(root)};
}

async function drive(scenario:Scenario,options:{maximumNodes?:number;store?:ReturnType<typeof createNodeStoreV1>}={}){
 const files=new Map(Object.entries(scenario.files)),reads={count:0},graph=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(files,reads),environment:environmentOf(files),verifyObserved:verifyObservedV1,trace:true,...options}),expected=model(scenario);
 const results:{values:Record<string,unknown>;recomputed:string[];changed:string[];reads:number}[]=[];
 const read=async()=>{const values:Record<string,unknown>={};for(const label of scenario.requests)values[label]=JSON.parse(JSON.stringify((await graph.get(keyOf(graph,scenario,label))).value),label.startsWith("project-config|")?(key,value)=>key==="error"&&typeof value==="string"?"<error>":value:undefined);return values;};
 graph.drainTrace();const before=reads.count;
 let values=await read();
 results.push({values,recomputed:graph.drainTrace().filter(event=>event.kind==="computed").map(event=>labelOf(event.id)),changed:[],reads:reads.count-before});
 for(const step of scenario.steps){
  for(const[path,text]of Object.entries(step.edit)){if(text===null)files.delete(path);else files.set(path,text);}
  const start=reads.count,dirty={paths:[...Object.keys(step.edit),...(step.touch??[])]},invalidation=await graph.invalidate(dirty);
  values=await read();
  results.push({values,recomputed:graph.drainTrace().filter(event=>event.kind==="computed").map(event=>labelOf(event.id)),changed:invalidation.changed.map(key=>key.kind+"|"+key.subject),reads:reads.count-start});
 }
 return{results,expected,graph,files};
}

test("the fixture scenario equals the naive model after every edit and the recomputed sets are exactly minimal",async()=>{
 for(const scenario of scenarios){
  const{results,expected,graph}=await drive(scenario);
  expect(graph.stats().patched,"glob nodes are patched, not rebuilt").toBeGreaterThan(0);
  expect(results.length).toBe(expected.length);
  results.forEach((result,index)=>{
   const want=expected[index]!,name=scenario.id+": "+want.name;
   expect(result.values,name).toEqual(want.values);
   expect(new Set(result.recomputed).size,name+" computed twice").toBe(result.recomputed.length);
   expect([...result.recomputed].sort(),name).toEqual(want.recomputed);
   expect([...result.changed].sort(),name).toEqual(want.changed);
   const pinned=index===0?undefined:scenario.steps[index-1]!.expect;
   if(pinned?.recomputedByKind)expect(count(result.recomputed),name).toEqual(pinned.recomputedByKind);
   if(pinned?.changedByKind)expect(count(result.changed),name).toEqual(pinned.changedByKind);
  });
 }
},120000);

test("a whitespace edit of a manifest reads one file and recomputes one manifest node and no closure",async()=>{
 const scenario=scenarios[0]!,{results}=await drive(scenario);
 expect(results[0]!.reads).toBeGreaterThan(10);
 expect(results[1]!.reads).toBe(1);
 expect(results[1]!.recomputed).toEqual(["manifest|a/project.json"]);
 expect(results[3]!.recomputed).toEqual([]);
 expect(results[3]!.reads).toBe(2);
});

function generator(seed:number){
 let state=seed>>>0;const next=(limit:number)=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state%limit;};
 const base=scenarios[0]!,pool=Object.keys(base.files),steps:Step[]=[];
 const variants:Record<string,(random:number)=>string>={
  "a/package.json":random=>JSON.stringify({name:"@x/a",dependencies:Object.fromEntries(["b","c","left-pad","zed"].filter((_,index)=>random>>index&1).map(name=>[name,"1."+random%3]))},null,random%2?2:undefined),
  "b/Cargo.toml":random=>"[package]\nname = \"b\"\n[dependencies]\n"+["serde","anyhow","tokio"].filter((_,index)=>random>>index&1).map(name=>name+" = { version = \""+random%4+"\", features = [\"f"+random%2+"\"] } # c").join("\n")+"\n",
  "c/go.mod":random=>"module c\nrequire (\n"+["x/a","x/b","x/c"].filter((_,index)=>random>>index&1).map(name=>"\t"+name+" v"+random%3+"\n").join("")+")\n",
  "a/project.json":random=>JSON.stringify({name:"a",namedInputs:{default:random%2?["src/**/*.ts"]:["src/*.ts","!src/x*.ts"],docs:["docs/*.md"]},targets:{build:{command:"tsc"+random%3,inputs:random%4?["default","^default"]:["default","docs","tsconfig.json"]},test:{command:"t",inputs:["docs"]}}}),
 };
 for(let index=0;index<12;index++){
  const kind=next(6),edit:Record<string,string|null>={};
  const manifests=Object.keys(variants);
  if(kind===0){const path=manifests[next(manifests.length)]!;edit[path]=variants[path]!(next(16));}
  else if(kind===1)edit["a/src/f"+next(4)+".ts"]=next(3)?"export const v = "+next(5)+";\n":null;
  else if(kind===2){const path=pool[next(pool.length)]!;edit[path]=null;}
  else if(kind===3){const path=pool[next(pool.length)]!;edit[path]=base.files[path]!;}
  else if(kind===4){const path=manifests[next(manifests.length)]!;edit[path]=(base.files[path]??"")+"\n";}
  else edit["a/src/x"+next(3)+".ts"]="export {};\n"+next(2);
  steps.push({name:"random "+index,edit,...(next(4)===0?{touch:[pool[next(pool.length)]!]}:{})});
 }
 return{...base,id:"random-"+seed,steps};
}

test("random edit sequences equal the naive model in values and in the exact recomputed and changed sets",async()=>{
 for(const seed of [1,2,3,5,8,13,21,34]){
  const scenario=generator(seed),{results,expected}=await drive(scenario);
  results.forEach((result,index)=>{
   const want=expected[index]!,name=scenario.id+": "+want.name;
   expect(result.values,name).toEqual(want.values);
   expect([...result.recomputed].sort(),name).toEqual(want.recomputed);
   expect([...result.changed].sort(),name).toEqual(want.changed);
  });
 }
},300000);

test("a graph bounded to a few nodes evicts with every node above it and still answers like the model",async()=>{
 const scenario=scenarios[0]!,{results,expected,graph}=await drive(scenario,{maximumNodes:7});
 results.forEach((result,index)=>expect(result.values,expected[index]!.name).toEqual(expected[index]!.values));
 expect(graph.stats().nodes).toBeLessThanOrEqual(7);
 expect(graph.stats().evicted).toBeGreaterThan(0);
});

test("a released graph holds no settled node and answers every request again like the model, from the durable store without running a rule",async()=>{
 const root=mkdtempSync(join(tmpdir(),"derived-release-")),scenario=scenarios[0]!;
 try{
  const store=createNodeStoreV1({root}),files=new Map(Object.entries(scenario.files)),graph=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(files),environment:environmentOf(files),verifyObserved:verifyObservedV1,store,trace:true}),expected=model({...scenario,steps:[]})[0]!;
  const read=async()=>Object.fromEntries(await Promise.all(scenario.requests.map(async label=>[label,JSON.parse(JSON.stringify((await graph.get(keyOf(graph,scenario,label))).value),label.startsWith("project-config|")?(key,value)=>key==="error"&&typeof value==="string"?"<error>":value:undefined)])));
  expect(await read()).toEqual(expected.values);
  await graph.flush();
  const computed=graph.stats().computed;expect(graph.stats().nodes).toBeGreaterThan(0);
  expect(await graph.release()).toBe(0);expect(graph.stats().nodes).toBe(0);expect(graph.stats().rdeps).toBe(0);
  expect(await read()).toEqual(expected.values);
  expect(graph.stats().computed,"a released node is read back from the store, never derived again").toBe(computed);
  const bare=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(files),environment:environmentOf(files),verifyObserved:verifyObservedV1});
  await bare.get(keyOf(bare,scenario,scenario.requests[0]!));expect(await bare.release(),"a graph without a store cannot answer a dropped node again and keeps it").toBeUndefined();expect(bare.stats().nodes).toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the durable store lets a new graph revalidate every node without running a rule, and an overflow dirty set rereads the world",async()=>{
 const root=mkdtempSync(join(tmpdir(),"derived-store-")),scenario=scenarios[0]!;
 try{
  const store=createNodeStoreV1({root}),first=await drive({...scenario,steps:[]},{store});
  await first.graph.flush();
  const files=new Map(Object.entries(scenario.files)),second=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(files),environment:environmentOf(files),verifyObserved:verifyObservedV1,store:createNodeStoreV1({root}),trace:true});
  for(const label of scenario.requests)await second.get(keyOf(second,scenario,label));
  expect(second.stats().computed).toBe(0);expect(second.stats().revalidated).toBeGreaterThan(0);
  files.set("a/src/index.ts","export const a = 9;\n");
  const third=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(files),environment:environmentOf(files),verifyObserved:verifyObservedV1,store:createNodeStoreV1({root}),trace:true});
  const values=await Promise.all(scenario.requests.map(async label=>JSON.parse(JSON.stringify((await third.get(keyOf(third,scenario,label))).value))));
  const edited=model({...scenario,files:Object.fromEntries(files),steps:[]})[0]!;
  expect(Object.fromEntries(scenario.requests.map((label,index)=>[label,values[index]]))).toEqual(edited.values);
  expect(third.stats().computed).toBeLessThan(first.graph.stats().computed);
  const pruned=await createNodeStoreV1({root,limits:{...DERIVED_NODES_POLICY_V1.store,maximumRecords:5}}).prune();
  expect(pruned.kept).toBe(5);expect(pruned.removed).toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("concurrent requests of one key run its rule once, a cancelled request throws, progress is reported and a cycle is refused",async()=>{
 const files=new Map(Object.entries(scenarios[0]!.files)),reads={count:0},graph=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(files,reads),environment:environmentOf(files),trace:true});
 const key=graph.key("manifest","a/package.json",{});
 await Promise.all([graph.get(key),graph.get(key),graph.get(key)]);
 expect(graph.stats().computed).toBe(1);expect(reads.count).toBe(1);
 const controller=new AbortController();controller.abort();
 await expect(graph.get(graph.key("manifest","b/Cargo.toml",{}),{signal:controller.signal})).rejects.toBeDefined();
 files.set("a/package.json","{\"name\":\"z\"}");
 const events:unknown[]=[];await graph.invalidate({paths:["a/package.json"]},{progress:event=>{events.push(event);}});
 expect(events.length).toBeGreaterThan(0);expect((await graph.get(key)).value.name).toBe("z");
 const loop=createDerivedGraphV1({rules:{closure:async ctx=>ctx.node(ctx.key("closure",ctx.self.subject==="a"?"b":"a",{}))},world:worldOf(new Map())});
 await expect(loop.get(loop.key("closure","a",{}))).rejects.toThrow(/cycle/u);
});

test("the glob matcher equals Bun.Glob and the restricted TOML reader equals Bun.TOML on the dependency tables",()=>{
 const paths=["src/a.ts","src/deep/b.ts","src/deep/er/c.test.ts","a.ts","docs/x.md","docs/y/z.md","src/😀.ts","x.rs","lib/a.rs"];
 for(const pattern of ["src/**/*.ts","**/*.ts","docs/*.md","*.ts","**/*.{rs,md}","src/?.ts","src/**","lib/*.rs"]){
  const match=globMatcherV1([pattern]),glob=new Bun.Glob(pattern);
  expect(paths.filter(match),pattern).toEqual(paths.filter(path=>glob.match(path)));
 }
 const text=scenarios[0]!.files["b/Cargo.toml"]!,own=parseTomlSubsetV1(text) as any,bun=(Bun as any).TOML.parse(text);
 expect(own.dependencies).toEqual(bun.dependencies);expect(own["dev-dependencies"]).toEqual(bun["dev-dependencies"]);expect(own.target).toEqual(bun.target);expect(own.package).toEqual(bun.package);
 expect((cargoDependenciesV1(text) as any).tables.dependencies.tokio).toEqual({version:"1",features:["macros","rt"]});
});

test("the keys are stable identities of kind, subject and params",()=>{
 const graph=createDerivedGraphV1({rules:KIND_RULES_V1,world:worldOf(new Map())});
 expect(keyIdV1(graph.key("manifest","x",{a:1,b:2}))).toBe(keyIdV1(graph.key("manifest","x",{b:2,a:1})));
 expect(keyIdV1(graph.key("manifest","x",{a:1}))).not.toBe(keyIdV1(graph.key("manifest","x",{a:2})));
});

test("a graph passes one immutable environment to every rule and keys its nodes by the environment key",async()=>{
 const rules={manifest:async(ctx:any)=>{(()=>{"use strict";expect(()=>{ctx.environment.provider="other";}).toThrow();})();return{provider:ctx.environment.provider};}};
 const environment={provider:"designation-a"},one=createDerivedGraphV1({rules,world:worldOf(new Map()),environment,environmentKey:"a"}),two=createDerivedGraphV1({rules,world:worldOf(new Map()),environment,environmentKey:"b"});
 expect((await one.get(one.key("manifest","x"))).value).toEqual({provider:"designation-a"});
 expect(keyIdV1(one.key("manifest","x"))).not.toBe(keyIdV1(two.key("manifest","x")));
 expect(Object.isFrozen(environment)).toBe(true);
});

test("an observed read seeds by its paths and by their children, covers a dirty directory, is verified by the rule, and survives the store",async()=>{
 const files=new Map([["d/a.txt","1"],["d/sub/b.txt","2"],["e/c.txt","3"]]),verified:string[]=[];
 const options={rules:KIND_RULES_V1,world:worldOf(files),environment:environmentOf(files),trace:true,verifyObserved:async(ref:any,context:any)=>{verified.push(ref.rows.dir);return verifyObservedV1(ref,context);}};
 const root=mkdtempSync(join(tmpdir(),"derived-observed-"));
 try{
  const graph=createDerivedGraphV1({...options,store:createNodeStoreV1({root})}),key=graph.key("directory-names","d");
  expect((await graph.get(key)).value.names.sort()).toEqual(["a.txt","sub"]);
  files.set("d/a.txt","changed");
  let result=await graph.invalidate({paths:["d/a.txt"]});
  expect(result.computed).toBe(0);expect(verified).toEqual(["d"]);
  files.set("d/new.txt","x");
  result=await graph.invalidate({paths:["d/new.txt"]});
  expect(result.computed).toBe(1);expect(result.changed.map(item=>item.subject)).toEqual(["d"]);
  files.delete("d/sub/b.txt");files.set("d/sub2/z.txt","y");files.delete("d/new.txt");
  result=await graph.invalidate({paths:[],directories:["d"]});
  expect(result.computed).toBe(1);expect((await graph.get(key)).value.names.sort()).toEqual(["a.txt","sub2"]);
  await graph.flush();
  files.set("d/late.txt","x");
  const again=createDerivedGraphV1({...options,store:createNodeStoreV1({root})});
  expect((await again.get(again.key("directory-names","d"))).value.names.sort()).toEqual(["a.txt","late.txt","sub2"]);
  expect((await graph.invalidate({paths:["e/c.txt"]})).computed).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("heavy glob work follows the capacity grant and the memory release is guarded by its measured gain",async()=>{
 const widths:number[]=[];
 for(const grant of [{cpus:2,availableBytes:1<<30},{cpus:16,availableBytes:16*2**30}]){
  let live=0,peak=0;
  const world={read:async()=>undefined,files:async(root:string)=>{live++;peak=Math.max(peak,live);await new Promise(done=>setTimeout(done,15));live--;return[root+"/a"];}};
  const rules={x:async(ctx:any)=>ctx.glob("",[ctx.self.subject+"/**"])};
  const graph=createDerivedGraphV1({rules,world,capacity:()=>grant});
  await graph.getMany(Array.from({length:12},(_,index)=>graph.key("x","d"+index)));
  widths.push(peak);
 }
 expect(widths[0]).toBe(1);expect(widths[1]).toBeGreaterThan(2);expect(widths[1]).toBeLessThanOrEqual(DERIVED_NODES_POLICY_V1.heavyMaximum);
 let calls=0;
 const world={read:async()=>undefined,files:async()=>[]},graph=createDerivedGraphV1({rules:KIND_RULES_V1,world,release:()=>{calls++;}});
 await graph.getMany([graph.key("manifest","m")]);
 expect(calls).toBe(0);
});

test("a record far above the record limit is durable as content addressed chunks and a new process finds it without running the rule",async()=>{
 const{createCasStoreV1}=await import("../../../🗄️cas/🗃️store/🟦️.ts");
 const dir=mkdtempSync(join(tmpdir(),"derived-large-")),rows=200000;
 const paths=Array.from({length:rows},(_,index)=>"w/"+String(index).padStart(6,"0")+".ts");
 let ruleRuns=0,walks=0;
 const world={read:async(path:string)=>Buffer.from(path),files:async()=>{walks++;return paths;}};
 const rules={walk:async(ctx:any)=>{ruleRuns++;const found=await ctx.glob("",["w/**/*.ts"]);return{rows:paths.slice(0,rows),digest:found.digest,files:found.files};}};
 const cas=createCasStoreV1({root:join(dir,"cas")}),open=()=>createDerivedGraphV1({rules,world,store:createNodeStoreV1({root:join(dir,"nodes"),blobs:cas as any}),concurrency:64});
 try{
  const first=open(),key=first.key("walk","crate-index"),before=(await first.get(key)).value;
  await first.flush();
  expect(before.rows.length).toBe(rows);expect(before.files).toBe(rows);
  const{stat}=await import("node:fs/promises"),{readdir}=await import("node:fs/promises");
  const names=await readdir(join(dir,"nodes","n"));
  expect((await stat(join(dir,"nodes","n",names[0]!))).size).toBeLessThan(4096);
  ruleRuns=0;
  const second=open(),again=(await second.get(second.key("walk","crate-index"))).value;
  expect(ruleRuns).toBe(0);expect(second.stats().computed).toBe(0);expect(again.digest).toBe(before.digest);expect(again.rows.length).toBe(rows);
  paths.push("w/new.ts");
  const third=open();await third.get(third.key("walk","crate-index"));
  expect(ruleRuns).toBe(1);
  expect(walks).toBeGreaterThan(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
},120000);

test("the values of the project assembly kinds satisfy the closed schema definitions after every step",async()=>{
 const scenario=scenarios.find(row=>row.id==="project assembly")!,{results}=await drive(scenario),ajv=new Ajv2020({strict:false}),valid=(name:string)=>ajv.compile({$ref:"#/definitions/"+name,definitions:schema.definitions});
 const config=valid("ProjectConfigValue"),set=valid("ProjectSetValue"),task=valid("TaskKeyValue");
 expect(results.length).toBeGreaterThan(5);
 for(const result of results)for(const[label,value]of Object.entries(result.values)){
  const validator=label.startsWith("project-config|")?config:label.startsWith("project-set|")?set:label.startsWith("task-key|")?task:undefined;
  if(validator)expect(validator(value),label+" "+JSON.stringify(validator.errors)).toBe(true);
 }
 expect(results[0]!.values["project-set|workspace"]).toMatchObject({names:{a:"a",b:"b",d:"d"},duplicates:[]});
 const whitespace=results[2]!;
 expect(whitespace.recomputed.filter(label=>label.startsWith("task-key|")),"a whitespace edit of a manifest stops at its config node").toEqual([]);
 expect(whitespace.recomputed).toContain("project-config|b");
});
