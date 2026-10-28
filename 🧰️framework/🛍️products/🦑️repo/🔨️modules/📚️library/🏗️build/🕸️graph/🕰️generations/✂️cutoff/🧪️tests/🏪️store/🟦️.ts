/** 🏪️ Contract: a generation store whose projects hold derived facts (the import closure of an entry) refreshes exactly the projects whose closure changed. An edit that leaves the closure equal refreshes nobody and still publishes the new supports (so the next verification finds nothing stale); an edit that changes it refreshes exactly the projects that hold the fact; every generation equals the graph of a cold build of the same tree and the closures equal an independent Python walker. */
import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,normalize}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createFrontierV1,createGenerationStoreV1,type GenerationRowV1}from"../../../🟨️.mjs";
import fixture from"./🔣️.json";

const sha=(text:string|Uint8Array)=>createHash("sha256").update(text).digest("hex"),IDENTITY=sha("cutoff-store-identity"),IMPLEMENTATION=sha("cutoff-store-implementation"),python=testPythonV1(),walker=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py");
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const fileRow=(root:string,path:string):GenerationRowV1=>({kind:"file",path,fact:{sha256:sha(readFileSync(join(root,path)))}});
const walk=(root:string,entry:string):string[]=>{const seen=new Set<string>(),todo=[entry];while(todo.length){const rel=todo.pop()!;if(seen.has(rel))continue;seen.add(rel);for(const line of readFileSync(join(root,rel),"utf8").split("\n"))if(line.startsWith("import ")){let name=line.slice(7).trim();if(!name.endsWith(".ts"))name+=".ts";todo.push(normalize(join(dirname(rel),name)));}}return[...seen].sort();};
const question=(entry:string)=>JSON.stringify({entries:[entry]});

function host(root:string,counters:{computed:string[];derived:number}){
 const derive=async(row:GenerationRowV1)=>{counters.derived++;const entry=(JSON.parse(row.path) as {entries:string[]}).entries[0]!;try{const closure=walk(root,entry);return{fact:sha(JSON.stringify(closure)),rows:closure.map(path=>fileRow(root,path))};}catch(error){throw error;}};
 const holds=async(rows:readonly GenerationRowV1[])=>rows.every(row=>{try{return row.kind==="file"&&(row.fact as {sha256:string}).sha256===sha(readFileSync(join(root,row.path)));}catch{return false;}});
 const compute=async(spec:{cold:boolean;owners:string[]})=>{
  const names=spec.cold?fixture.owners.map(owner=>owner.name):spec.owners,owners=[],derived=[];
  for(const name of names){
   const owner=fixture.owners.find(row=>row.name===name)!,closure=walk(root,owner.entry),head:GenerationRowV1={kind:"derived",path:question(owner.entry),fact:sha(JSON.stringify(closure))};
   counters.computed.push(name);
   owners.push({name,root:name,kind:"project" as const,node:{name,closure},edges:[],targets:["build"],config:name+"/project.json",rows:[fileRow(root,name+"/project.json"),head]});
   derived.push({row:head,support:closure.map(path=>fileRow(root,path))});
  }
  return{owners,env:{digest:sha("env"),rows:[]},global:{externalNodes:{},edges:[],unavailable:[]},derived};
 };
 const membership=async()=>({digest:sha("members"),frontier:createFrontierV1({directories:[],members:[],capturedAt:0}),changes:[]});
 return{derive,holds,compute,membership};
}

async function graphOf(store:ReturnType<typeof createGenerationStoreV1>){const current=await store.read();return(await store.assemble(current!)).projects;}

test("the store refreshes exactly the projects whose derived closure changed, and every generation equals a cold build and the Python walker",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cutoff-store-"))),storeRoot=join(root,"store");roots.push(root);
 write(root,fixture.files);
 const counters={computed:[] as string[],derived:0},parts=host(root,counters),store=createGenerationStoreV1({root:storeRoot,holds:parts.holds,derive:parts.derive,resources:{facts:"facts"}});
 let generation=-1;
 for(const step of fixture.steps){
  write(root,step.edit);counters.computed.length=0;counters.derived=0;
  const result=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:parts.membership,compute:parts.compute});
  expect([...new Set(counters.computed)].sort(),step.name).toEqual(step.dirty);
  expect(result.current.pointer.generation>generation,step.name+" publishes").toBe(step.published);
  if(!step.published)expect(result.current.pointer.generation,step.name).toBe(generation);
  generation=result.current.pointer.generation;
  if(step.held!==undefined)expect(result.plan?.cutoff?.held??0,step.name).toBe(step.held);
  const incremental=await graphOf(store);
  const coldRoot=join(root,"cold-"+generation),cold=createGenerationStoreV1({root:coldRoot,holds:parts.holds,derive:parts.derive,resources:{facts:"facts"}});
  await cold.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:parts.membership,compute:parts.compute});
  expect(incremental,step.name+": incremental equals cold").toEqual(await graphOf(cold));
  for(const owner of fixture.owners){
   const run=Bun.spawnSync([python!,"-I",walker,root,owner.entry]),expected=JSON.parse(run.stdout.toString()) as string[];
   expect((incremental[owner.name] as {closure:string[]}).closure,step.name+": "+owner.name).toEqual(expected);
  }
  const again=await store.plan({current:await store.read(),identity:IDENTITY,changes:[]});
  expect(again.kind,step.name+": nothing stale afterwards").toBe("clean");expect(again.cutoff?.questions??0,step.name).toBe(0);
 }
});

