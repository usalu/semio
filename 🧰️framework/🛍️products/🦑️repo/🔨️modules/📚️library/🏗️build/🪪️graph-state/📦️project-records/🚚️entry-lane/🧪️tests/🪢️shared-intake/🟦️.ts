/** 🪢️ Contract: a closure document is fetched, parsed and unpacked at most once per machine. The shared entries an entry names travel inside every entry that names them, each time with other bytes (the facts the publisher walked), so a taker that already holds a shared entry that still holds here neither downloads nor imports it again whatever the bytes; two lanes of one store that take the same entry together import each document once (the second waits for the first); and an intake parses each document once, where it parsed it twice (once to name the keys, once to localize).
 * @see ../../🟦️.ts `createClosureEntryLaneV1` */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../🧶️nx/🟦️.ts";
import{createClosureEntryLaneV1}from"../../🟦️.ts";
import{bindClosureScriptsV1,bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{libraryBootstrap}from"../../../../../🕸️graph/📥️inference/🟨️.mjs";
import{createClosureStoreV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`shared-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),TEXT="export const p=1;\n";
const recipeOf=(root:string,name:string)=>({fn:"native-target",target:{metadata:{semio:{}},options:{command:`repo "${root}/script.ts" run`,cwd:"libs/"+name,env:{CATALOG:`${root}/catalog`}}},projectRoot:"libs/"+name});
const tree=(root:string)=>{mkdirSync(join(root,"ce"),{recursive:true});writeFileSync(join(root,"p.ts"),TEXT);utimesSync(join(root,"p.ts"),1_700_000_000,1_700_000_000);bindClosureStoreV1(root,join(root,"ce"),true);bindClosureScriptsV1(root,{});};
const FACT="f".repeat(300000);
const derive=async(root:string,name:string)=>{
 const recipe=recipeOf(root,name),store=createClosureStoreV1({directory:join(root,"ce")}),rows=[{kind:"file",path:`${root}/p.ts`,fact:{sha256:sha(TEXT),size:Buffer.byteLength(TEXT),mtimeMs:1,ctimeMs:1,ino:1}}];
 await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:["{workspaceRoot}/p.ts"],rows,layers:[[...rows]],sidecar:{facts:[null],members:[null]}});
 return recipe;
};
const withShared=(shared:unknown,variant:number)=>({...closureTravelV1,async export(root:string,image:string){
 const own=await closureTravelV1.export(root,image),extra=await closureTravelV1.export(root,JSON.stringify({closureRecipes:{s:shared}}));
 return[...own,...extra.map(row=>({...row,bundle:{...(row.bundle as object),layers:(row.bundle as{layers:unknown}).layers,sidecar:{facts:[{walked:variant+FACT}],members:[null]}}}))];
}});
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
const counted=(root:string,environment:Record<string,string>)=>{
 const blobs:string[]=[],base=remoteCacheFetchV1(environment,{root}),fetchOf=((input:unknown,init?:{method?:string})=>{const url=String(typeof input==="string"?input:(input as{url?:string}).url??input);if((init?.method??"GET")==="GET"&&/\/cas\//u.test(url))blobs.push(url);return(base as unknown as(input:unknown,init?:unknown)=>Promise<Response>)(input,init);});
 const client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:fetchOf as never});
 return{client,blobs};
};
const tracked=(travel:typeof closureTravelV1)=>{const imported:string[][]=[];return{imported,travel:{...travel,async import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string;neutral?:string}>[]){imported.push(bundles.map(row=>row.recipeKey));return travel.import(root,bundles as never);}}};};
const bigParses=()=>{const real=JSON.parse,sizes:number[]=[];JSON.parse=((text:string,...rest:unknown[])=>{if(typeof text==="string"&&text.startsWith('{"version":2,"recipe":'))sizes.push(text.length);return(real as(...args:unknown[])=>unknown)(text,...rest);})as typeof JSON.parse;return{sizes,stop:()=>{JSON.parse=real;}};};

async function published(environment:Record<string,string>){
 await libraryBootstrap;
 const rootA=temp("a");tree(rootA);
 const e1=await derive(rootA,"one"),e2=await derive(rootA,"two"),shared=await derive(rootA,"shared"),laneA=createClosureEntryLaneV1({root:rootA,environment,owner:"a:1"})!;
 expect(await laneA.offer(e1,withShared(shared,1))).toBe(true);expect(await laneA.offer(e2,withShared(shared,2))).toBe(true);await laneA.flush();
 return{e1,e2,shared};
}

test("a shared entry that the store holds and that still holds is neither downloaded nor imported again when another entry brings it with other bytes",async()=>{
 const{server,environment}=await world();
 try{
  const made=await published(environment),rootB=temp("b");tree(rootB);
  const{client,blobs}=counted(rootB,environment),lane=createClosureEntryLaneV1({root:rootB,environment,client,owner:"b:1"})!,watch=tracked(closureTravelV1);
  expect((await lane.take(recipeOf(rootB,"one"),watch.travel)).outcome).toBe("taken");
  const first={blobs:blobs.length,imports:watch.imported.map(row=>row.length)};
  expect(first.imports,"the first entry brings its own document and the shared one").toEqual([1,1]);
  blobs.length=0;watch.imported.length=0;
  expect((await lane.take(recipeOf(rootB,"two"),watch.travel)).outcome).toBe("taken");
  expect(watch.imported.flat().length,"only the document of the second entry is imported").toBe(1);
  expect(blobs.length,"only the document of the second entry is downloaded").toBeLessThan(first.blobs);
  const store=createClosureStoreV1({directory:join(rootB,"ce")});
  for(const name of["one","two","shared"])expect(await store.load(`${rootB}\0${closureRecipeKeyV1(recipeOf(rootB,name))}`),name).toBeDefined();
  expect(made.shared).toBeDefined();
 }finally{await server.stop();}
},120000);

test("two lanes of one store that take the same entry together import each document once",async()=>{
 const{server,environment}=await world();
 try{
  await published(environment);
  const rootB=temp("b");tree(rootB);
  const one=createClosureEntryLaneV1({root:rootB,environment,owner:"b:1"})!,two=createClosureEntryLaneV1({root:rootB,environment,owner:"b:2"})!,watch=tracked(closureTravelV1);
  const outcomes=await Promise.all([one.take(recipeOf(rootB,"one"),watch.travel),two.take(recipeOf(rootB,"one"),watch.travel)]);
  expect(outcomes.map(row=>row.outcome)).toEqual(["taken","taken"]);
  expect(watch.imported.flat().length,"the entry and its shared entry, once each").toBe(2);
  const store=createClosureStoreV1({directory:join(rootB,"ce")});
  expect(await store.load(`${rootB}\0${closureRecipeKeyV1(recipeOf(rootB,"one"))}`)).toBeDefined();
 }finally{await server.stop();}
},120000);

test("an intake parses each document once",async()=>{
 const{server,environment}=await world();
 try{
  await published(environment);
  const rootB=temp("b");tree(rootB);
  const lane=createClosureEntryLaneV1({root:rootB,environment,owner:"b:1"})!,spy=bigParses();
  try{expect((await lane.take(recipeOf(rootB,"one"),closureTravelV1)).outcome).toBe("taken");}finally{spy.stop();}
  expect(spy.sizes.length,"the entry document and the shared document, one parse each").toBe(2);
 }finally{await server.stop();}
},120000);

test("a fleet entry that does not hold here is not stored, and its shared document is not taken again while the refusal is fresh",async()=>{
 const{server,environment}=await world();
 try{
  await published(environment);
  const rootB=temp("b");tree(rootB);writeFileSync(join(rootB,"p.ts"),"export const p=2;\n");
  const{client,blobs}=counted(rootB,environment),lane=createClosureEntryLaneV1({root:rootB,environment,client,owner:"b:1"})!,watch=tracked(closureTravelV1);
  expect((await lane.take(recipeOf(rootB,"one"),watch.travel)).outcome).toBe("taken");
  const store=createClosureStoreV1({directory:join(rootB,"ce")}),idOf=(name:string)=>`${rootB}\0${closureRecipeKeyV1(recipeOf(rootB,name))}`;
  expect(await store.load(idOf("one")),"an entry that does not hold is not stored").toBeUndefined();
  const first=blobs.length;blobs.length=0;watch.imported.length=0;
  expect((await lane.take(recipeOf(rootB,"two"),watch.travel)).outcome).toBe("taken");
  expect(watch.imported.flat().length,"only the document of the second entry is looked at").toBe(1);
  expect(blobs.length).toBeLessThan(first);
  expect(await store.load(idOf("two"))).toBeUndefined();
 }finally{await server.stop();}
},120000);
