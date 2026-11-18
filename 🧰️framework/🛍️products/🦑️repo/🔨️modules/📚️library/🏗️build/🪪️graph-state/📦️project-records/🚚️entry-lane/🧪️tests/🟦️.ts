import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{closureEntryKeyV1,closureEntryStateV1,createClosureEntryLaneV1}from"../🟦️.ts";
import{closureKeyV1,neutralBundleV1,hostOfRootV1}from"../../🧳️portable/🟦️.ts";
import{bindClosureScriptsV1,bindClosureStoreV1,closureEntryCountsV1,closureRecipeKeyV1,closureTravelV1,prefetchClosureEntriesV1,resetClosureEntriesV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{libraryBootstrap}from"../../../../🕸️graph/📥️inference/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
import{observationRowFreshV1,settleClosureRowsV1}from"../../../../👁️observation/🟨️.mjs";

/** 🚚️ Closure entries are shared on their own: one key on every machine for equal inputs whatever the checkout path, an entry derived once is taken by every other root without a derivation, a stale stored entry is replaced by a holding one, and an edit invalidates exactly the entries whose closure holds the edited file. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`entry-lane-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const recipeOf=(root:string)=>({fn:"native-target",target:{metadata:{semio:{}},options:{command:`repo "${root}/script.ts" run`,cwd:"libs/a",env:{CATALOG:`${root}/catalog`}}},projectRoot:"libs/a"});
const CONTENT={"p.ts":"export const p=1;\n","q.ts":"export const q=2;\n","other.ts":"export const o=3;\n"};
const fileRow=(root:string,name:keyof typeof CONTENT,text:string=CONTENT[name])=>({kind:"file",path:`${root}/${name}`,fact:{sha256:sha(text),size:Buffer.byteLength(text),mtimeMs:1,ctimeMs:1,ino:1}});
const tree=(root:string,mtime:number)=>{mkdirSync(join(root,"ce"),{recursive:true});for(const[name,text]of Object.entries(CONTENT)){writeFileSync(join(root,name),text);utimesSync(join(root,name),mtime,mtime);}bindClosureStoreV1(root,join(root,"ce"),true);};
const derive=async(root:string)=>{
 const recipe=recipeOf(root),store=createClosureStoreV1({directory:join(root,"ce")}),rows=[fileRow(root,"p.ts"),fileRow(root,"q.ts")];
 await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:["{workspaceRoot}/p.ts","{workspaceRoot}/q.ts"],rows,layers:[[...rows]],sidecar:{facts:[null],members:[null]}});
 return recipe;
};
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
const entryOf=async(root:string)=>createClosureStoreV1({directory:join(root,"ce")}).load(`${root}\0${closureRecipeKeyV1(recipeOf(root))}`);

test("the key of an entry is the same on every checkout path and equals the independent digest of the recipe with the root token",()=>{
 const a="/Users/one/semio",b="/srv/build/other-root",c="C:/work/semio";
 const independent=(root:string)=>closureKeyV1(JSON.parse(JSON.stringify(recipeOf(root)).split(root).join("⟦semio-root⟧")));
 expect(closureEntryKeyV1(recipeOf(a),a)).toBe(closureEntryKeyV1(recipeOf(b),b));
 expect(closureEntryKeyV1(recipeOf(a),a)).toBe(independent(a));
 expect(closureEntryKeyV1(recipeOf(c),c),"a Windows root spelled with forward slashes names the same entry").toBe(independent(c));
 expect(closureEntryKeyV1(recipeOf(c),c)).toBe(closureEntryKeyV1(recipeOf(a),a));
 expect(closureEntryKeyV1(recipeOf(b),b)).toBe(independent(b));
 expect(closureRecipeKeyV1(recipeOf(a)),"the local key still names the root").not.toBe(closureRecipeKeyV1(recipeOf(b)));
 const neutral=neutralBundleV1({recipeKey:closureRecipeKeyV1(recipeOf(a)),bundle:{recipe:recipeOf(a),rows:[],layers:[]}},hostOfRootV1(a),new Map());
 expect(neutral?.recipeKey,"the portable layer names the same recipe the same way").toBe(closureEntryKeyV1(recipeOf(a),a));
 expect(closureEntryKeyV1({...recipeOf(a),projectRoot:"libs/b"},a)).not.toBe(closureEntryKeyV1(recipeOf(a),a));
 const state=closureEntryStateV1({root:a,recipe:recipeOf(a),platform:"darwin-arm64"}),other=closureEntryStateV1({root:b,recipe:recipeOf(b),platform:"linux-x64"});
 expect(state.digest).toBe(other.digest);
 expect(state.frame.tooling).toBe(other.frame.tooling);
});

test("a second root with other paths and other mtimes takes the entry the first root derived and derives nothing; a cold reader takes it in bulk",async()=>{
 const{server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),rootC=temp("c");tree(rootA,1_700_000_000);tree(rootB,1_800_000_000);tree(rootC,1_900_000_000);
  const recipeA=await derive(rootA),laneA=createClosureEntryLaneV1({root:rootA,environment,owner:"a:1"})!,laneB=createClosureEntryLaneV1({root:rootB,environment,owner:"b:1"})!,laneC=createClosureEntryLaneV1({root:rootC,environment,owner:"c:1"})!;
  const before=closureEntryCountsV1().derived;
  expect(await laneA.offer(recipeA,closureTravelV1)).toBe(true);await laneA.flush();expect(laneA.stats().published).toBe(1);
  const taken=await laneB.take(recipeOf(rootB),closureTravelV1);
  expect(taken.outcome).toBe("taken");
  const entry=await entryOf(rootB);
  expect(entry).toBeDefined();
  expect(JSON.stringify(entry)).not.toContain(rootA);
  expect(entry!.rows.map(row=>(row as{path:string}).path)).toEqual([`${rootB}/p.ts`,`${rootB}/q.ts`]);
  expect(entry!.rows.every(row=>observationRowFreshV1(row)),"every row holds in the second tree by content").toBe(true);
  expect(await laneC.prefetch([recipeOf(rootC)],closureTravelV1)).toBe(1);
  expect((await entryOf(rootC))?.rows.every(row=>observationRowFreshV1(row))).toBe(true);
  expect(closureEntryCountsV1().derived-before,"no derivation anywhere").toBe(0);
  expect(laneB.stats().fetched).toBe(1);expect(laneC.stats().prefetched).toBe(1);
  const missing=await createClosureEntryLaneV1({root:rootC,environment,owner:"c:2"})!.prefetch([{fn:"native-command",x:1}],closureTravelV1);
  expect(missing,"an entry nobody derived is not taken").toBe(0);
 }finally{await server.stop();}
});

test("a stored entry that no longer holds is replaced by a holding entry of the fleet and kept when the fleet's entry does not hold here",async()=>{
 const{server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),rootC=temp("c");tree(rootA,1_700_000_000);tree(rootB,1_800_000_000);tree(rootC,1_900_000_000);
  const recipeA=await derive(rootA),laneA=createClosureEntryLaneV1({root:rootA,environment,owner:"a:1"})!;
  await laneA.offer(recipeA,closureTravelV1);await laneA.flush();
  const staleRows=[fileRow(rootC,"p.ts","export const p=0;\n"),fileRow(rootC,"q.ts")],recipeC=recipeOf(rootC),storeC=createClosureStoreV1({directory:join(rootC,"ce")});
  await storeC.save(`${rootC}\0${closureRecipeKeyV1(recipeC)}`,{recipe:recipeC,value:["{workspaceRoot}/p.ts","{workspaceRoot}/q.ts"],rows:staleRows,layers:[[...staleRows]],sidecar:{facts:[null],members:[null]}});
  expect((await entryOf(rootC))!.rows.every(row=>observationRowFreshV1(row))).toBe(false);
  const laneC=createClosureEntryLaneV1({root:rootC,environment,owner:"c:1"})!;
  expect((await laneC.take(recipeC,closureTravelV1)).outcome).toBe("taken");
  expect((await entryOf(rootC))!.rows.every(row=>observationRowFreshV1(row)),"the stale entry was replaced").toBe(true);
  writeFileSync(join(rootC,"p.ts"),"export const p=99;\n");
  const before=(await entryOf(rootC))!.digests.join(",");
  expect((await laneC.take(recipeC,closureTravelV1)).outcome).toBe("taken");
  expect((await entryOf(rootC))!.digests.join(","),"a fleet entry that does not hold here leaves the stored one alone").toBe(before);
 }finally{await server.stop();}
});

test("an unrelated edit and a restamp keep every entry valid and an edit of a closure file invalidates exactly the entries that hold it",()=>{
 const root=temp("edit");tree(root,1_700_000_000);
 const entryPQ=[fileRow(root,"p.ts"),fileRow(root,"q.ts")],entryOther=[fileRow(root,"other.ts")],fresh=(rows:readonly object[])=>rows.every(row=>observationRowFreshV1(row));
 expect(fresh(entryPQ)&&fresh(entryOther)).toBe(true);
 utimesSync(join(root,"p.ts"),1_950_000_000,1_950_000_000);
 expect(fresh(entryPQ),"a restamp of identical content keeps the entry").toBe(true);
 writeFileSync(join(root,"other.ts"),"export const o=4;\n");
 expect(fresh(entryPQ),"an edit of a file outside the closure keeps the entry").toBe(true);
 expect(fresh(entryOther)).toBe(false);
 writeFileSync(join(root,"q.ts"),"export const q=3;\n");
 expect(fresh(entryPQ)).toBe(false);
});

test("a cold reader settles the closure entries of a record from the fleet with zero derivations; an entry it derives is offered, and a fleet that is off changes nothing",async()=>{
 await libraryBootstrap;
 const{server,environment}=await world(),saved={...process.env};
 try{
  const rootA=temp("a"),rootB=temp("b");tree(rootA,1_700_000_000);tree(rootB,1_800_000_000);
  const recipeA=await derive(rootA),laneA=createClosureEntryLaneV1({root:rootA,environment,owner:"a:1"})!;
  await laneA.offer(recipeA,closureTravelV1);await laneA.flush();
  Object.assign(process.env,environment);delete process.env.SEMIO_CLOSURE_FLEET;
  resetClosureEntriesV1();bindClosureScriptsV1(rootB,{});
  const key=closureRecipeKeyV1(recipeOf(rootB)),before=closureEntryCountsV1();
  await settleClosureRowsV1([{kind:"closure",path:JSON.stringify([rootB,key,["digest"]]),fact:"x"}] as never,{closureRecipes:{[key]:recipeOf(rootB)}});
  const after=closureEntryCountsV1();
  expect(after.derived-before.derived,"zero derivations").toBe(0);
  expect(after.fetched-before.fetched,"one entry taken in bulk").toBe(1);
  expect(after.loaded-before.loaded,"and loaded as a holding entry").toBe(1);
  expect(await prefetchClosureEntriesV1(rootB,[recipeOf(rootB)]),"nothing left to take").toBe(0);
  process.env.SEMIO_CLOSURE_FLEET="off";
  const rootC=temp("c");tree(rootC,1_900_000_000);
  expect(await prefetchClosureEntriesV1(rootC,[recipeOf(rootC)]),"switched off").toBe(0);
 }finally{for(const key of Object.keys(process.env))if(!(key in saved))delete process.env[key];Object.assign(process.env,saved);await server.stop();}
});
