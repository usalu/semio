/** 🦀️ Contract: the source set of a crate follows every module mount and include a compilation reads (a `#[path]` mount that leaves the package directory, an inline `mod` a macro emits, `include!`, `include_str!`, `include_bytes!`, `env!("CARGO_MANIFEST_DIR")`) exactly as `rustc --emit=dep-info` does, and refuses, with a named reason, what no static reading can follow (a macro that mounts a file, a conditional mount, a macro-variable include, a build script) instead of falling back to the package directory, which misses every mounted file outside it. rustc is the independent oracle of every row.
 * @see ./🔣️.json */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import factSchema from "../../🦀️native-manifests/🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,libraryBootstrap}=inference as any;
await libraryBootstrap;
const ajv=new Ajv({strict:false}),slash=(path:string)=>path.split("\\").join("/"),prefix="{workspaceRoot}/";
const build=(root:string,files:Record<string,string>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const entriesOf=(root:string,crate:string):string[]=>{
 const directory=join(root,crate),manifest=readFileSync(join(directory,"Cargo.toml"),"utf8"),lib=/^path\s*=\s*"([^"]+)"/mu.exec(manifest.split("[lib]")[1]??"")?.[1]??"src/lib.rs",found=[lib];
 for(const folder of ["tests"])if(existsSync(join(directory,folder)))found.push(...["t.rs"].filter(name=>existsSync(join(directory,folder,name))).map(name=>`${folder}/${name}`));
 return found.filter(entry=>existsSync(join(directory,entry)));
};
const rustc=(root:string,crate:string,flags:string[]):string[]=>{
 const reads=new Set<string>(),directory=join(root,crate);
 for(const entry of entriesOf(root,crate)){
  const out=join(root,"oracle.d");
  rmSync(out,{force:true});
  spawnSync("rustc",["--edition=2021","--crate-type=lib","--crate-name=probe","--cap-lints=allow",...flags,`--emit=dep-info=${out}`,entry],{cwd:directory,encoding:"utf8",env:{...process.env,CARGO_MANIFEST_DIR:directory}});
  expect(existsSync(out),`rustc wrote no dep-info for ${crate}/${entry}`).toBe(true);
  for(const path of readFileSync(out,"utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/u))reads.add(slash(relative(root,resolve(directory,path))));
 }
 return [...reads].sort();
};

test("the corpus is closed by its schema",()=>{
 expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 const resolved=corpus.cases.find(row=>row.resolved)!,refused=corpus.cases.find(row=>!row.resolved)!;
 expect(ajv.validate(schema,{...corpus,cases:[{...resolved,reason:"build-script"}]}),"a resolved row names no reason").toBe(false);
 expect(ajv.validate(schema,{...corpus,cases:[{...refused,expected:["a"]}]}),"a refused row lists no files").toBe(false);
});

for(const row of corpus.cases)test(`${row.name}: ${row.mechanism}`,()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-mounted-"));
 try{
  build(root,row.files);
  const named=row.flavor==="named",set=cacheInternals.cargoSourceSetV1(row.crate,root,new Map(),named,!named);
  expect(ajv.validate(factSchema,set),JSON.stringify(ajv.errors)).toBe(true);
  const read=named?rustc(root,row.crate,["--test"]):[],outside=read.filter(path=>!path.startsWith(row.crate+"/")).sort();
  if(named)expect(outside,"files rustc reads outside the package directory").toEqual([...row.outside].sort());
  if(row.resolved){
   const derived=set.files.map((path:string)=>path.slice(prefix.length)).sort();
   expect(set.resolved).toBe(true);
   expect(derived).toEqual([...row.expected!].sort());
   expect(derived,"the derivation is exactly what rustc reads").toEqual(read);
   expect(set.emitters?.map((path:string)=>path.slice(prefix.length))??[]).toEqual(row.emitters??[]);
   expect(cacheInternals.cargoSourcesOrRefuseV1(row.crate,root,new Map(),named,!named).map((path:string)=>path.slice(prefix.length)).sort()).toEqual(derived);
  }else{
   expect(set.resolved).toBe(false);
   expect(set.reason).toBe(row.reason);
   const asked=()=>cacheInternals.cargoSourcesOrRefuseV1(row.crate,root,new Map(),named,!named);
   if(row.reason==="no-entries")expect(asked()).toBeUndefined();
   else{
    let thrown:any;
    try{asked();}catch(error){thrown=error;}
    expect(thrown?.key,"an unresolved crate refuses the precision derivation").toBe("build.graph.rust-sources-unresolved");
    expect(cacheInternals.cargoSourceInputs(row.crate,root,new Map(),named,!named),"the default pipeline keeps its directory and owner fallback").toBeUndefined();
   }
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a tests-only crate has no compiled entry as a dependency but its test targets as a root",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-tests-only-"));
 try{
  const row=corpus.cases.find(candidate=>candidate.name.startsWith("tests-only"))!;
  build(root,row.files);
  const set=cacheInternals.cargoSourceSetV1(row.crate,root,new Map(),true,false);
  expect(set.resolved).toBe(true);
  expect(set.files).toEqual([prefix+row.crate+"/tests/t.rs"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const workspace=(()=>{let at=import.meta.dir;while(at!==dirname(at)&&!existsSync(join(at,"rust-toolchain.toml")))at=dirname(at);return at;})();
const retained="🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/📦️packages/🦀️rust";
test.skipIf(!existsSync(join(workspace,retained,"Cargo.toml")))("the retained mutation crate of this repository resolves to its mounted owner tree, exactly the files rustc reads",()=>{
 const set=cacheInternals.cargoSourceSetV1(retained,workspace,new Map(),false,true);
 expect(set.resolved,JSON.stringify(set)).toBe(true);
 const derived=new Set<string>(set.files.map((path:string)=>path.slice(prefix.length))),directory=join(workspace,retained),out=join(tmpdir(),`rust-mounted-${process.pid}.d`);
 spawnSync("rustc",["--edition=2021","--crate-type=lib","--crate-name=probe","--cap-lints=allow",`--emit=dep-info=${out}`,join(directory,"🦀️.rs")],{cwd:tmpdir(),encoding:"utf8"});
 try{
  const reads=readFileSync(out,"utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/u).map(path=>slash(relative(workspace,resolve(directory,path))));
  expect(reads.length).toBeGreaterThan(8);
  for(const path of reads)expect(derived.has(path),`rustc reads ${path}`).toBe(true);
  for(const mounted of ["✂️span","🔎️key","🔺️diff","🩹️patch","⚠️diagnostic"])expect(derived.has(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/${mounted}/🦀️.rs`),mounted).toBe(true);
 }finally{rmSync(out,{force:true});}
});

const femNode="✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node";
test("dimensional FEM node source mounts resolve without macro expansion",()=>{
 const set=cacheInternals.cargoSourceSetV1(femNode+"/📦️packages/🦀️rust",workspace,new Map(),true,false);
 expect(set.resolved,JSON.stringify(set)).toBe(true);
 for(const path of ["🧬️schema/◻️2d/🦀️.rs","🧬️schema/🧊️3d/🦀️.rs","🧪️tests/🦀️.rs","🧪️tests/🧫️fixtures/🔣️.json"])expect(set.files).toContain(prefix+femNode+"/"+path);
});
