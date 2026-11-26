import{afterAll,expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{closureEntryKeyV1,closureEntryPolicyV1,createClosureEntryLaneV1,forgetPackedClosureDocumentsV1}from"../../🟦️.ts";
import{brotliDecompressSync}from"node:zlib";
import{hostOfRootV1,packBundleV1}from"../../../🧳️portable/🟦️.ts";
import{providerGroupNameV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🟨️.mjs";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";

/** 📤️ An owner never waits for the offer of its entry: `enqueue` returns at once, the queue is bounded and deduplicated by entry key, the background drain stores the entry, and a job the process did not finish is adopted from the outbox by the next process of the machine. The model of the queue is a SQLite table (third-party oracle).
 * @see ../../🟦️.ts */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`offer-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const recipeOf=(root:string,name="a")=>({fn:"native-target",target:{metadata:{semio:{}},options:{command:`repo "${root}/script.ts" ${name}`,cwd:"libs/a"}},projectRoot:"libs/a"});
const row=(root:string)=>{const text="export const p=1;\n";return{kind:"file",path:`${root}/p.ts`,fact:{sha256:sha(text),size:Buffer.byteLength(text),mtimeMs:1,ctimeMs:1,ino:1}};};
const derive=async(root:string,name:string)=>{
 mkdirSync(join(root,"ce"),{recursive:true});writeFileSync(join(root,"p.ts"),"export const p=1;\n");utimesSync(join(root,"p.ts"),1_700_000_000,1_700_000_000);bindClosureStoreV1(root,join(root,"ce"),true);
 const recipe=recipeOf(root,name),store=createClosureStoreV1({directory:join(root,"ce")}),rows=[row(root)];
 await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:["{workspaceRoot}/p.ts"],rows,layers:[[...rows]],sidecar:{facts:[null],members:[null]}});
 return recipe;
};
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
const deadPid=()=>{const child=spawnSync(process.execPath,["-e",""]);return child.pid!;};

test("enqueue answers before anything is stored, deduplicates by entry key and refuses beyond the bound, as the SQLite model of the queue says",async()=>{
 const{server,environment}=await world();
 try{
  const root=temp("a"),recipe=await derive(root,"a"),lane=createClosureEntryLaneV1({root,environment,owner:"a:1",outbox:join(root,"o")})!;
  const answers:string[]=[],db=new Database(":memory:");db.exec("create table jobs(key text)");
  answers.push(lane.enqueue(recipe,closureTravelV1));
  expect(lane.stats().published,"nothing is stored when enqueue returns").toBe(0);
  const limit=closureEntryPolicyV1.offerQueue;
  for(let at=0;at<limit+40;at++)answers.push(lane.enqueue(recipeOf(root,`n${at}`),closureTravelV1));
  answers.push(lane.enqueue(recipeOf(root,"n3"),closureTravelV1));
  for(const[at,answer]of answers.entries())if(answer==="queued")db.query("insert into jobs values(?)").run(String(at));
  const queued=answers.filter(answer=>answer==="queued").length;
  expect(db.query("select count(*) as n from jobs").get()).toEqual({n:queued});
  expect(queued,"one job runs at once, the queue holds at most the bound").toBe(limit+1);
  expect(answers.filter(answer=>answer==="full").length).toBe(1+40-1+0);
  expect(answers.at(-1)).toBe("duplicate");
  expect(lane.progress().queued).toBeLessThanOrEqual(limit);
  await lane.flush();
  expect(lane.progress()).toMatchObject({queued:0,offering:false,dropped:40});
  expect(lane.stats().published,"the entry that exists is stored; the others have no stored entry and are refused").toBe(1);
  expect(existsSync(join(root,"o"))?readdirSync(join(root,"o")):[],"every finished job leaves the outbox").toEqual([]);
 }finally{await server.stop();}
});

test("a job of a process that ended is adopted from the outbox by the next process and stored; a live process's job is left alone",async()=>{
 const{server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),recipeA=await derive(rootA,"a"),outbox=join(rootA,"o");mkdirSync(outbox,{recursive:true});
  const key=closureEntryKeyV1(recipeA,rootA),dead=deadPid();
  writeFileSync(join(outbox,`${key}.${dead}.json`),JSON.stringify({recipe:recipeA}));
  writeFileSync(join(outbox,`${sha("live")}.${process.ppid}.json`),JSON.stringify({recipe:recipeOf(rootA,"live")}));
  const lane=createClosureEntryLaneV1({root:rootA,environment,owner:"a:2",outbox})!;
  expect(lane.adopt(closureTravelV1)).toBe(1);
  await lane.flush();
  expect(lane.stats().published).toBe(1);expect(lane.progress().adopted).toBe(1);
  expect(readdirSync(outbox),"only the live process's job remains").toEqual([`${sha("live")}.${process.ppid}.json`]);
  const laneB=createClosureEntryLaneV1({root:rootB,environment,owner:"b:1"})!;
  mkdirSync(join(rootB,"ce"),{recursive:true});writeFileSync(join(rootB,"p.ts"),"export const p=1;\n");utimesSync(join(rootB,"p.ts"),1_800_000_000,1_800_000_000);bindClosureStoreV1(rootB,join(rootB,"ce"),true);
  expect((await laneB.take(recipeOf(rootB,"a"),closureTravelV1)).outcome,"another machine takes what the adopted job stored").toBe("taken");
 }finally{await server.stop();}
});

test("cancel drops the queued jobs, keeps their outbox files for adoption and lets the queue work again",async()=>{
 const{server,environment}=await world();
 try{
  const root=temp("a"),recipe=await derive(root,"a"),outbox=join(root,"o"),lane=createClosureEntryLaneV1({root,environment,owner:"a:3",outbox})!;
  lane.enqueue(recipeOf(root,"x1"),closureTravelV1);lane.enqueue(recipeOf(root,"x2"),closureTravelV1);lane.enqueue(recipeOf(root,"x3"),closureTravelV1);
  expect(lane.cancel()).toBe(2);
  expect(lane.progress().queued).toBe(0);
  await lane.flush();
  expect(readdirSync(outbox).length,"the cancelled jobs stay adoptable").toBe(2);
  expect(lane.enqueue(recipe,closureTravelV1)).toBe("queued");
  await lane.flush();
  expect(lane.stats().published).toBe(1);
 }finally{await server.stop();}
});

test("a shared entry an offer refers to is packed once per entry version: the next offers of the same process and of another process take its document from the memory or from the file, which equals a fresh pack",async()=>{
 const{server,environment}=await world();
 try{
  const root=temp("a"),documents=join(root,"d"),view="scheduling",shared={fn:"script-provider",port:"alpha",view},reference={projects:["@semio-tech/p"],input:providerGroupNameV1("alpha",view)};
  mkdirSync(join(root,"ce"),{recursive:true});writeFileSync(join(root,"p.ts"),"export const p=1;\n");utimesSync(join(root,"p.ts"),1_700_000_000,1_700_000_000);bindClosureStoreV1(root,join(root,"ce"),true);
  const store=createClosureStoreV1({directory:join(root,"ce")}),save=async(recipe:object,value:unknown)=>{const rows=[row(root)];await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value,rows,layers:[[...rows]],sidecar:{facts:[null],members:[null]}});};
  const first=recipeOf(root,"h1"),second=recipeOf(root,"h2");
  await save(shared,["{workspaceRoot}/p.ts"]);await save(first,[reference]);await save(second,[reference]);
  const lane=createClosureEntryLaneV1({root,environment,owner:"a:4",documents})!;
  expect(await lane.offer(first,closureTravelV1)).toBe(true);expect(lane.stats().reused).toBe(0);
  expect(await lane.offer(second,closureTravelV1)).toBe(true);expect(lane.stats().reused,"the shared entry is not packed again").toBe(1);
  const files=readdirSync(documents);expect(files.length).toBe(1);
  forgetPackedClosureDocumentsV1();
  const other=createClosureEntryLaneV1({root,environment,owner:"a:5",documents})!;
  expect(await other.offer(first,closureTravelV1)).toBe(true);expect(other.stats().reused,"another process reads the file").toBe(1);
  const bundle=(await closureTravelV1.export(root,JSON.stringify({closureRecipes:{[closureRecipeKeyV1(shared)]:shared}}))).find(entry=>entry.recipeKey===closureRecipeKeyV1(shared))!;
  const host=hostOfRootV1(root),fresh=packBundleV1(bundle as never,host,new Map([[closureRecipeKeyV1(shared),closureEntryKeyV1(shared,root)]]))!;
  const text=brotliDecompressSync(readFileSync(join(documents,files[0]!))),cut=text.indexOf(10);
  expect(text.subarray(cut+1).toString("utf8"),"the remembered document equals a fresh pack").toBe(JSON.stringify(fresh.document));
  await store.save(`${root}\0${closureRecipeKeyV1(shared)}`,{recipe:shared,value:["{workspaceRoot}/p.ts","{workspaceRoot}/p.ts"],rows:[row(root)],layers:[[row(root)]],sidecar:{facts:[null],members:[null]}});
  expect(await other.offer(second,closureTravelV1)).toBe(true);expect(other.stats().reused,"a rewritten shared entry is packed again").toBe(1);
  expect(readdirSync(documents).length,"the older version of its file is gone").toBe(1);
 }finally{await server.stop();}
});
