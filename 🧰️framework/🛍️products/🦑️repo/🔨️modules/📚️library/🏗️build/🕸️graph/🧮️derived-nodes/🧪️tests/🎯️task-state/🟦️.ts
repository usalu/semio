/** 🎯️ Contract: the key of a planned task is the digest of its complete declaration, its resolved inputs and the keys of its dependencies; a task whose recorded key equals the key of the plan is up to date and every other task is stale. The keys and the stale sets equal an independent Python model after every edit, for a fixture that any implementation can run. */
import{afterAll,expect,test}from"bun:test";
import{chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createCasStoreV1}from"../../../../🗄️cas/🗃️store/🟦️.ts";
import{createDerivedGraphV1}from"../../🟨️.mjs";
import{createDiskWorldV1}from"../../🌍️world/🟨️.mjs";
import{createStatIndexV1}from"../../🗂️stat-index/🟨️.mjs";
import{KIND_RULES_V1,verifyObservedV1}from"../../🧩️kinds/🟨️.mjs";
import{createHostTaskStateV1,createTaskRecorderV1,taskKeysV1}from"../../🎯️task-state/🟨️.mjs";
import{machinePathV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{nativeImageSharedReadResourceV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
import{acquireResourceLease}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import fixture from"./🔣️.json";

type Task={root:string;target:string;dependsOn:string[];cacheable:boolean};
type Scenario={id:string;files:Record<string,string>;declarations:Record<string,string>;order:string[];tasks:Record<string,Task>;run?:string[];steps:{name:string;edit:Record<string,string|null>;run?:string[]}[]};
const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py"),directories:string[]=[];
afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const scratch=(name:string)=>{const path=mkdtempSync(join(tmpdir(),"task-state-"+name+"-"));directories.push(path);return path;};
const model=(scenario:Scenario)=>{const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify(scenario))});expect(run.stderr.toString()).toBe("");expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()) as {steps:{name:string;keys:Record<string,string|null>;stale:string[]}[]};};

async function drive(scenario:Scenario){
 const files=new Map(Object.entries(scenario.files)),world={read:async(path:string)=>{const text=files.get(path);return text===undefined?undefined:Buffer.from(text);},files:async()=>[...files.keys()]};
 const graph=createDerivedGraphV1({rules:KIND_RULES_V1,world,environment:{},verifyObserved:verifyObservedV1}),recorder=createTaskRecorderV1({directory:join(scratch("pointers"),"task-keys"),cas:createCasStoreV1({root:scratch("cas")})});
 const tasks=new Map(Object.entries(scenario.tasks).map(([id,task])=>[id,{...task,declaration:scenario.declarations[id]}]));
 const plan=async()=>{
  const keys=await taskKeysV1({graph,tasks,order:scenario.order}),fresh=await recorder.upToDate("w",new Map([...keys].filter(([id])=>tasks.get(id)!.cacheable)));
  return{keys:Object.fromEntries(scenario.order.map(id=>[id,keys.get(id)??null])),stale:scenario.order.filter(id=>!fresh.has(id)),raw:keys};
 };
 const out:{name:string;keys:Record<string,string|null>;stale:string[]}[]=[];
 const steps=[{name:"initial",edit:{} as Record<string,string|null>,run:scenario.run??[]},...scenario.steps];
 for(const[index,step]of steps.entries()){
  if(index){for(const[path,text]of Object.entries(step.edit)){if(text===null)files.delete(path);else files.set(path,text);}await graph.invalidate({paths:Object.keys(step.edit)});}
  const planned=await plan();out.push({name:step.name,keys:planned.keys,stale:planned.stale});
  await recorder.record("w",(step.run??[]).filter(id=>planned.raw.get(id)!==null&&tasks.get(id)!.cacheable).map(id=>({id,key:planned.raw.get(id)!})));
 }
 return out;
}

test("the keys and the stale tasks equal the independent model after every edit",async()=>{
 for(const scenario of fixture.scenarios as unknown as Scenario[]){
  const expected=model(scenario).steps,actual=await drive(scenario);
  expect(actual.length).toBe(expected.length);
  actual.forEach((row,index)=>{expect(row.keys,scenario.id+": "+row.name).toEqual(expected[index]!.keys);expect(row.stale,scenario.id+": "+row.name).toEqual(expected[index]!.stale);});
  expect(actual[1]!.stale,"after a full successful run only the task that is never cacheable is stale").toEqual(["c:build"]);
 }
},120000);

