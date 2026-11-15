import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import vectors from "./🔣️.json" with {type:"json"};
import {appleLinkV1,installNameArgumentsV1,linkRootArgumentsV1,machineNeutralRemapV1,objectRootArgumentsV1,remappedNameV1,remapsOfV1} from "../🟦️.ts";

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
 expect(installNameArgumentsV1(host,"darwin")).toEqual(["-Clink-arg=-Wl,-install_name,@rpath/libplug.dylib"]);expect(installNameArgumentsV1([...host,"--target","wasm32-wasip2"],"darwin")).toEqual([]);
 expect(linkRootArgumentsV1(native,"/out","linux")).toEqual([]);
});
