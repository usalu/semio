import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join} from "node:path";
import {wrapRustcV1} from "../../📜️script.ts";
import {readLookupsV1} from "../../💾️store/🟦️.ts";
import {outDirectoryIdentityV1} from "../../🔏️key/🟦️.ts";

/** 🌍️ Contract: the fleet key of a unit does not depend on where its checkout lives. One registry crate, one workspace crate and one crate that consumes a build script's OUT_DIR are compiled through the real wrapper under three checkout roots (one under `.sinc/<name>`, each with its own Cargo home, lane and Cargo's build hash): every fleet key equals the same unit's key under the other roots. */
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"k5-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
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

type Checkout=Readonly<{root:string;home:string;cargoHome:string;lane:string;hash:string}>;
const checkouts=(base:string):readonly Checkout[]=>{
 const make=(root:string,home:string,cargoHome:string,hash:string):Checkout=>({root,home,cargoHome,lane:join(root,".cache","b","r","m","u","lane","t-"+hash.slice(0,6)),hash});
 const one=join(base,"Documents","semio"),two=join(base,".sinc","play"),three=join(base,"work","checkout-three");
 return[make(one,join(base,"h1"),join(one,".cache","b","r","m","cargo","home"),"a1b2c3d4e5f60718"),make(two,join(base,"h2"),join(two,".cache","b","r","m","cargo","home"),"0f1e2d3c4b5a6978"),make(three,join(base,"h3"),join(base,"h3",".cargo"),"1122334455667788")];
};

const sources={registry:"pub fn answer(x: u32) -> u32 { x + 1 }\n",workspace:"pub fn twice(x: u32) -> u32 { x * 2 }\n",generated:'include!(concat!(env!("OUT_DIR"), "/gen.rs"));\n'};

async function compile(checkout:Checkout,kind:keyof typeof sources,fleet:boolean):Promise<string|undefined>{
 const registry=kind==="registry",package_=registry?join(checkout.cargoHome,"registry","src","index.crates.io-test","foo-1.0.0"):join(checkout.root,"crates",kind),out=join(checkout.root,"target","release","build",kind+"-"+checkout.hash,"out");
 mkdirSync(join(package_,"src"),{recursive:true});writeFileSync(join(package_,"src","lib.rs"),sources[kind]);
 if(kind==="generated"){mkdirSync(out,{recursive:true});writeFileSync(join(out,"gen.rs"),"pub fn generated() -> u32 { 7 }\n");}
 mkdirSync(join(checkout.root,"target","deps"),{recursive:true});mkdirSync(checkout.lane,{recursive:true});
 const cwd=registry?package_:checkout.root,name=registry?"foo":kind,remap=registry?"--remap-path-prefix="+join(checkout.cargoHome,"registry","src")+"=":"--remap-path-prefix="+checkout.root+"=.";
 const argv=["--crate-name",name,"--edition=2021",join(package_,"src","lib.rs"),"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","metadata="+checkout.hash,"-C","extra-filename=-"+checkout.hash,"--out-dir",join(checkout.root,"target","deps"),"-L","dependency="+join(checkout.root,"target","deps"),remap];
 const environment={...process.env,HOME:checkout.home,CARGO_HOME:checkout.cargoHome,RUSTUP_HOME:join(checkout.home,".rustup"),SEMIO_UNIT_CACHE_ROOT:join(checkout.root,".cache","b","r","m","u"),SEMIO_UNIT_CACHE_LANE:"k5",SEMIO_UNIT_CACHE_LANE_ROOT:checkout.lane,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:checkout.root,SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc!]),SEMIO_UNIT_CACHE_TOOLCHAIN:"k5-test",CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"1.0.0",CARGO_MANIFEST_DIR:package_,PWD:cwd,RUSTC_BOOTSTRAP:"1",...kind==="generated"?{OUT_DIR:out}:{},...fleet?{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"1",SEMIO_REMOTE_CACHE_RETRIES:"0"}:{}} as Record<string,string|undefined>;
 delete environment.SEMIO_MACHINE_STATE_DIRECTORY;
 mkdirSync(environment.SEMIO_UNIT_CACHE_ROOT!,{recursive:true});process.env.SEMIO_CAS_ROOT=join(checkout.root,"cas");
 const before=readLookupsV1(environment.SEMIO_UNIT_CACHE_ROOT!).length,write=process.stdout.write.bind(process.stdout),err=process.stderr.write.bind(process.stderr);
 process.stdout.write=(()=>true) as typeof process.stdout.write;process.stderr.write=(()=>true) as typeof process.stderr.write;
 const origin=process.cwd();process.chdir(cwd);
 try{expect(await wrapRustcV1(argv,environment,cwd),name).toBe(0);}finally{process.chdir(origin);process.stdout.write=write;process.stderr.write=err;}
 return readLookupsV1(environment.SEMIO_UNIT_CACHE_ROOT!).slice(before).find(row=>row.crate===name)?.remote;
}

real("a registry crate, a workspace crate and a crate consuming OUT_DIR have one fleet key under three checkout roots with their own Cargo homes, lanes and Cargo build hashes",async()=>{
 const all=checkouts(directory("roots"));
 for(const kind of["registry","workspace","generated"] as const){
  const keys=[];for(const checkout of all)keys.push(await compile(checkout,kind,true));
  expect(keys.every(key=>typeof key==="string"&&/^[0-9a-f]{64}$/u.test(key)),kind+" has a fleet key under every root: "+JSON.stringify(keys)).toBe(true);
  expect(new Set(keys).size,kind+" gives one fleet key: "+JSON.stringify(keys)).toBe(1);
 }
},900000);

test("the OUT_DIR of a build script is keyed by the crate it belongs to, not by Cargo's hash of the unit or the directory it lives in",()=>{
 const lane="/w/.cache/lane/t-1";
 expect(outDirectoryIdentityV1("/w/target/release/build/serde-0123456789abcdef/out",lane)).toBe(outDirectoryIdentityV1("/elsewhere/t/debug/build/serde-fedcba9876543210/out",undefined));
 expect(outDirectoryIdentityV1(lane+"/release/build/serde-0123456789abcdef/out",lane)).toBe(outDirectoryIdentityV1("/elsewhere/t/debug/build/serde-fedcba9876543210/out",undefined));
 expect(outDirectoryIdentityV1("/a/build/serde-0123456789abcdef/out",undefined)).not.toBe(outDirectoryIdentityV1("/a/build/serde_core-0123456789abcdef/out",undefined));
 expect(outDirectoryIdentityV1("/w/b/release/build/serde_core/0123456789abcdef/out",lane)).toBe(outDirectoryIdentityV1("/elsewhere/b/debug/build/serde_core/fedcba9876543210/out",undefined));
 expect(outDirectoryIdentityV1("/w/b/release/build/serde_core/0123456789abcdef/out",lane)).toBe(outDirectoryIdentityV1("/elsewhere/t/debug/build/serde_core-fedcba9876543210/out",undefined));
 expect(outDirectoryIdentityV1("/a/build/serde/0123456789abcdef/out",undefined)).not.toBe(outDirectoryIdentityV1("/a/build/serde_core/0123456789abcdef/out",undefined));
 expect(outDirectoryIdentityV1("/a/somewhere/else",undefined)).toBe("/a/somewhere/else");
});
