import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetFetchContextV1} from "../../🟦️.ts";
import {cargoLockRegistryCratesV1,cargoRegistryCacheDirectoryV1,fleetCargoRegistryAroundV1,fleetCratesPublishV1,fleetCratesRestoreV1} from "../🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-crates-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const run=async(argv:string[],cwd:string,env:Record<string,string>={})=>{const child=Bun.spawn(argv,{cwd,stdout:"pipe",stderr:"pipe",env:{...process.env,...env}}),[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);return {out,err,code};};

test("the lock reader agrees with Bun's own TOML parser on the repository lockfile",()=>{
 const text=readFileSync(join(import.meta.dir,"../../../../../../../../../Cargo.lock"),"utf8"),ours=cargoLockRegistryCratesV1(text);
 const parsed=Bun.TOML.parse(text) as {package:{name:string;version:string;source?:string;checksum?:string}[]};
 const theirs=parsed.package.filter(row=>row.source==="registry+https://github.com/rust-lang/crates.io-index"&&row.checksum).map(row=>({name:row.name,version:row.version,sha256:row.checksum!}));
 expect(ours.length).toBeGreaterThan(100);expect(ours).toEqual(theirs);
});

test("the cache directory is the one Cargo made, else the sparse crates.io name",async()=>{
 const home=directory("home");expect(await cargoRegistryCacheDirectoryV1(home)).toBe(join(home,"registry","cache","index.crates.io-1949cf8c6b5b557f"));
 mkdirSync(join(home,"registry","cache","index.crates.io-0123456789abcdef"),{recursive:true});expect(await cargoRegistryCacheDirectoryV1(home)).toBe(join(home,"registry","cache","index.crates.io-0123456789abcdef"));
});

test("a second Cargo home takes the locked crate files from the fleet; the real cargo downloads none of them",async()=>{
 const probe=await fetch("https://index.crates.io/config.json",{signal:AbortSignal.timeout(8000)}).then(response=>response.status,()=>0);
 if(probe!==200)return;
 const project=directory("project"),store=directory("store");mkdirSync(join(project,"src"));
 writeFileSync(join(project,"Cargo.toml"),'[package]\nname="probe"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nitoa="=1.0.11"\nryu="=1.0.18"\n');writeFileSync(join(project,"src","main.rs"),"fn main(){}\n");
 const one=directory("home-one"),two=directory("home-two");
 const context=fleetFetchContextV1({},project,{storeRoot:store});
 const first=await run(["cargo","fetch","-v"],project,{CARGO_HOME:one});expect(first.code,first.err).toBe(0);expect(first.err+first.out).toContain("Downloaded");
 const pins=cargoLockRegistryCratesV1(readFileSync(join(project,"Cargo.lock"),"utf8"));expect(pins.map(pin=>pin.name).sort()).toEqual(["itoa","ryu"]);
 expect(await fleetCratesPublishV1(context,one,pins)).toBe(2);expect(await fleetCratesPublishV1(context,one,pins)).toBe(0);
 const restored=await fleetCratesRestoreV1(context,two,pins);expect(restored).toEqual({placed:2,present:0,missing:0});
 const second=await run(["cargo","fetch","--locked","-v"],project,{CARGO_HOME:two});expect(second.code,second.err).toBe(0);
 expect(second.err+second.out).not.toContain("Downloaded");
 const files=readdirSync(await cargoRegistryCacheDirectoryV1(two)).sort();expect(files).toEqual(["itoa-1.0.11.crate","ryu-1.0.18.crate"]);
},120000);

test("the wrapper restores before and publishes after the fetch it runs, and swallows a fleet that fails",async()=>{
 const project=directory("project"),store=directory("store"),home=directory("home");
 writeFileSync(join(project,"Cargo.toml"),'[package]\nname="p"\nversion="0.1.0"\n');
 const sha="a".repeat(64);writeFileSync(join(project,"Cargo.lock"),`version = 4\n\n[[package]]\nname = "x"\nversion = "1.0.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "${sha}"\n`);
 const context=fleetFetchContextV1({},project,{storeRoot:store});let ran=0;
 expect(await fleetCargoRegistryAroundV1(context,{home,manifest:join(project,"Cargo.toml"),workspace:project},async()=>{ran++;return "done";})).toBe("done");expect(ran).toBe(1);
 const broken=fleetFetchContextV1({},project,{storeRoot:"/nonexistent/store"});
 expect(await fleetCargoRegistryAroundV1(broken,{home,manifest:join(project,"Cargo.toml"),workspace:project},async()=>"ok")).toBe("ok");
});
