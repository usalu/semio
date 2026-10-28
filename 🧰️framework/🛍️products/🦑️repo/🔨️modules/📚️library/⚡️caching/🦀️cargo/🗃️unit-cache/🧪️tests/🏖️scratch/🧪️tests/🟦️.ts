import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import scratch from "../🔣️.json" with {type:"json"};
import {cargoAncestorFileV1,cargoScratchBaseV1} from "../🟦️.ts";

/** 🧱️ Scratch Cargo workspaces of the unit-cache tests sit where no enclosing configuration, workspace manifest or toolchain file reaches them. The language-agnostic table names each file and what it plants; Cargo and rustup themselves are the oracle: a build below a reached directory obeys the planted file, a build below the chosen base does not.
 * @see ../🟦️.ts */
const root=realpathSync(mkdtempSync(join(tmpdir(),"uc-scratch-"))),cargo=process.env.SEMIO_TEST_CARGO??"cargo",isolated={CARGO_HOME:join(root,"cargo-home")};
const rustup=spawnSync("rustup",["--version"],{encoding:"utf8"}).status===0;
const plant=(directory:string,row:(typeof scratch.ancestorFiles)[number]):void=>{const path=join(directory,row.path),plan=scratch.plants[row.kind as keyof typeof scratch.plants];mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.path==="rust-toolchain"&&"legacyText" in plan?plan.legacyText:plan.text);};
const crate=(base:string):string=>{const directory=mkdtempSync(join(base,"ws-"));mkdirSync(join(directory,"src"));writeFileSync(join(directory,"Cargo.toml"),'[package]\nname = "scratch"\nversion = "0.1.0"\nedition = "2021"\n');writeFileSync(join(directory,"src","lib.rs"),"pub fn f() -> u8 { 1 }\n");return directory;};
const build=(directory:string)=>{const environment=Object.fromEntries(Object.entries(process.env).filter(([name])=>!/^(RUSTUP_TOOLCHAIN|RUSTC(_.*)?|RUSTFLAGS|CARGO_(BUILD_.*|TARGET_DIR|ENCODED_RUSTFLAGS|INCREMENTAL|UNSTABLE_.*))$/u.test(name)));return spawnSync(cargo,["build","--offline"],{cwd:directory,env:{...environment,RUSTUP_AUTO_INSTALL:"0",CARGO_TERM_COLOR:"never"},encoding:"utf8"});};

test("the system temporary directory is outside Cargo's reach, so the tables below start clean",()=>{
 expect(cargoAncestorFileV1(root,isolated)).toBeUndefined();
});

test("each enclosing file reaches a directory below it, the base is the first candidate nothing reaches, and a run reached everywhere is refused naming every file",()=>{
 const free=join(root,"free");
 scratch.ancestorFiles.forEach((row,index)=>{
  const covered=join(root,"r"+index,"co"),art=join(covered,"deep","art");plant(covered,row);
  expect(cargoAncestorFileV1(art,isolated)).toBe(join(covered,row.path));
  expect(cargoScratchBaseV1([art,free],isolated)).toBe(resolve(free));
  expect(cargoScratchBaseV1([undefined," ",free,art],isolated)).toBe(resolve(free));
  expect(()=>cargoScratchBaseV1([art],isolated)).toThrow(join(covered,row.path));
 });
},30_000);

test("a run reached through every candidate is refused naming each file in order, and an artifact directory in the clear is the base itself",()=>{
 const covered=join(root,"refuse","co"),other=join(root,"refuse","other"),clear=join(root,"clear","art");plant(covered,scratch.ancestorFiles[0]!);plant(other,scratch.ancestorFiles[2]!);
 expect(()=>cargoScratchBaseV1([join(covered,"art"),join(other,"tmp")],isolated)).toThrow(join(covered,".cargo/config.toml")+", "+join(other,"Cargo.toml"));
 expect(cargoScratchBaseV1([clear,join(root,"unused")],isolated)).toBe(clear);
});

test("Cargo's home configuration does not count, because Cargo reads it wherever it runs; the same file under any other home does",()=>{
 const home=join(root,"home"),art=join(home,"project","art");plant(home,scratch.ancestorFiles[0]!);
 expect(cargoAncestorFileV1(art,{CARGO_HOME:join(home,".cargo")})).toBeUndefined();
 expect(cargoAncestorFileV1(art,{CARGO_HOME:join(root,"elsewhere")})).toBe(join(home,".cargo/config.toml"));
});

for(const[index,row]of scratch.ancestorFiles.entries())(row.kind==="toolchain"?test.skipIf(!rustup):test)(`Cargo itself obeys a planted ${row.path} below the reached directory and not below the chosen base`,()=>{
 const covered=join(root,"o"+index,"co"),art=join(covered,"art"),evidence=scratch.plants[row.kind as keyof typeof scratch.plants].evidence;plant(covered,row);mkdirSync(art,{recursive:true});
 const reached=build(crate(art));expect(reached.status).not.toBe(0);expect(reached.stdout+reached.stderr).toContain(evidence);
 const chosen=build(crate(cargoScratchBaseV1([art,join(root,"o"+index,"free")])));expect(chosen.status,chosen.stderr).toBe(0);
},120_000);
