import {test,expect} from "bun:test";
import {mkdtempSync,mkdirSync,writeFileSync,rmSync,rmdirSync,symlinkSync,realpathSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {join,dirname} from "node:path";
import {withPublishedBunWorkspaceOwnershipV1,withCurrentBunWorkspaceOwnershipV1,discoverCurrentBunWorkspaceScopesV1} from "../../🟦️.ts";

const fs=createRequire(import.meta.url)("node:fs") as typeof import("node:fs");
const control=()=>({signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}});
const manifest=(name:string,workspace?:object)=>JSON.stringify({name,...(workspace?{semio:{workspace}}:{})});
const random=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const noise=["node_modules/**","target/**"].map(pattern=>"!**/"+pattern);
type World={root:string;scopes:string[];dirs:string[];edges:Record<string,string[]>};
/** 🌍️ A seeded repository: a root scope over `lib`, `k` top-level scopes with their own packages, and random edges where one scope lists the packages of another. */
function world(seed:number):World{
 const next=random(seed),root=realpathSync(mkdtempSync(join(tmpdir(),"semio-closure-"))),k=2+Math.floor(next()*4),scopes:string[]=[],dirs:string[]=[],edges:Record<string,string[]>={};
 const dir=(path:string)=>{mkdirSync(join(root,path),{recursive:true});dirs.push(path);};
 const packageAt=(path:string)=>{dir(path);writeFileSync(join(root,path,"package.json"),manifest(path.replace(/\//g,"-")));};
 dir("lib");for(let i=0;i<2+Math.floor(next()*4);i++)packageAt("lib/p"+i);dir("lib/plain/deep");
 for(let i=0;i<k;i++){const name="s"+i;scopes.push(name);dir(name);for(let j=0;j<1+Math.floor(next()*4);j++)packageAt(name+"/q"+j);dir(name+"/plain/deeper");}
 for(const name of scopes){const others=scopes.filter(other=>other!==name&&next()<0.35);edges[name]=others;writeFileSync(join(root,name,"package.json"),manifest(name,{schemaVersion:1,members:["**","../lib/**",...others.map(other=>"../"+other+"/**"),...noise],owners:["*/package.json"]}));}
 writeFileSync(join(root,"package.json"),manifest("root",{schemaVersion:1,members:["lib/**",...noise],owners:["*/package.json"]}));
 return{root,scopes,dirs,edges};
}
const published=async(root:string)=>{await withPublishedBunWorkspaceOwnershipV1(root,{kind:"all"},control(),async()=>{});};
const depth=(directory:string)=>directory==="."?0:directory.split("/").length;
/** 📐️ The reference closure, computed from the report of the complete walk alone: the root, the roots, and the deepest declaring scope of every member of a reached scope. */
const reference=(report:readonly{manifest:string;packages:readonly string[]}[],roots:readonly string[]):string[]=>{
 const directories=report.map(row=>row.manifest==="package.json"?".":row.manifest.slice(0,-"/package.json".length)),byDirectory=new Map(report.map((row,index)=>[directories[index]!,row])),reached=new Set([".",...roots]),queue=[...reached];
 const ownerOf=(path:string)=>directories.filter(directory=>directory==="."||path===directory||path.startsWith(directory+"/")).sort((left,right)=>depth(right)-depth(left))[0]!;
 for(let i=0;i<queue.length;i++){const row=byDirectory.get(queue[i]!)!;for(const path of [queue[i]!,...row.packages]){const owner=ownerOf(path);if(!reached.has(owner)){reached.add(owner);queue.push(owner);}}}
 return report.filter((_,index)=>reached.has(directories[index]!)).map(row=>row.manifest);
};
const counting=():{calls:()=>number;stop:()=>void}=>{const lstat=fs.lstatSync,readdir=fs.readdirSync;let count=0;(fs as any).lstatSync=function(...args:any[]){count++;return (lstat as any).apply(this,args);};(fs as any).readdirSync=function(...args:any[]){count++;return (readdir as any).apply(this,args);};return{calls:()=>count,stop:()=>{(fs as any).lstatSync=lstat;(fs as any).readdirSync=readdir;}};};
const open=async<T>(root:string,selection:any,body:(custody:any,calls:number)=>Promise<T>):Promise<T>=>{const meter=counting();let calls=0;try{return await withCurrentBunWorkspaceOwnershipV1(root,selection,control(),async custody=>{calls=meter.calls();meter.stop();return body(custody,calls);});}finally{meter.stop();}};
const rows=(custody:any)=>custody.scopes.map((scope:any)=>({manifest:scope.manifest,packages:[...scope.packages]}));

test("model law: over 200 seeded worlds the closure walk reports exactly the reference closure of the complete walk, row for row, and never reads more",async()=>{
 let narrower=0,roots=0;
 for(let seed=1;seed<=200;seed++){
  const w=world(seed),next=random(seed*7919);
  try{
   await published(w.root);
   const chosen=w.scopes.filter(()=>next()<0.4);roots+=chosen.length;
   const full=await open(w.root,{kind:"all"},async(custody,calls)=>({rows:rows(custody),calls,paths:custody.observed().paths.length}));
   const scoped=await open(w.root,{kind:"closure",roots:chosen},async(custody,calls)=>({rows:rows(custody),calls,paths:custody.observed().paths.length}));
   const expected=reference(full.rows,chosen);
   expect({seed,manifests:scoped.rows.map((row:any)=>row.manifest)}).toEqual({seed,manifests:expected});
   for(const row of scoped.rows)expect({seed,row}).toEqual({seed,row:full.rows.find((other:any)=>other.manifest===row.manifest)});
   expect(scoped.calls).toBeLessThanOrEqual(full.calls);expect(scoped.paths).toBeLessThanOrEqual(full.paths);
   if(expected.length<full.rows.length){narrower++;expect({seed,calls:scoped.calls<full.calls||scoped.paths<full.paths}).toEqual({seed,calls:true});}
  }finally{rmSync(w.root,{recursive:true,force:true});}
 }
 expect(narrower).toBeGreaterThan(20);expect(roots).toBeGreaterThan(50);
},300000);

test("model law: the closure of the root scope alone is the root scope, however many scopes are declared",async()=>{
 const w=world(11);
 try{await published(w.root);await open(w.root,{kind:"closure",roots:[]},async custody=>{expect(custody.scopes.map((scope:any)=>scope.manifest)).toEqual(["package.json"]);});}finally{rmSync(w.root,{recursive:true,force:true});}
});

test("closure refusals: an undeclared or unknown root and a malformed selection are refused before any walk",async()=>{
 const w=world(5);
 try{
  await published(w.root);
  for(const selection of [{kind:"closure",roots:["nope"]},{kind:"closure",roots:["lib"]},{kind:"closure",roots:["s0","s0"]},{kind:"closure",roots:[1]},{kind:"closure"},{kind:"closure",roots:["../x"]},{kind:"closure",roots:["a/b"]},{kind:"closure",roots:["s0"],extra:1}])
   await expect(open(w.root,selection as any,async()=>undefined)).rejects.toThrow();
 }finally{rmSync(w.root,{recursive:true,force:true});}
});

type Tamper=(w:World,next:()=>number)=>{path:string;apply:()=>void};
const tampers:Record<string,Tamper>=({
 foreignOwner:(w,next)=>{const at=w.dirs[Math.floor(next()*w.dirs.length)]!;return{path:at,apply:()=>{mkdirSync(join(w.root,at,"fresh"));writeFileSync(join(w.root,at,"fresh","package.json"),manifest("fresh"));}};},
 missingDirectory:(w,next)=>{const at=w.dirs.filter(path=>path.includes("plain"))[Math.floor(next()*w.dirs.filter(path=>path.includes("plain")).length)]!;return{path:dirname(at),apply:()=>rmSync(join(w.root,at,"deeper"),{recursive:true,force:true})};},
 symlinkSwap:(w,next)=>{const at=w.dirs.filter(path=>path.endsWith("plain/deeper")||path==="lib/plain/deep")[0]!;void next;return{path:dirname(at),apply:()=>{rmdirSync(join(w.root,at));symlinkSync(w.root,join(w.root,at));}};},
 extraFile:(w,next)=>{const at=w.dirs[Math.floor(next()*w.dirs.length)]!;return{path:at,apply:()=>writeFileSync(join(w.root,at,"extra.txt"),"x")};},
} as Record<string,Tamper>);
const verdict=async(w:World,selection:any,tamper:{path:string;apply:()=>void})=>{
 let refused=false,watched=false;
 try{await open(w.root,selection,async custody=>{
  const observed=new Set<string>(custody.observed().paths.filter((row:any)=>row.list!==undefined).map((row:any)=>row.path));watched=observed.has(join(w.root,tamper.path))||(tamper.path==="."&&observed.has(w.root));
  tamper.apply();try{await custody.recheck();}catch{refused=true;}
 });}catch{refused=true;}
 return{refused,watched};
};

test("tamper corpus: inside the closure the scoped recheck refuses exactly what the complete recheck refuses; outside it nothing is refused",async()=>{
 let inside=0,outside=0;
 for(let seed=1;seed<=120;seed++){
  const kind=Object.keys(tampers)[seed%Object.keys(tampers).length]!,left=world(seed),right=world(seed),next=random(seed*104729),chosen=left.scopes.filter(()=>next()<0.4);
  try{
   await published(left.root);await published(right.root);
   const at=tampers[kind]!(left,random(seed*31)),twin=tampers[kind]!(right,random(seed*31));
   const complete=await verdict(right,{kind:"all"},twin),scoped=await verdict(left,{kind:"closure",roots:chosen},at);
   if(scoped.watched){inside++;expect({seed,kind,refused:scoped.refused}).toEqual({seed,kind,refused:complete.refused});}
   else{outside++;expect({seed,kind,refused:scoped.refused}).toEqual({seed,kind,refused:false});}
  }finally{rmSync(left.root,{recursive:true,force:true});rmSync(right.root,{recursive:true,force:true});}
 }
 expect(inside).toBeGreaterThan(30);expect(outside).toBeGreaterThan(5);
},300000);

test("call counts: a closure of one framework-like scope walks a fraction of the paths a complete walk walks",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"semio-closure-big-")));
 try{
  const fan=(base:string,n:number)=>{for(let a=0;a<n;a++)for(let b=0;b<n;b++){const d=join(root,base,"a"+a,"b"+b);mkdirSync(d,{recursive:true});writeFileSync(join(d,"package.json"),manifest(base+a+"-"+b));}};
  fan("lib",8);fan("heavy",24);
  writeFileSync(join(root,"package.json"),manifest("root",{schemaVersion:1,members:["lib/**",...noise],owners:["*/package.json"]}));
  writeFileSync(join(root,"heavy","package.json"),manifest("heavy",{schemaVersion:1,members:["**","../lib/**",...noise],owners:["*/package.json"]}));
  await published(root);
  const full=await open(root,{kind:"all"},async(custody,calls)=>({calls,paths:custody.observed().paths.length})),scoped=await open(root,{kind:"closure",roots:[]},async(custody,calls)=>({calls,paths:custody.observed().paths.length}));
  expect(scoped.paths*4).toBeLessThan(full.paths);expect(scoped.calls*4).toBeLessThan(full.calls);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
