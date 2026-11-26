import {afterEach,expect,test} from "bun:test";
import {chmodSync,mkdirSync,mkdtempSync,rmSync,truncateSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import type {RemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import type {RemoteExecutionDigestV1} from "../../../🟦️.ts";
import {validateRemoteExecutionRustcActionV1} from "../../../🧱️actions/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../../📐️policy/🟦️.ts";
import {readRemoteCachePolicyV1} from "../../../../🌐️remote-cache/🟦️.ts";
import type {OffloadPlanV1} from "../../🗺️paths/🟦️.ts";
import {buildOffloadActionV1,offloadRustcCommandV1} from "../🟦️.ts";

const scratch:string[]=[],policy=OFFLOAD_POLICY_V1;
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});

class Sink implements RemoteExecutionContentV1{
 async has(){return false;}
 async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return [...digests];}
 async read(){return undefined;}
 async stream(){return undefined;}
 async write(){}
 async readActionResult(){return undefined;}
 async writeActionResult(){}
}

const files=(label:string,mode:number):Map<string,string>=>{
 const base=mkdtempSync(join(testScratchRootV1("rexec-action"),`${label}-`));scratch.push(base);
 mkdirSync(join(base,"src"),{recursive:true});writeFileSync(join(base,"src","lib.rs"),"pub fn f(){}\n");writeFileSync(join(base,"Cargo.toml"),"[package]\n");
 chmodSync(join(base,"src","lib.rs"),mode);chmodSync(join(base,"Cargo.toml"),mode);
 return new Map([[".semio-offload/p/src/lib.rs",join(base,"src","lib.rs")],[".semio-offload/p/Cargo.toml",join(base,"Cargo.toml")]]);
};

const plan=(separator:"/"|"\\",mode:number,label:string):OffloadPlanV1=>{
 const name=(text:string)=>text.split("/").join(separator);
 return Object.freeze({
  arguments:["--crate-name","u","--edition=2021",".semio-offload/p/src/lib.rs","--out-dir",policy.virtual.out,"-L","dependency=.semio-offload/d"],
  environment:{PATH:policy.environment.path,CARGO_PKG_NAME:"u",SEMIO_CWD_NAME:name("./ws/crates/u"),SEMIO_REMAPS:`${policy.virtual.package}=${name("./ws/crates/u")}\n${policy.virtual.generated}=${name("./out/build-script")}`,SEMIO_ABSOLUTE:"CARGO_MANIFEST_DIR",SEMIO_REL_CARGO_MANIFEST_DIR:policy.virtual.package},
  files:files(label,mode),mappings:[],outDir:"out",crateName:"u",target:undefined,extraFilename:"",
 });
};
const platform=[{name:"Arch",value:"x86_64"},{name:"semio-target",value:"x86_64-pc-windows-msvc"},{name:"semio-toolchain",value:"cccccccccccccccc"}];

test("the same unit described by a POSIX requester and by a native Windows requester has one action digest: remapped names are spelled with /, no input carries an executable bit and every path below the root is @root-relative",async()=>{
 const posixPlan=plan("/",0o755,"posix"),windowsPlan=plan("\\",0o644,"windows");
 const build=async(value:OffloadPlanV1)=>{const built=await buildOffloadActionV1({plan:value,content:new Sink(),instance:"main",platform});if("declined" in built)throw Error(built.detail);return built.digest;};
 const left=await build(posixPlan),right=await build(windowsPlan);
 expect(right).toEqual(left);
 const other=await build({...posixPlan,environment:{...posixPlan.environment,SEMIO_CWD_NAME:"./elsewhere"}});
 expect(other.hash,"a different name is a different action").not.toBe(left.hash);
 for(const value of [posixPlan,windowsPlan]){
  const command=offloadRustcCommandV1(value,"x86_64-pc-windows-msvc");
  expect(command.arguments.join("\n")).not.toContain("\\");
  expect(validateRemoteExecutionRustcActionV1({arguments:[...command.arguments],environment_variables:command.environment.map(([name,text])=>({name,value:text})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""})).toBeUndefined();
 }
});

test("the macOS object prefix is part of the command only for an Apple target, decided by the requester's target, not by the executing host",()=>{
 const value=plan("/",0o644,"target");
 expect(offloadRustcCommandV1(value,"aarch64-apple-darwin").arguments.some(argument=>argument.includes("oso_prefix"))).toBe(true);
 expect(offloadRustcCommandV1(value,"x86_64-pc-windows-msvc").arguments.some(argument=>argument.includes("oso_prefix"))).toBe(false);
});

test("a tree larger than the upload bound is no reason to stay: blobs the store already holds do not travel, and only what is missing counts against the bound; one blob past the per-entry limit of the store can never be held, so it declines at once",async()=>{
 const server=readRemoteCachePolicyV1().server,entry=Math.min(server.maximumEntryBytes,Math.floor(server.maximumBytes/2)),seed=plan("/",0o644,"large"),extra=new Map<string,string>(),base=mkdtempSync(join(testScratchRootV1("rexec-action"),"big-"));scratch.push(base);
 const big=Array.from({length:Math.floor(policy.limits.maximumUploadBytes/entry)+1},(_,at)=>{const path=join(base,`big-${at}.bin`);writeFileSync(path,"");truncateSync(path,entry);extra.set(`.semio-offload/p/big-${at}.bin`,path);return path;});
 const input:OffloadPlanV1=Object.freeze({...seed,files:new Map([...seed.files,...extra])});
 const held=new class extends Sink{override async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return digests.filter(row=>row.size<1048576);}}();
 const digest=(path:string):string=>{const at=big.indexOf(path);return at<0?"b".repeat(64):String(at).repeat(64).slice(0,64);};
 const shipped=await buildOffloadActionV1({plan:input,content:held,instance:"main",platform,digest});
 expect("declined" in shipped,"the large blobs are already in the store").toBe(false);
 if(!("declined" in shipped)){expect(shipped.bytes).toBeGreaterThan(policy.limits.maximumUploadBytes);expect(shipped.uploaded).toBeLessThan(shipped.files+4);}
 const refused=await buildOffloadActionV1({plan:input,content:new Sink(),instance:"main",platform,digest});
 expect("declined" in refused&&refused.declined).toBe("inputs-too-large");
 expect("declined" in refused&&refused.detail.startsWith("upload ")).toBe(true);
 truncateSync(big[0]!,entry+1);
 const oversized=await buildOffloadActionV1({plan:input,content:held,instance:"main",platform,digest});
 expect("declined" in oversized&&oversized.declined).toBe("inputs-too-large");
 expect("declined" in oversized&&oversized.detail).toContain("per entry");
});
