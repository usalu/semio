/** 🔂️ Law of the closure pass a read of many owners can run in: inside ONE pass a fact that many owners share (here the Cargo closure every owner's assembly authority walks) is observed once per pass and not once per owner: over a 100,000-entry tree served through the observed world port, N owners cost O(files) observations inside the pass and O(owners x files) without it, the authorities are equal either way, the event loop keeps turning between owners, and every owner's closure equals an independent Python model.
 * The selected read runs its owners in one pass (`discoverBuildGraphV1`). The cap cancellations of 2026-11-24 22:21 to 22:40 after its owners round were the publication of the sealed read, not the pass (W15: g1-m12 on the code without the pass was cancelled the same way; `🤝️selected-exchange/🌊️streamed`).
 * @see ../../🟦️.ts `discoverBuildGraphPartV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{libraryBootstrap}from"../../../📥️inference/🟨️.mjs";
import{projectAssemblyAuthorityV1}from"../../../📥️inference/🔎️owner-discovery/🟨️.mjs";
import{nativeDependencyRoots}from"../../../📥️inference/🦀️native-manifests/🟨️.mjs";
import{withClosurePassV1}from"../../../📥️inference/🧭️script-inputs/🟨️.mjs";
import{withObservedWorldV1}from"../../../../👁️observation/🟨️.mjs";

await libraryBootstrap;
const root="/virtual/w14",CORE=240,OWNERS=160,FILES=250,old=Date.now()-86_400_000;
const files=new Map<string,Buffer>(),dirs=new Map<string,Set<string>>([[root,new Set()]]),manifests:Record<string,string>={};
const add=(path:string,text:string)=>{files.set(path,Buffer.from(text));let at=path;while(at!==root){const parent=at.slice(0,at.lastIndexOf("/"));if(!dirs.has(parent))dirs.set(parent,new Set());dirs.get(parent)!.add(at.slice(parent.length+1));at=parent;}};
const crate=(name:string,deps:readonly string[])=>{const text=`[package]\nname = "${name}"\nversion = "0.1.0"\n\n[dependencies]\n${deps.map(dep=>`${dep} = { path = "../${dep}" }`).join("\n")}\n`;add(`${root}/c/${name}/Cargo.toml`,text);manifests[`${root}/c/${name}/Cargo.toml`]=text;for(let at=0;at<FILES-1;at++)add(`${root}/c/${name}/src/m${at}.rs`,"");};
add(`${root}/Cargo.toml`,`[workspace]\nmembers = ["c/*"]\n`);manifests[`${root}/Cargo.toml`]=`[workspace]\nmembers = ["c/*"]\n`;
for(let at=0;at<CORE;at++)crate(`core${at}`,at===0?[]:[`core${at-1}`]);
for(let at=0;at<OWNERS;at++)crate(`own${at}`,[`core${CORE-1-(at%8)}`,...at%4===0?[`own${at+1}`]:[]]);
const owners=Array.from({length:OWNERS},(_,at)=>`c/own${at}`);
const missing=(path:string)=>Object.assign(new Error("ENOENT: "+path),{code:"ENOENT"});
const stat=(path:string)=>{const file=files.get(path),directory=dirs.has(path);if(file===undefined&&!directory)throw missing(path);return{size:file?.byteLength??64,mtimeMs:old,ctimeMs:old,ino:path.length,mode:file?0o100644:0o040755,dev:1,nlink:1,isFile:()=>file!==undefined,isDirectory:()=>directory,isSymbolicLink:()=>false};};
const calls=new Map<string,number>(),paths=new Set<string>(),count=(name:string,path:unknown)=>{calls.set(name,(calls.get(name)??0)+1);paths.add(String(path));};
const port={
 existsSync:(path:string)=>{count("existsSync",path);return files.has(String(path))||dirs.has(String(path));},
 statSync:(path:string)=>{count("statSync",path);return stat(String(path));},
 lstatSync:(path:string)=>{count("lstatSync",path);return stat(String(path));},
 readFileSync:(path:string,options?:unknown)=>{count("readFileSync",path);const bytes=files.get(String(path));if(bytes===undefined)throw missing(String(path));return options==="utf8"||(options as {encoding?:string})?.encoding==="utf8"?bytes.toString("utf8"):bytes;},
 readdirSync:(path:string,options?:{withFileTypes?:boolean})=>{count("readdirSync",path);const names=[...dirs.get(String(path))??[]].sort();return options?.withFileTypes?names.map(name=>({name,isFile:()=>files.has(`${path}/${name}`),isDirectory:()=>dirs.has(`${path}/${name}`),isSymbolicLink:()=>false})):names;},
};
const total=()=>[...calls.values()].reduce((sum,value)=>sum+value,0);
const authorities=async(pass:boolean,between?:()=>Promise<void>)=>{
 calls.clear();paths.clear();const cache=new Map(),closures=new Map(),digests:string[]=[],perOwner:number[]=[];
 const body=async()=>{for(const owner of owners){await between?.();const before=total();digests.push(projectAssemblyAuthorityV1(root,owner,new Map(),undefined,cache,closures));perOwner.push(total()-before);}};
 await withObservedWorldV1(port as never,()=>pass?withClosurePassV1(body):body());
 return{digests,perOwner,calls:total(),distinct:paths.size};
};

test("the tree holds 100,000 entries and every owner's closure equals the Python model",()=>{
 expect(files.size+dirs.size).toBeGreaterThanOrEqual(100_000);
 const ours=withObservedWorldV1(port as never,()=>Object.fromEntries(owners.map(owner=>[owner,nativeDependencyRoots(owner,root,true,new Map(),new Map())])));
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py")],{input:JSON.stringify({root,manifests,owners}),encoding:"utf8",maxBuffer:1<<26});
 expect(python.status,python.stderr).toBe(0);expect(ours).toEqual(JSON.parse(python.stdout));
 expect(ours["c/own0"]!.length).toBe(CORE+2);
});

test("inside one pass the owners cost O(files) observations, without it O(owners x files), with equal authorities",async()=>{
 const alone=await authorities(false),pass=await authorities(true);
 expect(pass.digests).toEqual(alone.digests);expect(new Set(pass.digests).size).toBe(OWNERS);
 expect(pass.distinct).toBeLessThanOrEqual(CORE+OWNERS+2);
 expect(pass.calls).toBeLessThanOrEqual(6*pass.distinct);
 expect(alone.calls).toBeGreaterThan(OWNERS/8*pass.calls);
 expect(pass.perOwner.slice(1).every(cost=>cost<=24)).toBe(true);
 expect(Math.min(...alone.perOwner.slice(1))).toBeGreaterThan(CORE);
});

test("the event loop turns between owners and no owner of the pass holds it beyond the cold closure",async()=>{
 let last=performance.now(),longest=0,ticks=0;const timer=setInterval(()=>{const at=performance.now();longest=Math.max(longest,at-last);last=at;ticks++;},1);
 try{const pass=await authorities(true,()=>new Promise<void>(resolve=>setImmediate(resolve)));expect(pass.perOwner[0]).toBeGreaterThan(CORE);expect(pass.perOwner.slice(1).reduce((sum,cost)=>sum+cost,0)).toBeLessThan(pass.perOwner[0]!*OWNERS/8);}
 finally{clearInterval(timer);}
 expect(ticks).toBeGreaterThan(8);expect(longest).toBeLessThan(2000);
});


/** 🥇️ `discoverBuildGraphV1` (the selected read, every owner in the read's main thread) reads its owners inside one closure pass, like a part read and a farm child. */
test("discoverBuildGraphV1 reads the owners inside one closure pass",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
 expect(source).toContain("const primary=await withClosurePassV1(()=>owner.discoverBuildProjectsV1(");
});
