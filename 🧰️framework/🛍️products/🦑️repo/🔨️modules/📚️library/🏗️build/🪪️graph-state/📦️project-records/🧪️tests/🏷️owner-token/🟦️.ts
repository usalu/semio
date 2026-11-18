import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
import{PROVIDER_CLOSURE_POLICY_V1,providerReferenceV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🟨️.mjs";

/** 🏷️ Contract: a closure entry that travels with a project record names the provider of the producer's analyses by a token, and the taker puts in the provider of its own machine whether or not the record names one itself; an entry stored with the token never holds and every consumer derives it again. @see ../../🧳️portable/🟦️.ts */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`closure-travel-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const view=Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)[0]!;
const providerRecipe={fn:"script-provider",port:"alpha-port",view},commandRecipe={fn:"native-command"};

async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 return{server,environment};
}
const image=JSON.stringify({name:"alpha",closureRecipes:{[closureRecipeKeyV1(commandRecipe)]:commandRecipe}});
const infoOf=(root:string,taken:unknown[],travel:boolean):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/alpha",{name:"alpha"}]]),exportRecord:()=>({digest:sha("d"),authority:[],observations:[{kind:"file",path:`${root}/libs/alpha/src/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image}),async import(portable){taken.push(portable);return true;},...(travel?{closures:closureTravelV1}:{})});

test("a closure entry whose analysis facts name the provider of the producer is taken with the provider of this machine, even when the record itself names none",async()=>{
 const {server,environment}=await world(),prefix="build:script-provider-ports:v1:",producer=prefix+"a".repeat(64),taker=prefix+"b".repeat(64);
 try{
  const rootA=temp("pa"),rootB=temp("pb");mkdirSync(join(rootA,"ce"),{recursive:true});mkdirSync(join(rootB,"ce"),{recursive:true});writeFileSync(join(rootA,"e.ts"),"export const e=1;\n");
  const analysis={kind:"analysis",path:`${rootA}/e.ts`,fact:{provider:producer,revision:"r".repeat(64),source:{size:20,mtimeMs:0,ctimeMs:0,ino:0,sha256:sha("e.ts")},resultSha256:sha("result"),nestedObservations:[]}};
  const storeA=createClosureStoreV1({directory:join(rootA,"ce")});
  await storeA.save(`${rootA}\0${closureRecipeKeyV1(commandRecipe)}`,{recipe:commandRecipe,value:["{workspaceRoot}/e.ts"],rows:[],layers:[[analysis]],sidecar:{facts:[null],members:[null]}});
  bindClosureStoreV1(rootA,join(rootA,"ce"),true);bindClosureStoreV1(rootB,join(rootB,"ce"),true);
  const a=createProjectFleetPortV1({root:rootA,environment,owner:"a:3",providers:async()=>[producer]})!,b=createProjectFleetPortV1({root:rootB,environment,owner:"b:3",providers:async()=>[taker]})!;
  const gateA=await a.gate(infoOf(rootA,[],true));gateA.publish();await a.flush();
  const gateB=await b.gate(infoOf(rootB,[],true));expect(gateB.seeded).toBe(true);expect(b.stats().closures).toBe(1);
  const entry=await createClosureStoreV1({directory:join(rootB,"ce")}).bundle(`${rootB}\0${closureRecipeKeyV1(commandRecipe)}`);
  expect(entry).toBeDefined();
  const rows=(entry!.layers as {kind:string;fact:{provider:string}}[][]).flat().filter(row=>row.kind==="analysis");
  expect(rows.map(row=>row.fact.provider),"the provider of the producer is replaced by the one of this machine").toEqual([taker]);
  expect(JSON.stringify(entry)).not.toContain("⟦analysis-owner⟧");
 }finally{await server.stop();}
},60000);
