import {afterAll,expect,test} from "bun:test";
import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {fleetFetchContextV1,pinnedFetchV1} from "../../../🟦️.ts";
import {cargoLockRegistryCratesV1,fleetCargoRegistryAroundV1} from "../../../🦀️crates/🟦️.ts";
import {fleetUvClaimV1,fleetUvManifestNameV1} from "../../../🐍️uv/🟦️.ts";

const scratch:string[]=[],stops:Array<()=>Promise<void>>=[],TOKEN="write-token";
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`work-wire-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const stop of stops)await stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const serve=async()=>{const server=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:TOKEN}});stops.push(()=>server.stop());return server;};
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const upstream=(payload:Uint8Array<ArrayBuffer>,delayMilliseconds:number)=>{
 const served={requests:0};
 const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:async()=>{served.requests++;await new Promise(resolve=>setTimeout(resolve,delayMilliseconds));return new Response(payload);}});
 stops.push(async()=>{server.stop(true);});return {url:`http://127.0.0.1:${server.port}/tool.bin`,served};
};
const machine=(cache:string,store:string|undefined)=>fleetFetchContextV1({},directory("root"),{storeRoot:store,remote:{endpoint:cache,instance:"main",token:TOKEN,authenticated:true}});

test("nine machines download one pinned tool at once through a shared group store: the internet is asked once and every machine holds identical bytes",async()=>{
 const server=await serve(),payload=randomBytes(300000),site=upstream(payload,600),store=directory("store"),pin={url:site.url,sha256:sha(payload),bytes:payload.byteLength};
 const bodies=await Promise.all(Array.from({length:9},async()=>new Uint8Array(await (await pinnedFetchV1(machine(server.url,store),pin)).arrayBuffer())));
 expect(site.served.requests,"one download for nine machines").toBe(1);
 for(const body of bodies)expect(sha(body)).toBe(pin.sha256);
});

test("nine machines without a shared directory share the tool through the cache server: one download, eight served from the content store",async()=>{
 const server=await serve(),payload=randomBytes(200000),site=upstream(payload,600),pin={url:site.url,sha256:sha(payload),bytes:payload.byteLength};
 const bodies=await Promise.all(Array.from({length:9},async()=>new Uint8Array(await (await pinnedFetchV1(machine(server.url,undefined),pin)).arrayBuffer())));
 expect(site.served.requests).toBe(1);
 for(const body of bodies)expect(sha(body)).toBe(pin.sha256);
});

test("a pin that the upstream serves wrongly is still refused to nobody: the caller keeps verifying and the claim does not outlive the stream",async()=>{
 const server=await serve(),payload=randomBytes(1000),site=upstream(randomBytes(1000),10),store=directory("store"),pin={url:site.url,sha256:sha(payload)};
 const body=new Uint8Array(await (await pinnedFetchV1(machine(server.url,store),pin)).arrayBuffer());
 expect(sha(body)).not.toBe(pin.sha256);
 const status=await (await fetch(`${server.url}/status`,{headers:{authorization:`Bearer ${TOKEN}`}})).json() as {claims?:{active:number}};
 expect(status.claims?.active).toBe(0);
});

test("nine machines with one lockfile fetch the crate files once: the first downloads, the others restore them from the fleet",async()=>{
 const server=await serve(),store=directory("store"),project=directory("project");
 const crate=randomBytes(5000),pins=[{name:"itoa",version:"1.0.11",sha256:sha(crate)}];
 writeFileSync(join(project,"Cargo.lock"),`[[package]]\nname = "itoa"\nversion = "1.0.11"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "${pins[0]!.sha256}"\n`);
 expect(cargoLockRegistryCratesV1(readFileSync(join(project,"Cargo.lock"),"utf8"))).toEqual(pins);
 mkdirSync(join(project,"src"));writeFileSync(join(project,"Cargo.toml"),"[package]\nname=\"p\"\n");
 let downloads=0;
 await Promise.all(Array.from({length:9},async()=>{
  const home=directory("home"),context=machine(server.url,store);
  await fleetCargoRegistryAroundV1(context,{home,manifest:join(project,"Cargo.toml"),workspace:project},async()=>{
   const cache=join(home,"registry","cache","index.crates.io-1949cf8c6b5b557f"),target=join(cache,"itoa-1.0.11.crate");
   let present=true;try{readFileSync(target);}catch{present=false;}
   if(!present){downloads++;await new Promise(resolve=>setTimeout(resolve,600));mkdirSync(cache,{recursive:true});writeFileSync(target,crate);}
  });
 }));
 expect(downloads,"cargo downloaded the crate once for nine machines").toBe(1);
});

test("nine machines sync one uv lock: the first syncs and seeds the fleet, the others find the seed and sync nothing",async()=>{
 const server=await serve(),store=directory("store"),root=directory("uvroot"),lock="version = 1\n";let syncs=0;
 const manifest=join(store,"v1","fleet","uv",fleetUvManifestNameV1(createHash("sha256").update(lock).digest("hex")));
 const outcomes=await Promise.all(Array.from({length:9},async()=>{
  const context=fleetFetchContextV1({},root,{storeRoot:store,remote:{endpoint:server.url,instance:"main",token:TOKEN,authenticated:true}}),claim=await fleetUvClaimV1(context,lock);
  if(claim.outcome==="granted"){syncs++;await new Promise(resolve=>setTimeout(resolve,500));mkdirSync(join(store,"v1","fleet","uv"),{recursive:true});writeFileSync(manifest,"{}\n");await claim.release();}
  return claim.outcome;
 }));
 expect(syncs,"uv synced once for nine machines").toBe(1);expect(outcomes.filter(outcome=>outcome==="ready")).toHaveLength(8);
});
