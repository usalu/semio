import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join} from "node:path";
import {wrapRustcV1} from "../../📜️script.ts";
import {readLookupsV1} from "../../💾️store/🟦️.ts";
import {portableRootsV1,slotMetadataV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {canonicalMetadataV1,cargoIdentityArgumentsV1,extraFilenameV1,normalizeRustcArgumentsV1,withCanonicalMetadataV1} from "../../🔏️key/🟦️.ts";

/** 🆔️ Contract: the unit key leaves out the identity Cargo derives from its workspace (`-C metadata`, `-C extra-filename`), so one crate compiled in two Cargo workspaces with other roots, member sets and metadata is one entry: the second workspace is a local hit, gets its own file names, and its dependants link and run. */
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"k1-meta-"+label+"-")));scratch.push(path);return path;};
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

type Workspace={root:string;work:string;metadata:Record<string,string>;extra:Record<string,string>};
const sources:Record<string,string>={a:"pub fn answer(x: u32) -> u32 { x + 1 }\n",b:"pub fn twice(x: u32) -> u32 { a::answer(x) * 2 }\n"};
function workspace(label:string,cache:string,members:readonly string[]):Workspace{
 const root=directory(label),work=join(root,"ws");
 mkdirSync(join(work,"target","deps"),{recursive:true});
 for(const member of members){mkdirSync(join(work,member));writeFileSync(join(work,member,"Cargo.toml"),`[package]\nname="${member}"\n`);}
 process.env.SEMIO_CAS_ROOT=join(root,"cas");
 const hash=(text:string)=>Buffer.from(text).toString("hex").padEnd(16,"0").slice(0,16);
 return{root,work,metadata:{a:hash(label+"a"),b:hash(label+"b")},extra:{a:"-"+hash(label+"xa"),b:"-"+hash(label+"xb")}};
}
const argvOf=(w:Workspace,crate:string)=>["--crate-name",crate,"--edition=2021",crate+"/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","metadata="+w.metadata[crate],"-C","extra-filename="+w.extra[crate],"--out-dir","target/deps","-L","dependency=target/deps","--remap-path-prefix="+w.work+"=.",...crate==="b"?["--extern","a=target/deps/liba"+w.extra.a+".rlib"]:[]];
async function compile(w:Workspace,cache:string,crate:string):Promise<number>{
 mkdirSync(join(w.work,crate),{recursive:true});writeFileSync(join(w.work,crate,"lib.rs"),sources[crate]!);
 const environment={...process.env,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"d",SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc!]),SEMIO_UNIT_CACHE_TOOLCHAIN:"k1-test",CARGO_PKG_NAME:crate,CARGO_MANIFEST_DIR:join(w.work,crate),RUSTC_BOOTSTRAP:"1"} as Record<string,string|undefined>;
 delete environment.SEMIO_UNIT_CACHE_LANE_ROOT;delete environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT;
 const write=process.stdout.write.bind(process.stdout),err=process.stderr.write.bind(process.stderr);
 process.stdout.write=(()=>true) as typeof process.stdout.write;process.stderr.write=(()=>true) as typeof process.stderr.write;
 const origin=process.cwd();process.chdir(w.work);
 try{return await wrapRustcV1(argvOf(w,crate),environment,w.work);}finally{process.chdir(origin);process.stdout.write=write;process.stderr.write=err;}
}
const round=async(w:Workspace,cache:string)=>{
 const before=readLookupsV1(cache).length;
 for(const crate of["a","b"])expect(await compile(w,cache,crate),crate).toBe(0);
 return Object.fromEntries(readLookupsV1(cache).slice(before).map(row=>[row.crate,row]));
};

test("the identity arguments are recognised in every spelling, left out of the normalized line and replaced by the canonical metadata",()=>{
 const spellings=[["-C","metadata=aa","-C","extra-filename=-aa"],["-Cmetadata=aa","-Cextra-filename=-aa"],["--codegen","metadata=aa","--codegen=extra-filename=-aa"]];
 for(const identity of spellings){
  const args=["--crate-name","c",...identity,"-C","opt-level=3","x.rs"];
  expect(normalizeRustcArgumentsV1(args,new Map(),undefined)).toEqual(["--crate-name","c","-C","opt-level=3","x.rs"]);
  expect(cargoIdentityArgumentsV1(args)).toEqual(["metadata=aa","extra-filename=-aa"]);
  expect(extraFilenameV1(args)).toBe("-aa");
  const rewritten=withCanonicalMetadataV1(args,"0123456789abcdef");
  expect(rewritten.filter(argument=>argument.includes("metadata=")).every(argument=>argument.endsWith("metadata=0123456789abcdef"))).toBe(true);
  expect(rewritten.some(argument=>argument.includes("extra-filename=-aa"))).toBe(true);
 }
 expect(canonicalMetadataV1("k1")).toMatch(/^[0-9a-f]{16}$/u);expect(canonicalMetadataV1("k1")).not.toBe(canonicalMetadataV1("k2"));
});

