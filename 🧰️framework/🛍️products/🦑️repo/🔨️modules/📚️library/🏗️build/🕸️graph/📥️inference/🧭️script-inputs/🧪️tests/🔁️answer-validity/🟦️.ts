/** 🔁️ Law: a stored owner record whose closure entry is stale or absent at take time is judged on the ANSWER of the closure question, not on the bytes of the closure: the question is asked again once in a walk child (here a port), the record stays valid when the answer equals the recorded one (no owner is inferred) and fails with `closure-value` when it differs; one walk serves every owner of the closure group, a walk that fails or is not offered leaves the miss as before, and a cancelled look is cancelled, not stale. The decision table equals the independent Python oracle.
 * @see ./🐍️.py
 * @see ../../🟨️.mjs `reaskClosureEntryV1` */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});

const inference=await import("../../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const scriptInputs=await import("../../🟨️.mjs") as any,observation=await import("../../../../../👁️observation/🟨️.mjs") as any;
type Scenario={name:string;before:Record<string,string>;after:Record<string,string>;queries:string[][];recorded:boolean[];now:boolean[];holds:boolean;walks:number;failure:string|null};
const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py")],{encoding:"utf8",maxBuffer:1<<24});
const scenarios=(JSON.parse(oracle.stdout) as {scenarios:Scenario[]}).scenarios;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

async function prepare(scenario:Scenario,mode:"stale"|"absent"){
 const root=await repository.scratch("answer-validity-");roots.push(root);
 write(root,scenario.before);
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,join(root,".ce"),true);
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),entry=join(root,"tools/a.mjs");
 const handle=scriptInputs.closureHandleOfV1(await api.closureOf([entry],[],"test-discovery")),recipes=[[handle.key,handle.recipe]];
 await api.resolve(recipes);
 const rows=scenario.queries.map((query,at)=>({kind:"closure",path:JSON.stringify([root,handle.key,["some",query.map(name=>"tools/"+name)]]),fact:JSON.stringify(scenario.recorded[at])}));
 for(const name of Object.keys(scenario.before))if(!(name in scenario.after))rmSync(join(root,name),{force:true});
 write(root,scenario.after);
 const later=new Date(Date.now()+5000);for(const name of Object.keys(scenario.after))if(scenario.after[name]!==scenario.before[name])utimesSync(join(root,name),later,later);
 if(mode==="absent")rmSync(join(root,".ce"),{recursive:true,force:true});
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,join(root,".ce"),true);scriptInputs.closureApiV1(scripts,root);
 const walks={count:0};
 const port=async(workspaceRoot:string,recipe:unknown)=>{walks.count++;return scriptInputs.deriveClosureRecipeV1(workspaceRoot,recipe,scripts);};
 const verify=(reask:unknown,signal?:AbortSignal)=>{
  const failures:{why?:string}[]=[];
  const holds=scriptInputs.runClosureTakeV1(()=>observation.projectRowsHoldV1(rows,{closureRecipes:{[handle.key]:handle.recipe}},()=>true,(_row:unknown,why?:string)=>failures.push({why})),signal,reask===undefined?{}:{reask});
  return holds.then((value:boolean)=>({holds:value,failures}));
 };
 return{verify,port,walks,rows};
}

test("the oracle ran and states a table with a record that holds and one that does not",()=>{
 expect(oracle.status).toBe(0);
 expect(scenarios.some(scenario=>scenario.holds)).toBe(true);
 expect(scenarios.some(scenario=>!scenario.holds)).toBe(true);
});

for(const mode of["stale","absent"] as const)for(let at=0;at<3;at++)test("a "+mode+" entry is judged on the answer (scenario "+at+")",async()=>repository.invoke(async()=>{
 const scenario=scenarios[at],site=await prepare(scenario,mode);
 await scriptInputs.withClosurePassV1(async()=>{
  const before=scriptInputs.closureEntryCountsV1(),said=await site.verify(site.port);
  expect([said.holds,said.failures]).toEqual([scenario.holds,scenario.holds?[]:said.failures]);
  expect(site.walks.count).toBe(scenario.walks);
  if(!scenario.holds)expect(said.failures.some(failure=>failure.why===scenario.failure)).toBe(true);
  const after=scriptInputs.closureEntryCountsV1();
  expect(after.reasked-before.reasked).toBe(1);
  expect(scenario.holds?after.reaskEqual-before.reaskEqual:after.reaskDifferent-before.reaskDifferent).toBeGreaterThan(0);
 });
}));

test("one walk serves every owner of the closure group, and the stored entry serves the owners that come later",async()=>repository.invoke(async()=>{
 const scenario=scenarios[0],site=await prepare(scenario,"stale");
 await scriptInputs.withClosurePassV1(async()=>{
  const owners=await Promise.all(Array.from({length:8},()=>site.verify(site.port)));
  expect(owners.every(owner=>owner.holds===scenario.holds)).toBe(true);
  expect(site.walks.count).toBe(1);
  const later=await Promise.all(Array.from({length:4},()=>site.verify(site.port)));
  expect(later.every(owner=>owner.holds===scenario.holds)).toBe(true);
  expect(site.walks.count).toBe(1);
 });
}));

test("a walk that fails or is not offered leaves the miss as before, and without the option nothing is asked",async()=>repository.invoke(async()=>{
 const scenario=scenarios[0];
 const failing=await prepare(scenario,"stale");
 await scriptInputs.withClosurePassV1(async()=>{
  const said=await failing.verify(async()=>{throw new Error("child died");});
  expect(said.holds).toBe(false);
  expect(said.failures.some(failure=>String(failure.why).startsWith("closure-entry-stale"))).toBe(true);
  expect(scriptInputs.closureEntryCountsV1().reaskFailed).toBeGreaterThan(0);
 });
 const silent=await prepare(scenario,"absent");
 await scriptInputs.withClosurePassV1(async()=>{
  const none=await silent.verify(async()=>undefined);
  expect(none.holds).toBe(false);
  expect(none.failures.some(failure=>String(failure.why).startsWith("closure-entry-absent"))).toBe(true);
  const plain=await silent.verify(undefined);
  expect(plain.holds).toBe(false);
  expect(silent.walks.count).toBe(0);
 });
}));

test("a cancelled look is cancelled, not stale",async()=>repository.invoke(async()=>{
 const site=await prepare(scenarios[0],"stale"),stop=new AbortController();
 await scriptInputs.withClosurePassV1(async()=>{
  const pending=site.verify(()=>new Promise(()=>{}),stop.signal);
  setTimeout(()=>stop.abort(new Error("cancelled")),200);
  await expect(pending).rejects.toThrow("cancelled");
 });
}));
