import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,runTakeLookV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{directoryFactOfV1,observationLayerHoldsV1,observationRowFreshV1,observationRowsHoldV1}from"../../../../👁️observation/🟨️.mjs";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";

/** 🤝️ Law: what a pool of any machine counts as stored is found by the reader of every other machine of the same tree. A record the pool would call stored while a reader finds its closure entries changed is the defect (the pool then never refreshed it and every reader inferred the owner itself): the pool path (`claim`, `publish`) and the reader path (`gate` inside a take look, then the verdict a taker gives the closure entries it imported) run in one test against one cache server, the second test on three origins.
 * @see ../../🟦️.ts */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`pool-reader-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(bytes:Buffer|string)=>createHash("sha256").update(bytes).digest("hex");
const commandRecipe={fn:"native-command"},image=JSON.stringify({name:"alpha",closureRecipes:{[closureRecipeKeyV1(commandRecipe)]:commandRecipe}});
const rowOf=(root:string,name:string)=>{const path=`${root}/${name}`,status=lstatSync(path);return{kind:"file",path,fact:{sha256:sha(readFileSync(path)),size:status.size,mtimeMs:status.mtimeMs,ctimeMs:status.ctimeMs,ino:status.ino}};};

const tree=(root:string,module:string)=>{mkdirSync(join(root,"ce"),{recursive:true});mkdirSync(join(root,"libs","alpha"),{recursive:true});writeFileSync(join(root,"libs","alpha","a.ts"),"export const a=1;\n");writeFileSync(join(root,"e.ts"),module);bindClosureStoreV1(root,join(root,"ce"),true);};
const walk=async(root:string)=>{
 const store=createClosureStoreV1({directory:join(root,"ce")}),row=rowOf(root,"e.ts");
 await store.save(`${root}\0${closureRecipeKeyV1(commandRecipe)}`,{recipe:commandRecipe,value:["{workspaceRoot}/e.ts"],rows:[row],layers:[[row]],sidecar:{facts:[null],members:[null]}});
};
const edit=async(roots:readonly string[],module:string)=>{for(const root of roots){writeFileSync(join(root,"e.ts"),module);rmSync(join(root,".cache"),{recursive:true,force:true});}};
const infoOf=(root:string):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/alpha",{name:"alpha"}]]),exportRecord:()=>({digest:sha("d"),authority:[],observations:[rowOf(root,"libs/alpha/a.ts")],image}),async import(){return true;},closures:closureTravelV1});

/** 🔍️ The reader path: a take look at the fleet cache for the record, then the verdict a taker gives the closure entries the look imported into its store (the rows of the entry and of its layers probed against the tree). */
const readerFinds=async(root:string,environment:Record<string,string>):Promise<boolean>=>{
 const port=createProjectFleetPortV1({root,environment,owner:`reader:${process.pid}:${Math.random()}`})!;
 const gate=await runTakeLookV1(()=>port.gate(infoOf(root)));
 if(!gate.seeded)return false;
 const entry=await createClosureStoreV1({directory:join(root,"ce")}).bundle(`${root}\0${closureRecipeKeyV1(commandRecipe)}`);
 return entry!==undefined&&observationRowsHoldV1(entry.rows)&&[...entry.layers].every(layer=>observationLayerHoldsV1(layer));
};
const later=()=>Date.now()+3600_000;
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}

test("a record the pool stored is found by the reader while the tree holds, and once a file its closure walked changed the pool no longer calls it stored and the refreshed record is found again",async()=>{
 const {server,environment}=await world();
 try{
  const pool=temp("pool"),reader=temp("reader");tree(pool,"export const e=1;\n");tree(reader,"export const e=1;\n");await walk(pool);
  const first=createProjectFleetPortV1({root:pool,environment,owner:"pool:1"})!,claim=await first.claim(infoOf(pool));
  expect(claim.state).toBe("granted");expect(await (claim as {publish():Promise<boolean>}).publish()).toBe(true);await first.flush();
  expect(await readerFinds(reader,environment),"fresh tree: the reader finds what the pool stored").toBe(true);
  const quiet=createProjectFleetPortV1({root:pool,environment,owner:"pool:2",now:later})!;
  expect((await quiet.claim(infoOf(pool))).state,"fresh tree: stored").toBe("stored");
  await edit([pool,reader],"export const e=2;\n");
  expect(await readerFinds(reader,environment),"a file of the closure changed: the reader cannot take the record").toBe(false);
  const second=createProjectFleetPortV1({root:pool,environment,owner:"pool:3",now:later})!,again=await second.claim(infoOf(pool));
  expect(again.state,"the pool must not call a record stored that a reader rejects").toBe("granted");
  await walk(pool);expect(await (again as {publish():Promise<boolean>}).publish()).toBe(true);await second.flush();
  expect(await readerFinds(reader,environment),"the refreshed record is found").toBe(true);
  expect((await createProjectFleetPortV1({root:pool,environment,owner:"pool:4",now:later})!.claim(infoOf(pool))).state).toBe("stored");
 }finally{await server.stop();}
},120000);

test("three origins: what the pool of one origin stored is found by the readers of the other two, and a pool of another origin counts it stored exactly while they do",async()=>{
 const {server,environment}=await world();
 try{
  const [one,two,three]=["one","two","three"].map(label=>{const root=temp(label);tree(root,"export const e=1;\n");return root;}) as [string,string,string];await walk(one);
  const origin=createProjectFleetPortV1({root:one,environment,owner:"one:1"})!,claim=await origin.claim(infoOf(one));
  expect(claim.state).toBe("granted");expect(await (claim as {publish():Promise<boolean>}).publish()).toBe(true);await origin.flush();
  expect([await readerFinds(two,environment),await readerFinds(three,environment)]).toEqual([true,true]);
  expect((await createProjectFleetPortV1({root:two,environment,owner:"two:1",now:later})!.claim(infoOf(two))).state).toBe("stored");
  await edit([one,two,three],"export const e=3;\n");
  expect([await readerFinds(two,environment),await readerFinds(three,environment)]).toEqual([false,false]);
  expect((await createProjectFleetPortV1({root:three,environment,owner:"three:1",now:later})!.claim(infoOf(three))).state,"a pool of the third origin refreshes what both readers reject").toBe("granted");
 }finally{await server.stop();}
},120000);

test("a record whose directory listing changed is not stored for the pool either: the taker rejects it on the listing, so the pool refreshes it",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("listing");tree(root,"export const e=1;\n");await walk(root);
  const dir=join(root,"libs","alpha"),listing={kind:"readdirNames",path:dir,fact:directoryFactOfV1("readdirNames",dir)},info={...infoOf(root),exportRecord:()=>({digest:sha("d"),authority:[],observations:[rowOf(root,"libs/alpha/a.ts"),listing],image})};
  const first=createProjectFleetPortV1({root,environment,owner:"list:1"})!,claim=await first.claim(info);
  expect(claim.state).toBe("granted");expect(await (claim as {publish():Promise<boolean>}).publish()).toBe(true);await first.flush();
  expect(observationRowFreshV1(listing),"the listing holds").toBe(true);
  expect((await createProjectFleetPortV1({root,environment,owner:"list:2",now:later})!.claim(info)).state,"nothing changed: stored").toBe("stored");
  writeFileSync(join(dir,"machine-local.rs"),"fn main(){}\n");rmSync(join(root,".cache"),{recursive:true,force:true});
  expect(observationRowFreshV1(listing),"a file appeared: the taker rejects the row").toBe(false);
  const again=await createProjectFleetPortV1({root,environment,owner:"list:3",now:later})!.claim(info);
  expect(again.state,"the pool must not call stored what a taker rejects on a listing").toBe("granted");
  await (again as {abandon():Promise<void>}).abandon();
 }finally{await server.stop();}
},120000);
