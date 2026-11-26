import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {brotliCompressSync} from "node:zlib";
import Ajv from "ajv";
import {encodeClosureMemoV1,inventoryFragmentClosuresV1,readClosureMemoV1} from "../../🧲️closure-shards/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import references from "./🔣️.json";

const fragments=new Map(Object.entries(references).map(([label,entries])=>{
 const tokens=entries.map(entry=>(entry[0]==="i"?"SEMIO_SOURCE_CLOSURE_":"SEMIO_CARGO_LOCK_")+entry.slice(2,-5));
 const bytes=brotliCompressSync(JSON.stringify({label,tokens}));
 return [label,{bytes,name:createHash("sha256").update(bytes).digest("hex"),entries}] as const;
}));

test("closure memo reuses exact fragment addresses and replaces obsolete entries",async()=>{
 using oracle=new Database(":memory:");
 const reads:string[]=[],read=async(name:string)=>{
  reads.push(name);
  return [...fragments.values()].find(row=>row.name===name)?.bytes;
 };
 const previous=["first","retained","empty","removed"].map(label=>fragments.get(label)!.name);
 const memo=readClosureMemoV1(undefined);
 await inventoryFragmentClosuresV1(previous,read,undefined,memo);
 expect(reads.splice(0)).toEqual(previous);
 const saved=encodeClosureMemoV1(previous,memo)!;
 const contract=new Ajv({strict:true}).compile({definitions:schema.definitions,$ref:"#/definitions/ClosureMemo"});
 expect(contract(JSON.parse(saved))).toBe(true);
 expect(encodeClosureMemoV1([...previous].reverse(),memo)).toBe(saved);

 const current=["first","retained","empty","added"].map(label=>fragments.get(label)!);
 const names=current.map(row=>row.name),restored=readClosureMemoV1(Buffer.from(saved));
 const inventory=await inventoryFragmentClosuresV1(names,read,undefined,restored);
 expect(reads).toEqual([fragments.get("added")!.name]);
 const expected=oracle.query(`SELECT DISTINCT closure.value, fragment.key
   FROM json_each(?1) AS fragment, json_each(fragment.value) AS closure
   ORDER BY closure.value, fragment.key`).values(JSON.stringify(Object.fromEntries(current.map(row=>[row.name,row.entries]))));
 const actual=[...inventory].flatMap(([entry,row])=>[...row.fragments].map(name=>[entry,name])).sort();
 expect<unknown>(actual).toEqual(expected);
 expect([...readClosureMemoV1(Buffer.from(encodeClosureMemoV1(names,restored)!)).keys()]).toEqual([...names].sort());
});

test("closure memo rejects malformed, duplicate and oversized cached data",async()=>{
 const {name,bytes}=fragments.get("empty")!;
 const body=(entries:[string,string[]][],version=1)=>Buffer.from(JSON.stringify({
  version,
  sha256:new Bun.CryptoHasher("sha256").update(JSON.stringify(entries)).digest("hex"),
  entries,
 }));
 const canonical=body([[name,[]]]);
 expect(readClosureMemoV1(canonical).size).toBe(1);
 const changed=JSON.parse(canonical.toString());
 changed.entries[0][1].push(references.first[0]);
 const malformed=[
  Buffer.from("{"),
  body([[name,[]]],2),
  body([[name,[]],[name,[]]]),
  body([[name,["unknown"]]]),
  body([[name,[references.first[0],references.first[0]]]]),
  Buffer.from(JSON.stringify(changed)),
  Buffer.alloc(schema.definitions.Policy.const.maximumClosureMemoBytes+1),
 ];
 for(const body of malformed)expect(readClosureMemoV1(body).size).toBe(0);
 const reason=Error("cancelled"),signal=AbortSignal.abort(reason),memo=readClosureMemoV1(undefined);
 await expect(inventoryFragmentClosuresV1([name],async()=>bytes,signal,memo)).rejects.toBe(reason);
 await expect(inventoryFragmentClosuresV1([name],async()=>Buffer.from("wrong"),undefined,memo)).rejects.toThrow("Generation fragment unavailable");
 expect(memo.size).toBe(0);
});
