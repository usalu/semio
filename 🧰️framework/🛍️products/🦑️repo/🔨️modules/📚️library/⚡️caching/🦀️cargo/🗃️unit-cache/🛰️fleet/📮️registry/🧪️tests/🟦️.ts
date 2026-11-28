import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {sweepSpoolsV1} from "../../../../../../🏗️build/🌐️remote-execution/🛰️sidecar/📮️spool/🟦️.ts";
import {wrapRustcV1} from "../../../📜️script.ts";
import {readLookupsV1} from "../../../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../📡️client/🟦️.ts";
import {readBuildCountersV1} from "../../📊️counters/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
import {enqueueUploadV1,type SpoolJobV1} from "../../📤️uploader/🟦️.ts";
import {spoolLengthV1} from "../../📏️spool-size/🟦️.ts";
import {readSpoolRegistryV1,registerSpoolV1} from "../🟦️.ts";
import {processPidExistsV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
process.env.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS="1500";

/** 📮️ Contract: every unit a build compiled while the fleet cache was unreachable reaches the fleet cache exactly once, after the build ended, after its upload worker was killed (twice), with no build running: the machine's spool role finds the root in the registry and starts the worker again. Real wrapper, real rustc, real cache server behind a switchable proxy that counts the result writes of each unit. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[],proxies:{stop(force?:boolean):void}[]=[],fleet=FLEET_POLICY_V1;
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"u2-"+label+"-")));scratch.push(path);return path;};
const holder=(cache:string):number|undefined=>{try{return (JSON.parse(readFileSync(join(cache,fleet.layout.state,"uploader.lock"),"utf8")) as {pid:number}).pid;}catch{return undefined;}};
const killWorker=async(cache:string):Promise<void>=>{const pid=holder(cache);if(pid===undefined)return;try{process.kill(pid,"SIGKILL");}catch{}const until=Date.now()+10000;while(processPidExistsV1(pid)&&Date.now()<until)await new Promise(done=>setTimeout(done,50));};
const pause=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));
const caches:string[]=[];
afterAll(async()=>{for(const cache of caches)await killWorker(cache);for(const proxy of proxies)proxy.stop(true);for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function findRustc():string|undefined{
 if(process.env.SEMIO_TEST_RUSTC)return process.env.SEMIO_TEST_RUSTC;
 for(const root of[process.env.SEMIO_TEST_CHECKOUT,"/Users/ueli/Documents/semio"]){
  if(root===undefined)continue;
  try{
   const channel=/^channel\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(root,"rust-toolchain.toml"),"utf8"))?.[1],base=join(process.env.RUSTUP_HOME??join(homedir(),".rustup"),"toolchains"),name=readdirSync(base).find(entry=>entry.startsWith(channel+"-"));
   if(name!==undefined&&existsSync(join(base,name,"bin","rustc")))return join(base,name,"bin","rustc");
  }catch{}
 }
 return undefined;
}
const rustc=findRustc(),real=rustc===undefined?test.skip:test;

async function compile(machine:Readonly<{root:string;home:string;cache:string;cas:string;lane:string}>,endpoint:string,name:string,hash:string){
 const package_=join(machine.root,"crates",name),out=join(machine.root,"target","deps");
 mkdirSync(join(package_,"src"),{recursive:true});writeFileSync(join(package_,"src","lib.rs"),`pub fn ${name}() -> &'static str { "${name}" }\n`);
 mkdirSync(out,{recursive:true});mkdirSync(machine.lane,{recursive:true});mkdirSync(machine.cache,{recursive:true});
 const argv=["--crate-name",name,"--edition=2021",join(package_,"src","lib.rs"),"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","metadata="+hash,"-C","extra-filename=-"+hash,"--out-dir",out,"-L","dependency="+out,"--remap-path-prefix="+machine.root+"=."];
 const environment={...process.env,HOME:machine.home,CARGO_HOME:join(machine.home,".cargo"),RUSTUP_HOME:join(machine.home,".rustup"),SEMIO_UNIT_CACHE_ROOT:machine.cache,SEMIO_UNIT_CACHE_LANE:"u2",SEMIO_UNIT_CACHE_LANE_ROOT:machine.lane,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:machine.root,SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc!]),SEMIO_UNIT_CACHE_TOOLCHAIN:"u2-test",CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"1.0.0",CARGO_MANIFEST_DIR:package_,PWD:machine.root,RUSTC_BOOTSTRAP:"1",SEMIO_CAS_ROOT:machine.cas,SEMIO_REMOTE_CACHE_ENDPOINT:endpoint,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"2",SEMIO_OFFLOAD:"never"} as Record<string,string|undefined>;
 delete environment.SEMIO_MACHINE_STATE_DIRECTORY;process.env.SEMIO_CAS_ROOT=machine.cas;
 const write=process.stdout.write.bind(process.stdout),err=process.stderr.write.bind(process.stderr);
 process.stdout.write=(()=>true) as typeof process.stdout.write;process.stderr.write=(()=>true) as typeof process.stderr.write;
 const origin=process.cwd();process.chdir(machine.root);
 try{expect(await wrapRustcV1(argv,environment,machine.root),"compile "+name).toBe(0);}finally{process.chdir(origin);process.stdout.write=write;process.stderr.write=err;}
 return environment;
}

real("units compiled while the cache is unreachable survive the end of the build and two killed upload workers, and arrive exactly once when the cache returns, with no build running",async()=>{
 const base=directory("m"),machine={root:join(base,"ws"),home:join(base,"h"),cache:join(base,"ws",".cache","b","r","m","u"),cas:join(base,"cas"),lane:join(base,"ws",".cache","b","r","m","u","lane","t-u2")};caches.push(machine.cache);
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"},graceMilliseconds:50});servers.push(server);
 const writes=new Map<string,number>(),handler=async(request:Request)=>{
  const url=new URL(request.url);if(request.method==="PUT"&&url.pathname.includes("/ac/"))writes.set(url.pathname,(writes.get(url.pathname)??0)+1);
  return fetch(server.url+url.pathname+url.search,{method:request.method,headers:request.headers,body:request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer()});
 };
 const first=Bun.serve({port:0,fetch:handler}),port=first.port;first.stop(true);
 const endpoint=`http://127.0.0.1:${port}`,names=["ua","ub","uc","ud"];
 let environment:Record<string,string|undefined>={};
 for(const [index,name] of names.entries())environment=await compile(machine,endpoint,name,(index+1).toString(16).repeat(16));
 const keys=readLookupsV1(machine.cache).filter(row=>names.includes(row.crate)).map(row=>row.remote!);
 expect(keys.length,"four units were compiled").toBe(4);
 expect(spoolLengthV1(machine.cache),"all four wait in the spool: the cache was down for the whole build").toBe(4);
 await killWorker(machine.cache);
 expect(spoolLengthV1(machine.cache),"the build ended and its worker died: nothing was lost").toBe(4);

 const sweeping={SEMIO_CAS_ROOT:machine.cas,SEMIO_REMOTE_CACHE_TOKEN:"w-token"};
 const registered=readSpoolRegistryV1(sweeping);
 expect(registered.map(row=>row.registration.root),"the build registered its root with the machine").toEqual([machine.cache]);
 expect(JSON.stringify(registered[0]!.registration),"no secret is registered").not.toContain("w-token");
 expect(registered[0]!.registration.environment.SEMIO_REMOTE_CACHE_ENDPOINT).toBe(endpoint);

 let started=0;for(const until=Date.now()+20000;started===0&&Date.now()<until;await pause(500))started=sweepSpoolsV1(sweeping).started;
 expect(started,"the sweep started an upload worker for the orphaned spool").toBe(1);
 await pause(3000);
 expect(spoolLengthV1(machine.cache),"the cache is still down: every unit still waits").toBe(4);
 expect(readBuildCountersV1(machine.root).skipReasons,"waiting is no loss").toEqual({});
 await killWorker(machine.cache);

 proxies.push(Bun.serve({port,fetch:handler}));
 const deadline=Date.now()+150000;
 while(spoolLengthV1(machine.cache)>0&&Date.now()<deadline){sweepSpoolsV1(sweeping);await pause(1000);}
 expect(spoolLengthV1(machine.cache),"the spool drained with no build running").toBe(0);
 const client=createFleetClientV1(readFleetSettingsV1(machine.cache,{...environment,SEMIO_REMOTE_CACHE_ENDPOINT:endpoint})!);
 for(const key of keys){
  expect(await client.hasResult(key),"unit "+key.slice(0,12)+" is in the fleet cache").toBe(true);
  expect([...writes].filter(([path])=>path.includes(key)).map(([,count])=>count),"and was written exactly once").toEqual([1]);
 }
 expect(readBuildCountersV1(machine.root).skipReasons,"nothing was counted as lost").toEqual({});
},300000);

const job=(key:string,localKey:string):SpoolJobV1=>({version:1,key,localKey,slot:"2".repeat(64),crate:"core",cwd:"/w",workspace:null,laneRoot:null,owner:"t:1:aa",claimed:false,costMs:1,platform:{version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""},queuedMs:Date.now()});

test("the registry holds no secret; a spool queued under another cache identity is not drained by a process that names another token; a vanished root and a long-empty spool are forgotten; a job queued twice under one key is one job",()=>{
 const cas=directory("cas"),root=join(directory("r"),"cache"),gone=join(directory("g"),"cache"),empty=join(directory("e"),"cache"),workspace=directory("w");
 for(const path of[root,gone,empty])mkdirSync(path,{recursive:true});
 const environment={SEMIO_CAS_ROOT:cas,SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"one",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace,CARGO_PKG_NAME:"x",PATH:"/usr/bin",OTHER_SECRET:"s"};
 const identity=readFleetSettingsV1(root,environment)!.identity;
 for(const path of[root,gone,empty])registerSpoolV1(path,environment,identity);
 const rows=readSpoolRegistryV1(environment);expect(rows.length).toBe(3);
 const text=JSON.stringify(rows.map(row=>row.registration));expect(text).not.toContain("one");expect(text).not.toContain("OTHER_SECRET");expect(text).not.toContain("CARGO_PKG_NAME");expect(rows[0]!.registration.environment.PATH).toBe("/usr/bin");
 expect(enqueueUploadV1(root,job("a".repeat(64),"b".repeat(64)))).toBe(true);expect(enqueueUploadV1(root,job("a".repeat(64),"b".repeat(64)))).toBe(true);expect(spoolLengthV1(root),"one job per key").toBe(1);
 rmSync(gone,{recursive:true,force:true});
 const old=new Date(Date.now()-fleet.uploader.registryRetentionMilliseconds-60000);
 for(const row of readSpoolRegistryV1(environment))if(row.registration.root===empty)utimesSync(row.file,old,old);
 const sweep=sweepSpoolsV1({SEMIO_CAS_ROOT:cas,SEMIO_REMOTE_CACHE_TOKEN:"two"});
 expect(sweep).toEqual({roots:3,waiting:1,started:0,skipped:1,forgotten:2});
 expect(readSpoolRegistryV1(environment).map(row=>row.registration.root),"only the root that waits is left").toEqual([root]);
});