test("a verification scoped to some projects asks only the derived questions those projects hold; the others stay stale for the verification that selects their holders",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cutoff-scope-"))),storeRoot=join(root,"store");roots.push(root);write(root,fixture.files);
 const counters={computed:[] as string[],derived:0},parts=host(root,counters),store=createGenerationStoreV1({root:storeRoot,holds:parts.holds,derive:parts.derive,resources:{facts:"facts"}});
 await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:parts.membership,compute:parts.compute});
 writeFileSync(join(root,"lib/c.ts"),"// shared by the closures of p1, p2 and p4\n");counters.derived=0;
 const scoped=await store.plan({current:await store.read(),identity:IDENTITY,changes:[],only:new Set(["p2"])});
 expect(counters.derived,"only the question p2 holds").toBe(1);expect(scoped.cutoff?.questions).toBe(1);expect(scoped.cutoff?.held).toBe(1);expect([...scoped.dirty]).toEqual([]);
 counters.derived=0;const whole=await store.plan({current:await store.read(),identity:IDENTITY,changes:[]});
 expect(counters.derived,"the deferred question is still stale for a whole verification").toBe(2);expect(whole.cutoff?.questions).toBe(2);
 writeFileSync(join(root,"lib/c.ts"),"import ./d\n");counters.derived=0;
 const changed=await store.plan({current:await store.read(),identity:IDENTITY,changes:[],only:new Set(["p2"])});
 expect(counters.derived).toBe(1);expect([...changed.dirty]).toEqual(["p2"]);
 counters.derived=0;const holders=await store.plan({current:await store.read(),identity:IDENTITY,changes:[],only:new Set(["p4"])});
 expect(counters.derived).toBe(1);expect([...holders.dirty]).toEqual(["p4"]);
 counters.computed.length=0;await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",only:new Set(["p2"]),membership:parts.membership,compute:parts.compute});
 expect([...new Set(counters.computed)].sort()).toEqual(["p2"]);
 counters.computed.length=0;await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:parts.membership,compute:parts.compute});
 expect([...new Set(counters.computed)].sort(),"the deferred holders are refreshed by the next whole verification").toEqual(["p1","p4"]);
 writeFileSync(join(root,"lib/c.ts"),fixture.files["lib/c.ts"]);
},60000);

test("a derived fact whose computation fails stales exactly its holders, and a store without a computation treats every suspect as changed",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cutoff-fail-"))),storeRoot=join(root,"store");roots.push(root);write(root,fixture.files);
 const counters={computed:[] as string[],derived:0},parts=host(root,counters);
 const store=createGenerationStoreV1({root:storeRoot,holds:parts.holds,derive:parts.derive,resources:{facts:"facts"}});
 await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:parts.membership,compute:parts.compute});
 writeFileSync(join(root,"lib/b.ts"),"import ./missing\n");
 const failing=await store.plan({current:await store.read(),identity:IDENTITY,changes:[]});
 expect([...failing.dirty].sort()).toEqual(["p1","p4"]);expect(failing.cutoff?.failed).toBe(1);
 writeFileSync(join(root,"lib/b.ts"),fixture.files["lib/b.ts"]);
 writeFileSync(join(root,"lib/c.ts"),"// again\n");
 const bare=createGenerationStoreV1({root:storeRoot,holds:parts.holds,resources:{facts:"facts"}}),without=await bare.plan({current:await bare.read(),identity:IDENTITY,changes:[]});
 expect([...without.dirty].sort()).toEqual(["p1","p2","p4"]);
});

