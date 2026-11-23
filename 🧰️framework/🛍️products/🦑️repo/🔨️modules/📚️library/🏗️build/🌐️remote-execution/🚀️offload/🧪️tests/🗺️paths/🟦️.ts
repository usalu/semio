import {expect,test} from "bun:test";
import {readdirSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,isAbsolute,join} from "node:path";
import {environmentReadNamesV1,exactReadsV1,includedSourcesV1,planOffloadV1} from "../../🗺️paths/🟦️.ts";
import {parseDepInfoV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";

const SERDE_CORE_LAYOUT:Record<string,string>={
 "lib.rs":'#[macro_use]\nmod crate_root;\n#[macro_use]\nmod macros;\ncrate_root!();\n',
 "crate_root.rs":'macro_rules! crate_root {\n    () => {\n        mod lib { mod core { pub use core::fmt; } }\n        #[cfg_attr(all(docsrs, if_docsrs_then_no_serde_core), path = "core/de/mod.rs")]\n        pub mod de;\n        #[cfg_attr(all(docsrs, if_docsrs_then_no_serde_core), path = "core/ser/mod.rs")]\n        pub mod ser;\n        mod format;\n        #[doc(hidden)]\n        #[cfg_attr(\n            all(docsrs, if_docsrs_then_no_serde_core),\n            path = "core/private/mod.rs"\n        )]\n        mod private;\n        include!(concat!(env!("OUT_DIR"), "/private.rs"));\n    };\n}\n',
 "macros.rs":"\n","format.rs":"\n",
 "de/mod.rs":"pub mod value;\nmod ignored_any;\nmod impls;\n","de/value.rs":"\n","de/ignored_any.rs":"\n","de/impls.rs":"\n",
 "ser/mod.rs":"mod impls;\n","ser/impls.rs":"\n","private/mod.rs":"mod doc;\npub use self::doc::*;\n","private/doc.rs":"\n"
};

function serdeCoreFixture(root:string):{entry:string;out:string}{
 mkdirSync(join(root,"src"),{recursive:true});
 for(const [name,text] of Object.entries(SERDE_CORE_LAYOUT)){mkdirSync(dirname(join(root,"src",name)),{recursive:true});writeFileSync(join(root,"src",name),text);}
 mkdirSync(join(root,"out"));writeFileSync(join(root,"out","private.rs"),"\n");
 writeFileSync(join(root,"Cargo.toml"),'[package]\nname = "serde_core"\nversion = "1.0.0"\n');
 return{entry:join(root,"src","lib.rs"),out:join(root,"out")};
}

test("a module declared inside a macro_rules body that another file expands (serde_core: crate_root! in lib.rs declares de, ser, format, private) resolves beside the expanding file: every file rustc reads is reached, and a recorded gap or the dependency file of an earlier compilation ships what the scan cannot know",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-serde-core-")));
 try{
  const {entry,out}=serdeCoreFixture(root),info=join(root,"lib.d");
  const done=spawnSync("rustc",["--edition=2021","--crate-type=lib","--crate-name=serde_core",`--emit=dep-info=${info}`,entry],{encoding:"utf8",cwd:root,env:testRustEnvironmentV1({CARGO_MANIFEST_DIR:root,OUT_DIR:out})});
  expect(done.status,done.stderr).toBe(0);
  const oracle=parseDepInfoV1(readFileSync(info,"utf8").replace(/\\\r?\n/gu," ")).dependencies.map(path=>isAbsolute(path)?path:join(root,path)).filter(path=>path!==entry&&path.startsWith(root+"/src"));
  expect(oracle.length).toBeGreaterThan(8);
  const reached=includedSourcesV1(entry,undefined,undefined,root);
  for(const path of oracle)expect(reached,path).toContain(path);
  const planned=planOffloadV1({args:["--crate-name","serde_core","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${root}=.`],environment:{CARGO_MANIFEST_DIR:root,CARGO_PKG_NAME:"serde_core",CARGO_PKG_VERSION:"1.0.0",CARGO_CRATE_NAME:"serde_core"},cwd:root});
  if("declined" in planned)throw Error(planned.declined+" "+planned.detail);
  const shipped=[...planned.files.values()];
  for(const path of oracle)expect(shipped,path).toContain(path);
  writeFileSync(join(root,"src","stray.rs"),"\n");
  const withGap=planOffloadV1({args:["--crate-name","serde_core","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${root}=.`],environment:{CARGO_MANIFEST_DIR:root,CARGO_PKG_NAME:"serde_core",CARGO_PKG_VERSION:"1.0.0",CARGO_CRATE_NAME:"serde_core"},cwd:root,recorded:[join(root,"src","stray.rs"),join(root,"src","missing.rs")]});
  if("declined" in withGap)throw Error(withGap.declined+" "+withGap.detail);
  expect([...withGap.files.values()]).toContain(join(root,"src","stray.rs"));
  expect([...withGap.files.values()]).not.toContain(join(root,"src","missing.rs"));
  expect(exactReadsV1(join(root,"nothing.d"),root,[])).toEqual([]);
  expect(exactReadsV1(info,root,[]).sort()).toEqual(expect.arrayContaining(oracle));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the real serde_core crate of the cargo registry: the scan reaches every file that rustc reads",()=>{
 const registry=join(process.cwd(),".cache","b","r","m","cargo","home","registry","src");
 let crate:string|undefined;try{for(const index of readdirSync(registry)){const found=readdirSync(join(registry,index)).find(name=>name.startsWith("serde_core-"));if(found!==undefined){crate=join(registry,index,found);break;}}}catch{}
 if(crate===undefined)return;
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-serde-real-")));
 try{
  mkdirSync(join(root,"out"));writeFileSync(join(root,"out","private.rs"),"\n");
  const entry=join(crate,"src","lib.rs"),info=join(root,"lib.d");
  const done=spawnSync("rustc",["--edition=2021","--crate-type=lib","--crate-name=serde_core","--cfg",'feature="std"',"--cfg",'feature="alloc"',`--emit=dep-info=${info}`,entry],{encoding:"utf8",cwd:root,env:testRustEnvironmentV1({CARGO_MANIFEST_DIR:crate,OUT_DIR:join(root,"out")})});
  expect(done.status,done.stderr).toBe(0);
  const oracle=parseDepInfoV1(readFileSync(info,"utf8").replace(/\\\r?\n/gu," ")).dependencies.filter(path=>path.startsWith(crate+"/")&&path!==entry);
  expect(oracle.length).toBeGreaterThan(8);
  const reached=includedSourcesV1(entry,undefined,undefined,crate);
  for(const path of oracle)expect(reached,path).toContain(path);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("variables the compilation does not read never block or travel: NATIVE_PLAY_OUT and __CARGO_TEST_ROOT with a checkout path are dropped silently, while a variable the source reads with env! that carries the checkout path still declines unmapped-path",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-env-reads-")));
 try{
  mkdirSync(join(root,"src"));
  writeFileSync(join(root,"Cargo.toml"),"[package]\nname = \"caller\"\nversion = \"0.1.0\"\n");
  writeFileSync(join(root,"src","lib.rs"),'pub const A: Option<&str> = option_env!("READ_ME");\n');
  const environment={CARGO_MANIFEST_DIR:root,CARGO_PKG_NAME:"caller",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"caller",NATIVE_PLAY_OUT:join(root,"out","play"),__CARGO_TEST_ROOT:join(root,"x"),SHELL_NOISE:"plain"};
  const args=["--crate-name","caller","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${root}=.`];
  const planned=planOffloadV1({args,environment,cwd:root});
  if("declined" in planned)throw Error(planned.declined+" "+planned.detail);
  expect(Object.keys(planned.environment)).not.toContain("NATIVE_PLAY_OUT");
  expect(Object.keys(planned.environment)).not.toContain("__CARGO_TEST_ROOT");
  expect(Object.keys(planned.environment)).not.toContain("SHELL_NOISE");
  expect(planned.environment.CARGO_PKG_NAME).toBe("caller");
  const read=planOffloadV1({args,environment:{...environment,READ_ME:join(root,"out")},cwd:root});
  expect("declined" in read?read.declined:"planned").toBe("unmapped-path");
  const harmless=planOffloadV1({args,environment:{...environment,READ_ME:"value"},cwd:root});
  if("declined" in harmless)throw Error(harmless.declined+" "+harmless.detail);
  expect(harmless.environment.READ_ME).toBe("value");
  expect([...environmentReadNamesV1([join(root,"src","lib.rs")])]).toEqual(["READ_ME"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a dependency of the earlier compilation inside OUT_DIR (a lane target directory outside the package: serde_core's private.rs) is already shipped and mapped, so it never declines unmapped-path",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-lane-dep-")));
 try{
  const pkg=join(root,"pkg"),generated=join(root,"lane","release","build","serde_core","abc","out");
  mkdirSync(join(pkg,"src"),{recursive:true});mkdirSync(generated,{recursive:true});mkdirSync(join(pkg,"out"));
  writeFileSync(join(pkg,"Cargo.toml"),"[package]\nname = \"caller\"\nversion = \"0.1.0\"\n");
  writeFileSync(join(pkg,"src","lib.rs"),'include!(concat!(env!("OUT_DIR"), "/private.rs"));\n');
  writeFileSync(join(generated,"private.rs"),"\n");
  writeFileSync(join(pkg,"out","caller.d"),`out/caller.rmeta: src/lib.rs ${join(generated,"private.rs")}\n`);
  const environment={CARGO_MANIFEST_DIR:pkg,CARGO_PKG_NAME:"caller",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"caller",OUT_DIR:generated};
  const planned=planOffloadV1({args:["--crate-name","caller","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${pkg}=.`,"--remap-path-prefix",`${root}=/r`],environment,cwd:pkg});
  if("declined" in planned)throw Error(planned.declined+" "+planned.detail);
  expect([...planned.files.values()]).toContain(join(generated,"private.rs"));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the WIT and witx files of a bindgen!, generate! or from_witx! macro are reached: every file below it ships next to the source, because rustc reads it only through the macro and a first compilation has no dependency file",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-wit-")));
 try{
  mkdirSync(join(root,"src"));mkdirSync(join(root,"wit","deps"),{recursive:true});mkdirSync(join(root,"other"));
  writeFileSync(join(root,"Cargo.toml"),'[package]\nname = "bindings"\nversion = "0.1.0"\n');
  writeFileSync(join(root,"src","lib.rs"),'wasmtime::component::bindgen!({\n    path: "wit",\n    with: { "a:b/c": crate::C },\n});\nwit_bindgen::generate!({ world: "w", path: ["other/w.wit"] });\nwiggle::from_witx!({\n    witx: ["witx/p0/a.witx"],\n    async: { a::* },\n});\n');
  mkdirSync(join(root,"witx","p0"),{recursive:true});writeFileSync(join(root,"witx","p0","a.witx"),"use \"b.witx\"\n");writeFileSync(join(root,"witx","p0","b.witx"),";;\n");
  writeFileSync(join(root,"wit","world.wit"),"package a:b;\n");writeFileSync(join(root,"wit","deps","io.wit"),"package c:d;\n");writeFileSync(join(root,"other","w.wit"),"package e:f;\n");
  const entry=join(root,"src","lib.rs"),reached=includedSourcesV1(entry,undefined,undefined,root);
  expect([...reached].sort()).toEqual([join(root,"other","w.wit"),join(root,"wit","deps","io.wit"),join(root,"wit","world.wit"),join(root,"witx","p0","a.witx"),join(root,"witx","p0","b.witx")]);
  expect(includedSourcesV1(entry)).toEqual([]);
  const planned=planOffloadV1({args:["--crate-name","bindings","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${root}=.`],environment:{CARGO_MANIFEST_DIR:root,CARGO_PKG_NAME:"bindings",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"bindings"},cwd:root});
  if("declined" in planned)throw Error(planned.declined+" "+planned.detail);
  for(const path of reached)expect([...planned.files.values()]).toContain(path);
 }finally{rmSync(root,{recursive:true,force:true});}
});