real("one crate and its dependant in two Cargo workspaces (other roots, members and Cargo metadata): equal keys, a local hit, own file names, and the result links and runs",async()=>{
 const cache=directory("cache"),first=workspace("one",cache,["a","b"]),second=workspace("two",cache,["a","b","extra-member","another"]);
 const miss=await round(first,cache);
 expect(miss.a!.outcome).toBe("miss");expect(miss.b!.outcome).toBe("miss");
 const hit=await round(second,cache);
 expect(hit.a!.outcome).toBe("local-hit");expect(hit.b!.outcome).toBe("local-hit");
 expect(hit.a!.key).toBe(miss.a!.key);expect(hit.b!.key).toBe(miss.b!.key);
 for(const crate of["a","b"])for(const [workspace_,suffix] of[[first,"rlib"],[first,"rmeta"],[second,"rlib"],[second,"rmeta"]] as const)expect(existsSync(join(workspace_.work,"target","deps","lib"+crate+workspace_.extra[crate]+"."+suffix)),crate+suffix).toBe(true);
 expect(readFileSync(join(second.work,"target","deps","a"+second.extra.a+".d"),"utf8")).toContain("liba"+second.extra.a+".rlib");
 expect(readFileSync(join(second.work,"target","deps","a"+second.extra.a+".d"),"utf8")).not.toContain(first.extra.a);
 const main=join(second.work,"main.rs");writeFileSync(main,"fn main(){println!(\"{}\",b::twice(20));}\n");
 const link=spawnSync(rustc!,["--edition","2021",main,"-o",join(second.work,"main"),"-L","dependency="+join(second.work,"target","deps"),"--extern","b="+join(second.work,"target","deps","libb"+second.extra.b+".rlib"),"--extern","a="+join(second.work,"target","deps","liba"+second.extra.a+".rlib")],{encoding:"utf8"});
 expect(link.status,link.stderr).toBe(0);
 const run=spawnSync(join(second.work,"main"),{encoding:"utf8"});expect(run.stdout.trim()).toBe("42");
 const plain=join(second.work,"plain");mkdirSync(plain);
 const oracle=(crate:string,more:readonly string[])=>spawnSync(rustc!,["--edition","2021","--crate-name",crate,"--crate-type","lib","--emit=link","-C","metadata=plain"+crate,"--out-dir",plain,"-L","dependency="+plain,join(second.work,crate,"lib.rs"),...more],{encoding:"utf8"});
 expect(oracle("a",[]).status).toBe(0);expect(oracle("b",["--extern","a="+join(plain,"liba.rlib")]).status).toBe(0);
});

const slotInput=(over:Partial<{toolchain:string;normalized:string[];sourceSha256:string;externs:{name:string;sha256:string}[]}>={})=>({toolchain:"tc",normalized:["--crate-name","a","--edition=2021","/work/ws/a/lib.rs","--extern","dep=#digest-1"],sourceSha256:"s1",externs:[{name:"dep",sha256:"digest-1"}],environment:[["CARGO_PKG_NAME","a"]] as [string,string][],...over});
const slotParts=["rel","host","opt=0","a@1.0.0","a","lib/link","f1",""];

test("the metadata of a unit is its slot-level identity: equal across workspaces and edits, distinct across crates, versions, kinds, features, flags, externs and compilers",()=>{
 const one=portableRootsV1({cwd:"/work/ws",workspace:undefined}),two=portableRootsV1({cwd:"/elsewhere/ws2",workspace:undefined});
 const base=slotMetadataV1(slotInput(),one,slotParts);
 expect(base).toMatch(/^[0-9a-f]{16}$/u);
 expect(slotMetadataV1(slotInput({sourceSha256:"s2",externs:[{name:"dep",sha256:"digest-2"}],normalized:["--crate-name","a","--edition=2021","/work/ws/a/lib.rs","--extern","dep=#digest-2"]}),one,slotParts),"neither source nor extern bytes").toBe(base);
 expect(slotMetadataV1(slotInput({normalized:["--crate-name","a","--edition=2021","/elsewhere/ws2/a/lib.rs","--extern","dep=#digest-1"]}),two,slotParts),"another checkout root").toBe(base);
 expect(slotMetadataV1(slotInput(),one,["other",...slotParts.slice(1)]),"the release digest is the toolchain's business").toBe(base);
 const differs=[
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,3),"a@1.0.1",...slotParts.slice(4)]),
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,3),"b@1.0.0",...slotParts.slice(4)]),
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,3),"a@1.0.0#sub/a",...slotParts.slice(4)]),
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,5),"rlib/link",...slotParts.slice(6)]),
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,5),"lib/metadata",...slotParts.slice(6)]),
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,6),"f1,f2",""]),
  slotMetadataV1(slotInput(),one,[...slotParts.slice(0,2),"opt=3",...slotParts.slice(3)]),
  slotMetadataV1(slotInput(),one,[slotParts[0]!,"wasm32",...slotParts.slice(2)]),
  slotMetadataV1(slotInput({toolchain:"tc2"}),one,slotParts),
  slotMetadataV1(slotInput({normalized:["--crate-name","a","--edition=2018","/work/ws/a/lib.rs","--extern","dep=#digest-1"]}),one,slotParts),
  slotMetadataV1(slotInput({normalized:["--crate-name","a","--edition=2021","/work/ws/a/lib.rs","--extern","other=#digest-1"]}),one,slotParts),
 ];
 expect(new Set([base,...differs]).size,"every difference is a distinct identity").toBe(differs.length+1);
 expect(slotMetadataV1(slotInput(),one,undefined)).toMatch(/^[0-9a-f]{16}$/u);
});

