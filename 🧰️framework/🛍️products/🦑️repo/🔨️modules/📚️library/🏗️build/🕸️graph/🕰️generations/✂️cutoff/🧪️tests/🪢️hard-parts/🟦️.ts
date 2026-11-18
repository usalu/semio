/** 🪢️ Hard parts: 2 tests covering closures whose parts hold a bare module resolution (a row no path list bounds, asked again once per start and never the reason to walk a closure while it holds) and a start that was cancelled half way (the next start derives only what was not persisted). */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{CLOSURE_SETTLE_V1,createClosureSettlerV1,stampOfV1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createNodeStoreV1}from"../../../../🧮️derived-nodes/💾️store/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../🟨️.mjs";
import{createFactIndexV1}from"../../../🟨️.mjs";
import type{GenerationRowV1}from"../../../🟨️.mjs";

const COUNT=40,names=Array.from({length:COUNT},(_,at)=>"q"+String(at).padStart(3,"0")),rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1;
const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),{kind:"file",path:"own/"+name,fact:"v"}]] as [string,GenerationRowV1[]])]));
const suspects=new Set(names.map(name=>derivedOwnerNameV1(rowOf(name))));

function fixture(){
 const store=mkdtempSync(join(tmpdir(),"hard-store-")),workspace=mkdtempSync(join(tmpdir(),"hard-ws-"));
 mkdirSync(join(workspace,"own"));mkdirSync(join(workspace,"shared"));writeFileSync(join(workspace,"shared/s.txt"),"s0");writeFileSync(join(workspace,"own/package.json"),"{}");
 for(const name of names)writeFileSync(join(workspace,"own",name),"v0-"+name);
 const text=(path:string)=>readFileSync(join(workspace,path),"utf8");
 const run=async(settler:ReturnType<typeof createClosureSettlerV1>,dirty:{paths:string[]}|undefined,options:{during?:(count:number)=>void;signal?:AbortSignal}={})=>{
  const derived:string[]=[];let rows=0;
  const settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),signal:options.signal,derive:async(row:{path:string})=>{derived.push(row.path);options.during?.(derived.length);return{fact:"old",parts:[[{kind:"resolve",path:JSON.stringify([join(workspace,"own/x.ts"),"shared-package"]),fact:text("shared/s.txt")}],[{kind:"file",path:"own/"+row.path,fact:text("own/"+row.path)}]]};},hold:async(list:{kind:string;path:string;fact:string}[])=>{rows+=list.length;return list.every(row=>(row.kind==="resolve"?text("shared/s.txt"):text(row.path))===row.fact);},dirty,vouchStamp:(name:string)=>stampOfV1(join(workspace,name))} as never);
  return{settled,derived,rows};
 };
 const fresh=(chunk=CLOSURE_SETTLE_V1.settleChunkClosures)=>createClosureSettlerV1({store:createNodeStoreV1({root:store}),limits:{...CLOSURE_SETTLE_V1,settleChunkClosures:chunk,deriveConcurrency:4}});
 return{store,workspace,run,fresh,drop:()=>{rmSync(store,{recursive:true,force:true});rmSync(workspace,{recursive:true,force:true});}};
}

test("a manifest named in the journal moves no closure, a bare resolution is asked again once per start however many closures hold it, nothing is derived while it holds and everything when it moved",async()=>{
 const world=fixture(),dirty={paths:["own/package.json"]};
 try{
  const first=world.fresh(),cold=await world.run(first,undefined);expect(cold.derived.length).toBe(COUNT);await first.markIndexed({held:cold.settled.held});
  for(let round=0;round<3;round++){
   const again=await world.run(world.fresh(),dirty);
   expect(again.derived,"round "+round+": the rows hold, so no closure is walked").toEqual([]);
   expect(again.rows,"round "+round+": the resolution row once, not once per closure").toBe(1);
   expect(again.settled.held.size+again.settled.kept.size).toBe(COUNT);
  }
  writeFileSync(join(world.workspace,"shared/s.txt"),"s1");
  const moved=await world.run(world.fresh(),dirty);expect(moved.derived.length,"a moved row reaches every closure that read it").toBe(COUNT);
 }finally{world.drop();}
});

test("a start cancelled half way keeps what it derived: the next start derives only the closures not persisted before the cancel",async()=>{
 const world=fixture();
 try{
  const stop=new AbortController();
  const first=world.fresh(10);
  await expect(world.run(first,undefined,{signal:stop.signal,during:count=>{if(count===COUNT/2)stop.abort();}})).rejects.toBeDefined();
  const resumed=await world.run(world.fresh(10),undefined);
  expect(resumed.derived.length,"derived again: "+resumed.derived.length).toBeLessThanOrEqual(COUNT/2);
  expect(resumed.settled.failed).toBe(0);
 }finally{world.drop();}
});
