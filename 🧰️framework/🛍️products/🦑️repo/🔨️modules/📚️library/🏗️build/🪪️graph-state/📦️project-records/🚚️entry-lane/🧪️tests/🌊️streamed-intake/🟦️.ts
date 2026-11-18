import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createClosureEntryLaneV1,closureEntryPolicyV1}from"../../🟦️.ts";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{libraryBootstrap}from"../../../../../🕸️graph/📥️inference/🟨️.mjs";
import{createClosureStoreV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";

/** 🌊️ Law: taking closure entries from the fleet holds one bundle at a time. A worker that took twelve entries at once unpacked and localized about 37 MB of JSON per entry beside each other (4.4 to 5.3 GB for one owner, 12 GB footprint with the compressor), so the intake of a lane is serial whatever the fetch width: no two imports overlap, each import carries one bundle, and every entry still arrives. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`intake-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),COUNT=6,TEXT="export const p=1;\n";
const recipeOf=(root:string,n:number)=>({fn:"native-target",target:{metadata:{semio:{}},options:{command:`repo "${root}/script.ts" run`,cwd:"libs/a",env:{CATALOG:`${root}/catalog`}}},projectRoot:"libs/"+n});
const tree=(root:string)=>{mkdirSync(join(root,"ce"),{recursive:true});writeFileSync(join(root,"p.ts"),TEXT);utimesSync(join(root,"p.ts"),1_700_000_000,1_700_000_000);bindClosureStoreV1(root,join(root,"ce"),true);};
const derive=async(root:string,n:number)=>{
 const recipe=recipeOf(root,n),store=createClosureStoreV1({directory:join(root,"ce")}),rows=[{kind:"file",path:`${root}/p.ts`,fact:{sha256:sha(TEXT),size:Buffer.byteLength(TEXT),mtimeMs:1,ctimeMs:1,ino:1}}];
 await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:["{workspaceRoot}/p.ts"],rows,layers:[[...rows]],sidecar:{facts:[null],members:[null]}});
 return recipe;
};

test("a lane takes entries one bundle at a time even when the fetch width is wider, and every entry arrives",async()=>{
 await libraryBootstrap;
 expect(closureEntryPolicyV1.fetchConcurrency,"the fetch is wider than one, so only the serial intake can keep the bundles apart").toBeGreaterThan(1);
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},rootA=temp("a"),rootB=temp("b");tree(rootA);tree(rootB);
  const laneA=createClosureEntryLaneV1({root:rootA,environment,owner:"a:1"})!,laneB=createClosureEntryLaneV1({root:rootB,environment,owner:"b:1"})!;
  for(let n=0;n<COUNT;n++)expect(await laneA.offer(await derive(rootA,n),closureTravelV1)).toBe(true);
  await laneA.flush();
  let active=0,widest=0,largest=0,calls=0;
  const travel={...closureTravelV1,async import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string}>[]){
   calls++;active++;widest=Math.max(widest,active);largest=Math.max(largest,bundles.length);
   try{await Bun.sleep(25);return await closureTravelV1.import(root,bundles);}finally{active--;}
  }};
  const taken=await laneB.prefetch(Array.from({length:COUNT},(_,n)=>recipeOf(rootB,n)),travel);
  expect(taken,"every entry arrives").toBe(COUNT);
  expect(calls).toBe(COUNT);
  expect(widest,"no two imports overlap").toBe(1);
  expect(largest,"one bundle per import").toBe(1);
  for(let n=0;n<COUNT;n++)expect(await createClosureStoreV1({directory:join(rootB,"ce")}).load(`${rootB}\0${closureRecipeKeyV1(recipeOf(rootB,n))}`),"entry "+n).toBeDefined();
 }finally{await server.stop();}
},120000);

test("a lane builds the documents of one entry at a time when several are offered together",async()=>{
 await libraryBootstrap;
 const cache=temp("cache2"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},root=temp("c");tree(root);
  const lane=createClosureEntryLaneV1({root,environment,owner:"c:1"})!;
  let active=0,widest=0;
  const travel={...closureTravelV1,async export(path:string,image:string){active++;widest=Math.max(widest,active);try{await Bun.sleep(25);return await closureTravelV1.export(path,image);}finally{active--;}}};
  const recipes=[];for(let n=0;n<COUNT;n++)recipes.push(await derive(root,n));
  const stored=await Promise.all(recipes.map(recipe=>lane.offer(recipe,travel)));
  await lane.flush();
  expect(stored.every(Boolean),"every entry is offered").toBe(true);
  expect(widest,"no two documents are built together").toBe(1);
 }finally{await server.stop();}
},120000);
