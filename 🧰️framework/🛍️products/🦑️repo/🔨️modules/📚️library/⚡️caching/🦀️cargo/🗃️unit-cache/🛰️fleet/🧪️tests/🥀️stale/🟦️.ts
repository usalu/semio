import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {readRemoteCacheMetaV1} from "../../../../../../🏗️build/🌐️remote-cache/🏷️retention/🟦️.ts";
import {newestEntryV1,publishUnitV1} from "../../../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../📡️client/🟦️.ts";
import {installFleetUnitV1} from "../../📥️adopt/🟦️.ts";
import {readBuildCountersV1} from "../../📊️counters/🟦️.ts";
import {drainUploadsV1,enqueueUploadV1,type SpoolJobV1} from "../../📤️uploader/🟦️.ts";
import {portableRootsV1} from "../../🪪️identity/🟦️.ts";

/** 🥀️ The whole path of a declaration: the unit cache records what a unit was built from, the uploader carries it in the manifest, the server judges it against its tree when the unit arrives and retires it when the tree moves on, and a worker that installs the unit keeps it. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[],sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),`stale-e2e-${label}-`)));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const platform={version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""} as const;

test("a unit built from the current tree is stored with its declaration and installs with it; a unit built from an older tree is refused as stale-on-arrival; when the tree moves on the stored unit is retired",async()=>{
 const tree=directory("tree"),cas=directory("cas"),root=directory("cache"),other=directory("other-cache"),previous=process.env.SEMIO_CAS_ROOT;process.env.SEMIO_CAS_ROOT=cas;
 try{
  mkdirSync(join(tree,"core","src"),{recursive:true});writeFileSync(join(tree,"core","src","lib.rs"),"pub fn id()->u32{1}\n");
  const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"},graceMilliseconds:50,staleness:{mode:"enforce",workspace:tree,recheckSeconds:0,manifestNames:["unit.json"],examples:3}});servers.push(server);
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:tree};
  const build=(label:string,key:string,localKey:string)=>{
   const out=directory("out-"+label);writeFileSync(join(out,"libcore.rlib"),"rlib "+label);writeFileSync(join(out,"core.d"),`${out}/core.d: core/src/lib.rs\n`);
   const entry=publishUnitV1(root,{key:localKey,crate:"core",cwd:tree,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore.rlib"),join(out,"core.d")],stdout:"",stderr:"",stampDirectory:undefined,retention:{workspace:tree,externs:[sha("blob:dep")],toolchain:"rustc test"}});
   expect(entry?.retention?.sources).toEqual([{path:"core/src/lib.rs",sha256:sha("pub fn id()->u32{1}\n")}]);
   const job:SpoolJobV1={version:1,key,localKey,slot:"2".repeat(64),crate:"core",cwd:tree,workspace:tree,laneRoot:null,owner:"t:1:aa",claimed:false,costMs:1,platform,queuedMs:Date.now()};
   expect(enqueueUploadV1(root,job)).toBe(true);
  };
  build("fresh","a".repeat(64),"b".repeat(64));
  expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:1,failed:0});
  const meta=await readRemoteCacheMetaV1(server.store.root,`main/ac/aa/${"a".repeat(64)}`);
  expect(meta?.declaration).toMatchObject({version:1,kind:"rust-unit",sources:[{path:"core/src/lib.rs",sha256:sha("pub fn id()->u32{1}\n")}],externs:[sha("blob:dep")]});expect(meta?.declaration?.toolchain).toBe(sha("rustc test"));
  const client=createFleetClientV1(readFleetSettingsV1(other,environment)!);
  expect(await installFleetUnitV1({client,root:other,localKey:"c".repeat(64),key:"a".repeat(64),slot:"2".repeat(64),platform,roots:portableRootsV1({cwd:tree,workspace:tree})}),"another machine installs it").toBe("hit");
  expect(newestEntryV1(other,"c".repeat(64))?.retention,"and keeps what it was built from").toEqual(newestEntryV1(root,"b".repeat(64))?.retention);
  build("older","d".repeat(64),"e".repeat(64));
  writeFileSync(join(tree,"core","src","lib.rs"),"pub fn id()->u32{2}\n");
  expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:1});
  expect(readBuildCountersV1(tree).skipReasons).toMatchObject({"stale-on-arrival":1});
  expect(await client.hasResult("d".repeat(64)),"nothing was stored").toBe(false);
  await server.store.collect();
  expect(server.store.stalenessStatus()?.refused).toEqual({"stale-on-arrival":1});
  expect(await client.hasResult("a".repeat(64)),"the unit of the old tree is retired").toBe(false);
  expect(JSON.parse(readFileSync(join(server.store.root,".retention","sweeps.jsonl"),"utf8").trim().split("\n").pop()!)).toMatchObject({mode:"enforce",retired:1,stale:{edited:1}});
 }finally{if(previous===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previous;}
},120000);
