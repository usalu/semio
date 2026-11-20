/** 🫎️ Contract: an inventory record that a concurrent writer stores, replaces or removes while a presence batch looks at it makes the batch look again (at most the declared attempts, each against the state that is there then, saying why in English and German); a look that still changes ends with a typed change that the reader of the owners answers by requeueing that owner, never with an untyped failure of the read; every other failure ends at once. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{createPresenceCoalescerV1}from"../../📥️reuse/🧺️coalesce/🟨️.mjs";
import{inventoryNamesSettledV1}from"../../📥️reuse/🟨️.mjs";
import schema from"../../📥️reuse/🧬️schema/🔣️.json";

const changed=(text="Inference inventory absence changed")=>Object.assign(Error(text),{code:"BUILD_STORE_CHANGED"});
const pool=Array.from({length:6},(_,at)=>"SEMIO_SOURCE_CLOSURE_"+at.toString(16).padStart(64,"0"));

test("the attempts are declared by the policy of the reuse schema",()=>{
 expect(schema.definitions.Policy.const.changeAttempts).toBe(3);
});

test("an absence that turned into a presence while the batch looked is answered by the next look, in one state",async()=>{
 const stored=new Set<string>(),lines:string[]=[];let calls=0;
 const run=async(names:readonly string[])=>{calls++;const missing=new Set(names.filter(name=>!stored.has(name)));if(calls===1){stored.add(pool[1]!);throw changed();}return missing;};
 const missing=await inventoryNamesSettledV1(run,pool.slice(0,3),{say:line=>lines.push(line)});
 expect(calls).toBe(2);expect([...missing].sort()).toEqual([pool[0]!,pool[2]!].sort());
 expect(lines).toHaveLength(1);expect(lines[0]).toContain("absence changed");expect(lines[0]).toContain("Inventar");expect(lines[0]).toContain("inventory changed");
});

test("a look that keeps changing ends typed after the declared attempts, every other failure at once",async()=>{
 let calls=0;
 await expect(inventoryNamesSettledV1(async()=>{calls++;throw changed("Inference inventory leaf changed");},pool,{say:()=>{}})).rejects.toMatchObject({code:"BUILD_STORE_CHANGED"});
 expect(calls).toBe(schema.definitions.Policy.const.changeAttempts+1);
 calls=0;
 await expect(inventoryNamesSettledV1(async()=>{calls++;throw Error("Inference inventory record identity refused");},pool,{say:()=>{}})).rejects.toThrow("identity refused");
 expect(calls).toBe(1);
});

test("through the coalescer only the callers whose names keep changing see the change; the others are answered",async()=>{
 const stored=new Set(pool.slice(0,4)),flapping=pool[5]!;
 const coalescer=createPresenceCoalescerV1({run:names=>inventoryNamesSettledV1(async list=>{if(list.includes(flapping))throw changed();return new Set(list.filter(name=>!stored.has(name)));},names,{say:()=>{}})}),round={};
 const rows=await Promise.allSettled([coalescer.ask([pool[0]!],round),coalescer.ask([flapping],round),coalescer.ask([pool[4]!],round)]);
 expect(rows.map(row=>row.status)).toEqual(["fulfilled","rejected","fulfilled"]);
 expect((rows[0]as PromiseFulfilledResult<boolean>).value).toBe(true);expect((rows[2]as PromiseFulfilledResult<boolean>).value).toBe(false);
 expect((rows[1]as PromiseRejectedResult).reason.code).toBe("BUILD_STORE_CHANGED");
});

test("every change the batch can see is typed and the presence checks go through the settled look",()=>{
 const source=readFileSync(new URL("../../📥️reuse/🟨️.mjs",import.meta.url),"utf8");
 for(const text of["ancestry changed","absence changed","descriptor changed","leaf changed before delivery","leaf changed"])expect(source).toContain('inventoryChangedV1("Inference inventory '+text+'")');
 expect(source).not.toMatch(/Error\("Inference inventory (ancestry|absence|descriptor|leaf) changed/u);
 expect(source).toContain("run:(names,withdraw)=>inventoryNamesSettledV1(list=>missingInventoryNamesV1(root,list,control,withdraw),names,");
 expect(source).toContain("(await inventoryNamesSettledV1(list=>missingInventoryNamesV1(root,list,control,withdraw),names,");
});
