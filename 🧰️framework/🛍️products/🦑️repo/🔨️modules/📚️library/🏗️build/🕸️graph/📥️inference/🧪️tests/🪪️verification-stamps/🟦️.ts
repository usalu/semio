/** 🪪️ Contract: an entry that one process of a machine verified in full is verified by every later process through its verification stamp: the layers are not read, not listed and not probed again while no stamped path moves, and an entry whose stamped path moved is verified in full again and (when its rows still hold) stamped anew. The model of which entries a touched directory stales is the one of the layer-ids contract (an independent python program), and the stamps of an entry are checked against a plain stat of the same paths. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
process.env.SEMIO_GRAPH_WALK_FARM="off";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
import{requireTestToolV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
const pythonTool=requireTestToolV1("python");

const inference=await import("../../🟨️.mjs") as any;
await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any,entries=await import("../../🧭️script-inputs/💾️entries/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const DIRECTORIES=40,ENTRIES=12,LAYERS_PER_ENTRY=6,TOUCHED=3;
const layerRows=(entry:number,layer:number)=>Array.from({length:10+layer},(_,at)=>(entry*3+layer*2+at)%DIRECTORIES);
const scenario=()=>({entries:Array.from({length:ENTRIES},(_,entry)=>Array.from({length:LAYERS_PER_ENTRY},(_,layer)=>layerRows(entry,layer))),touched:[TOUCHED]});
const model=()=>{
 const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("../🪪️layer-ids/🐍️.py",import.meta.url))],{stdin:Buffer.from(JSON.stringify(scenario()))});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 return JSON.parse(result.stdout.toString()) as {distinctRows:number;rowReferences:number;distinctLayers:number;entries:number;staleEntries:number;staleLayers:number};
};

const prepare=async(prefix:string)=>{
 const root=await repository.scratch(prefix),store=await repository.scratch(prefix+"store-");roots.push(root,store);
 const rows:unknown[]=[];
 for(let at=0;at<DIRECTORIES;at++){
  const directory=join(root,"d"+at);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"f"),String(at));
  const detached=await observation.observeDetachedV1(()=>observation.observedFs.readdirSync(directory));
  rows.push(detached.rows[0]);
 }
 const writer=entries.createClosureStoreV1({directory:store}),recipes:unknown[]=[];
 for(const[at,layers]of scenario().entries.entries()){
  const recipe={fn:"closure-of",entries:["e"+at],extras:[],role:"test-discovery"};recipes.push(recipe);
  await writer.save(root+"\0"+scriptInputs.closureRecipeKeyV1(recipe),{recipe,value:[],rows:[],layers:layers.map(layer=>layer.map(row=>rows[row]))});
 }
 await Bun.sleep(2300);
 return{root,store,recipes};
};

const process_=async(root:string,store:string,recipes:unknown[])=>{
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,store,true);
 const before={probes:observation.observationProbeCountsV1().readdir,layers:scriptInputs.layerVerdictCountsV1(),closure:scriptInputs.closureEntryCountsV1()};
 const owners=Array.from({length:ENTRIES/2},(_,at)=>recipes.slice(at*2,at*2+2));
 await scriptInputs.withClosurePassV1(async()=>{
  for(const owner of owners)await observation.withObservationRoundV1(async()=>{for(const recipe of owner)try{await scriptInputs.primeSharedClosuresV1(root,{},{recipes:[recipe]});}catch{}});
 });
 const layers=scriptInputs.layerVerdictCountsV1(),closure=scriptInputs.closureEntryCountsV1();
 return{probes:observation.observationProbeCountsV1().readdir-before.probes,checked:layers.checked-before.layers.checked,indexed:layers.indexed-(before.layers.indexed??0),loaded:closure.loaded-before.closure.loaded,stale:closure.stale-before.closure.stale,stamped:closure.stamped-before.closure.stamped,kept:closure.stampsKept-before.closure.stampsKept};
};

test("an entry verified by one process is verified by the next through its stamp, without reading, listing or probing its layers",async()=>repository.invoke(async()=>{
 const expected=model(),{root,store,recipes}=await prepare("stamps-warm-");
 const first=await process_(root,store,recipes),second=await process_(root,store,recipes),third=await process_(root,store,recipes);
 expect(first.loaded).toBe(expected.entries);expect(first.stamped).toBe(0);expect(first.kept).toBe(expected.entries);expect(first.probes).toBe(expected.distinctRows);
 expect(second.loaded).toBe(expected.entries);expect(second.stamped).toBe(expected.entries);expect(second.kept).toBe(0);
 expect(second.checked+second.indexed).toBe(0);expect(second.probes).toBe(0);
 expect(third.stamped).toBe(expected.entries);expect(third.checked+third.indexed+third.probes).toBe(0);
}));

test("a stamp is invalidated by the change of a path it names and exactly the entries that name it are verified in full and stamped anew",async()=>repository.invoke(async()=>{
 const expected=model(),{root,store,recipes}=await prepare("stamps-moved-");
 await process_(root,store,recipes);
 writeFileSync(join(root,"d"+TOUCHED,"added"),"x");
 const second=await process_(root,store,recipes),third=await process_(root,store,recipes);
 expect(expected.staleEntries).toBeGreaterThan(0);expect(expected.staleEntries).toBeLessThan(expected.entries);
 expect(second.stale).toBe(expected.staleEntries);expect(second.loaded).toBe(expected.entries-expected.staleEntries);
 expect(second.stamped).toBe(expected.entries-expected.staleEntries);
 expect(third.stamped).toBe(third.loaded);
}));

test("what a stamp names is what a plain stat of the same paths reports at the time of the stamp",async()=>repository.invoke(async()=>{
 const{root,store,recipes}=await prepare("stamps-oracle-");
 await process_(root,store,recipes);
 const reader=entries.createClosureStoreV1({directory:store,readOnly:true});
 const id=root+"\0"+scriptInputs.closureRecipeKeyV1(recipes[0]),stored=await reader.load(id),witness=reader.loadStamp(stored.digest);
 expect(witness.paths.length).toBeGreaterThan(0);
 for(const[at,path]of witness.paths.entries()){const status=statSync(path);expect(witness.tuples[at]).toBe(status.size+":"+status.mtimeMs+":"+status.ctimeMs+":"+status.ino+":"+status.mode);}
}));

test("a stamp over type and existence rows survives every name created beside the named ones and is stale only when a named row changes",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("stamps-typed-"),store=await repository.scratch("stamps-typed-store-");roots.push(root,store);
 const names=Array.from({length:DIRECTORIES},(_,at)=>join(root,"d"+at,"f"));
 for(const[at,name]of names.entries()){mkdirSync(join(root,"d"+at),{recursive:true});writeFileSync(name,String(at));}
 const rows:unknown[]=[];
 for(const name of names){
  rows.push((await observation.observeDetachedV1(()=>observation.observedFs.lstatSync(name))).rows[0]);
  rows.push((await observation.observeDetachedV1(()=>observation.observedFs.existsSync(join(name,"..","absent")))).rows[0]);
 }
 const writer=entries.createClosureStoreV1({directory:store}),recipe={fn:"closure-of",entries:["typed"],extras:[],role:"test-discovery"};
 await writer.save(root+"\0"+scriptInputs.closureRecipeKeyV1(recipe),{recipe,value:[],rows:[],layers:[rows]});
 await Bun.sleep(2300);
 const first=await process_(root,store,[recipe]);
 expect(first.kept).toBe(1);
 writeFileSync(join(root,"beside"),"x");writeFileSync(join(root,"d1","sibling"),"x");
 const second=await process_(root,store,[recipe]);
 expect(second.stamped).toBe(1);expect(second.stale+second.checked+second.indexed).toBe(0);
 writeFileSync(join(root,"d3","absent"),"x");
 const third=await process_(root,store,[recipe]);
 expect(third.stale).toBe(1);
}));

test("an entry taken from the fleet is stamped when it is stored, so the first process that loads it reads, lists and probes no layer",async()=>repository.invoke(async()=>{
 const{root,store,recipes}=await prepare("stamps-taken-"),target=await repository.scratch("stamps-taken-store-");roots.push(target);
 const source=entries.createClosureStoreV1({directory:store,readOnly:true}),bundles:unknown[]=[];
 for(const recipe of recipes){const key=scriptInputs.closureRecipeKeyV1(recipe);bundles.push({recipeKey:key,bundle:await source.bundle(root+"\0"+key)});}
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,target,true);scriptInputs.bindClosureScriptsV1(root,{});
 expect(await scriptInputs.closureTravelV1.import(root,bundles)).toBe(recipes.length);
 const first=await process_(root,target,recipes);
 expect(first.loaded).toBe(recipes.length);expect(first.stamped).toBe(recipes.length);expect(first.kept).toBe(0);
 expect(first.checked+first.indexed+first.probes).toBe(0);
 const reader=entries.createClosureStoreV1({directory:target,readOnly:true}),stored=await reader.load(root+"\0"+scriptInputs.closureRecipeKeyV1(recipes[0])),witness=reader.loadStamp(stored.digest);
 for(const[at,path]of witness.paths.entries()){const status=statSync(path);expect(witness.tuples[at],"the stamp equals a plain stat of the same paths").toBe(status.size+":"+status.mtimeMs+":"+status.ctimeMs+":"+status.ino+":"+status.mode);}
}));
