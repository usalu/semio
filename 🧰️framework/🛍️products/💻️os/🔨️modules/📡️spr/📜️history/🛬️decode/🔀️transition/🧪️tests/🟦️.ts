import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json";
import law from"./🧫️fixtures/🔣️.json";

test("neutral History transition bytes match SQLite projections",()=>{
 expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  for(const row of law.transitions){
   const wire=Buffer.from(row.wireHex,"hex");let at=0;
   const scalar=()=>{let n=0,shift=0;for(let i=0;i<10;i++){const b=wire[at++];if(b===undefined)throw Error("truncated original scalar");n+=(b&127)*2**shift;if(!(b&128))return n;shift+=7}throw Error("unbounded original scalar")};
   const id=()=>{expect(wire[at++]).toBe(0);const len=scalar();if(at+len>wire.length)throw Error("truncated original text");const out=wire.subarray(at,at+len).toString("utf8");at+=len;return out};
   const format=wire[at++];expect(id()).toBe(row.id);expect(id()).toBe(row.actor);expect([scalar(),scalar(),scalar()]).toEqual(row.hlt);
   const count=scalar(),deps=Array.from({length:count},id);expect(deps).toEqual(row.dependencies);
   expect(db.query("select json_group_array(value) as deps from json_each(?)").get(JSON.stringify(deps))).toEqual({deps:JSON.stringify(row.dependencies)});
   const bytes=scalar(),payload=wire.subarray(at,at+bytes);at+=bytes;
   expect(payload.toString("hex")).toBe(row.payloadHex);expect(db.query("select lower(hex(?)) as payload").get(payload)).toEqual({payload:row.payloadHex});
   expect(format===2?id():null).toBe(row.observed);expect(at).toBe(wire.length);
  }
 }finally{db.close()}
});
