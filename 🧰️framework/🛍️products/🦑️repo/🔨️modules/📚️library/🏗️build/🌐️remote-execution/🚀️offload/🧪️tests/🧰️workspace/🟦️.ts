import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import fixture from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🧫️fixtures/🔣️.json" with {type:"json"};

/** 🧰️ The pinned compiler the tests run, named by environment so requester and workers see the same build. */
export const TEST_TOOLCHAIN_V1="nightly-2026-07-20";
export const testRustEnvironmentV1=(extra:Readonly<Record<string,string|undefined>>={}):Record<string,string>=>Object.fromEntries(Object.entries({...process.env,RUSTUP_TOOLCHAIN:TEST_TOOLCHAIN_V1,...extra}).filter((row):row is [string,string]=>row[1]!==undefined));
/** #️⃣ A stable 16-hex `extra-filename` per crate name, the shape Cargo gives. */
export const externHash=(name:string):string=>createHash("sha256").update(name).digest("hex").slice(0,16);

/** 📁️ One scratch checkout: a working directory below a fresh temporary directory, so two checkouts differ in their absolute path; the directory name carries an emoji like the repository's. */
export type TestCheckoutV1=Readonly<{base:string;root:string;target:string;deps:string;crate:(name:string,source:string)=>string;compile:(name:string,source:string,externs?:readonly string[])=>void;arguments:(name:string,externs?:readonly string[],more?:readonly string[])=>string[];environment:(name:string)=>Record<string,string>}>;

export function createTestCheckoutV1(label:string,scratch:string=tmpdir()):TestCheckoutV1{
 const base=realpathSync(mkdtempSync(join(realpathSync(scratch),`offload-${label}-`))),root=join(base,"ws✏️"),target=join(root,"t"),deps=join(target,"debug","deps");
 mkdirSync(deps,{recursive:true});
 const crate=(name:string,source:string):string=>{const directory=join(root,"pk",name);mkdirSync(join(directory,"src"),{recursive:true});writeFileSync(join(directory,"src","lib.rs"),source);writeFileSync(join(directory,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\n`);return directory;};
 const environment=(name:string)=>({CARGO_MANIFEST_DIR:join(root,"pk",name),CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:name,CARGO_PRIMARY_PACKAGE:"1",RUST_MIN_STACK:"8388608"});
 const args=(name:string,externs:readonly string[]=[],more:readonly string[]=[])=>["--crate-name",name,"--edition=2021",`pk/${name}/src/lib.rs`,"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","debuginfo=2","--remap-path-prefix",`${root}=.`,"-C",`metadata=${externHash(name)}`,"-C",`extra-filename=-${externHash(name)}`,"--out-dir",deps,"-L",`dependency=${deps}`,...externs.flatMap(external=>["--extern",`${external}=${join(deps,`lib${external}-${externHash(external)}.rlib`)}`]),...more];
 const compile=(name:string,source:string,externs:readonly string[]=[])=>{
  crate(name,source);
  const result=spawnSync("rustc",args(name,externs),{cwd:root,encoding:"utf8",env:testRustEnvironmentV1(environment(name))});
  if(result.status!==0)throw Error(`rustc failed for ${name}: ${result.stderr}`);
 };
 return Object.freeze({base,root,target,deps,crate,compile,arguments:args,environment});
}

/** 🌡️ A host probe for the capacity decision, built from the sampling fixtures: `critical-pressure-gigabyte-swap` is a machine that must make units wait, `quiet-no-swap` with `roomy` is one with 62 GB free that must not. */
export function testHostProbeV1(name:"critical-pressure-gigabyte-swap"|"quiet-no-swap",roomy=false){
 const row=fixture.darwin.find(candidate=>candidate.name===name)!,vmStat=roomy?row.vmStat.replace(/Pages free:\s+\d+\./u,"Pages free:                                  4000000."):row.vmStat;
 return{platform:"darwin" as const,read:async()=>null,run:async(command:string,args:readonly string[])=>command==="vm_stat"?vmStat:command==="sysctl"&&args.includes("vm.swapusage")?row.swapUsage:command==="sysctl"&&args.includes("kern.memorystatus_vm_pressure_level")?row.pressureLevel:command==="sysctl"?(roomy?"":row.filesTable):""};
}