test("a record that the content store collected makes its task stale again, and a record of another key never matches",async()=>{
 const cas=createCasStoreV1({root:scratch("collected-cas")}),directory=join(scratch("collected"),"task-keys"),recorder=createTaskRecorderV1({directory,cas}),key="a".repeat(64);
 await recorder.record("w",[{id:"p:t",key}]);
 expect([...await recorder.upToDate("w",new Map([["p:t",key]]))]).toEqual(["p:t"]);
 expect([...await recorder.upToDate("w",new Map([["p:t","b".repeat(64)]]))]).toEqual([]);
 expect([...await recorder.upToDate("other",new Map([["p:t",key]]))]).toEqual([]);
 expect([...await recorder.upToDate("w",new Map([["p:t",null]]))]).toEqual([]);
 const[pointerPath]=[...new Bun.Glob("**/*.json").scanSync(directory)],pointer=JSON.parse(readFileSync(join(directory,pointerPath!),"utf8")) as {record:string};
 chmodSync(cas.objectPath(pointer.record),0o644);rmSync(cas.objectPath(pointer.record));
 expect([...await recorder.upToDate("w",new Map([["p:t",key]]))],"the record is gone").toEqual([]);
});

test("an edit nobody announced, even a whitespace one, changes the key after the graph re-verifies its files, and an unchanged tree stays up to date",async()=>{
 const scenario=(fixture.scenarios as unknown as Scenario[])[0]!,source=scratch("verify-source");
 for(const[path,text]of Object.entries(scenario.files)){mkdirSync(dirname(join(source,path)),{recursive:true});writeFileSync(join(source,path),text);}
 const world=createDiskWorldV1(source,{index:createStatIndexV1({root:scratch("verify-index")})}),graph=createDerivedGraphV1({rules:KIND_RULES_V1,world,statIndex:createStatIndexV1({root:scratch("verify-stats")}),environment:{},verifyObserved:verifyObservedV1});
 const tasks=new Map(Object.entries(scenario.tasks).map(([id,task])=>[id,{...task,declaration:scenario.declarations[id]}])),keys=()=>taskKeysV1({graph,tasks,order:scenario.order});
 const before=await keys();
 expect(await graph.verify(),"nothing was edited").toEqual([]);
 expect(await keys()).toEqual(before);
 writeFileSync(join(source,"a/src/x.ts"),scenario.files["a/src/x.ts"]+"\n");
 expect(await graph.verify(),"the edited file is found without any dirty set").toEqual(["a/src/x.ts"]);
 const after=await keys();
 expect(after.get("a:build")).not.toBe(before.get("a:build"));
 expect(after.get("a:test"),"the edit reaches the task above").not.toBe(before.get("a:test"));
 expect(after.get("b:build"),"an unrelated task keeps its key").toBe(before.get("b:build"));
});

test("the host task state runs a section under the shared lease of the graph image, which keeps an exclusive writer out and is given back when idle, and a failing section leaves the next one working",async()=>{
 const root=scratch("host-lease");mkdirSync(join(root,".cache/b/l/m/data"),{recursive:true});mkdirSync(join(root,".cache/b/g"),{recursive:true});
 const state=await createHostTaskStateV1({cas:createCasStoreV1({root:scratch("host-cas")})})({root}),signal=new AbortController().signal,resource=nativeImageSharedReadResourceV1(machinePathV1(root,".cache/b/g")),busy=()=>acquireResourceLease({directory:machinePathV1(root,".cache/b/l/m/data"),resource,mode:"exclusive",signal,timeoutMs:0});
 expect(Object.keys(state.recorder).sort()).toEqual(["record","upToDate"]);
 const answer=await state.leased(async()=>{await expect(busy()).rejects.toThrow("timed out");return await state.leased(async()=>"nested");});
 expect(answer).toBe("nested");
 await expect(state.leased(async()=>{throw new Error("section failed");})).rejects.toThrow("section failed");
 let freed=false;
 for(let at=0;at<60&&!freed;at++){try{(await busy()).release();freed=true;}catch{await new Promise(resolve=>setTimeout(resolve,50));}}
 expect(freed,"the lease is given back after the idle time").toBe(true);
 expect(await state.leased(async()=>"again")).toBe("again");
});