test("under seeded random edits the refreshed projects are exactly those whose closure or own file changed, and every generation equals a cold build",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cutoff-random-"))),storeRoot=join(root,"store");roots.push(root);
 let seed=20261026;const random=(count:number)=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return(seed>>>9)%count;};
 const modules=Array.from({length:14},(_,at)=>"lib/f"+at+".ts"),imports=new Map<string,Set<string>>(modules.map(path=>[path,new Set<string>()]));
 const text=(path:string,comment:number)=>[...imports.get(path)!].sort().map(target=>"import "+normalize(join(dirname(path)==="lib"?"./":"",target.replace(/^lib\//u,"")))+"\n").join("")+"// "+comment+"\n";
 for(const path of modules)for(let step=0;step<2;step++)imports.get(path)!.add(modules[random(modules.length)]!);
 for(const path of modules)imports.get(path)!.delete(path);
 const files:Record<string,string>={};for(const path of modules)files[path]=text(path,0);
 const owners=Array.from({length:6},(_,at)=>({name:"r"+at,entry:modules[random(modules.length)]!}));
 for(const owner of owners)files[owner.name+"/project.json"]="{}\n";
 write(root,files);
 const counters={computed:[] as string[],derived:0},local=host(root,counters),entryOf=new Map(owners.map(owner=>[owner.name,owner.entry]));
 const compute=async(spec:{cold:boolean;owners:string[]})=>{const names=spec.cold?[...entryOf.keys()]:spec.owners,out=[],derived=[];for(const name of names){const closure=walk(root,entryOf.get(name)!),head:GenerationRowV1={kind:"derived",path:question(entryOf.get(name)!),fact:sha(JSON.stringify(closure))};counters.computed.push(name);out.push({name,root:name,kind:"project" as const,node:{name,closure},edges:[],targets:["build"],config:name+"/project.json",rows:[fileRow(root,name+"/project.json"),head]});derived.push({row:head,support:closure.map(path=>fileRow(root,path))});}return{owners:out,env:{digest:sha("env"),rows:[]},global:{externalNodes:{},edges:[],unavailable:[]},derived};};
 const store=createGenerationStoreV1({root:storeRoot,holds:local.holds,derive:local.derive,resources:{facts:"facts"}});
 const refresh=()=>store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:local.membership,compute});
 await refresh();
 let previous=new Map(owners.map(owner=>[owner.name,JSON.stringify(walk(root,owner.entry))])),refreshing=0,absorbed=0;
 for(let step=0;step<60;step++){
  const kind=random(10),path=modules[random(modules.length)]!,target=modules[random(modules.length)]!,own=random(6),existing=[...imports.get(path)!].sort();
  const edit:Record<string,string>={};let ownEdit:string|undefined;
  if(kind<4&&target!==path)imports.get(path)!.add(target);else if(kind>=4&&kind<7&&existing.length)imports.get(path)!.delete(existing[random(existing.length)]!);
  if(kind>=8){ownEdit="r"+own;edit[ownEdit+"/project.json"]="{\"step\":"+step+"}\n";}
  edit[path]=text(path,step+1);write(root,edit);counters.computed.length=0;
  await refresh();
  const next=new Map(owners.map(owner=>[owner.name,JSON.stringify(walk(root,owner.entry))])),expected=new Set(owners.filter(owner=>next.get(owner.name)!==previous.get(owner.name)).map(owner=>owner.name));
  if(ownEdit)expected.add(ownEdit);
  expect([...new Set(counters.computed)].sort(),"step "+step).toEqual([...expected].sort());
  if(expected.size)refreshing++;else absorbed++;
  previous=next;
  const incremental=await graphOf(store),cold=createGenerationStoreV1({root:join(root,"cold-"+step),holds:local.holds,derive:local.derive,resources:{facts:"facts"}});
  await cold.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership:local.membership,compute});
  expect(incremental,"step "+step+": incremental equals cold").toEqual(await graphOf(cold));
 }
 expect(refreshing,"edits that changed a closure or an own file").toBeGreaterThan(5);
 expect(absorbed,"edits the cutoff absorbed").toBeGreaterThan(5);
});

test("the compute spec names the members that changed (directories excluded) and nothing on a cold build, so the discovery of the test and policy owners can tell whether its inputs moved",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cutoff-changed-"))),storeRoot=join(root,"store");roots.push(root);write(root,fixture.files);
 const counters={computed:[] as string[],derived:0},local=host(root,counters),specs:{cold:boolean;changed:string[]}[]=[];
 const store=createGenerationStoreV1({root:storeRoot,holds:local.holds,derive:local.derive,resources:{facts:"facts"}});
 let changes:{path:string;kind:"changed"|"added";directory?:boolean}[]=[];
 const membership=async()=>({digest:sha("members"+changes.length),frontier:createFrontierV1({directories:[],members:[],capturedAt:0}),changes});
 const compute=async(spec:{cold:boolean;owners:string[];changed:string[]})=>{specs.push({cold:spec.cold,changed:spec.changed});return local.compute(spec);};
 const refresh=()=>store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:"fresh",membership,compute:compute as never});
 await refresh();
 changes=[{path:"p3/project.json",kind:"changed"},{path:"p3",kind:"changed",directory:true},{path:"lib/new.feature",kind:"added"}];
 write(root,{"p3/project.json":"{\"x\":1}\n"});
 await refresh();
 expect(specs[0]).toEqual({cold:true,changed:[]});
 expect(specs[1]!.cold).toBe(false);expect([...specs[1]!.changed].sort()).toEqual(["lib/new.feature","p3/project.json"]);
});