const logger=(root:string):string=>{
 const file=join(root,"logger.js");
 writeFileSync(file,'const {appendFileSync}=require("node:fs"),{spawnSync}=require("node:child_process"),[,,log,rustc,...args]=process.argv;const rows=args.filter((argument,index)=>argument.startsWith("metadata=")&&args[index-1]==="-C"||argument.startsWith("-Cmetadata="));if(rows.length>0)appendFileSync(log,JSON.stringify(rows)+"\\n");process.exit(spawnSync(rustc,args,{stdio:"inherit"}).status??1);\n');
 return file;
};

real("a body edit of a library keeps its metadata, so the dependant of a check chain stays a hit and the whole chain still links and runs",async()=>{
 const cache=directory("cut"),w=workspace("cut",cache,["a","b"]),log=join(w.root,"metadata.log");
 const check=(crate:string)=>argvOf(w,crate).map(argument=>argument==="--emit=dep-info,metadata,link"?"--emit=dep-info,metadata":argument).map((argument,index,all)=>all[index-1]==="--extern"?argument.replace(".rlib",".rmeta"):argument);
 const run=async(a:string):Promise<Record<string,string>>=>{
  sources.a=a;const before=readLookupsV1(cache).length;
  for(const crate of["a","b"]){
   mkdirSync(join(w.work,crate),{recursive:true});writeFileSync(join(w.work,crate,"lib.rs"),sources[crate]!);
   const environment={...process.env,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"d",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,logger(w.root),log,rustc!]),SEMIO_UNIT_CACHE_TOOLCHAIN:"k4-test",CARGO_PKG_NAME:crate,CARGO_PKG_VERSION:"1.0.0",CARGO_MANIFEST_DIR:join(w.work,crate),RUSTC_BOOTSTRAP:"1"} as Record<string,string|undefined>;
   delete environment.SEMIO_UNIT_CACHE_LANE_ROOT;delete environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT;
   const write=process.stdout.write.bind(process.stdout),err=process.stderr.write.bind(process.stderr);
   process.stdout.write=(()=>true) as typeof process.stdout.write;process.stderr.write=(()=>true) as typeof process.stderr.write;
   const origin=process.cwd();process.chdir(w.work);
   try{expect(await wrapRustcV1(check(crate),environment,w.work),crate).toBe(0);}finally{process.chdir(origin);process.stdout.write=write;process.stderr.write=err;}
  }
  return Object.fromEntries(readLookupsV1(cache).slice(before).map(row=>[row.crate,row.outcome]));
 };
 const original=sources.a!;
 try{
  expect(await run(original)).toEqual({a:"miss",b:"miss"});
  expect(await run(original.replace("x + 1","x + 100")),"a is compiled again, b is served").toEqual({a:"miss",b:"local-hit"});
  const rows=readFileSync(log,"utf8").trim().split("\n").map(row=>JSON.parse(row) as string[]);
  expect(rows.length,"the compiler ran for a, b, a").toBe(3);
  expect(rows[0],"a keeps its metadata across the body edit").toEqual(rows[2]);
  expect(rows[0],"a and b never share one").not.toEqual(rows[1]);
  expect(rows[0]![0]).toMatch(/^metadata=[0-9a-f]{16}$/u);
 }finally{sources.a=original;}
 const consumerSource=join(w.work,"d.rs");writeFileSync(consumerSource,"pub fn run() -> u32 { b::twice(1) + a::answer(1) }\n");
 const consumer=spawnSync(rustc!,["--edition","2021","--emit=metadata","--crate-type","lib","--crate-name","k4d",consumerSource,"--out-dir",join(w.work,"d"),"-L","dependency="+join(w.work,"target","deps"),"--extern","b="+join(w.work,"target","deps","libb"+w.extra.b+".rmeta"),"--extern","a="+join(w.work,"target","deps","liba"+w.extra.a+".rmeta")],{encoding:"utf8"});
 expect(consumer.status,consumer.stderr).toBe(0);
},900000);
