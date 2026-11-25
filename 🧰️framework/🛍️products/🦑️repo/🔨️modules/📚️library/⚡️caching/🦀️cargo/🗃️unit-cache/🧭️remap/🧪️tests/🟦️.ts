import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import vectors from "./🔣️.json" with {type:"json"};
import {appleLinkV1,canonicalOutDirectoryArgumentsV1,canonicalSourceArgumentsV1,installNameArgumentsV1,linkRootArgumentsV1,machineNeutralRemapV1,objectRootArgumentsV1,remappedNameV1,remapsOfV1,rustSourceRemapArgumentsV1} from "../🟦️.ts";

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

test("one source file spelled with and without `..` segments compiles to the same bytes after canonicalisation (oracle: rustc)",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"remap-spelling-")));
 try{
  mkdirSync(join(base,"module","packages","rust"),{recursive:true});writeFileSync(join(base,"module","lib.rs"),"pub fn f<T:Clone>(x:&T)->T{x.clone()}\npub const N:&str=file!();\n");
  const spelled="module/packages/rust/../../lib.rs",plain="module/lib.rs";
  const compile=(source:string,out:string):Buffer[]=>{
   mkdirSync(join(base,out),{recursive:true});
   const args=["--edition=2021","--crate-type","lib","--crate-name","spell","--emit=metadata,link","-Cmetadata=x","-Cextra-filename=-x","--remap-path-prefix="+base+"=.","--out-dir",out,source];
   const result=spawnSync("rustc",canonicalSourceArgumentsV1(args,source,base) as string[],{encoding:"utf8",cwd:base,env:{...process.env,RUSTC_BOOTSTRAP:"1"}});
   expect(result.status,result.stderr).toBe(0);
   return[readFileSync(join(base,out,"libspell-x.rlib")),readFileSync(join(base,out,"libspell-x.rmeta"))];
  };
  const raw=(source:string,out:string):Buffer[]=>{
   mkdirSync(join(base,out),{recursive:true});
   const result=spawnSync("rustc",["--edition=2021","--crate-type","lib","--crate-name","spell","--emit=metadata,link","-Cmetadata=x","-Cextra-filename=-x","--remap-path-prefix="+base+"=.","--out-dir",out,source],{encoding:"utf8",cwd:base,env:{...process.env,RUSTC_BOOTSTRAP:"1"}});
   expect(result.status,result.stderr).toBe(0);
   return[readFileSync(join(base,out,"libspell-x.rlib")),readFileSync(join(base,out,"libspell-x.rmeta"))];
  };
  const [rawSpelled,rawPlain]=[raw(spelled,"r1"),raw(plain,"r2")];
  expect(rawSpelled[0]!.equals(rawPlain[0]!),"control: the spelling alone changes the rlib").toBe(false);
  const [canonSpelled,canonPlain]=[compile(spelled,"c1"),compile(plain,"c2")];
  expect(canonSpelled[0]!.equals(canonPlain[0]!)).toBe(true);expect(canonSpelled[1]!.equals(canonPlain[1]!)).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("canonicalisation rewrites only a relative source whose normal form is the same file",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"remap-spelling-")));
 try{
  mkdirSync(join(base,"a","b"),{recursive:true});mkdirSync(join(base,"elsewhere","deep"),{recursive:true});
  writeFileSync(join(base,"a","x.rs"),"");writeFileSync(join(base,"x.rs"),"");writeFileSync(join(base,"elsewhere","x.rs"),"");symlinkSync(join(base,"elsewhere","deep"),join(base,"a","link"));
  const args=(source:string)=>["--crate-name","c",source,"--edition=2021"];
  expect(canonicalSourceArgumentsV1(args("a/b/../x.rs"),"a/b/../x.rs",base)).toEqual(args("a/x.rs"));
  expect(canonicalSourceArgumentsV1(args("./a/x.rs"),"./a/x.rs",base)).toEqual(args("a/x.rs"));
  expect(canonicalSourceArgumentsV1(args("a/x.rs"),"a/x.rs",base)).toEqual(args("a/x.rs"));
  expect(canonicalSourceArgumentsV1(args("a/link/../x.rs"),"a/link/../x.rs",base),"a link makes the lexical form another file").toEqual(args("a/link/../x.rs"));
  expect(canonicalSourceArgumentsV1(args(join(base,"a","b","..","x.rs")),join(base,"a","b","..","x.rs"),base)).toEqual(args(join(base,"a","b","..","x.rs")));
  expect(canonicalSourceArgumentsV1(args("missing/../x.rs"),"missing/../x.rs",base)).toEqual(args("missing/../x.rs"));
  expect(canonicalSourceArgumentsV1(args("a/x.rs"),"",base)).toEqual(args("a/x.rs"));
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("the generated-source directory of a build script is named one way in every compile, in the crate and in its dependent (oracle: rustc)",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"remap-out-")));
 try{
  mkdirSync(join(base,"src"),{recursive:true});
  writeFileSync(join(base,"src","d.rs"),'include!(concat!(env!("OUT_DIR"),"/gen.rs"));\npub trait T{fn t(&self)->u32;}\nimpl<X:Clone> T for X{fn t(&self)->u32{gen()}}\n');
  writeFileSync(join(base,"src","w.rs"),"extern crate d;\nuse d::T;\npub fn f()->u32{5u8.t()+String::new().t()}\n");
  const build=(hash:string,canonical:boolean):{d:Buffer;w:Buffer}=>{
   const outDir=join(base,"lane"+hash,"build","d",hash,"out"),out=join(base,"o"+hash+(canonical?"c":"r"));
   mkdirSync(outDir,{recursive:true});mkdirSync(out,{recursive:true});writeFileSync(join(outDir,"gen.rs"),"pub fn gen()->u32{1}\n");
   const given=["--edition=2021","--crate-type","lib","--crate-name","d","--emit=metadata","-Cmetadata=dm","-Cextra-filename=-dm","--remap-path-prefix="+join(base,"lane"+hash,"build")+"=/cargo/build-dir","--out-dir",out,"src/d.rs"],env={...process.env,OUT_DIR:outDir,RUSTC_BOOTSTRAP:"1"};
   const first=spawnSync("rustc",(canonical?canonicalOutDirectoryArgumentsV1(given,outDir):given) as string[],{encoding:"utf8",cwd:base,env});expect(first.status,first.stderr).toBe(0);
   const second=spawnSync("rustc",["--edition=2021","--crate-type","lib","--crate-name","w","--emit=metadata","-Cmetadata=wm","--extern","d="+join(out,"libd-dm.rmeta"),"--out-dir",out,"src/w.rs"],{encoding:"utf8",cwd:base,env});expect(second.status,second.stderr).toBe(0);
   return{d:readFileSync(join(out,"libd-dm.rmeta")),w:readFileSync(join(out,"libw.rmeta"))};
  };
  const [rawA,rawB]=[build("aaaa1111aaaa1111",false),build("bbbb2222bbbb2222",false)];
  expect(rawA.d.equals(rawB.d),"control: Cargo's hash directory alone changes the crate").toBe(false);expect(rawA.w.equals(rawB.w),"control: and its dependent").toBe(false);
  const [canonA,canonB]=[build("cccc3333cccc3333",true),build("dddd4444dddd4444",true)];
  expect(canonA.d.equals(canonB.d)).toBe(true);expect(canonA.w.equals(canonB.w)).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("the generated-source remapping is appended last, once, and only for an absolute OUT_DIR; the key sees it as a role",()=>{
 const args=["--crate-name","c","--remap-path-prefix=/b=/cargo/build-dir","x.rs"],out="/b/serde/0123456789abcdef/out";
 const once=canonicalOutDirectoryArgumentsV1(args,out);
 expect(once).toEqual([...args,"--remap-path-prefix="+out+"=/cargo/out-dir"]);
 expect(canonicalOutDirectoryArgumentsV1(once,out)).toEqual(once);
 expect(canonicalOutDirectoryArgumentsV1(args,undefined)).toEqual(args);expect(canonicalOutDirectoryArgumentsV1(args,"")).toEqual(args);expect(canonicalOutDirectoryArgumentsV1(args,"out")).toEqual(args);
 expect(canonicalOutDirectoryArgumentsV1(args,out+"/")).toEqual([...args,"--remap-path-prefix="+out+"=/cargo/out-dir"]);
 expect(remappedNameV1(remapsOfV1(once),out+"/private.rs")).toBe("/cargo/out-dir/private.rs");
 for(const from of["/b/x/1111111111111111/out","/Users/a/lane/build/y/2222222222222222/out"])expect(machineNeutralRemapV1("--remap-path-prefix="+from+"=/cargo/out-dir")).toBe("--remap-path-prefix=⟨cargo-out-dir⟩=/cargo/out-dir");
});
