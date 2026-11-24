import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {readRemoteCacheMetaV1} from "../../../../../🏗️build/🌐️remote-cache/🏷️retention/🟦️.ts";
import {wrapRustcV1} from "../../📜️script.ts";
import {newestEntryV1,readLookupsV1} from "../../💾️store/🟦️.ts";
import {drainUploadsV1,spoolLengthV1} from "../../🛰️fleet/📤️uploader/🟦️.ts";
import {readBuildCountersV1} from "../../🛰️fleet/📊️counters/🟦️.ts";
import {readManifestV1} from "../../🛰️fleet/📋️manifest/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
process.env.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS="1500";

/** 📋️ Contract: a unit is never compiled twice anywhere because of a file the compiler reads that the scan of its sources did not reach. The crate below reads `data.txt` through an `include_str!` that a macro expands (the stand-in of a derive that reads a data file at expansion: the compiler lists the file, the scan of the source text does not find it). Machine A compiles it and publishes the manifest and the unit; machine B, at another checkout path with an empty local store that never compiled it, hits the fleet with its first lookup; an edit of the data file misses, recompiles, publishes and then hits; an input set that grows (a second data file) is learned once and found by a third machine; the unit entry declares the full input set to retention. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"k7-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
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
type Machine=Readonly<{root:string;home:string;cache:string;cas:string;lane:string;hash:string}>;
const machine=(label:string,depth:string,hash:string):Machine=>{const base=directory(label),root=join(base,depth,"ws");return{root,home:join(base,"h"),cache:join(root,".cache","b","r","m","u"),cas:join(base,"cas"),lane:join(root,".cache","b","r","m","u","lane","t-"+hash.slice(0,6)),hash};};
const fleet=FLEET_POLICY_V1;

async function compile(m:Machine,server:RemoteCacheServerV1,source:string,files:Record<string,string>){
 const package_=join(m.root,"crates","data"),out=join(m.root,"target","deps");
 mkdirSync(join(package_,"src"),{recursive:true});writeFileSync(join(package_,"src","lib.rs"),source);for(const [name,text] of Object.entries(files))writeFileSync(join(package_,"src",name),text);
 mkdirSync(out,{recursive:true});mkdirSync(m.lane,{recursive:true});mkdirSync(m.cache,{recursive:true});
 const argv=["--crate-name","data","--edition=2021",join(package_,"src","lib.rs"),"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","metadata="+m.hash,"-C","extra-filename=-"+m.hash,"--out-dir",out,"-L","dependency="+out,"--remap-path-prefix="+m.root+"=."];
 const environment={...process.env,HOME:m.home,CARGO_HOME:join(m.home,".cargo"),RUSTUP_HOME:join(m.home,".rustup"),SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:"k7",SEMIO_UNIT_CACHE_LANE_ROOT:m.lane,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:m.root,SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc!]),SEMIO_UNIT_CACHE_TOOLCHAIN:"k7-test",CARGO_PKG_NAME:"data",CARGO_PKG_VERSION:"1.0.0",CARGO_MANIFEST_DIR:package_,PWD:m.root,RUSTC_BOOTSTRAP:"1",SEMIO_CAS_ROOT:m.cas,SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"5",SEMIO_OFFLOAD:"never"} as Record<string,string|undefined>;
 delete environment.SEMIO_MACHINE_STATE_DIRECTORY;process.env.SEMIO_CAS_ROOT=m.cas;
 const before=readLookupsV1(m.cache).length,write=process.stdout.write.bind(process.stdout),err=process.stderr.write.bind(process.stderr);
 process.stdout.write=(()=>true) as typeof process.stdout.write;process.stderr.write=(()=>true) as typeof process.stderr.write;
 const origin=process.cwd();process.chdir(m.root);
 try{expect(await wrapRustcV1(argv,environment,m.root),"compile").toBe(0);}finally{process.chdir(origin);process.stdout.write=write;process.stderr.write=err;}
 const deadline=Date.now()+60000;
 while(Date.now()<deadline){const result=await drainUploadsV1(m.cache,environment,{idleMilliseconds:0});if(result!=="busy"&&spoolLengthV1(m.cache)===0&&!existsSync(join(m.cache,fleet.layout.state,"uploader.lock")))break;await new Promise(done=>setTimeout(done,150));}
 return readLookupsV1(m.cache).slice(before).filter(row=>row.crate==="data");
}
const lib=(extra:string[])=>`macro_rules! data { ($path:literal) => { include_str!($path) }; }\npub const DATA: &str = data!("data.txt");\n${extra.map((name,index)=>`pub const X${index}: &str = data!("${name}");\n`).join("")}`;

real("a data file the scan cannot reach is part of the unit key from the first compile on: another machine hits with its first lookup, an edit misses once, a grown input set is learned once",async()=>{
 const a=machine("a","a","a1b2c3d4e5f60718"),server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"},graceMilliseconds:50,staleness:{mode:"dry-run",workspace:a.root,recheckSeconds:0,manifestNames:["unit.json","stage.json"],examples:3}});servers.push(server);
 const b=machine("b","b/c/d","0f1e2d3c4b5a6978"),c=machine("c","e","1122334455667788"),d=machine("d","f/g","2233445566778899");
 const one=lib([]),data1={"data.txt":"first"};

 const first=await compile(a,server,one,data1);
 expect(first.map(row=>row.outcome),"A compiles the cold unit").toEqual(["miss"]);
 const manifest=readManifestV1(a.cache,[...new Set(readdirSync(join(a.cache,fleet.layout.state,fleet.layout.manifests)).flatMap(directoryName=>readdirSync(join(a.cache,fleet.layout.state,fleet.layout.manifests,directoryName)).filter(name=>/^[0-9a-f]{64}$/u.test(name))))][0]!);
 expect(manifest?.manifest.files,"the manifest names the data file by its portable name").toEqual(["@@semio-cwd@@/crates/data/src/data.txt"]);expect(manifest?.shared).toBe(true);
 expect(readBuildCountersV1(a.root).skipReasons?.["key-incomplete"]??0,"the compiled unit is not discarded as key-incomplete").toBe(0);

 const second=await compile(b,server,one,data1);
 expect(second.map(row=>row.outcome),"B, which never compiled it, hits the fleet with its first lookup").toEqual(["fleet-hit"]);
 const fleetKey=second[0]!.remote!,meta=await readRemoteCacheMetaV1(server.store.root,`main/ac/${fleetKey.slice(0,2)}/${fleetKey}`);
 expect(meta?.declaration?.sources.map(row=>row.path),"the unit declares the full input set to retention").toEqual(["crates/data/src/data.txt","crates/data/src/lib.rs"]);
 expect(newestEntryV1(b.cache,second[0]!.key)?.retention?.sources.map(row=>row.path)).toEqual(["crates/data/src/data.txt","crates/data/src/lib.rs"]);

 const edited=await compile(b,server,one,{"data.txt":"second"});
 expect(edited.map(row=>row.outcome),"an edit of the data file misses").toEqual(["miss"]);
 const again=await compile(b,server,one,{"data.txt":"second"});expect(again.map(row=>row.outcome),"and then hits locally").toEqual(["local-hit"]);
 const other=await compile(c,server,one,{"data.txt":"second"});expect(other.map(row=>row.outcome),"a third machine hits the recompiled unit with its first lookup").toEqual(["fleet-hit"]);
 const back=await compile(c,server,one,data1);expect(back.map(row=>row.outcome),"the first content is still served from the fleet").toEqual(["fleet-hit"]);

 const grown=lib(["more.txt"]),both={"data.txt":"second","more.txt":"extra"};
 const learn=await compile(a,server,grown,both);
 expect(learn.map(row=>row.outcome),"a source edit that reads one more file compiles once").toEqual(["miss"]);
 expect(readBuildCountersV1(a.root).skipReasons?.["key-incomplete"]??0).toBe(0);
 const found=await compile(d,server,grown,both);expect(found.map(row=>row.outcome),"a machine that never compiled it finds the grown manifest and hits").toEqual(["fleet-hit"]);
 const stale=await compile(b,server,grown,both);
 expect(stale.map(row=>row.outcome),"a machine with the old manifest looks again before compiling and hits").toEqual(["fleet-hit"]);
 const changed=await compile(d,server,grown,{"data.txt":"second","more.txt":"changed"});expect(changed.map(row=>row.outcome),"the new file is keyed").toEqual(["miss"]);
},900000);
