/** 🪪️ Contract: the witness of rows that all hold names the few paths whose stamp decides them, holds while none of them moves and never holds while a row it covers does not (checked against a git tree digest of the directory, which sees every name and every byte, and against the full probe of every row); a path written inside the racy window is refused, and a witness never replaces the probe of the kinds it does not cover. */
import{expect,test}from"bun:test";
import{execFileSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{directoryFactOfV1,observationRowFreshV1,observationWitnessHoldsSyncV1,observationWitnessHoldsV1,observationWitnessV1,observeDetachedV1,observedFieldV1}from"../../🟨️.mjs";

const git=(directory:string,...args:string[])=>execFileSync("git",["-C",directory,"-c","core.fsmonitor=false",...args],{encoding:"utf8"}).trim();
const treeOf=(directory:string)=>{git(directory,"add","-A","-f");return git(directory,"write-tree");};
const scenario=async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"witness-")));
 git(root,"init","-q");
 mkdirSync(join(root,"src"));mkdirSync(join(root,"lib"));
 writeFileSync(join(root,"src","a.js"),"export const a=1;\n");writeFileSync(join(root,"src","b.js"),"export const b=2;\n");writeFileSync(join(root,"lib","c.js"),"c\n");writeFileSync(join(root,"lib","p.json"),'{"name":"x"}');
 symlinkSync(join(root,"lib","c.js"),join(root,"src","link.js"));
 const file=(...parts:string[])=>join(root,...parts);
 const field=(await observeDetachedV1(()=>observedFieldV1(join(root,"lib","p.json"),"json","/name"))).rows[0];
 const rows=[
  field,
  {kind:"lstat",path:file("src","a.js"),fact:"file"},
  {kind:"lstat",path:file("src"),fact:"directory"},
  {kind:"fileExists",path:file("src","b.js"),fact:true},
  {kind:"fileExists",path:file("src","absent.js"),fact:false},
  {kind:"exists",path:file("src","link.js"),fact:true},
  {kind:"directoryExists",path:file("lib"),fact:true},
  {kind:"stat",path:file("lib"),fact:"directory"},
  {kind:"readdir",path:file("src"),fact:directoryFactOfV1("readdir",file("src"))},
  {kind:"resolve",path:JSON.stringify([file("src","a.js"),file("src","b.js")]),fact:file("src","b.js")},
  {kind:"realpath",path:file("src","link.js"),fact:file("lib","c.js")},
 ];
 return{root,file,rows};
};
const later=()=>Date.now()+60_000;
const holdsAll=(rows:readonly unknown[])=>rows.every(row=>observationRowFreshV1(row));

