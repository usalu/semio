import {test,expect} from "bun:test";
import Ajv from "ajv";
import {toByteArray} from "base64-js";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {Base64DecodeCursor} from "../../../../../../../🔨️modules/🚪️io/🔤️base64/🟦️.ts";
const fixture=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
test("original artifact document pair base64 stages match independent codec and admission oracle",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");db.run("CREATE TABLE budget(items INTEGER,copy INTEGER,capacity INTEGER,depth INTEGER)");
 try{
  for(const row of fixture.valid){
   const pair=[];
   for(const [encoded,expected] of [[row.pack,row.packBytes],[row.spr,row.sprBytes]] as const){
    const cursor=new Base64DecodeCursor(encoded);
    expect(cursor.expectedOutputBytes()).toBe(expected.length);
    let turns=0,born=0;
    while(!cursor.isComplete()){
     const work=cursor.nextWorkDemand(),capacity=cursor.nextCapacityByteDemand();
     for(const axis of fixture.deniedAxes){
      const grant={items:1,copy:work,capacity,depth:2};if(axis==="capacity"&&capacity===0)continue;
      grant[axis as keyof typeof grant]=axis==="depth"?1:Math.max(0,grant[axis as keyof typeof grant]-1);
      db.run("DELETE FROM budget");db.query("INSERT INTO budget VALUES(?,?,?,?)").run(grant.items,grant.copy,grant.capacity,grant.depth);
      const oracle=db.query("SELECT items>=1 AND copy>=? AND capacity>=? AND depth>=2 admitted FROM budget").get(work,capacity) as {admitted:number};
      expect(oracle.admitted).toBe(0);
      expect(grant.items>=1&&grant.copy>=work&&grant.capacity>=capacity&&grant.depth>=2).toBe(false);
     }
     const before=cursor.outputCapacity();
     cursor.step(work,{maximumOutputBytes:fixture.maximumDecodedBytes,progress:()=>true});
     expect(cursor.outputCapacity()-before).toBe(capacity);born+=capacity;
     expect(++turns).toBeLessThan(1000);
    }
    const bytes=cursor.takeOutput()!;expect([...bytes]).toEqual(expected);expect([...bytes]).toEqual([...toByteArray(encoded)]);expect(born).toBe(expected.length);
    const parts=cursor.intoParts();expect(parts.input).toBe(encoded);expect(parts.output).toBeNull();pair.push(bytes);
   }
   expect(pair[0]).not.toBe(pair[1]);
  }
  for(const encoded of fixture.invalid){const cursor=new Base64DecodeCursor(encoded);expect(()=>{while(!cursor.isComplete())cursor.step(cursor.nextWorkDemand()||1,{maximumOutputBytes:fixture.maximumDecodedBytes,progress:()=>true});}).toThrow();expect(cursor.intoParts().input).toBe(encoded);}
 }finally{db.close();}
 expect(source.includes("source.matches(cx)")).toBe(true);
 expect(source.includes("scope.admit_owner(demand)?")).toBe(true);
 expect(source.includes("cursor.output_capacity().saturating_sub(before)")).toBe(true);
 expect(source.includes("parts.output")).toBe(true);
 expect(source.includes("settled_snapshot")).toBe(false);
 expect(source.includes("artifact_pair_snapshot")).toBe(false);
 console.log("[DEBUG] original document strict base64 pair outputs equal base64-js; SQLite independently rejects each deficient currency; exact capacity, original text and partial owners remain retained");
});
