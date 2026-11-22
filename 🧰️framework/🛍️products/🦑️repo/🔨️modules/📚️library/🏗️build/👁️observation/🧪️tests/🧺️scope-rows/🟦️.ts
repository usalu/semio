/** 🧺️ Contract: the rows an owner's memo hit retains into the enclosing observation scopes are stored once and referenced by the scopes, so a read of thousands of owners holds its distinct rows once however many scopes enclose each hit, and the facts the scopes answer are exactly those of an independent union. */
import{expect,test}from"bun:test";
const{captureObservedSourceInputsV1,observationCollectionsV1,retainObservedRowsV1,settleObservedComputationV1}=await import(process.env.SCOPE_ROWS_MODULE??"../../🟨️.mjs");

type Row={kind:string;path:string;fact:{size:number;mtimeMs:number;ctimeMs:number;ino:number;sha256:string}};
const sizes={attempts:Number(process.env.SCOPE_ROWS_ATTEMPTS??4),owners:Number(process.env.SCOPE_ROWS_OWNERS??1900),rows:Number(process.env.SCOPE_ROWS_PER_OWNER??240),shared:Number(process.env.SCOPE_ROWS_SHARED_PERCENT??55)};
const row=(at:number):Row=>({kind:"file",path:"/w/workspace/products/tool/library/module-"+(at%97)+"/package-"+at+"/source/index.ts",fact:{size:at,mtimeMs:at*2,ctimeMs:at*3,ino:at,sha256:at.toString(16).padStart(64,"0")}});

/** 🏭️ Synthetic owners: each takes a slice of a shared pool (every owner reads the same toolchain) and the rest is its own. */
function fixture(){
 const pool=Math.max(1,Math.floor(sizes.rows*sizes.shared/100)),own=sizes.rows-pool,shared=Array.from({length:pool},(_,at)=>row(at));
 return Array.from({length:sizes.owners},(_,owner)=>[...shared,...Array.from({length:own},(_,at)=>row(pool+owner*own+at))]);
}
const key=(value:Row)=>JSON.stringify(value);

async function read(owners:Row[][],depth:number,sample:()=>void=()=>{}){
  const hit=async(rows:Row[],level:number):Promise<void>=>{
  if(level===0){retainObservedRowsV1(rows);return;}
  await captureObservedSourceInputsV1(()=>hit(rows,level-1),undefined,false);
 };
 const value=await settleObservedComputationV1(async()=>{
  await captureObservedSourceInputsV1(async()=>{for(let attempt=0;attempt<sizes.attempts;attempt++)for(const rows of owners)await hit(rows,depth);},undefined,false);
  sample();
  return"done";
 });
 return value;
}

test("the facts of the enclosing scopes are exactly the union of the owners' rows",async()=>{
 const owners=fixture(),expected=new Set(owners.flat().map(key));
 const outer=await captureObservedSourceInputsV1(async()=>{await read(owners,2);},undefined,false);
 const observed=[...outer.observations as Iterable<Row>].map(key);
 expect(new Set(observed)).toEqual(expected);expect(observed.length).toBe(expected.size);
});

test("measured: heap and scope entries of a read of memo hits",async()=>{
 const owners=fixture(),distinct=new Set(owners.flat().map(key)).size;
 Bun.gc(true);const before=process.memoryUsage().heapUsed,started=performance.now();
 let entries=0,held=0;
 const outer=await captureObservedSourceInputsV1(async()=>{await read(owners,3,()=>{const collections=observationCollectionsV1();entries=collections.filter((item:{name:string})=>item.name.startsWith("observation:scopes")).reduce((sum:number,item:{entries:number})=>sum+item.entries,0);Bun.gc(true);held=process.memoryUsage().heapUsed;});},undefined,false);
 const elapsed=performance.now()-started;
 console.log("[bench]",JSON.stringify({owners:sizes.owners,rowsPerOwner:sizes.rows,distinct,observed:(outer.observations as Row[]).length,scopeEntries:entries,heapMB:Math.round((held-before)/1048576),ms:Math.round(elapsed)}));
 expect((outer.observations as Row[]).length).toBe(distinct);expect(entries).toBeLessThanOrEqual(distinct);
});