test("a witness holds while nothing it names moves and costs one stamp per distinct path",async()=>{
 const{root,file,rows}=await scenario();
 try{
  expect(holdsAll(rows)).toBe(true);
  const witness=observationWitnessV1(rows,{since:later()})!;
  expect(witness).toBeDefined();
  expect(new Set(witness.paths)).toEqual(new Set([file("src"),file("lib","p.json")]));
  expect(witness.typed.map(([kind,path])=>kind+" "+path).sort()).toEqual(rows.filter(row=>["lstat","stat","exists","fileExists","directoryExists"].includes(row.kind)).map(row=>row.kind+" "+row.path).sort());
  expect(witness.paths.length).toBe(new Set(witness.paths).size);
  expect(witness.live).toHaveLength(1);
  expect(witness.code.some(([key])=>key.startsWith("f\0json"))).toBe(true);
  expect(await observationWitnessHoldsV1(witness)).toBe(true);
  const stamped=new Map<string,string>();
  expect(await observationWitnessHoldsV1(witness,{tuples:stamped})).toBe(true);
  expect(stamped.size).toBe(witness.paths.length);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a witness never holds while a row it covers does not, whatever changes the tree",async()=>{
 const changes:Record<string,(file:(...parts:string[])=>string)=>void>={
  "a name appears":file=>writeFileSync(file("src","added.js"),"x"),
  "a named file goes":file=>rmSync(file("src","b.js")),
  "an absent name appears":file=>writeFileSync(file("src","absent.js"),"x"),
  "a file is replaced by a directory":file=>{rmSync(file("src","a.js"));mkdirSync(file("src","a.js"));},
  "a file is renamed over another":file=>renameSync(file("src","a.js"),file("src","b.js")),
  "a link is retargeted":file=>{rmSync(file("src","link.js"));symlinkSync(file("src","a.js"),file("src","link.js"));},
  "the target of a followed link goes":file=>rmSync(file("lib","c.js")),
  "a field value changes":file=>writeFileSync(file("lib","p.json"),'{"name":"y"}'),
  "a directory is removed":file=>rmSync(file("lib"),{recursive:true}),
 };
 for(const[name,change]of Object.entries(changes)){
  const{root,file,rows}=await scenario();
  try{
   const witness=observationWitnessV1(rows,{since:later()})!,before=treeOf(root);
   change(file);
   const moved=treeOf(root)!==before,rowsHold=holdsAll(rows),witnessHolds=await observationWitnessHoldsV1(witness);
   expect(observationWitnessHoldsSyncV1(witness),"the synchronous verdict is the asynchronous one").toBe(witnessHolds);
   expect(moved,name).toBe(true);
   expect(rowsHold,name).toBe(false);
   expect(witnessHolds,name).toBe(false);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("a name created or removed beside the named rows leaves the witness holding, as the rows do",async()=>{
 const{root,file,rows}=await scenario();
 try{
  const witness=observationWitnessV1(rows,{since:later()})!;
  writeFileSync(file("beside.txt"),"x");mkdirSync(file("beside"));writeFileSync(file("lib","unrelated.txt"),"x");
  rmSync(file("beside"),{recursive:true});
  expect(holdsAll(rows)).toBe(true);
  expect(await observationWitnessHoldsV1(witness)).toBe(true);
  writeFileSync(file("src","absent.js"),"x");
  expect(holdsAll(rows)).toBe(false);
  expect(await observationWitnessHoldsV1(witness)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a change that leaves every row holding may or may not move the witness, never the other way",async()=>{
 const{root,file,rows}=await scenario();
 try{
  const witness=observationWitnessV1(rows,{since:later()})!;
  writeFileSync(file("lib","unrelated.txt"),"x");
  const rowsHold=holdsAll(rows),witnessHolds=await observationWitnessHoldsV1(witness);
  expect(rowsHold).toBe(true);
  expect(!witnessHolds||rowsHold).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a path written within the racy window gets no witness and the live rows are probed again",async()=>{
 const{root,file,rows}=await scenario();
 try{
  expect(observationWitnessV1(rows,{since:Date.now()})).toBeUndefined();
  const witness=observationWitnessV1(rows,{since:later()})!;
  rmSync(file("src","link.js"));symlinkSync(file("lib","c.js"),file("src","link.js"));
  expect(holdsAll(rows)).toBe(true);
  expect(await observationWitnessHoldsV1(witness)).toBe(false);
  const live=observationWitnessV1(rows.filter(row=>row.kind==="realpath"),{since:later()})!;
  expect(live.paths).toHaveLength(0);
  expect(await observationWitnessHoldsV1(live)).toBe(true);
  rmSync(file("lib","c.js"));writeFileSync(file("lib","d.js"),"d");rmSync(file("src","link.js"));symlinkSync(file("lib","d.js"),file("src","link.js"));
  expect(await observationWitnessHoldsV1(live)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the witnesses of 300 nodes that read the same 3000 paths share one copy of each path: the heap holds the arrays, not 900 000 strings",()=>{
 const heap=()=>{Bun.gc(true);return process.memoryUsage().heapUsed/1048576;};
 const rows=(salt:number)=>Array.from({length:3000},(_,at)=>({kind:"exists",path:"/tree/"+"✏️s/🔌️plugins/🌍️gis/🗿️artifacts/📦️packages/"+at+(salt>=0?"":"x"),fact:true}));
 const before=heap(),kept:unknown[]=[];
 for(let node=0;node<300;node++)kept.push(observationWitnessV1(rows(node) as never,{since:0}));
 const held=heap()-before;
 expect(kept.length).toBe(300);expect((kept[0] as {typed:unknown[]}).typed.length).toBe(3000);
 expect(held).toBeLessThan(65);
});
