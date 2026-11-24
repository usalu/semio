/** 🧷️ Contract: a machine under memory pressure releases the retained observations of an owner the moment it is taken; the memo of the read still remembers the owner (it is filled before the release), so a read whose paths keep moving takes again only the owners that read a moved path and still ends, whatever the pressure.
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../../👁️observation/🟨️.mjs";
import {createTakenMemoV1,keepTakenThenReleaseV1} from "../../🟨️.mjs";

const OWNERS=60,SHARED=6;
const memoOf=()=>createTakenMemoV1({holds:rows=>observation.observationRowsHoldV1(rows as never),retain:rows=>observation.retainObservedRowsV1(rows as never)});

async function world(body:(input:{root:string;files:ReadonlyMap<string,readonly string[]>;write:(file:string,text:string)=>Promise<void>}) =>Promise<void>){
 const root=await mkdtemp(join(tmpdir(),"taken-pressure-")),write=async(file:string,text:string)=>{await writeFile(join(root,file),text);},files=new Map<string,string[]>();
 try{
  await mkdir(join(root,"shared"),{recursive:true});
  for(let at=0;at<SHARED;at++)await write("shared/"+at+".txt","shared "+at);
  for(let at=0;at<OWNERS;at++){
   const directory="owner-"+at;await mkdir(join(root,directory),{recursive:true});await write(directory+"/src.txt","source "+at);
   files.set(directory,[directory+"/src.txt","shared/"+at%SHARED+".txt"]);
  }
  await body({root,files,write});
 }finally{await rm(root,{recursive:true,force:true});}
}

/** 🏃️ One pass: replay the memo, take the rest; `pressure` releases the retained observations of a taken owner at once (as the governor of a loaded machine does), `before` is the order of the fix, `false` the old order (read after the release). */
async function pass(root:string,files:ReadonlyMap<string,readonly string[]>,memo:ReturnType<typeof createTakenMemoV1>,input:{pressure:boolean;before:boolean;moved?:readonly string[]}){
 const took:string[]=[],publications=new Map<string,readonly unknown[]>();
 const captured=await observation.captureObservedSourceInputsV1(async()=>{
  memo.evictReferencing(input.moved??[]);
  const reused=new Set((await memo.replay([...files.keys()].map(name=>({name,key:"job "+name})))).map(row=>row.name));
  for(const[name,list]of files){
   if(reused.has(name))continue;
   took.push(name);
   const own=await observation.captureObservedSourceInputsV1(async()=>{for(const file of list)observation.observedFs.readFileSync(join(root,file),"utf8");return name;},undefined,false);
   observation.retainObservedRowsV1(own.observations);publications.set(name,own.observations);
   const release=input.pressure?()=>{publications.delete(name);}:undefined;
   if(input.before)keepTakenThenReleaseV1({memo,name,key:"job "+name,project:name,read:()=>publications.get(name),release});
   else{release?.();keepTakenThenReleaseV1({memo,name,key:"job "+name,project:name,read:()=>publications.get(name)});}
  }
  return reused.size;
 },undefined,false);
 return{took:took.sort(),reused:captured.value};
}

test("under pressure the old order remembers nothing, the new order remembers every taken owner",()=>world(async({root,files})=>{
 const old=memoOf(),fixed=memoOf();
 expect((await pass(root,files,old,{pressure:true,before:false})).took).toHaveLength(OWNERS);
 expect((await pass(root,files,fixed,{pressure:true,before:true})).took).toHaveLength(OWNERS);
 expect(old.stats().remembered).toBe(0);expect(fixed.stats().remembered).toBe(OWNERS);
 expect((await pass(root,files,old,{pressure:true,before:false})).took).toHaveLength(OWNERS);
 expect((await pass(root,files,fixed,{pressure:true,before:true})).took).toHaveLength(0);
}));

test("a path that moves in every pass: each pass takes again exactly the owners that read it, and the read ends",()=>world(async({root,files,write})=>{
 const memo=memoOf();let moved:string[]=[],total=0;
 await pass(root,files,memo,{pressure:true,before:true});
 for(let round=0;round<SHARED*3;round++){
  const file="shared/"+round%SHARED+".txt";await write(file,"edit "+round);moved=[join(root,file)];
  const expected=[...files].filter(([,list])=>list.includes(file)).map(([name])=>name).sort();
  const next=await pass(root,files,memo,{pressure:true,before:true,moved});
  expect(next.took).toEqual(expected);expect(next.reused).toBe(OWNERS-expected.length);total+=next.took.length;
 }
 expect(total).toBe(OWNERS*3);
 expect((await pass(root,files,memo,{pressure:true,before:true})).took).toHaveLength(0);
}));

test("a path that moved without an eviction hint is still found by the rows the memo verifies",()=>world(async({root,files,write})=>{
 const memo=memoOf();await pass(root,files,memo,{pressure:true,before:true});
 await write("shared/2.txt","changed without a hint");
 const expected=[...files].filter(([,list])=>list.includes("shared/2.txt")).map(([name])=>name).sort();
 expect((await pass(root,files,memo,{pressure:true,before:true})).took).toEqual(expected);
}));
