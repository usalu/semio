import {afterAll,beforeAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {lookupBuildRunV1,publishBuildRunV1,restoreBuildRunV1} from "../../../💾️store/🟦️.ts";
import {installFleetUnitV1} from "../../../🛰️fleet/📥️adopt/🟦️.ts";
import {uploadUnitV1,type SpoolJobV1} from "../../../🛰️fleet/📤️uploader/🟦️.ts";
import {portableRootsV1,remoteKeyV1,toolRootsV1,localKeyV1,type PlatformV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";
import {buildRunKeyInputV1} from "../../🔑️run-key/🟦️.ts";
import type {FleetClientV1} from "../../../🛰️fleet/📡️client/🟦️.ts";

/** 🌐️ A build script run recorded on one machine is installed on another through the fleet tier and replayed there: its OUT_DIR text carries the roots of the second machine, its binary file is byte-equal, its streams name the second machine's Cargo home; a run whose output names the first machine's home never leaves it. */
const scratch=realpathSync(mkdtempSync(join(tmpdir(),"fleet-run-"))),previous=process.env.SEMIO_CAS_ROOT;
const sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
const platform={version:1,os:"darwin",arch:"arm64",host:"aarch64-apple-darwin",rustc:"a".repeat(64),toolchain:"t",linker:"",cpu:""} as PlatformV1;
afterAll(()=>{if(previous===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previous;rmSync(scratch,{recursive:true,force:true});});
beforeAll(()=>{process.env.SEMIO_CAS_ROOT=join(scratch,"cas-start");});

type Machine=Readonly<{name:string;workspace:string;package:string;cargo:string;rustup:string;home:string;root:string;out:string;cas:string}>;
function machine(name:string,layout:string,cargoInside:boolean):Machine{
 const workspace=join(scratch,name,layout),cargo=cargoInside?join(workspace,".cache","cargo-home"):join(scratch,name,"home",".cargo"),pack=join(workspace,"vendor","ring-0.17.0");
 mkdirSync(join(pack,"src"),{recursive:true});mkdirSync(join(cargo,"registry","src","idx","dep-1.0","lib"),{recursive:true});
 writeFileSync(join(pack,"build.rs"),"fn main(){}\n");writeFileSync(join(pack,"Cargo.toml"),"[package]\nname=\"ring\"\n");writeFileSync(join(pack,"src","lib.rs"),"//\n");
 return{name,workspace,package:pack,cargo,rustup:join(scratch,name,"home",".rustup"),home:join(scratch,name,"home"),root:join(scratch,name,"cache"),out:join(scratch,name,"out"),cas:join(scratch,name,"cas")};
}
const environmentOf=(m:Machine)=>({HOME:m.home,CARGO_HOME:m.cargo,RUSTUP_HOME:m.rustup,CARGO_MANIFEST_DIR:m.package,CARGO_PKG_NAME:"ring",CARGO_PKG_VERSION:"0.17.0",TARGET:"aarch64-apple-darwin",HOST:"aarch64-apple-darwin",PROFILE:"release"});
const rootsOf=(m:Machine)=>portableRootsV1({cwd:m.package,workspace:m.workspace,hints:[m.package],...toolRootsV1(environmentOf(m))});

function fakeClient(){
 const results=new Map<string,Uint8Array>(),blobs=new Map<string,Uint8Array>();
 const client={
  settings:{endpoint:"http://fake",instance:"main",token:undefined,upload:true,stateDirectory:join(scratch,"state"),signedRoot:undefined,identity:"fake"},
  usable:()=>true,state:()=>"ok",claimsUsable:()=>true,
  async getResult(key:string){return results.get(key)??"miss";},
  async getBlob(hash:string,size:number){const bytes=blobs.get(hash);return bytes!==undefined&&bytes.length===size?bytes:undefined;},
  async hasBlob(hash:string){return blobs.has(hash);},
  async storeBlob(hash:string,bytes:Uint8Array){if(sha(bytes)!==hash)return "refused";blobs.set(hash,bytes);return "stored";},
  async storeResult(key:string,bytes:Uint8Array){if(results.has(key))return "diverged";results.set(key,bytes);return "stored";},
  async renew(){return "granted";},async release(){},async claimSlot(){return true;},
 } as unknown as FleetClientV1;
 return{client,results,blobs};
}

function record(m:Machine,generated:string,stdout:string){
 process.env.SEMIO_CAS_ROOT=m.cas;
 mkdirSync(join(m.out,"gen"),{recursive:true});writeFileSync(join(m.out,"gen","table.rs"),generated);writeFileSync(join(m.out,"blob.bin"),new Uint8Array([0,255,1,254]));
 const input=buildRunKeyInputV1({toolchain:"rustc 1.0",identity:"unit:"+"1".repeat(64),args:[],environment:environmentOf(m),laneRoot:undefined}),roots=rootsOf(m);
 const key=localKeyV1(input,roots),remote=remoteKeyV1(platform,input,roots);
 const entry=publishBuildRunV1(m.root,{key,package:"ring",cwd:m.package,laneRoot:undefined,directives:{files:["build.rs"],variables:[]},environment:environmentOf(m),outDir:m.out,stdout,stderr:"",stampDirectory:undefined});
 return{key,remote,entry};
}
const jobOf=(m:Machine,key:string,remote:string):SpoolJobV1=>({version:1,key:remote,localKey:key,slot:sha("slot"),crate:"ring",cwd:m.package,workspace:m.workspace,laneRoot:null,owner:"o",claimed:false,costMs:5,platform,queuedMs:Date.now(),run:true,tools:toolRootsV1(environmentOf(m))});

test("a build script run recorded on one machine is installed on another with other home, Cargo home (inside the checkout on the first) and checkout root, and replays there with its own roots",async()=>{
 const one=machine("one","Users/ueli/semio",true),two=machine("two","Users/Shared/semio",false);
 const world=fakeClient(),first=record(one,"// table from "+one.cargo+"/registry/src/idx/dep-1.0\nconst N:usize=3;\n","cargo:rerun-if-changed=build.rs\ncargo:rustc-link-search="+one.cargo+"/registry/src/idx/dep-1.0/lib\ncargo:rustc-env=OUT="+one.out+"/gen\n");
 expect(first.entry).toBeDefined();
 process.env.SEMIO_CAS_ROOT=one.cas;
 expect(await uploadUnitV1(world.client,one.root,jobOf(one,first.key,first.remote))).toBe(true);
 expect(world.results.has(first.remote)).toBe(true);
 const wire=[...world.blobs.values()].map(bytes=>Buffer.from(bytes).toString("latin1")).join("\n");
 expect(wire).not.toContain(one.cargo);expect(wire).not.toContain(one.workspace);expect(wire).toContain("@@semio-cargo-home@@");
 process.env.SEMIO_CAS_ROOT=two.cas;
 const input=buildRunKeyInputV1({toolchain:"rustc 1.0",identity:"unit:"+"1".repeat(64),args:[],environment:environmentOf(two),laneRoot:undefined}),keyTwo=localKeyV1(input,rootsOf(two)),remoteTwo=remoteKeyV1(platform,input,rootsOf(two));
 expect(keyTwo,"the local key is the same on both machines").toBe(first.key);expect(remoteTwo,"so is the fleet key").toBe(first.remote);
 expect(await installFleetUnitV1({client:world.client,root:two.root,localKey:keyTwo,key:remoteTwo,slot:sha("slot"),platform,roots:rootsOf(two),run:true})).toBe("hit");
 const entry=lookupBuildRunV1(two.root,keyTwo,{cwd:two.package,laneRoot:undefined,environment:environmentOf(two),stampDirectory:undefined});
 expect(entry,"the facts of the recorded run still hold in the second tree").toBeDefined();
 mkdirSync(join(two.out,"gen"),{recursive:true});
 const messages=restoreBuildRunV1(two.root,entry!,{outDir:two.out,laneRoot:undefined});
 const table=readFileSync(join(two.out,"gen","table.rs"),"utf8");
 expect(table).toContain("// table from "+two.cargo+"/registry/src/idx/dep-1.0");expect(table).not.toContain(one.cargo);
 expect(messages.stdout).toContain("cargo:rustc-link-search="+two.cargo+"/registry/src/idx/dep-1.0/lib");
 expect(messages.stdout).toContain("cargo:rustc-env=OUT="+two.out+"/gen");
 expect([...readFileSync(join(two.out,"blob.bin"))]).toEqual([0,255,1,254]);
 const python=spawnSync("python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",join(two.out,"blob.bin")],{encoding:"utf8"});
 expect(python.status).toBe(0);expect(python.stdout.trim()).toBe(sha(new Uint8Array([0,255,1,254])));
});

test("a run holding a binary file with a root of its machine is not uploaded: nothing reaches the fleet",async()=>{
 const m=machine("leaky","Users/ueli/semio",false),world=fakeClient(),run=record(m,"// plain\n","cargo:rerun-if-changed=build.rs\n");
 process.env.SEMIO_CAS_ROOT=m.cas;
 expect(run.entry).toBeDefined();
 writeFileSync(join(m.out,"embedded.bin"),Buffer.concat([Buffer.from([0]),Buffer.from(m.cargo+"/registry"),Buffer.from([0])]));
 const again=publishBuildRunV1(m.root,{key:run.key,package:"ring",cwd:m.package,laneRoot:undefined,directives:{files:["build.rs"],variables:[]},environment:environmentOf(m),outDir:m.out,stdout:"cargo:rerun-if-changed=build.rs\n",stderr:"",stampDirectory:undefined});
 expect(again).toBeDefined();
 expect(await uploadUnitV1(world.client,m.root,jobOf(m,run.key,run.remote))).toBe(false);
 expect(world.results.size).toBe(0);expect(world.blobs.size).toBe(0);
});
