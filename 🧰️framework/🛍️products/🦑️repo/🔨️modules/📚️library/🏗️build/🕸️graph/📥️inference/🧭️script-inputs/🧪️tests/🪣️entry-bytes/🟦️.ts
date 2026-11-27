/** 🪣️ Contract: the closure entries a process keeps across waves are charged what their strings really cost, so the byte budget holds and an evicted answer is the same when asked again.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{INFERENCE_LIMITS_V1}from"../../../🧬️schema/🟨️.mjs";
import{createBoundedCacheV1}from"../../../../../🧊️bounded/🟨️.mjs";
import{closureEntryChargeBytesV1}from"../../🟨️.mjs";
import{closureValueChargeBytesV1}from"../../💾️entries/🟨️.mjs";

const files=(wave:number,owner:number)=>JSON.parse(JSON.stringify(Array.from({length:800},(_,at)=>"/workspace/packages/wave-"+wave+"/owner-"+owner+"/src/module-"+at+".ts")));
const entry=(wave:number,owner:number)=>({recipe:{wave,owner},value:{closure:files(wave,owner),generatedDependencies:[]},rows:[],layers:undefined,digests:["a","b"]});
const owned=(value:{closure:string[]})=>value.closure.reduce((sum,path)=>sum+path.length,0);

test("a string-heavy value is charged at least its characters, not a pointer per item",()=>{
 const one=entry(1,1);
 expect(closureEntryChargeBytesV1(one)).toBeGreaterThanOrEqual(owned(one.value));
});

test("many waves of owners keep the charged bytes and the really held characters inside the budget, and an answer asked again after an eviction is identical",()=>{
 const maxBytes=4_000_000,cache=createBoundedCacheV1({name:"test:closure-entries",maxEntries:INFERENCE_LIMITS_V1.closureEntryCacheEntries,maxBytes,sizeOf:closureEntryChargeBytesV1});
 const digest=(value:{closure:string[]})=>value.closure.join("\n").length+":"+value.closure[799];
 const answers=new Map<string,string>();
 for(let wave=1;wave<=12;wave++)for(let owner=0;owner<40;owner++){
  const key=wave+"/"+owner,made=entry(wave,owner);answers.set(key,digest(made.value));cache.set(key,made);
  expect(cache.stats().bytes).toBeLessThanOrEqual(maxBytes);
 }
 const held=cache.keys() as string[],value=(key:string)=>(cache.peek(key) as ReturnType<typeof entry>).value;
 expect(held.length).toBeGreaterThan(0);
 expect(held.length).toBeLessThan(480);
 expect(held.reduce((sum,key)=>sum+owned(value(key)),0)).toBeLessThanOrEqual(maxBytes);
 expect(cache.peek("1/0")).toBeUndefined();
 for(const key of ["1/0","6/17","12/39"]){const[wave,owner]=key.split("/").map(Number);expect(digest(entry(wave,owner).value)).toBe(answers.get(key)!);}
 for(const key of held)expect(digest(value(key))).toBe(answers.get(key)!);
});

test("one rule charges every closure cache: the entry charge contains the value charge, and a raised limit raises it",()=>{
 const one=entry(1,1),value=closureValueChargeBytesV1(one.value);
 expect(value).toBeGreaterThanOrEqual(owned(one.value));
 expect(closureEntryChargeBytesV1(one)).toBe(INFERENCE_LIMITS_V1.closureEntryBaseBytes+INFERENCE_LIMITS_V1.closureEntryDigestBytes*2+value);
 expect(closureValueChargeBytesV1(one.value,{...INFERENCE_LIMITS_V1,closureEntryValueStringBytes:INFERENCE_LIMITS_V1.closureEntryValueStringBytes+10})).toBe(value+10*800);
 expect(closureValueChargeBytesV1({closure:one.value.closure})).toBe(closureValueChargeBytesV1(one.value.closure)+INFERENCE_LIMITS_V1.closureEntryValueItemBytes);
});
