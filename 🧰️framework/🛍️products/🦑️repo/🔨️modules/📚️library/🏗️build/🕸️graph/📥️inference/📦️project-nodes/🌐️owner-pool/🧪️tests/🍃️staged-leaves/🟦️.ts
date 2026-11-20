import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{graphFleetClientV1}from"../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{ownerPoolFleetPortV1}from"../../🔬️inference/🟨️.mjs";

/** 🧷️ A pool child infers into a staging directory and removes it after the publication: the fleet port of the child must read the leaves of the record from there, or every owner it infers on a machine whose store lacks the leaves is refused as `leaf-missing` and never reaches the fleet. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`staged-leaves-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("the fleet port of a pool child publishes a record whose leaves only its staging directory holds, and the leaves are in the fleet afterwards",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},root=temp("root"),staging=temp("staging");
  const text=["{\"version\":1,\"files\":[\"a.rs\"]}","{\"version\":1,\"manifest\":\"a/Cargo.toml\"}"],source=sha(text[0]!),lock=sha(text[1]!);
  mkdirSync(join(staging,"build-input-closures"),{recursive:true});mkdirSync(join(staging,"build-lock-closures"),{recursive:true});
  writeFileSync(join(staging,"build-input-closures",source+".json"),text[0]!);writeFileSync(join(staging,"build-lock-closures",lock+".json"),text[1]!);
  const client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!),port=ownerPoolFleetPortV1({root,environment,client,staging})!;
  const image=JSON.stringify({name:"a",tasks:[{env:`SEMIO_SOURCE_CLOSURE_${source}`},{env:`SEMIO_CARGO_LOCK_${lock}`}]});
  const answer=await port.claim({cachePath:join(root,"np","a.json"),fleet:"libs/a",keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/a",{name:"a"}]]),exportRecord:()=>({digest:sha("a"),authority:[],observations:[{kind:"file",path:`${root}/libs/a/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image}),async import(){return false;}} as never);
  expect(answer.state).toBe("granted");
  if(answer.state==="granted"){expect(await answer.publish(),answer.why()).toBe(true);}
  for(const hash of[source,lock])expect(await client.hasBlob(hash),"the staged leaf reached the fleet").toBe(true);
 }finally{await server.stop();}
});

test("the pool child hands the staging directory of its inference to the fleet port",()=>{
 const text=readFileSync(new URL("../../🔬️inference/🟨️.mjs",import.meta.url),"utf8");
 expect(text).toContain("ownerPoolFleetPortV1({ root, environment, client, staging: service.stagingDirectoryV1() })");
 expect(text).not.toContain("createProjectFleetPortV1({ root, environment, client })");
});
