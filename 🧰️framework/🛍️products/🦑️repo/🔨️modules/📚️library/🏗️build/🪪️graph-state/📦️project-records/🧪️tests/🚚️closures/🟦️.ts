import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
import{providerReferenceV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🟨️.mjs";

/** 🚚️ The closure entries a project record names travel with it: a machine that takes the record from the fleet finds the entries in its own closure store, with its own paths and its own digests, and a forged bundle is refused. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`closure-travel-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const view="scheduling";
const providerRecipe={fn:"script-provider",port:"alpha-port",view},commandRecipe={fn:"native-command"};

const populate=async(root:string)=>{
 mkdirSync(join(root,"ce"),{recursive:true});
 for(const name of["e.ts","p.ts"])writeFileSync(join(root,name),`export const ${name[0]}=1;\n`);
 const row=(name:string)=>({kind:"file",path:`${root}/${name}`,fact:{sha256:sha(name),size:20,mtimeMs:1,ctimeMs:1,ino:1}});
 const store=createClosureStoreV1({directory:join(root,"ce")});
 await store.save(`${root}\0${closureRecipeKeyV1(providerRecipe)}`,{recipe:providerRecipe,value:["{workspaceRoot}/p.ts"],rows:[row("p.ts")],layers:[[row("p.ts")]],sidecar:{facts:[null],members:[null]}});
 await store.save(`${root}\0${closureRecipeKeyV1(commandRecipe)}`,{recipe:commandRecipe,value:["{workspaceRoot}/e.ts",providerReferenceV1("providers","alpha-port",view)],rows:[row("e.ts"),row("p.ts")],layers:[[row("e.ts")]],sidecar:{facts:[null],members:[null]}});
 bindClosureStoreV1(root,join(root,"ce"),true);
};

async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 return{server,environment};
}
const image=JSON.stringify({name:"alpha",closureRecipes:{[closureRecipeKeyV1(commandRecipe)]:commandRecipe}});
const infoOf=(root:string,taken:unknown[],travel:boolean):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/alpha",{name:"alpha"}]]),exportRecord:()=>({digest:sha("d"),authority:[],observations:[{kind:"file",path:`${root}/libs/alpha/src/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image}),async import(portable){taken.push(portable);return true;},...(travel?{closures:closureTravelV1}:{})});

test("the closure entries a record names are exported with the entries their values refer to, root-neutral and in recipe order",async()=>{
 const root=temp("export");await populate(root);
 const bundles=await closureTravelV1.export(root,image);
 expect(bundles.map(row=>row.recipeKey).sort()).toEqual([closureRecipeKeyV1(commandRecipe),closureRecipeKeyV1(providerRecipe)].sort());
 expect(await closureTravelV1.export(root,"not json")).toEqual([]);
 expect(await closureTravelV1.export(root,JSON.stringify({closureRecipes:{x:{fn:"native-target",target:"unknown"}}}))).toEqual([]);
});

test("a machine that takes a record from the fleet takes its closure entries with it and finds them in its own store with its own paths; a bundle that does not hash to its key is refused",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),rootC=temp("c");await populate(rootA);mkdirSync(join(rootB,"ce"),{recursive:true});bindClosureStoreV1(rootB,join(rootB,"ce"),true);mkdirSync(join(rootC,"ce"),{recursive:true});bindClosureStoreV1(rootC,join(rootC,"ce"),true);
  const a=createProjectFleetPortV1({root:rootA,environment,owner:"a:1"})!,b=createProjectFleetPortV1({root:rootB,environment,owner:"b:1"})!,c=createProjectFleetPortV1({root:rootC,environment,owner:"c:1"})!;
  const gateA=await a.gate(infoOf(rootA,[],true));expect(gateA.seeded).toBe(false);gateA.publish();await a.flush();
  expect(a.stats().published).toBe(1);
  const takenB:unknown[]=[],gateB=await b.gate(infoOf(rootB,takenB,true));
  expect(gateB.seeded).toBe(true);expect(takenB.length).toBe(1);expect(b.stats().closures,"both entries came with the record").toBe(2);
  const store=createClosureStoreV1({directory:join(rootB,"ce")}),entry=await store.bundle(`${rootB}\0${closureRecipeKeyV1(commandRecipe)}`);
  expect(entry).toBeDefined();
  expect(JSON.stringify(entry)).not.toContain(rootA);
  expect(entry!.rows.map(row=>(row as {path:string}).path)).toEqual([`${rootB}/e.ts`,`${rootB}/p.ts`]);
  expect(entry!.layers.length).toBe(1);expect((entry!.layers[0]![0] as {path:string}).path).toBe(`${rootB}/e.ts`);
  expect(entry!.value).toEqual(["{workspaceRoot}/e.ts",providerReferenceV1("providers","alpha-port",view)]);
  const provider=await store.load(`${rootB}\0${closureRecipeKeyV1(providerRecipe)}`);expect(provider?.value).toEqual(["{workspaceRoot}/p.ts"]);
  expect(await closureTravelV1.import(rootB,await closureTravelV1.export(rootA,image)),"entries that are there are kept").toBe(0);
  const forged=(await closureTravelV1.export(rootA,image)).map(row=>({recipeKey:"0".repeat(64),bundle:row.bundle}));
  expect(await closureTravelV1.import(rootC,forged),"a bundle under another key is refused").toBe(0);
  expect(await closureTravelV1.import(rootC,[{recipeKey:closureRecipeKeyV1(commandRecipe),bundle:{version:2}}]),"a bundle that is not closed is refused").toBe(0);
  const takenC:unknown[]=[],gateC=await c.gate(infoOf(rootC,takenC,false));
  expect([gateC.seeded,c.stats().closures],"a taker without a closure store takes the record alone").toEqual([true,0]);
 }finally{await server.stop();}
},60000);

test("documents of a record with closures are named by their recipe, deterministic, and a record that names no stored entry travels alone",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("da"),rootB=temp("db");mkdirSync(join(rootA,"ce"),{recursive:true});bindClosureStoreV1(rootA,join(rootA,"ce"),true);mkdirSync(join(rootB,"ce"),{recursive:true});bindClosureStoreV1(rootB,join(rootB,"ce"),true);
  const a=createProjectFleetPortV1({root:rootA,environment,owner:"a:2"})!,b=createProjectFleetPortV1({root:rootB,environment,owner:"b:2"})!;
  const gateA=await a.gate(infoOf(rootA,[],true));gateA.publish();await a.flush();
  const gateB=await b.gate(infoOf(rootB,[],true));
  expect([gateB.seeded,b.stats().closures]).toEqual([true,0]);
 }finally{await server.stop();}
},60000);
