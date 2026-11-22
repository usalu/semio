import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import vectors from "./🔣️.json" with {type:"json"};
import {appleLinkV1,installNameArgumentsV1,linkRootArgumentsV1,machineNeutralRemapV1,objectRootArgumentsV1,remappedNameV1,remapsOfV1,rustSourceRemapArgumentsV1} from "../🟦️.ts";

const fromSpec=(specs:readonly string[])=>remapsOfV1(specs.map(spec=>"--remap-path-prefix="+spec));
test("every vector gives the recorded name (TypeScript model)",()=>{for(const row of vectors.cases)expect(remappedNameV1(fromSpec(row.remaps),row.path),row.label).toBe(row.name);});
test("an independent Python model gives the same names as the vectors and the TypeScript model",()=>{
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const rows=run.stdout.trim().split("\n").map(line=>JSON.parse(line) as {label:string;name:string});
 expect(rows.length).toBe(vectors.cases.length);
 rows.forEach((row,index)=>{expect(row.name,row.label).toBe(vectors.cases[index]!.name);});
});
test("both spellings of the flag and a path with equals signs",()=>{
 expect(remapsOfV1(["--remap-path-prefix","/a=b","--remap-path-prefix=/c=d=e","--other"])).toEqual([{from:"/a",to:"b"},{from:"/c=d",to:"e"}]);
});
test("the machine roots of a remapping are replaced by their role, everything else is kept",()=>{
 expect(machineNeutralRemapV1("--remap-path-prefix=/Users/a/.cargo/registry/src=")).toBe("--remap-path-prefix=⟨cargo-registry-src⟩=");
 expect(machineNeutralRemapV1("--remap-path-prefix=/home/b/c/registry/src=")).toBe("--remap-path-prefix=⟨cargo-registry-src⟩=");
 expect(machineNeutralRemapV1("--remap-path-prefix=/x/.cargo/git/checkouts=")).toBe("--remap-path-prefix=⟨cargo-git-checkouts⟩=");
 for(const from of["/tmp/r16-odd-target","/Users/a/lane/t-ce555540","C:\\work\\target"])expect(machineNeutralRemapV1("--remap-path-prefix="+from+"=/cargo/build-dir")).toBe("--remap-path-prefix=⟨cargo-build-dir⟩=/cargo/build-dir");
 expect(machineNeutralRemapV1("--remap-path-prefix=/work=/cargo/other")).toBe("--remap-path-prefix=/work=/cargo/other");
 expect(machineNeutralRemapV1("--remap-path-prefix=/u/.rustup/toolchains/t/lib/rustlib/src/rust=/rustc/abc")).toBe("--remap-path-prefix=⟨rust-src⟩=/rustc/abc");
 expect(machineNeutralRemapV1("--remap-path-prefix=/w=.")).toBe("--remap-path-prefix=/w=.");
 expect(machineNeutralRemapV1("--edition=2021")).toBe("--edition=2021");
});
test("the compiler itself records the names the model predicts (oracle: rustc)",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"remap-oracle-")));
 try{
  for(const [index,row] of vectors.cases.entries()){
   const remaps=row.remaps.map(spec=>{const cut=spec.lastIndexOf("=");const from=spec.slice(0,cut);return from.startsWith("/")?join(base,from)+spec.slice(cut):spec;});
   const root=join(base,String(index));mkdirSync(join(root,row.path.startsWith("/")?row.path.split("/").slice(0,-1).join("/"):"."),{recursive:true});
   const full=join(root,row.path);mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"pub const N:&str=file!();\n");
   const rooted=row.remaps.map(spec=>{const cut=spec.lastIndexOf("=");return"--remap-path-prefix="+root+spec.slice(0,cut)+spec.slice(cut);});
   const result=spawnSync("rustc",["--edition=2021","--crate-type","lib","--emit=metadata","--remap-path-scope=all",...rooted,"-o",join(root,"o.rmeta"),full],{encoding:"utf8",env:{...process.env,RUSTUP_TOOLCHAIN:"nightly-2026-07-20"}});
   if(result.status!==0)continue;
   const strings=spawnSync("strings",["-a",join(root,"o.rmeta")],{encoding:"utf8"}).stdout;
   const expected=remappedNameV1(fromSpec(row.remaps.map(spec=>{const cut=spec.lastIndexOf("=");return root+spec.slice(0,cut)+spec.slice(cut);})),full);
   if(expected!==""&&!/^\/(?:private\/)?var/.test(expected))expect(strings.includes(expected),`${row.label}: ${expected}`).toBe(true);
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("Apple's linker options are given to Mach-O links only: never to a wasm component, always to host units and macOS targets",()=>{
 const wasm=["--crate-name","plug","--crate-type","cdylib","--target","wasm32-wasip2"],host=["--crate-name","plug","--crate-type","proc-macro"],native=["--crate-type","bin","--target","aarch64-apple-darwin"],other=["--crate-type","cdylib","--target","x86_64-unknown-linux-gnu"];
 expect([appleLinkV1(wasm),appleLinkV1(host),appleLinkV1(native),appleLinkV1(other),appleLinkV1(["--target=wasm32-unknown-unknown"])]).toEqual([false,true,true,false,false]);
 expect(linkRootArgumentsV1(wasm,"/out","darwin")).toEqual([]);expect(linkRootArgumentsV1(other,"/out","darwin")).toEqual([]);
 expect(objectRootArgumentsV1(native,"/out/","darwin")).toEqual(["-Clink-arg=-Wl,-oso_prefix,/out/"]);expect(objectRootArgumentsV1(["--crate-type","cdylib"],"/out","darwin")).toEqual(["-Clink-arg=-Wl,-oso_prefix,/out/"]);
 expect(installNameArgumentsV1(host,"darwin")).toEqual(["-Clink-arg=-Wl,-install_name,@rpath/libplug.dylib"]);
 for(const type of ["cdylib","dylib"])expect(installNameArgumentsV1(["--crate-name","plug","--crate-type",type,"-C","extra-filename=-ab12"],"darwin"),type).toEqual(["-Clink-arg=-Wl,-install_name,@rpath/libplug-ab12.dylib"]);
 for(const type of ["bin","staticlib","lib"])expect(installNameArgumentsV1(["--crate-name","plug","--crate-type",type],"darwin"),type).toEqual([]);expect(installNameArgumentsV1([...host,"--target","wasm32-wasip2"],"darwin")).toEqual([]);
 expect(linkRootArgumentsV1(native,"/out","linux")).toEqual([]);
});

test("the standard library source is remapped for every unit: the rule is stated once, a remap that exists is not repeated, and the compiler itself records no sysroot path with the flag (oracle: rustc)",()=>{
 const sysroot="/Users/a/.rustup/toolchains/t",commit="9f36de775bc636c8e88c31a173c2bcb6995956a0",from=sysroot+"/lib/rustlib/src/rust";
 expect(rustSourceRemapArgumentsV1(["--crate-name","x"],sysroot,commit)).toEqual([`--remap-path-prefix=${from}=/rustc/${commit}`]);
 expect(rustSourceRemapArgumentsV1(["--remap-path-prefix="+from+"=/rustc/"+commit],sysroot+"/",commit)).toEqual([]);
 expect(rustSourceRemapArgumentsV1([],undefined,commit)).toEqual([]);expect(rustSourceRemapArgumentsV1([],sysroot,undefined)).toEqual([]);
 expect(machineNeutralRemapV1(rustSourceRemapArgumentsV1([],sysroot,commit)[0]!)).toBe(`--remap-path-prefix=⟨rust-src⟩=/rustc/${commit}`);
 const environment={...process.env,RUSTUP_TOOLCHAIN:"nightly-2026-07-20"},probe=spawnSync("rustc",["--print","sysroot"],{encoding:"utf8",env:environment}),real=probe.stdout.trim(),hash=/^commit-hash: ([0-9a-f]{40})$/mu.exec(spawnSync("rustc",["-vV"],{encoding:"utf8",env:environment}).stdout)?.[1];
 if(probe.status!==0||hash===undefined||!existsSync(join(real,"lib","rustlib","src","rust","library")))return;
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"rust-src-")));
 try{
  writeFileSync(join(base,"l.rs"),"pub fn f(x:u32)->u32{ x.wrapping_pow(3).rotate_left(5).saturating_add(x.pow(2)) }\npub fn g(v:&[u32])->u32{ v.iter().map(|x|x.wrapping_mul(3)).sum() }\n");
  const build=(extra:readonly string[],out:string)=>{mkdirSync(join(base,out));const run=spawnSync("rustc",["--crate-name","l","--edition=2021","l.rs","--crate-type","lib","--emit=link","-C","debuginfo=2","-C","opt-level=1","-C","metadata=a","-C","extra-filename=-a","--remap-path-scope=all",`--remap-path-prefix=${base}=.`,...extra,"--out-dir",out],{cwd:base,encoding:"utf8",env:environment});expect(run.status,run.stderr).toBe(0);return readFileSync(join(base,out,"libl-a.rlib"));};
  const plain=build([],"plain"),mapped=build(rustSourceRemapArgumentsV1([],real,hash),"mapped");
  expect(plain.includes(Buffer.from(real)),"without the flag the sysroot path is in the unit").toBe(true);expect(mapped.includes(Buffer.from(real)),"with the flag no path of the machine is left").toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});
